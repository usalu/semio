import { describe, expect, it } from "vitest";
import { fixedSlotTableFaults, inlineSlotTableBytes, type BoxedFixedSlotsBudget } from "../../🟦️.ts";

const { readFileSync } = await import("node:fs");
const { fileURLToPath } = await import("node:url");
const { dirname, join } = await import("node:path");
const budget = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"), "utf8")) as BoxedFixedSlotsBudget;

describe("🧱️ boxed fixed slot tables", () => {
  it("re-checks every committed table's N × size_of arithmetic", () => {
    expect(fixedSlotTableFaults(budget)).toEqual([]);
  });

  it("keeps every listed table far enough over the threshold to be worth the helper", () => {
    const under = budget.tables.filter((table) => inlineSlotTableBytes(table) <= budget.conversionThresholdBytes);
    expect(under.map((table) => table.owner)).toEqual([]);
  });

  it("reports the three tables that alone exceeded the worker-pool thread stack", () => {
    const overWorkerStack = budget.tables.filter((table) => inlineSlotTableBytes(table) > budget.workerPoolStackBytes).map((table) => table.owner);
    expect([...overWorkerStack].sort()).toEqual(["engine_canvas::EngineSurfaceRegistry", "scenes::AdmittedSurfaceMap<World3dState>", "wgpu::engine::UiSurfaceRegistry"]);
  });

  it("names a capacity constant and a slot type for every table", () => {
    expect(budget.tables.filter((table) => table.capacityConstant.length === 0 || table.elementType.length === 0)).toEqual([]);
  });
});
