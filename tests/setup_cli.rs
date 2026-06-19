use std::process::Command;

fn cdxcore_output(args: &[&str]) -> String {
    let output = cdxcore_command(args).output().expect("run cdxcore");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout utf8")
}

fn cdxcore_command(args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cdxcore"));
    command.args(args);
    command
}

#[test]
fn top_level_help_mentions_setup_and_optional_command_guard() {
    let stdout = cdxcore_output(&["--help"]);

    assert!(stdout.contains("setup"));
    assert!(stdout.contains("guard-hook"));
    assert!(stdout.contains("Optional command guard"));
    assert!(stdout.contains("--enable-command-guard"));
}

#[test]
fn guard_hook_help_marks_hooks_optional_and_feedback_only() {
    let stdout = cdxcore_output(&["guard-hook", "--help"]);

    assert!(stdout.contains("optional"));
    assert!(stdout.contains("feedback-only"));
    assert!(stdout.contains("pre-tool-use"));
    assert!(stdout.contains("post-tool-use"));
}

#[test]
fn setup_codex_help_mentions_mcp_default_and_guard_opt_in() {
    let stdout = cdxcore_output(&["setup", "codex", "--help"]);

    assert!(stdout.contains("cdxcore serve"));
    assert!(stdout.contains("--enable-command-guard"));
    assert!(stdout.contains("Default setup installs only the CDXCore MCP server"));
    assert!(stdout.contains("feedback-only command guard hooks"));
}
