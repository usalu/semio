/** 🧊️ The frame conversion between the staffage world and the neutral model: points, pseudo-vectors, bone frames, parent-relative placement, the mirrored solid transforms and repeats, and the sample figure's solids tessellated through the pipeline against an independent computation.
 *
 * THE REFERENCE is three.js (`Matrix4.makeBasis`, `Quaternion.setFromRotationMatrix`, `Matrix4.compose`, `Box3`), used for the expected values only; the hand-computed numbers live in `🧫️fixtures/🧊️model-frame/🔣️.json`.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { Box3, Matrix4, Quaternion, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, restWorldMatrices, tessellateSolid, transformPoint } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { solidOf } from "../../🔨️modules/🧊️volume/🧊️solids/🟦️.ts";
import { basisOf, mirrorRepeat, mirrorTransform, pseudoToNeutral, quaternionOf, rotate, toNeutral, inverseBasis, multiply } from "../../🔨️modules/🧊️volume/🧭️frame/🟦️.ts";
import { placementsOf, skeletonOf } from "../../🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { restFrames } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { sampleFigure, volumeAssets } from "../🧰️support/🧊️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

type Triple = [number, number, number];
type Frame = { o: Triple; u: Triple; v: Triple; w: Triple };
type Vectors = {
  points: { name: string; staffage: Triple; neutral: Triple }[];
  pseudo: { name: string; staffage: Triple; neutral: Triple }[];
  frames: (Frame & { name: string; translation: Triple; basis: [Triple, Triple, Triple] })[];
  relative: { name: string; parent: Frame; child: Frame; translation: Triple; basis: [Triple, Triple, Triple] }[];
  transforms: { name: string; transform: { translation?: Triple; rotation?: [number, number, number, number]; scale?: Triple } | null; repeat: { count: number; translation?: Triple; rotationAxis?: Triple; rotationRad?: number; origin?: Triple } | null }[];
  probes: Triple[];
  loft: { solid: string; boundsBoneLocal: { min: Triple; max: Triple }; toleranceM: number };
};

const vectors = fixture<Vectors>("🧊️model-frame");
const near = (a: readonly number[], b: readonly number[], eps = 1e-9): void => void a.forEach((v, i) => expect(Math.abs(v - b[i]!), `${a} vs ${b}`).toBeLessThanOrEqual(eps));
const FINE = { chordM: 0.002, minSegments: 64, maxSegments: 64 };

const matrixOf = (t?: { translation?: readonly number[]; rotation?: readonly number[]; scale?: readonly number[] } | null): Matrix4 =>
  new Matrix4().compose(new Vector3(...(t?.translation ?? [0, 0, 0])), new Quaternion(...((t?.rotation ?? [0, 0, 0, 1]) as [number, number, number, number])).normalize(), new Vector3(...(t?.scale ?? [1, 1, 1])));

const stepOf = (r: NonNullable<Vectors["transforms"][number]["repeat"]>): Matrix4 => {
  const origin = new Vector3(...(r.origin ?? [0, 0, 0]));
  const turn = new Matrix4().makeTranslation(origin).multiply(new Matrix4().makeRotationAxis(new Vector3(...(r.rotationAxis ?? [0, 1, 0])).normalize(), r.rotationRad ?? 0)).multiply(new Matrix4().makeTranslation(origin.clone().negate()));
  return new Matrix4().makeTranslation(new Vector3(...(r.translation ?? [0, 0, 0]))).multiply(turn);
};

const mirror = new Matrix4().makeScale(1, 1, -1);

describe("🧊️ frame conversion between the staffage world and the neutral model", () => {
  it.each(vectors.points)("maps the point $name by (x, y, z) -> (z, y, x)", ({ staffage, neutral }) => {
    near(toNeutral(staffage), neutral);
  });

  it.each(vectors.pseudo)("maps the pseudo-vector $name, negated and swapped by the reflection", ({ staffage, neutral }) => {
    near(pseudoToNeutral(staffage), neutral);
  });

  it.each(vectors.frames)("places the bone frame $name: hand-computed basis, three.js quaternion, right-handed", (frame) => {
    const basis = basisOf(frame);
    near(basis.flat(), frame.basis.flat());
    near(toNeutral(frame.o), frame.translation);
    const q = quaternionOf(basis);
    const oracle = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(new Vector3(...frame.basis[0]), new Vector3(...frame.basis[1]), new Vector3(...frame.basis[2])));
    expect(Math.abs(q[0] * oracle.x + q[1] * oracle.y + q[2] * oracle.z + q[3] * oracle.w)).toBeCloseTo(1, 12);
    expect(Math.hypot(...q)).toBeCloseTo(1, 12);
    const x = new Vector3(...basis[0]), y = new Vector3(...basis[1]), z = new Vector3(...basis[2]);
    near(x.clone().cross(y).toArray(), z.toArray());
    for (const [axis, column] of [[[1, 0, 0], basis[0]], [[0, 1, 0], basis[1]], [[0, 0, 1], basis[2]]] as const) near(new Vector3(...axis).applyQuaternion(new Quaternion(...q)).toArray(), column, 1e-12);
  });

  it.each(vectors.relative)("carries the child frame into its parent's node frame: $name", ({ parent, child, translation, basis }) => {
    const p = basisOf(parent);
    const c = basisOf(child);
    const inverse = inverseBasis(p);
    const delta = toNeutral(child.o).map((v, i) => v - toNeutral(parent.o)[i]!) as Triple;
    near(rotate(inverse, delta), translation);
    near(multiply(inverse, c).flat(), basis.flat());
  });

  it("derives every local placement of the sample rig from the rest frames: three.js world composition equals the solved frames", () => {
    const asset = sampleFigure();
    const rest = restFrames(asset.rig);
    const nodes = skeletonOf(asset);
    const world = new Map<string, Matrix4>();
    for (const node of nodes) {
      const local = new Matrix4().compose(new Vector3(...node.translation), new Quaternion(...(node.rotation as [number, number, number, number])), new Vector3(1, 1, 1));
      world.set(node.id, node.parent === null ? local : world.get(node.parent)!.clone().multiply(local));
    }
    for (const bone of asset.rig.bones) {
      const f = rest[bone.id]!.frame;
      const expected = new Matrix4().makeBasis(new Vector3(...basisOf(f)[0]), new Vector3(...basisOf(f)[1]), new Vector3(...basisOf(f)[2])).setPosition(new Vector3(...toNeutral(f.o)));
      near(world.get(bone.id)!.toArray(), expected.toArray(), 1e-9);
    }
    expect(placementsOf(asset, Object.fromEntries(Object.entries(rest).map(([id, s]) => [id, s.frame])))[asset.rig.root]!.translation).toEqual(skeletonOf(asset)[0]!.translation);
  });

  it.each(vectors.transforms)("mirrors the solid transform and repeat: $name puts every probe where mirroring the transformed point would", ({ transform, repeat }) => {
    const t = mirrorTransform(transform ?? undefined);
    const r = mirrorRepeat(repeat ?? undefined);
    const ours = matrixOf(t);
    const expected = mirror.clone().multiply(matrixOf(transform));
    const count = repeat?.count ?? 1;
    const stepExpected = repeat === null ? new Matrix4() : stepOf(repeat);
    const stepOurs = r === undefined ? new Matrix4() : stepOf(r as never);
    let copyExpected = new Matrix4();
    let copyOurs = new Matrix4();
    for (let k = 0; k < count; k++) {
      for (const probe of vectors.probes) {
        const want = new Vector3(...probe).applyMatrix4(expected).applyMatrix4(new Matrix4()).toArray();
        const wantCopy = new Vector3(...probe).applyMatrix4(matrixOf(transform)).applyMatrix4(copyExpected).applyMatrix4(mirror).toArray();
        const got = new Vector3(...probe).applyMatrix4(ours).applyMatrix4(copyOurs).toArray();
        if (k === 0) near(new Vector3(...probe).applyMatrix4(ours).toArray(), want, 1e-12);
        near(got, wantCopy, 1e-12);
      }
      copyExpected = stepExpected.clone().multiply(copyExpected);
      copyOurs = stepOurs.clone().multiply(copyOurs);
    }
  });

  it.each(["sample-figure", "dairy-cow", "bookshelf", "toaster"])("reproduces in the staffage world, at rest and in the default pose, the point set of the raw kernel tessellation carried by the bone frames (the reference of the authoring tool volume-lib) for %s, wound outward", async (id) => {
    const asset = id === "sample-figure" ? sampleFigure() : volumeAssets().find((a) => a.id === id);
    if (asset === undefined) return;
    const document = await modelOf(asset, { scale: 50, views: ["light-solid"], poses: [asset.presentation.defaultPose], clips: [], at: { kind: "pose", pose: asset.presentation.defaultPose } });
    const nodes = skeletonOf(asset);
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
    const states = [
      { frames: Object.fromEntries(Object.entries(restFrames(asset.rig)).map(([k, v]) => [k, v.frame])), world: restWorldMatrices(nodes), weighted: true },
      { frames: Object.fromEntries(Object.entries(state.bones).map(([k, b]) => [k, { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] }])), world: new Map(Object.entries(poseModel(document, asset.presentation.defaultPose).world)), weighted: false },
    ];
    for (const { frames, world, weighted } of states) {
      for (const solid of asset.volume!.solids) {
        if (!weighted && solid.weights !== undefined) continue;
        if (solid.maxDenominator !== undefined && solid.maxDenominator < 50) continue;
        const raw = tessellateSolid({ ...solid, node: solid.bone } as never, FINE);
        const ours = tessellateSolid(solidOf(solid, "m"), FINE);
        expect(ours.positions.length, solid.id).toBe(raw.positions.length);
        const f = (frames as Record<string, Frame>)[solid.bone]!;
        const frame = new Matrix4().makeBasis(new Vector3(...f.u), new Vector3(...f.v), new Vector3(...f.w)).setPosition(new Vector3(...f.o));
        for (let i = 0; i < raw.positions.length; i += 3) {
          const staffage = new Vector3(raw.positions[i], raw.positions[i + 1], raw.positions[i + 2]).applyMatrix4(frame);
          near(transformPoint(world.get(solid.bone)!, [ours.positions[i]!, ours.positions[i + 1]!, ours.positions[i + 2]!]), [staffage.z, staffage.y, staffage.x], 1e-8);
        }
        const at = (i: number): Vector3 => new Vector3(ours.positions[i * 3], ours.positions[i * 3 + 1], ours.positions[i * 3 + 2]);
        let volume = 0;
        for (let t = 0; t < ours.indices.length; t += 3) volume += at(ours.indices[t]!).dot(at(ours.indices[t + 1]!).cross(at(ours.indices[t + 2]!))) / 6;
        expect(volume, `${asset.id}/${solid.id} outward`).toBeGreaterThan(0);
      }
    }
  });

  it("keeps the loft sections as authored: the torso sections, turned by their rotation, lie across the bone, extent from the hand-computed numbers", () => {
    const asset = sampleFigure();
    const solid = asset.volume!.solids.find((s) => s.id === vectors.loft.solid)!;
    expect(solid.kind).toBe("loft");
    const geometry = tessellateSolid(solidOf(solid, "m"), FINE);
    const box = new Box3();
    for (let i = 0; i < geometry.positions.length; i += 3) box.expandByPoint(new Vector3(geometry.positions[i], geometry.positions[i + 1], -geometry.positions[i + 2]!));
    const { min, max } = vectors.loft.boundsBoneLocal;
    near(box.min.toArray(), min, vectors.loft.toleranceM);
    near(box.max.toArray(), max, vectors.loft.toleranceM);
  });
});
