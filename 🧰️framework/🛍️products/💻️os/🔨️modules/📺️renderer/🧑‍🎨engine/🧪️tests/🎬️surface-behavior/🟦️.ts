import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🎬️surface-behavior/🔣️.json";
import schema from "../../🧬️schema/🎬️surface-behavior/🔣️.json";

describe("app-backed surface behavior", () => {
  it("validates exact Draw and Note cancellation policies", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.cases.map(({ id }) => id)).toEqual(["draw-canvas2d", "note-ink-canvas"]);
    const [draw, note] = fixture.cases;
    expect(draw).toMatchObject({ appId: "s.draw.drawing@1/*#editor", surfaceKind: "canvas-2d", terminalPolicy: "cancelled-action-discards-draft", artifactAfterCancel: "unchanged" });
    expect(draw.publishedOnCancel).toEqual([{ action: "canvasPointerUp", phase: null, cancelled: true }]);
    expect(note).toMatchObject({ appId: "s.note.note@1/*#editor", surfaceKind: "ink-canvas", terminalPolicy: "retain-accepted-events", artifactAfterCancel: "retain-accepted-begin-live" });
    expect(note.acceptedBeforeCancel.map(({ phase }) => phase)).toEqual(["begin", "live"]);
    expect(note.publishedOnCancel).toEqual([]);
    expect(note.blockedAfterCancel).toEqual(["inkApplyEvents:live", "inkApplyEvents:commit"]);
  });
});
