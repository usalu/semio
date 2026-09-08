import { type DependencyJsLockMismatchKind, dependencyJsLockWorkspaceMismatches } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes synthetic fixtures for stale, missing, drifted, and absent workspace snapshots. */
export function dependencyJsLockParitySelfTests(): number {
  const manifest = new Map<string, unknown>([["unit/package.json", { dependencies: { alpha: "^1.0.0" }, devDependencies: { beta: "2.0.0" } }]]);
  const fixtures: readonly { expected: readonly DependencyJsLockMismatchKind[]; name: string; workspaces: unknown }[] = [
    { name: "matching snapshot", workspaces: { unit: { dependencies: { alpha: "^1.0.0" }, devDependencies: { beta: "2.0.0" } } }, expected: [] },
    { name: "stale workspace row", workspaces: { unit: { dependencies: { alpha: "^1.0.0", retired: "1.0.0" }, devDependencies: { beta: "2.0.0" } } }, expected: ["stale-in-lock"] },
    { name: "missing workspace row", workspaces: { unit: { dependencies: {}, devDependencies: { beta: "2.0.0" } } }, expected: ["missing-in-lock"] },
    { name: "workspace version drift", workspaces: { unit: { dependencies: { alpha: "^2.0.0" }, devDependencies: { beta: "2.0.0" } } }, expected: ["version-mismatch"] },
    { name: "missing workspace snapshot", workspaces: {}, expected: ["workspace-missing"] },
  ];
  for (const fixture of fixtures) {
    const actual = dependencyJsLockWorkspaceMismatches(manifest, fixture.workspaces).map((finding) => finding.kind);
    if (actual.join("\0") !== fixture.expected.join("\0")) throw new Error(`[verify dependencies parity js] lock self-test ${JSON.stringify(fixture.name)} failed: expected ${fixture.expected.join(",") || "clean"}, got ${actual.join(",") || "clean"}.`);
  }
  return fixtures.length;
}
