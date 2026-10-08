/** 🔎️ Third-party oracles for the 3d analysis queries: `three` triangulations and `manifold-3d` meshes are measured
 *  independently and compared with the language-agnostic fixtures the Rust tests assert against, so both kernels
 *  and both oracles agree on one set of closed-form numbers. */
import { readFileSync } from "node:fs";
import { Box3, BoxGeometry, BufferGeometry, ConeGeometry, CylinderGeometry, Float32BufferAttribute, Sphere, SphereGeometry, TorusGeometry, Vector3 } from "three";

type Vitest = NonNullable<ImportMeta["vitest"]>;
type TestSource = { readonly directory: string; readonly url: string };
type Vec = [number, number, number];
type Matrix = number[][];

const read = (source: TestSource, relative: string) => JSON.parse(readFileSync(new URL(relative, source.url), "utf8"));

/** 🧮️ Triangles of a `BufferGeometry` as vertex triples, indexed or not. */
function triangles(geometry: BufferGeometry): Vector3[][] {
  const position = geometry.getAttribute("position");
  const vertex = (i: number) => new Vector3().fromBufferAttribute(position, i);
  const index = geometry.getIndex();
  const count = index ? index.count : position.count;
  const out: Vector3[][] = [];
  for (let t = 0; t < count; t += 3) {
    const ids = [0, 1, 2].map((k) => (index ? index.getX(t + k) : t + k));
    out.push(ids.map(vertex));
  }
  return out;
}

/** 🧮️ Triangulates polygon faces from their first vertex. */
function fan(positions: Vec[], faces: number[][]): Vector3[][] {
  const point = (i: number) => new Vector3(...positions[i]!);
  return faces.flatMap((face) => face.slice(1, -1).map((_, k) => [point(face[0]!), point(face[k + 1]!), point(face[k + 2]!)]));
}

type Measured = { area: number; volume: number; centroid: Vec; inertia: Matrix; box: Box3 };

/** 🧮️ Area, signed volume, centroid and centroid inertia (unit density) of a triangle soup through signed tetrahedra. */
function measure(soup: Vector3[][]): Measured {
  let area = 0;
  let volume = 0;
  const moment = new Vector3();
  const covariance: Matrix = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
  const box = new Box3();
  for (const [a, b, c] of soup as [Vector3, Vector3, Vector3][]) {
    box.expandByPoint(a).expandByPoint(b).expandByPoint(c);
    area += new Vector3().subVectors(b, a).cross(new Vector3().subVectors(c, a)).length() / 2;
    const tetra = a.dot(new Vector3().crossVectors(b, c)) / 6;
    volume += tetra;
    moment.addScaledVector(new Vector3().add(a).add(b).add(c), tetra / 4);
    const sum = new Vector3().add(a).add(b).add(c).toArray();
    const corners = [a.toArray(), b.toArray(), c.toArray()];
    for (let i = 0; i < 3; i++) {
      for (let j = 0; j < 3; j++) {
        const own = corners.reduce((total, corner) => total + corner[i]! * corner[j]!, 0);
        covariance[i]![j]! += (tetra / 20) * (own + sum[i]! * sum[j]!);
      }
    }
  }
  const centroid = moment.clone().divideScalar(volume);
  const trace = covariance[0]![0]! + covariance[1]![1]! + covariance[2]![2]!;
  const c = centroid.toArray();
  const squared = centroid.lengthSq();
  const orientation = Math.sign(volume);
  const inertia = covariance.map((row, i) => row.map((value, j) => orientation * ((i === j ? trace : 0) - value - volume * ((i === j ? squared : 0) - c[i]! * c[j]!))));
  return { area, volume, centroid: c as Vec, inertia, box };
}

