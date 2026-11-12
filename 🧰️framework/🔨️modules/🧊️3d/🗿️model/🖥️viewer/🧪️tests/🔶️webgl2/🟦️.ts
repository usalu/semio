/** 🔶️ WebGL2 backend against a recording stub context (no GPU): shader/uniform bookkeeping, buffer and palette-texture upload, draw order and state per pass, uniform offsets, DPR sizing, context loss and restore. What a real GPU draws is checked separately (see the GPU harness). */
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { MATERIAL_STRIDE } from "../../🧬️contract/🟦️.ts";
import { cameraOf } from "../../📷️camera/🟦️.ts";
import { frameOf } from "../../🎞️frame/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { WebGl2Backend } from "../../🔶️webgl2/🟦️.ts";
import { FRAGMENT_SOURCE, VERTEX_SOURCE } from "../../🔶️webgl2/🎨️shaders/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

const rig = fixture("🧱️rig").document as ModelDocument;
type Call = [string, ...unknown[]];

function stub() {
  const calls: Call[] = [];
  let lost = false;
  const gl: Record<string, unknown> = new Proxy({}, {
    get: (_, name: string) => {
      if (name === "isContextLost") return () => lost;
      if (name === "getShaderParameter" || name === "getProgramParameter") return () => true;
      if (name === "getUniformLocation") return (_p: unknown, n: string) => (calls.push(["lookup", n]), { uniform: n });
      if (/^[A-Z0-9_]+$/.test(name)) return name;
      return (...args: unknown[]) => {
        calls.push([name, ...args]);
        return name.startsWith("create") ? { made: name, n: calls.length } : undefined;
      };
    },
  });
  const listeners = new Map<string, Set<(e: Event) => void>>();
  const canvas = {
    width: 300, height: 150,
    getContext: (kind: string) => (kind === "webgl2" ? gl : null),
    addEventListener: (t: string, f: (e: Event) => void) => (listeners.get(t) ?? listeners.set(t, new Set()).get(t)!).add(f),
    removeEventListener: (t: string, f: (e: Event) => void) => listeners.get(t)?.delete(f),
  };
  const fire = (type: string) => {
    let prevented = false;
    for (const f of listeners.get(type) ?? []) f({ preventDefault: () => (prevented = true) } as unknown as Event);
    return prevented;
  };
  return { canvas: canvas as unknown as HTMLCanvasElement, calls, fire, listeners, setLost: (v: boolean) => (lost = v) };
}

const scene = sceneOf(rig);
const frame = frameOf(scene, { pose: "bent", camera: cameraOf("perspective"), aspect: 2 });
const named = (calls: Call[], name: string) => calls.filter((c) => c[0] === name);

describe("shaders", () => {
  it("declares every uniform the backend looks up, in the right stage", () => {
    const s = stub();
    new WebGl2Backend(s.canvas);
    const looked = named(s.calls, "lookup").map((c) => c[1]);
    const wanted = ["uViewProjection", "uModel", "uPalette", "uPaletteOffset", "uBase", "uEmissiveUnlit", "uSurface", "uEye", "uLightDirection", "uLightColor", "uSky", "uGround"];
    expect([...looked].sort()).toEqual([...wanted].sort());
    for (const n of wanted) expect(`${VERTEX_SOURCE}${FRAGMENT_SOURCE}`.includes(`uniform`) && `${VERTEX_SOURCE}${FRAGMENT_SOURCE}`.includes(n), n).toBe(true);
    for (const n of ["uViewProjection", "uModel", "uPalette", "uPaletteOffset"]) expect(VERTEX_SOURCE).toContain(n);
    for (const n of ["uBase", "uEmissiveUnlit", "uSurface", "uEye", "uLightDirection", "uLightColor", "uSky", "uGround"]) expect(FRAGMENT_SOURCE).toContain(n);
    for (const src of [VERTEX_SOURCE, FRAGMENT_SOURCE]) expect(src.startsWith("#version 300 es")).toBe(true);
    expect(VERTEX_SOURCE).toContain("texelFetch");
    expect(FRAGMENT_SOURCE).toContain("gl_FrontFacing");
  });
  it("matches the attribute layout of the interleaved buffers", () => {
    for (const n of ["location = 0) in vec3 aPosition", "location = 1) in vec3 aNormal", "location = 2) in vec4 aColor", "location = 3) in vec4 aJoints", "location = 4) in vec4 aWeights"]) expect(VERTEX_SOURCE).toContain(n);
  });
});

