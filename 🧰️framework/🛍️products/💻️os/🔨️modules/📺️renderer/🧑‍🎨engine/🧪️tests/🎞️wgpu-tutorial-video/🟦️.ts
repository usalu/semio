import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import contract from "@semio-tech/framework-renderer-wgpu/media-slots/contract.json";
import mediaContract from "@semio-tech/plugin-window-kits/media/contract.json";
import shellFixture from "../🧫️fixtures/🧊️wgpu/🎬️media-slots/🎞️shell-tutorial-video/🔣️.json";
import { parseTutorialVideoPlan, splitShellMediaSlots, TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS, tutorialVideoLocalTimeMs, tutorialVideoNeedsReseek, tutorialVideoRect } from "../../🎬️media/🌐️browser/🎞️tutorial-video/🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎬️tutorial-presentation/🔣️.json"), "utf8"));

describe("wgpu tutorial video overlay", () => {
  it("places a cue by its normalized rect", () => {
    for (const row of fixture.videoRect) {
      const rect = tutorialVideoRect(row.normalized, { width: row.viewport[0], height: row.viewport[1] });
      expect([rect.x, rect.y, rect.width, rect.height]).toEqual(row.expect);
    }
  });

  it("reads the offset into the cue's own source", () => {
    for (const row of fixture.videoLocalTime) expect(tutorialVideoLocalTimeMs(row.playheadMs, row.cueAtMs, row.sourceOffsetMs)).toBe(row.expect);
  });

  it("reseeks only past the shared drift tolerance", () => {
    expect(TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS).toBe(300);
    for (const row of fixture.videoReseek) expect(tutorialVideoNeedsReseek(row.currentMs, row.targetMs)).toBe(row.expect);
  });

  it("carries a shell-owned slot the presented-slot schema and the page parser both admit", () => {
    const ajv = new Ajv2020({ strict: true }).addMetaSchema(draft7).addKeyword("x-semio-formats").addSchema(mediaContract);
    const validate = ajv.compile(contract);
    for (const row of shellFixture.cases) {
      if (!row.slot) continue;
      expect(validate([row.slot])).toBe(true);
      const { transport, tutorialVideo } = splitShellMediaSlots([row.slot]);
      expect(transport).toEqual([]);
      expect(tutorialVideo).toEqual({ src: row.input.src, rect: { x: row.input.rect[0], y: row.input.rect[1], width: row.input.rect[2], height: row.input.rect[3] }, muted: row.input.muted, playing: row.input.playing, rate: row.input.rate, localTimeMs: row.input.localTimeMs });
      expect(parseTutorialVideoPlan(tutorialVideo)).toEqual(tutorialVideo);
    }
  });

  it("refuses a forged owner, a duplicate shell slot and a plan without a source", () => {
    const slot = shellFixture.cases[0]!.slot!;
    expect(() => splitShellMediaSlots([{ ...slot, pluginId: "plugin.media" }])).toThrow("browser-media.shell-owner");
    expect(() => splitShellMediaSlots([slot, slot])).toThrow("browser-media.shell-duplicate");
    expect(() => parseTutorialVideoPlan({ src: "", rect: { x: 0, y: 0, width: 1, height: 1 }, muted: false, playing: true, rate: 1, localTimeMs: 0 })).toThrow("browser-media.shell-plan");
    expect(new Ajv2020({ strict: true }).addMetaSchema(draft7).addKeyword("x-semio-formats").addSchema(mediaContract).compile(contract)([{ ...slot, controllerId: "other" }])).toBe(false);
  });
});
