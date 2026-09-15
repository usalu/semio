/** 🎯️ The screen-space hit test's own geometry, and the leftover overlay's active object.
 *
 * 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/🎯️pick-bounds.json`.
 *
 * 🧯️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46. A URL-backed mesh record carries no inline `data`,
 * and both screen-space hit tests — the pointerup pick fallback and the marquee — used to answer that
 * with a unit cube (the marquee with the single point `[0,0,0]`). Measured on the fully rendered
 * 180-object Nakagin tower: `candidates=180 containing=0 hit=none boxes=["28x30@427,394", …]` on 20 of
 * 20 clicks spread over the whole pane. With the loaded GLB's own extents recorded, the same click at
 * the same pane fraction reads `candidates=180 containing=14 hit=e80dc9d0-… boxes=["97x88@392,349", …]`.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🎯️pick-bounds.json" with { type: "json" };
import { hoveredInteractionTargetV1, mergeWorldSelectionWithLeftoverV1, resolveClickInstanceIdFromProjected, world3dInstanceLocalCorners } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

type Corner = readonly [number, number, number];

/** 📐️ The axis spans of eight local corners, rounded so a declaration reads in metres. */
const spanOf = (corners: readonly Corner[]): readonly [number, number, number] => {
  const axis = (index: 0 | 1 | 2) => {
    const values = corners.map((corner) => corner[index]);
    return Math.round((Math.max(...values) - Math.min(...values)) * 1e6) / 1e6;
  };
  return [axis(0), axis(1), axis(2)];
};

/** 🧫️ A declaration's number lists are widened to `number[]` on import — narrowed here once, so the
 * laws below read the same tuple shapes the host's own signatures take. */
const corners3 = (rows: readonly (readonly number[])[]): readonly Corner[] => rows.map((row) => [row[0]!, row[1]!, row[2]!] as const);
const corners2 = (rows: readonly (readonly number[])[]): readonly (readonly [number, number])[] => rows.map((row) => [row[0]!, row[1]!] as const);
const recorded = new Map<string, readonly Corner[]>(Object.entries(fixture.recordedBounds).map(([url, rows]) => [url, corners3(rows)]));
const pickCandidates = fixture.projectedPick.candidates.map((candidate) => ({ id: candidate.id, corners: corners2(candidate.corners), depth: candidate.depth }));

describe("🎯️ world-3d pick bounds", () => {
  for (const testCase of fixture.cases) {
    it(testCase.name, () => {
      const corners = world3dInstanceLocalCorners(testCase.meshData as never, testCase.meshUrl, recorded);
      expect(corners).toHaveLength(8);
      expect(spanOf(corners)).toEqual(testCase.expectedSpan);
      if (testCase.expect === "unitCube") expect(corners).toEqual(corners3(fixture.unitCube));
      if (testCase.expect === "recorded") expect(corners).toEqual(recorded.get(testCase.meshUrl!));
    });
  }

  it("a url-backed instance is unpickable at unit-cube size and pickable at GLB size", () => {
    const { click, expectedHit } = fixture.projectedPick;
    const unitOnly = pickCandidates.filter((candidate) => candidate.id !== "capsule-glb");
    expect(resolveClickInstanceIdFromProjected(click, unitOnly.filter((candidate) => candidate.id !== "capsule-unit"))).toBeNull();
    expect(resolveClickInstanceIdFromProjected(click, unitOnly)).toBe(expectedHit);
    expect(resolveClickInstanceIdFromProjected(click, pickCandidates.filter((candidate) => candidate.id !== "capsule-unit"))).toBe("capsule-glb");
  });

  for (const testCase of fixture.leftoverOverlay.cases) {
    it(testCase.name, () => {
      const merged = mergeWorldSelectionWithLeftoverV1(
        { ids: testCase.baseIds, activeObjectId: testCase.baseActiveObjectId ?? undefined },
        { ids: testCase.overlayIds, hoveredId: null, gumballActive: false, gumballAnchorId: null },
      );
      expect(merged.ids).toEqual(testCase.expectedIds);
      expect(merged.activeObjectId ?? null).toBe(testCase.expectedActiveObjectId);
    });
  }
});

/** 🎯️ HOVER AND SELECT MUST AGREE ON THE SAME POINT.
 *
 * The pane's two hit tests are different code: hover rides R3F's raycast against the real geometry,
 * and "did this click hit anything" is R3F's own `onPointerMissed` bookkeeping. On the generation3d
 * preview they disagreed — sweeping the pointer over the hexagonal column resolved `extrude@solid`
 * and clicking that very point arrived at the BACKGROUND handler, which dispatched
 * `interactionSelect targets:[]`. So the object could be hovered and inspected but never selected, and
 * a selection that was already standing was erased by the attempt. Measured 26/09/15 on :6023,
 * `🐍️world-pick-recon.mjs`; the row it turned red is `Hexagonal Mushroom Column · select`
 * (`📓️hot-swap-board-remount-2026-09-15.md` §3).
 *
 * The rule the background path now obeys: a pane that is PUBLISHING a hover target picks it instead of
 * clearing. A marker layer is excluded — a vortex and a reference own dedicated pick handlers and must
 * not be selected by a background click — and genuinely empty canvas still clears. */
describe("🎯️ a background click never clears at a point the pane is hovering", () => {
  it("picks the object the pane published as its hover target", () => {
    expect(hoveredInteractionTargetV1("extrude@solid")).toEqual({ granularity: "object", id: "extrude@solid" });
    expect(hoveredInteractionTargetV1("profile@wire")).toEqual({ granularity: "object", id: "profile@wire" });
  });

  it("still clears on genuinely empty canvas", () => {
    expect(hoveredInteractionTargetV1(null)).toBeNull();
    expect(hoveredInteractionTargetV1(undefined)).toBeNull();
    expect(hoveredInteractionTargetV1("")).toBeNull();
  });

  it("leaves every MARKER layer to its own pick handler", () => {
    expect(hoveredInteractionTargetV1("reference:datum-a")).toBeNull();
    expect(hoveredInteractionTargetV1("vortex:node-1:out")).toBeNull();
    expect(hoveredInteractionTargetV1("targetVolume:zone-3")).toBeNull();
  });
});
