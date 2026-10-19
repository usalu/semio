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

`bun repo` is the only build entry. It is tool-neutral: every project target is declared once in a `📋️project.json`, a neutral graph describes all projects and targets, and two interchangeable implementations execute them: **Nx with Bun** (default) and **Bazel** (`--tool=bazel`). Never call `nx`, `bunx nx`, `npx nx`, `bazel`, raw `cargo` or `rustc` yourself. Paths that start with `📚️library/` are below `🔨️modules/` of this product. A German version of this chapter follows its last section.

```bash
bun repo run <project>:<target>[:<configuration>]       # one task; add --tool=bazel for the Bazel implementation
bun repo run-many --targets=<targets> [--projects=<projects>]
bun repo affected --targets=<targets>                    # projects that changed against HEAD, plus their dependents
bun repo exec --projects=<projects> -- <command>         # a command once per project, project root as working directory
bun repo run <project>:<target> --dry-run                # print the task graph, run nothing
bun repo show projects|project <name>|targets            # the neutral graph (also: bun repo graph)
bun repo cache status|clear                              # task cache of the selected tool
bun repo tools setup [nx|bazel|all]                      # provision the pinned build tools for this host
bun repo dependencies sync --all                         # install dependencies (--scope <owner> for one owner)
bun repo dependencies lock --all                         # regenerate bun.lock after a manifest edit
bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib
```

- **Help.** `bun repo --help` prints every command and every flag; `bun repo <command> --help` (also with a subcommand, for example `cache clear --help`) prints that command's own usage, options and examples. The text is rendered from the operation catalogue in English or German: `--lang=en|de`, then `SEMIO_LANGUAGE`, then the locale (`LC_ALL`, `LC_MESSAGES`, `LANG`), then the first language of the catalogue. Help is answered before any tool or store starts.
- **Dependencies.** `dependencies sync` installs into the dependencies partition (`node_modules` at the root is a symlink into it). Any `package.json` or `bun.lock` edit makes the installed generation stale: run `dependencies lock --all`, then `dependencies sync --all` once, never an ad-hoc `bun install`. Before it installs, a sync retires every `j/<20 hex>` dependency generation that is neither a current plan generation nor the target of a live binding (progress `dependencies:retire-obsolete:<n>`). On macOS 27 the installer can fail with `EPERM: Operation not permitted: failed to link package`; see item 7 of Recovery, the sync repairs that case itself.
- **Cargo.** Raw `cargo` and `rustc` are refused by the compiler ingress gate in `.cargo/config.toml`. Compile through `bun repo cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo subcommand> <args>`; `--store` and `--configuration` are mandatory and the Cargo subcommand follows `--`; without `--manifest-path` Cargo reads the `Cargo.toml` of the working directory (the root workspace when called from the root; `🏃️process/🦀️cargo/🚪️launcher`). Never create a private `CARGO_TARGET_DIR`: the shared store is a fixed-capacity partition and parallel private targets exhaust the disk.
- **Zero-touch.** The build CLI must boot on a fresh clone with no `node_modules`, no installed tool and no cache image, because the dependencies live inside a cache partition the CLI itself creates. Everything in the CLI import closure (`📚️library/🏗️build/📜️script.ts`) therefore imports by relative path, never through a bare `@semio-tech/*` specifier, and the `dependencies` command and the environment bootstrap run before the dependencies partition is pinned. After editing a library module, check `bun repo --help`: it must print the usage line. The same bootstrap must work unchanged in the devcontainer, native Windows, native macOS and native Linux (see Setup per platform below).
- **Cancellation and progress.** The catalogue below declares every operation cancellable (`cancellable`) and every operation except `graph` and `show` as reporting progress (`progress`) in `📚️library/🏗️build/⚙️operations/🔣️catalog.json`; the implementations pass the caller's process control to every path (`runOwnedBuildInLeasesV1` in `📚️library/🏗️build/📜️script.ts`). Not yet verified: no per-operation cancellation run has been recorded.

## 🧭️ Neutral model

| Layer | What it is | Source |
|---|---|---|
| Manifest | One `📋️project.json` per project: targets with executor, command, inputs, outputs, `cache`, `continuous`, `dependsOn`. | schema `📚️library/🏗️build/🧬️schema/📋️project/🔣️.json` |
| Policy | `defaultTool`, the named input groups (`default`, `production`, `sharedGlobals`, `cargo`, `go`, `uv`, `dotnet`, `schemaSources`, `nativeSources`, `nativeTestSources`) and the files that may change the graph. | `📚️library/🏗️build/🔣️policy.json` |
| Graph | `BuildGraphV1`: projects, targets and dependency edges inferred in TypeScript from the manifests, Cargo, Go, `package.json` and `bun.lock`; independent of Nx. Snapshots live in the graph partition `g`. `show` and `graph` print it. | `📚️library/🏗️build/🕸️graph` |
| Operations | The catalogue of operations, which tool implements each one how, and the JSON shapes their results must have. | `📚️library/🏗️build/⚙️operations/🔣️catalog.json`, schema `⚙️operations/🧬️schema/🔣️.json` |
| Implementations | Nx adapter and Bazel adapter; each one only translates the neutral request into its tool. | Sections Nx implementation and Bazel implementation below. |

**Tool selection.** `--tool=nx|bazel` on the command line wins, then the environment variable `SEMIO_BUILD_TOOL`, then `defaultTool` of the policy (`nx`). The selected tool is exported as `SEMIO_BUILD_TOOL` to every nested task, so a task that calls `bun repo` again keeps the tool; the dashboard axis `build-tool` sets the same variable (see Command registry). Anything else is refused ("Unknown build tool"). Choose the compiler configuration with `--configuration=prototype|release` (or `-c`, a `:<configuration>` suffix on the task, or `SEMIO_BUILD_MODE`); disagreeing values are refused ("Conflicting build modes").

**Operations.** The state of each operation per tool: `native` = the tool's own command, `adapter` = our code drives the tool, `neutral` = tool independent, `missing` = not implemented and the call refuses.

