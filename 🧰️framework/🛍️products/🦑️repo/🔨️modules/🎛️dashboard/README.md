# 🎛️ Dashboard

The native Rust developer control plane discovers workspace commands and manages their terminal
sessions through one workspace daemon. It owns startup, preferences, command selection, output,
process lifecycle and reconnect. It works without an editor.

## 🎛️ Developer Control Plane

Start with `bun run dashboard` or the Dashboard launch entry. The first start builds and installs
the executable once; `bun run dashboard:install` republishes it after dashboard source changes.
Installation builds and publishes an immutable executable with visible progress and cancellation. Ordinary startup reads its small installation record and starts the native executable
directly, without provisioning tools, hashing the binary, computing an Nx graph or building.
The installation command prints the executable path for direct native invocation.

The first view immediately shows task controls. English, dark appearance, native terminology,
React and tabbed output are opinionated defaults; startup asks no questions. Command discovery
and daemon connection run in the background. Choose New Task and type search terms to select a
dev server, build, test, renderer or example directly. Settings are optional and persistent.
A selection made while the workspace daemon connects remains pending and can be cancelled with
`Ctrl+B c`; at most 128 starts can wait. The known catalog and full discovery run after the first frame.
The launcher offers every Nx project target, every root Nx package script and every terminal
configuration and compound of `.vscode/launch.json`, each filed under the verb it performs:
`setup`, `start`, `dev`, `serve`, `watch`, `activate`, `prepare`, `run`, `build`, `package`, `test`,
`smoke`, `check`, `typecheck`, `verify`, `gate`, `lint`, `format`, `generate`, `publish`, `deploy`,
`preview`, `bench`, `clean`, and `task` for any other name. A target reads
`build / ♻️mit-bestand / 📋️bericht / 🟦️typescript / build-zwischenbericht`, a script
`build / workspace scripts / mit-bestand / zwischenbericht / ▶ run` and a launch configuration
`build / launch / <name> — <command>`, so typing `build mit bestand zwischenbericht` lists all three.
Scripts and launch configurations are available on the first frame; project targets follow discovery.
Project targets execute through `bun nx run`; root scripts execute through `bun run` and their
declared Nx command. A launch configuration runs its declared command, working directory and
environment with `${workspaceFolder}`, `${env:NAME}` and defaulted `${input:id}` substituted: a plain
argument list runs directly, anything needing expansion runs through `sh -c` or `cmd.exe /d /s /c`.
A compound starts each member in its own task. A configuration needing an input without a default,
or of another debugger type, is not offered.
A finite task (every verb except `start`, `dev`, `serve`, `watch`, `activate` and `preview`) starts
from the project graph Nx last published instead of rebuilding it, by setting
`NX_FORCE_REUSE_CACHED_GRAPH=true`; file hashes and the task cache still read the current sources.
Rebuilding that graph is what Nx reports as "Waiting for graph construction in another process",
and it takes minutes while many Nx processes run. The published graph is used only while it is at
least as new as `nx.json`, the root `package.json` and every project manifest the last discovery
found; otherwise the task rebuilds and republishes it. Manifests created since the last discovery
(`Ctrl+B f`) and targets inferred from other files are not part of that check;
`SEMIO_DASHBOARD_GRAPH=fresh` makes every task rebuild the graph.
Dashboard tasks set `NX_NATIVE_COMMAND_RUNNER=false` and `NX_TUI=false` so the workspace daemon
owns terminal rendering and process lifecycle throughout Nx execution.
The dashboard entry streams Nx output so an outer Nx task view does not take over its terminal.
Independent daemon tasks clear the launcher's inherited Nx invocation identifier, allowing
repeated starts and restarts without appearing recursive to Nx.

`Ctrl+B` opens the keyboard controls:

| Key | Action |
| --- | --- |
| `n` | Search and start a task |
| `h` | Task overview, including statuses and exit codes |
| `p` | Optional settings |
| `f` | Refresh command discovery |
| `e` | Cancel discovery while keeping available commands |
| `s` | Restore all task views, including closed panes |
| `r` | Restart the selected task |
| `c` | Interrupt the selected task; terminate after a two-second grace period |
| `k` | Terminate the selected task and its descendants |
| `d` | Detach the view |
| `Q` | Shut down the daemon and every task |
| `a` | Switch appearance |
| `l` | Switch language |
| `-` / `\|` | Split horizontally / vertically |
| `z` | Toggle pane zoom |
| `t` | Toggle terminal input |
| `x` | Close the selected view |

Use `Tab` / `Shift+Tab` to select panes, `Esc` to leave terminal input, and `Ctrl+W` to close a
pane. Closing panes or detaching leaves tasks running. Reattaching restores their process IDs,
status, exit codes and recent output. Starting an already running command selects its task.
The daemon preserves the task's working directory and environment when restarting it.

