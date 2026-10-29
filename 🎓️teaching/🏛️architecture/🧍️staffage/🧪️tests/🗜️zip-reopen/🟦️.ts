/** 🗜️ PNG-sequence ZIP reopened with unzip and python's zipfile. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { frameName, frameSchedule, writePngSequenceZip } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { python, STAFFAGE_ROOT, tool, withScratch } from "../🧰️support/🧰️.ts";

type Report = { bad: string | null; names: string[]; stored: boolean; manifest: { fps: number; frameCount: number; asset: string; view: string; scale: { denominator: number }; frames: { index: number; timeS: number; file: string }[] }; signatures: string[] };
const reopen = (file: string): Report => JSON.parse(python(`${STAFFAGE_ROOT}/🧪️tests/🗜️zip-reopen/🐍️.py`, [], [file])) as Report;

describe("🗜️ PNG sequence ZIP", () => {
  it("is a valid stored ZIP with manifest.json and every frame", async () => {
    await withScratch(async (out) => {
      const [file] = await runExport(parseArguments(["--asset", "walking-commuter", "--clip", "walk-cycle", "--view", "dark-line", "--scale", "100", "--fps", "12", "--format", "png-sequence", "--out", out, "--lang", "en", "--px-per-mm", "2"]));
      expect(tool("unzip", ["-t", file!])).toContain("No errors detected");
      const report = reopen(file!);
      expect(report.bad).toBeNull();
      expect(report.stored).toBe(true);
      expect(report.names).toEqual(["manifest.json", ...Array.from({ length: 14 }, (_, index) => frameName(index))]);
      expect(new Set(report.signatures)).toEqual(new Set(["89504e470d0a1a0a"]));
      expect(report.manifest).toMatchObject({ fps: 12, frameCount: 14, asset: "walking-commuter", view: "dark-line", scale: { denominator: 100 } });
      report.manifest.frames.forEach((frame, index) => { expect(frame.timeS).toBe(index / 12); expect(frame.file).toBe(frameName(index)); });
    });
  });

  it("refuses a frame source that disagrees with the schedule and honours cancellation", async () => {
    const schedule = frameSchedule({ durationS: 0.2, loop: false }, 10);
    const input = { asset: "a", clip: "c", view: "dark-line", scaleDenominator: 100, widthPx: 1, heightPx: 1, transparent: true } as const;
    const short = async function* (): AsyncGenerator<Uint8Array> { yield new Uint8Array([1]); };
    await expect(writePngSequenceZip(schedule, input, short())).rejects.toThrow(/schedule needs 3/u);
    const controller = new AbortController();
    controller.abort();
    await expect(writePngSequenceZip(schedule, input, short(), { signal: controller.signal })).rejects.toThrow();
    await withScratch((directory) => { const file = join(directory, "x.zip"); writeFileSync(file, new Uint8Array(0)); expect(readFileSync(file).length).toBe(0); });
  });
});
