/** 🖼️ Software reference rasteriser: the {@link RenderBackend} on the CPU. Z-buffer, near-plane clipping, back-face culling, flat Lambert + hemisphere shading (no specular), interpolated vertex colours, alpha blending, depth-tested 1 px lines with a constant plus slope-scaled bias (like polygon offset), sRGB output (gamma 2.2) like the GPU shader.
 *
 * It renders headlessly for tests and thumbnails; `depth` holds the NDC z of the nearest triangle per pixel (`Infinity` where empty). Allocates per frame; it is a reference, not a fast path.
 * The optional `operation` of {@link SoftwareBackend.render} is polled every few thousand pixels, vertices and line steps and reports progress per triangle; a cancelled render throws `CancelledError` and leaves `pixels` and `depth` as they were (the picture is composed in work buffers and committed at the end).
 * @see ../🧬️contract/🟦️.ts */
import type { Operation } from "../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MATERIAL_STRIDE, type Frame, type RenderBackend, type RenderStyle, type Scene } from "../🧬️contract/🟦️.ts";
import { posedPositions } from "../🎞️frame/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

const LINE_BIAS = 1e-5;
const GAMMA = 1 / 2.2;

type Clip = { x: number; y: number; z: number; w: number; r: number; g: number; b: number; a: number };

/** 🖨️ The headless backend; `readPixels` gives RGBA bytes, top row first. */
export class SoftwareBackend implements RenderBackend {
  readonly kind = "software";
  width: number;
  height: number;
  pixels: Uint8ClampedArray;
  depth: Float32Array;
  private scene: Scene | null = null;
  private work: { pixels: Uint8ClampedArray; depth: Float32Array } | null = null;

  constructor(width = 256, height = 256) {
    this.width = width;
    this.height = height;
    this.pixels = new Uint8ClampedArray(width * height * 4);
    this.depth = new Float32Array(width * height);
  }

  load(scene: Scene): void {
    this.scene = scene;
  }

  resize(cssWidth: number, cssHeight: number, pixelRatio: number): void {
    this.width = Math.max(1, Math.round(cssWidth * pixelRatio));
    this.height = Math.max(1, Math.round(cssHeight * pixelRatio));
    this.pixels = new Uint8ClampedArray(this.width * this.height * 4);
    this.depth = new Float32Array(this.width * this.height);
  }

  readPixels(): Uint8ClampedArray {
    return this.pixels;
  }

  dispose(): void {
    this.scene = null;
  }

  /** 📺️ Copies the picture into a 2D canvas (browser only), for the fallback when WebGL2 is unavailable. */
  present(canvas: { width: number; height: number; getContext(kind: "2d"): { putImageData(data: unknown, x: number, y: number): void } | null }, ImageDataClass: new (data: Uint8ClampedArray, w: number, h: number) => unknown): void {
    canvas.width = this.width;
    canvas.height = this.height;
    canvas.getContext("2d")?.putImageData(new ImageDataClass(this.pixels, this.width, this.height), 0, 0);
  }

