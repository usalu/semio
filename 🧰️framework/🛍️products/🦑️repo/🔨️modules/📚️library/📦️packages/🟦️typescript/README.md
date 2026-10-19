# @semio-tech/repo-lib (JS)

Repo policy lint facade and GraphQL CLI subprocess helpers.

## Bundle `script.ts` conventions

Every bundle or workspace router is a single `script.ts`. Implement commands as **`Script` subclasses** registered on a **`ScriptRouter`**, not long `if (command === …)` chains.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "./src/bundle-script.ts";

class TestScript extends BundleScript {
 run(segments: string[]) {
  runVitest(this.root, segments);
 }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url);
```

- **`BundleScript`**: `this.root` is the bundle directory; `this.repoRoot` is the monorepo root (`findRepoRoot`).
- **`runBundleScriptMain(router, import.meta.url)`**: runs `policy` when `export const policy` is present, then dispatches argv.
- **`runPolicyOnlyMain(import.meta.url)`**: policy-only bundles (no subcommands).
- **Workspace root** (`/script.ts`): same `Script` base class with `ScriptRouter` over workspace verbs (see root `script.ts`).
- **Native bootstrap**: the checked-in platform bootstrap under `🔨️modules/🔩️native/🥾️bootstrap`, rerun with the dashboard command `workspace:setup-native`.
- Helpers: `runCmd`, `runBun`, `runBunx`, `runViteDev`, `runViteBuild`, `runVitest`, `runWasmPackWebBuild`, `playPollingEnv`, `dispatchSubcommand`, `runWorkspaceScriptMain`, `devToolingEnv`, `spawnBunx`.
- Nested verbs inside a command class use `dispatchSubcommand(segments, { … }, usage, defaultKey?)`.

## Policy scripts (`script.ts` only)

- **File**: `export const policyFile = "index.ts"` plus `export const policy = defineLint(...)` in the bundle `script.ts`.
- **Folder / bundle / technology**: `export const policy = defineLint(...)` in `script.ts` at that directory — runner resolves entity kind from `folder(path)` GraphQL.

Choose **New Task** (`Ctrl+B n`) in `bun run dashboard` and select the owner's registered `lint` or `check` target. For a scoped authored policy route, select `tool:workspace/run-script-verb` and set its `commandLine` to `lint <repo-id> [--markdown]`. The dashboard owns the original command line and streams policy events; inferred `lint-policy` targets remain bound to each owner.

The repository's shared [policy inference](../../🏗️build/🕸️graph/📥️inference/🛡️policy/🟨️.mjs) discovers `📜️script.ts` exports and adds a cacheable `lint-policy` task to the owning project. Its `lint` task depends on `lint-policy`; projects without a declared owner receive a generated `breach-*` project. Both build adapters consume this shared graph.

Run declared tasks through the neutral repository interface:

```bash
bun repo run <project>:<task>[:<configuration>]
bun repo --tool=bazel run <project>:<task>[:<configuration>]
```

Nx remains the configured default while Bazel qualification is in progress. Backend registration and argument translation belong to the private adapters.
