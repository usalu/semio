# 🎛️ Dashboard

The native Rust developer control plane discovers workspace commands and manages their terminal
sessions through one workspace daemon. It owns startup, preferences, command selection, output,
process lifecycle and reconnect. It works without an editor. A ready task offers **Open ready address**
in its overview row and output context menu; `Ctrl+B u` opens the verified URL and `Ctrl+B U` cancels
opening while the task keeps running. Both bindings are customizable. The opener runs outside the
view loop with a bounded, cancellable helper. `SEMIO_DASHBOARD_URL_OPENER` optionally selects a JSON
argument vector to which the exact ready URL is appended.

## 🎛️ Developer Control Plane

The dashboard is the only control plane for developers, editors and agents. Everything runnable is a
registry command (below); the interactive view and the command line start commands the same way,
through the same workspace daemon, so a task started in one is the task the other shows. Choose **New Task**
to select a declared command, fill its typed form and inspect the preview before starting.

Start with `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream` (the root script is the build runner bootstrap; it runs the installed native
executable `semio`). Commands, typed parameters, arguments, environment, output, progress and
cancellation are controlled through the dashboard. No IDE workspace, solution, task, launch or IDE MCP configuration is generated or required.
Headless agent client files are derived from registered dashboard declarations by
`workspace:agents-write`. The first start builds and installs the executable once. Installation builds and
publishes an immutable executable with visible progress and cancellation (`Ctrl+C`), and records beside it
a digest of the sources it was built from: the names, sizes and modification times of the files below the
dashboard module and the modules of every path dependency of its crate (tests, fixtures, build output and
`node_modules` excluded), plus `Cargo.toml`, `Cargo.lock` and `rust-toolchain.toml`. Ordinary startup reads
the small installation record, recomputes that digest (about 20 ms, no file is read) and starts the native
executable directly, without provisioning tools, hashing the binary or computing a build graph. When the digest
differs, the start first rebuilds and reinstalls with the same visible progress and says why; a failed or
cancelled rebuild stops the start instead of running an outdated executable, except for the verbs that only observe or
end what already runs (`tasks`, `logs`, `stop`, `kill`, `open`, `daemon stop`, `daemon status`): a broken working
tree must never keep a developer from stopping the daemon. `bun repo run @semio-tech/repo-dashboard-rs:build --exclude-dependencies --skip-cache --output=stream` republishes on demand and
prints the executable path for direct native invocation.

The first view immediately shows task controls. Dark appearance, native terminology,
React and tabbed output are opinionated defaults; the language is the one the system names (`LC_ALL`,
`LC_MESSAGES`, `LANG`) or else the first language of the declared chain in `⚙️preferences/🌐️language/🔣️.json`
(English, then German); startup asks no questions. Command discovery
and daemon connection run in the background. Choose New Task and type search terms to select a
dev server, build, test, renderer or example directly. Settings are optional and persistent.
A selection made while the workspace daemon connects remains pending and can be cancelled with
`Ctrl+B c`; at most 128 starts can wait. The known catalog and full discovery run after the first frame.

### 🧑‍💻 Daily Work

Choose **New Task** (`Ctrl+B n`), search the words below, select the command and
complete its form. Choose **Start** after checking the preview. The same form
accepts extra arguments and environment values; recurring operations belong in
an owner target or typed dashboard declaration.

