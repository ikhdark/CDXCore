# CDXMCPFix v3 — Cross-Client Read-Only Fleet View (Deferred Design Note)

> **Status: DEFERRED DESIGN NOTE — NOT PART OF THE CDXMCPFix v1 MVP.**
> This document records design decisions for a *future* v3 capability. Nothing
> here is implemented in v1. v1 ships no fleet-scan code, no new CLI commands, no
> additional config readers, no tests, no plugin config, and no implementation
> stubs related to cross-client scanning.

## v1 scope (unchanged)

CDXMCPFix v1 is a **Codex-only, strictly read-only MCP config and startup checker**.
v3 does not change v1. No v3 behavior ships in the MVP.

## v3 name

**Cross-client read-only fleet view.**

## v3 goal

Inspect MCP **config and startup health across all installed agent clients** on a
machine and emit a **single aggregated report**, rather than diagnosing one
client's config at a time.

## Clients

Reader coverage is staged, Codex first:

1. Codex
2. Claude Desktop
3. Cursor
4. Windsurf
5. VS Code

## Why it fits

v3 should extend v1's check model rather than adding a new kind of access. The
current v1 implementation is Codex-specific, so v3 work must first extract
client readers before adding Claude Desktop, Cursor, Windsurf, or VS Code
readers. Once that exists, v3 can list known client config locations, run the
per-server checks across all of them, and aggregate the results.

## Trust model

v3 **remains read-only**, the same safety promise as v1. It crosses no state
or control line. (Command Guard now lives in the separate CDXCoreGuard tool and remains outside v3 fleet inspection.)

## Commands

This distinction is load-bearing and mirrors v1's own config-only vs startup-check split:

- **`cdxmcpfix scan`** — reviews **config and source information only**.
  It reads config files and reasons about them. It does **not** launch any
  configured MCP server.
- **`cdxmcpfix scan --profile`** — may **explicitly launch and check configured
  command-based MCP servers** for startup timing, under the same read-only
  rules as v1 (short timeouts, `initialize` + bounded `tools/list`
  only, no arbitrary tool calls, guaranteed child-process termination).

Launching servers happens **only** under the explicit `--profile` flag, never on
a bare `scan`.

### Connection Limit

v3 inherits v1's connection limit: **only command-based servers can be launched and checked.**
HTTP / streamable HTTP configs receive **config-only validation** and are not
launched or connected, even under `--profile`. A fleet scan across
clients such as Claude Desktop or Cursor will commonly encounter HTTP servers;
these are checked from config, not launched.

## Cross-client findings

These are findings that **no single-client run can produce** — they emerge only
from looking across clients together:

- the **same server duplicated across clients**
- the **same broken `npx` / `node` / `uv` PATH issue** affecting multiple clients
- **conflicting names or duplicate server identities** across clients
- **managed, bundled, or injected entries that differ between clients**
- **unclear config source** (which file or layer a server actually came from)

## Secret handling (inherited from v1)

v3 **reuses v1's redaction rules.** Because a fleet report aggregates many
clients' configs into one output, its leak blast radius is larger than any
single-client run — so the no-raw-secrets guarantee applies to the **aggregated**
report as a whole: aggregated fleet reports must **never** expose raw secrets in
any field.

## Relationship to CDXCoreGuard

The former v2 command guard is now the separate CDXCoreGuard tool. v3 remains a read-only extension of CDXMCPFix and does not depend on CDXCoreGuard.

## Reaffirmation

v1 remains a Codex-only, read-only MCP config and startup checker. No fleet-scan,
no cross-client behavior, and no v3 implementation ships in the MVP.