  /** 🖌️ Draws one frame into `pixels` and `depth`; cancellable through the optional `operation`. */
  render(frame: Frame, style: RenderStyle, operation: Operation = silentOperation): void {
    const scene = this.scene;
    if (!scene) throw new Error("software backend: no scene loaded");
    const pace = new Pace(operation);
    pace.check();
    const { width: W, height: H } = this;
    if (this.work?.pixels.length !== W * H * 4) this.work = { pixels: new Uint8ClampedArray(W * H * 4), depth: new Float32Array(W * H) };
    const { pixels, depth } = this.work;
    const bg = style.background;
    for (let i = 0; i < W * H; i++) {
      if ((i & 1023) === 0) pace.tick(1024);
      pixels[i * 4] = bg[0] * 255; pixels[i * 4 + 1] = bg[1] * 255; pixels[i * 4 + 2] = bg[2] * 255; pixels[i * 4 + 3] = bg[3] * 255;
      depth[i] = Infinity;
    }
    const vp = frame.viewProjection;
    const light = frame.light;
    for (let di = 0; di < frame.drawCount; di++) {
      const d = frame.draws[di]!;
      const prim = scene.primitives[d.primitive]!;
      const mo = d.material * MATERIAL_STRIDE, m = frame.materials;
      const base = [m[mo]!, m[mo + 1]!, m[mo + 2]!, m[mo + 3]!];
      const emissive = [m[mo + 4]!, m[mo + 5]!, m[mo + 6]!];
      const unlit = m[mo + 7] === 1 || prim.mode === "lines", doubleSided = m[mo + 10] === 1, blend = m[mo + 11] === 1;
      const w = posedPositions(scene, frame, prim.index);
      const clip: Clip[] = new Array(prim.vertexCount);
      for (let i = 0; i < prim.vertexCount; i++) {
        pace.tick(4);
        const x = w[i * 3]!, y = w[i * 3 + 1]!, z = w[i * 3 + 2]!, o = (prim.vertexStart + i) * 10;
        clip[i] = {
          x: vp[0]! * x + vp[4]! * y + vp[8]! * z + vp[12]!, y: vp[1]! * x + vp[5]! * y + vp[9]! * z + vp[13]!, z: vp[2]! * x + vp[6]! * y + vp[10]! * z + vp[14]!, w: vp[3]! * x + vp[7]! * y + vp[11]! * z + vp[15]!,
          r: scene.vertices[o + 6]! * base[0]!, g: scene.vertices[o + 7]! * base[1]!, b: scene.vertices[o + 8]! * base[2]!, a: scene.vertices[o + 9]! * base[3]!,
        };
      }
      const toScreen = (c: Clip) => ({ x: (c.x / c.w * 0.5 + 0.5) * W, y: (1 - (c.y / c.w * 0.5 + 0.5)) * H, z: c.z / c.w, r: c.r, g: c.g, b: c.b, a: c.a });
      const put = (idx: number, r: number, g: number, b: number, a: number) => {
        const sr = portable.pow(Math.max(0, Math.min(1, r)), GAMMA) * 255, sg = portable.pow(Math.max(0, Math.min(1, g)), GAMMA) * 255, sb = portable.pow(Math.max(0, Math.min(1, b)), GAMMA) * 255;
        const o = idx * 4;
        if (blend || a < 1) {
          pixels[o] = sr * a + pixels[o]! * (1 - a); pixels[o + 1] = sg * a + pixels[o + 1]! * (1 - a); pixels[o + 2] = sb * a + pixels[o + 2]! * (1 - a); pixels[o + 3] = 255 * a + pixels[o + 3]! * (1 - a);
        } else {
          pixels[o] = sr; pixels[o + 1] = sg; pixels[o + 2] = sb; pixels[o + 3] = 255;
        }
      };

      if (prim.mode === "triangles") {
        for (let t = 0; t < prim.indexCount; t += 3) {
          pace.tick(8, (di + t / prim.indexCount) / frame.drawCount);
          const ia = scene.indices[prim.indexStart + t]! - prim.vertexStart, ib = scene.indices[prim.indexStart + t + 1]! - prim.vertexStart, ic = scene.indices[prim.indexStart + t + 2]! - prim.vertexStart;
          const ax = w[ia * 3]!, ay = w[ia * 3 + 1]!, az = w[ia * 3 + 2]!;
          const e1 = [w[ib * 3]! - ax, w[ib * 3 + 1]! - ay, w[ib * 3 + 2]! - az], e2 = [w[ic * 3]! - ax, w[ic * 3 + 1]! - ay, w[ic * 3 + 2]! - az];
          let nx = e1[1]! * e2[2]! - e1[2]! * e2[1]!, ny = e1[2]! * e2[0]! - e1[0]! * e2[2]!, nz = e1[0]! * e2[1]! - e1[1]! * e2[0]!;
          const nl = portable.hypot(nx, ny, nz);
          if (nl < 1e-20) continue;
          nx /= nl; ny /= nl; nz /= nl;
          const toEye = [frame.eye[0]! - ax, frame.eye[1]! - ay, frame.eye[2]! - az];
          const orthoView = Math.abs(frame.projection[15]! - 1) < 1e-6;
          const vx = orthoView ? frame.view[2]! : toEye[0]!, vy = orthoView ? frame.view[6]! : toEye[1]!, vz = orthoView ? frame.view[10]! : toEye[2]!;
          if (nx * vx + ny * vy + nz * vz < 0) {
            if (!doubleSided) continue;
            nx = -nx; ny = -ny; nz = -nz;
          }
          let shade = [1, 1, 1];
          if (!unlit) {
            const ndl = Math.max(0, nx * light.direction[0]! + ny * light.direction[1]! + nz * light.direction[2]!), h = ny * 0.5 + 0.5;
            const metal = m[mo + 8]!;
            const k = 1 - metal * 0.8;
            shade = [0, 1, 2].map((c) => k * (light.ground[c]! + (light.sky[c]! - light.ground[c]!) * h + light.color[c]! * ndl));
          }
          let poly: Clip[] = [clip[ia]!, clip[ib]!, clip[ic]!];
          const out: Clip[] = [];
          for (let i = 0; i < poly.length; i++) {
            const p = poly[i]!, q = poly[(i + 1) % poly.length]!;
            const dp = p.z + p.w, dq = q.z + q.w;
            if (dp >= 0) out.push(p);
            if ((dp >= 0) !== (dq >= 0)) {
              const s = dp / (dp - dq);
              out.push({ x: p.x + (q.x - p.x) * s, y: p.y + (q.y - p.y) * s, z: p.z + (q.z - p.z) * s, w: p.w + (q.w - p.w) * s, r: p.r + (q.r - p.r) * s, g: p.g + (q.g - p.g) * s, b: p.b + (q.b - p.b) * s, a: p.a + (q.a - p.a) * s });
            }
          }
          poly = out;
          if (poly.length < 3) continue;
          const s = poly.map(toScreen);
          for (let i = 1; i + 1 < s.length; i++) {
            const A = s[0]!, B = s[i]!, C = s[i + 1]!;
            const area = (B.x - A.x) * (C.y - A.y) - (C.x - A.x) * (B.y - A.y);
            if (Math.abs(area) < 1e-12) continue;
            const x0 = Math.max(0, Math.floor(Math.min(A.x, B.x, C.x))), x1 = Math.min(W - 1, Math.ceil(Math.max(A.x, B.x, C.x)));
            const y0 = Math.max(0, Math.floor(Math.min(A.y, B.y, C.y))), y1 = Math.min(H - 1, Math.ceil(Math.max(A.y, B.y, C.y)));
            for (let y = y0; y <= y1; y++) for (let x = x0; x <= x1; x++) {
              if (x === x0) pace.tick(x1 - x0 + 1);
              const px = x + 0.5, py = y + 0.5;
              const w0 = ((B.x - px) * (C.y - py) - (C.x - px) * (B.y - py)) / area;
              const w1 = ((C.x - px) * (A.y - py) - (A.x - px) * (C.y - py)) / area;
              const w2 = 1 - w0 - w1;
              if (w0 < 0 || w1 < 0 || w2 < 0) continue;
              const z = w0 * A.z + w1 * B.z + w2 * C.z;
              const idx = y * W + x;
              if (z < -1 || z > 1 || z >= depth[idx]!) continue;
              const a = w0 * A.a + w1 * B.a + w2 * C.a;
              if (blend && a <= 0) continue;
              if (!blend) depth[idx] = z;
              put(idx, (w0 * A.r + w1 * B.r + w2 * C.r) * shade[0]! + emissive[0]!, (w0 * A.g + w1 * B.g + w2 * C.g) * shade[1]! + emissive[1]!, (w0 * A.b + w1 * B.b + w2 * C.b) * shade[2]! + emissive[2]!, blend ? a : 1);
            }
          }
        }
      } else {
        for (let t = 0; t < prim.indexCount; t += 2) {
          pace.tick(8, (di + t / prim.indexCount) / frame.drawCount);
          let p = clip[scene.indices[prim.indexStart + t]! - prim.vertexStart]!, q = clip[scene.indices[prim.indexStart + t + 1]! - prim.vertexStart]!;
          const dp = p.z + p.w, dq = q.z + q.w;
          if (dp < 0 && dq < 0) continue;
          if (dp < 0 || dq < 0) {
            const s = dp / (dp - dq);
            const cut = { x: p.x + (q.x - p.x) * s, y: p.y + (q.y - p.y) * s, z: p.z + (q.z - p.z) * s, w: p.w + (q.w - p.w) * s, r: p.r + (q.r - p.r) * s, g: p.g + (q.g - p.g) * s, b: p.b + (q.b - p.b) * s, a: p.a + (q.a - p.a) * s };
            if (dp < 0) p = cut;
            else q = cut;
          }
          const A = toScreen(p), B = toScreen(q);
          const steps = Math.max(1, Math.ceil(Math.max(Math.abs(B.x - A.x), Math.abs(B.y - A.y))));
          for (let i = 0; i <= steps; i++) {
            if ((i & 63) === 0) pace.tick(64);
            const f = i / steps, x = Math.floor(A.x + (B.x - A.x) * f), y = Math.floor(A.y + (B.y - A.y) * f);
            if (x < 0 || y < 0 || x >= W || y >= H) continue;
            const z = A.z + (B.z - A.z) * f, idx = y * W + x;
            if (z < -1 || z > 1) continue;
            const here = depth[idx]!;
            let slope = 0;
            if (here !== Infinity) {
              for (const n of [idx - 1, idx + 1, idx - W, idx + W]) {
                const dn = depth[n];
                if (dn !== undefined && dn !== Infinity) slope = Math.max(slope, Math.abs(dn - here));
              }
            }
            if (z > here + LINE_BIAS + slope) continue;
            put(idx, A.r + (B.r - A.r) * f + emissive[0]!, A.g + (B.g - A.g) * f + emissive[1]!, A.b + (B.b - A.b) * f + emissive[2]!, A.a + (B.a - A.a) * f);
          }
        }
      }
    }
    pace.step(1);
    this.pixels.set(pixels);
    this.depth.set(depth);
  }
}