| Work | Search or command | Next action |
| --- | --- | --- |
| Prepare the checkout | `workspace:setup` | Start the setup task; read progress in its output. For native toolchain repair choose `workspace:setup-native`. |
| Inspect build commands | `tool:workspace/build-help` | Read the current runner's help in retained task output. |
| Test affected projects | `tool:workspace/build-affected` | Choose target names, backend and optional prototype/release configuration. |
| Test a project | `test` plus the owner name | Select `<project>:test`; choose the test level in the form. `workspace:test` runs the workspace suite. |
| Test one Bun file | `tool:workspace/bun-test` | Choose an owner or search source suggestions; selecting a file fills its current owner. |
| Check one TypeScript project | `@semio-tech/repo-lib:typecheck-project` | Select an authored configuration and diagnostic detail. The current compiler owner manages state, progress and cancellation. |
| Check, lint and format | `workspace:verify`, `workspace:lint`, `workspace:format` | Choose the check when offered, then inspect the exit code and output. Search `check` or `typecheck` plus the owner for scoped checks. |
| Build and publish | `workspace:build`, `workspace:publish` or `build` plus the owner | Choose the build mode and scope offered by the form; output identifies the artifacts. |
| Serve and preview | `playground:s`, `os-hub:dev`, `workspace:dev-storybook` or `dev` plus the owner | Choose renderer/example/user slot when offered; wait for readiness, then choose **Open ready address** (`Ctrl+B u`). |
| Inspect a failure | **Task overview** (`Ctrl+B h`) | Select the task to see its status, exit code and retained output. Restore a hidden output view with `Ctrl+B s`. |
| Retry or cancel | Selected task menu | **Restart** (`Ctrl+B r`), **Stop** (`Ctrl+B c`) or **Kill** (`Ctrl+B k`) controls the task and descendants. Closing its view keeps it running. |
| Debug interactively | `@semio-tech/repo-developer-tools:debug` | Check `debuggers` for installed tools, then select Node.js Inspector, Python Pdb, GDB or LLDB, a workspace program, working directory and literal program argument list. Toggle task input with `Ctrl+B t` to set breakpoints and step; stop or restart with the task menu. Node uses a fresh local inspector port. Windows GDB currently accepts ASCII target and working-directory paths; check LLDB installation for Unicode native targets. |
| Open code at a line | `@semio-tech/repo-developer-tools:open-source` | Fill in file, optional line and installed editor; edit the source, then return to the task controls. |
| Inspect repository changes | `@semio-tech/repo-developer-tools:git-status`, `git-diff`, `git-history` | Choose path, change area or revision in the form; staged diffs take one base revision. |
| Inspect a revision or line history | `@semio-tech/repo-developer-tools:git-show`, `@semio-tech/repo-developer-tools:git-blame` | For Show choose one revision, path and summary/patch view. For Blame choose file, optional revision, starting line and up to 500 lines. |
| Manage tickets and goals | `repo` plus `ticket`, `goal`, `analyze`, `tree` or `statute` | Select the domain action, fill in its typed parameters and confirm any state-changing action. |
| Start developer tools | `tool` plus `mcp`, `storybook`, or the tool name | Select the registered tool; readiness and process controls appear in its task. |
| Run several targets | `tool:workspace/run-targets` | Select target names, project names or patterns, concurrency and cache. |
| Run a scoped authored operation | `tool:workspace/run-script` | Search canonical script suggestions and inspect the selected owner and working directory; override the directory when the operation requires it. Put command words in **Arguments**. |

The task overview and context menus offer these controls without memorizing keys.
Settings (`Ctrl+B p`) changes language, appearance and bindings. English and
German labels are available throughout. A detached view reconnects to the same
running tasks on the next dashboard start. For automation, `semio tasks`,
`semio logs <task> --follow`, `semio restart <task>` and `semio stop <task>` use
the same handles and output as those controls.

### 🪟️ Window System

Drag a tab to the left or right edge for side-by-side windows, or to the top or bottom edge for stacked
windows. Drop in the center or on another group's tab strip to join its tabs. A tinted outline previews
the final rectangle, including space reclaimed from the old group; release commits the arrangement.
Drag tabs along their strip to reorder them and drag dividers to resize adjacent windows. Splits can nest
in either direction, and each split retains its own tab group. Escape, focus loss, terminal resize and
release outside a window cancel a pending docking gesture. Rearranging and closing views preserve running
tasks and terminal output; layouts belong to each local view.

The prefix defaults to `Ctrl+B`. After it, `Ctrl+Left/Right/Up/Down` moves the current tab beside the next
window, `m` joins that window's tabs, and `Shift+Left/Right` changes tab order. Existing `|` / `v` and `-`
create splits, `+` / `_` resize, `z` zooms or restores, `Tab` / `Shift+Tab` cycles focus and `Ctrl+W` closes a
view. Every action is customizable and listed in English and German keyboard help.

The docking command is schema-defined in `🧬️schema/🪟️windows/🔣️.json`. The shared TUI layout owns atomic
docking, topology normalization, weights and geometry; its engine owns pointer capture, hit testing and
preview painting. The dashboard routes the same commands for pointer and keyboard input. Run the
**test windows** registry target for contract and native interaction tests, and **test journeys** for the
independent pseudo-terminal screen checks.

### 🎮️ The Command Registry

The registry (`🎮️registry`) is the one source of what can be started. Its sources, and nothing else:

