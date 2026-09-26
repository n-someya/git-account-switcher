use crate::cli::Shell;
use crate::config::{normalize_path, Manifest};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Resolver;

impl Resolver {
    pub fn resolve_account(manifest: &Manifest, cwd: Option<&Path>) -> Option<String> {
        if let Some(account) = Self::resolve_from_git() {
            return Some(account);
        }

        Self::resolve_from_roots(manifest, cwd)
    }

    fn resolve_from_git() -> Option<String> {
        let output = Command::new("git")
            .args(["config", "--get", "ghmultiaccount.account"])
            .output()
            .ok()?;

        if output.status.success() {
            let account = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !account.is_empty() {
                return Some(account);
            }
        }
        None
    }

    fn resolve_from_roots(manifest: &Manifest, cwd: Option<&Path>) -> Option<String> {
        let current_dir = match cwd {
            Some(p) => p.to_path_buf(),
            None => env::current_dir().ok()?,
        };

        let current_dir = current_dir.canonicalize().unwrap_or(current_dir);

        for (account_id, account) in &manifest.accounts {
            for root in &account.roots {
                let norm = normalize_path(root);
                let norm = norm.canonicalize().unwrap_or(norm);

                if current_dir.starts_with(&norm) {
                    return Some(account_id.clone());
                }
            }
        }

        None
    }

    pub fn emit_env_exports(
        account: Option<&str>,
        shell: Shell,
        base_gh_config_dir: Option<&Path>,
    ) -> String {
        let default_config_base = dirs::config_dir()
            .or_else(dirs::home_dir)
            .map(|p| p.join("gh-multiaccount").join("gh"))
            .unwrap_or_else(|| PathBuf::from(".config/gh-multiaccount/gh"));

        let config_base = base_gh_config_dir.unwrap_or(&default_config_base);

        match account {
            Some(acc) => {
                let account_gh_dir = config_base.join(acc);
                let dir_str = account_gh_dir.to_string_lossy();
                match shell {
                    Shell::Bash | Shell::Zsh => {
                        format!("export GH_CONFIG_DIR=\"{}\"\n", dir_str)
                    }
                    Shell::Fish => {
                        format!("set -gx GH_CONFIG_DIR \"{}\"\n", dir_str)
                    }
                }
            }
            None => match shell {
                Shell::Bash | Shell::Zsh => "unset GH_CONFIG_DIR\n".to_string(),
                Shell::Fish => "set -e GH_CONFIG_DIR\n".to_string(),
            },
        }
    }
}
