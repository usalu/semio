/** 🎞️ Frame schedule exactness and the encoded videos re-read with PyAV (the ffprobe facts) and the pinned ffmpeg of the imageio-ffmpeg wheel. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { frameSchedule } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { encodeClip, videoCapabilities, type VideoFrame } from "../../🔨️modules/📤️export/🎥️video/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { ToolMissingError, repoToolPort, type ToolPort } from "../../🔨️modules/📤️export/🔌️tools/🟦️.ts";
import { requireTool, withScratch } from "../🧰️support/🧰️.ts";
import { ffmpegBytes, ffmpegExecutable, oracleToolPort, probeVideo, type VideoReport } from "../🧰️support/🐍️.ts";

const CLI = (format: string, out: string): string[] => ["--asset", "walking-commuter", "--clip", "walk-cycle", "--view", "dark-solid", "--viewpoint", "elevation", "--scale", "50", "--fps", "12", "--format", format, "--out", out, "--lang", "en", "--px-per-mm", "4"];

function spacing(file: VideoReport): number[] {
  const times = [...file.packetTimesS].sort((a, b) => a - b);
  return times.slice(1).map((time, index) => time - times[index]!);
}

describe("🎞️ frame schedule", () => {
  it("counts round(d·fps) for loops and one more for non-loops, at exactly n / fps", () => {
    const looping = frameSchedule({ durationS: 1.2, loop: true }, 25);
    const once = frameSchedule({ durationS: 1.2, loop: false }, 25);
    expect([looping.count, once.count]).toEqual([30, 31]);
    for (let n = 0; n < once.count; n += 1) expect(once.times[n]).toBe(n / 25);
    expect(looping.times.at(-1)).toBe(29 / 25);
    expect(frameSchedule({ durationS: 0.1, loop: true }, 15).count).toBe(2);
    expect(() => frameSchedule({ durationS: 1, loop: true }, 0)).toThrow();
    expect(() => frameSchedule({ durationS: 0, loop: true }, 25)).toThrow();
  });
});

requireTool("uv");
ffmpegExecutable();

describe("🎬️ encoded clips", () => {
  it("MP4 from the CLI: 12 fps constant, scheduled frame count, yuv420p", async () => {
    await withScratch(async (out) => {
      const [file] = await runExport(parseArguments(CLI("mp4", out)), oracleToolPort);
      const expected = 14;
      const video = probeVideo(file!);
      expect(video.codec).toBe("h264");
      expect(video.pixelFormat).toBe("yuv420p");
      expect(video.rFrameRate).toBe("12/1");
      expect(video.framesRead).toBe(expected);
      expect(Math.abs(video.durationS - expected / 12)).toBeLessThanOrEqual(1 / 12);
      for (const gap of spacing(video)) expect(gap).toBeCloseTo(1 / 12, 4);
    });
  });

  it("WebM from the CLI: VP9 with the alpha tag and independently decoded transparent RGBA", async () => {
    await withScratch(async (out) => {
      const [file] = await runExport(parseArguments(CLI("webm", out)), oracleToolPort);
      const video = probeVideo(file!);
      expect(video.codec).toBe("vp9");
      expect(video.rFrameRate).toBe("12/1");
      expect(video.tags.alpha_mode ?? video.tags.ALPHA_MODE).toBe("1");
      expect(video.framesRead).toBe(14);
      const decoded = ffmpegBytes(["-v", "error", "-c:v", "libvpx-vp9", "-i", file!, "-frames:v", "1", "-f", "rawvideo", "-pix_fmt", "rgba", "pipe:1"]);
      expect(decoded.length).toBe(video.width * video.height * 4);
      const alpha = decoded.filter((_, index) => index % 4 === 3);
      expect(alpha.some(value => value === 0)).toBe(true);
      expect(alpha.some(value => value === 255)).toBe(true);
      for (const gap of spacing(video)) expect(Math.abs(gap - 1 / 12)).toBeLessThan(0.001);
    });
  });

  it("first-party browser MP4 (H.264 I_PCM) decodes in ffmpeg with the exact frame count and rate", async () => {
    const width = 32, height = 32;
    const frame = (value: number): VideoFrame => { const data = new Uint8ClampedArray(width * height * 4); for (let at = 0; at < data.length; at += 4) data.set([value, 255 - value, 128, at % 8 === 0 ? 255 : 128], at); return { data, width, height }; };
    const source = async function* (): AsyncGenerator<VideoFrame> { for (const value of [10, 10, 80, 200, 200]) yield frame(value); };
    const clip = await encodeClip(source(), "mp4", { viewpoint: "plan", fps: 10, frameCount: 5 });
    expect(clip.alpha).toBe("none");
    expect(clip.frames).toBe(5);
    await withScratch((directory) => {
      const file = join(directory, "first-party.mp4");
      writeFileSync(file, clip.bytes);
      const video = probeVideo(file);
      expect(video.rFrameRate).toBe("10/1");
      expect(video.framesRead).toBe(5);
      ffmpegBytes(["-v", "error", "-i", file, "-f", "null", "-"]);
      expect(readFileSync(file).length).toBe(clip.bytes.length);
    });
  });
});

describe("🎬️ missing tools", () => {
  const missing = (tool: string): ToolPort => ({ run: async (name, args, signal) => (name === tool ? Promise.reject(new ToolMissingError(tool)) : oracleToolPort.run(name, args, signal)) });

  it("names ffmpeg and stops the video export with a bilingual reason", async () => {
    await withScratch(async (out) => {
      const failure = await runExport(parseArguments(CLI("mp4", out)), missing("ffmpeg")).then(() => null, (error: unknown) => error);
      expect(failure).toBeInstanceOf(ToolMissingError);
      const { tool, reason } = failure as ToolMissingError;
      expect(tool).toBe("ffmpeg");
      expect(reason.en).toMatch(/"ffmpeg" is not available/u);
      expect(reason.de).toMatch(/"ffmpeg" ist nicht verfügbar/u);
    });
  });

  it("names rsvg-convert and stops the raster export with a bilingual reason", async () => {
    await withScratch(async (out) => {
      const failure = await runExport(parseArguments(CLI("mp4", out)), missing("rsvg-convert")).then(() => null, (error: unknown) => error);
      expect(failure).toBeInstanceOf(ToolMissingError);
      expect((failure as ToolMissingError).reason.de).toMatch(/librsvg/u);
    });
  });

  it("turns a program that is not on the PATH into the same named error and keeps cancellation of a running one", async () => {
    await expect(repoToolPort.run("staffage-no-such-tool-xyz", [])).rejects.toBeInstanceOf(ToolMissingError);
    const controller = new AbortController();
    const running = repoToolPort.run(process.execPath, ["-e", "setTimeout(() => {}, 60000)"], controller.signal);
    setTimeout(() => controller.abort(), 200);
    await expect(running).rejects.toMatchObject({ name: "AbortError" });
  });
});

describe("🎬️ video capabilities", () => {
  it("probes capabilities honestly: the first-party MP4 writer is always there, MediaRecorder codecs only when the host has them", () => {
    const caps = videoCapabilities();
    expect(caps.codecs.find((entry) => entry.engine === "first-party")?.available).toBe(true);
    expect(caps.codecs.filter((entry) => entry.engine === "MediaRecorder").every((entry) => entry.available === false)).toBe(true);
    expect(caps.codecs.filter((entry) => entry.format === "webm").every((entry) => entry.alpha === "unverified")).toBe(true);
  });
});