| Source | Maintained in |
| --- | --- |
| build targets, their `continuous` flag and `configurations` | each `📋️project.json`, read from the manifests and the published project graph |
| Playground variants (ports, examples, user slots, hub, data directories, viewer path) | `[[package.metadata.semio.playground]]` of the plugin crate, through the generated catalog |
| Declarations: parameters, ready ports, requirements, tools, compounds, groups | `metadata.semio.dashboard` of the owning `📋️project.json`, schema `🧬️schema/🎮️registry/🔣️.json` |
| Ticket tools and compounds, offered while the ticket is open | `🎮️commands.json` of the ticket folder |
| Repo-domain actions: tickets, goals, analyze, tree, statutes | the Rust domain crates, in process |

Root `package.json` scripts are not a source: the root keeps only the bootstrap scripts.

A command has an identity (`<project>:<target>[:configuration]`, `playground:<variant>`,
`tool:<project>/<id>`, `compound:<project>/<id>`, `group:<project>/<id>`, `ticket:<ticket id>/<id>`,
`repo:<action>`), a verb (`setup`, `start`, `dev`, `serve`, `watch`, `activate`, `prepare`, `run`, `build`,
`package`, `test`, `smoke`, `check`, `typecheck`, `verify`, `gate`, `lint`, `format`, `generate`,
`publish`, `deploy`, `preview`, `bench`, `clean`, or `task` for any other name) and the parameters it
accepts. A target reads `build / ♻️mit-bestand / 📋️bericht / 🟦️typescript / build-zwischenbericht`, so
typing `build mit bestand zwischenbericht` finds it. Long-running means `continuous`, a tool's
`continuous`, a playground, or a declared `ready`.

Declarations (all optional, schema-first, validated; a broken one is a reported problem naming its
file and never hides a valid command):

- `ready { port, portEnv, path, printed }`: the task is ready when its output shows
  `http://(127.0.0.1|localhost|0.0.0.0):<port>`; the ready address is that match plus `path`, or with
  `printed` the whole printed address. The daemon checks that exact local HTTP endpoint for a 2xx or 3xx response before publishing readiness. Bounded probes run outside the supervisor loop and are cancelled when a task stops or restarts.
- `requires`: commands started (or reused) and awaited first. A compound's `members` and a target's or
  tool's `requires` share one shape: `"<id>"` or `{ "run": "<id>", "parameters": {…}, "env": {…} }`, where
  `env` states a relationship between members, for example the hub address a shell joins.
- `parameters`: a `choice` (one effect per value), a `flag` or a `text`, with `env`, `buildFlags` (before
  `--`, ignored by commands that do not use the build runner) and `args` (after `--`). The global axes live on the root
  project: `cache`, `test-level`, `dependencies`, `build-mode`, `nextest-output`, `cargo-jobs`,
  `build-budget`; `appliesTo.verbs` selects every command with that verb, tools included.
- `tools`, `compounds` (`stop: "together"`), `groups` (`targets: [..]` over build project patterns).
- Tokens `{workspace}`, `{port}` and `{<parameter id>}`; literal absolute paths and the runner variables
  `NX_DAEMON`, `NX_TUI`, … are never stored.
- A service port belongs to one owner: the project of a target or tool, the ticket of a ticket tool, the
  playground itself. Commands of one owner that share a port are alternatives of the same service (the
  scoped storybooks, the hub variants, the inspector tool beside its targets); two owners claiming the same
  port are a problem of `semio commands --check`, which compares the default ready ports of long-running
  commands with every playground port (react, wgpu, user slots). `--check` also reports a playground
  renderer whose build target is missing from the graph and every plugin error recorded in the published
  graph (a partial graph is not healthy).
- Playgrounds gain `renderer`, `example` (its slug or bare id), `user-slot`, `app-role`, `language`,
  `terminology`, `appearance` and, from their catalog row, `hub`, `data` and `local-only`.

Authored scripts use `tool:workspace/run-script`: `script` is the workspace-relative `📜️script.ts`,
`project` selects its build owner (default `workspace`), and `directory` selects the working directory
(default the workspace root). Extra arguments follow `--`, and explicit environment values use
`--env KEY=value`. This entry invokes the authored script through the build runner; named recurring workflows
belong in their owner's target or typed declaration.
Everything resolves to plain argument lists. Dashboard task launches use `bun repo run` and
`bun repo run-many`. The selected build adapter owns backend flags, environment and graph discovery.
The dashboard reads the versioned neutral publication at `.cache/b/g/m/canonical.json`. Its shared Rust path derives from the neutral cache partition policy, the native mounted namespace and the publication-location schema; `graph.json` in that partition remains the storage charge receipt.
Finite tasks request graph reuse only while the publication and its manifest content remain current.
`SEMIO_BUILD_TERMINAL=false` delegates terminal rendering and lifecycle to the dashboard daemon.
Independent daemon tasks clear the inherited `SEMIO_BUILD_INVOCATION_PID` before starting.

