use git_account_switcher::cli::Shell;
use git_account_switcher::config::{Account, Manifest, Settings, ShellIntegration};
use git_account_switcher::generator::Generator;
use git_account_switcher::resolver::Resolver;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_git_configuration_switching_e2e() {
    let temp_root = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_root.path();

    let fake_home = temp_path.join("home");
    let base_config_dir = fake_home.join(".config").join("gh-multiaccount");
    let work_root = temp_path.join("Developer").join("work");
    let personal_root = temp_path.join("Developer").join("personal");
    let outside_root = temp_path.join("Other").join("project");

    fs::create_dir_all(&fake_home).unwrap();
    fs::create_dir_all(&work_root).unwrap();
    fs::create_dir_all(&personal_root).unwrap();
    fs::create_dir_all(&outside_root).unwrap();

    let mut accounts = HashMap::new();
    accounts.insert(
        "work".to_string(),
        Account {
            name: "Work Developer".to_string(),
            email: "work@company.com".to_string(),
            roots: vec![work_root.clone()],
            signing_key: None,
            ssh_key: Some(fake_home.join(".ssh").join("id_ed25519_work")),
            ssh_host_alias: Some("github.com-work".to_string()),
        },
    );
    accounts.insert(
        "personal".to_string(),
        Account {
            name: "Personal Developer".to_string(),
            email: "personal@me.org".to_string(),
            roots: vec![personal_root.clone()],
            signing_key: None,
            ssh_key: Some(fake_home.join(".ssh").join("id_ed25519_personal")),
            ssh_host_alias: Some("github.com-personal".to_string()),
        },
    );

    let manifest = Manifest {
        settings: Settings {
            shell_integration: ShellIntegration::Direnv,
        },
        accounts,
    };

    let generator = Generator::new(base_config_dir.clone());
    let gen_res = generator.apply(&manifest).expect("Apply should succeed");

    assert!(gen_res.git_includes_path.exists());
    assert!(gen_res.ssh_config_path.exists());

    // ADR-0002: ~/.gitconfig points to includes.gitconfig
    let user_gitconfig_path = fake_home.join(".gitconfig");
    let user_gitconfig_content = format!(
        "[user]\n    name = Global Fallback\n    email = global@example.com\n[include]\n    path = {}\n",
        gen_res.git_includes_path.display()
    );
    fs::write(&user_gitconfig_path, user_gitconfig_content).unwrap();

    let work_repo = work_root.join("repo-alpha");
    fs::create_dir_all(&work_repo).unwrap();
    run_git_cmd(&work_repo, &fake_home, &["init"]);

    let personal_repo = personal_root.join("repo-beta");
    fs::create_dir_all(&personal_repo).unwrap();
    run_git_cmd(&personal_repo, &fake_home, &["init"]);

    let outside_repo = outside_root.join("repo-gamma");
    fs::create_dir_all(&outside_repo).unwrap();
    run_git_cmd(&outside_repo, &fake_home, &["init"]);

    // Case 1: Work repository matches work account
    let work_name = run_git_cmd(&work_repo, &fake_home, &["config", "user.name"]);
    let work_email = run_git_cmd(&work_repo, &fake_home, &["config", "user.email"]);
    let work_account = run_git_cmd(
        &work_repo,
        &fake_home,
        &["config", "--get", "ghmultiaccount.account"],
    );
    let work_github_account = run_git_cmd(
        &work_repo,
        &fake_home,
        &["config", "--get", "github.account"],
    );

    assert_eq!(work_name.trim(), "Work Developer");
    assert_eq!(work_email.trim(), "work@company.com");
    assert_eq!(work_account.trim(), "work");
    assert_eq!(work_github_account.trim(), "work");

    // Case 2: Personal repository matches personal account
    let personal_name = run_git_cmd(&personal_repo, &fake_home, &["config", "user.name"]);
    let personal_email = run_git_cmd(&personal_repo, &fake_home, &["config", "user.email"]);
    let personal_account = run_git_cmd(
        &personal_repo,
        &fake_home,
        &["config", "--get", "ghmultiaccount.account"],
    );

    assert_eq!(personal_name.trim(), "Personal Developer");
    assert_eq!(personal_email.trim(), "personal@me.org");
    assert_eq!(personal_account.trim(), "personal");

    // Case 3: Outside repository falls back to global
    let outside_name = run_git_cmd(&outside_repo, &fake_home, &["config", "user.name"]);
    let outside_email = run_git_cmd(&outside_repo, &fake_home, &["config", "user.email"]);
    assert_eq!(outside_name.trim(), "Global Fallback");
    assert_eq!(outside_email.trim(), "global@example.com");

    let outside_account_output = Command::new("git")
        .current_dir(&outside_repo)
        .env("HOME", &fake_home)
        .args(["config", "--get", "ghmultiaccount.account"])
        .output()
        .unwrap();
    assert!(!outside_account_output.status.success());

    // Case 4: Resolver cwd matching outside git repositories
    let uninitialized_work_subfolder = work_root.join("subfolder");
    fs::create_dir_all(&uninitialized_work_subfolder).unwrap();
    let resolved_account =
        Resolver::resolve_account(&manifest, Some(&uninitialized_work_subfolder));
    assert_eq!(resolved_account.as_deref(), Some("work"));

    // Case 5: Emit env exports
    let exports = Resolver::emit_env_exports(Some("work"), Shell::Bash, Some(&fake_home));
    assert!(exports.contains("export GH_CONFIG_DIR="));
    assert!(exports.contains("work"));

    // Case 6: Idempotent apply test
    let first_includes = fs::read_to_string(&gen_res.git_includes_path).unwrap();
    let second_res = generator
        .apply(&manifest)
        .expect("Second apply should succeed");
    let second_includes = fs::read_to_string(&second_res.git_includes_path).unwrap();
    assert_eq!(first_includes, second_includes);
}

fn run_git_cmd(cwd: &std::path::Path, fake_home: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(cwd)
        .env("HOME", fake_home)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run git {:?}: {}", args, e));

    assert!(
        output.status.success(),
        "git command failed: {:?} (stderr: {})",
        args,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("Valid UTF-8 output from git")
}
