/** 🟢️ The shared declarations of the package language semantic handoff suite tests. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readScriptPolicy, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🎟️ Supplies each router dispatch the original 15-second control and a silent progress port. */
export function routerInvocation(owner: string): ScriptInvocation {
  const controller = new AbortController(), started = performance.now();
  return {
    policy: readScriptPolicy({ version: 1, owner, maximumElapsedMilliseconds: 15000 }),
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => Math.max(1, 15000 - (performance.now() - started)),
      publish: () => undefined,
      yieldContinuation: () => Promise.resolve(),
    },
    capabilities: {},
  };
}

/** 🦀️ The workspace root manifest, read only for the version every member inherits. */
export interface CargoWorkspaceManifest {
  readonly workspace: { readonly package: { readonly version: string }; readonly members: string[]; readonly dependencies: Record<string, unknown> };
}

export const inputRoot = join(contractRoot, "../🧫️fixtures/🤝️package-language-kind-handoff");

export const schema = JSON.parse(readFileSync(join(contractRoot, "../../🔣️taxonomy.json"), "utf8"));