| Operation | Nx | Bazel | `--dry-run` | Notes |
|---|---|---|---|---|
| `run <project:target[:configuration]>` | native | adapter | yes | A dry run executes nothing. |
| `run-many --targets=… [--projects=…]` | native | adapter | yes | |
| `affected --targets=…` | native | adapter | yes | Changed files are the tracked changes against `HEAD` plus untracked files; `--base`/`--head`, `--files`, `--uncommitted`, `--untracked` narrow them and `--all` selects every project. Affected projects (the same rule for both tools, asserted against Nx's own locators): the deepest project root that owns a changed file, projects whose target inputs name a changed `{workspaceRoot}/…` file, every project when `nx.json` changes, then all dependents. `--projects` is refused for every tool; use `run-many` to narrow. |
| `exec --projects=… -- <command>` | native | adapter | yes | Once per selected project, working directory is the project root, and the tool-neutral pair `SEMIO_PROJECT_NAME` (project name) and `SEMIO_PROJECT_ROOT` (project root relative to the workspace) is exported by both tools; the Nx implementation maps the names its own `exec` exports (see Nx implementation). Under Bazel each project gets an uncached `repo-exec` task. `--dry-run` prints the `repo-exec` plan and runs nothing. |
| `watch --projects=… -- <command>` | neutral | neutral | no | Reruns the command when selected source files change. |
| `graph`, `show projects\|project <name>\|targets` | neutral | neutral | no | |
| `reset` | adapter | adapter | no | Nx: invalidates the graph and runs `nx reset`. Bazel: invalidates the graph and empties `bazel/actions` (the disk cache), `bazel/outputs` and `bazel/workspaces` of the products partition. |
| `tools setup [nx\|bazel\|all]` | adapter | adapter | no | Without an argument the selected tool is set up (`--tool`, default `nx`). See Setup per platform below. |
| `preflight` | neutral | neutral | no | Checks the host before any store is provisioned, from `📚️library/🏗️build/🩺️preflight/🔣️policy.json`: CPU architecture, 8 GiB free, required tools (`cc` and `pkg-config` on Linux, `cc` and `hdiutil` on macOS) and, on Linux, atomic directory exchange (`renameat2` `RENAME_EXCHANGE`) on a case-sensitive workspace file system. |
| `dependencies sync\|lock --all\|--scope <owner>` | neutral | neutral | no | |
| `cargo …` | neutral | neutral | no | |
| `cache status\|clear` | adapter | adapter | no | Nx: the artifact directory of the rust partition. Bazel: the disk cache of the products partition. See the two implementation sections. |

**Flags.** Flags before `--` are portable and validated before any tool starts; an unknown flag, a stray positional argument or an invalid selector is refused. Everything after `--` is passed literally to the task. The flag list is part of the catalogue and is printed by `bun repo --help`; the task commands (`run`, `run-many`, `affected`, `exec`, `watch`) accept all flags of the table, but a flag only has an effect where the last column names its command.

| Flag | Meaning | Commands |
|---|---|---|
| `--tool=nx\|bazel` | Select the implementation (otherwise `SEMIO_BUILD_TOOL`, then the policy default). | all |
| `--help`, `-h`; `--lang=en\|de` | Print the usage of the command in English or German and stop. | all |
| `--projects`, `-p`, `--exclude` | Comma lists of project names with `*` wildcards, `tag:<tag>`, `name:<name>`, `directory:<root>`, `!<pattern>` negation. | `run-many`, `exec`, `watch`; `--exclude` also `affected` |
| `--targets`, `--target`, `-t` | Target names or `*` patterns. | `run-many`, `affected` |
| `--configuration`, `-c` | Compiler configuration (`cargo` takes the full name only). | `run`, `run-many`, `affected`, `cargo` |
| `--parallel=<n>`, `--bail`, `--verbose`, `--output=stream\|stream-without-prefixes\|static` (alias `--output-style`) | Concurrency, stop at first failure, detail, output style. | `run`, `run-many`, `affected` |
| `--skip-cache`, `--skip-remote-cache`, `--exclude-dependencies` (alias `--exclude-task-dependencies`) | Ignore the task cache, ignore the remote cache, run only the named tasks. | `run`, `run-many`, `affected` |
| `--dry-run`, `--graph[=<file>]` | Print the task graph as a `DryRunPlan` on stdout and execute nothing. `--graph` is the same flag on both tools; with a file the plan is written there (Nx: the file as given, Bazel: a report whose path is printed). | `run`, `run-many`, `affected`, `exec` |
| `--all`, `--base`, `--head`, `--files`, `--uncommitted`, `--untracked` | Change selection (`--all`: every project). | `affected`; `--all` also `watch` and `dependencies` |
| `--include-global-files` | `watch` also reacts to files outside any project. | `watch` |
| `--file=<path>`, `--json` | Write the result as a report file; print lists as JSON. | `graph`, `show` |
| `--scope <owner>` | Restrict the operation to one dependency owner (instead of `--all`). | `dependencies` |
| `--store=shared\|isolated`, `--parallelism=<cores>` | Cargo target store and core count. | `cargo` |

**Result shapes.** Results that scripts consume are specified by the schema `⚙️operations/🧬️schema/🔣️.json`:

- `DryRunPlan`: `{"tasks":{"roots":[<id>],"tasks":{<id>:{"id":…,"target":{"project":…,"target":…,"configuration"?:…}}},"dependencies":{<id>:[<id>]},"continuousDependencies"?:{<id>:[<id>]}}}`. A task id is `<project>:<target>[:<configuration>]`; `exec` uses the target `repo-exec`. The Bazel plan adds `selected` (the requested task ids) and lists dependencies on continuous tasks apart in `continuousDependencies`, because those only have to be running.
- `CacheStatus`: `{"version":1,"tool","available","directory","entries","bytes","activity"?:{"builds","actions","executed","cacheHits"}}` (`activity` is filled by Bazel). `CacheClear`: `{"version":1,"tool","cleared","removedEntries","removedBytes"}`.

## 📋️ Manifests

A task in `targets` of a `📋️project.json` uses these fields:

| Field | Meaning |
|---|---|
| `executor` | `repo:script` (default, runs `command` through the repo process authority) or `repo:noop` (ordering only, runs nothing). |
| `command`, `options.command`, `options.commands`, `options.args` | What runs; `args` are literal arguments that are never shell-interpolated. `configurations.<name>` may override `command`, `commands`, `cwd`, `env`, `args`. `defaultConfiguration` names the configuration used when none is requested. |
| `dependsOn` | Tasks that must finish first: `"target"` (same project), `"^target"` (dependency projects), `"project:target"`, or `{target, projects, dependencies, params}`. |
| `inputs` | What the result depends on, see below. |
| `outputs` | Paths the task writes, relative tokens `{projectRoot}`, `{workspaceRoot}` and option tokens such as `{options.env.SEMIO_BUILD_MODE}`. `[]` means "no files to restore". |
| `cache` | Boolean, see the classes below. |
| `continuous` | The task is a long-running service. |
| `metadata.semio.dashboard.ready` | How readiness of a continuous task is detected: the output shows `http://(127.0.0.1\|localhost\|0.0.0.0):<port>` and that address answers 2xx or 3xx; `port` or `portEnv` declares the port, `path` appends a path, `printed` takes the whole printed address. |

**Inputs.** A list of file globs (`{projectRoot}/**/*`, `{workspaceRoot}/…`, `!` excludes), named groups from the policy (`default`, `production`, `cargo`, …), `^<group>` (the same group of the dependency projects), and objects: `{"env": "NAME"}`, `{"runtime": "<command>"}` (the command's output), `{"externalDependencies": [...]}` and `{"dependentTasksOutputFiles": "<glob>", "transitive": true|false}` (the outputs of the tasks in `dependsOn`). Declare everything the task reads that is not reachable from its own project: data outside the project, spawned test files, tool and toolchain files, environment. A project can add its own `namedInputs` (for example `reads`, `testEnv`, `cargoToolchain`). `default` of a project replaces the workspace `default`, so re-add `sharedGlobals` (platform, Bun version, `SEMIO_BUILD_MODE`) there.

**Cache classes.** `cache` is the switch both tools read. Next to it the inference attaches a machine-readable decision `metadata.semio.cache = {"version":1,"class":…,"reason":…}` to every target it forces, synthesizes or refuses to replay (closed schema `📚️library/🏗️build/🏃️execution/🧬️schema/💾️cache.json`). The class names the physical fact that allows or forbids a replay:

| Class | Reasons | Meaning | Cached |
|---|---|---|---|
| `deterministic` | `pure-function-of-inputs`, `check-without-outputs` | A pure function of its declared inputs; every declared output is a plain restorable tree, or there is none. | yes, the only replayable class |
| `driver-owned` | `freshness-owned-by-driver` | The command compiles through the build driver, whose Cargo freshness and stage cache are the authority (playground `prepare-*` and `build-*-react*`, `component`). | no |
| `managed-output` | `outputs-are-store-links`, `outputs-in-managed-store` | Outputs live in the leased products store behind a binding or manifest protocol (TypeScript artifact `build`, wasm slots) and the manifest does not author `cache: true`; restoring a copy would be refused or bypass the store. With `cache: true` the target is `deterministic` instead, because its receipt is the restorable output (see Store receipts). | no |
| `continuous` | `serves-until-stopped` | Runs until stopped. | no |
| `side-effecting` | `publishes-live-state`, `publishes-remote`, `mutates-workspace` | Mutates state outside its declared outputs. | no |
| `live` | `drives-running-process` | Observes or drives a running process (names with `e2e` or `live`). | no |
| `aggregate` | `orders-prerequisites` | Orders prerequisites and does no work of its own. | no |
| `diagnostic` | `admission-refusal`, `measures-environment` | Reports a refusal or a measurement and is always re-evaluated. | no |

The Bazel adapter reads the decision (`bazelTargetCacheClassV1` validates it against the closed schema and refuses an invalid one with "Closed repository cache decision required") and derives its cache decision from the class, see the Bazel cache decision below. The Nx projection does not read it; Nx decides from `cache`, `continuous` and `outputs`. A manifest does not write it; the synthesized playground targets, the TypeScript artifact `test`, `check` and (with `cache: true`) `build` targets, and the wasm slot producers with `cache: true` carry `deterministic`, targets cached by the name rules below carry none. The policy that sets `cache` and the decision follows this order, the first matching row wins (`📚️library/🏗️build/🕸️graph/📥️inference/🎯️target-policy`, names from `📚️library/⚡️caching/🔣️policy.json`):

| Order | Rule | Cached | Decision attached |
|---|---|---|---|
| 1 | `continuous: true`, or a name `dev`, `serve`, `watch`, `start`, `test-watch`, `daemon` (also `<name>-…`) | no | `continuous` |
| 2 | `cache: false` in the manifest | no | none |
| 3 | A name of the `uncached` list (`audit`, `bench`, `clean`, `deploy`, `deps`, `install`, `publish`, `seed`, `setup`, `test-e2e`, `update`, …) or an exact name (`format`, `commit`, `os`, `parity`, …); `cache: true` is overridden | no | `live` (`e2e`, `live`, `os`, `semio`), `diagnostic` (`audit`, `bench`, `doctor`, `parity`, …), `side-effecting` (`publish`, `deploy` as `publishes-remote`, otherwise `mutates-workspace`) |
| 4 | `hub-live-catalog-check`, `hub-live-catalog-native-check`, `hub-live-catalog-oracle` | yes | none (exceptions to row 5) |
| 5 | A dash-separated word `clean`, `gc`, `prune`, `setup`, `fuzz`, `e2e` or `live` in the name | no | as row 3 |
| 6 | A deterministic family: `test`, `check`, `lint`, `typecheck`, `format-check`, `generate`, `schema`, `verify`, `build`, `wasm`, `native-build`, `package`, `*-contract`, … | yes | none; default inputs `default` and `^default`, outputs `[]` unless the manifest says otherwise |
| 7 | Anything else | **yes** unless `cache: false` | none |

Row 7 is fail-open under Nx (Bazel additionally requires an `outputs` array and a class that allows replay, see its cache decision): a target that touches host state and says nothing is cached, and a cache hit then skips its effect. Write `cache: false` for these reasons (the convention across the manifests, mirrored by the classes above): tests that exercise real storage, lease, process or toolchain state; live host, browser or server runs; writers that rewrite the tree or a store outside their declared outputs; launchers of long-lived tools; network downloads; steps whose input set is unbounded (digests of the whole workspace).

**Store receipts.** Products that are published into a leased, content-addressed store instead of the project directory (Cargo artifacts in the compiler store, Vite sites, wasm packages, TypeScript artifacts) cannot be restored by copying files into that store from outside. They declare a receipt as their only output, `{projectRoot}/.🧾️store/<target>.json` (schema `📚️library/🏗️build/🧾️store-receipt/🧬️schema/🔣️.json`, git-ignored, deliberately not below `dist`, which is a link into the store for TypeScript artifacts). The receipt names each published store path (`entries`: kind, bytes, files, SHA-256; a directory digest hashes its sorted file list) and each link that binds a project path to a store path (`links`). The central publishers write it after publication, so no build script changes: the artifact destination that the Vite and wasm publishers write through (`⚡️caching/📦️artifacts/🟦️.ts`), the compiler store publisher of Cargo `build` (`⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`, skipped when an observation output is requested) and the TypeScript artifact binder (`⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🟦️.ts`). Both adapters hand the receipt location to the one task that declares it (`SEMIO_TASK_RECEIPT`, issued by `issueStoreReceiptV1` in `📚️library/🏗️build/🧾️store-receipt/🟨️.mjs`; Nx: `⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs`, Bazel: `executeTask` in `📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`); every other task has it unset, so a nested task never records into its parent's receipt, and a publisher does nothing without it (`recordStoreContentV1`).

After every `run` or `run-many`, both tools verify the receipt of each selected task and of its prerequisites. A receipt that is absent, damaged or points at a missing or changed store entry counts as a miss: that one task reruns alone and uncached (Nx: `run <task> --skip-nx-cache --excludeTaskDependencies`, the one extra child selection the caller admits, which also replaces the cache entry; Bazel: in-process with `skipCache` and without dependencies, which leaves the stale disk-cache entry in place, so a later hit repairs again) and is verified again, and the invocation fails closed otherwise. A failed first run is retried once after the repair. A task that declares a receipt but whose handler wrote none is an error, not silently accepted. TypeScript artifact `build` and wasm slot producers carry the receipt as their output: they replay when the manifest authors `cache: true` (decision `deterministic`) and stay `managed-output` otherwise. A target declares a receipt only when its `outputs` list the receipt path (`declaredStoreReceiptV1`): `check`, `test` and `lint` publish a verdict, not store content, and `publish` and the course `build` list other outputs or none. Alternating `prototype` and `release` runs of one target share one receipt file (its path has no mode), so a switch is expected to rerun the target once (inferred from the path rule, not observed). Not yet verified: no run has exercised the repair path (first proof: run `@teaching/proctor:build` twice, delete the compiler `published/<hash>` slot, run again and expect one `[store-receipt] re-executing` line, with `--tool=nx` and `--tool=bazel`).

The Bazel implementation decides per action from `cache`, `continuous` and `outputs` and is stricter about missing declarations (see its cache decision below).

## 🔌️ Nx implementation

Nx is the default (`defaultTool`). The repository does not use Nx project discovery; it feeds the neutral graph into Nx.

- **Plugin.** `nx.json` registers the plugin `@repo/emoji-project-json` (`📚️library/🟨️.mjs`) for `📋️project.json`. Its `createNodesV2` publishes the neutral graph as Nx projects and its `createDependencies` publishes the neutral edges; it refuses edges whose projects it does not know. Each target is projected onto an Nx executor and receives a private engine identity as an additional input, so a change of the executing implementation invalidates cached results.
- **Executors.** `@semio-tech/repo-lib:command` and `@semio-tech/repo-lib:owner-command` (`📚️library/📦️packages/🟦️typescript/📋️executors.json`) `command` runs the literal command arguments through the original process authority and the Bun shell, `owner-command` forwards the literal argv to the native or process owner without shell interpolation (descriptions in `executors.json`); `repo:noop` becomes `nx:noop`.
- **Translation.** `--skip-cache` becomes `--skip-nx-cache`, `--exclude-dependencies` becomes `--excludeTaskDependencies`, `--output` becomes `--output-style`, `--bail` becomes `--nx-bail`, `--dry-run` and a bare `--graph` (or `--graph=true`) become `--graph=stdout`, `--graph=<file>` is kept. `exec` runs the command in a Bun wrapper that copies the names Nx's own `exec` exports (`NX_PROJECT_NAME`, `NX_PROJECT_ROOT_PATH`) to the neutral `SEMIO_PROJECT_NAME` and `SEMIO_PROJECT_ROOT`; the Nx names exist only in this implementation. `affected` without a selector adds `--base=HEAD`, `--uncommitted --untracked` becomes `--base=HEAD`, `--all` becomes `run-many`. `exec --dry-run` prints the neutral `repo-exec` plan and runs nothing.
- **Boundary.** Production code outside `⚡️caching/🚀️bootstrap` never loads an Nx module. It goes through the typed port `⚡️caching/🚀️bootstrap/🛠️tools/🔌️sdk`, whose closed catalogue names the 13 Nx modules the repository may touch. A test scans every production file under `🦑️repo/🔨️modules` with an own tokenizer and the TypeScript compiler as an independent oracle and requires zero offenders.
- **Cache location.** Nx task results live in `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` (`cacheDirectory` in `nx.json`; the rust partition, 16 GiB, which also holds the shared Cargo target and build directories of `.cargo/config.toml`). The separate partition `n` (`.cache/b/n`, 1.5 GiB) is the Nx pool: it holds the Nx graph cache as funded child images (`⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph`), not the task results. `bun repo cache status` walks the artifact directory and prints `CacheStatus`; `cache clear` runs `nx reset --onlyCache` inside the bounded namespace and prints `CacheClear`.
- **Not yet verified.** The whole flow (a real cold run that fills `nx/artifacts`, then a warm run served from it) is scripted but has not been run at the time of writing; the unit and contract tests of the pieces above passed on 2026-10-19 (`⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations`, `🔌️sdk`, `🔁️probe`, `🏗️build/🧭️selection`, `🏗️build/🧭️invocation`: 23 tests, 0 failures).

## 🏗️ Bazel implementation

`--tool=bazel` (or `SEMIO_BUILD_TOOL=bazel`) selects Bazel 8.4.2 (`.bazelversion`, pinned download per OS and CPU). The adapter is `📚️library/🏗️build/🔧️tools/🏗️bazel`. It turns the selected tasks into real Bazel actions, so Bazel's own action cache, disk cache, optional remote cache and lazy evaluation do the work.

- **Generated workspace.** For each distinct selection the adapter writes a workspace under `<products>/bazel/workspaces/<namespace>`: `MODULE.bazel`, a `BUILD.bazel` file, the rules in `tasks.bzl` (`repo_task`, `repo_launch`, `repo_noop`), a task manifest `<label>.json` and a fingerprint `<label>.input` per action, and `s`, a link to the real checkout (a junction on Windows). A task label is `t_` plus 16 hex digits of the SHA-256 of the task id.
- **Minimal builds.** Only the selected tasks and the tasks reachable through `dependsOn` become actions, and the build is `bazel build` of exactly the selected labels; pure ordering tasks (`repo:noop`) vanish behind their producers unless they are selected themselves. A target that is not needed is never analyzed or run.
- **Declared inputs.** The `inputs` of a cacheable task (named groups, `^` expansion, globs, `!` exclusions, `externalDependencies` as `bun.lock` and `package.json`, `dependentTasksOutputFiles` as the producers' outputs) are resolved against the files of the checkout and declared as Bazel inputs: `srcs` is a `filegroup` of labels `s/<path>` (deduplicated by file set). Paths Bazel cannot name as labels (label-unsafe names, directories, dangling links, anything below a nested `BUILD` package) are hashed into the fingerprint instead. The fingerprint also carries the host platform and CPU, the Bun version, the build mode, declared environment values and `runtime` command outputs. The action manifest is content-stable: it names the task and the executing implementation but no workspace, runtime or snapshot path; the live data sits in `runtime.json`, outside every key.
- **Declared outputs.** Each task has exactly one declared tree-artifact output `<label>.snapshot`. A consumer receives the snapshots of its producers (the transitive closure when its input says `transitive: true`) as inputs. After the build the snapshots of the selected tasks are restored into the checkout; intermediate snapshots are restored only inside the consuming action.
- **Cache decision (fail-closed).** Each action carries `cache` and, when it is uncached, a machine-readable reason; the Starlark rule fails without one and tags the action `no-cache`. The adapter decides in this order (`bazelCacheDecisionV1`):
  1. An explicit `cache: true` against a plugin class other than `deterministic` is a conflict and throws ("Conflicting cache declaration"), unless the task is a pure ordering task.
  2. `continuous` (a service) is uncached with reason `continuous`.
  3. `--skip-cache` is uncached with reason `skipped`.
  4. A pure ordering task (`repo:noop`) is always cacheable.
  5. A class other than `deterministic` (see Cache classes) is uncached with the class name as reason: `driver-owned`, `managed-output`, `side-effecting`, `live`, `aggregate`, `diagnostic`.
  6. `cache: false` in the manifest is `declared` and wins even against a `deterministic` class.
  7. No `cache: true` and no `deterministic` class is `undeclared`: the task reruns until it declares one.
  8. `cache: true` or a `deterministic` class without an `outputs` array is `unbounded`: a hit could not restore what it produced.
  9. Everything else is cacheable.

  So declare `outputs` (`[]` for pure checks) on every cacheable target. Continuous tasks become `repo_launch` actions that run with `bazel run`, are waited for by their readiness declaration and share the build's server.
- **Execution requirements.** Every task and launcher action is tagged `no-remote-exec` (the `repo_noop` marker writes a file and runs no process). Actions that carry `host_state` are also tagged `no-sandbox`; today that is every task and launcher action, only the `repo_noop` marker is sandboxable (`STARLARK_RULES` in `🏗️build/🔧️tools/🏗️bazel/📜️script.ts`; `Actions.kinds` in `🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json`). The two reasons the schema records (`ActionPolicy.kinds`): a *task* action runs the task process in the real checkout, with its working directory, declared source-tree and shared-store writes, the loopback control socket and the out-of-tree runtime channel, which a sandbox would hide or block; a *launcher* action binds exact execroot output paths and obtains authority over the invocation's control socket, which a sandbox execroot would change. Consequence: results are keyed by the declared inputs and fingerprint, not by what a sandbox observed, so a missing input declaration shows up as a stale hit, not as a failure. The repository `.bazelrc` sets no `--spawn_strategy`, `--sandbox_*` or `--strategy` flag today (the test "the repository .bazelrc carries exactly the authored static cache policy and no host paths" in `🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests` pins its cache lines and rejects host paths, but nothing forbids a sandbox flag), so any action that is ever declared without `host_state` runs in Bazel's default sandbox.
- **Sandbox and host paths.** Bazel 8.4.2 on macOS handles non-ASCII paths of its own state incorrectly, in two places found by experiment: an emoji or accented `--install_base` crashes the server at startup ("Could not find or load main class"), and the `darwin-sandbox` profile is written from Bazel's byte-wise path strings but encodes each byte as UTF-8 a second time, so the write rules for a non-ASCII `--output_base` never match and every sandboxed action (including its `stats.out`) fails with "Operation not permitted". It looks location dependent only because `/private/tmp`, `/private/var/tmp`, `TMPDIR` and `~/Library/Caches` are allow-listed for any spelling. The production layout keeps the `install` base, the `outputs` root and the `server` output bases (`<products>/bazel/outputs/server/<hash>`) inside the native partitions below `.cache/b`, which is ASCII whenever the checkout path is ASCII (for example `/Users/<name>/Documents/semio`), and `requireBazelHostPathsV1` (called by `bazelCacheLayoutV1` in `🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts`, which every Bazel storage user builds) refuses a layout whose `install`, `outputs` or `server` path contains a non-ASCII character on darwin ("ASCII Bazel host path required on darwin: outputs"), at layout time instead of during a build. So keep the checkout path ASCII on macOS. `--sandbox_base` is not used. A sandboxed action can only write inside its execroot, the temp directories and `~/Library/{Caches,Logs,Developer}`; if a task that writes into the checkout or a partition is ever sandboxed it needs `--sandbox_writable_path=<ASCII path>`, which goes through the same encoder and so must be ASCII too. Linux (bind mounts instead of a profile) and Windows were not examined, the guard is darwin only; Windows is covered by the readiness refusal for non-ASCII workspace and partition paths.
- **Emoji source labels.** Task sources are labelled `s/<path>` (`bazelSourceLabelV1`): the file reached through the `s` link, a target name inside the root package. Observed with the pinned 8.4.2 (the real-binary test in `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions` replays the fixture names and is skipped without `SEMIO_TEST_BAZEL_BINARY`): Bazel accepts any non-ASCII character in a target name, including astral emoji, ZWJ sequences, skin tones, combining marks, CJK and a no-break space, also as action input and output names; only a backslash and control characters are rejected. Only package names must be ASCII, so the adapter never needs an emoji package; it treats a directory that holds `BUILD` or `BUILD.bazel` as a package boundary whatever its spelling and hashes everything below it (`bazelSourceClassesV1`). The adapter admits a path as a label when it matches `^(?:[A-Za-z0-9_@=,+~%./\-]|[^\u0000-\u007f])+$` (code point based, so it holds in Python, Rust and Go too) and hashes everything else (space, backslash, control characters, directories, dangling links, anything below a directory that holds `BUILD`) into the fingerprint instead. In a scratch experiment the Bazel disk cache and the cache keys were identical across `LANG=C` and a UTF-8 locale (no test covers it); the adapter sets a UTF-8 locale on darwin (`LANG` and `LC_ALL` `en_US.UTF-8` in `runBazelTasksInStorageV1`).
- **Disk cache.** `<products>/bazel/actions`, with garbage collection (`--experimental_disk_cache_gc_max_size` set to 40 percent of the products partition budget, aligned to MiB; `.bazelrc` adds `--experimental_disk_cache_gc_idle_delay=10s` and `--experimental_disk_cache_gc_max_age=30d`). Collection only runs inside a persistent server that has been idle (measured with Bazel 8.4.2: under `--batch` it never ran), so the adapter does not use `--batch` for builds. The repository contents cache keeps entries 3 days (`--repo_contents_cache_gc_max_age`). The module catalog and repository cache live in `<dependencies>/bazel/modules`; builds run with `--repository_disable_download`, so they never fetch modules.
- **Shared output base.** `--output_base` is `<products>/bazel/outputs/server/<first 16 hex of sha256(workspace directory)>`: one server per generated workspace directory (Bazel restarts a server that is moved between workspace directories), shared by the build and every `bazel run` of continuous tasks of that selection. The server idles out after 120 seconds. The disk cache is shared by all selections, which is what makes a second selection reuse the first one's results.
- **Remote cache switch** (off by default, no flag is passed while the endpoint is unset):

  | Variable | Meaning |
  |---|---|
  | `SEMIO_BAZEL_REMOTE_CACHE` | Endpoint `grpc://`, `grpcs://`, `http://`, `https://` or `unix:`. |
  | `SEMIO_BAZEL_REMOTE_CACHE_UPLOAD` | `true` uploads local results. Anything else (default) is read-only, because the actions run unsandboxed and must not poison a shared cache. |
  | `SEMIO_BAZEL_REMOTE_CACHE_CREDENTIAL_HELPER` | Absolute path or bare command name; no secrets in arguments. |

  `--skip-remote-cache` drops the endpoint for one invocation; `--skip-cache` adds `--noremote_accept_cached`. Invalid values are refused ("Closed Bazel remote cache environment required"). Compression, a 30 s timeout and 3 retries are set in `.bazelrc`.
- **Cache operations.** `bun repo --tool=bazel cache status` counts the files of the `ac` and `cas` folders of the disk cache and adds the recorded build activity (builds, actions, executed, cache hits), which each build appends to `<products>/bazel/outputs/server/metrics.json` from its build event stream. `cache clear` stops the persistent servers (by removing their pid file), removes their output bases (Bazel's local action cache) and empties the disk cache, so the next build is a miss; the Bazel binary, modules and toolchains stay.
- **Storage.** Bazel needs the tools, dependencies and products partitions (`bazel/bin` and `bazel/install` in tools, `bazel/modules` in dependencies, `bazel/workspaces`, `bazel/outputs`, `bazel/actions` in products). `bun repo --tool=bazel reset` invalidates the graph and empties `bazel/actions`, `bazel/outputs` (which holds the server output bases and the activity ledger) and `bazel/workspaces`; it does not stop a running Bazel server first, so run it when no build is active (`cache clear` stops them).
- **Verified and not yet verified.** Bazel itself was exercised on macOS arm64 with the pinned 8.4.2 in scratch workspaces through the tests that set `SEMIO_TEST_BAZEL_BINARY`. Re-run on 2026-10-19 and passing: the real-binary test of the storage suite (sandboxed build, disk cache hit after `bazel clean`) and the labelled-emoji test of the actions suite. Recorded earlier and not re-run now: `cache status` and `cache clear` against the real products partition, the host path guard, and `affected` against `bazel query rdeps`. The storage and operations suites (cache flags, layout, remote switch, cache operations, dry-run and `affected` parity) passed on 2026-10-19 without the real binary; the action suites (cache decision and class oracles, restore, input selection, exact `BUILD.bazel` text, Starlark syntax checked with an independent Python interpreter) need an issued process envelope and run only through the aggregate suite. That aggregate suite (`bun repo run workspace:test-build-tool-bazel`, that is `🏗️build/📜️script.ts test bazel`) is not green: on 2026-10-19 its first test, `Bazel component contracts have one canonical owner with unchanged normalized constraints`, failed because two normalized schema constraints differ from their baseline (`unchanged` is false), and a failing test closes the shared envelope for its successors (41 of its 78 tests passed, 34 failed). An earlier blocker, a build-graph import of the coarse `@semio-tech/repo-lib/storage` barrel that pulled `typescript` into the Bazel closure, no longer exists. Not yet verified: any real `bun repo --tool=bazel run` of a product (neither a cold run, a warm run served from the disk cache, nor an unrelated-edit run), the build activity ledger after a real run (`cache status` must show `activity.builds >= 1`), the CLI path of `cache clear`, Linux and Windows (including the emoji label rules and the sandbox encoding there), and the devcontainer.

## 🎛️ Command registry

The command registry is the developer command surface: every executable command is declared once beside its source and selected by developers in the dashboard (`bun run dashboard`, **New Task**); agents and automation use the same daemon through the `semio` client; no IDE launch, task or solution file registers commands (`.vscode/` does not exist in the tree). The registry is data: the root `📋️project.json` holds `metadata.semio.dashboard` with `parameters`, `tools`, `compounds` and `groups`, validated against the schema `🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` (`ProjectManifest`); other projects declare typed controls on their targets, playground controls come from crate manifests. The [dashboard guide](🔨️modules/🎛️dashboard/README.md#-daily-work) lists the daily rows; the build part is:

| Tool id (`tool:workspace/…`) | Runs | Notes |
|---|---|---|
| `build-task`, `run-targets`, `build-affected`, `build-watch` | `bun repo run`, `run-many`, `affected`, `watch` | `run-targets` and `build-affected` take a `dry-run` flag (`--dry-run`) like `build-task`. |
| `build-exec` | `bun repo exec --projects={projects} --dry-run={dry-run} --` plus the command words in **Arguments** | `dry-run` is a choice (`false` default, `true`) inside the words before `--`, because arguments after `--` belong to the child program; `--dry-run=false` is a no-op. `bun-test`, `run-script` and `native-cargo` offer the same choice. |
| `build-cache-status`, `build-cache-clear` | `bun repo cache status`, `cache clear` | The `build-tool` axis selects whose cache. |
| `build-preflight`, `build-deps-sync`, `build-deps-lock` | `bun repo preflight`, `dependencies sync --all`, `dependencies lock --all` | Host check; install and lock all owners (`--scope <owner>` stays on the command line). |
| `build-graph`, `build-projects`, `build-targets`, `build-project`, `build-help`, `build-reset`, `build-tools`, `cargo` | `graph`, `show …`, `--help`, `reset`, `tools setup`, `bun repo cargo` | |

Global axes (parameters) of the root registry: `build-tool` (`SEMIO_BUILD_TOOL=nx|bazel`), `cache` (`use`, `skip-local` = `--skip-cache`, `skip-all` = `--skip-cache --skip-remote-cache`), `build-mode=ship` (release build, both caches skipped), `dependencies` (`--exclude-dependencies`), `test-level`, `nextest-output`, `cargo-jobs`, `build-budget`. `build-tool` is offered on every verb that runs build tasks, including `publish`, `prepare`, `deploy`, `activate`, `serve`, `reset` and `inspect`, so the same task can run under Nx or Bazel from the dashboard; a law compares its verbs with the verbs of the Rust registry (`VERBS`) plus the fallback `task` plus `reset` and `inspect`. `cache` is not offered on `dev`, `run`, `watch`, `clean`, `reset`, `inspect` and `setup`, which are continuous or have no task cache. A fresh native build generation is a registry parameter `generation=fresh` (`SEMIO_NATIVE_BUILD_GENERATION=fresh`) of the Play `build` target and the root `release` target, combined with `build-mode=ship`.

Commands of the root script (`bun ./📜️script.ts verify …` and similar) are routed through targets that declare `metadata.semio.workspaceCommand`: an array of at least two words of `[a-zA-Z0-9-]`, unique across the repository (for example `verify fem-combination-stress`). A malformed or duplicate declaration makes the route loader throw ("Invalid owned command", "Duplicate owned command") and breaks every `verify …` route, so the declarations of the whole tree are validated together.

Choose **New Task** → `@semio-tech/repo-lib:test-dashboard-control` to check the command registry. The suite compares the root registry with Ajv 2020 and the repository schema compiler, neutral owner fixtures with SQLite and the Rust registry verbs, and preview arguments with `validateBuildTaskArgumentsV1` and `nxBuildArgumentsV1`. Output and results stay with the task. Agents and automation reach the same owner through `bun repo run @semio-tech/repo-lib:test-dashboard-control`. The `@semio-tech/repo-dashboard-rs:test-journeys` target additionally exercises the installed native client and actual terminal journeys.

## 🧾️ Build owner registry

`📋️build-owners.json` at the repository root lists every module that owns a build manifest (`owners`) and the catalog providers. A law keeps it equal to the tree: `bun ./📜️script.ts verify build-owners` (root targets `workspace:verify-build-owners`, cached with `outputs: []`, and bare `verify`/`workspace:verify`) fails with one finding per mismatch, for example `project-without-row`, `cargo-without-row`, `missing-manifest`, `name-mismatch`, `cargo-row-shadows-project` or `package-stale` (`📚️library/🏗️build/🧬️schema/🧾️owner-consistency.json` lists all kinds); `verify build-owners sync` (target `workspace:verify-build-owners-sync`, uncached) repairs what is mechanical (drops rows of vanished modules (only when no manifest exists and git tracks nothing at the path), relocates a moved module, renames a renamed project, hands a Cargo row over to a `📋️project.json`, adds rows for new manifests in byte order) and then fails on what remains: findings that need an author (catalog providers) and rows whose paths git still tracks while the files are absent (a concurrent move in progress). Rows whose manifest sits under a git-ignored path are exempt, so a fresh clone does not fire the law. The scaffolding writers (crate carve, subset scaffold, mutation tree) call the same reconciler with a scope. An applied move plan reconciles the registry for the subtrees it touched (`reconcileBuildOwnerRegistry` in `📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts`); any other directory move or rename is caught by the law and fixed by `verify build-owners sync`. The core and its fixture cases live in `📚️library/🏗️build/🕸️graph/🧾️registry-consistency`; the registry file holds one row per module that owns a manifest (3718 rows), and `bun ./📜️script.ts verify build-owners` printed `passed` on 2026-10-19.

The tools that run before the dependencies are installed (`bun repo` help, preflight, `tools setup`, `dependencies lock` and `sync`, the native dependency installers, the wasm tools, the Bazel bootstrap, and the root script's router, `setup` and entrypoint routes) must run on a checkout without `node_modules`. A law keeps their import closures relative: `bun ./📜️script.ts verify zero-touch` (root target `workspace:verify-zero-touch`, cached, and bare `verify`/`workspace:verify`) walks the closures listed in `🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json` and fails for a bare workspace specifier (`workspace-specifier`), any other external package such as `typescript` (`external-package`), a closure entry or seam that is gone (`missing-entry`) and a static import of a post-install seam (`seam-static`, for example the graph inference that reads sources through the TypeScript compiler; seams may only be loaded dynamically).

## 🧰️ Setup per platform

`bun repo tools setup [nx|bazel|all]` provisions what the selected implementation needs, and `workspace:setup` depends on the target `deps-build-tools`, which runs `tools setup all` (next to the language environments it also prepares), so the devcontainer's `postCreateCommand` (`bun repo run workspace:setup`) and the native bootstrap scripts provision the build tools without a separate step.

- **`nx`**: installs the pinned Nx as an immutable toolset below `.cache/b/t/m/nx/<digest>`, points `.nx/installation` at it (a symlink, a junction on Windows) and checks `nx --version`.
- **`bazel`**: (1) checks the platform before any image is attached, (2) opens the tools, dependencies and products partitions, (3) downloads and verifies the pinned Bazel binary with progress lines (phase and every 10 percent), (4) prepares the Bazel module catalog and repository cache (offline-aware, cancellable), (5) prints `[bazel] ready binary=… modules=… lock=…` and a `[bazel] readiness {json}` line. A plain `--tool=bazel run` makes the same platform check.
- **`all`**: Bazel, then Nx.

| Host | Fixed cache images | What has to be true | Status |
|---|---|---|---|
| macOS (arm64, x64) | case-sensitive APFS disk images through `hdiutil` | Xcode Command Line Tools (the native bootstrap script prints the missing step). No privilege. The Bazel `install`, `outputs` and `server` paths (all below the checkout, so the checkout path itself) must be ASCII: the pinned Bazel crashes at startup on a non-ASCII install base, and its macOS sandbox profile mis-encodes a non-ASCII output base so that sandboxed actions fail with "Operation not permitted"; `requireBazelHostPathsV1` refuses such a layout. | Implemented; the primary development host. |
| Linux native | Directory-backed partition: `fixed.dir` anchor, `v` volume and an empty `m` placeholder; attach and detach are one atomic `renameat2(RENAME_EXCHANGE)` of `m` and `v` | No privilege: no root, `CAP_SYS_ADMIN`, loop device or mount tool. A native case-sensitive file system with atomic directory exchange (`renameat2` `RENAME_EXCHANGE`, Linux 3.15 or newer; the Bazel readiness message names ext4, xfs and btrfs; ext4 on a Docker volume and tmpfs were exercised by the directory suite in an unprivileged Linux container when the backend landed, not re-run here). `preflight` probes it beneath the nearest existing ancestor of `.cache` and the Bazel readiness check beneath that of the tools partition, and both print the probe error when it is missing; case sensitivity is probed again when a partition is born. Capacity is accounting, not a file system size: the journal admits reservations, and the directory meter re-measures the real tree at least every 16 MiB admitted or every 2 seconds, on every commit and before any refusal (`createNativeDirectoryMeterV1`; `meter` in the directory schema). | Run unprivileged in a Docker `oven/bun` container on a Linux volume (preflight, leases, diagnostics, dependencies and tools partitions born, attached and reopened); `tools setup` and publication were not reached. A bind mount from a case-insensitive host (macOS or Windows Docker Desktop) is refused on purpose. |
| Devcontainer | as Linux | The compose service declares no `privileged`, capability or device (rebuild the container after pulling this change); the `docker-in-docker` feature of `devcontainer.json` is applied by the devcontainer CLI with the feature's own metadata, which was not examined, so the cache partitions need no privilege but the container as a whole is not proven unprivileged. `.cache` (the native partitions) is a named volume private to the checkout, not part of the bind mount, because a bind mount from a case-insensitive host (Docker Desktop on macOS or Windows) is refused by `preflight`; the image generates `en_US.UTF-8` and declares 128 GB of storage. `postCreateCommand` is `bun repo run workspace:setup`. See the [devcontainer README](../../../.devcontainer/README.md). | As above: not verified in a container. |
| Windows native | directory-backed partitions as on Linux (`fixed.dir`, `v`, `m`), swapped by a journaled two-phase write-through rename (`createNativeDirectoryJournaledPortV1`); `openNativePlatformStorageV1` no longer selects the earlier `fixed.vhd` / `Mount-DiskImage` backend | NTFS with per-directory case sensitivity: Windows build 17107 or newer, the optional feature `Microsoft-Windows-Subsystem-Linux` enabled and an empty cache directory on which the flag can be set; the backend refuses with a message naming the missing piece. Long paths enabled (`LongPathsEnabled`, `core.longpaths`; `setup git` and the script print a readiness line, and the Bazel readiness check refuses while `LongPathsEnabled` is off) and a short ASCII checkout root (Bazel 8.4.2 refuses to start under a non-ASCII workspace or partition path, so the readiness check refuses before any download and names the character and its index). An elevated (Administrator) terminal is still demanded by `🔵️.ps1` (unless `-SkipRepoBootstrap`) and by the Bazel readiness check; both messages still give the retired VHD attach as the reason, so that requirement is due for review. `🔵️.ps1` sets `core.longpaths` and `core.symlinks` in the clone's own git config, stores the Playwright browsers below `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, and also sets the user-level environment variable `PLAYWRIGHT_BROWSERS_PATH` and moves the tool directories to the front of the user `PATH`. | Not run on Windows. The switch to the directory backend is recent and unverified; treat this row as provisional. |

All hosts: the native bootstrap scripts (`🔩️native/🥾️bootstrap/🐚️.sh` and `🔵️.ps1`) install Bun, Node, Go, rustup and uv from a checksum-verified pin manifest (`🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`: Node 24.15.0, Go 1.26.9, rustup 1.29.0, uv 0.11.6 and the `dotnet-install.sh` commit, 24 rows for darwin, linux and win32 on x64 and arm64 plus one platform-independent row) instead of piping installers into a shell; Bun 1.4.2 comes from its own schema row and equals `packageManager`. The shell script additionally installs the Chromium system libraries through Playwright's `install-deps chromium` on apt hosts. Other Windows packages (Git, CMake, Build Tools, …) are installed through winget when missing and not version-pinned. Bazel 8.4.2 has a checksummed release row for darwin, linux and win32 on arm64 and x64, and no cache path may exceed 256 characters. The pull request workflow `.github/workflows/build-system.yml` has two jobs on Ubuntu, macOS and Windows: `smoke` installs the pinned Bun with a verified SHA-256 and runs `bun repo --help`, `preflight`, `tools setup all`, `dependencies sync --all` and a cache proof for both tools on `@teaching/architecture-course:test` (clear, run, status grows, run again and recognise a hit, status unchanged); `products` then runs that target under `--tool=nx` and `--tool=bazel`. It uses no `sudo` and no installer. Not yet verified: the workflow has never run, so the Bazel hit rule (a calibrated reading of Bazel's console summary) and the restore of the Bun store and the Bazel disk cache into the owned partitions are untested.

## 🚀️ Product chains

All chains use the neutral entry, so each one works with `--tool=nx` (default) and `--tool=bazel`.

| Product | Chain | Notes |
|---|---|---|
| Course `@teaching/architecture-course` | `typecheck` and `test` (cached) → `dev` (continuous: the proctor, then the site, stopped together; site port 6061) → `test-e2e` (Playwright, throw-away stacks on free ports, uncached) → `deploy-check` | `build` and `publish` are uncached in the manifest. `preflight` is the generic host preflight; the Docker image targets refuse outside a native Linux x64 host (`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🐳️docker-image`). |
| Proctor `@teaching/proctor` | `build`, `test` (cached; the binary lives in the compiler store and `build` declares its store receipt) → `dev` (continuous, port 8791) | The operator verbs (`backup`, `restore`, `erase`, `prune`, `capacity`, …) are uncached. |
| Play `@semio-tech/semio-tech-play` | `prepare-prototype` → `activate-prototype` (cached; writes `dist/♻️activation/prototype`) → `serve` / `dev` (continuous, port 6033) | `prepare-e2e` → `serve-e2e` → `test-e2e` needs the Playwright browsers (`workspace:deps-browsers`). `build` (uncached) declares a store receipt as its output (`.🧾️store/build.json`). |
| Demonstrator `@semio-tech/mit-bestand-demonstrator` | `prepare-prototype` → `activate-prototype` → `serve` / `dev` (continuous, port 6029) | Same e2e chain; `build` (uncached) needs `prepare-release`. |

Example: `bun repo run @semio-tech/semio-tech-play:serve --tool=bazel`. Not yet verified: no end-to-end run of these chains under either tool has been recorded for this documentation; the target names, dependencies, ports and cache flags above are read from the manifests.

## ✅️ Verification status

| Claim | Evidence |
|---|---|
| The operation catalogue, the per-command help in English and German (snapshot per command and language), the documented flags against the task validator, the neutral `exec` environment and the result schemas agree | `📚️library/🏗️build/⚙️operations/🧪️tests/🟦️.ts` (also runs `preflight --help` through the real CLI; 24 tests with the store receipt suite, 0 failures on 2026-10-19) |
| Selection, flags and the `affected` rule | `🏗️build/🧭️selection/🧪️tests`, `🏗️build/🧭️invocation/🧪️tests` (the `affected` rule is cross-checked against Nx's own file parser) |
| Nx translation, boundary and probe | `⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations`, `…/🔌️sdk`, `…/🔁️probe` (`🧪️tests`) |
| Bazel cache flags, storage layout and remote switch | `🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests` (one more test runs against the real binary when `SEMIO_TEST_BAZEL_BINARY` is set) |
| Bazel readiness per platform | `⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🧪️tests` (fixtures only, no Windows or Linux run) |
| Bazel actions, inputs, restore, manifest | `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions` and the adapter suite (`bun repo run workspace:test-build-tool-bazel`; not green on 2026-10-19, see Bazel implementation) |
| Bazel cache operations, dry-run, `affected` and `exec` parity with Nx (compared against Nx's own `filterAffected` and `createTaskGraph`, and `bazel query rdeps` when `SEMIO_TEST_BAZEL_BINARY` is set) | `🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests` |
| Cache decision classes and store receipts | `🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache`, `🏗️build/🧾️store-receipt/🧪️tests` |
| Command registry: axes, cache and exec tools, dry-run forms | `@semio-tech/repo-lib:test-dashboard-control`, `@semio-tech/repo-dashboard-rs:test-journeys` |
| Build owner registry law and its sync | `🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests`, `📚️library/🧪️tests/📦️crate-tree/🔒️workspace-registration` (on 2026-10-19 `verify build-owners` passed on the real registry) |
| Linux directory partitions, preflight | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests` (also in an unprivileged Linux container), `📚️library/🏗️build/🩺️preflight` |
| Dependency installer repair and generation retirement | `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests` (fixtures; the full sync path is not recorded) |
| Real cold and warm cache behaviour of Nx and Bazel, a real Bazel product run, the store receipt repair path, `build-system.yml`, Windows, Linux beyond the partition tests, devcontainer setup, product end-to-end chains | Not yet verified |

## 💾️ Cache partitions

All build state lives in fixed-capacity partitions below `.cache/b/<partition>` (git-ignored, derived, safe to delete when nothing runs). On macOS each partition is a UDIF disk image formatted as case-sensitive APFS, `fixed.dmg`, mounted at `m` next to its `image.json` receipt (`ImageProtocol` fixes `UDIF` and `Case-sensitive APFS`; HFS+ was ruled out earlier because it lacks the atomic exchange `RENAME_SWAP` that publication and binding use, not re-checked here). Linux uses a directory-backed partition without any privilege (`fixed.dir`, `v`, `m`, switched by an atomic `renameat2(RENAME_EXCHANGE)`) and Windows a directory partition too, swapped by a journaled two-phase write-through rename instead of `renameat2` (implemented from source, not verified on those hosts; a swap fails with "A handle is still open inside the partition" while any process holds a file in it). The `hdiutil` steps below do not apply there: nothing is mounted, so stop all processes and remove the partition directory instead. Capacities come from `📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`.

| Partition | Content | Capacity |
|---|---|---|
| `d` | dependencies, including `node_modules` generations | 8 GiB |
| `t` | tools: rustup, Bazel, Nx | 8 GiB |
| `g` | neutral build graph and inference records | 4 GiB |
| `o` | observations and diagnostics | 2 GiB |
| `p` | products: build outputs and published artifacts | 24 GiB |
| `l` | leases and locks | 64 MiB |
| `n` | Nx cache pool, namespaces are funded child images | 1.5 GiB |

`.cache/b/c` is the plain control directory (no image, at most 1 MiB; it holds the lease database of the images, `budget.controlDirectory`). The Rust target and build directories are a separate fixed partition at `.🧬semio/🦑️repo/⚡️cache/rust/fixed` (16 GiB). `bun ./📜️script.ts clean` never touches any of them, nor the `🗑️generated` folder of an open ticket.

### 🩹️ Recovery

Check items 1 to 6 before touching code when every `bun repo` call refuses at startup (the `hdiutil` commands are macOS only, see item 2 for Linux); item 7 concerns the dependency installer. Stop all `bun repo`, cargo and Nx processes first.

1. **"Pending Native image publication must settle"**: a killed run left `.cache/b/<partition>/.publication/intent.json`. Confirm no process is alive, move the file aside, rerun.
2. **Receipt or mount mismatch** ("Exact canonical persistent Native publication required", "Ambiguous Native backing kernel association", an `image.json` without its `fixed.dmg`): recreate the partition.
   ```bash
   hdiutil detach .cache/b/<partition>/m || umount -f .cache/b/<partition>/m   # diskutil may report it unmounted while it is not
   hdiutil info                                  # no image may still claim the path
   rm -rf .cache/b/<partition>
   bun repo --help && bun repo tools setup all   # the next call recreates it
   ```
   Never `mv` a partition directory while it is mounted: the image keeps its old path and the association becomes ambiguous. On Linux nothing is mounted: skip the `hdiutil` lines, confirm that `lsof +D .cache/b/<partition>` is empty (an atomic exchange succeeds even with open files, unlike `umount`) and remove the directory. A directory partition that `preflight` refuses ("Atomic directory exchange is unsupported by this kernel or filesystem", or a case-insensitive file system) cannot be repaired by recreating it: move `.cache` to a native case-sensitive file system, in the devcontainer onto its `.cache` volume.
3. **Nested attach "Permission denied"**: the parent was mounted `noowners`. With nothing open (`lsof +f -- .cache/b/<partition>/m` is empty) run `hdiutil detach` on the parent; the next call re-attaches with `-owners on`.
4. **Poisoned Nx pool** (every graph or Nx call fails "Exact original parent child allocation required"): `hdiutil detach .cache/b/n/m/<shard>/native/m`, then `rm -rf` the shard directory.
5. **ENOSPC in `d`**: empty `.cache/b/d/m/tmp` (installer leftovers), then rerun `dependencies sync --all`.
6. **Image created before the APFS switch** (ENOTSUP on rename): recreate it as in step 2.
7. **`dependencies sync` fails with `EPERM: Operation not permitted: failed to link package`** (macOS 27, observed on arm64): the kernel refuses to hard-link or clone files of the Bun package cache `.cache/b/d/m/bun-cache` that carry the `com.apple.provenance` attribute of another application identity or session, while files the current process created link fine (`ln` and `cp -c` of an old cache file give EPERM, a fresh copy links; `xattr -d` returns success but the attribute stays). The sync now handles it: it classifies that installer stderr as a cache link refusal, prints `[DATA] Store dependency cache link refused; purging the Bun package cache and retrying`, empties `bun-cache` in place (the directory keeps its identity; progress `dependencies:purge-bun-cache:<n>`), re-prepares the environment and retries the installer once per sync, so the retry re-downloads the packages it needs. If it still fails, empty `.cache/b/d/m/bun-cache` yourself with nothing running, or run the sync from the same application identity that populated the cache. Not yet verified end to end: the classifier, the generation selection and the tree removal are covered by fixture tests, but a full `dependencies sync` through the purge-and-retry path was not recorded.

Recreating `d` costs one full `dependencies sync --all` (about 16 minutes with a warm package cache in the one recorded run on the primary macOS arm64 host, 2026-10-19), recreating `g` one cold graph inference (minutes; not timed for this documentation).

## 🎛️ Developer Dashboard

The [dashboard daily workflow](🔨️modules/🎛️dashboard/README.md#-daily-work) is the developer entry point. Choose **New Task**, search for the operation and its owner, then complete the typed form. The preview shows its arguments and environment before **Start**. Output, progress, readiness, source navigation, interactive debugging, cancellation and restart remain attached to that task.

The Command registry section of the build chapter lists the build tools, the global axes (`build-tool`, `cache`, `build-mode`) and the dry-run forms. Declare executable operations beside their source: build targets in `📋️project.json`, typed controls in `metadata.semio.dashboard`, and authored ticket tools in the ticket's `🎮️commands.json`. Playground controls come from `[[package.metadata.semio.playground]]` in crate manifests; native crate targets come from `📋️build-owners.json` and the neutral build graph. Parameter choices, environments, readiness and compound membership belong to these owner declarations. Editors provide source navigation and editing; the dashboard owns execution.

## 🐹️ Go layout

Go code follows the canonical taxonomy layout instead of package-root files:

- A member is a directory with one `🐹️.go` leaf: `<module>/📦️packages/🐹️go/<member>/🐹️.go`. The member name is registered in `📚️library/🔣️taxonomy.json` (`members-of-members-of-members-of-modules`).
- White-box tests are cases at the language-neutral owner: `<module>/🧪️tests/<case>/🐹️.go`, absorbed into the package when the package clause is the module's or `<package>_test`. Black-box adapters are never absorbed.
- Because files are not at the package root, plain `go test` and `go list` do not see the members. Run Go through the package `📜️script.ts` (`runCanonicalGoTests`) or `bun repo run <project>:test`, which pass the merged source overlay.

## 🇩🇪️ Deutsch

Dieser Abschnitt ist die deutsche Fassung von „Build and run“. Pfade, Befehle, Flags und Bezeichner bleiben unübersetzt. Pfade, die mit `📚️library/` beginnen, liegen unter `🔨️modules/` dieses Produkts.

`bun repo` ist der einzige Build-Einstieg. Er ist werkzeugneutral: Jedes Target wird einmal in einer `📋️project.json` deklariert, ein neutraler Graph beschreibt alle Projekte und Targets, und zwei austauschbare Implementierungen führen sie aus: **Nx mit Bun** (Standard) und **Bazel** (`--tool=bazel`). Rufe nie selbst `nx`, `bunx nx`, `npx nx`, `bazel`, rohes `cargo` oder `rustc` auf. Die Befehle:

```bash
bun repo run <project>:<target>[:<configuration>]       # eine Task; mit --tool=bazel die Bazel-Implementierung
bun repo run-many --targets=<targets> [--projects=<projects>]
bun repo affected --targets=<targets>                    # Projekte, die sich gegenüber HEAD geändert haben, plus ihre Abhängigen
bun repo exec --projects=<projects> -- <command>         # ein Befehl je Projekt, Projektwurzel als Arbeitsverzeichnis
bun repo run <project>:<target> --dry-run                # Task-Graph ausgeben, nichts ausführen
bun repo show projects|project <name>|targets            # der neutrale Graph (auch: bun repo graph)
bun repo cache status|clear                              # Task-Cache des gewählten Werkzeugs
bun repo tools setup [nx|bazel|all]                      # die gepinnten Build-Werkzeuge für diesen Rechner einrichten
bun repo dependencies sync --all                         # Abhängigkeiten installieren (--scope <owner> für einen Eigentümer)
bun repo dependencies lock --all                         # bun.lock nach einer Manifest-Änderung neu erzeugen
bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib
```

Zusätzlich gilt:

- **Hilfe.** `bun repo --help` gibt jeden Befehl und jedes Flag aus; `bun repo <command> --help` (auch mit Unterbefehl, etwa `cache clear --help`) gibt Verwendung, Optionen und Beispiele genau dieses Befehls aus. Der Text wird aus dem Operationskatalog auf Englisch oder Deutsch erzeugt: `--lang=en|de`, dann `SEMIO_LANGUAGE`, dann die Locale (`LC_ALL`, `LC_MESSAGES`, `LANG`), dann die erste Sprache des Katalogs. Die Hilfe wird beantwortet, bevor ein Werkzeug oder Store startet.

- **Abhängigkeiten.** Nach jeder Änderung an einer `package.json` oder `bun.lock` ist die installierte Generation veraltet: erst `dependencies lock --all`, dann einmal `dependencies sync --all`, nie ein freies `bun install`. Vor der Installation entfernt ein Sync jede Abhängigkeits-Generation `j/<20 Hexziffern>`, die weder zum aktuellen Plan gehört noch Ziel einer lebenden Bindung ist (Fortschritt `dependencies:retire-obsolete:<n>`). Unter macOS 27 kann der Installer mit `EPERM: Operation not permitted: failed to link package` scheitern; siehe Punkt 7 der Wiederherstellung, der Sync repariert das selbst.
- **Cargo.** Rohes `cargo` und `rustc` verweigert das Compiler-Eingangstor in `.cargo/config.toml`. Kompiliere über `bun repo cargo`; lege nie ein eigenes `CARGO_TARGET_DIR` an.
- **Zero-touch.** Das Build-CLI muss auf einem frischen Klon ohne `node_modules`, ohne installiertes Werkzeug und ohne Cache-Image starten. Sein Import-Abschluss importiert deshalb nur relativ. Nach jeder Änderung an einem Bibliotheksmodul muss `bun repo --help` die Usage-Zeile ausgeben. Dasselbe Bootstrap läuft unverändert im Devcontainer sowie nativ unter Windows, macOS und Linux (siehe Einrichtung je Plattform unten).
- **Abbruch und Fortschritt.** Der Katalog deklariert jede Operation als abbrechbar (`cancellable`) und alle außer `graph` und `show` als fortschrittsmeldend (`progress`) in `📚️library/🏗️build/⚙️operations/🔣️catalog.json`; die Implementierungen reichen die Prozesskontrolle des Aufrufers an jeden Pfad weiter (`runOwnedBuildInLeasesV1` in `📚️library/🏗️build/📜️script.ts`). Noch nicht verifiziert: Es wurde kein Abbruchlauf je Operation aufgezeichnet.

### Neutrales Modell

| Schicht | Was sie ist | Quelle |
|---|---|---|
| Manifest | Eine `📋️project.json` je Projekt: Targets mit Executor, Command, Inputs, Outputs, `cache`, `continuous`, `dependsOn`. | Schema `📚️library/🏗️build/🧬️schema/📋️project/🔣️.json` |
| Richtlinie | `defaultTool`, die benannten Input-Gruppen (`default`, `production`, `sharedGlobals`, `cargo`, `go`, `uv`, `dotnet`, `schemaSources`, `nativeSources`, `nativeTestSources`) und die Dateien, die den Graphen ändern können. | `📚️library/🏗️build/🔣️policy.json` |
| Graph | `BuildGraphV1`: Projekte, Targets und Abhängigkeitskanten, in TypeScript aus Manifesten, Cargo, Go, `package.json` und `bun.lock` abgeleitet, unabhängig von Nx. Schnappschüsse liegen in der Graph-Partition `g`. `show` und `graph` geben ihn aus. | `📚️library/🏗️build/🕸️graph` |
| Operationen | Der Katalog der Operationen, wie jedes Werkzeug sie umsetzt, und die JSON-Formen ihrer Ergebnisse. | `📚️library/🏗️build/⚙️operations/🔣️catalog.json`, Schema `⚙️operations/🧬️schema/🔣️.json` |
| Implementierungen | Nx-Adapter und Bazel-Adapter; beide übersetzen nur die neutrale Anfrage in ihr Werkzeug. | Abschnitte Nx und Bazel unten |

**Werkzeugwahl.** `--tool=nx|bazel` auf der Kommandozeile gewinnt, dann die Umgebungsvariable `SEMIO_BUILD_TOOL`, dann `defaultTool` der Richtlinie (`nx`). Das gewählte Werkzeug wird als `SEMIO_BUILD_TOOL` an jede verschachtelte Task weitergegeben; die Dashboard-Achse `build-tool` setzt dieselbe Variable (siehe Befehlsregister). Alles andere wird abgelehnt („Unknown build tool“). Die Compiler-Konfiguration wählst du mit `--configuration=prototype|release` (oder `-c`, einem Suffix `:<configuration>` an der Task oder `SEMIO_BUILD_MODE`); widersprüchliche Werte werden abgelehnt („Conflicting build modes“).

**Operationen.** Zustand je Werkzeug: `native` = der eigene Befehl des Werkzeugs, `adapter` = unser Code steuert das Werkzeug, `neutral` = werkzeugunabhängig, `missing` = nicht implementiert, der Aufruf wird abgelehnt.

| Operation | Nx | Bazel | `--dry-run` | Hinweise |
|---|---|---|---|---|
| `run <project:target[:configuration]>` | native | adapter | ja | Ein Trockenlauf führt nichts aus. |
| `run-many --targets=… [--projects=…]` | native | adapter | ja | |
| `affected --targets=…` | native | adapter | ja | Geänderte Dateien sind die nachverfolgten Änderungen gegen `HEAD` plus unversionierte Dateien; `--base`/`--head`, `--files`, `--uncommitted`, `--untracked` schränken ein, `--all` wählt alle Projekte. Betroffene Projekte (gleiche Regel für beide Werkzeuge, gegen Nx' eigene Locatoren geprüft): die tiefste Projektwurzel, die eine geänderte Datei besitzt, Projekte, deren Target-Inputs eine geänderte `{workspaceRoot}/…`-Datei nennen, alle Projekte bei Änderung von `nx.json`, dann alle Abhängigen. `--projects` wird für jedes Werkzeug abgelehnt; benutze `run-many`. |
| `exec --projects=… -- <command>` | native | adapter | ja | Einmal je gewähltem Projekt, Arbeitsverzeichnis ist die Projektwurzel, und das werkzeugneutrale Paar `SEMIO_PROJECT_NAME` (Projektname) und `SEMIO_PROJECT_ROOT` (Projektwurzel relativ zum Workspace) wird von beiden Werkzeugen gesetzt; die Nx-Implementierung bildet die Namen ab, die Nx' eigenes `exec` setzt (siehe Nx-Implementierung). Unter Bazel erhält jedes Projekt eine ungecachte `repo-exec`-Task. `--dry-run` gibt den `repo-exec`-Plan aus und führt nichts aus. |
| `watch --projects=… -- <command>` | neutral | neutral | nein | Führt den Befehl erneut aus, wenn gewählte Quelldateien sich ändern. |
| `graph`, `show projects\|project <name>\|targets` | neutral | neutral | nein | |
| `reset` | adapter | adapter | nein | Nx: Graph verwerfen und `nx reset`. Bazel: Graph verwerfen und `bazel/actions` (den Disk-Cache), `bazel/outputs` und `bazel/workspaces` der Produkte-Partition leeren. |
| `tools setup [nx\|bazel\|all]` | adapter | adapter | nein | Ohne Argument wird das gewählte Werkzeug eingerichtet (`--tool`, Standard `nx`). Siehe Einrichtung je Plattform unten. |
| `preflight` | neutral | neutral | nein | `preflight` prüft den Host, bevor ein Store bereitgestellt wird (`📚️library/🏗️build/🩺️preflight/🔣️policy.json`): CPU-Architektur, 8 GiB frei, benötigte Werkzeuge (`cc` und `pkg-config` unter Linux, `cc` und `hdiutil` unter macOS) und unter Linux atomaren Verzeichnisaustausch (`renameat2` `RENAME_EXCHANGE`) auf einem Dateisystem mit Groß-/Kleinschreibung. |
| `dependencies sync\|lock --all\|--scope <owner>` | neutral | neutral | nein | |
| `cargo …` | neutral | neutral | nein | |
| `cache status\|clear` | adapter | adapter | nein | Nx: das Artefaktverzeichnis der Rust-Partition. Bazel: der Disk-Cache der Produkte-Partition. Siehe die beiden Implementierungsabschnitte. |

**Flags.** Flags vor `--` sind portabel und werden validiert, bevor ein Werkzeug startet; unbekannte Flags, überzählige Positionsargumente und ungültige Selektoren werden abgelehnt. Alles nach `--` geht wörtlich an die Task. Die Flag-Liste gehört zum Katalog und wird von `bun repo --help` ausgegeben; die Task-Befehle (`run`, `run-many`, `affected`, `exec`, `watch`) akzeptieren alle Flags der Tabelle, wirken aber nur bei den Befehlen der letzten Spalte.

| Flag | Bedeutung | Befehle |
|---|---|---|
| `--tool=nx\|bazel` | Implementierung wählen (sonst `SEMIO_BUILD_TOOL`, dann der Standard der Richtlinie). | alle |
| `--help`, `-h`; `--lang=en\|de` | Verwendung des Befehls auf Englisch oder Deutsch ausgeben und enden. | alle |
| `--projects`, `-p`, `--exclude` | Kommalisten von Projektnamen mit `*`-Platzhaltern, `tag:<tag>`, `name:<name>`, `directory:<root>`, `!<pattern>` als Verneinung. | `run-many`, `exec`, `watch`; `--exclude` auch `affected` |
| `--targets`, `--target`, `-t` | Target-Namen oder `*`-Muster. | `run-many`, `affected` |
| `--configuration`, `-c` | Compiler-Konfiguration (`cargo` nimmt nur den vollen Namen). | `run`, `run-many`, `affected`, `cargo` |
| `--parallel=<n>`, `--bail`, `--verbose`, `--output=stream\|stream-without-prefixes\|static` (Alias `--output-style`) | Parallelität, Stopp beim ersten Fehler, Detailgrad, Ausgabestil. | `run`, `run-many`, `affected` |
| `--skip-cache`, `--skip-remote-cache`, `--exclude-dependencies` (Alias `--exclude-task-dependencies`) | Task-Cache ignorieren, Remote-Cache ignorieren, nur die benannten Tasks ausführen. | `run`, `run-many`, `affected` |
| `--dry-run`, `--graph[=<file>]` | Task-Graph als `DryRunPlan` auf stdout ausgeben und nichts ausführen. `--graph` ist auf beiden Werkzeugen dasselbe Flag; mit Datei wird der Plan dorthin geschrieben (Nx: die Datei wie angegeben, Bazel: ein Bericht, dessen Pfad ausgegeben wird). | `run`, `run-many`, `affected`, `exec` |
| `--all`, `--base`, `--head`, `--files`, `--uncommitted`, `--untracked` | Änderungsauswahl (`--all`: jedes Projekt). | `affected`; `--all` auch `watch` und `dependencies` |
| `--include-global-files` | `watch` reagiert auch auf Dateien außerhalb jedes Projekts. | `watch` |
| `--file=<path>`, `--json` | Ergebnis als Berichtsdatei schreiben; Listen als JSON ausgeben. | `graph`, `show` |
| `--scope <owner>` | Operation auf einen Abhängigkeits-Eigentümer beschränken (statt `--all`). | `dependencies` |
| `--store=shared\|isolated`, `--parallelism=<cores>` | Cargo-Zielspeicher und Kernzahl. | `cargo` |

**Ergebnisformen.** Das Schema `⚙️operations/🧬️schema/🔣️.json` legt fest: `DryRunPlan` ist `{"tasks":{"roots":[…],"tasks":{<id>:{"id","target":{"project","target","configuration"?}}},"dependencies":{<id>:[…]},"continuousDependencies"?:{<id>:[…]}}}` mit der Task-ID `<project>:<target>[:<configuration>]`; der Bazel-Plan ergänzt `selected` (die angeforderten Task-IDs) und führt Abhängigkeiten auf kontinuierliche Tasks getrennt in `continuousDependencies`, weil diese nur laufen müssen. `CacheStatus` ist `{"version":1,"tool","available","directory","entries","bytes","activity"?:{"builds","actions","executed","cacheHits"}}` (`activity` füllt Bazel), `CacheClear` ist `{"version":1,"tool","cleared","removedEntries","removedBytes"}`.

### Manifeste

Felder einer Task: `executor` (`repo:script`, Standard, führt `command` über die Repo-Prozessautorität aus, oder `repo:noop`, nur Reihenfolge, führt nichts aus); `command`, `options.command`, `options.commands`, `options.args` (wörtliche Argumente, nie per Shell interpoliert) mit Überschreibungen je `configurations.<name>`; `dependsOn` (`"target"`, `"^target"`, `"project:target"` oder `{target, projects, dependencies, params}`); `inputs`; `outputs` (mit `{projectRoot}`, `{workspaceRoot}` und Option-Tokens wie `{options.env.SEMIO_BUILD_MODE}`; `[]` heißt „keine Dateien wiederherzustellen“); `cache`; `continuous`; `metadata.semio.dashboard.ready` (Bereitschaft: die Ausgabe zeigt `http://(127.0.0.1|localhost|0.0.0.0):<port>` und diese Adresse antwortet mit 2xx oder 3xx).

**Inputs.** Eine Liste aus Datei-Globs (`{projectRoot}/**/*`, `{workspaceRoot}/…`, `!` schließt aus), benannten Gruppen der Richtlinie, `^<gruppe>` (dieselbe Gruppe der Abhängigkeitsprojekte) und Objekten: `{"env": "NAME"}`, `{"runtime": "<command>"}`, `{"externalDependencies": […]}` und `{"dependentTasksOutputFiles": "<glob>", "transitive": true|false}`. Deklariere alles, was die Task liest und nicht aus dem eigenen Projekt erreichbar ist: Daten außerhalb, gestartete Testdateien, Werkzeug- und Toolchain-Dateien, Umgebung. Das `default` eines Projekts ersetzt das Workspace-`default`; füge `sharedGlobals` dort wieder hinzu.

**Cache-Klassen.** `cache` ist der Schalter, den beide Werkzeuge lesen. Daneben hängt die Inferenz an jedes Target, das sie erzwingt, synthetisiert oder nicht wiederholt, eine maschinenlesbare Entscheidung `metadata.semio.cache = {"version":1,"class":…,"reason":…}` (geschlossenes Schema `📚️library/🏗️build/🏃️execution/🧬️schema/💾️cache.json`). Die Klasse nennt die physische Tatsache, die eine Wiedergabe erlaubt oder verbietet:

| Klasse | Gründe | Bedeutung | Gecacht |
|---|---|---|---|
| `deterministic` | `pure-function-of-inputs`, `check-without-outputs` | Reine Funktion der deklarierten Inputs; jeder deklarierte Output ist ein einfacher wiederherstellbarer Baum oder es gibt keinen. | ja, die einzige wiedergebbare Klasse |
| `driver-owned` | `freshness-owned-by-driver` | Der Befehl kompiliert über den Build-Treiber, dessen Cargo-Frische und Stage-Cache maßgeblich sind (Playground `prepare-*` und `build-*-react*`, `component`). | nein |
| `managed-output` | `outputs-are-store-links`, `outputs-in-managed-store` | Outputs liegen im geleasten Produkte-Store hinter einem Bindungs- oder Manifest-Protokoll (TypeScript-Artefakt-`build`, Wasm-Slots), und das Manifest schreibt kein `cache: true`; eine wiederhergestellte Kopie würde abgelehnt oder umginge den Store. Mit `cache: true` ist das Target stattdessen `deterministic`, weil sein Beleg der wiederherstellbare Output ist (siehe Store-Belege). | nein |
| `continuous` | `serves-until-stopped` | Läuft bis zum Stopp. | nein |
| `side-effecting` | `publishes-live-state`, `publishes-remote`, `mutates-workspace` | Verändert Zustand außerhalb der deklarierten Outputs. | nein |
| `live` | `drives-running-process` | Beobachtet oder steuert einen laufenden Prozess (Namen mit `e2e` oder `live`). | nein |
| `aggregate` | `orders-prerequisites` | Ordnet Voraussetzungen und leistet keine eigene Arbeit. | nein |
| `diagnostic` | `admission-refusal`, `measures-environment` | Meldet eine Ablehnung oder Messung und wird immer neu bewertet. | nein |

Der Bazel-Adapter liest die Entscheidung (`bazelTargetCacheClassV1` validiert sie gegen das geschlossene Schema und lehnt eine ungültige mit „Closed repository cache decision required“ ab) und leitet daraus seine Cache-Entscheidung ab, siehe unten. Die Nx-Projektion liest sie nicht; Nx entscheidet aus `cache`, `continuous` und `outputs`. Ein Manifest schreibt sie nicht; die synthetisierten Playground-Targets, die TypeScript-Artefakt-Targets `test`, `check` und (mit `cache: true`) `build` sowie die Wasm-Slot-Erzeuger mit `cache: true` tragen `deterministic`, per Namensregel gecachte Targets tragen keine. Die Richtlinie, die `cache` und die Entscheidung setzt, folgt dieser Reihenfolge, die erste passende Zeile gewinnt (`📚️library/🏗️build/🕸️graph/📥️inference/🎯️target-policy`, Namen aus `📚️library/⚡️caching/🔣️policy.json`):

| Rang | Regel | Gecacht | Angehängte Entscheidung |
|---|---|---|---|
| 1 | `continuous: true` oder ein Name `dev`, `serve`, `watch`, `start`, `test-watch`, `daemon` (auch `<name>-…`) | nein | `continuous` |
| 2 | `cache: false` im Manifest | nein | keine |
| 3 | Ein Name der Liste `uncached` (`audit`, `bench`, `clean`, `deploy`, `deps`, `install`, `publish`, `seed`, `setup`, `test-e2e`, `update`, …) oder ein exakter Name (`format`, `commit`, `os`, `parity`, …); ein geschriebenes `cache: true` wird überstimmt | nein | `live` (`e2e`, `live`, `os`, `semio`), `diagnostic` (`audit`, `bench`, `doctor`, `parity`, …), `side-effecting` (`publish`, `deploy` als `publishes-remote`, sonst `mutates-workspace`) |
| 4 | `hub-live-catalog-check`, `hub-live-catalog-native-check`, `hub-live-catalog-oracle` | ja | keine (Ausnahmen von Rang 5) |
| 5 | Ein durch Bindestriche getrenntes Wort `clean`, `gc`, `prune`, `setup`, `fuzz`, `e2e` oder `live` im Namen | nein | wie Rang 3 |
| 6 | Eine deterministische Familie: `test`, `check`, `lint`, `typecheck`, `format-check`, `generate`, `schema`, `verify`, `build`, `wasm`, `native-build`, `package`, `*-contract`, … | ja | keine; Standard-Inputs `default` und `^default`, Outputs `[]`, sofern das Manifest nichts anderes sagt |
| 7 | Alles andere | **ja**, außer bei `cache: false` | keine |

Rang 7 ist unter Nx fail-open (Bazel verlangt zusätzlich ein `outputs`-Array und eine Klasse, die Wiedergabe erlaubt, siehe seine Cache-Entscheidung): Eine Task, die Host-Zustand berührt und nichts sagt, wird gecacht, und ein Cache-Treffer überspringt ihre Wirkung. Schreibe `cache: false` bei Tests, die echten Speicher-, Lease-, Prozess- oder Toolchain-Zustand prüfen, bei Läufen gegen laufende Hosts, Browser oder Server, bei Schreibern außerhalb ihrer deklarierten Outputs, bei Startern langlebiger Werkzeuge, bei Netzwerk-Downloads und bei Schritten mit unbegrenzter Eingabemenge.

**Store-Belege.** Produkte, die in einen geleasten, inhaltsadressierten Store statt ins Projektverzeichnis veröffentlicht werden (Cargo-Artefakte im Compiler-Store, Vite-Sites, Wasm-Pakete, TypeScript-Artefakte), lassen sich nicht durch Kopieren von außen in diesen Store wiederherstellen. Sie deklarieren einen Beleg als einzigen Output, `{projectRoot}/.🧾️store/<target>.json` (Schema `📚️library/🏗️build/🧾️store-receipt/🧬️schema/🔣️.json`, git-ignoriert, bewusst nicht unter `dist`, das bei TypeScript-Artefakten ein Link in den Store ist). Der Beleg nennt jeden veröffentlichten Store-Pfad (`entries`: Art, Bytes, Dateien, SHA-256; der Digest eines Verzeichnisses hasht seine sortierte Dateiliste) und jeden Link, der einen Projektpfad an einen Store-Pfad bindet (`links`). Die zentralen Veröffentlicher schreiben ihn nach der Veröffentlichung, es ändert sich also kein Build-Skript: das Artefaktziel, über das die Vite- und Wasm-Veröffentlicher schreiben (`⚡️caching/📦️artifacts/🟦️.ts`), der Compiler-Store-Veröffentlicher von Cargo-`build` (`⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`, übersprungen, wenn ein Beobachtungs-Output angefordert ist) und der Binder der TypeScript-Artefakte (`⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🟦️.ts`). Beide Adapter übergeben den Ort des Belegs nur der einen Task, die ihn deklariert (`SEMIO_TASK_RECEIPT`, ausgestellt von `issueStoreReceiptV1` in `📚️library/🏗️build/🧾️store-receipt/🟨️.mjs`; Nx: `⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs`, Bazel: `executeTask` in `📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`); bei jeder anderen Task ist er ungesetzt, eine verschachtelte Task schreibt also nie in den Beleg ihres Elternteils, und ein Veröffentlicher tut ohne ihn nichts (`recordStoreContentV1`).

Nach jedem `run` oder `run-many` prüfen beide Werkzeuge den Beleg jeder gewählten Task und ihrer Voraussetzungen. Ein fehlender, beschädigter oder auf einen fehlenden oder geänderten Store-Eintrag zeigender Beleg gilt als Fehltreffer: Diese eine Task läuft allein und ungecacht erneut (Nx: `run <task> --skip-nx-cache --excludeTaskDependencies`, die eine zusätzliche Kindauswahl, die der Aufrufer zulässt und die auch den Cache-Eintrag ersetzt; Bazel: im Prozess mit `skipCache` und ohne Abhängigkeiten, wobei der veraltete Disk-Cache-Eintrag bestehen bleibt, sodass ein späterer Treffer wieder repariert) und wird nochmals geprüft, sonst schlägt der Aufruf geschlossen fehl. Ein gescheiterter erster Lauf wird nach der Reparatur einmal wiederholt. Eine Task, die einen Beleg deklariert, deren Handler aber keinen schrieb, ist ein Fehler und wird nicht stillschweigend akzeptiert. TypeScript-Artefakt-`build` und Wasm-Slot-Erzeuger tragen den Beleg als Output: Sie werden wiedergegeben, wenn das Manifest `cache: true` schreibt (Entscheidung `deterministic`), sonst bleiben sie `managed-output`. Eine Task deklariert einen Beleg nur, wenn ihre `outputs` den Belegpfad nennen (`declaredStoreReceiptV1`): `check`, `test` und `lint` liefern ein Urteil statt Store-Inhalt, und `publish` sowie der Kurs-`build` nennen andere oder keine Outputs. Abwechselnde `prototype`- und `release`-Läufe eines Targets teilen sich eine Belegdatei (ihr Pfad enthält keinen Modus), ein Wechsel lässt das Target daher voraussichtlich einmal neu laufen (aus der Pfadregel gefolgert, nicht beobachtet). Noch nicht verifiziert: Kein Lauf hat den Reparaturpfad ausgeübt (erster Nachweis: `@teaching/proctor:build` zweimal ausführen, den Slot `published/<hash>` des Compiler-Stores löschen, erneut ausführen und eine Zeile `[store-receipt] re-executing` erwarten, mit `--tool=nx` und `--tool=bazel`).

Die Bazel-Implementierung entscheidet je Action aus `cache`, `continuous` und `outputs` und ist bei fehlenden Deklarationen strenger (siehe ihre Cache-Entscheidung unten).

### Nx-Implementierung

Nx ist der Standard. Das Repository nutzt keine Nx-Projektentdeckung, sondern speist den neutralen Graphen in Nx ein.

- **Plugin.** `nx.json` registriert `@repo/emoji-project-json` (`📚️library/🟨️.mjs`) für `📋️project.json`; `createNodesV2` veröffentlicht den neutralen Graphen als Nx-Projekte, `createDependencies` die neutralen Kanten und lehnt Kanten unbekannter Projekte ab. Jedes Target wird auf einen Nx-Executor abgebildet und erhält eine private Engine-Identität als zusätzlichen Input, damit der Wechsel der ausführenden Implementierung Ergebnisse ungültig macht.
- **Executors.** `@semio-tech/repo-lib:command` und `@semio-tech/repo-lib:owner-command` (`📚️library/📦️packages/🟦️typescript/📋️executors.json`) `command` führt die wörtlichen Argumente über die ursprüngliche Prozessautorität und die Bun-Shell aus, `owner-command` leitet die wörtliche argv ohne Shell-Interpolation an den nativen oder Prozess-Eigentümer weiter (Beschreibungen in `executors.json`); `repo:noop` wird zu `nx:noop`.
- **Übersetzung.** `--skip-cache` → `--skip-nx-cache`, `--exclude-dependencies` → `--excludeTaskDependencies`, `--output` → `--output-style`, `--bail` → `--nx-bail`, `--dry-run` und ein bloßes `--graph` (oder `--graph=true`) → `--graph=stdout`, `--graph=<file>` bleibt. `exec` führt den Befehl in einem Bun-Wrapper aus, der die Namen von Nx' eigenem `exec` (`NX_PROJECT_NAME`, `NX_PROJECT_ROOT_PATH`) auf die neutralen `SEMIO_PROJECT_NAME` und `SEMIO_PROJECT_ROOT` kopiert; die Nx-Namen existieren nur in dieser Implementierung. `affected` ohne Selektor ergänzt `--base=HEAD`, `--uncommitted --untracked` wird zu `--base=HEAD`, `--all` zu `run-many`.
- **Grenze.** Produktionscode außerhalb von `⚡️caching/🚀️bootstrap` lädt nie ein Nx-Modul, sondern geht durch den typisierten Port `⚡️caching/🚀️bootstrap/🛠️tools/🔌️sdk` mit einem geschlossenen Katalog von 13 Nx-Modulen. Ein Test scannt alle Produktionsdateien unter `🦑️repo/🔨️modules` mit eigenem Tokenizer und dem TypeScript-Compiler als unabhängigem Orakel und verlangt null Verstöße.
- **Cache-Ort.** Nx-Ergebnisse liegen in `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` (`cacheDirectory` in `nx.json`; die Rust-Partition, 16 GiB, die auch die gemeinsamen Cargo-Target- und Build-Verzeichnisse aus `.cargo/config.toml` hält). Die getrennte Partition `n` (`.cache/b/n`, 1,5 GiB) ist der Nx-Pool: Sie hält den Nx-Graph-Cache als finanzierte Kind-Images (`⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph`), nicht die Task-Ergebnisse. `cache status` läuft über das Artefaktverzeichnis; `cache clear` führt `nx reset --onlyCache` im begrenzten Namensraum aus.
- **Noch nicht verifiziert.** Der Gesamtablauf (ein echter Kaltlauf, der `nx/artifacts` füllt, dann ein Warmlauf daraus) ist skriptiert, wurde zum Zeitpunkt dieser Dokumentation aber nicht ausgeführt; die Unit- und Vertragstests der Einzelteile bestanden am 2026-10-19 (`⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations`, `🔌️sdk`, `🔁️probe`, `🏗️build/🧭️selection`, `🏗️build/🧭️invocation`: 23 Tests, 0 Fehler).

### Bazel-Implementierung

`--tool=bazel` (oder `SEMIO_BUILD_TOOL=bazel`) wählt Bazel 8.4.2 (`.bazelversion`, festgelegter Download je OS und CPU). Der Adapter liegt in `📚️library/🏗️build/🔧️tools/🏗️bazel`. Er macht aus den gewählten Tasks echte Bazel-Actions, sodass Action-Cache, Disk-Cache, optionaler Remote-Cache und lazy Auswertung von Bazel selbst kommen.

- **Generierter Workspace.** Je eigenständiger Auswahl entsteht `<products>/bazel/workspaces/<namespace>` mit `MODULE.bazel`, `BUILD.bazel`, den Regeln in `tasks.bzl` (`repo_task`, `repo_launch`, `repo_noop`), je Action einem Manifest `<label>.json` und einem Fingerabdruck `<label>.input` sowie `s`, einem Link auf den echten Checkout (unter Windows eine Junction). Das Label einer Task ist `t_` plus 16 Hexziffern des SHA-256 der Task-ID.
- **Minimale Builds.** Nur die gewählten Tasks und die über `dependsOn` erreichbaren werden zu Actions; gebaut wird genau `bazel build` der gewählten Labels. Reine Reihenfolge-Tasks (`repo:noop`) verschwinden hinter ihren Erzeugern, es sei denn, sie sind selbst gewählt. Ein nicht benötigtes Target wird nie analysiert oder ausgeführt.
- **Deklarierte Inputs.** Die `inputs` einer cachebaren Task werden gegen die Dateien des Checkouts aufgelöst und als Bazel-Inputs deklariert: `srcs` ist eine `filegroup` aus Labels `s/<pfad>`. Pfade, die Bazel nicht als Label benennen kann (label-unsichere Namen, Verzeichnisse, hängende Links, alles unter einem verschachtelten `BUILD`-Paket), gehen stattdessen in den Fingerabdruck ein; dieser enthält außerdem Plattform, CPU, Bun-Version, Build-Modus, deklarierte Umgebungswerte und `runtime`-Ausgaben. Das Action-Manifest ist inhaltsstabil: Es nennt Task und ausführende Implementierung, aber keinen Workspace-, Laufzeit- oder Snapshot-Pfad; die Live-Daten stehen in `runtime.json` außerhalb jedes Schlüssels.
- **Deklarierte Outputs.** Jede Task hat genau einen deklarierten Tree-Artifact-Output `<label>.snapshot`. Ein Verbraucher erhält die Snapshots seiner Erzeuger (bei `transitive: true` den transitiven Abschluss) als Inputs. Nach dem Build werden die Snapshots der gewählten Tasks in den Checkout wiederhergestellt.
- **Cache-Entscheidung (fail-closed).** Jede Action trägt `cache` und, wenn sie ungecacht ist, einen maschinenlesbaren Grund; die Starlark-Regel scheitert ohne einen und versieht die Action mit `no-cache`. Der Adapter entscheidet in dieser Reihenfolge (`bazelCacheDecisionV1`):
  1. Ein ausdrückliches `cache: true` gegen eine Plugin-Klasse außer `deterministic` ist ein Konflikt und wirft („Conflicting cache declaration“), außer bei einer reinen Reihenfolge-Task.
  2. `continuous` (ein Dienst) ist ungecacht mit dem Grund `continuous`.
  3. `--skip-cache` ist ungecacht mit dem Grund `skipped`.
  4. Eine reine Reihenfolge-Task (`repo:noop`) ist immer cachebar.
  5. Eine Klasse außer `deterministic` (siehe Cache-Klassen) ist ungecacht, der Klassenname ist der Grund: `driver-owned`, `managed-output`, `side-effecting`, `live`, `aggregate`, `diagnostic`.
  6. `cache: false` im Manifest ist `declared` und gewinnt auch gegen eine Klasse `deterministic`.
  7. Weder `cache: true` noch Klasse `deterministic` ist `undeclared`: die Task läuft immer, bis eines deklariert ist.
  8. `cache: true` oder Klasse `deterministic` ohne `outputs`-Array ist `unbounded`: ein Treffer könnte nicht wiederherstellen, was erzeugt wurde.
  9. Alles andere ist cachebar.

  Deklariere also `outputs` (für reine Prüfungen `[]`) an jedem cachebaren Target. Kontinuierliche Tasks werden zu `repo_launch`-Actions, laufen mit `bazel run`, ihre Bereitschaft wird abgewartet, und sie teilen sich den Server des Builds.
- **Ausführungsanforderungen.** Jede Task- und Launcher-Action trägt `no-remote-exec` (der `repo_noop`-Marker schreibt nur eine Datei und startet keinen Prozess); Actions mit `host_state` zusätzlich `no-sandbox`, heute ist das jede Task- und Launcher-Action, nur der `repo_noop`-Marker ist sandboxfähig (`STARLARK_RULES` in `🏗️build/🔧️tools/🏗️bazel/📜️script.ts`; `Actions.kinds` in `🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json`). Gründe laut Schema (`ActionPolicy.kinds`): Eine *Task*-Action führt den Task-Prozess im echten Checkout aus, mit Arbeitsverzeichnis, deklarierten Schreibzugriffen auf Quellbaum und gemeinsamen Store, dem Loopback-Steuersocket und dem Laufzeitkanal außerhalb des Baums, was eine Sandbox verbergen oder blockieren würde; eine *Launcher*-Action bindet exakte Execroot-Output-Pfade und erhält Autorität über den Steuersocket der Invokation, was ein Sandbox-Execroot ändern würde. Folge: Ergebnisse sind durch deklarierte Inputs und Fingerabdruck bestimmt, nicht durch das, was eine Sandbox beobachtet hat; eine fehlende Input-Deklaration zeigt sich als veralteter Treffer, nicht als Fehler. Die `.bazelrc` des Repositorys setzt heute kein `--spawn_strategy`, `--sandbox_*` oder `--strategy` (der Test „the repository .bazelrc carries exactly the authored static cache policy and no host paths“ in `🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests` legt ihre Cache-Zeilen fest und lehnt Host-Pfade ab, verbietet aber kein Sandbox-Flag); jede Action, die je ohne `host_state` deklariert wird, läuft also in Bazels Standard-Sandbox.
- **Sandbox und Host-Pfade.** Bazel 8.4.2 unter macOS behandelt Nicht-ASCII-Pfade seines eigenen Zustands falsch, an zwei durch Experimente gefundenen Stellen: Eine `--install_base` mit Emoji oder Akzent lässt den Server beim Start abstürzen („Could not find or load main class“), und das `darwin-sandbox`-Profil wird aus Bazels byteweisen Pfadstrings geschrieben, aber jedes Byte ein zweites Mal als UTF-8 kodiert, sodass die Schreibregeln für eine Nicht-ASCII-`--output_base` nie passen und jede Sandbox-Action (auch ihre `stats.out`) mit „Operation not permitted“ scheitert. Es wirkt nur ortsabhängig, weil `/private/tmp`, `/private/var/tmp`, `TMPDIR` und `~/Library/Caches` für jede Schreibweise freigegeben sind. Das Produktionslayout hält die `install`-Basis, die `outputs`-Wurzel und die `server`-Output-Bases (`<products>/bazel/outputs/server/<hash>`) in den nativen Partitionen unter `.cache/b`, und die sind ASCII, solange der Checkout-Pfad ASCII ist (zum Beispiel `/Users/<name>/Documents/semio`). `requireBazelHostPathsV1` (aufgerufen von `bazelCacheLayoutV1` in `🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts`, das jeder Bazel-Speichernutzer aufbaut) lehnt unter darwin ein Layout ab, dessen Pfad `install`, `outputs` oder `server` ein Nicht-ASCII-Zeichen enthält („ASCII Bazel host path required on darwin: outputs“), und zwar beim Aufbau des Layouts statt mitten im Build. Halte den Checkout-Pfad unter macOS also ASCII. `--sandbox_base` wird nicht benutzt. Eine Sandbox-Action darf nur in ihr Execroot, die Temp-Verzeichnisse und `~/Library/{Caches,Logs,Developer}` schreiben; wird je eine Task sandboxed, die in den Checkout oder eine Partition schreibt, braucht sie `--sandbox_writable_path=<ASCII-Pfad>`, das denselben Kodierer durchläuft und deshalb ebenfalls ASCII sein muss. Linux (Bind-Mounts statt Profil) und Windows wurden nicht untersucht, die Sperre gilt nur für darwin; Windows deckt die Bereitschaftsprüfung für Nicht-ASCII-Pfade von Workspace und Partition ab.
- **Emoji in Quell-Labels.** Task-Quellen heißen `s/<pfad>` (`bazelSourceLabelV1`): die Datei hinter dem Link `s`, ein Target-Name im Wurzelpaket. Beobachtet mit dem festgelegten 8.4.2 (der Test mit dem echten Binary in `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions` spielt die Fixture-Namen ab und wird ohne `SEMIO_TEST_BAZEL_BINARY` übersprungen): Bazel akzeptiert in Target-Namen jedes Nicht-ASCII-Zeichen, auch astrale Emoji, ZWJ-Sequenzen, Hautfarben, kombinierende Zeichen, CJK und das geschützte Leerzeichen, auch als Action-Input- und -Output-Namen; abgelehnt werden nur Backslash und Steuerzeichen. Nur Paketnamen müssen ASCII sein, der Adapter braucht also nie ein Emoji-Paket; ein Verzeichnis mit `BUILD` oder `BUILD.bazel` gilt ihm unabhängig von der Schreibweise als Paketgrenze, alles darunter geht in den Fingerabdruck (`bazelSourceClassesV1`). Der Adapter lässt einen Pfad als Label zu, wenn er `^(?:[A-Za-z0-9_@=,+~%./\-]|[^\u0000-\u007f])+$` entspricht (codepunktbasiert, gilt also auch in Python, Rust und Go), und nimmt alles andere (Leerzeichen, Backslash, Steuerzeichen, Verzeichnisse, hängende Links, alles unter einem Verzeichnis mit `BUILD`) stattdessen in den Fingerabdruck auf. In einem Scratch-Experiment waren Disk-Cache und Cache-Schlüssel unter `LANG=C` und einer UTF-8-Locale identisch (kein Test deckt das ab); der Adapter setzt unter darwin eine UTF-8-Locale (`LANG` und `LC_ALL` `en_US.UTF-8` in `runBazelTasksInStorageV1`).
- **Disk-Cache.** `<products>/bazel/actions` mit Garbage-Collection (`--experimental_disk_cache_gc_max_size` gleich 40 Prozent des Budgets der Produkte-Partition, auf MiB abgerundet; `.bazelrc` ergänzt `--experimental_disk_cache_gc_idle_delay=10s` und `--experimental_disk_cache_gc_max_age=30d`). Die Sammlung läuft nur in einem dauerhaften, untätigen Server (gemessen mit Bazel 8.4.2: unter `--batch` lief sie nie), deshalb nutzt der Adapter für Builds kein `--batch`. Der Repository-Contents-Cache behält Einträge 3 Tage. Modulkatalog und Repository-Cache liegen in `<dependencies>/bazel/modules`; Builds laufen mit `--repository_disable_download` und laden keine Module nach.
- **Gemeinsame Output-Base.** `--output_base` ist `<products>/bazel/outputs/server/<erste 16 Hexziffern von sha256(Workspace-Verzeichnis)>`: ein Server je generiertem Workspace-Verzeichnis, geteilt von Build und allen `bazel run` kontinuierlicher Tasks dieser Auswahl; er endet nach 120 Sekunden Leerlauf. Der Disk-Cache wird von allen Auswahlen geteilt, deshalb nutzt eine zweite Auswahl die Ergebnisse der ersten.
- **Remote-Cache-Schalter** (standardmäßig aus; solange der Endpunkt fehlt, wird kein Flag übergeben): `SEMIO_BAZEL_REMOTE_CACHE` (Endpunkt `grpc://`, `grpcs://`, `http://`, `https://` oder `unix:`); `SEMIO_BAZEL_REMOTE_CACHE_UPLOAD` (`true` lädt lokale Ergebnisse hoch, sonst nur lesen, weil die Actions ohne Sandbox laufen und einen geteilten Cache nicht vergiften dürfen); `SEMIO_BAZEL_REMOTE_CACHE_CREDENTIAL_HELPER` (absoluter Pfad oder Befehlsname, keine Geheimnisse in Argumenten). `--skip-remote-cache` lässt den Endpunkt für einen Aufruf weg, `--skip-cache` ergänzt `--noremote_accept_cached`. Ungültige Werte werden abgelehnt.
- **Cache-Operationen.** `bun repo --tool=bazel cache status` zählt die Dateien der Ordner `ac` und `cas` des Disk-Caches und ergänzt die aufgezeichnete Build-Aktivität (Builds, Actions, ausgeführt, Cache-Treffer), die jeder Build aus seinem Build-Event-Stream in `<products>/bazel/outputs/server/metrics.json` fortschreibt. `cache clear` stoppt die dauerhaften Server (durch Entfernen ihrer pid-Datei), entfernt ihre Output-Bases (Bazels lokalen Action-Cache) und leert den Disk-Cache, sodass der nächste Build ein Fehltreffer ist; Bazel-Binary, Module und Toolchains bleiben.
- **Speicher.** Bazel braucht die Partitionen tools, dependencies und products (`bazel/bin`, `bazel/install`; `bazel/modules`; `bazel/workspaces`, `bazel/outputs`, `bazel/actions`). `bun repo --tool=bazel reset` verwirft den Graphen und leert `bazel/actions`, `bazel/outputs` (mit den Server-Output-Bases und dem Aktivitätsbuch) und `bazel/workspaces`; es stoppt einen laufenden Bazel-Server nicht, führe es nur aus, wenn kein Build läuft (`cache clear` stoppt sie).
- **Verifiziert und noch nicht verifiziert.** Bazel selbst wurde unter macOS arm64 mit dem festgelegten 8.4.2 in Scratch-Workspaces über die Tests mit `SEMIO_TEST_BAZEL_BINARY` geprüft. Am 2026-10-19 erneut gelaufen und bestanden: der Test mit dem echten Binary in der Storage-Suite (Sandbox-Build, Disk-Cache-Treffer nach `bazel clean`) und der Emoji-Label-Test der Actions-Suite. Früher aufgezeichnet, jetzt nicht erneut gelaufen: `cache status` und `cache clear` gegen die echte Produkte-Partition, die Host-Pfad-Sperre und `affected` gegen `bazel query rdeps`. Die Storage- und Operations-Suiten (Cache-Flags, Layout, Remote-Schalter, Cache-Operationen, Trockenlauf- und `affected`-Gleichlauf) bestanden am 2026-10-19 ohne das echte Binary; die Action-Suiten (Cache-Entscheidungs- und Klassen-Orakel, Wiederherstellung, Input-Auswahl, exakter `BUILD.bazel`-Text, Starlark-Syntax mit einem unabhängigen Python-Interpreter geprüft) brauchen eine ausgestellte Prozesshülle und laufen nur über die aggregierte Suite. Diese aggregierte Suite (`bun repo run workspace:test-build-tool-bazel`, also `🏗️build/📜️script.ts test bazel`) ist nicht grün: Am 2026-10-19 scheiterte ihr erster Test, `Bazel component contracts have one canonical owner with unchanged normalized constraints`, weil zwei normalisierte Schema-Constraints von ihrer Baseline abweichen (`unchanged` ist false), und ein fehlschlagender Test schließt die gemeinsame Hülle für seine Nachfolger (41 ihrer 78 Tests bestanden, 34 scheiterten). Ein früheres Hindernis, ein Build-Graph-Import der groben Sammeldatei `@semio-tech/repo-lib/storage`, der `typescript` in den Bazel-Abschluss zog, besteht nicht mehr. Noch nicht verifiziert: jeder echte `bun repo --tool=bazel run` eines Produkts (weder ein Kaltlauf, noch ein Warmlauf aus dem Disk-Cache, noch ein Lauf nach unabhängiger Änderung), das Aktivitätsbuch nach einem echten Lauf (`cache status` muss `activity.builds >= 1` zeigen), der CLI-Pfad von `cache clear`, Linux und Windows (auch die Emoji-Label-Regeln und die Sandbox-Kodierung dort) sowie der Devcontainer.

### Befehlsregister

Das Befehlsregister ist die Befehlsoberfläche der Entwickler: Jeder ausführbare Befehl wird einmal neben seinem Quelltext deklariert und von Entwicklern im Dashboard (`bun run dashboard`, **New Task**) gewählt; Agenten und Automation verwenden denselben Daemon über den Client `semio`; keine IDE-Launch-, Task- oder Solution-Datei registriert Befehle (`.vscode/` existiert im Baum nicht). Das Register besteht aus Daten: Die `📋️project.json` an der Wurzel enthält `metadata.semio.dashboard` mit `parameters`, `tools`, `compounds` und `groups`, geprüft gegen das Schema `🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` (`ProjectManifest`); andere Projekte deklarieren typisierte Steuerelemente an ihren Targets, Playground-Steuerelemente stammen aus Crate-Manifesten. Der [Dashboard-Leitfaden](🔨️modules/🎛️dashboard/README.md#-daily-work) führt die täglichen Zeilen auf; der Build-Teil ist:

| Tool-ID (`tool:workspace/…`) | Führt aus | Hinweise |
|---|---|---|
| `build-task`, `run-targets`, `build-affected`, `build-watch` | `bun repo run`, `run-many`, `affected`, `watch` | `run-targets` und `build-affected` haben wie `build-task` ein Flag `dry-run` (`--dry-run`). |
| `build-exec` | `bun repo exec --projects={projects} --dry-run={dry-run} --` plus die Befehlswörter in **Arguments** | `dry-run` ist eine Auswahl (`false` Standard, `true`) innerhalb der Wörter vor `--`, weil Argumente nach `--` zum Kindprogramm gehören; `--dry-run=false` ist wirkungslos. `bun-test`, `run-script` und `native-cargo` bieten dieselbe Auswahl. |
| `build-cache-status`, `build-cache-clear` | `bun repo cache status`, `cache clear` | Die Achse `build-tool` wählt, wessen Cache. |
| `build-preflight`, `build-deps-sync`, `build-deps-lock` | `bun repo preflight`, `dependencies sync --all`, `dependencies lock --all` | Host-Prüfung; Installation und Sperre für alle Eigentümer (`--scope <owner>` bleibt auf der Kommandozeile). |
| `build-graph`, `build-projects`, `build-targets`, `build-project`, `build-help`, `build-reset`, `build-tools`, `cargo` | `graph`, `show …`, `--help`, `reset`, `tools setup`, `bun repo cargo` | |

Globale Achsen (Parameter) des Wurzelregisters: `build-tool` (`SEMIO_BUILD_TOOL=nx|bazel`), `cache` (`use`, `skip-local` = `--skip-cache`, `skip-all` = `--skip-cache --skip-remote-cache`), `build-mode=ship` (Release-Build, beide Caches übersprungen), `dependencies` (`--exclude-dependencies`), `test-level`, `nextest-output`, `cargo-jobs`, `build-budget`. `build-tool` wird bei jedem Verb angeboten, das Build-Tasks ausführt, auch bei `publish`, `prepare`, `deploy`, `activate`, `serve`, `reset` und `inspect`; dieselbe Task läuft also im Dashboard unter Nx oder Bazel. Ein Gesetz vergleicht seine Verben mit den Verben des Rust-Registers (`VERBS`) plus dem Rückfall `task` plus `reset` und `inspect`. `cache` wird bei `dev`, `run`, `watch`, `clean`, `reset`, `inspect` und `setup` nicht angeboten, die kontinuierlich sind oder keinen Task-Cache haben. Eine frische native Build-Generation ist ein Registerparameter `generation=fresh` (`SEMIO_NATIVE_BUILD_GENERATION=fresh`) des Play-Targets `build` und des Wurzel-Targets `release`, kombiniert mit `build-mode=ship`.

Befehle des Wurzelskripts (`bun ./📜️script.ts verify …` und ähnliche) werden über Targets geleitet, die `metadata.semio.workspaceCommand` deklarieren: ein Array aus mindestens zwei Wörtern aus `[a-zA-Z0-9-]`, im ganzen Repository eindeutig (zum Beispiel `verify fem-combination-stress`). Eine fehlerhafte oder doppelte Deklaration lässt den Routenlader werfen („Invalid owned command“, „Duplicate owned command“) und bricht jede Route `verify …`; deshalb werden die Deklarationen des ganzen Baums gemeinsam geprüft.

Wähle im Dashboard **New Task** → `@semio-tech/repo-lib:test-dashboard-control`, um das Befehlsregister zu prüfen. Die Prüfung vergleicht das Wurzelregister mit Ajv 2020 und dem eigenen Schema-Compiler, neutrale Eigentümer-Fixtures mit SQLite und den Verben des Rust-Registers sowie Vorschauargumente mit `validateBuildTaskArgumentsV1` und `nxBuildArgumentsV1`. Ausgabe und Ergebnis bleiben an der Task. Agenten und Automation erreichen denselben Eigentümer über `bun repo run @semio-tech/repo-lib:test-dashboard-control`. Das Target `@semio-tech/repo-dashboard-rs:test-journeys` prüft zusätzlich den installierten nativen Client und tatsächliche Terminalabläufe.

### Build-Owner-Register

`📋️build-owners.json` an der Wurzel führt jedes Modul auf, das ein Build-Manifest besitzt (`owners`), und die Katalog-Provider. Ein Gesetz hält es gleich dem Baum: `bun ./📜️script.ts verify build-owners` (Wurzel-Targets `workspace:verify-build-owners`, gecacht mit `outputs: []`, sowie bloßes `verify`/`workspace:verify`) scheitert mit einem Befund je Abweichung, zum Beispiel `project-without-row`, `cargo-without-row`, `missing-manifest`, `name-mismatch`, `cargo-row-shadows-project` oder `package-stale` (`📚️library/🏗️build/🧬️schema/🧾️owner-consistency.json` nennt alle Arten); `verify build-owners sync` (Target `workspace:verify-build-owners-sync`, ungecacht) repariert das Mechanische (entfernt Zeilen verschwundener Module (nur wenn kein Manifest existiert und git am Pfad nichts verfolgt), verschiebt ein bewegtes Modul, benennt ein umbenanntes Projekt um, übergibt eine Cargo-Zeile an eine `📋️project.json`, fügt Zeilen für neue Manifeste in Byte-Reihenfolge ein) und scheitert danach an dem, was bleibt: Befunden, die einen Autor brauchen (Katalog-Provider), und Zeilen, deren Pfade git noch verfolgt, obwohl die Dateien fehlen (eine laufende gleichzeitige Verschiebung). Zeilen, deren Manifest unter einem git-ignorierten Pfad liegt, sind ausgenommen, damit ein frischer Klon das Gesetz nicht auslöst. Die Gerüst-Schreiber (Crate-Carve, Subset-Gerüst, Mutationsbaum) rufen denselben Abgleicher mit einem Geltungsbereich auf. Ein angewandter Verschiebeplan gleicht das Register für die berührten Teilbäume ab (`reconcileBuildOwnerRegistry` in `📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts`); jede andere Verzeichnisverschiebung oder -umbenennung fängt das Gesetz, und `verify build-owners sync` behebt sie. Kern und Fixture-Fälle liegen in `📚️library/🏗️build/🕸️graph/🧾️registry-consistency`; die Registerdatei hält eine Zeile je Modul mit Manifest (3718 Zeilen), und `bun ./📜️script.ts verify build-owners` gab am 2026-10-19 `passed` aus.

Die Werkzeuge, die vor der Installation der Abhängigkeiten laufen (`bun repo` Hilfe, Preflight, `tools setup`, `dependencies lock` und `sync`, die nativen Installer, die Wasm-Werkzeuge, der Bazel-Bootstrap sowie Router, `setup` und Entrypoint-Routen des Wurzel-Skripts), müssen auf einem Checkout ohne `node_modules` laufen. Ein Gesetz hält ihre Import-Hüllen relativ: `bun ./📜️script.ts verify zero-touch` (Wurzel-Target `workspace:verify-zero-touch`, gecacht, sowie bloßes `verify`/`workspace:verify`) durchläuft die Hüllen aus `🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json` und scheitert bei einem bloßen Workspace-Specifier (`workspace-specifier`), jedem anderen externen Paket wie `typescript` (`external-package`), einem Hüllen-Eintrag oder einer Naht, die fehlt (`missing-entry`), und einem statischen Import einer Nach-Installations-Naht (`seam-static`, etwa die Graph-Inferenz, die Quellen über den TypeScript-Compiler liest; Nähte dürfen nur dynamisch geladen werden).

### Einrichtung je Plattform

`bun repo tools setup [nx|bazel|all]` stellt bereit, was die gewählte Implementierung braucht; `workspace:setup` hängt vom Target `deps-build-tools` ab, das `tools setup all` ausführt (neben den Sprachumgebungen, die es ebenfalls vorbereitet); der `postCreateCommand` des Devcontainers (`bun repo run workspace:setup`) und die nativen Bootstrap-Skripte stellen die Build-Werkzeuge also ohne eigenen Schritt bereit. `nx` installiert das festgelegte Nx als unveränderliches Toolset unter `.cache/b/t/m/nx/<digest>` und prüft `nx --version`. `bazel` prüft die Plattform, bevor ein Image angehängt wird, öffnet die drei Partitionen, lädt und verifiziert das festgelegte Bazel-Binary mit Fortschrittszeilen, bereitet Modulkatalog und Repository-Cache vor (offline-fähig, abbrechbar) und gibt `[bazel] ready …` sowie eine Zeile `[bazel] readiness {json}` aus. `all` ist Bazel, dann Nx.

| Host | Feste Cache-Images | Voraussetzungen | Stand |
|---|---|---|---|
| macOS (arm64, x64) | case-sensitive APFS-Images über `hdiutil` | Xcode Command Line Tools. Keine Rechte nötig. Die Bazel-Pfade `install`, `outputs` und `server` (alle unterhalb des Checkouts, also auch dessen Pfad) müssen ASCII sein: das festgelegte Bazel stürzt bei einer Nicht-ASCII-Install-Base beim Start ab, und sein macOS-Sandbox-Profil kodiert eine Nicht-ASCII-Output-Base falsch, sodass Sandbox-Actions mit „Operation not permitted“ scheitern; `requireBazelHostPathsV1` lehnt ein solches Layout ab. | Implementiert; primärer Entwicklungshost. |
| Linux nativ | Verzeichnisbasierte Partition: Anker `fixed.dir`, Volume `v` und leerer Platzhalter `m`; Anhängen und Lösen ist ein atomarer `renameat2(RENAME_EXCHANGE)` von `m` und `v` | Keine Rechte: kein Root, `CAP_SYS_ADMIN`, Loop-Gerät oder Mount-Werkzeug. Ein natives, Groß-/Kleinschreibung unterscheidendes Dateisystem mit atomarem Verzeichnisaustausch (`renameat2` `RENAME_EXCHANGE`, Linux 3.15 oder neuer; die Bazel-Meldung nennt ext4, xfs und btrfs; ext4 auf einem Docker-Volume und tmpfs wurden bei der Einführung des Backends von der Verzeichnis-Suite in einem unprivilegierten Linux-Container geprüft, hier nicht erneut). `preflight` testet es unter dem nächsten vorhandenen Vorfahren von `.cache`, die Bazel-Prüfung unter dem der Werkzeug-Partition, und beide geben den Fehler des Tests aus; die Groß-/Kleinschreibung wird bei der Geburt einer Partition nochmals geprüft. Die Kapazität ist Buchführung: das Journal lässt Reservierungen zu, und der Verzeichniszähler misst den echten Baum spätestens alle 16 MiB Zulassung oder alle 2 Sekunden, bei jedem Commit und vor jeder Ablehnung neu (`createNativeDirectoryMeterV1`; `meter` im Verzeichnis-Schema). | Unprivilegiert in einem Docker-`oven/bun`-Container auf einem Linux-Volume ausgeführt (Preflight, Leases-, Diagnose-, Abhängigkeits- und Werkzeug-Partition angelegt, angehängt und wieder geöffnet); `tools setup` und Publikation wurden nicht erreicht. Ein Bind-Mount von einem Host ohne Groß-/Kleinschreibung (Docker Desktop auf macOS oder Windows) wird absichtlich abgelehnt. |
| Devcontainer | wie Linux | Der Compose-Dienst deklariert kein `privileged`, keine Capability und kein Gerät (Container nach dem Pull dieser Änderung neu bauen); das Feature `docker-in-docker` aus `devcontainer.json` wendet das Devcontainer-CLI mit den eigenen Metadaten des Features an, die nicht untersucht wurden: Die Cache-Partitionen brauchen also keine Rechte, der Container als Ganzes ist aber nicht als unprivilegiert belegt. `.cache` (die nativen Partitionen) ist ein benanntes Volume je Checkout und nicht Teil des Bind-Mounts, weil `preflight` einen Bind-Mount von einem Host ohne Groß-/Kleinschreibung (Docker Desktop auf macOS oder Windows) ablehnt; das Image erzeugt `en_US.UTF-8` und deklariert 128 GB Speicher. `postCreateCommand` ist `bun repo run workspace:setup`. Siehe [Devcontainer-README](../../../.devcontainer/README.md). | Wie oben: im Container nicht verifiziert. |
| Windows nativ | verzeichnisbasierte Partitionen wie unter Linux (`fixed.dir`, `v`, `m`), getauscht durch einen journalisierten Zwei-Phasen-Rename mit Write-Through (`createNativeDirectoryJournaledPortV1`); `openNativePlatformStorageV1` wählt das frühere Backend `fixed.vhd` / `Mount-DiskImage` nicht mehr | NTFS mit Groß-/Kleinschreibung je Verzeichnis: Windows ab Build 17107, das optionale Feature `Microsoft-Windows-Subsystem-Linux` aktiviert und ein leeres Cache-Verzeichnis, an dem das Flag gesetzt werden kann; das Backend lehnt mit einer Meldung ab, die das Fehlende nennt. Aktivierte lange Pfade (`LongPathsEnabled`, `core.longpaths`; `setup git` und das Skript geben eine Bereitschaftszeile aus, und die Bazel-Prüfung lehnt ab, solange `LongPathsEnabled` aus ist) und eine kurze ASCII-Checkout-Wurzel (Bazel 8.4.2 startet nicht unter einem Nicht-ASCII-Pfad von Workspace oder Partition; die Prüfung lehnt vor jedem Download ab und nennt Zeichen und Index). Ein Terminal mit Administratorrechten verlangen `🔵️.ps1` (außer mit `-SkipRepoBootstrap`) und die Bazel-Bereitschaftsprüfung weiterhin; beide Meldungen nennen noch das abgelöste VHD-Anhängen als Grund, diese Anforderung ist also zu überprüfen. `🔵️.ps1` setzt `core.longpaths` und `core.symlinks` in der Git-Konfiguration des Klons, legt die Playwright-Browser unter `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` ab, setzt außerdem die Benutzer-Umgebungsvariable `PLAYWRIGHT_BROWSERS_PATH` und stellt die Werkzeugverzeichnisse im Benutzer-`PATH` nach vorn. | Nicht unter Windows ausgeführt. Der Wechsel auf das Verzeichnis-Backend ist jung und unverifiziert; diese Zeile ist vorläufig. |

Auf allen Hosts installieren die nativen Bootstrap-Skripte (`🔩️native/🥾️bootstrap/🐚️.sh` und `🔵️.ps1`) Bun, Node, Go, rustup und uv aus einem prüfsummenverifizierten Pin-Manifest (`🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`: Node 24.15.0, Go 1.26.9, rustup 1.29.0, uv 0.11.6 und der Commit von `dotnet-install.sh`, 24 Zeilen für darwin, linux und win32 auf x64 und arm64 plus eine plattformunabhängige Zeile), statt Installer in eine Shell zu leiten; Bun 1.4.2 kommt aus seiner eigenen Schema-Zeile und entspricht `packageManager`. Das Shell-Skript installiert auf apt-Hosts zusätzlich die Chromium-Systembibliotheken über Playwrights `install-deps chromium`. Weitere Windows-Pakete (Git, CMake, Build Tools, …) werden über winget installiert, wenn sie fehlen, und nicht versionsfest gepinnt. Bazel 8.4.2 hat eine geprüfte Release-Zeile für darwin, linux und win32 auf arm64 und x64, und kein Cache-Pfad darf 256 Zeichen überschreiten. Der Pull-Request-Workflow `.github/workflows/build-system.yml` hat zwei Jobs auf Ubuntu, macOS und Windows: `smoke` installiert das festgelegte Bun mit geprüftem SHA-256 und führt `bun repo --help`, `preflight`, `tools setup all`, `dependencies sync --all` und einen Cache-Nachweis für beide Werkzeuge auf `@teaching/architecture-course:test` aus (leeren, laufen, Status wächst, erneut laufen und einen Treffer erkennen, Status unverändert); `products` führt dieses Target danach unter `--tool=nx` und `--tool=bazel` aus. Er braucht weder `sudo` noch einen Installer. Noch nicht verifiziert: Der Workflow ist nie gelaufen, die Bazel-Trefferregel (eine kalibrierte Lesart von Bazels Konsolenzusammenfassung) und die Wiederherstellung des Bun-Stores und des Bazel-Disk-Caches in die eigenen Partitionen sind ungetestet.

### Produktketten

Alle Ketten laufen über den neutralen Einstieg, also mit `--tool=nx` (Standard) und `--tool=bazel`.

- **Kurs `@teaching/architecture-course`:** `typecheck` und `test` (gecacht) → `dev` (kontinuierlich: der Proctor, dann die Site, gemeinsam gestoppt; Site-Port 6061) → `test-e2e` (Playwright, Wegwerf-Stacks auf freien Ports, ungecacht) → `deploy-check`. `build` und `publish` sind im Manifest ungecacht. `preflight` ist der allgemeine Host-Preflight; die Docker-Image-Targets lehnen außerhalb eines nativen Linux-x64-Hosts ab (`🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🐳️docker-image`).
- **Proctor `@teaching/proctor`:** `build`, `test` (gecacht; das Binary liegt im Compiler-Store, `build` deklariert seinen Store-Beleg) → `dev` (kontinuierlich, Port 8791). Die Betriebsverben (`backup`, `restore`, `erase`, `prune`, `capacity`, …) sind ungecacht.
- **Play `@semio-tech/semio-tech-play`:** `prepare-prototype` → `activate-prototype` (gecacht, schreibt `dist/♻️activation/prototype`) → `serve` / `dev` (kontinuierlich, Port 6033). `prepare-e2e` → `serve-e2e` → `test-e2e` braucht die Playwright-Browser (`workspace:deps-browsers`). `build` (ungecacht) deklariert einen Store-Beleg als Output (`.🧾️store/build.json`).
- **Demonstrator `@semio-tech/mit-bestand-demonstrator`:** `prepare-prototype` → `activate-prototype` → `serve` / `dev` (kontinuierlich, Port 6029); gleiche E2E-Kette; `build` (ungecacht) braucht `prepare-release`.

Beispiel: `bun repo run @semio-tech/semio-tech-play:serve --tool=bazel`. Noch nicht verifiziert: Für diese Dokumentation wurde kein End-to-End-Lauf dieser Ketten unter einem der beiden Werkzeuge aufgezeichnet; Target-Namen, Abhängigkeiten, Ports und Cache-Flags sind den Manifesten entnommen.

### Verifikationsstand

Geprüft durch Tests: Operationskatalog, die Hilfe je Befehl auf Englisch und Deutsch (Schnappschuss je Befehl und Sprache), die dokumentierten Flags gegen den Task-Validator, die neutrale `exec`-Umgebung und die Ergebnisschemas stimmen überein (`📚️library/🏗️build/⚙️operations/🧪️tests`, einschließlich `preflight --help` über das echte CLI; 24 Tests mit der Store-Beleg-Suite, 0 Fehler am 2026-10-19); Selektion, Flags und die `affected`-Regel (`🏗️build/🧭️selection/🧪️tests`, `🏗️build/🧭️invocation/🧪️tests`); Nx-Übersetzung, Grenze und Probe (`⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations`, `🔌️sdk`, `🔁️probe`); Bazel-Cache-Flags, Speicherlayout, Host-Pfad-Sperre und Remote-Schalter (`🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests`); Bazel-Bereitschaft je Plattform nur mit Fixtures; Bazel-Actions, Inputs, Cache-Entscheidung, Wiederherstellung und Manifest (`🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions`; die aggregierte Suite war am 2026-10-19 nicht grün, siehe Bazel-Implementierung); Bazel-Cache-Operationen, Trockenlauf sowie `affected` und `exec` im Gleichlauf mit Nx (`🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests`); Cache-Entscheidungsklassen und Store-Belege (`🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache`, `🏗️build/🧾️store-receipt/🧪️tests`); Befehlsregister (`@semio-tech/repo-lib:test-dashboard-control` und `@semio-tech/repo-dashboard-rs:test-journeys`, siehe Befehlsregister) und Build-Owner-Register (`🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests`); Linux-Verzeichnispartitionen unprivilegiert in einem Docker-Container (Verzeichnis-, Preflight- und Bereitschaftssuiten). **Nicht verifiziert:** das echte Kalt- und Warmverhalten der Caches von Nx und Bazel, ein echter Bazel-Lauf eines Produkts, der Reparaturpfad der Store-Belege, die Einrichtung unter Windows und im Devcontainer sowie auf Linux über `tools setup` hinaus, der Workflow `build-system.yml`, die Reparatur des Abhängigkeits-Installers bei Provenance-Fehlern und die End-to-End-Ketten der Produkte.

### Cache-Partitionen und Wiederherstellung

Der gesamte Build-Zustand liegt in Partitionen fester Größe unter `.cache/b/<partition>` (git-ignoriert, abgeleitet, löschbar, solange nichts läuft): `d` Abhängigkeiten (8 GiB), `t` Werkzeuge (8 GiB), `g` Graph (4 GiB), `o` Beobachtungen und Diagnose (2 GiB), `p` Produkte (24 GiB), `l` Leases (64 MiB), `n` Nx-Pool (1,5 GiB); die Größen stehen in `📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`. `.cache/b/c` ist das einfache Steuerverzeichnis (kein Image, höchstens 1 MiB; es hält die Lease-Datenbank der Images, `budget.controlDirectory`), die Rust-Partition liegt getrennt unter `.🧬semio/🦑️repo/⚡️cache/rust/fixed` (16 GiB). Unter macOS ist jede Partition ein UDIF-Image mit case-sensitive APFS (`fixed.dmg`, bei `m` eingehängt, daneben der Beleg `image.json`; `ImageProtocol` legt `UDIF` und `Case-sensitive APFS` fest, HFS+ wurde früher ausgeschlossen, weil ihm der atomare Austausch `RENAME_SWAP` fehlt, den Veröffentlichung und Bindung nutzen, hier nicht erneut geprüft), unter Linux ein Verzeichnis ohne Rechte (`fixed.dir`, `v`, `m`, getauscht durch atomares `renameat2(RENAME_EXCHANGE)`), unter Windows ebenfalls ein Verzeichnis (journalisierter Zwei-Phasen-Tausch mit Write-Through statt `renameat2`; Linux und Windows aus dem Quelltext, nicht auf allen Hosts verifiziert). `bun ./📜️script.ts clean` berührt nie eine davon.

Wenn jeder `bun repo`-Aufruf beim Start ablehnt, prüfe die Punkte 1 bis 6 vor jeder Codeänderung (die `hdiutil`-Befehle gelten nur für macOS); Punkt 7 betrifft den Abhängigkeits-Installer. Stoppe zuerst alle `bun repo`-, Cargo- und Nx-Prozesse.

1. **„Pending Native image publication must settle“**: Ein abgebrochener Lauf hat `.cache/b/<partition>/.publication/intent.json` hinterlassen. Prüfe, dass kein Prozess lebt, verschiebe die Datei, starte neu.
2. **Beleg- oder Mount-Konflikt** („Exact canonical persistent Native publication required“, „Ambiguous Native backing kernel association“, ein `image.json` ohne sein `fixed.dmg`): Partition neu anlegen: `hdiutil detach .cache/b/<partition>/m || umount -f …`, `hdiutil info` (kein Image darf den Pfad noch beanspruchen), `rm -rf .cache/b/<partition>`, dann `bun repo --help && bun repo tools setup all`. Nie ein eingehängtes Partitionsverzeichnis mit `mv` verschieben. Unter Linux ist nichts eingehängt: überspringe die `hdiutil`-Zeilen, prüfe, dass `lsof +D .cache/b/<partition>` leer ist (ein atomarer Austausch gelingt auch mit offenen Dateien, anders als `umount`), und entferne das Verzeichnis. Eine Verzeichnispartition, die `preflight` ablehnt („Atomic directory exchange is unsupported by this kernel or filesystem“ oder ein Dateisystem ohne Groß-/Kleinschreibung), lässt sich durch Neuanlegen nicht reparieren: verschiebe `.cache` auf ein natives Dateisystem mit Groß-/Kleinschreibung, im Devcontainer auf sein Volume `.cache`.
3. **Verschachteltes Anhängen „Permission denied“**: Das Elternimage war `noowners` eingehängt. Ist nichts offen (`lsof +f -- .cache/b/<partition>/m` leer), `hdiutil detach` auf dem Elternimage; der nächste Aufruf hängt mit `-owners on` neu an.
4. **Vergifteter Nx-Pool** (jeder Graph- oder Nx-Aufruf scheitert mit „Exact original parent child allocation required“): `hdiutil detach .cache/b/n/m/<shard>/native/m`, dann das Shard-Verzeichnis mit `rm -rf` entfernen.
5. **ENOSPC in `d`**: `.cache/b/d/m/tmp` leeren (Installer-Reste), dann `dependencies sync --all` wiederholen.
6. **Vor dem APFS-Wechsel erzeugtes Image** (ENOTSUP bei rename): wie in Punkt 2 neu anlegen.
7. **`dependencies sync` scheitert mit `EPERM: Operation not permitted: failed to link package`** (macOS 27, auf arm64 beobachtet): Der Kernel verweigert Hardlinks und Clones von Dateien des Bun-Paket-Caches `.cache/b/d/m/bun-cache`, die das Attribut `com.apple.provenance` einer anderen Anwendungsidentität oder Sitzung tragen, während Dateien, die der aktuelle Prozess erzeugt hat, sich verlinken lassen (`ln` und `cp -c` einer alten Cache-Datei ergeben EPERM, eine frische Kopie lässt sich verlinken; `xattr -d` meldet Erfolg, das Attribut bleibt). Der Sync behandelt das jetzt: Er stuft diese Installer-Ausgabe als Cache-Link-Verweigerung ein, gibt `[DATA] Store dependency cache link refused; purging the Bun package cache and retrying` aus, leert `bun-cache` an Ort und Stelle (das Verzeichnis behält seine Identität; Fortschritt `dependencies:purge-bun-cache:<n>`), bereitet die Umgebung neu vor und wiederholt den Installer einmal je Sync, wobei die benötigten Pakete neu geladen werden. Scheitert er trotzdem, leere `.cache/b/d/m/bun-cache` selbst, wenn nichts läuft, oder führe den Sync mit derselben Anwendungsidentität aus, die den Cache gefüllt hat. Nicht Ende-zu-Ende verifiziert: Klassifikator, Generationsauswahl und Baumentfernung sind durch Fixture-Tests abgedeckt, ein vollständiger `dependencies sync` über den Weg Leeren-und-Wiederholen wurde nicht aufgezeichnet.

Das Neuanlegen von `d` kostet einen vollständigen `dependencies sync --all` (etwa 16 Minuten mit warmem Paket-Cache im einen aufgezeichneten Lauf auf dem primären macOS-arm64-Host, 2026-10-19), das von `g` eine kalte Graph-Inferenz (Minuten; für diese Dokumentation nicht gemessen).

### Developer Dashboard

Der [tägliche Dashboard-Ablauf](🔨️modules/🎛️dashboard/README.md#-daily-work) ist der Einstieg für Entwickler. Wähle **New Task**, suche die Operation und ihren Eigentümer und fülle dann das typisierte Formular aus. Die Vorschau zeigt Argumente und Umgebung vor **Start**. Ausgabe, Fortschritt, Bereitschaft, Quellnavigation, interaktives Debugging, Abbruch und Neustart bleiben an dieser Task.

Der Abschnitt Befehlsregister des Build-Kapitels führt die Build-Werkzeuge, die globalen Achsen (`build-tool`, `cache`, `build-mode`) und die Trockenlauf-Formen auf. Deklariere ausführbare Operationen neben ihrem Quelltext: Build-Targets in `📋️project.json`, typisierte Steuerelemente in `metadata.semio.dashboard` und selbst geschriebene Ticket-Werkzeuge in der `🎮️commands.json` des Tickets. Playground-Steuerelemente stammen aus `[[package.metadata.semio.playground]]` in Crate-Manifesten; native Crate-Targets stammen aus `📋️build-owners.json` und dem neutralen Build-Graphen. Parameterauswahl, Umgebungen, Bereitschaft und Compound-Mitgliedschaft gehören zu diesen Eigentümer-Deklarationen. Editoren bieten Quellnavigation und Bearbeitung; das Dashboard besitzt die Ausführung.

### Go-Layout

Go-Code folgt dem kanonischen Taxonomie-Layout statt Dateien in der Paketwurzel:

- Ein Member ist ein Verzeichnis mit genau einem `🐹️.go`-Blatt: `<module>/📦️packages/🐹️go/<member>/🐹️.go`. Der Member-Name ist in `📚️library/🔣️taxonomy.json` registriert (`members-of-members-of-members-of-modules`).
- White-Box-Tests sind Fälle am sprachneutralen Eigentümer: `<module>/🧪️tests/<case>/🐹️.go`, in das Paket aufgenommen, wenn die Paketklausel die des Moduls oder `<package>_test` ist. Black-Box-Adapter werden nie aufgenommen.
- Weil die Dateien nicht in der Paketwurzel liegen, sehen einfaches `go test` und `go list` die Member nicht. Führe Go über das Paket-`📜️script.ts` (`runCanonicalGoTests`) oder `bun repo run <project>:test` aus, die die zusammengeführte Quell-Überlagerung übergeben.

# 🔨️ Modules

- [`💻️client/⌨️cli`](🔨️modules/💻️client/⌨️cli) – Command line tool for monorepo interactions
- [`💻️client/🔌️mcp`](🔨️modules/💻️client/🔌️mcp) – MCP server the agent clients speak to
- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode) – Passive source navigation and language editing; the dashboard exclusively controls developer operations
- [`💻️client/🪶️sqlite`](🔨️modules/💻️client/🪶️sqlite/README.md) – SQLite schema of the client's local-only entity store
- [`🖥️server/🎛️coordinator`](🔨️modules/🖥️server/🎛️coordinator/README.md) – Coordinator REST API and event log
- [`🖥️server/🧬️schema`](🔨️modules/🖥️server/🧬️schema) – PostgreSQL schema of the shared server state
- [`📚️library`](🔨️modules/📚️library) – Taxonomy, discovery, normalization and the derived schema catalog
- [`🧪️test`](🔨️modules/🧪️test/README.md) – Cross-module test harness
