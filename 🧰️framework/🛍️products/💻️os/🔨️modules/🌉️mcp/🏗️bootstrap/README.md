# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🕸️ CLI: `semio-os-mcp stdio [--folder <dir> | --hub <url> --space <id>]
[--principal <id>] [--scopes a,b] [--auto-approve never|readonly|all] [--no-bridge]` and
`semio-os-mcp http [--port <p>] [--bind <addr>]
[--folder <dir> | --hub <url> --space <id>] [--principal <id>] [--scopes a,b]
[--auto-approve never|readonly|all] [--audit-dir <dir>] [--allow-origin <origin>]…` (P1b + P1c +
P7-headless-workspace) — this binary owns argv parsing only; all real logic lives in
`crate::{run_stdio, run_http}`. `semio-os-mcp schemas` additionally prints the
`os.mcp` draft-07 schema mirror on stdout (the `schema-mirror` repository task's generator) (P1a's brief §2.5, "keep main thin, all logic in
the lib" — mirrors `🏃️run/🏗️bootstrap/🦀️.rs`'s own split). Unknown modes exit with a clear message rather
than silently doing nothing. `--folder`/`--hub` are mutually exclusive (`📋️master.md` §2.1:
"`--folder <space dir>`…`--hub <url> --space <id>`"). Hub authority is claimed from protected fd 3
before argv parsing and never enters argv or workspace state. HTTP and bridge admission reuse
that protected authority without copying it into argv, a URL, a file, logs, or protocol output.

<a id="source-notes-27"></a>

### 🦀️.rs:27

🚨️ `semio-os-mcp audit [--folder <dir>]` — compiles the catalog source from the committed
plugin descriptors under `<dir>` and prints every `CatalogAuditFinding` (a gesture-named
route published to agents with no declared audience; a delete/clear/replace mutation with
`effects.destructive = false`) and every `DescriptionFinding` (an agent-published verb whose
en/de description breaks the manifest's `CapabilityDescription` contract). Exits 1 when either
list is non-empty, so it is a gate.