### ⌨️ The Command Line

The binary `semio` drives the same registry and daemon without a view (`semio` without a verb is the
dashboard; `semio --help` prints the usage). `semio …` is shorthand for `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream -- …`: the devcontainer
puts a `semio` shim on `PATH`, a native host does not, so there `semio run os-hub:dev --detach --wait-ready` is typed
`bun repo run @semio-tech/repo-dashboard-rs:run --output=stream -- run os-hub:dev --detach --wait-ready`; the tables below use the short form.

| Verb | Does |
| --- | --- |
| `semio commands [words…] [--json] [--all] [--check] [--refresh] [--root PATH] [--snapshot PATH]` | list or search the registry; `--check` proves every declaration and reference and fails on any problem |
| `semio run <id> [--param k=v\|flag]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--raw] [--timeout S] [--refresh] [--root PATH] [--snapshot PATH] [-- args…]` | run a command as a daemon task |
| `semio tasks [--json] [--root PATH]` | list the tasks of the daemon (or of the journal while no daemon runs) |
| `semio logs <task> [--follow] [--raw] [--root PATH]` | print the output of a task, then optionally follow it |
| `semio stop\|restart\|kill <task> [--group] [--timeout S] [--root PATH]` | control a task and wait until it left its state |
| `semio open <task> [--print] [--root PATH]` | open the ready address of a task in the system browser |
| `semio daemon start\|status\|stop\|attach\|serve [--root PATH]` | manage the workspace daemon |
| `semio dashboard [--language L] [--appearance A] [--terminology T] [--renderer R] [--layout L] [--prefix KEY] [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace] [--help]` | the interactive dashboard (also plain `semio`); `--help` prints its keyboard help |
| `semio preferences show\|set [--language L] [--appearance A] [--terminology T] [--renderer R] [--layout L] [--prefix KEY] [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace]` | read or record preferences |
| `semio catalog [--json]` | list the playground catalog |
| `semio plugin registry generate\|check` | generate or check the plugin registry |
| `semio command-tree [--dump-tree] [--root PATH]` | print the command tree |

`semio run`: attached, it streams the output of the launch's last process and exits with that process's
exit code (Ctrl-C leaves the task running); `--detach` returns once the daemon accepted the launch;
`--wait-ready` returns once every process that declares readiness printed its address, which is printed as
the last lines of stdout; `--dry-run` prints the resolved launch (processes, environment, ready address,
requirements) as JSON, starts nothing and needs no daemon. A task already running with the same command is
reused. `--param id=value` chooses a parameter (`--param id` alone switches a `flag` on); `--env KEY=value`
adds free environment to every process of the launch (`{workspace}`, `{port}` and chosen `{parameter}` tokens
expand) and words after `--` are appended after `--` of the command. The retired `semio dev <variant>` is
`semio run playground:<variant> --param renderer=…`; free build switches are `--env SKIP_PLUGIN_BUILD=1`. A task has one handle, its session id, which `run`, `tasks`, `logs`, `stop`,
`restart`, `kill` and `open` all print and accept and which never changes; they also take a group id (all
members of a launch), a command id (its live session, else its latest) or a unique part of a session or
command id, and an ambiguous name is refused with the candidates. A position in a listing is no handle. A selection the declarations do not allow exits with status 2 and
names the problem on stderr.

`semio --help` prints the usage with every flag (`semio dashboard --help` the keyboard help). Semio has no other
verbs and forwards nothing to another script: every task of the monorepo is a registry command. The routes of
the root script are targets of the root project, so `semio run workspace:test`, `semio run workspace:verify
--param check=taxonomy`, `semio run workspace:dev-storybook`, `semio run workspace:os -- run <bundle>.studio`,
`semio run workspace:semio -- inspect <path>` and `semio run workspace:examples -- list` start them as daemon
tasks (`semio commands workspace` lists them). Agent skills use registered dashboard commands for authored operations. Any other verb is an
error that prints the usage and exits with status 2.

