---
name: repo
kind: infrastructure
summary: Monorepo tools that are ai-ready.
---

## ✨️ Features

- 🤖️ AI integrations
  - ✅️ Currently
    - 🧑️‍✈️ Copilot
    - 🌊️ Cascade
    - 🖱️ Cursor Agent
    - ❄️ Claude Code
    - ⚙️ Codex
  - 📅️ Possible
    - 🦾️ Droid
    - …
- 🎛️ Dashboard developer operations
  - Setup, build, test, check, serve and debugging through registered commands
  - Shared task overview, output, progress, restart and cancellation
  - Passive source navigation and editing through an installed editor
- 🗣️ Language integrations
  - ✅️ Currently
    - 🟦️ Typescript
    - 🐹️ Go
    - 🐍️ Python
    - 🦀️ Rust
    - 🟣️ C#
  - 📅️ Possible
    - ♨️ Java
    - 🟪️ Kotlin
    - ➕️ C++
    - © C
    - 🐘️ PHP
    - 💎️ Ruby
    - 🐦️ Swift
    - …
- 📦️ Sandbox integrations
  - ✅️ Currently
    - 🐋️ Devcontainers
  - 📅️ Possible
    - 🦭️ Podman
    - …
- 👥️ Tracker integrations
  - ✅️ Currently
    - 🐙️ GitHub
  - 📅️ Possible
    - 🦊️ GitLab
    - 🪣️ Bitbucket
    - …

### 🤖️ AI

| System          | Agents | Skills | Hooks |
| --------------- | :----: | :----: | :---: |
| 🧑️‍✈️ Copilot      |   ✅️   |   ✅️   |  ✅️   |
| 🌊️ Cascade      |   ✅️   |   ✅️   |  ✅️   |
| 🖱️ Cursor Agent |   ✅️   |   ✅️   |  ✅️   |
| ❄️ Claude Code  |   ✅️   |   ✅️   |  ✅️   |
| ⚙️ Codex        |   ✅️   |   ✅️   |  ❌️   |
| (🦾️ Droid)      |   ✅️   |   ✅️   |  ✅️   |

###

## 🥇️ Why repo is a game changer

### 📈️ Requirements + Docs + Stats + Semantics

#### 🚀️ Agents love TDD

