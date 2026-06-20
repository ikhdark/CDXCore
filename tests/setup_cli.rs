use std::env;
use std::process::Command;
use tempfile::tempdir;

fn cdxmcpfix_output(args: &[&str]) -> String {
    let output = cdxmcpfix_command(args).output().expect("run cdxmcpfix");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout utf8")
}

fn cdxmcpfix_command(args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cdxmcpfix"));
    command.args(args);
    command
}

#[test]
fn top_level_help_mentions_setup_and_mcp_server() {
    let stdout = cdxmcpfix_output(&["--help"]);

    assert!(stdout.contains("setup"));
    assert!(stdout.contains("scan"));
    assert!(stdout.contains("mcp-server"));
    assert!(!stdout.contains("\n  check"));
    assert!(!stdout.contains("doctor"));
    assert!(!stdout.contains("explain"));
    assert!(!stdout.contains("guard-hook"));
}

#[test]
fn setup_codex_help_mentions_mcp_default_only() {
    let stdout = cdxmcpfix_output(&["setup", "codex", "--help"]);

    assert!(stdout.contains("cdxmcpfix mcp-server"));
    assert!(stdout.contains("Default setup installs only the CDXMCPFix MCP server"));
    assert!(!stdout.contains("--enable-command-guard"));
    assert!(!stdout.contains("--enable-retry-ledger"));
    assert!(!stdout.contains("--enable-command-repair"));
}

#[test]
fn legacy_command_aliases_are_removed() {
    for alias in [
        "check",
        "inspect-config",
        "suggest-fixes",
        "validate",
        "serve",
    ] {
        let output = cdxmcpfix_command(&[alias]).output().expect("run cdxmcpfix");
        assert!(
            !output.status.success(),
            "legacy alias {alias} unexpectedly succeeded"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("unrecognized subcommand"));
    }
}

#[test]
fn setup_codex_configures_mcp_server_without_hooks() {
    let fake_bin = tempdir().unwrap();
    let codex_home = tempdir().unwrap();
    write_fake_codex(fake_bin.path());

    let mut paths = vec![fake_bin.path().to_path_buf()];
    if let Some(existing_path) = env::var_os("PATH") {
        paths.extend(env::split_paths(&existing_path));
    }
    let joined_path = env::join_paths(paths).unwrap();

    let output = cdxmcpfix_command(&["setup", "codex"])
        .env("CODEX_HOME", codex_home.path())
        .env("PATH", joined_path)
        .env("PATHEXT", ".COM;.EXE;.BAT;.CMD")
        .output()
        .expect("run setup codex");

    assert!(
        output.status.success(),
        "setup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Configured Codex MCP server `cdxmcpfix`"));
    assert!(!codex_home.path().join("hooks.json").exists());
}

#[test]
fn setup_codex_writes_config_when_codex_cli_cannot_launch() {
    let empty_bin = tempdir().unwrap();
    let codex_home = tempdir().unwrap();

    let output = cdxmcpfix_command(&["setup", "codex"])
        .env("CODEX_HOME", codex_home.path())
        .env("PATH", empty_bin.path())
        .env("PATHEXT", ".COM;.EXE;.BAT;.CMD")
        .output()
        .expect("run setup codex");

    assert!(
        output.status.success(),
        "setup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Codex CLI could not be launched"));
    assert!(stdout.contains("Configured Codex MCP server `cdxmcpfix` in"));
    let config = std::fs::read_to_string(codex_home.path().join("config.toml")).unwrap();
    assert!(config.contains("[mcp_servers.cdxmcpfix]"));
    assert!(config.contains("startup_timeout_sec = 15"));
    assert!(config.contains("command = \"cdxmcpfix\""));
    assert!(config.contains("args = [\"mcp-server\"]"));
    assert!(!codex_home.path().join("hooks.json").exists());
}

#[cfg(windows)]
fn write_fake_codex(dir: &std::path::Path) {
    let cmd = env::var_os("COMSPEC").unwrap_or_else(|| "C:\\Windows\\System32\\cmd.exe".into());
    std::fs::copy(cmd, dir.join("codex.exe")).unwrap();
}

#[cfg(unix)]
fn write_fake_codex(dir: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let path = dir.join("codex");
    std::fs::write(&path, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}