Agents and editors start servers with `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream -- run <id> --detach --wait-ready`, read the printed
address and attach browser previews to it; find ids with `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream -- commands <words> --json`.

The bootstrap preserves the original process envelope through native preparation and cancels its build
subprocess tree on interruption or deadline expiry. Cancelling a view ends that view and retains the
workspace daemon and its tasks. A separately supervised task gets a new invocation
for its actual command: the view's script envelope, capabilities and policy are discarded from inherited
environment, while explicit task environment remains authoritative.

### 🚀 The Launcher

The launcher is a pure state machine over the registry. Commands are filed by verb, then by owner; typing
searches all words incrementally (Unicode labels included) and selects the first match, and a second step
activates (select, then activate; a mouse click does the same). A playground is one entry with a
`renderer` parameter (the preferred renderer pre-selects it); `language`, `terminology` and `appearance`
are pre-selected from the preferences for the commands that offer them. A command with parameters opens
a parameter form: `choice`, `text` and `flag` parameters with their defaults, free extra arguments and
free extra environment. A live preview shows what the choice resolves to: the services it requires, the
members of a compound and the exact command. A command that changes repository state (closing or
reopening a ticket) asks for confirmation. Starting resolves the selection with the registry and sends one
group to the daemon; services that already run are not started again. `Esc` goes back one step.

Text fields use the framework input widget and show the hardware cursor at the actual caret.
Arrow keys, Home, End, Backspace, Delete, pointer positioning and bracketed paste edit that field;
line breaks in pasted command parameters become spaces. The configured Start action launches
with one click, including when a text field still has keyboard focus. Confirmation previews remain
pending until their visible Start action is activated.

### ⌨️ Keys

The keyboard is data: a prefix key and bindings in three scopes (`prefix`, `window`, `view`), each binding an
action id and its keys, defaults in `⚙️preferences/⌨️keymap/🔣️.json`. Preference events override single
bindings (`--prefix`, `--bindings`); a collision, an unknown action or a typing key is ignored and named,
never silent. The footer, the help window (`Ctrl+B ?`) and `semio --help` are generated from the keymap and
the label catalogue (English and German), so this table is only the default state:

| Key after `Ctrl+B` | Action |
| --- | --- |
| `n` | Search and start a task |
| `h` | Task overview, including statuses and exit codes |
| `p` | Optional settings |
| `?` | Keyboard help |
| `s` | Restore hidden task views |
| `f` / `e` | Refresh / cancel command discovery |
| `r` / `c` / `k` | Restart / stop (interrupt, terminate after a grace period) / kill the selected task and its descendants |
| `y` | Copy the selection |
| `t` | Toggle terminal input |
| `a` / `l` | Switch appearance / language |
| `-` / `\|` or `v` | Split down / right |
| `+` / `_` | Grow / shrink the window |
| `z` | Toggle zoom |
| `]` `o` / `[` `O` | Next / previous window |
| `x` | Close the window |
| `d` | Detach the view |
| `Q` | Shut down the daemon and every task (waits for it) |
| `Esc` | Cancel the armed prefix |

Without the prefix, `Tab` / `Shift+Tab` select windows and `Ctrl+W` closes one. A focused terminal receives
every key except the prefix (`Ctrl+B Ctrl+B` sends it), encoded for the child's mode; paste, mouse (click,
drag, wheel), text selection and copy go through the engine, and a right-click opens a context menu for the
window, tab or terminal. Closing windows or detaching leaves tasks running. Reattaching restores their
process IDs, status, exit codes and recent output. The daemon preserves a task's working directory and
environment when restarting it. A selection made while the daemon connects stays pending (at most 128) and
`Ctrl+B c` cancels it.

## ⚙️ Preferences

Settings record commands as durable events. The default save scope is local-only; the settings
view can select workspace-shared changes. Startup precedence is defaults → workspace events
→ local events → environment → explicit launch arguments. Presentation is bound to a task when
it starts, and restart preserves that task's environment. Interactive changes take effect in the
current view; subsequent launches resolve stored events with their own environment and arguments.

Query with `semio preferences show`. Customize with
`semio preferences set --language de --appearance light --terminology reuse`.
Add `--workspace` to record a shared change; `--prefix` and `--bindings '{…}'` record keymap changes. Native
launch accepts `--language`, `--appearance`, `--terminology`, `--renderer`, `--layout`, `--prefix`, `--root` and
`--config`; `--help` lists their values.

