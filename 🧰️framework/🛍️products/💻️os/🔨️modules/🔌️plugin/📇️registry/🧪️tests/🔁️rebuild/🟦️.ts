/** 🔁️ The declared all-plugin rebuild chain is well formed and keeps its stages in dependency order. */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { readRebuildChain, REBUILD_STAGES } from "../../🔁️rebuild/🟦️.ts";

describe("rebuild-all chain", () => {
  it("declares descriptors, registry, guests and catalog in dependency order", () => {
    const chain = readRebuildChain();
    expect(chain.map((step) => step.id)).toEqual(["describe", "generate", "check", "activate-s", "publish-catalog"]);
    expect([...new Set(chain.map((step) => step.stage))]).toEqual([...REBUILD_STAGES]);
    expect(chain.find((step) => step.id === "publish-catalog")?.command.slice(-2)).toEqual(["--packages", "all"]);
  });

  it("refuses a chain whose stages run backwards or repeat a step", () => {
    const root = mkdtempSync(join(tmpdir(), "rebuild-chain-"));
    const write = (steps: unknown[]): string => {
      const path = join(root, `${steps.length}-${Math.random()}.json`);
      writeFileSync(path, JSON.stringify({ schema: "semio.plugin-registry.rebuild-chain/v1", steps }));
      return path;
    };
    const describeStep = { id: "describe", stage: "descriptors", command: ["nx", "run-many", "-t", "describe"] };
    const catalogStep = { id: "publish-catalog", stage: "catalog", command: ["nx", "run", "os-hub:trusted-catalog-bootstrap"] };
    expect(() => readRebuildChain(write([catalogStep, describeStep]))).toThrow(/stage order/);
    expect(() => readRebuildChain(write([describeStep, describeStep]))).toThrow(/duplicated/);
    expect(() => readRebuildChain(write([{ ...describeStep, command: ["cargo", "build"] }]))).toThrow(/nx command/);
  });
});
