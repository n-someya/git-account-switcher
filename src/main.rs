use anyhow::{Context, Result};
use clap::Parser;
use git_account_switcher::cli::{Cli, Commands};
use git_account_switcher::config::Manifest;
use git_account_switcher::doctor::{CheckStatus, Doctor};
use git_account_switcher::generator::Generator;
use git_account_switcher::resolver::Resolver;
use git_account_switcher::ui::Reporter;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let base_dir = Generator::default_base_dir();
    let default_config_path = base_dir.join("accounts.toml");

    match cli.command {
        Commands::Init { force } => {
            let mut reporter = Reporter::stdout();
            handle_init(&mut reporter, &base_dir, &default_config_path, force)?;
        }
        Commands::Apply { config } => {
            let mut reporter = Reporter::stdout();
            let config_path = config.unwrap_or(default_config_path);
            handle_apply(&mut reporter, &base_dir, &config_path)?;
        }
        Commands::Env { shell } => {
            handle_env(&default_config_path, shell)?;
        }
        Commands::Doctor => {
            let mut reporter = Reporter::stdout();
            handle_doctor(&mut reporter, &base_dir, &default_config_path)?;
        }
    }

    Ok(())
}

pub fn handle_init<W: Write>(
    reporter: &mut Reporter<W>,
    base_dir: &Path,
    config_path: &Path,
    force: bool,
) -> Result<()> {
    if config_path.exists() && !force {
        reporter.warn(format!(
            "Manifest already exists at: {}",
            config_path.display()
        ))?;
        reporter.info("Use --force to overwrite with a fresh template.")?;
        return Ok(());
    }

    fs::create_dir_all(base_dir).context("Failed to create configuration directory")?;

    let template = r#"# git-account-switcher configuration
[settings]
shell_integration = "direnv"

[accounts.personal]
name = "Personal Name"
email = "personal@example.com"
roots = ["~/Developer/personal"]
ssh_host_alias = "github.com-personal"
ssh_key = "~/.ssh/id_ed25519_personal"

# [accounts.work]
# name = "Work Name"
# email = "work@example.com"
# roots = ["~/Developer/work"]
# ssh_host_alias = "github.com-work"
# ssh_key = "~/.ssh/id_ed25519_work"
"#;

    fs::write(config_path, template)
        .with_context(|| format!("Failed to write template to {}", config_path.display()))?;

    reporter.success(format!(
        "Initialized manifest at: {}",
        config_path.display()
    ))?;
    reporter
        .info("Edit this file to declare your accounts, then run: git-account-switcher apply")?;
    Ok(())
}

pub fn handle_apply<W: Write>(
    reporter: &mut Reporter<W>,
    base_dir: &Path,
    config_path: &Path,
) -> Result<()> {
    if !config_path.exists() {
        reporter.error(format!(
            "Configuration file not found: {}\nRun 'git-account-switcher init' first.",
            config_path.display()
        ))?;
        std::process::exit(1);
    }

    let manifest = Manifest::load_from_file(config_path)?;
    let generator = Generator::new(base_dir.to_path_buf());
    let result = generator.apply(&manifest)?;

    reporter.success("Applied configuration successfully.")?;

    reporter.section("Generated files:")?;
    for f in &result.generated_files {
        reporter.item(f.display())?;
    }

    reporter.section("Required one-time setup:")?;
    reporter.key_value("1. Git include", "Add to ~/.gitconfig:")?;
    reporter.raw_line(format!(
        "     [include]\n         path = {}\n",
        result.git_includes_path.display()
    ))?;

    reporter.key_value("2. SSH include", "Add near the top of ~/.ssh/config:")?;
    reporter.raw_line(format!(
        "     Include {}\n",
        result.ssh_config_path.display()
    ))?;

    Ok(())
}

fn handle_env(config_path: &Path, shell: git_account_switcher::cli::Shell) -> Result<()> {
    let manifest = if config_path.exists() {
        Manifest::load_from_file(config_path).ok()
    } else {
        None
    };

    let account = if let Some(m) = &manifest {
        Resolver::resolve_account(m, None)
    } else {
        None
    };

    let exports = Resolver::emit_env_exports(account.as_deref(), shell, None);
    print!("{}", exports);
    Ok(())
}

pub fn handle_doctor<W: Write>(
    reporter: &mut Reporter<W>,
    base_dir: &Path,
    config_path: &Path,
) -> Result<()> {
    reporter.section("git-account-switcher diagnostics")?;

    if !config_path.exists() {
        reporter.warn(format!(
            "Manifest file not found at: {}",
            config_path.display()
        ))?;
        reporter.info("Run 'git-account-switcher init' to create one.")?;
        return Ok(());
    }

    let manifest = match Manifest::load_from_file(config_path) {
        Ok(m) => {
            reporter.success(format!(
                "Manifest syntax is valid: {}",
                config_path.display()
            ))?;
            m
        }
        Err(e) => {
            reporter.error(format!("Manifest error: {:#}", e))?;
            return Ok(());
        }
    };

    let reports = Doctor::diagnose(&manifest, base_dir);
    for report in reports {
        match report.status {
            CheckStatus::Ok => {
                reporter.success(&report.check_name)?;
            }
            CheckStatus::Warning(msg) => {
                reporter.warn(format!("{}: {}", report.check_name, msg))?;
            }
            CheckStatus::Error(msg) => {
                reporter.error(format!("{}: {}", report.check_name, msg))?;
            }
        }
    }

    reporter.raw_line("\nDiagnostics complete.")?;
    Ok(())
}
