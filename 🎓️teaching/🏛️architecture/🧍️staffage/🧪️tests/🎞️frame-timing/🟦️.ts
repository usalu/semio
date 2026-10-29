/** 🎞️ Frame schedule exactness and the encoded videos re-read with ffprobe/ffmpeg. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { frameSchedule } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { encodeClip, videoCapabilities, type VideoFrame } from "../../🔨️modules/📤️export/🎥️video/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { tool, withScratch } from "../🧰️support/🧰️.ts";

type Probe = { streams: { pix_fmt: string; r_frame_rate: string; nb_read_frames: string; tags?: Record<string, string>; codec_name: string; width: number; height: number }[]; format: { duration: string }; packets?: { pts_time: string }[] };
const probe = (file: string, extra: readonly string[] = []): Probe => JSON.parse(tool("ffprobe", ["-v", "error", ...extra, "-count_frames", "-show_streams", "-show_format", "-show_packets", "-of", "json", file])) as Probe;

const CLI = (format: string, out: string): string[] => ["--asset", "walking-commuter", "--clip", "walk-cycle", "--view", "dark-solid", "--scale", "100", "--fps", "12", "--format", format, "--out", out, "--lang", "en", "--px-per-mm", "4"];

function spacing(file: Probe): number[] {
  const times = file.packets!.map((packet) => Number(packet.pts_time)).sort((a, b) => a - b);
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

describe("🎬️ encoded clips", () => {
  it("MP4 from the CLI: 12 fps constant, scheduled frame count, yuv420p", async () => {
    await withScratch(async (out) => {
      const [file] = await runExport(parseArguments(CLI("mp4", out)));
      const expected = 14;
      const info = probe(file!);
      const video = info.streams[0]!;
      expect(video.codec_name).toBe("h264");
      expect(video.pix_fmt).toBe("yuv420p");
      expect(video.r_frame_rate).toBe("12/1");
      expect(Number(video.nb_read_frames)).toBe(expected);
      expect(Math.abs(Number(info.format.duration) - expected / 12)).toBeLessThanOrEqual(1 / 12);
      for (const gap of spacing(info)) expect(gap).toBeCloseTo(1 / 12, 4);
    });
  });

  it("WebM from the CLI: VP9 with the alpha tag, decoded as yuva420p", async () => {
    await withScratch(async (out) => {
      const [file] = await runExport(parseArguments(CLI("webm", out)));
      const info = probe(file!);
      const video = info.streams[0]!;
      expect(video.codec_name).toBe("vp9");
      expect(video.r_frame_rate).toBe("12/1");
      expect(video.tags?.alpha_mode ?? video.tags?.ALPHA_MODE).toBe("1");
      expect(Number(video.nb_read_frames)).toBe(14);
      const decoded = probe(file!, ["-c:v", "libvpx-vp9"]).streams[0]!;
      expect(decoded.pix_fmt).toBe("yuva420p");
      for (const gap of spacing(info)) expect(Math.abs(gap - 1 / 12)).toBeLessThan(0.001);
    });
  });

  it("first-party browser MP4 (H.264 I_PCM) decodes in ffmpeg with the exact frame count and rate", async () => {
    const width = 32, height = 32;
    const frame = (value: number): VideoFrame => { const data = new Uint8ClampedArray(width * height * 4); for (let at = 0; at < data.length; at += 4) data.set([value, 255 - value, 128, at % 8 === 0 ? 255 : 128], at); return { data, width, height }; };
    const source = async function* (): AsyncGenerator<VideoFrame> { for (const value of [10, 10, 80, 200, 200]) yield frame(value); };
    const clip = await encodeClip(source(), "mp4", { fps: 10, frameCount: 5 });
    expect(clip.alpha).toBe("none");
    expect(clip.frames).toBe(5);
    await withScratch((directory) => {
      const file = join(directory, "first-party.mp4");
      writeFileSync(file, clip.bytes);
      const info = probe(file);
      expect(info.streams[0]!.r_frame_rate).toBe("10/1");
      expect(Number(info.streams[0]!.nb_read_frames)).toBe(5);
      tool("ffmpeg", ["-v", "error", "-i", file, "-f", "null", "-"]);
      expect(readFileSync(file).length).toBe(clip.bytes.length);
    });
  });

  it("probes capabilities honestly: the first-party MP4 writer is always there, MediaRecorder codecs only when the host has them", () => {
    const caps = videoCapabilities();
    expect(caps.codecs.find((entry) => entry.engine === "first-party")?.available).toBe(true);
    expect(caps.codecs.filter((entry) => entry.engine === "MediaRecorder").every((entry) => entry.available === false)).toBe(true);
    expect(caps.codecs.filter((entry) => entry.format === "webm").every((entry) => entry.alpha === "unverified")).toBe(true);
  });
});
