/** 🖼️ Software rasteriser: coverage masks and depths equal a per-pixel three Raycaster on the same posed meshes (language-agnostic vectors recorded from three), colours, sRGB encoding, culling, blending, depth-correct lines, near clipping, resize, determinism, variants and animated colours in the image. */
import { Color } from "three";
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { clipPlanes, cameraOf, fitCamera } from "../../📷️camera/🟦️.ts";
import { boundsOf, createFrame, frameOf } from "../../🎞️frame/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { SoftwareBackend } from "../../🖼️software/🟦️.ts";
import { fixture, withoutLines } from "../🧰️support/🟦️.ts";

const rig = fixture("🧱️rig").document as ModelDocument;
const data = fixture("🖼️raster");
const white = [1, 1, 1, 1] as const;

const material = (id: string, extra: object = {}) => ({ id, title: { en: id, de: id }, baseColor: [0.25, 0.5, 1, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: true, ...extra });
const quadAt = (z: number, mat: string, size = 1) => [-size, -size, z, size, -size, z, size, size, z, -size, size, z];
const world = (primitives: object[], materials: object[]): ModelDocument => ({
  version: "semio.model/v1", id: "w", title: { en: "w", de: "w" }, solids: [], skins: [], variants: [], poses: [], animations: [],
  nodes: [{ id: "n", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], mesh: "m" }],
  meshes: [{ id: "m", primitives: primitives as never }], materials: materials as never,
});
const quad = (z: number, mat: string, extra: object = {}) => ({ mode: "triangles", material: mat, positions: quadAt(z, mat), normals: Array(4).fill([0, 0, 1]).flat(), indices: [0, 1, 2, 0, 2, 3], ...extra });

const draw = (doc: ModelDocument, camera = { ...cameraOf("front"), target: [0, 0, 0] as [number, number, number], distance: 5, orthoHeight: 4, extent: 2 }, size = [32, 32], pose?: never, variant: string | null = null, background: readonly [number, number, number, number] = [0, 0, 0, 1]) => {
  const scene = sceneOf(doc);
  const backend = new SoftwareBackend(size[0], size[1]);
  backend.load(scene);
  const frame = frameOf(scene, { camera, aspect: size[0]! / size[1]!, pose, variant });
  backend.render(frame, { background });
  return { backend, scene, frame, px: (x: number, y: number) => Array.from(backend.pixels.subarray((y * size[0]! + x) * 4, (y * size[0]! + x) * 4 + 4)) };
};

describe("against three's per-pixel raycaster", () => {
  const scene = sceneOf(withoutLines(rig));
  for (const img of data.images) {
    it(`${img.preset}: silhouette and depth`, () => {
      const backend = new SoftwareBackend(img.width, img.height);
      backend.load(scene);
      const frame = frameOf(scene, { pose: data.pose, camera: img.camera, aspect: img.width / img.height });
      backend.render(frame, { background: [0, 0, 0, 1] });
      let mismatch = 0, covered = 0;
      for (let y = 0; y < img.height; y++) for (let x = 0; x < img.width; x++) {
        const o = (y * img.width + x) * 4;
        const ours = backend.pixels[o]! + backend.pixels[o + 1]! + backend.pixels[o + 2]! > 0;
        const theirs = img.mask[y]![x] === "1";
        if (theirs) covered++;
        if (ours !== theirs) mismatch++;
      }
      expect(covered).toBeGreaterThan(20);
      expect(mismatch / (img.width * img.height), `${img.preset} mismatching pixels ${mismatch}`).toBeLessThanOrEqual(data.maxMismatch);
      const { near, far } = clipPlanes(img.camera);
      for (const [x, y, distance] of img.depth) {
        const z = backend.depth[y * img.width + x]!;
        if (z === Infinity) continue;
        const view = img.camera.projection === "orthographic" ? near + ((z + 1) / 2) * (far - near) : (2 * far * near) / (far + near - z * (far - near));
        expect(Math.abs(view - distance) / distance, `${img.preset} depth at ${x},${y}`).toBeLessThan(data.depthTolerance);
      }
    });
  }
});

