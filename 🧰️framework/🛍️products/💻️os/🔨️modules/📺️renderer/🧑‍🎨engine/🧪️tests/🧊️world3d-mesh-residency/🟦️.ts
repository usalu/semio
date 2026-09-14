/** 🧊️ The mesh half of the world-3d residency lanes: `advanceWorldMeshResidency` must keep the object
 * IDENTITY of every mesh whose wire text did not move, so the instanced layer rebuilds the
 * `BufferGeometry` of exactly the meshes that changed rather than all of them — the whole-array memo
 * boundary `📓️react-perf-ceilings-audit-2026-09-14.md` §2c named — and every visual it retires must be
 * disposed exactly once.
 *
 * 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json`, the neutral
 * declaration a second implementation of this lane is pinned against.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json" with { type: "json" };
import { advanceWorldMeshResidency, buildMeshVisuals, disposeMeshVisuals, splitJsonArrayElements, type WorldMeshResidencyV1 } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

/** 🧾️ The retained set a case starts from, built the way a cold consumer builds it. */
const residencyOf = (meshesJson: string | null): WorldMeshResidencyV1 | null => (meshesJson === null ? null : advanceWorldMeshResidency(null, meshesJson));

describe("🧊️ world-3d mesh residency lane", () => {
  it("declares the lane both implementations are pinned against", () => {
    expect(fixture.field).toBe("meshesJson");
    expect(fixture.contract.fallbackIsAlwaysCorrect).toBe(true);
    expect(fixture.contract.disposeOnRemove).toBe(true);
  });

  for (const scenario of fixture.splitCases) {
    it(`splits ${scenario.json}`, () => {
      expect(splitJsonArrayElements(scenario.json)).toEqual(scenario.elements);
    });
  }

  for (const scenario of fixture.cases) {
    it(scenario.name, () => {
      const previous = residencyOf(scenario.previousMeshesJson);
      const advanced = advanceWorldMeshResidency(previous, scenario.meshesJson);
      expect(advanced.records.map((record) => record.id)).toEqual(scenario.expect.ids);
      // 🪪️ Object identity is the whole point: a mesh the payload did not move must be the SAME
      // object the consumer already held, which is what keeps its built buffers alive.
      const reused = advanced.records.filter((record) => previous?.records.includes(record)).map((record) => record.id);
      expect(reused).toEqual(scenario.expect.reusedIds);
      expect(advanced.records.length - reused.length).toBe(scenario.expect.built);
    });
  }

  it("builds one visual set per mesh and disposes every buffer exactly once", () => {
    const residency = advanceWorldMeshResidency(null, fixture.cases[0]!.meshesJson);
    const visuals = residency.records.map(buildMeshVisuals);
    expect(visuals.map((entry) => Boolean(entry.geometry))).toEqual([true, true]);
    const disposed: string[] = [];
    for (const entry of visuals) {
      for (const [label, target] of [
        ["geometry", entry.geometry],
        ["border", entry.border],
        ["edge", entry.edge],
        ["vertexPick", entry.vertexPick?.geometry ?? null],
      ] as const) {
        if (!target) continue;
        const original = target.dispose.bind(target);
        target.dispose = () => {
          disposed.push(`${entry.record.id}:${label}`);
          original();
        };
      }
      disposeMeshVisuals(entry);
    }
    expect(disposed).toEqual(["a:geometry", "a:border", "b:geometry", "b:border"]);
    expect(new Set(disposed).size).toBe(disposed.length);
  });

  it("keeps the retained visuals of an unchanged mesh across an advance that moved a sibling", () => {
    const first = advanceWorldMeshResidency(null, fixture.cases[2]!.previousMeshesJson!);
    const held = new Map(first.records.map((record) => [record.id, buildMeshVisuals(record)] as const));
    const second = advanceWorldMeshResidency(first, fixture.cases[2]!.meshesJson);
    const rebuilt = second.records.filter((record) => held.get(record.id)?.record !== record).map((record) => record.id);
    expect(rebuilt).toEqual(["b"]);
    for (const entry of held.values()) disposeMeshVisuals(entry);
  });
});
