/** 🎞️ Viewer scenes: `sceneOf` prepares the shared buffers once for every model scene; `frameOf` selects the active scene (default the first) by filtering the draw list only, so switching rebuilds and re-uploads nothing; bounds, picking, follow and the software rasteriser honour it. Oracles: the hand-derived bounds of the language-agnostic vector (also checked against three's `Box3` in the GLB suite) and the kernel's own `boundsOf`. */
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { boundsOf as kernelBounds } from "../../../📐️bounds/🟦️.ts";
import { tessellate } from "../../../🔺️tessellation/🟦️.ts";
import { fixture as modelFixture } from "../../../🧪️tests/🧰️support/🟦️.ts";
import { cameraOf } from "../../📷️camera/🟦️.ts";
import { boundsOf, createFrame, frameOf } from "../../🎞️frame/🟦️.ts";
import { followPoint } from "../../🎥️follow/🟦️.ts";
import { pick } from "../../🎯️picking/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { SoftwareBackend } from "../../🖼️software/🟦️.ts";
import { posedPositions } from "../../🎞️frame/🟦️.ts";

const data = modelFixture("🎞️scenes");
const quiet = { report: () => undefined, cancelled: () => false };
const resolved = (await tessellate(data.document, data.quality, quiet)) as ModelDocument;
const ids = ["body", "structure", "thermal"] as const;
const camera = { ...cameraOf("front"), target: [0.3, 1, 0] as [number, number, number], distance: 6, orthoHeight: 2.4, extent: 1.2 };
const meshCarriers: Record<(typeof ids)[number], string[]> = { body: ["hull-body", "hull-arm", "skinned-body-structure"], structure: ["frame-body", "frame-arm", "skinned-body-structure"], thermal: ["region-a", "skinned-thermal"] };

describe("🎞️ viewer scenes", () => {
  const scene = sceneOf(resolved);

  it("prepares one set of shared buffers and the scenes of the model", () => {
    expect(scene.scenes.map((s) => s.id)).toEqual([...ids]);
    expect(scene.scenes.map((s) => s.title.en)).toEqual(["Body", "Structure", "Thermal"]);
    ids.forEach((id, k) => {
      const inside = scene.primitives.filter((p) => p.scenes.includes(k)).map((p) => p.node);
      expect([...new Set(inside)].sort()).toEqual([...meshCarriers[id]].sort());
    });
    expect(scene.primitives.filter((p) => p.node === "skinned-body-structure").every((p) => p.scenes.join() === "0,1")).toBe(true);
  });

  it("the draw list follows the active scene, defaults to the first and rebuilds no buffer", () => {
    const frame = createFrame(scene);
    const vertices = scene.vertices, indices = scene.indices;
    const count = (id?: string) => frameOf(scene, { camera, aspect: 1, scene: id }, frame).drawCount;
    ids.forEach((id, k) => expect(count(id), id).toBe(scene.primitives.filter((p) => p.scenes.includes(k)).length));
    expect(count()).toBe(count("body"));
    expect(frame.sceneIndex).toBe(0);
    expect(scene.vertices).toBe(vertices);
    expect(scene.indices).toBe(indices);
    expect(() => frameOf(scene, { camera, aspect: 1, scene: "nope" }, frame)).toThrow(/unknown scene/u);
  });

  it.each(ids)("%s: the bounds of the frame equal the vector at rest and bent, and the kernel's boundsOf", (id) => {
    for (const [pose, key] of [[undefined, "rest"], ["bent", "bent"]] as const) {
      const frame = frameOf(scene, { camera, aspect: 1, scene: id, pose });
      const b = boundsOf(scene, frame);
      const want = data.expected.bounds[id][key];
      const kernel = kernelBounds(resolved, pose, { scene: id })!;
      for (let k = 0; k < 3; k++) {
        expect(b.min[k]!).toBeCloseTo(want.min[k], 5);
        expect(b.max[k]!).toBeCloseTo(want.max[k], 5);
        expect(b.min[k]!).toBeCloseTo(kernel.min[k]!, 5);
        expect(b.max[k]!).toBeCloseTo(kernel.max[k]!, 5);
      }
    }
  });

  it("picking sees only the active scene: the same ray hits a different solid in every scene", () => {
    const ray = { origin: [0, 1.2, 5] as [number, number, number], direction: [0, 0, -1] as [number, number, number] };
    const hits = ids.map((id) => pick(scene, frameOf(scene, { camera, aspect: 1, scene: id }), ray)?.solid);
    expect(hits).toEqual(["hull-torso", "frame-column", "region-core"]);
    const side = { origin: [0.7, 1, 5] as [number, number, number], direction: [0, 0, -1] as [number, number, number] };
    expect(pick(scene, frameOf(scene, { camera, aspect: 1, scene: "thermal" }), side)).toBeNull();
    expect(pick(scene, frameOf(scene, { camera, aspect: 1, scene: "body" }), side)?.node).toBe("hull-arm");
  });

  it("follows the bounds of the active scene", () => {
    const out: [number, number, number] = [0, 0, 0];
    for (const id of ids) {
      const frame = frameOf(scene, { camera, aspect: 1, scene: id });
      expect(followPoint(scene, frame, "bounds", out, (p) => posedPositions(scene, frame, p))).toBe(true);
      const want = data.expected.bounds[id].rest;
      for (let k = 0; k < 3; k++) expect(out[k]!).toBeCloseTo((want.min[k] + want.max[k]) / 2, 5);
    }
  });

  it("the software rasteriser draws each scene differently and a scene switch is reversible and deterministic", () => {
    const backend = new SoftwareBackend(48, 48);
    backend.load(scene);
    const shot = (id: string) => {
      backend.render(frameOf(scene, { camera, aspect: 1, scene: id }), { background: [0, 0, 0, 1] });
      return Array.from(backend.pixels);
    };
    const lit = (px: number[]) => px.reduce((n, v, i) => n + (i % 4 !== 3 && v > 0 ? 1 : 0), 0);
    const body = shot("body"), structure = shot("structure"), thermal = shot("thermal");
    expect(new Set([body.join(), structure.join(), thermal.join()]).size).toBe(3);
    for (const px of [body, structure, thermal]) expect(lit(px)).toBeGreaterThan(30);
    expect(shot("body")).toEqual(body);
    expect(lit(structure)).toBeLessThan(lit(body));
  });
});
