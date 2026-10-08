/** 🧮️ Third-party `manifold-3d` oracle for the boolean differential matrix: the language-agnostic fixture
 *  `../../🧫️fixtures/differential-matrix/🔣️.json` names operands and an operation; manifold-3d rebuilds the same
 *  solids on dense tessellations and measures the result, which the Rust kernel's exact result is asserted against. */
import { readFileSync } from "node:fs";

type Vitest = NonNullable<ImportMeta["vitest"]>;
type TestSource = { readonly directory: string; readonly url: string };
type Vec = readonly [number, number, number];
export type ShapeSpec =
  | { kind: "box"; size: Vec; rotate?: Vec; translate?: Vec }
  | { kind: "sphere"; radius: number; rotate?: Vec; translate?: Vec }
  | { kind: "cylinder"; radius: number; height: number; rotate?: Vec; translate?: Vec }
  | { kind: "cone"; radius: number; height: number; rotate?: Vec; translate?: Vec }
  | { kind: "torus"; major: number; minor: number; rotate?: Vec; translate?: Vec }
  | { kind: "prism"; polygon: [number, number][]; height: number; rotate?: Vec; translate?: Vec }
  | { kind: "revolved"; profile: [number, number][]; rotate?: Vec; translate?: Vec };
export type Operation = "fuse" | "cut" | "intersect";
export type Expected = { empty: boolean; volume: number; area: number; euler: number; shells: number };
export type MatrixCase = { id: string; operation: Operation; a: ShapeSpec; b: ShapeSpec; tags: string[]; compareTopology: boolean; expected: Expected };
export type Wasm = Awaited<ReturnType<typeof import("manifold-3d")["default"]>>;

/** 🧮️ Builds one operand with the same canonical pose the Rust kernel's primitives use, then rotates (X, Y, Z in degrees) and translates it. */
export function buildShape(wasm: Wasm, spec: ShapeSpec, segments: number) {
  const { CrossSection, Manifold } = wasm;
  let shape;
  switch (spec.kind) {
    case "box": shape = Manifold.cube([...spec.size], false); break;
    case "sphere": shape = Manifold.sphere(spec.radius, segments); break;
    case "cylinder": shape = Manifold.cylinder(spec.height, spec.radius, spec.radius, segments, false); break;
    case "cone": shape = Manifold.cylinder(spec.height, spec.radius, 0, segments, false); break;
    case "torus": shape = Manifold.revolve(CrossSection.circle(spec.minor, segments).translate([spec.major, 0]), segments); break;
    case "prism": shape = Manifold.extrude(CrossSection.ofPolygons([spec.polygon]), spec.height); break;
    case "revolved": shape = Manifold.revolve(CrossSection.ofPolygons([spec.profile]), segments); break;
  }
  if (spec.rotate) shape = shape.rotate([...spec.rotate]);
  return spec.translate ? shape.translate([...spec.translate]) : shape;
}

/** 🧮️ Volume, area, Euler characteristic of the closed surface(s) and shell count of `a op b` measured by manifold-3d. */
export function measureCase(wasm: Wasm, entry: Pick<MatrixCase, "operation" | "a" | "b">, segments: number): Expected {
  const a = buildShape(wasm, entry.a, segments);
  const b = buildShape(wasm, entry.b, segments);
  const result = entry.operation === "fuse" ? a.add(b) : entry.operation === "cut" ? a.subtract(b) : a.intersect(b);
  try {
    if (result.isEmpty() || result.volume() < 1e-6) return { empty: true, volume: 0, area: 0, euler: 0, shells: 0 };
    const mesh = result.getMesh();
    const parts = result.decompose();
    const shells = parts.length;
    parts.forEach((part) => part.delete());
    return { empty: false, volume: result.volume(), area: result.surfaceArea(), euler: mesh.numVert - mesh.numTri / 2, shells };
  } finally {
    result.delete();
    b.delete();
    a.delete();
  }
}

const read = (source: TestSource, relative: string) => JSON.parse(readFileSync(new URL(relative, source.url), "utf8"));

/** 🧪️ manifold-3d reproduces every expected measurement of the fixture on an independent, coarser tessellation. */
export async function registerBooleanDifferentialTests(vitest: Vitest, source: TestSource): Promise<void> {
  const { describe, expect, it } = vitest;
  const { default: loadManifold } = await import("manifold-3d");
  const wasm = await loadManifold();
  wasm.setup();

  describe("boolean differential matrix", () => {
    it("manifold-3d reproduces the fixture's volume, area, topology and emptiness for every case", () => {
      const fixture = read(source, "./📐️brep/🛠️operations/🔀️boolean/🧫️fixtures/differential-matrix/🔣️.json");
      expect(fixture.cases.length).toBeGreaterThan(0);
      const failures: string[] = [];
      for (const entry of fixture.cases as MatrixCase[]) {
        const measured = measureCase(wasm, entry, fixture.checkSegments);
        const want = entry.expected;
        const bad = (what: string, got: number, expected: number) => failures.push(`${entry.id}: ${what} ${got} vs ${expected}`);
        if (measured.empty !== want.empty) bad("empty", Number(measured.empty), Number(want.empty));
        if (Math.abs(measured.volume - want.volume) > fixture.tolerance.volume * Math.max(1, want.volume)) bad("volume", measured.volume, want.volume);
        if (Math.abs(measured.area - want.area) > fixture.tolerance.area * Math.max(1, want.area)) bad("area", measured.area, want.area);
        if (entry.compareTopology && measured.euler !== want.euler) bad("euler", measured.euler, want.euler);
        if (entry.compareTopology && measured.shells !== want.shells) bad("shells", measured.shells, want.shells);
      }
      expect(failures).toEqual([]);
    }, 600000);
  });
}
