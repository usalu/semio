import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { actorTypegenPlan, actorTypegenTarget } from "../../🧬️typegen/📋️plan/🟦️.ts";
import { publishActorTypegen } from "../../🧬️typegen/📤️publication/🟦️.ts";
import { waitForActorTypegenProcess } from "../../🧬️typegen/🏃️execution/🟦️.ts";

const repoRoot = join(import.meta.dir, "../../../../..");

describe("actor typegen ownership", () => {
  test("the plan describes the generated mirror and exact stale siblings", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-actor-typegen-plan-"));
    try {
      const target = actorTypegenTarget(root);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(join(dirname(target), "stale.ts"), "stale");
      const plan = actorTypegenPlan(root, Buffer.from("actor"));
      expect(plan.contractId).toBe("actor-typegen");
      expect(plan.nodes.map(({ nodeKind }) => nodeKind)).toEqual(["directory", "file"]);
      expect(plan.staleRemovals).toEqual(["🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/stale.ts"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("publication writes the exact mirror then prunes only its output siblings", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-actor-typegen-publish-"));
    try {
      const target = actorTypegenTarget(root);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(join(dirname(target), "stale.ts"), "stale");
      mkdirSync(join(dirname(target), "stale-dir"));
      publishActorTypegen(target, Buffer.from("owned\n"));
      expect(readFileSync(target, "utf8")).toBe("owned\n");
      expect(existsSync(join(dirname(target), "stale.ts"))).toBe(false);
      expect(existsSync(join(dirname(target), "stale-dir"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("publication refuses a linked output root before any write or prune", () => {
    if (process.platform === "win32") return;
    const root = mkdtempSync(join(tmpdir(), "semio-actor-typegen-link-"));
    try {
      const target = actorTypegenTarget(root);
      const external = join(root, "external");
      mkdirSync(external, { recursive: true });
      writeFileSync(join(external, "keep"), "kept");
      mkdirSync(dirname(dirname(target)), { recursive: true });
      symlinkSync(external, dirname(target), "dir");
      expect(() => publishActorTypegen(target, Buffer.from("owned"))).toThrow("symbolic link");
      expect(readFileSync(join(external, "keep"), "utf8")).toBe("kept");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("native export waiting enforces cancellation and deadline by killing its child", async () => {
    const pending = () => {
      let killed = false;
      return {
        child: { exited: new Promise<number>(() => {}), kill: () => { killed = true; } },
        killed: () => killed,
      };
    };
    const cancelled = pending();
    await expect(waitForActorTypegenProcess(cancelled.child, { deadlineMs: 1_000, isCancelled: () => true, pollMs: 1 })).rejects.toThrow("cancelled");
    expect(cancelled.killed()).toBe(true);
    const timedOut = pending();
    let now = 0;
    await expect(waitForActorTypegenProcess(timedOut.child, { deadlineMs: 2, now: () => now++, pollMs: 1 })).rejects.toThrow("deadline");
    expect(timedOut.killed()).toBe(true);
  });

  test("Nx inputs, native exporter and TypeScript consumers close over the semantic owners", () => {
    const project = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json"), "utf8")) as { namedInputs: { default: string[] } };
    for (const suffix of ["🧬️typegen/📋️plan/🟦️.ts", "🧬️typegen/📤️publication/🟦️.ts", "🧬️typegen/🏃️execution/🟦️.ts", "🧪️tests/🧬️typegen/🟦️.ts"]) expect(project.namedInputs.default.some((path) => path.endsWith(suffix))).toBe(true);
    const nativeExporter = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    expect(nativeExporter).toContain("SEMIO_TYPEGEN_OUT");
    expect(nativeExporter).toContain("include_str!(\"../../🤖️generated/🎭️actor/🟦️.ts\")");
    for (const consumer of ["📬️mailbox/🟦️.ts", "📮️shard-client/🟦️.ts"]) expect(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor", consumer), "utf8")).toContain("../🤖️generated/🎭️actor/🟦️.ts");
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as { generatorContracts: Record<string, { ownerPath: string; inputPatterns: string[]; previewLimits?: { timeoutMs: number } }> };
    const contract = taxonomy.generatorContracts["actor-typegen"]!;
    expect(contract.ownerPath).toBe("🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust");
    expect(contract.previewLimits?.timeoutMs).toBe(600_000);
  });
});