describe("backend", () => {
  it("compiles and links, uploads buffers and a float palette texture", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    expect(named(s.calls, "compileShader")).toHaveLength(2);
    expect(named(s.calls, "linkProgram")).toHaveLength(1);
    expect(named(s.calls, "shaderSource").map((c) => c[2])).toEqual([VERTEX_SOURCE, FRAGMENT_SOURCE]);
    b.load(scene);
    const data = named(s.calls, "bufferData");
    expect(data).toHaveLength(3);
    expect(data[0]![2]).toBe(scene.vertices);
    expect(data[1]![2]).toBe(scene.skinning);
    expect(data[2]![2]).toBe(scene.indices);
    const tex = named(s.calls, "texImage2D")[0]!;
    expect(tex.slice(3, 6)).toEqual(["RGBA32F", 4, scene.jointCount]);
    expect(named(s.calls, "vertexAttribPointer").map((c) => [c[1], c[2], c[5], c[6]])).toEqual([[0, 3, 40, 0], [1, 3, 40, 12], [2, 4, 40, 24], [3, 4, 32, 0], [4, 4, 32, 16]]);
  });

  it("disables the skin attributes for rigid-only scenes", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.load(sceneOf({ ...rig, nodes: rig.nodes.map((n) => ({ ...n, skin: undefined })) } as unknown as ModelDocument));
    expect(named(s.calls, "disableVertexAttribArray").map((c) => c[1])).toEqual([3, 4]);
  });

  it("draws every frame entry once with the right primitive mode, range and uniforms", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.load(scene);
    s.calls.length = 0;
    b.render(frame, { background: [0.1, 0.2, 0.3, 1] });
    const draws = named(s.calls, "drawElements");
    expect(draws).toHaveLength(frame.drawCount);
    draws.forEach((c, i) => {
      const d = frame.draws[i]!, p = scene.primitives[d.primitive]!;
      expect(c.slice(1)).toEqual([d.pass === 2 ? "LINES" : "TRIANGLES", p.indexCount, "UNSIGNED_INT", p.indexStart * 4]);
    });
    const models = named(s.calls, "uniformMatrix4fv").filter((c) => (c[1] as { uniform: string }).uniform === "uModel");
    expect(models).toHaveLength(frame.drawCount);
    models.forEach((c, i) => {
      expect(c[3]).toBe(frame.matrices);
      expect(c[4]).toBe(frame.draws[i]!.matrixOffset);
      expect(c[5]).toBe(16);
    });
    const offsets = named(s.calls, "uniform1i").filter((c) => (c[1] as { uniform: string }).uniform === "uPaletteOffset").map((c) => c[2]);
    expect(offsets).toEqual(frame.draws.slice(0, frame.drawCount).map((d) => d.paletteOffset));
    const bases = named(s.calls, "uniform4f").filter((c) => (c[1] as { uniform: string }).uniform === "uBase");
    bases.forEach((c, i) => expect(c.slice(2)).toEqual(Array.from(frame.materials.subarray(frame.draws[i]!.material * MATERIAL_STRIDE, frame.draws[i]!.material * MATERIAL_STRIDE + 4))));
    const upload = named(s.calls, "texSubImage2D")[0]!;
    expect(upload[upload.length - 1]).toBe(frame.palette);
    expect(named(s.calls, "clearColor")[0]!.slice(1)).toEqual([0.1, 0.2, 0.3, 1]);
  });

  it("orders state per pass: blend without depth writes for translucent, no polygon offset and unlit for lines", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.load(scene);
    s.calls.length = 0;
    b.render(frame, { background: [0, 0, 0, 1] });
    const log = s.calls.map((c) => c[0] + (c[1] !== undefined && typeof c[1] === "string" ? `:${c[1]}` : "") + (c[0] === "depthMask" ? `:${c[1]}` : ""));
    const firstBlendDraw = frame.draws.findIndex((d) => d.pass === 1), firstLineDraw = frame.draws.findIndex((d) => d.pass === 2);
    const drawAt = (n: number) => log.reduce<number[]>((a, v, i) => (v.startsWith("drawElements") ? [...a, i] : a), [])[n]!;
    expect(log.indexOf("enable:BLEND")).toBeGreaterThan(drawAt(firstBlendDraw - 1));
    expect(log.indexOf("enable:BLEND")).toBeLessThan(drawAt(firstBlendDraw));
    expect(named(s.calls, "blendFunc")[0]!.slice(1)).toEqual(["ONE", "ONE_MINUS_SRC_ALPHA"]);
    expect(log.indexOf("depthMask:false")).toBeLessThan(drawAt(firstBlendDraw));
    expect(log.indexOf("disable:POLYGON_OFFSET_FILL")).toBeGreaterThan(drawAt(firstLineDraw - 1));
    expect(log.indexOf("disable:POLYGON_OFFSET_FILL")).toBeLessThan(drawAt(firstLineDraw));
    const unlit = named(s.calls, "uniform4f").filter((c) => (c[1] as { uniform: string }).uniform === "uEmissiveUnlit");
    for (let i = firstLineDraw; i < frame.drawCount; i++) expect(unlit[i]![5]).toBe(1);
    expect(named(s.calls, "enable").filter((c) => c[1] === "DEPTH_TEST")).toHaveLength(1);
  });

  it("toggles face culling only for double-sided materials", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.load(scene);
    s.calls.length = 0;
    b.render(frame, { background: [0, 0, 0, 1] });
    const doubleSided = frame.draws.slice(0, frame.drawCount).filter((d) => d.pass !== 2 && frame.materials[d.material * MATERIAL_STRIDE + 10] === 1).length;
    expect(doubleSided).toBeGreaterThan(0);
    expect(named(s.calls, "disable").filter((c) => c[1] === "CULL_FACE").length).toBeGreaterThan(0);
  });

  it("sizes the backing store by css size times the pixel ratio", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.resize(200, 100, 2);
    expect([s.canvas.width, s.canvas.height]).toEqual([400, 200]);
    b.resize(200.4, 99.6, 1.5);
    expect([s.canvas.width, s.canvas.height]).toEqual([301, 149]);
    b.render(frame, { background: [0, 0, 0, 1] });
    expect(named(s.calls, "viewport")).toHaveLength(0);
  });

  it("survives context loss: cancels the default, skips rendering, rebuilds and re-uploads on restore", () => {
    const s = stub();
    const seen: string[] = [];
    const b = new WebGl2Backend(s.canvas, (st) => seen.push(st));
    b.load(scene);
    expect(s.fire("webglcontextlost")).toBe(true);
    s.calls.length = 0;
    b.render(frame, { background: [0, 0, 0, 1] });
    expect(named(s.calls, "drawElements")).toHaveLength(0);
    b.load(scene);
    expect(named(s.calls, "bufferData")).toHaveLength(0);
    s.fire("webglcontextrestored");
    expect(named(s.calls, "linkProgram")).toHaveLength(1);
    expect(named(s.calls, "bufferData")).toHaveLength(3);
    s.calls.length = 0;
    b.render(frame, { background: [0, 0, 0, 1] });
    expect(named(s.calls, "drawElements")).toHaveLength(frame.drawCount);
    expect(seen).toEqual(["ready", "lost", "restored"]);
  });

  it("releases resources and listeners on dispose and throws when WebGL2 is unavailable", () => {
    const s = stub();
    const b = new WebGl2Backend(s.canvas);
    b.load(scene);
    b.dispose();
    expect(named(s.calls, "deleteProgram")).toHaveLength(1);
    expect(named(s.calls, "deleteBuffer")).toHaveLength(3);
    expect([...s.listeners.values()].every((set) => set.size === 0)).toBe(true);
    expect(() => new WebGl2Backend({ getContext: () => null, addEventListener: () => undefined } as unknown as HTMLCanvasElement)).toThrow(/webgl2/);
  });
});
