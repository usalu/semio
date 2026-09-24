/** 🎨️ Actual Three EdgesGeometry oracle for render-only GLB outlines. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { BufferGeometry, EdgesGeometry, Float32BufferAttribute } from "three";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎨️world3d-glb-outline/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🎨️world3d-glb-outline/🔣️.json"), "utf8"));

type Point = readonly [number, number, number];
type Segment = readonly [Point, Point];

const comparePoint = (left: Point, right: Point) => left[0] - right[0] || left[1] - right[1] || left[2] - right[2];
const canonicalSegment = (left: Point, right: Point): Segment => comparePoint(left, right) <= 0 ? [left, right] : [right, left];
const compareSegment = (left: Segment, right: Segment) => comparePoint(left[0], right[0]) || comparePoint(left[1], right[1]);

function actualThreeSegments(record: { readonly positions: readonly Point[]; readonly indices: readonly number[] }): Segment[] {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(record.positions.flat(), 3));
  geometry.setIndex([...record.indices]);
  const edges = new EdgesGeometry(geometry, fixture.thresholdAngleDegrees);
  const positions = edges.getAttribute("position");
  const segments: Segment[] = [];
  for (let index = 0; index < positions.count; index += 2) {
    const left: Point = [positions.getX(index), positions.getY(index), positions.getZ(index)];
    const right: Point = [positions.getX(index + 1), positions.getY(index + 1), positions.getZ(index + 1)];
    segments.push(canonicalSegment(left, right));
  }
  geometry.dispose();
  edges.dispose();
  return segments.sort(compareSegment);
}

describe("🎨️ render-only GLB outlines", () => {
  it("validates the neutral bounded derivation contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.derivation).toEqual({
      workUnit: "oneTrianglePerStep",
      publication: "meshAndOutlineTogether",
      cancellation: "retirePartialWithoutPublication",
    });
    expect(fixture.semanticEdgeIds).toEqual([]);
  });

  it("matches actual Three boundary, crease-angle and duplicate-position welding", () => {
    for (const record of fixture.cases) {
      expect(record.indices.length % 3, record.id).toBe(0);
      expect(actualThreeSegments(record), record.id).toEqual(record.expectedSegments);
    }
  });

  it("keeps the React outline scale separate from semantic mesh edges", () => {
    expect(fixture.outlineScale).toBe(1.001);
    for (const record of fixture.cases) {
      const scaled = actualThreeSegments(record).map((segment) => segment.map((point) => point.map((value) => value * fixture.outlineScale)));
      const expected = record.expectedSegments.map((segment: Segment) => segment.map((point) => point.map((value) => value * fixture.outlineScale)));
      expect(scaled, record.id).toEqual(expected);
    }
    expect(fixture.semanticEdgeIds).toHaveLength(0);
  });
});