describe("pixels", () => {
  it("paints an unlit colour sRGB-encoded (gamma 2.2) on the background", () => {
    const { px } = draw(world([quad(0, "a")], [material("a")]));
    const centre = px(16, 16);
    expect(centre[0]).toBeCloseTo(255 * Math.pow(0.25, 1 / 2.2), 0);
    expect(centre[1]).toBeCloseTo(255 * Math.pow(0.5, 1 / 2.2), 0);
    expect(centre[2]).toBe(255);
    expect(centre[3]).toBe(255);
    const c = new Color().setRGB(0.25, 0.5, 1, "srgb-linear").convertLinearToSRGB();
    expect(Math.abs(centre[0]! - c.r * 255)).toBeLessThan(4);
    expect(Math.abs(centre[1]! - c.g * 255)).toBeLessThan(4);
    expect(px(0, 0)).toEqual([0, 0, 0, 255]);
  });

  it("fills the background colour and straight alpha", () => {
    const { px } = draw(world([quad(0, "a", { })], [material("a")]), undefined, undefined, undefined, undefined, [0.2, 0.4, 0.6, 0.5]);
    expect(px(0, 0)).toEqual([51, 102, 153, 128]);
  });

  it("culls back faces unless double-sided", () => {
    const flipped = { indices: [0, 2, 1, 0, 3, 2] };
    expect(draw(world([quad(0, "a", flipped)], [material("a")])).px(16, 16)).toEqual([0, 0, 0, 255]);
    expect(draw(world([quad(0, "a", flipped)], [material("a", { doubleSided: true })])).px(16, 16)[2]).toBe(255);
  });

  it("depth-tests: the nearer quad wins whatever the draw order", () => {
    const near = draw(world([quad(1, "b"), quad(-1, "a")], [material("a", { baseColor: [1, 0, 0, 1] }), material("b", { baseColor: [0, 1, 0, 1] })])).px(16, 16);
    expect(near).toEqual([0, 255, 0, 255]);
  });

  it("blends translucent surfaces over what is behind, back to front", () => {
    const blend = material("g", { baseColor: [0, 1, 0, 0.5], alphaMode: "blend" });
    const { px } = draw(world([quad(-1, "a"), quad(1, "g", { positions: quadAt(1, "g", 0.5) })], [material("a", { baseColor: [1, 0, 0, 1] }), blend]));
    const c = px(16, 16);
    expect(c[0]).toBeCloseTo(127.5, -1);
    expect(c[1]).toBeCloseTo(127.5, -1);
    expect(px(10, 10)).toEqual([255, 0, 0, 255]);
  });

  it("shows lines in front of and on a surface but hides them behind it", () => {
    const line = (z: number, y = 0) => ({ mode: "lines", material: "ink", positions: [-0.8, y, z, 0.8, y, z], indices: [0, 1] });
    const ink = material("ink", { baseColor: [1, 1, 1, 1] });
    const a = material("a", { baseColor: [1, 0, 0, 1] });
    const front = draw(world([quad(0, "a"), line(0.5)], [a, ink]));
    expect(front.px(16, 16)).toEqual([255, 255, 255, 255]);
    const on = draw(world([quad(0, "a"), line(0)], [a, ink]));
    expect(on.px(16, 16)).toEqual([255, 255, 255, 255]);
    const behind = draw(world([quad(0, "a"), line(-0.5)], [a, ink]));
    expect(behind.px(16, 16)).toEqual([255, 0, 0, 255]);
  });

  it("clips at the near plane when the camera sits inside a large box", () => {
    const camera = { ...cameraOf("perspective"), target: [0, 0, 0] as [number, number, number], distance: 0.2, extent: 2, fovY: 1.2 };
    const rigged = world([{ mode: "triangles", material: "a", positions: [-5, -5, -5, 5, -5, -5, 0, 5, -5, -5, -5, 5, 5, -5, 5, 0, 5, 5], normals: Array(6).fill([0, 0, 1]).flat(), indices: [0, 1, 2, 3, 5, 4] }], [material("a", { doubleSided: true })]);
    const { backend } = draw(rigged, camera);
    expect(backend.pixels.some((v) => v > 0)).toBe(true);
    expect(Array.from(backend.depth).every((z) => z === Infinity || (z >= -1 && z <= 1))).toBe(true);
  });

  it("shades lit surfaces by the light: surfaces facing the camera light are brighter than grazing ones", () => {
    const sphere = rig;
    const scene = sceneOf(withoutLines(sphere));
    const backend = new SoftwareBackend(48, 48);
    backend.load(scene);
    const cam = fitCamera(cameraOf("front"), boundsOf(scene, frameOf(scene, { camera: cameraOf("front"), aspect: 1 })), 1);
    backend.render(frameOf(scene, { camera: cam, aspect: 1 }), { background: [0, 0, 0, 1] });
    const lit = [...backend.pixels].filter((_, i) => i % 4 === 0).filter((v, i) => backend.depth[i]! !== Infinity);
    expect(Math.max(...lit)).toBeGreaterThan(Math.min(...lit) + 20);
  });

  it("resizes with the device pixel ratio and is deterministic", () => {
    const b = new SoftwareBackend(1, 1);
    b.resize(10, 5, 2);
    expect([b.width, b.height, b.readPixels().length]).toEqual([20, 10, 800]);
    const doc = world([quad(0, "a")], [material("a")]);
    const one = draw(doc).backend.pixels, two = draw(doc).backend.pixels;
    expect(Array.from(one)).toEqual(Array.from(two));
  });

  it("requires a loaded scene", () => {
    expect(() => new SoftwareBackend().render({} as never, { background: white })).toThrow(/no scene/);
  });
});

describe("variants and animated colours in the image", () => {
  const scene = sceneOf(withoutLines(rig));
  const cam = fitCamera(cameraOf("front"), boundsOf(scene, frameOf(scene, { camera: cameraOf("front"), aspect: 1 })), 1);
  const image = (pose: never, variant: string | null) => {
    const b = new SoftwareBackend(32, 32);
    b.load(scene);
    b.render(frameOf(scene, { pose, variant, camera: cam, aspect: 1 }), { background: [0, 0, 0, 1] });
    return Array.from(b.pixels);
  };
  it("looks different per variant and per clip time", () => {
    const light = image(undefined as never, "light"), dark = image(undefined as never, "dark");
    expect(light).not.toEqual(dark);
    const brightness = (px: number[]) => px.reduce((s, v, i) => s + (i % 4 < 3 ? v : 0), 0);
    expect(brightness(dark)).toBeLessThan(brightness(light));
    const t0 = image({ animation: "wave", timeS: 0 } as never, "dark"), t1 = image({ animation: "wave", timeS: 2 } as never, "dark");
    expect(t0).toEqual(t1);
    const emissive = image({ animation: "wave", timeS: 1.5 } as never, "light"), none = image({ animation: "wave", timeS: 0.5 } as never, "light");
    expect(emissive).not.toEqual(none);
  });
});
