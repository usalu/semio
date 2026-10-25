import { selectPackageManagerV1 } from "../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

/** 🧪️ Executes synthetic fixtures for matching, stale, missing, drifted, unlisted and unbacked workspace rows through the package manager's lock check; the pinned-Bun oracle and the independent model live next to the Bun adapter. */
export function dependencyJsLockParitySelfTests(): number {
  const manager = selectPackageManagerV1("javascript");
  const lock = (workspaces: unknown, packages: unknown = { alpha: ["alpha@1.0.0"], beta: ["beta@2.0.0"], retired: ["retired@1.0.0"] }): string => JSON.stringify({ lockfileVersion: 1, workspaces, packages });
  const manifests = new Map<string, unknown>([["package.json", { name: "root" }], ["unit/package.json", { name: "unit", dependencies: { alpha: "^1.0.0" }, devDependencies: { beta: "2.0.0" } }]]);
  const row = (dependencies: Record<string, string>, devDependencies: Record<string, string>) => ({ name: "unit", dependencies, devDependencies });
  const fixtures: readonly { expected: readonly string[]; lock: string; name: string }[] = [
    { name: "matching snapshot", lock: lock({ "": { name: "root" }, unit: row({ alpha: "^1.0.0" }, { beta: "2.0.0" }) }), expected: [] },
    { name: "stale workspace row of a package that lost its last user", lock: lock({ "": { name: "root" }, unit: row({ alpha: "^1.0.0", retired: "1.0.0" }, { beta: "2.0.0" }) }), expected: ["stale-in-lock:true"] },
    { name: "stale workspace row of a package another table still declares", lock: lock({ "": { name: "root" }, unit: row({ alpha: "^1.0.0", beta: "2.0.0" }, { beta: "2.0.0" }) }), expected: ["stale-in-lock:false"] },
    { name: "missing workspace row", lock: lock({ "": { name: "root" }, unit: row({}, { beta: "2.0.0" }) }), expected: ["missing-in-lock:false"] },
    { name: "missing workspace row of a package no lock carries", lock: lock({ "": { name: "root" }, unit: row({}, { beta: "2.0.0" }) }, {}), expected: ["missing-in-lock:true"] },
    { name: "workspace version drift", lock: lock({ "": { name: "root" }, unit: row({ alpha: "^2.0.0" }, { beta: "2.0.0" }) }), expected: ["version-mismatch:false"] },
    { name: "missing workspace snapshot", lock: lock({ "": { name: "root" } }), expected: ["workspace-missing:true"] },
    { name: "unbacked workspace snapshot", lock: lock({ "": { name: "root" }, unit: row({ alpha: "^1.0.0" }, { beta: "2.0.0" }), gone: { name: "gone" } }), expected: ["workspace-stale:true"] },
  ];
  for (const fixture of fixtures) {
    const actual = manager.checkLock(fixture.lock, manifests).map((finding) => `${finding.kind}:${finding.frozen}`);
    if (actual.join("\0") !== fixture.expected.join("\0")) throw new Error(`[verify dependencies parity js] lock self-test ${JSON.stringify(fixture.name)} failed: expected ${fixture.expected.join(",") || "clean"}, got ${actual.join(",") || "clean"}.`);
  }
  return fixtures.length;
}
