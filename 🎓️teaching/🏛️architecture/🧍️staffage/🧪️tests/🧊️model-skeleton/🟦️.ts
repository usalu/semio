/** 🧊️ The rig as the neutral node tree: the nodes and the mechanism joints of the rig, every pose and every clip sampled through `evaluate` and replayed by the kernel against the evaluated bone frames, and the FBX of a clip replayed by three's `FBXLoader` and `AnimationMixer`.
 *
 * THE REFERENCES are `evaluate` (the single source of the bone frames, composed independently here from `(ahead, up, left)` to `(left, up, ahead)` with three.js matrices) and, for the FBX, three.js loading and animating the bytes.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️model-skeleton/🔣️.json — the vectors
 */
import { AnimationMixer, Matrix4, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { writeFbx } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { frameTimes, modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { catalogAsset } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { loadFbx, sampleFigure } from "../🧰️support/🧊️.ts";

type Joint = Record<string, unknown>;
type Vectors = {
  gullWing: { asset: string; primaries: number; secondaries: number; minimumSpanM: number; maximumSpanM: number; foldDeg: number; glideDeg: number };
  passengers: { asset: string; root: string; neck: string; head: string; outlets: [string, string][]; segments: string[] }[];
  joints: Record<string, Joint>;
  variants: { name: string; pose: string; poseBones: Record<string, unknown>; jointPatch: Record<string, Record<string, unknown>> | null; bone: string; expected: Joint }[];
  clips: Record<string, { fps: number; durationS: number; keys: number }>;
  tolerances: { poseWorldM: number; clipKeyWorldM: number; clipBetweenKeysM: number; clipBetweenKeysRad: number };
};
const vectors = fixture<Vectors>("🧊️model-skeleton");
const OPTIONS = { scale: 50, view: "light-solid" } as const;
const near = (a: readonly number[], b: readonly number[], eps: number, what = ""): void => void a.forEach((v, i) => expect(Math.abs(v - b[i]!), `${what} ${a} vs ${b}`).toBeLessThanOrEqual(eps));

const variantOf = (v: Vectors["variants"][number]): Asset => {
  const asset = structuredClone(sampleFigure()) as unknown as { rig: { bones: { id: string; joint: unknown }[] }; poses: { id: string; bones: Record<string, unknown> }[] };
  for (const [bone, joint] of Object.entries(v.jointPatch ?? {})) asset.rig.bones.find((b) => b.id === bone)!.joint = joint;
  const pose = asset.poses.find((p) => p.id === v.pose)!;
  pose.bones = { ...pose.bones, ...v.poseBones };
  return asset as unknown as Asset;
};

/** 🧮️ The neutral world matrix of an evaluated bone, composed here independently: columns (u, v, -w) mapped (x, y, z) -> (z, y, x). */
const expectedWorld = (frame: { o: { x: number; y: number; z: number }; u: { x: number; y: number; z: number }; v: { x: number; y: number; z: number }; w: { x: number; y: number; z: number } }): Matrix4 =>
  new Matrix4().makeBasis(new Vector3(frame.u.z, frame.u.y, frame.u.x), new Vector3(frame.v.z, frame.v.y, frame.v.x), new Vector3(-frame.w.z, -frame.w.y, -frame.w.x)).setPosition(frame.o.z, frame.o.y, frame.o.x);

const worldOf = (document: ModelDocument, pose: Parameters<typeof poseModel>[1]) => poseModel(document, pose);

describe("🧊️ the rig as the neutral node tree", () => {
  it("folds and opens the gull’s remiges from their anatomical attachments at all four scales", () => {
    const f = vectors.gullWing, asset = catalogAsset(f.asset);
    const bones = new Map(asset.rig.bones.map(b => [b.id,b]));
    const vector = (p: { x: number; y: number; z: number }) => new Vector3(p.x,p.y,p.z);
    for (const [pose, degrees] of [["standing",f.foldDeg],["gliding",f.glideDeg]] as const) {
      const state = evaluate(asset, { kind:"pose", pose }), tips: number[] = [];
      for (const side of ["n","f"]) {
        for (const [series,count,parent] of [["p",f.primaries,"hand"],["s",f.secondaries,"forearm"]] as const) {
          for (let i = 1; i <= count; i++) {
            const id = series+i+"-"+side;
            expect(bones.get(id)?.parent).toBe(parent+"-"+side);
            for (const vol of Object.values(asset.volumes ?? {})) {
              if (vol.status !== "authored") throw new Error(id);
              const solid = vol.body.find(q => q.bone === id);
              expect(solid, id).toBeDefined();
              expect(vol.thermal.some(q => "of" in q && q.of === solid!.id), id).toBe(true);
              expect(vol.structure.some(q => q.member === id), id).toBe(true);
            }
          }
        }
        const axis = (i: number) => vector(state.bones["p"+i+"-"+side]!.frame.u);
        expect(axis(1).angleTo(axis(f.primaries))*180/Math.PI).toBeCloseTo(degrees,5);
        const id = "p9-"+side, frame = state.bones[id]!.frame;
        tips.push(vector(frame.o).addScaledVector(vector(frame.u),bones.get(id)!.lengthM).z);
      }
      if (pose === "gliding") {
        const span = Math.abs(tips[1]!-tips[0]!);
        expect(span).toBeGreaterThan(f.minimumSpanM);
        expect(span).toBeLessThan(f.maximumSpanM);
      }
    }
  });

  it.each(vectors.passengers)("keeps the $asset passenger’s joints and respiratory landmarks attached at all scales", (p) => {
    const asset = catalogAsset(p.asset);
    const parentOf = (id: string) => asset.rig.bones.find(b => b.id === id)?.parent;
    expect(parentOf(p.head)).toBe(p.neck);
    expect(parentOf(p.neck)).toBe(p.root);
    for (const volume of Object.values(asset.volumes ?? {})) {
      expect(volume.status).toBe("authored");
      if (volume.status !== "authored") throw new Error(p.asset);
      for (const id of [p.neck, p.head, ...p.segments]) {
        expect(volume.body.some(s => s.bone === id), id).toBe(true);
        expect(volume.structure.some(s => s.member === id), id).toBe(true);
      }
      for (const [landmark, id] of p.outlets) {
        const anchor = asset.anchors.find(q => q.id === landmark)!;
        const outlet = volume.thermal.find(q => q.id === id);
        expect(anchor.bone).toBe(p.head);
        if (!outlet || !("bone" in outlet) || !outlet.transform?.translation) throw new Error(id);
        expect(outlet.bone).toBe(p.head);
        const distance = new Vector3(...outlet.transform.translation).distanceTo(new Vector3(anchor.x, anchor.y, anchor.z ?? 0));
        expect(distance, id).toBeLessThan(0.001);
      }
    }
  });

  it("makes one node per bone, parents first, with the hand-computed joints and a free root", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, OPTIONS);
    const bones = document.nodes.filter((n) => n.tags?.includes("bone"));
    expect(bones.map((n) => n.id).sort()).toEqual(asset.rig.bones.map((b) => b.id).sort());
    for (const bone of asset.rig.bones) {
      const node = bones.find((n) => n.id === bone.id)!;
      expect(node.parent).toBe(bone.parent);
      expect(node.joint?.type).toBe(vectors.joints[bone.id]!.type);
      near(node.joint!.axis, vectors.joints[bone.id]!.axis as number[], 1e-12, bone.id);
      for (const key of ["min", "max"] as const) {
        const want = vectors.joints[bone.id]![key] as number | undefined;
        if (want === undefined) expect(node.joint?.[key], `${bone.id}.${key}`).toBeUndefined();
        else expect(node.joint?.[key], `${bone.id}.${key}`).toBeCloseTo(want, 12);
      }
    }
    document.nodes.forEach((n, i) => {
      if (n.parent !== null) expect(document.nodes.findIndex((p) => p.id === n.parent)).toBeLessThan(i);
    });
  });

  it.each(vectors.variants)("maps $name to the mechanism joint, and every pose still replays through the kernel without a clamp", async (variant) => {
    const asset = variantOf(variant);
    const document = await modelOf(asset, { ...OPTIONS, poses: asset.poses.map((p) => p.id), clips: [] });
    const joint = document.nodes.find((n) => n.id === variant.bone)!.joint!;
    expect(joint.type).toBe(variant.expected.type);
    near(joint.axis, variant.expected.axis as number[], 1e-12);
    for (const key of ["min", "max", "twistMin", "twistMax"] as const) if (variant.expected[key] !== undefined) expect(joint[key], key).toBeCloseTo(variant.expected[key] as number, 12);
    for (const pose of document.poses) {
      const replay = worldOf(document, pose.id);
      expect(replay.issues, pose.id).toEqual([]);
      const state = evaluate(asset, { kind: "pose", pose: pose.id });
      for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.poseWorldM, `${pose.id}/${bone.id}`);
    }
  });

  it("replays every pose, named or key, of the sample figure to the evaluated bone frames", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: asset.poses.map((p) => p.id), clips: [] });
    expect(document.poses.map((p) => p.id)).toEqual(asset.poses.map((p) => p.id));
    for (const pose of document.poses) {
      const replay = worldOf(document, pose.id);
      expect(replay.issues, pose.id).toEqual([]);
      const state = evaluate(asset, { kind: "pose", pose: pose.id });
      for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.poseWorldM, `${pose.id}/${bone.id}`);
    }
  });

  it("samples every clip at its fps: the count, the key times and the evaluated frames at the keys", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    expect(document.animations.map((a) => a.id)).toEqual(asset.clips.map((c) => c.id));
    for (const clip of asset.clips) {
      const expected = vectors.clips[clip.id]!;
      const animation = document.animations.find((a) => a.id === clip.id)!;
      expect(frameTimes(clip)).toHaveLength(expected.keys);
      expect(animation.durationS).toBe(expected.durationS);
      expect(animation.loop).toBe(clip.loop);
      for (const channel of animation.channels.filter((c) => "node" in c.target)) {
        expect(channel.times).toHaveLength(expected.keys);
        expect(channel.times[0]).toBe(0);
        expect(channel.times[channel.times.length - 1]).toBe(expected.durationS);
        channel.times.forEach((t, i) => i > 0 && expect(t).toBeGreaterThan(channel.times[i - 1]!));
      }
      const keys = frameTimes(clip).slice(0, -1);
      for (const timeS of keys.filter((_, i) => i % 7 === 0)) {
        const replay = worldOf(document, { animation: clip.id, timeS });
        expect(replay.issues).toEqual([]);
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.clipKeyWorldM, `${clip.id}@${timeS}/${bone.id}`);
      }
      expect(animation.events.map((e) => e.t)).toEqual(clip.events.map((e) => e.t));
    }
  });

  it("samples between the keys within the tolerance of the evaluated frames, at arbitrary times", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    for (const clip of asset.clips) {
      for (const fraction of [0.013, 0.211, 0.5003, 0.77, 0.9917]) {
        const timeS = fraction * clip.durationS;
        const replay = worldOf(document, { animation: clip.id, timeS });
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) {
          const want = expectedWorld(state.bones[bone.id]!.frame).toArray();
          near(replay.world[bone.id]!.slice(12, 15), want.slice(12, 15), vectors.tolerances.clipBetweenKeysM, `${clip.id}@${timeS}/${bone.id} origin`);
          near(replay.world[bone.id]!.slice(0, 12), want.slice(0, 12), vectors.tolerances.clipBetweenKeysRad, `${clip.id}@${timeS}/${bone.id} axes`);
        }
      }
    }
  });

  it("carries root motion: a looping planted clip ends one cycle further ahead, and rotation keys never flip hemisphere", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    const walk = asset.clips.find((c) => c.id === "walk")!;
    const root = document.animations.find((a) => a.id === "walk")!.channels.find((c) => "node" in c.target && c.target.node === asset.rig.root && c.target.path === "translation")!;
    const n = root.times.length;
    const first = root.values.slice(0, 3);
    const last = root.values.slice((n - 1) * 3, n * 3);
    expect(last[2]! - first[2]!).toBeCloseTo(walk.advanceM!, 3);
    for (const animation of document.animations) for (const c of animation.channels.filter((k) => "node" in k.target && k.target.path === "rotation"))
      for (let i = 1; i < c.times.length; i++) expect(c.values.slice(i * 4, i * 4 + 4).reduce((s, v, k) => s + v * c.values[(i - 1) * 4 + k]!, 0)).toBeGreaterThanOrEqual(0);
  });

  it("replays the FBX of a clip through three's FBXLoader and AnimationMixer to the evaluated bone positions", async () => {
    const asset = sampleFigure();
    for (const clip of asset.clips) {
      const scene = loadFbx(await writeFbx(asset, { clip: clip.id, scale: 50 }));
      expect(scene.animations).toHaveLength(1);
      expect(scene.animations[0]!.duration).toBeCloseTo(clip.durationS, 6);
      const mixer = new AnimationMixer(scene);
      mixer.clipAction(scene.animations[0]!).play();
      for (const fraction of [0, 0.25, 0.5, 0.75, 0.9]) {
        const timeS = fraction * clip.durationS;
        mixer.setTime(timeS);
        scene.updateMatrixWorld(true);
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) {
          const at = scene.getObjectByName(bone.id)!.getWorldPosition(new Vector3());
          const f = state.bones[bone.id]!.frame.o;
          near([at.x, at.y, at.z], [f.z, f.y, f.x], vectors.tolerances.clipBetweenKeysM, `${clip.id}@${timeS}/${bone.id}`);
        }
      }
    }
  });
});
