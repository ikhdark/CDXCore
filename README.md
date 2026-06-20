# CDXMCPFix

CDXMCPFix finds Codex MCP config and startup problems.

It gives Codex a local support layer for MCP startup problems, confusing client config, and PATH differences. If an MCP server
works in your terminal but Codex cannot start it, CDXMCPFix helps answer the
practical questions:

- Which Codex config did this server come from?
- Is the command actually on the PATH Codex sees?
- Is `node`, `npx`, `uv`, `python`, `pnpm`, or `bun` missing?
- Is the working directory wrong?
- Did the server exit before MCP startup finished?
- Did `tools/list` hang, return bad tools, or miss `inputSchema`?
- Are plugin, bundled, or managed MCP entries confusing the picture?

Command Guard moved to the standalone CDXCoreGuard tool. CDXMCPFix stays focused on read-only MCP config and startup checks.

Do not install CDXMCPFix through an MCP marketplace for normal use. Install the
`cdxmcpfix` CLI, then let Codex launch `cdxmcpfix mcp-server`.

## Quick Install

Windows:

```powershell
irm https://github.com/ikhdark/CDXMCPFix/releases/latest/download/install.ps1 | iex
```

macOS/Linux:

```sh
curl -fsSL https://github.com/ikhdark/CDXMCPFix/releases/latest/download/install.sh | sh
```

Then check it:

```powershell
cdxmcpfix --version
```

The installer puts `cdxmcpfix` on PATH and runs:

```powershell
cdxmcpfix setup codex
```

That registers this MCP entry with Codex:

```toml
[mcp_servers.cdxmcpfix]
command = "cdxmcpfix"
args = ["mcp-server"]
```

Restart Codex after install so the app sees the updated PATH.

## What You Run

Most of the time, start here:

```powershell
cdxmcpfix check
```

That reviews your Codex MCP config and briefly starts each configured
command-based MCP server to time startup. It stops child processes after each
check.

Use `cdxmcpfix check <server>` for the same config review plus startup check
focused on one server.

Example report:

```text
Server: notion
Status: fail (not working)
Meaning: CDXMCPFix could not confirm this server starts and responds.
What to do: Treat this server as unavailable until the reported command, folder,
PATH, environment, connection type, or startup issue is fixed; then rerun check.
Cause: npx not found from Codex PATH
Evidence: command failed before MCP initialize
Suggested fix: use an absolute Node/npm path or add PATH in the MCP env block
Config source: C:\Users\<you>\.codex\config.toml
Secrets: redacted
```

Suggested command order:

```powershell
cdxmcpfix scan
cdxmcpfix check
cdxmcpfix fixes
cdxmcpfix check <server>
cdxmcpfix mcp-server
```

Older names remain available as aliases: `inspect-config`, `suggest-fixes`,
`validate`, and `serve`.

Add `--json` when you want JSON output:

```powershell
cdxmcpfix check --json
```

The JSON schema is `cdxmcpfix.diagnostics.v1` and lives at
`schemas/cdxmcpfix.diagnostics.v1.schema.json`.

Exit codes:

- `0`: working; the check completed with `pass`
- `1`: working but needs review; the check completed with `warn`
- `2`: not working; the check completed with `fail`, or config could not be
  read or parsed enough to list servers

CLI parser errors and unexpected internal CDXMCPFix errors are not health results.

Health meanings:

- `pass` / exit `0` means CDXMCPFix completed the requested check and found no
  problems. No action is required.
- `warn` / exit `1` means the server or config appears reachable enough to
  inspect, but CDXMCPFix found something that needs review. Read `Cause`,
  `Evidence`, and `Suggested fix`; common actions are checking the result in
  Codex, moving literal secrets into environment variables, or confirming that a
  v1 limitation such as HTTP config-only checks is acceptable.
- `fail` / exit `2` means CDXMCPFix could not confirm the server starts and
  responds. Treat the server as unavailable until the reported command, working
  directory, PATH, environment, connection type, or startup problem is fixed,
  then rerun `cdxmcpfix check <server>` or `cdxmcpfix check`.
- Config blocked / exit `2` means CDXMCPFix could not read enough config to list
  servers. Fix the reported TOML/JSON/path problem first, then run
  `cdxmcpfix scan` before checking individual servers.

## What `cdxmcpfix mcp-server` Is

`cdxmcpfix mcp-server` starts CDXMCPFix as an MCP server for Codex.

It exposes these tools to Codex:

- `inspect_mcp_config`
- `profile_mcp_startup`
- `validate_mcp_server`
- `diagnose_runtime`
- `suggest_config_fixes`

