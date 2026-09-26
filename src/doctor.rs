use crate::config::{normalize_path, Manifest};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum CheckStatus {
    Ok,
    Warning(String),
    Error(String),
}

pub struct DiagnosticReport {
    pub check_name: String,
    pub status: CheckStatus,
}

pub struct Doctor;

impl Doctor {
    pub fn diagnose(manifest: &Manifest, base_dir: &Path) -> Vec<DiagnosticReport> {
        let mut reports = Vec::new();

        reports.push(Self::check_git_include(base_dir));
        reports.push(Self::check_ssh_include(base_dir));

        for (account_id, account) in &manifest.accounts {
            if let Some(ssh_key) = &account.ssh_key {
                reports.push(Self::check_ssh_key_permissions(account_id, ssh_key));
            }

            for root in &account.roots {
                let norm = normalize_path(root);
                if !norm.exists() {
                    reports.push(DiagnosticReport {
                        check_name: format!("Root directory exists for account '{}'", account_id),
                        status: CheckStatus::Warning(format!(
                            "Root directory '{}' does not exist yet.",
                            norm.display()
                        )),
                    });
                }
            }
        }

        reports
    }

    fn check_git_include(base_dir: &Path) -> DiagnosticReport {
        let gitconfig_path = dirs::home_dir().map(|h| h.join(".gitconfig"));
        let target_include = base_dir.join("git").join("includes.gitconfig");

        let status = match gitconfig_path {
            Some(path) if path.exists() => {
                let content = fs::read_to_string(&path).unwrap_or_default();
                if content.contains(&target_include.to_string_lossy().to_string()) {
                    CheckStatus::Ok
                } else {
                    CheckStatus::Warning(format!(
                        "~/.gitconfig does not include '{}'. Add:\n  [include]\n    path = {}",
                        target_include.display(),
                        target_include.display()
                    ))
                }
            }
            _ => CheckStatus::Warning(format!(
                "~/.gitconfig not found. Make sure to add:\n  [include]\n    path = {}",
                target_include.display()
            )),
        };

        DiagnosticReport {
            check_name: "Git global include check".to_string(),
            status,
        }
    }

    fn check_ssh_include(base_dir: &Path) -> DiagnosticReport {
        let ssh_config_path = dirs::home_dir().map(|h| h.join(".ssh").join("config"));
        let target_include = base_dir.join("ssh").join("config");

        let status = match ssh_config_path {
            Some(path) if path.exists() => {
                let content = fs::read_to_string(&path).unwrap_or_default();
                if content.contains(&target_include.to_string_lossy().to_string()) {
                    CheckStatus::Ok
                } else {
                    CheckStatus::Warning(format!(
                        "~/.ssh/config does not include '{}'. Add near the top:\n  Include {}",
                        target_include.display(),
                        target_include.display()
                    ))
                }
            }
            _ => CheckStatus::Warning(format!(
                "~/.ssh/config not found. Add near the top of ~/.ssh/config:\n  Include {}",
                target_include.display()
            )),
        };

        DiagnosticReport {
            check_name: "SSH global include check".to_string(),
            status,
        }
    }

    fn check_ssh_key_permissions(account_id: &str, key_path: &Path) -> DiagnosticReport {
        let norm = normalize_path(key_path);
        let check_name = format!("SSH key permission check for account '{}'", account_id);

        if !norm.exists() {
            return DiagnosticReport {
                check_name,
                status: CheckStatus::Error(format!(
                    "SSH key file '{}' does not exist.",
                    norm.display()
                )),
            };
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&norm) {
                let mode = metadata.permissions().mode() & 0o777;
                if mode != 0o600 && mode != 0o400 {
                    return DiagnosticReport {
                        check_name,
                        status: CheckStatus::Warning(format!(
                            "SSH key '{}' has loose permissions (0{:o}). Recommended: chmod 600 {}",
                            norm.display(),
                            mode,
                            norm.display()
                        )),
                    };
                }
            }
        }

        DiagnosticReport {
            check_name,
            status: CheckStatus::Ok,
        }
    }
}
