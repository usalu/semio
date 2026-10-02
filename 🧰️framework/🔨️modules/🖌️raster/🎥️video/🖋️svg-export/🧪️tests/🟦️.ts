import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import corpus from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("the logo builder loads and resolves authored identities after every product is removed", async () => {
  const { build } = await import("esbuild");
  const framework = resolve(import.meta.dir, "../../../../..");
  const logo = resolve(framework, "🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts");
  const refused = resolve(framework, "🛍️products").replaceAll("\\", "/") + "/";
  const program = "import {logoKeyframePaths} from " + JSON.stringify(logo) + ";console.log(JSON.stringify(logoKeyframePaths(" + JSON.stringify("logo") + ").map(path=>path.replaceAll(String.fromCharCode(92),'/'))));";
  const built = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, external: ["playwright", "jsdom"], define: { "import.meta.url": JSON.stringify(pathToFileURL(logo).href), "import.meta.dir": JSON.stringify(resolve(logo, "..")) }, plugins: [{ name: "removed-logo-products", setup(builder) { builder.onLoad({ filter: /.*/ }, input => input.path.replaceAll("\\", "/").startsWith(refused) ? { errors: [{ text: "Logo builder loads a removed product: " + input.path }] } : undefined); } }] });
  const native = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(built.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  expect(native.exitCode, Buffer.from(native.stderr).toString()).toBe(0);
  const actual = JSON.parse(Buffer.from(native.stdout).toString());
  expect(actual).toEqual(corpus.logoPaths.map(path => "logo/" + path));
}, 15_000);

test("SVG video requests match independent Ajv for every portable vector", async () => {
  const { default: Ajv } = await import("ajv");
  const oracle = new Ajv({ strict: true }).compile(schema);
  const { parseSvgVideoRequestV1 } = await import("../🟦️.ts");
  for (const row of corpus.requests) {
    expect(oracle(row.value), row.id).toBe(row.accepted);
    if (row.accepted) expect(parseSvgVideoRequestV1(row.value), row.id).toEqual(row.value);
    else expect(() => parseSvgVideoRequestV1(row.value), row.id).toThrow();
  }
});

test("actual Chromium frames encode through FFmpeg and preserve output on cancellation", async () => {
  const root = process.env.SEMIO_TEST_ARTIFACTS_DIR;
  if (!root) throw new Error("SVG video laws require caller-owned SEMIO_TEST_ARTIFACTS_DIR");
  mkdirSync(root, { recursive: true });
  const sandbox = mkdtempSync(join(root, "svg-video-"));
  try {
    const { exportSvgVideoV1 } = await import("../🟦️.ts");
    const { createChromiumSvgVideoRuntimeV1 } = await import("../🌐️browser/🟦️.ts");
    const runtime = await createChromiumSvgVideoRuntimeV1();
    try {
    const input = join(sandbox, "input.svg"), output = join(sandbox, "output.mp4"), staging = join(sandbox, "staging");
    writeFileSync(input, corpus.native.svg);
    const request = { inputSvgPath: input, outputMp4Path: output, artifactRoot: staging, width: corpus.native.width, height: corpus.native.height, fps: corpus.native.fps, durationSeconds: corpus.native.durationSeconds };
    const progress: number[] = [];
    await exportSvgVideoV1(request, { openBrowser: runtime.openBrowser, encoderExecutable: "ffmpeg", progress: event => progress.push(event.completed) });
    const probe = Bun.spawnSync(["ffprobe", "-v", "error", "-select_streams", "v:0", "-count_frames", "-show_entries", "stream=width,height,nb_read_frames", "-of", "json", output], { stdout: "pipe", stderr: "pipe" });
    expect(probe.exitCode, Buffer.from(probe.stderr).toString()).toBe(0);
    const observed = JSON.parse(Buffer.from(probe.stdout).toString()).streams[0];
    expect(observed).toEqual({ width: corpus.native.width, height: corpus.native.height, nb_read_frames: String(corpus.native.frames) });
    expect(progress).toEqual([0, 1, 2]);
    const decode = Bun.spawnSync(["ffmpeg", "-v", "error", "-i", output, "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgba", "-"], { stdout: "pipe", stderr: "pipe" });
    expect(decode.exitCode, Buffer.from(decode.stderr).toString()).toBe(0);
    const rgba = new Uint8Array(decode.stdout);
    expect(rgba.length).toBe(corpus.native.width * corpus.native.height * 4);
    expect(rgba[0]).toBeLessThanOrEqual(3);
    expect(rgba[1]).toBeGreaterThanOrEqual(252);
    expect(rgba[2]).toBeLessThanOrEqual(3);
    expect(rgba[3]).toBe(255);
    const completed = readFileSync(output);
    const controller = new AbortController();
    await expect(exportSvgVideoV1(request, { openBrowser: runtime.openBrowser, encoderExecutable: "ffmpeg", signal: controller.signal, progress: event => { if (event.completed === 1) controller.abort(new Error("portable cancellation")); } })).rejects.toThrow("portable cancellation");
    expect(readFileSync(output)).toEqual(completed);
    expect(readdirSync(staging)).toEqual([]);
    } finally {
      await runtime.close();
    }
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);