They do the same kind of checks as the CLI, but from inside a Codex session.

## What CDXMCPFix Checks

CDXMCPFix can report:

- missing config files
- invalid TOML or JSON
- duplicate names across config sources
- duplicate server commands
- missing executables
- bad working directories
- PATH mismatch between a terminal shell and the Codex process environment
- missing or suspicious environment, header, or OAuth values
- command-based servers that exit before `initialize`
- slow or timed-out `initialize`
- slow or timed-out `tools/list`
- missing tool `inputSchema`
- bounded `tools/list` pagination problems
- plugin `.mcp.json` servers
- plugin MCP policy overrides
- bundled or managed entries when CDXMCPFix can identify where they came from

HTTP and streamable HTTP servers are checked from config only in this release.
CDXMCPFix does not connect to HTTP MCP servers yet.

CDXMCPFix may not be able to identify every plugin-provided or bundled MCP entry
in v1. When it cannot verify an entry, it marks the report incomplete instead of
guessing.

## Secrets

CDXMCPFix is built to be safe to share.

It does not print raw env values by default. It redacts args, headers, OAuth
fields, URL userinfo/query values, server output, suggested snippets, and
JSON/TOML values under secret-looking keys.

Secret-looking terms include:

```text
token, key, secret, password, bearer, auth, credential, cookie, session, api, oauth
```

Suggested config snippets use placeholders such as `${TOKEN_ENV_VAR}` or
`<absolute path>`. They should not echo discovered secret values.

## Read-Only Promise

The check commands and MCP tools do not edit your MCP configs, delete state,
reset state, or call arbitrary MCP tools.

The only command that writes Codex config is:

```powershell
cdxmcpfix setup codex
```

That setup command is explicit. It installs only the CDXMCPFix MCP entry.

CDXMCPFix launches configured command-based MCP servers only when you run
`check`, `check <server>`, or the matching MCP tool.

## If PATH Is Weird

GUI apps often have a different PATH than your terminal.

If `cdxmcpfix --version` works in a terminal but Codex cannot start CDXMCPFix, use an
absolute path in `~/.codex/config.toml` or `$CODEX_HOME/config.toml`:

```toml
[mcp_servers.cdxmcpfix]
startup_timeout_sec = 15
command = "C:\\Users\\you\\AppData\\Local\\CDXMCPFix\\bin\\cdxmcpfix.exe"
args = ["mcp-server"]
```

For local developer testing, point at your local build:

```toml
[mcp_servers.cdxmcpfix]
startup_timeout_sec = 15
command = "C:\\Users\\kuh\\Desktop\\CDXMCPFix\\target\\release\\cdxmcpfix.exe"
args = ["mcp-server"]
```

## Plugin Files In This Repo

This repo includes:

- `.codex-plugin/plugin.json`
- `.mcp.json`

Those are for plugin-wrapper and local marketplace testing. Normal users should
install the CLI instead.

The plugin wrapper runs:

```json
{
  "command": "cdxmcpfix",
  "args": ["mcp-server"]
}
```

If Codex cannot find `cdxmcpfix` from its app PATH, use an absolute path
to the binary in `.mcp.json`.

## Updating

Update through the same release channel you used to install. If release notes say
the Codex MCP entry changed, rerun:

```powershell
cdxmcpfix setup codex
```

## Uninstalling

Remove the `cdxmcpfix` binary using the same method you used to install it.

If you also want to remove the Codex MCP entry, delete this block from
`~/.codex/config.toml` or `$CODEX_HOME/config.toml`:

```toml
[mcp_servers.cdxmcpfix]
```


## For Contributors

You only need this section if you are working on CDXMCPFix itself. Normal users
should use the installers above.

Local checks:

```powershell
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
sh -n scripts/install.sh
bash -n scripts/install.sh
```

Release downloads:

- Windows x64: `cdxmcpfix-v0.1.5-x86_64-pc-windows-msvc.zip`
- Linux x64: `cdxmcpfix-v0.1.5-x86_64-unknown-linux-gnu.tar.gz`
- macOS Apple Silicon: `cdxmcpfix-v0.1.5-aarch64-apple-darwin.tar.gz`
- Installers: `install.ps1`, `install.sh`

Latest release:

```text
https://github.com/ikhdark/CDXMCPFix/releases/latest
```

Versioned release:

```text
https://github.com/ikhdark/CDXMCPFix/releases/tag/v0.1.5
```

Verify downloads against `SHA256SUMS.txt`. CDXMCPFix binaries are not signed yet.
