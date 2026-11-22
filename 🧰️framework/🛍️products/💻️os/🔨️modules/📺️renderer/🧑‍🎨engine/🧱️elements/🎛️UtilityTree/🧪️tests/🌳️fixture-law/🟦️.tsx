/** 🌳️ `UtilityTree`'s pure model replayed from the language-agnostic fixture the wgpu twin replays too: category grouping, ribbon segments, active-path reconciliation, pressed-leaf lookups and deduplication. */
import { describe, expect, it } from "vitest";
import type { UtilityNode } from "@semio-tech/framework";
import { buildUtilityRibbonSegments, dedupeUtilityNodesById, findPressedUtilityLeafId, findUtilityGroupPath, groupUtilityNodesByCategory, reconcileUtilityPath, UTILITY_CATEGORIES } from "../../🟦️.tsx";
import fixture from "../🧫️fixtures/🌳️utility-tree/🔣️.json";

const asNodes = (value: unknown): UtilityNode[] => value as UtilityNode[];

function summary(node: UtilityNode): Record<string, unknown> {
  const wire = node as unknown as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const key of ["id", "kind", "iconId", "text", "order", "category"]) if (wire[key] !== undefined) out[key] = wire[key];
  if (node.kind === "collection") out.childIds = node.children.map((child) => child.id);
  return out;
}

describe("utility tree fixture law", () => {
  it("buckets nodes into ordered category collections", () => {
    for (const row of fixture.grouping) {
      const grouped = row.categories === null ? groupUtilityNodesByCategory(asNodes(row.nodes)) : groupUtilityNodesByCategory(asNodes(row.nodes), row.categories as never);
      expect(grouped.map(summary), row.name).toEqual(row.expected);
    }
    expect([...UTILITY_CATEGORIES]).toEqual(["selection", "utilities"]);
  });

  it("deduplicates by first id", () => {
    expect(dedupeUtilityNodesById(fixture.dedupe.lists.map(asNodes)).map((node) => node.id)).toEqual(fixture.dedupe.expected);
  });

  it("builds ribbon segments, reconciles paths and finds the pressed leaf", () => {
    const tree = asNodes(fixture.tree);
    for (const row of fixture.segments) {
      const projected = buildUtilityRibbonSegments(tree, row.path).map((segment) => ({ kind: segment.kind, depth: segment.depth, ids: (segment.kind === "picker" ? segment.collections : segment.items).map((node) => node.id) }));
      expect(projected).toEqual(row.expected);
    }
    for (const row of fixture.reconcile) expect(reconcileUtilityPath(tree, row.path)).toEqual(row.expected);
    expect(findPressedUtilityLeafId(tree)).toBe(fixture.pressed.leafId);
    for (const [target, expected] of Object.entries(fixture.pressed.groupPaths)) expect(findUtilityGroupPath(tree, target)).toEqual(expected);
  });
});