/** 🧮️ Eigenvalues of a symmetric 3x3 matrix by the closed trigonometric solution, ascending. */
function eigenvalues(m: Matrix): Vec {
  const off = m[0]![1]! ** 2 + m[0]![2]! ** 2 + m[1]![2]! ** 2;
  const q = (m[0]![0]! + m[1]![1]! + m[2]![2]!) / 3;
  if (off === 0) return [m[0]![0]!, m[1]![1]!, m[2]![2]!].sort((a, b) => a - b) as Vec;
  const p2 = (m[0]![0]! - q) ** 2 + (m[1]![1]! - q) ** 2 + (m[2]![2]! - q) ** 2 + 2 * off;
  const p = Math.sqrt(p2 / 6);
  const b = m.map((row, i) => row.map((value, j) => (value - (i === j ? q : 0)) / p));
  const det = b[0]![0]! * (b[1]![1]! * b[2]![2]! - b[1]![2]! * b[2]![1]!) - b[0]![1]! * (b[1]![0]! * b[2]![2]! - b[1]![2]! * b[2]![0]!) + b[0]![2]! * (b[1]![0]! * b[2]![1]! - b[1]![1]! * b[2]![0]!);
  const phi = Math.acos(Math.max(-1, Math.min(1, det / 2))) / 3;
  const e1 = q + 2 * p * Math.cos(phi);
  const e3 = q + 2 * p * Math.cos(phi + (2 * Math.PI) / 3);
  return [e3, 3 * q - e1 - e3, e1];
}

const sorted = (values: number[]) => [...values].sort((a, b) => a - b);
const extents = (box: Box3) => sorted(box.getSize(new Vector3()).toArray());

/** 🌀 Principal curvatures by central differences of the parametrisation, outward normal, convex positive. */
function numericCurvature(point: (u: number, v: number) => Vec3Like, u: number, v: number, outward: number): [number, number] {
  const h = 1e-4;
  const p = (a: number, b: number) => new Vector3(...point(a, b));
  const du = p(u + h, v).sub(p(u - h, v)).divideScalar(2 * h);
  const dv = p(u, v + h).sub(p(u, v - h)).divideScalar(2 * h);
  const duu = p(u + h, v).sub(p(u, v).multiplyScalar(2)).add(p(u - h, v)).divideScalar(h * h);
  const dvv = p(u, v + h).sub(p(u, v).multiplyScalar(2)).add(p(u, v - h)).divideScalar(h * h);
  const duv = p(u + h, v + h).sub(p(u + h, v - h)).sub(p(u - h, v + h)).add(p(u - h, v - h)).divideScalar(4 * h * h);
  const n = new Vector3().crossVectors(du, dv).normalize().multiplyScalar(outward);
  const [e, f, g] = [du.dot(du), du.dot(dv), dv.dot(dv)];
  const [l, m, nn] = [-duu.dot(n), -duv.dot(n), -dvv.dot(n)];
  const det = e * g - f * f;
  const gaussian = (l * nn - m * m) / det;
  const mean = (e * nn - 2 * f * m + g * l) / (2 * det);
  const spread = Math.sqrt(Math.max(mean * mean - gaussian, 0));
  return [mean + spread, mean - spread];
}
type Vec3Like = [number, number, number];

