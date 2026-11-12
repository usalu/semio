/** 🖥️ Browser-side GPU harness (bundled by the GPU test or by hand): renders the same frames with the WebGL2 backend and the software rasteriser and reports agreement, so a real GPU (or SwiftShader) can be compared with the reference. Exposes `window.semioGpu.run(document)`. */
import { ModelViewer, SoftwareBackend, boundsOf, cameraOf, createFrame, createWebGl2Backend, fitCamera, frameOf, sceneOf, type CameraPreset } from "../../../🟦️.ts";

type Row = { preset: string; variant: string | null; coverageAgreement: number; meanAbsDiff: number; glCovered: number; swCovered: number };

const BACKGROUND = [0.1, 0.2, 0.3, 1] as const;
const covered = (px: Uint8ClampedArray, i: number) => Math.abs(px[i]! - 26) + Math.abs(px[i + 1]! - 51) + Math.abs(px[i + 2]! - 77) > 24;

async function run(document: any, options: { width?: number; height?: number } = {}) {
  const W = options.width ?? 160, H = options.height ?? 128;
  const scene = sceneOf(document);
  const rows: Row[] = [];
  const canvas = window.document.createElement("canvas");
  canvas.style.cssText = "width:160px;height:128px;margin:4px";
  window.document.body.append(canvas);
  const status: string[] = [];
  const gl = createWebGl2Backend(canvas, (s) => status.push(s));
  gl.resize(W, H, 1);
  gl.load(scene);
  const sw = new SoftwareBackend(W, H);
  sw.load(scene);
  const frame = createFrame(scene);
  const rest = frameOf(scene, { pose: "bent", camera: cameraOf("front"), aspect: W / H }, createFrame(scene));
  const bounds = boundsOf(scene, rest);
  let last: Uint8ClampedArray = new Uint8ClampedArray(0);
  for (const preset of ["front", "elevation", "plan", "axonometric", "perspective"] as CameraPreset[]) for (const variant of [null, "dark"]) {
    const camera = fitCamera(cameraOf(preset), bounds, W / H, 1.1);
    frameOf(scene, { pose: "bent", variant, camera, aspect: W / H }, frame);
    gl.render(frame, { background: BACKGROUND });
    sw.render(frame, { background: BACKGROUND });
    const a = gl.readPixels(), b = sw.readPixels();
    let agree = 0, glc = 0, swc = 0, diff = 0, both = 0;
    for (let i = 0; i < W * H * 4; i += 4) {
      const g = covered(a, i), s = covered(b, i);
      if (g) glc++;
      if (s) swc++;
      if (g === s) agree++;
      if (g && s) {
        both++;
        diff += (Math.abs(a[i]! - b[i]!) + Math.abs(a[i + 1]! - b[i + 1]!) + Math.abs(a[i + 2]! - b[i + 2]!)) / 3;
      }
    }
    rows.push({ preset, variant, coverageAgreement: agree / (W * H), meanAbsDiff: both ? diff / both : 0, glCovered: glc, swCovered: swc });
    last = a;
  }
  const lose = (canvas.getContext("webgl2") as WebGL2RenderingContext).getExtension("WEBGL_lose_context");
  let recovered: boolean | null = null;
  if (lose) {
    const before = Array.from(last);
    lose.loseContext();
    await new Promise((r) => setTimeout(r, 50));
    lose.restoreContext();
    await new Promise((r) => setTimeout(r, 100));
    gl.render(frame, { background: BACKGROUND });
    recovered = Array.from(gl.readPixels()).every((v, i) => v === before[i]);
  }
  const info = (canvas.getContext("webgl2") as WebGL2RenderingContext).getExtension("WEBGL_debug_renderer_info");
  const renderer = info ? (canvas.getContext("webgl2") as WebGL2RenderingContext).getParameter(info.UNMASKED_RENDERER_WEBGL) : "unknown";
  return { renderer, status, rows, recovered, size: [W, H] };
}

function show(document: any, preset: CameraPreset, variant: string | null, W = 160, H = 128) {
  const scene = sceneOf(document);
  const canvas = window.document.createElement("canvas");
  const gl = createWebGl2Backend(canvas);
  gl.resize(W, H, 1);
  gl.load(scene);
  const sw = new SoftwareBackend(W, H);
  sw.load(scene);
  const rest = frameOf(scene, { pose: "bent", camera: cameraOf("front"), aspect: W / H }, createFrame(scene));
  const camera = fitCamera(cameraOf(preset), boundsOf(scene, rest), W / H, 1.1);
  const frame = frameOf(scene, { pose: "bent", variant, camera, aspect: W / H });
  gl.render(frame, { background: BACKGROUND });
  sw.render(frame, { background: BACKGROUND });
  const a = gl.readPixels(), b = sw.readPixels();
  const diff = new Uint8ClampedArray(W * H * 4);
  for (let i = 0; i < W * H * 4; i += 4) {
    diff[i] = Math.min(255, Math.abs(a[i]! - b[i]!) * 4);
    diff[i + 1] = Math.min(255, Math.abs(a[i + 1]! - b[i + 1]!) * 4);
    diff[i + 2] = Math.min(255, Math.abs(a[i + 2]! - b[i + 2]!) * 4);
    diff[i + 3] = 255;
  }
  window.document.body.replaceChildren();
  for (const px of [a, b, diff]) {
    const c = window.document.createElement("canvas");
    c.width = W;
    c.height = H;
    c.style.cssText = `width:${W * 2}px;height:${H * 2}px;margin:2px`;
    c.getContext("2d")!.putImageData(new ImageData(new Uint8ClampedArray(px), W, H), 0, 0);
    window.document.body.append(c);
  }
}

(window as unknown as { semioGpu: { run: typeof run; show: typeof show } }).semioGpu = { run, show };

(window as unknown as { semioViewer: { ModelViewer: typeof ModelViewer } }).semioViewer = { ModelViewer };
