import { lstatSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { BundleScript } from "../../🏃️process/🧭️routing/🟦️.ts";
import { canonicalGoPlan, runCanonicalGoTests } from "../../📦️packages/🟦️typescript/🟦️.ts";

export type GoInputSelectionOperations = Readonly<{
  realpath(path: string): string;
  isDirectory(path: string): boolean;
  plan(moduleRoot: string): Readonly<{ packages: readonly string[]; replacements: Readonly<Record<string, string>> }>;
}>;

/** 🎯️ Projects one admitted source path to its canonical Go compiler package. */
export function projectGoInputPackages(
  moduleRoot: string,
  input: string,
  operations: GoInputSelectionOperations = {
    realpath: realpathSync,
    isDirectory: (path) => lstatSync(path).isDirectory(),
    plan: canonicalGoPlan,
  },
): { moduleRoot: string; packages?: string[] } {
  const root = operations.realpath(resolve(moduleRoot));
  if (input === "-") return { moduleRoot: root };
  const path = operations.realpath(resolve(input));
  const plan = operations.plan(root);
  const projected = Object.entries(plan.replacements).find(([, source]) => operations.realpath(source) === path)?.[0];
  const projectedOwner = projected ? dirname(isAbsolute(projected) ? projected : resolve(root, projected)) : undefined;
  const owner = relative(root, projectedOwner ?? (operations.isDirectory(path) ? path : dirname(path)))
    .split(sep)
    .join("/");
  const selected = owner ? `./${owner}` : ".";
  if (owner === ".." || owner.startsWith("../") || !plan.packages.includes(selected)) throw new Error(`Go input has no compiler package owner: ${path}`);
  return { moduleRoot: root, packages: [selected] };
}

/** 🐹️ Executes one canonical Go module selection for native callers and Nx routes. */
export class GoTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length < 2) throw new Error("Expected go-test <module-root> <input-path|-> [go-test-args...]");
    const selection = projectGoInputPackages(segments[0]!, segments[1]!);
    await runCanonicalGoTests(selection.moduleRoot, segments.slice(2), { env: process.env, packages: selection.packages });
  }
}