Environment overrides are `SEMIO_LOCALE`, `SEMIO_APPEARANCE`, `SEMIO_LOCKED_TERMINOLOGY`,
`SEMIO_DASHBOARD_RENDERER`, `SEMIO_DASHBOARD_LAYOUT` and `SEMIO_DASHBOARD_PREFIX`. `SEMIO_DASHBOARD_CONFIG` selects a local
journal. Values are validated strictly. The language resolves from the system and the declared chain at the
bottom, then shared events, local events, `SEMIO_LOCALE` and `--language` over it; `Locale` has no `Default`. Local events live under
`.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/preferences.jsonl`; shared events live under
`.🧬semio/🦑️repo/🎛️dashboard/⚙️preferences.jsonl`. Event revisions are serialized by operating-system
file locks, and each journal has a 1 MiB admission limit. Task output and view state remain ephemeral.

### 🌀️ The Daemon

`semio daemon start`, `semio daemon status` and `semio daemon stop` manage the workspace daemon without an
attached view; `semio run` starts it when none runs. One daemon serves one workspace and owns every task
independently of the views attached to it. Windows uses a named pipe (stable name from the XXH3 of the
canonical root, current user only, remote clients rejected, a second daemon for the same root refused)
and a kernel job object; Unix a private per-user socket directory and process groups.

