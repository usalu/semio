/** 🖋️SVG video export with caller-owned browser, encoder and temporary artifact ports. */
import { spawn } from "node:child_process";
import { mkdirSync, mkdtempSync, renameSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export interface SvgVideoRequestV1 {
  readonly inputSvgPath: string;
  readonly outputMp4Path: string;
  readonly artifactRoot: string;
  readonly width: number;
  readonly height: number;
  readonly fps: number;
  readonly durationSeconds: number;
}

export interface SvgVideoPageV1 {
  goto(url: string): Promise<unknown>;
  waitForSelector(selector: string): Promise<unknown>;
  setViewportSize(size: { width: number; height: number }): Promise<void>;
  evaluate<T, A = undefined>(operation: (argument: A) => T, argument?: A): Promise<T>;
  screenshot(options: { omitBackground: boolean }): Promise<Uint8Array>;
}

export interface SvgVideoBrowserV1 {
  newPage(): Promise<SvgVideoPageV1>;
  close(): Promise<void>;
}

export interface SvgVideoExportControlV1 {
  readonly openBrowser: () => Promise<SvgVideoBrowserV1>;
  readonly encoderExecutable: string;
  readonly signal?: AbortSignal;
  readonly progress?: (event: { completed: number; total: number }) => void;
}

/** 🛂️Admits the closed language-neutral request before constructing expensive resources. */
export function parseSvgVideoRequestV1(input: unknown): SvgVideoRequestV1 {
  const failures = validateJsonSchemaSubset(schema, input);
  if (failures.length) throw new Error("Invalid SVG video request: " + failures.join("; "));
  return Object.freeze({ ...input as SvgVideoRequestV1 });
}

/** 🎬️Publishes only a completed video and preserves any previous output on cancellation. */
export async function exportSvgVideoV1(input: SvgVideoRequestV1, control: SvgVideoExportControlV1): Promise<void> {
  const request = parseSvgVideoRequestV1(input);
  const inputSvgPath = request.inputSvgPath, outputMp4Path = request.outputMp4Path, options = control;
  if (resolve(inputSvgPath) === resolve(outputMp4Path)) throw new Error("SVG input and video output must have distinct identities");
  if (typeof control.openBrowser !== "function" || !control.encoderExecutable) throw new Error("SVG export requires explicit browser and encoder ports");
  const fps = request.fps;
  const duration = request.durationSeconds;
  const total = Math.ceil(fps * duration), controller = new AbortController();
  const output = resolve(outputMp4Path);
  let browser: SvgVideoBrowserV1 | undefined;
  let encoder: ReturnType<typeof spawn> | undefined;
  let encoded: Promise<Error | undefined> | undefined;
  let staging: string | undefined;
  let force: ReturnType<typeof setTimeout> | undefined;
  const terminate = (): void => {
    if (!encoder || encoder.exitCode !== null || encoder.signalCode !== null) return;
    encoder.kill("SIGTERM");
    force ??= setTimeout(() => encoder?.kill("SIGKILL"), 5000);
    force.unref();
  };
  const cancel = (): void => controller.abort(options.signal?.reason ?? new Error("SVG export cancelled"));
  const abort = (): void => { terminate(); void browser?.close().catch(() => {}); };
  controller.signal.addEventListener("abort", abort, { once: true });
  options.signal?.addEventListener("abort", cancel, { once: true });
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  try {
    if (options.signal?.aborted) cancel();
    controller.signal.throwIfAborted();
    browser = await control.openBrowser();
    controller.signal.throwIfAborted();
    const page = await browser.newPage();
    await page.goto(pathToFileURL(resolve(inputSvgPath)).href);
    await page.waitForSelector("svg");
    const width = Math.ceil(request.width / 2) * 2, height = Math.ceil(request.height / 2) * 2;
    await page.setViewportSize({ width, height });
    await page.evaluate(() => document.querySelector("svg")!.pauseAnimations());
    mkdirSync(dirname(output), { recursive: true });
    mkdirSync(request.artifactRoot, { recursive: true });
    staging = mkdtempSync(join(resolve(request.artifactRoot), "svg-video-stage-"));
    const artifact = join(staging, "animation.mp4");
    encoder = spawn(control.encoderExecutable, ["-nostdin", "-loglevel", "error", "-y", "-f", "image2pipe", "-vcodec", "png", "-r", String(fps), "-i", "-", "-c:v", "libx264", "-pix_fmt", "yuv420p", artifact], { stdio: ["pipe", "ignore", "inherit"] });
    encoded = new Promise((accept) => { encoder!.once("error", accept); encoder!.once("close", (code) => accept(code === 0 ? undefined : new Error(`ffmpeg exited with code ${code}`))); });
    encoder.stdin!.on("error", () => {});
    options.progress?.({ completed: 0, total });
    for (let frame = 0; frame < total; frame++) {
      controller.signal.throwIfAborted();
      await page.evaluate((time: number) => document.querySelector("svg")!.setCurrentTime(time), frame / fps);
      const buffer = await page.screenshot({ omitBackground: true });
      await new Promise<void>((accept, reject) => encoder!.stdin!.write(buffer, (error) => error ? reject(error) : accept()));
      options.progress?.({ completed: frame + 1, total });
    }
    encoder.stdin!.end();
    const error = await encoded;
    controller.signal.throwIfAborted();
    if (error) throw error;
    renameSync(artifact, output);
  } catch (error) {
    controller.signal.throwIfAborted();
    throw error;
  } finally {
    terminate();
    if (encoded) await encoded;
    if (force) clearTimeout(force);
    await browser?.close();
    if (staging) rmSync(staging, { recursive: true, force: true });
    options.signal?.removeEventListener("abort", cancel);
    controller.signal.removeEventListener("abort", abort);
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
