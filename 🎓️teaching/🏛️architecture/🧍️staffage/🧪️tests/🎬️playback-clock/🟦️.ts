/** 🎬️ Cast time to frame: hand-authored vectors, the exporter schedule, and sequential versus seeked playback.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🎬️playback-clock/🔣️.json — the vectors, also read by the Python bridge test
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { playbackFrame, type Playback } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { frameSchedule } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { thinned } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";

type Vectors = {
  playback: (Playback & { name: string; samples: { t: number; frame: number }[] })[];
  schedules: { durationS: number; loop: boolean; fps: number; count: number; exportFps: number }[];
};
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎬️playback-clock/🔣️.json", import.meta.url), "utf8")) as Vectors;

describe("🎬️ playback clock", () => {
  it("maps the authored cast times to the authored frames", () => {
    for (const { name, samples, ...playback } of fixture.playback) for (const { t, frame } of samples) expect(playbackFrame(t, playback), `${name} at ${t}`).toBe(frame);
  });

  it("returns frame n for the exported frame time n / fps of every schedule", () => {
    for (const row of fixture.schedules) {
      const schedule = thinned(frameSchedule({ durationS: row.durationS, loop: row.loop }, row.fps));
      expect(schedule.count).toBe(row.count);
      expect(schedule.fps).toBeCloseTo(row.exportFps, 12);
      const playback = { fps: schedule.fps, count: schedule.count, durationS: schedule.durationS, loop: schedule.loop };
      schedule.times.forEach((time, n) => expect(playbackFrame(time, playback), `${row.durationS} s frame ${n}`).toBe(n));
      if (row.loop) expect(playbackFrame(row.durationS, playback)).toBe(0);
      else expect(playbackFrame(row.durationS * 3, playback)).toBe(row.count - 1);
    }
  });

  it("is a pure function: an accumulating clock and a seek straight to each time agree", () => {
    for (const { name, samples: _, ...playback } of fixture.playback) {
      const seeked = Array.from({ length: 400 }, (_, i) => playbackFrame(i / 24, playback));
      const stepped: number[] = [];
      for (let i = 0, clock = 0; i < 400; i += 1, clock += 1 / 24) stepped.push(playbackFrame(clock, playback));
      expect(stepped, name).toEqual(seeked);
    }
  });
});