- **Handshake.** A connection says hello with its protocol revision, the build id of its executable and its
  own environment, which is the base environment of every task it starts (the daemon's is not). A different
  protocol cannot be served and is reported with the way out (`semio daemon stop` once the tasks finish); a
  different build is served and reported, never silently. A request before hello is refused.
- **Starting.** A launch is one group message: the services it requires first (started or reused, each
  awaited until ready), then its processes in order, each after the previous one is ready when it declares
  `ready`; `stop: together` ends the group when one member ends. Tasks get `TERM=xterm-256color` and
  `COLORTERM=truecolor`; a signal death reports `128 + signal`.
- **Readiness.** The daemon watches each task's output for `http://(127.0.0.1|localhost|0.0.0.0):<port>`
  (the whole printed address with `printed`) and publishes the ready address in the session; views and
  `--wait-ready` only consume it.
- **Output.** Each session keeps a 2 MiB ring in memory and rotating log files (2 x 8 MiB) under
  `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/logs`; a replay restores the terminal modes the child was in and, for
  a full-screen program, the screen from the moment it took the screen. `semio logs` reads the same files,
  also when no daemon runs.
- **Flow control.** Every view reads at its own cursor; a view that cannot keep up resynchronises from the
  log instead of being disconnected, and never slows another view or a task. Input to a task is queued and
  retried. At most 16 views are attached and 128 sessions are retained.
- **Journal.** Lifecycle events and their session projections are persisted locally
  (`events.jsonl`, compacted) beside the daemon's endpoint and pid files. After a daemon restart completed
  sessions stay listed and sessions that were alive are marked interrupted.
- **Instances.** `SEMIO_DASHBOARD_INSTANCE=<name>` selects a second, isolated daemon of the same workspace
  root: its own pipe or socket key and its own folder `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/instances/<name>`
  (endpoint record, pid, lock, `events.jsonl`, `logs`). Every `semio` verb run with the variable talks to that
  instance, so a smoke test on the real workspace never touches the developer's daemon. Letters, digits `.`
  `_` `-` only (others become `_`), at most 32 characters; unset or empty is the default instance.
- **Connections.** Up to 64 connections are served and at most 16 of them are attached views (`Attach`);
  command-line verbs (`tasks`, `run`, `logs`, `stop` …) never need a view slot. A refused view gets the error
  `view_limit`, which the view shows in its language in the status line before it retries. `semio tasks` reads the
  journal without a daemon only when none runs; a running daemon's journal is never shown as interrupted.
- **Windows programs.** A bare program name resolves to a real executable (`bun.exe`) on the task's `PATH`
  before a `.cmd`/`.bat` shim. `cmd /c "<string>"` is started as `cmd /s /c "<string>"`, so the string reaches the
  shell exactly as written. Interrupting a batch job answers its "Terminate batch job (Y/N)?" prompt.
- **Stopping.** Interrupt, then terminate (Ctrl+Break on Windows) after two seconds, then kill after four;
  `Ctrl+B Q` or `semio daemon stop` shuts the daemon down after ending every task. Closing or detaching a
  view never ends a task, and a disconnection does not transfer process ownership to a view.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-dashboard` (binary `semio`, entry point in
  `🚪️entrypoint`), build project `@semio-tech/repo-dashboard-rs` with the `build`, `install`, `run`,
  `daemon` and `preferences` targets behind the root `dashboard` scripts
- `📦️installation` — the installation record, the immutable executable publication and the
  native launch the build runner bootstrap wrapper takes for `run`, `daemon` and `preferences`

The repo command line and the repo MCP server (`⌨️cli`) do not depend on this crate, so a compile
break in the dashboard or the terminal UI never reaches them.

The crate is a family of producer crates, one per layer, each rooted at the folder that states what it is; every folder keeps its
own `📦️packages/🦀️rust` package. From the bottom: `🧱️invocation-catalog` (the invocation, the plugin and playground catalog, the dev environment and the
content hash), `🧾️launch` (the launch contract), `🏛️repo-domain` (the repo actions), `🎮️registry` (commands, declarations and resolution),
`📚️inventory` (reads the sources, keeps the snapshot, runs the background discovery; it also mounts `🌳️command-tree`, the registry
projected as launcher rows and wizard steps, and runs the laws of the registry through a real discovery), `🌀️daemon/🧵️wire` (the frames,
the transport, readiness, replay, journal, lease and `📎️connection`), `🌀️daemon` (the views, groups, supervisor, control surface and
lifecycle), `⚙️preferences`, `🖥️terminal/🌐️labels` (every string, the tab texts and the window titles), `🖥️terminal` (the launcher, windows,
panes, sessions and controls) and `🧭️cli` (the verbs above). The package of `📦️packages/🦀️rust` keeps the dispatch of `semio`,
`📇️playground-catalog`, `🔌️plugin-registry`, `⌨️usage`, the `🌀️daemon/⌨️command` verb and `🚪️entrypoint`.

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; the recorded no-oracle decisions live in `🔮️oracles/🔣️.json`.

`🌳️command-tree-projection` pins the projection of a frozen workspace. `🎮️registry` runs the frozen
workspace `./🧪️tests/🧫️fixtures/🎮️registry/🏗️workspace.json` through the `semio` binary (`commands --json`,
`commands --check`, `run --dry-run`) and compares every result with an independent TypeScript resolver
written from the declaration rules, validates declarations and outputs with Ajv against the registry schema,
and checks the XXH3 vectors of the graph-reuse decision against Bun's own hash:
`bun repo run @semio-tech/repo-dashboard-rs:test` builds the debug binary and runs it. The scenarios of
`🧫️fixtures/🎮️registry/🥒️.feature` are mapped to Rust tests of `🎮️registry`.

`🌀️control-plane` defines language-neutral lifecycle scenarios and protocol vectors. The owned
Rust codec and the existing Ajv library validate the same vectors. The quick suite runs actual
Bun PTYs, Nx server/build/test targets, output replay, cancellation, restart, process-tree
termination, singleton enforcement and reattachment: `bun repo run @semio-tech/repo-dashboard-rs:test -- quick`.

`🧊️execution` pins the native launch vectors of `./🧪️tests/🧫️fixtures/🧊️execution`, the build-runner scheduling of
`run` and `install`, the immutable installation that keeps a running executable's bytes while
a new build is published, and the source digest that decides whether an installation is stale (the fixture names the
files that must and must not change it): choose `@semio-tech/repo-dashboard-rs:test-execution` in New Task.

After installing, set `SEMIO_TEST_CLI` to the printed native executable and run the dashboard
test target with `--args='-- --include-ignored --nocapture'` to include native startup,
attach/detach and launcher tests. The launcher test types a search into an actual pseudo-terminal,
runs the selected command to its exit code and shuts the daemon down with `Ctrl+B Q`. `SEMIO_TEST_BUN` selects the repository-pinned Bun executable;
`SEMIO_TEST_ARTIFACT_DIR` keeps temporary fixture workspaces in the ticket's generated directory.
The native regression covers both a wrapper with redirected output and the actual Nx native
command runner. The test bridge captures its process search path before Cargo adds test DLL paths.

Shared preference and launcher fixtures are checked by Rust, existing Ajv and independent
JavaScript projections. Native first-frame timing is measured through an actual pseudo-terminal.
