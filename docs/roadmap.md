# CDXMCPFix Roadmap

One-screen index of CDXMCPFix's planned scope. CDXMCPFix is a CLI-first tool with Codex setup support. **v1 is the only shipped product in this repository.**

## v1 — Read-only Codex MCP config and startup checks

CDXMCPFix v1 is strictly **read-only**: inspect Codex MCP config, report config issues, and check command-based MCP server startup only when explicitly requested. **This is the default MVP.**

## Command Guard — moved out

The former v2a/v2b/v2c Command Guard work now lives in `C:\Users\kuh\Desktop\CDXCoreGuard` and will be released as its own tool. Do not add Command Guard commands, hooks, tests, installer flags, or release packaging back into CDXMCPFix v1.

## v3 — Cross-client fleet view (deferred)

Cross-client read-only fleet view: inspect MCP config and startup health across installed agent clients in one report. **Requires extracting client readers from the Codex-only v1 code, and remains read-only like v1.** See [v3-fleet-view.md](v3-fleet-view.md).