[Test-Driven-Development (TDD)](https://en.wikipedia.org/wiki/Test-driven_development) is a game changer for agents 🚀️

##### ↔ Multi-lanugage development

Ever had the problem that you domain-expert devs write Python/C++/C/Javascript/Ruby/Lua/… but not Typescript/Go/Rust/C#/Julia/… or vice versa?

No problem, let them write what they know, until the tests are extended and let agents reimplement it natively until the tests succeeds ✅️

## 😥️ What, you'll have to abandon

### ❌️ No granular files, only godfiles

### ❌️ No normal docstrings

### ❌️ No inline comments

## 😲️ What, you'll get

### 🚀️ Zero-touch development

### 🧪️ Shared test infrastructure

### 💯️ Consistent requirements

### 📑️ Conistent docs

### 🔮️ Future proof infrastructure

### 📊️ Meaningful stats

## Developer Operations

Start `bun run dashboard` at the monorepo root. Choose **New Task** (`Ctrl+B n`),
search by action and owner, complete the command form and choose **Start**. The
[dashboard guide](🔨️modules/🎛️dashboard/README.md#-daily-work) gives concrete
setup, testing, debugging, source navigation and repository workflows. Every
command uses the workspace daemon, so its status, output and cancellation remain
available after closing a view or restarting an editor.

Editor integrations provide source navigation and language editing. Executable
commands are maintained in owner dashboard metadata or Nx targets and selected
in the dashboard.

## Configuration

Repo-wide settings live in [`.🧬semio/🦑️repo/config.toml`](../.🧬semio/🦑️repo/config.toml) at the monorepo root.

| Key                  | Default    | Description                                                                 |
| -------------------- | ---------- | --------------------------------------------------------------------------- |
| `logging.session`    | `false`    | Write per-session `session.json` under `.🧬semio/🦑️repo/⚡️/🤖️/…` on agent hooks       |
| `logging.operations` | `true`     | Append derived `agent.<operation>.<phase>` events (requires `session = true`)      |
| `logging.plan`       | `true`     | Track agent plan steps in `session.json` (requires `session = true`)        |
| `logging.detail`     | `standard` | `minimal` (event only), `standard` (+ response), or `full` (+ native stdin) |

Set `logging.session = true` to enable session-file logging for debugging or coordinator ingestion.

# 🏗️ Build and run

`bun repo` is the only build entry. It is tool-neutral: the same command line runs on Nx (default) or Bazel, and every project target is declared once in a `📋️project.json` with the `repo:script` executor. Never call `nx`, `bunx nx`, `npx nx`, raw `cargo` or `rustc` yourself.

```bash
bun repo run <project>:<target>[:<configuration>]       # one task; add --tool=bazel for the Bazel backend
bun repo run-many --targets=<targets> [--projects=<projects>]
bun repo affected --targets=<targets>
bun repo show projects|project <name>|targets            # the neutral graph
bun repo tools setup [nx|bazel|all]                      # provision the pinned build tools for this host
bun repo dependencies sync --all                         # install dependencies (--scope <owner> for one owner)
bun repo dependencies lock --all                         # regenerate bun.lock after a manifest edit
bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib
```

- **Tool selection.** Nx is the default (`defaultTool` in `📚️library/🏗️build/🔣️policy.json`). `--tool=bazel` or `SEMIO_BUILD_TOOL=bazel` selects Bazel, nested tasks inherit it. The shared flags `--skip-cache`, `--parallel`, `--configuration` and `--output=stream` are translated by the selected backend. `bun repo tools setup all` provisions both; `workspace:setup` includes it.
- **Dependencies.** `dependencies sync` installs into the dependencies partition (`node_modules` at the root is a symlink into it). Any `package.json` or `bun.lock` edit makes the installed generation stale: run `dependencies lock --all`, then `dependencies sync --all` once, never an ad-hoc `bun install`.
- **Cargo.** Raw `cargo` and `rustc` are refused by the compiler ingress gate in `.cargo/config.toml`. Compile through `bun repo cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo subcommand> <args>`; without `--manifest-path` the root workspace is targeted. Never create a private `CARGO_TARGET_DIR`: the shared store is a fixed-capacity partition and parallel private targets exhaust the disk.
- **Zero-touch.** The build CLI must boot on a fresh clone with no `node_modules`, no installed tool and no cache image, because the dependencies live inside a cache partition the CLI itself creates. Everything in the CLI import closure (`📚️library/🏗️build/📜️script.ts`) therefore imports by relative path, never through a bare `@semio-tech/*` specifier, and the `dependencies` command and the environment bootstrap run before the dependencies partition is pinned. After editing a library module, check `bun repo --help`: it must print the usage line. The same bootstrap must work unchanged in the devcontainer, native Windows, native macOS and native Linux.

## 💾️ Cache partitions

All build state lives in fixed-capacity partitions below `.cache/b/<partition>` (git-ignored, derived, safe to delete when nothing runs). On macOS each partition is a UDIF disk image formatted as case-sensitive APFS, `fixed.dmg`, mounted at `m` next to its `image.json` receipt (HFS+ is not usable: it lacks the `RENAME_SWAP` that publication and binding need). Linux and Windows use the first-party fixed-capacity backend of the same storage layer instead of a mounted image (the `hdiutil` steps below do not apply there; stop all processes and remove the partition directory instead). Capacities come from `📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`.

| Partition | Content | Capacity |
|---|---|---|
| `d` | dependencies, including `node_modules` generations | 8 GiB |
| `t` | tools: rustup, Bazel, Nx | 8 GiB |
| `g` | neutral build graph and inference records | 4 GiB |
| `o` | observations and diagnostics | 2 GiB |
| `p` | products: build outputs and published artifacts | 24 GiB |
| `l` | leases and locks | 64 MiB |
| `n` | Nx cache pool, namespaces are funded child images | 1.5 GiB |

`.cache/b/c` is the plain control store (sqlite claims, no image). The Rust target and build directories are a separate fixed partition at `.🧬semio/🦑️repo/⚡️cache/rust/fixed` (16 GiB). `bun ./📜️script.ts clean` never touches any of them, nor the `🗑️generated` folder of an open ticket.

### 🩹️ Recovery

Check these before touching code when every `bun repo` call refuses at startup. Stop all `bun repo`, cargo and Nx processes first.

1. **"Pending Native image publication must settle"**: a killed run left `.cache/b/<partition>/.publication/intent.json`. Confirm no process is alive, move the file aside, rerun.
2. **Receipt or mount mismatch** ("Exact canonical persistent Native publication required", "Ambiguous Native backing kernel association", an `image.json` without its `fixed.dmg`): recreate the partition.
   ```bash
   hdiutil detach .cache/b/<partition>/m || umount -f .cache/b/<partition>/m   # diskutil may report it unmounted while it is not
   hdiutil info                                  # no image may still claim the path
   rm -rf .cache/b/<partition>
   bun repo --help && bun repo tools setup all   # the next call recreates it
   ```
   Never `mv` a partition directory while it is mounted: the image keeps its old path and the association becomes ambiguous.
3. **Nested attach "Permission denied"**: the parent was mounted `noowners`. With nothing open (`lsof +f -- .cache/b/<partition>/m` is empty) run `hdiutil detach` on the parent; the next call re-attaches with `-owners on`.
4. **Poisoned Nx pool** (every graph or Nx call fails "Exact original parent child allocation required"): `hdiutil detach .cache/b/n/m/<shard>/native/m`, then `rm -rf` the shard directory.
5. **ENOSPC in `d`**: empty `.cache/b/d/m/tmp` (installer leftovers), then rerun `dependencies sync --all`.
6. **Image created before the APFS switch** (ENOTSUP on rename): recreate it as in step 2.

Recreating `d` costs one full `dependencies sync --all` (about 16 minutes with a warm package cache), recreating `g` one cold graph inference (7 to 9 minutes).

## 🎛️ Developer Dashboard

The [dashboard daily workflow](🔨️modules/🎛️dashboard/README.md#-daily-work) is the developer entry point. Choose **New Task**, search for the operation and its owner, then complete the typed form. The preview shows its arguments and environment before **Start**. Output, progress, readiness, source navigation, interactive debugging, cancellation and restart remain attached to that task.

Declare executable operations beside their source: build targets in `📋️project.json`, typed controls in `metadata.semio.dashboard`, and authored ticket tools in the ticket's `🎮️commands.json`. Playground controls come from `[[package.metadata.semio.playground]]` in crate manifests; native crate targets come from `📋️build-owners.json` and the neutral build graph. Parameter choices, environments, readiness and compound membership belong to these owner declarations. Editors provide source navigation and editing; the dashboard owns execution.

## 🐹️ Go layout

Go code follows the canonical taxonomy layout instead of package-root files:

- A member is a directory with one `🐹️.go` leaf: `<module>/📦️packages/🐹️go/<member>/🐹️.go`. The member name is registered in `📚️library/🔣️taxonomy.json` (`members-of-members-of-members-of-modules`).
- White-box tests are cases at the language-neutral owner: `<module>/🧪️tests/<case>/🐹️.go`, absorbed into the package when the package clause is the module's or `<package>_test`. Black-box adapters are never absorbed.
- Because files are not at the package root, plain `go test` and `go list` do not see the members. Run Go through the package `📜️script.ts` (`runCanonicalGoTests`) or `bun repo run <project>:test`, which pass the merged source overlay.

# 🔨️ Modules

- [`💻️client/⌨️cli`](🔨️modules/💻️client/⌨️cli) – Command line tool for monorepo interactions
- [`💻️client/🔌️mcp`](🔨️modules/💻️client/🔌️mcp) – MCP server the agent clients speak to
- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode) – Passive source navigation and language editing; the dashboard exclusively controls developer operations
- [`💻️client/🪶️sqlite`](🔨️modules/💻️client/🪶️sqlite/README.md) – SQLite schema of the client's local-only entity store
- [`🖥️server/🎛️coordinator`](🔨️modules/🖥️server/🎛️coordinator/README.md) – Coordinator REST API and event log
- [`🖥️server/🧬️schema`](🔨️modules/🖥️server/🧬️schema) – PostgreSQL schema of the shared server state
- [`📚️library`](🔨️modules/📚️library) – Taxonomy, discovery, normalization and the derived schema catalog
- [`🧪️test`](🔨️modules/🧪️test/README.md) – Cross-module test harness
