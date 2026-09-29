// #region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { AvcPcmEncoder, bt601Cb, bt601Cr, bt601Luma, encodeVideoRuns } from "../../🟦️.ts";
// #endregion 🔌️Adapters

/** 🎞️ Differential adapter of `🥒️.feature`: the SUBJECT states what the first-party encoder claims to have written for every
 * fixture case (stream facts, key frames, the digest of every frame's exact `yuv420p` planes); the ORACLE encodes the same
 * case, lets FFmpeg's `ffprobe` and `ffmpeg` read and decode it, and states what FFmpeg saw. `ordered-json-v1` compares
 * the two projections; nothing here tolerates a single differing sample. */

// #region 🧫️Cases
type Rgb = readonly [number, number, number];
type Run = Readonly<{ rgb?: Rgb; quadrants?: readonly Rgb[]; frames: number }>;
type VideoCase = Readonly<{ id: string; width: number; height: number; fps: number; runs: readonly Run[]; expected: Readonly<{ frameCount: number; syncSamples: readonly number[]; mp4Sha256: string }> }>;

function cases(ctx: AdapterContext): readonly VideoCase[] {
  return (JSON.parse(readFileSync(ctx.fixture("shared://🔣️.json"), "utf8")) as { cases: readonly VideoCase[] }).cases;
}

function colourAt(run: Run, width: number, height: number, x: number, y: number): Rgb {
  return run.quadrants ? run.quadrants[(y < height / 2 ? 0 : 2) + (x < width / 2 ? 0 : 1)]! : run.rgb!;
}

function rgbaOf(testCase: VideoCase, run: Run): Uint8Array {
  const rgba = new Uint8Array(testCase.width * testCase.height * 4);
  for (let y = 0; y < testCase.height; y += 1) for (let x = 0; x < testCase.width; x += 1) rgba.set([...colourAt(run, testCase.width, testCase.height, x, y), 255], (y * testCase.width + x) * 4);
  return rgba;
}

/** 🎨️ The exact `yuv420p` frame the encoder claims for `run`: BT.601 luma per pixel, chroma the rounded mean of each 2×2
 * block (edge pixels repeated past an odd border), planes Y, U, V. */
function claimedFrame(testCase: VideoCase, run: Run): Uint8Array {
  const { width, height } = testCase;
  const chromaWidth = Math.ceil(width / 2);
  const chromaHeight = Math.ceil(height / 2);
  const frame = new Uint8Array(width * height + 2 * chromaWidth * chromaHeight);
  const pixel = (x: number, y: number): Rgb => colourAt(run, width, height, Math.min(x, width - 1), Math.min(y, height - 1));
  for (let y = 0; y < height; y += 1) for (let x = 0; x < width; x += 1) frame[y * width + x] = bt601Luma(...pixel(x, y));
  for (let y = 0; y < chromaHeight; y += 1) {
    for (let x = 0; x < chromaWidth; x += 1) {
      let cb = 0;
      let cr = 0;
      for (const [dx, dy] of [[0, 0], [1, 0], [0, 1], [1, 1]] as const) {
        cb += bt601Cb(...pixel(x * 2 + dx, y * 2 + dy));
        cr += bt601Cr(...pixel(x * 2 + dx, y * 2 + dy));
      }
      frame[width * height + y * chromaWidth + x] = (cb + 2) >> 2;
      frame[width * height + chromaWidth * chromaHeight + y * chromaWidth + x] = (cr + 2) >> 2;
    }
  }
  return frame;
}

const sha256 = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");

function encoded(testCase: VideoCase): Uint8Array {
  const encoder = new AvcPcmEncoder({ width: testCase.width, height: testCase.height, fps: testCase.fps });
  return encodeVideoRuns(encoder, testCase.runs.map((run) => ({ rgba: rgbaOf(testCase, run), frames: run.frames })), () => undefined);
}
// #endregion 🧫️Cases

// #region 🧭️Adapter
/** 🧾️ What the encoder claims, case by case. */
function subject(ctx: AdapterContext): AdapterOutcome {
  return {
    projection: cases(ctx).map((testCase) => ({
      id: testCase.id,
      codec: "h264",
      profile: "Constrained Baseline",
      width: testCase.width,
      height: testCase.height,
      pixFmt: "yuv420p",
      frameRate: `${testCase.fps}/1`,
      frames: testCase.expected.frameCount,
      keyFrames: testCase.expected.syncSamples,
      mp4Sha256: testCase.expected.mp4Sha256,
      frameDigests: testCase.runs.flatMap((run) => Array<string>(run.frames).fill(sha256(claimedFrame(testCase, run)))),
    })),
  };
}

function run(command: string, args: readonly string[]): Uint8Array {
  const result = spawnSync(command, args, { maxBuffer: 1 << 30 });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${command} ${args.join(" ")} exited ${result.status}: ${result.stderr.toString("utf8").trim()}`);
  return result.stdout;
}

/** 🔮️ What FFmpeg reads and decodes out of the encoder's own bytes, case by case. */
function oracle(ctx: AdapterContext): AdapterOutcome {
  return {
    projection: cases(ctx).map((testCase) => {
      const mp4 = encoded(testCase);
      const file = join(ctx.workDir, `${testCase.id}.mp4`);
      writeFileSync(file, mp4);
      const stream = JSON.parse(new TextDecoder().decode(run("ffprobe", ["-v", "error", "-count_frames", "-select_streams", "v:0", "-show_entries", "stream=codec_name,profile,width,height,pix_fmt,r_frame_rate,nb_read_frames", "-of", "json", file]))).streams[0];
      const frames = JSON.parse(new TextDecoder().decode(run("ffprobe", ["-v", "error", "-select_streams", "v:0", "-show_entries", "frame=key_frame", "-of", "json", file]))).frames as { key_frame: number }[];
      const raw = run("ffmpeg", ["-v", "error", "-i", file, "-f", "rawvideo", "-pix_fmt", "yuv420p", "-"]);
      const frameBytes = testCase.width * testCase.height + 2 * Math.ceil(testCase.width / 2) * Math.ceil(testCase.height / 2);
      if (raw.length % frameBytes !== 0) throw new Error(`${testCase.id}: ffmpeg decoded ${raw.length} bytes, not whole ${testCase.width}x${testCase.height} yuv420p frames`);
      return {
        id: testCase.id,
        codec: stream.codec_name,
        profile: stream.profile,
        width: stream.width,
        height: stream.height,
        pixFmt: stream.pix_fmt,
        frameRate: stream.r_frame_rate,
        frames: Number(stream.nb_read_frames),
        keyFrames: frames.flatMap((frame, index) => (frame.key_frame === 1 ? [index + 1] : [])),
        mp4Sha256: sha256(mp4),
        frameDigests: Array.from({ length: raw.length / frameBytes }, (_, index) => sha256(raw.subarray(index * frameBytes, (index + 1) * frameBytes))),
      };
    }),
  };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: { "decodes-every-fixture-stream": { subject, oracle } },
});
// #endregion 🧭️Adapter