export async function registerAnalysisOracleTests(vitest: Vitest, source: TestSource): Promise<void> {
  const { describe, expect, it } = vitest;
  const close = (actual: number, expected: number, relative: number, label: string) =>
    expect(Math.abs(actual - expected), `${label}: ${actual} vs ${expected}`).toBeLessThanOrEqual(relative * Math.max(Math.abs(expected), 1));
  const { default: loadManifold } = await import("manifold-3d");
  const wasm = await loadManifold();
  wasm.setup();
  const { CrossSection, Manifold, Mesh } = wasm;

  describe("3d analysis oracles", () => {
    it("three and manifold-3d measure the mesh quality fixtures exactly like the closed forms", () => {
      const fixture = read(source, "./🥽️mesh/🧫️fixtures/🔎️quality/🔣️.json");
      for (const entry of fixture.cases) {
        const expected = entry.expected;
        const soup = fan(entry.positions, entry.faces);
        const used = [...new Set<number>(entry.faces.flat())].map((i) => entry.positions[i] as Vec);
        const measured = measure(soup);
        const bounds = new Box3().setFromPoints(used.map((p) => new Vector3(...p)));
        close(measured.area, expected.area, 1e-9, `${entry.name} three area`);
        close(measured.volume, expected.signedVolume, 1e-9, `${entry.name} three signed volume`);
        expect(bounds.min.toArray(), `${entry.name} three bbox min`).toEqual(expected.boundingBox[0]);
        expect(bounds.max.toArray(), `${entry.name} three bbox max`).toEqual(expected.boundingBox[1]);
        const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(entry.positions.flat(), 3));
        expect(geometry.getAttribute("position").count, `${entry.name} three vertices`).toBe(expected.vertexCount);
        if (expected.inertia) {
          expected.inertia.forEach((row: number[], i: number) => row.forEach((value, j) => close(measured.inertia[i]![j]!, value, 1e-9, `${entry.name} three inertia[${i}][${j}]`)));
          measured.centroid.forEach((value, axis) => close(value, expected.centroid[axis], 1e-9, `${entry.name} three centroid[${axis}]`));
        }
        const triVerts = new Uint32Array(entry.faces.flatMap((face: number[]) => face.slice(1, -1).flatMap((_: number, k: number) => [face[0], face[k + 1], face[k + 2]])));
        const build = () => new Manifold(new Mesh({ numProp: 3, vertProperties: new Float32Array(entry.positions.flat()), triVerts }));
        if (!expected.manifoldAccepts) {
          expect(build, `${entry.name} manifold-3d must refuse this mesh`).toThrow();
          continue;
        }
        const manifold = build();
        expect(manifold.status(), entry.name).toBe("NoError");
        close(manifold.volume(), expected.signedVolume, 1e-6, `${entry.name} manifold volume`);
        close(manifold.surfaceArea(), expected.area, 1e-6, `${entry.name} manifold area`);
        if (expected.manifoldGenus !== undefined) expect(manifold.genus(), `${entry.name} manifold genus`).toBe(expected.manifoldGenus);
        if (expected.genus !== null && entry.expected.connectedComponents === 1) expect(manifold.genus(), `${entry.name} genus`).toBe(expected.genus);
        const box = manifold.boundingBox();
        expect(box.min, `${entry.name} manifold bbox min`).toEqual(expected.boundingBox[0]);
        expect(box.max, `${entry.name} manifold bbox max`).toEqual(expected.boundingBox[1]);
      }
    });

    it("three triangulations and manifold-3d solids reproduce the closed-form primitive mass properties", () => {
      const fixture = read(source, "./📐️brep/🧫️fixtures/🔎️analysis/🔣️.json");
      const resolution = 256;
      for (const entry of fixture.primitives) {
        const expected = entry.expected;
        const p = entry.params;
        const geometry: BufferGeometry = {
          box: () => new BoxGeometry(p.width, p.depth, p.height),
          sphere: () => new SphereGeometry(p.radius, resolution, resolution / 2),
          cylinder: () => new CylinderGeometry(p.radius, p.radius, p.height, resolution),
          cone: () => new ConeGeometry(p.radius, p.height, resolution),
          torus: () => new TorusGeometry(p.major, p.minor, resolution / 2, resolution),
        }[entry.primitive as "box"]();
        const measured = measure(triangles(geometry));
        const tolerance = entry.primitive === "box" ? 1e-9 : 2e-3;
        close(measured.volume, expected.volume, tolerance, `${entry.name} three volume`);
        close(measured.area, expected.area, tolerance, `${entry.name} three area`);
        const moments = eigenvalues(measured.inertia);
        const wanted = sorted([expected.inertia[0][0], expected.inertia[1][1], expected.inertia[2][2]]);
        moments.forEach((value, k) => expect(Math.abs(value - wanted[k]!), `${entry.name} three principal moment ${k}: ${value} vs ${wanted[k]}`).toBeLessThanOrEqual(tolerance * wanted[2]!));
        const size = sorted(expected.boundingBox[1].map((hi: number, axis: number) => hi - expected.boundingBox[0][axis]));
        extents(measured.box).forEach((value, k) => expect(Math.abs(value - size[k]!)).toBeLessThanOrEqual(1e-9 + (entry.primitive === "box" ? 0 : tolerance * size[2]!)));

        const solid = {
          box: () => Manifold.cube([p.width, p.depth, p.height]),
          sphere: () => Manifold.sphere(p.radius, resolution),
          cylinder: () => Manifold.cylinder(p.height, p.radius, p.radius, resolution),
          cone: () => Manifold.cylinder(p.height, p.radius, 0, resolution),
          torus: () => Manifold.revolve(CrossSection.circle(p.minor, resolution / 2).translate([p.major, 0]), resolution),
        }[entry.primitive as "box"]();
        close(solid.volume(), expected.volume, tolerance, `${entry.name} manifold volume`);
        close(solid.surfaceArea(), expected.area, tolerance, `${entry.name} manifold area`);
        expect(solid.genus(), `${entry.name} manifold genus`).toBe(expected.counts.genus);
        const box = solid.boundingBox();
        sorted(box.max.map((hi: number, axis: number) => hi - box.min[axis]!)).forEach((value, k) => expect(Math.abs(value - size[k]!)).toBeLessThanOrEqual(1e-9 + (entry.primitive === "box" ? 0 : tolerance * size[2]!)));
      }
    });

    it("finite-difference curvature of the parametrisations agrees with the fixture's closed forms", () => {
      const fixture = read(source, "./📐️brep/🧫️fixtures/🔎️analysis/🔣️.json");
      const surfaces: Record<string, (s: Record<string, number>) => (u: number, v: number) => Vec3Like> = {
        sphere: (s) => (u, v) => [s.radius! * Math.cos(v) * Math.cos(u), s.radius! * Math.cos(v) * Math.sin(u), s.radius! * Math.sin(v)],
        cylinder: (s) => (u, v) => [s.radius! * Math.cos(u), s.radius! * Math.sin(u), v],
        cone: (s) => (u, v) => [v * Math.tan(s.halfAngle!) * Math.cos(u), v * Math.tan(s.halfAngle!) * Math.sin(u), v],
        torus: (s) => (u, v) => [(s.major! + s.minor! * Math.cos(v)) * Math.cos(u), (s.major! + s.minor! * Math.cos(v)) * Math.sin(u), s.minor! * Math.sin(v)],
      };
      let checked = 0;
      for (const entry of fixture.curvature) {
        const build = surfaces[entry.surface.kind];
        if (!build) continue;
        const parametrisation = build(entry.surface);
        for (const sample of entry.samples) {
          const wanted: number[] = sample.principal ?? entry.expected.principal;
          const [k1, k2] = numericCurvature(parametrisation, sample.u, sample.v, entry.flipped ? -1 : 1);
          close(k1, wanted[0]!, 1e-4, `${entry.name} k1`);
          close(k2, wanted[1]!, 1e-4, `${entry.name} k2`);
          checked += 1;
        }
      }
      expect(checked).toBeGreaterThanOrEqual(9);
    });

    it("three's Box3 and Sphere distances agree with the fixture's solid and point distances", () => {
      const fixture = read(source, "./📐️brep/🧫️fixtures/🔎️analysis/🔣️.json");
      const placed = (entry: { box: Vec; offset: Vec }) => new Box3(new Vector3(...entry.offset), new Vector3(...entry.offset).add(new Vector3(...entry.box)));
      for (const entry of fixture.distances) {
        const a = placed(entry.a);
        const b = placed(entry.b);
        const gap = new Vector3(Math.max(a.min.x - b.max.x, b.min.x - a.max.x, 0), Math.max(a.min.y - b.max.y, b.min.y - a.max.y, 0), Math.max(a.min.z - b.max.z, b.min.z - a.max.z, 0));
        close(a.intersectsBox(b) ? 0 : gap.length(), entry.distance, 1e-12, entry.name);
      }
      for (const entry of fixture.pointDistances) {
        const point = new Vector3(...(entry.point as Vec));
        const params = entry.shape.params;
        if (entry.shape.primitive === "box") {
          const box = new Box3(new Vector3(0, 0, 0), new Vector3(params.width, params.depth, params.height));
          if (box.containsPoint(point)) {
            const inside = Math.min(...[point.x, point.y, point.z].flatMap((value, axis) => [value - [0, 0, 0][axis]!, [params.width, params.depth, params.height][axis]! - value]));
            close(-inside, entry.signed, 1e-12, entry.name);
          } else close(box.distanceToPoint(point), entry.distance, 1e-12, entry.name);
        } else if (entry.shape.primitive === "sphere") {
          close(new Sphere(new Vector3(0, 0, 0), params.radius).distanceToPoint(point), entry.distance, 1e-12, entry.name);
        } else {
          const ring = Math.hypot(point.x, point.y);
          close(Math.hypot(ring - params.major, point.z) - params.minor, entry.distance, 1e-12, entry.name);
        }
      }
    });
  });
}
