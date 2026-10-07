import { lstatSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { canonicalGoPlan, runCanonicalGoTests, runRepositoryTestCommand } from "../../📦️packages/🟦️typescript/🟦️.ts";

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
  if (owner === ".." || owner.startsWith("..") || !plan.packages.includes(selected)) throw new Error(`Go input has no compiler package owner: ${path}`);
  return { moduleRoot: root, packages: [selected] };
}

/** 🐹️ Executes one canonical Go module selection for native callers and Nx routes. */
export class GoTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "binary") {
      if (segments.length !== 4 || !/^Test[A-Za-z0-9_]+$/.test(segments[3]!)) throw new Error("Expected go-test binary <module-root> <owned-binary> <exact-test-name>");
      const root = realpathSync(resolve(segments[1]!)), executable = realpathSync(resolve(segments[2]!));
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!artifacts) throw new Error("Go test binary requires caller-owned artifacts");
      const within = relative(realpathSync(artifacts), executable);
      if (!within || within === ".." || within.startsWith(`..${sep}`) || isAbsolute(within) || !lstatSync(executable).isFile()) throw new Error("Go test binary escapes caller-owned artifacts");
      if (!canonicalGoPlan(root).packages.includes(".")) throw new Error("Go test binary has no canonical root package");
      const owner = mkdtempSync(join(realpathSync(artifacts), "go-test-binary-"));
      try {
        await runRepositoryTestCommand(executable, ["-test.v", "-test.count=1", `-test.run=^${segments[3]}$`], { cwd: root, env: { ...process.env, SEMIO_GO_OVERLAY_OWNER: owner }, throwOnFailure: true });
      } finally {
        rmSync(owner, { recursive: true, force: true });
      }
      return;
    }
    if (segments.length < 2) throw new Error("Expected go-test <module-root> <input-path|-> [go-test-args...]");
    const selection = projectGoInputPackages(segments[0]!, segments[1]!);
    await runCanonicalGoTests(selection.moduleRoot, segments.slice(2), { env: process.env, packages: selection.packages });
  }
}
