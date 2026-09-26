use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    #[serde(default)]
    pub settings: Settings,
    pub accounts: HashMap<String, Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    #[serde(default = "default_shell_integration")]
    pub shell_integration: ShellIntegration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shell_integration: default_shell_integration(),
        }
    }
}

fn default_shell_integration() -> ShellIntegration {
    ShellIntegration::Direnv
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ShellIntegration {
    Direnv,
    Wrapper,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub roots: Vec<PathBuf>,
    #[serde(default)]
    pub signing_key: Option<String>,
    #[serde(default)]
    pub ssh_key: Option<PathBuf>,
    #[serde(default)]
    pub ssh_host_alias: Option<String>,
}

impl Manifest {
    pub fn from_toml_str(content: &str) -> Result<Self> {
        let manifest: Manifest =
            toml::from_str(content).context("Failed to parse accounts.toml")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref()).with_context(|| {
            format!("Failed to read manifest file: {}", path.as_ref().display())
        })?;
        Self::from_toml_str(&content)
    }

    pub fn validate(&self) -> Result<()> {
        if self.accounts.is_empty() {
            bail!("Manifest must contain at least one account.");
        }

        let mut seen_roots = HashMap::new();

        for (account_id, account) in &self.accounts {
            if account.name.trim().is_empty() {
                bail!("Account '{}' has an empty name.", account_id);
            }
            if account.email.trim().is_empty() {
                bail!("Account '{}' has an empty email.", account_id);
            }
            if account.roots.is_empty() {
                bail!("Account '{}' has no roots configured.", account_id);
            }

            for root in &account.roots {
                let normalized = normalize_path(root);
                if let Some(existing_account) = seen_roots.insert(normalized.clone(), account_id) {
                    bail!(
                        "Root directory '{}' is duplicated between '{}' and '{}'.",
                        normalized.display(),
                        existing_account,
                        account_id
                    );
                }
            }
        }

        Ok(())
    }
}

pub fn normalize_path(path: &Path) -> PathBuf {
    if let Ok(stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }
    path.to_path_buf()
}