`Esc` clears a launcher filter; another `Esc` returns to tasks. Search matches all typed words,
including Unicode labels. Preferred renderer moves matching commands to the front, and output
layout chooses tabs, columns or rows for new tasks. Manual splitting and zoom remain available.

## ⚙️ Preferences

Settings record commands as durable events. The default save scope is local-only; the settings
view can select workspace-shared changes. Startup precedence is defaults → workspace events
→ local events → environment → explicit launch arguments. Presentation is bound to a task when
it starts, and restart preserves that task's environment. Interactive changes take effect in the
current view; subsequent launches resolve stored events with their own environment and arguments.

Query with `bun run dashboard:preferences show`. Customize with
`bun run dashboard:preferences set --language de --appearance light --terminology reuse`.
Add `--workspace` to record a shared change. Native launch accepts `--language`, `--appearance`,
`--terminology`, `--renderer`, `--layout`, `--root` and `--config`; `--help` lists their values.

Environment overrides are `SEMIO_LOCALE`, `SEMIO_APPEARANCE`, `SEMIO_LOCKED_TERMINOLOGY`,
`SEMIO_DASHBOARD_RENDERER` and `SEMIO_DASHBOARD_LAYOUT`. `SEMIO_DASHBOARD_CONFIG` selects a local
journal. Values are validated strictly. Local events live under
`.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/preferences.jsonl`; shared events live under
`.🧬semio/🦑️repo/🎛️dashboard/⚙️preferences.jsonl`. Event revisions are serialized by operating-system
file locks, and each journal has a 1 MiB admission limit. Task output and view state remain ephemeral.

`bun run dashboard:start`, `bun run dashboard:status` and `bun run dashboard:stop` manage the
same daemon without an attached view. No editor launch configuration is required. Windows uses
named pipes and kernel job objects; Unix uses a private local socket and process groups.

Lifecycle events and their session projections are persisted locally in the repository dashboard
cache. Output is ephemeral local-only, bounded to 64 KiB per session. Views and input are also
ephemeral local-only. One workspace supports up to 128 retained sessions and 16 simultaneous
views. Disconnections do not transfer process ownership to a view; reconnect runs asynchronously.
Daemon restart retains completed projections and marks interrupted sessions as exited.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-dashboard` (binary `semio`, entry point in
  `🚪️entrypoint`), Nx project `@semio-tech/repo-dashboard-rs` with the `build`, `install`, `run`,
  `daemon`, `workflow` and `preferences` targets behind the root `dashboard` scripts
- `📦️installation` — the installation record, the immutable executable publication and the
  native launch the Nx bootstrap wrapper takes for `run`, `daemon`, `workflow` and `preferences`

The repo command line and the repo MCP server (`⌨️cli`) do not depend on this crate, so a compile
break in the dashboard or the terminal UI never reaches them.

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the recorded no-oracle decisions live in `🔮️oracles/🔣️.json`.

`🌳️command-tree-projection`. `🧫️fixtures/🚀️launch-configurations` pins launch-configuration
resolution, verb filing and search results for Rust and for an independent `jsonc-parser` projection.

`🌀️control-plane` defines language-neutral lifecycle scenarios and protocol vectors. The owned
Rust codec and the existing Ajv library validate the same vectors. The quick suite runs actual
Bun PTYs, Nx server/build/test targets, output replay, cancellation, restart, process-tree
termination, singleton enforcement and reattachment: `bun nx run @semio-tech/repo-dashboard-rs:test-quick`.

`🧊️execution` pins the native launch vectors of `🧫️fixtures/🧊️execution`, the Nx scheduling of
`run` and `install`, and the immutable installation that keeps a running executable's bytes while
a new build is published: `bun ./📜️script.ts test execution` in `📦️packages/🦀️rust`.

After installing, set `SEMIO_TEST_CLI` to the printed native executable and run the dashboard
test target with `--args='-- --include-ignored --nocapture'` to include native startup,
attach/detach and launcher tests. The launcher test types a search into an actual pseudo-terminal,
runs the selected launch configuration to its exit code and shuts the daemon down with `Ctrl+B Q`. `SEMIO_TEST_BUN` selects the repository-pinned Bun executable;
`SEMIO_TEST_ARTIFACT_DIR` keeps temporary fixture workspaces in the ticket's generated directory.
The native regression covers both a wrapper with redirected output and the actual Nx native
command runner. The test bridge captures its process search path before Cargo adds test DLL paths.

Shared preference and launcher fixtures are checked by Rust, existing Ajv and independent
JavaScript projections. Native first-frame timing is measured through an actual pseudo-terminal;
`SEMIO_DASHBOARD_TRACE=1` emits its elapsed microseconds for diagnostics.
