use std::io::Write;
use std::process::{Command, Stdio};

fn run_cdxcore(args: &[&str], stdin: &str) -> (i32, String, String) {
    run_cdxcore_with_env(args, stdin, &[])
}

fn run_cdxcore_with_env(
    args: &[&str],
    stdin: &str,
    envs: &[(&str, &str)],
) -> (i32, String, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cdxcore"));
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in envs {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("spawn cdxcore");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait cdxcore");
    (
        output.status.code().unwrap_or_default(),
        String::from_utf8(output.stdout).expect("stdout utf8"),
        String::from_utf8(output.stderr).expect("stderr utf8"),
    )
}

#[test]
fn guard_hook_json_flag_does_not_change_contract_output() {
    let payload = r#"{"tool_name":"Bash","tool_input":{"command":"rm -rf target"}}"#;
    let (plain_code, plain_stdout, plain_stderr) =
        run_cdxcore(&["guard-hook", "pre-tool-use"], payload);
    let (json_code, json_stdout, json_stderr) =
        run_cdxcore(&["--json", "guard-hook", "pre-tool-use"], payload);

    assert_eq!(plain_code, 0);
    assert_eq!(json_code, 0);
    assert_eq!(plain_stderr, "");
    assert_eq!(json_stderr, "");
    assert_eq!(plain_stdout, json_stdout);
    assert!(plain_stdout.contains("\"hookSpecificOutput\""));
    assert!(!plain_stdout.contains("permissionDecision"));
}

#[test]
fn guard_hook_malformed_input_is_silent_success() {
    let (code, stdout, stderr) = run_cdxcore(&["guard-hook", "pre-tool-use"], "{");

    assert_eq!(code, 0);
    assert_eq!(stdout, "");
    assert_eq!(stderr, "");
}

#[test]
fn guard_hook_debug_env_does_not_write_stderr() {
    let (code, stdout, stderr) = run_cdxcore_with_env(
        &["guard-hook", "pre-tool-use"],
        "{",
        &[("CDXCORE_DEBUG", "1")],
    );

    assert_eq!(code, 0);
    assert_eq!(stdout, "");
    assert_eq!(stderr, "");
}

#[test]
fn guard_hook_oversized_input_is_silent_success() {
    let oversized = format!(
        "{{\"tool_name\":\"Bash\",\"tool_input\":{{\"command\":\"rm -rf target\"}},\"padding\":\"{}\"}}",
        "x".repeat(140 * 1024)
    );
    let (code, stdout, stderr) = run_cdxcore(&["guard-hook", "pre-tool-use"], &oversized);

    assert_eq!(code, 0);
    assert_eq!(stdout, "");
    assert_eq!(stderr, "");
}
