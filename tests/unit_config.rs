use git_account_switcher::config::{Manifest, ShellIntegration};
use std::path::PathBuf;

#[test]
fn test_parse_valid_manifest() {
    let toml_str = r#"
[settings]
shell_integration = "direnv"

[accounts.work]
name = "Work User"
email = "work@example.com"
roots = ["~/Developer/work"]
ssh_host_alias = "github.com-work"

[accounts.personal]
name = "Personal User"
email = "personal@example.com"
roots = ["~/Developer/personal"]
"#;

    let manifest: Manifest = toml::from_str(toml_str).expect("manifest should parse");
    assert_eq!(
        manifest.settings.shell_integration,
        ShellIntegration::Direnv
    );
    assert_eq!(manifest.accounts.len(), 2);

    let work = manifest.accounts.get("work").unwrap();
    assert_eq!(work.name, "Work User");
    assert_eq!(work.email, "work@example.com");
    assert_eq!(work.roots, vec![PathBuf::from("~/Developer/work")]);
    assert_eq!(work.ssh_host_alias.as_deref(), Some("github.com-work"));
}

#[test]
fn test_manifest_default_settings() {
    let toml_str = r#"
[accounts.personal]
name = "Personal User"
email = "personal@example.com"
roots = ["~/Developer/personal"]
"#;

    let manifest: Manifest = toml::from_str(toml_str).expect("manifest should parse");
    assert_eq!(
        manifest.settings.shell_integration,
        ShellIntegration::Direnv
    );
}
