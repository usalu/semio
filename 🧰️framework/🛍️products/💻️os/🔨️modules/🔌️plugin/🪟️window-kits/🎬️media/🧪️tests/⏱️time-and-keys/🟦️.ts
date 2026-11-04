import { describe, expect, it } from "vitest";
import fixture from "./🧫️fixture.json";
import { formatMediaTime, mediaKeyCommand, mediaTimeValueText, mediaTransportLabels, parseMediaTransportProps, seekedMediaPosition, trimValueForSelection, type MediaKeyCommand } from "../../🟦️.ts";

describe("media transport time and keyboard vocabulary", () => {
  for (const row of fixture.time) {
    it(`formats ${row.ms} ms in both locales`, () => {
      expect(formatMediaTime(row.ms, "en")).toBe(row.en);
      expect(formatMediaTime(row.ms, "de")).toBe(row.de);
      expect(mediaTimeValueText(row.ms, "en", mediaTransportLabels("en"))).toBe(row.enText);
      expect(mediaTimeValueText(row.ms, "de", mediaTransportLabels("de"))).toBe(row.deText);
    });
  }

  it("agrees with the platform's own locale decimal mark (independent cross-check)", () => {
    for (const locale of ["en", "de"] as const) {
      const mark = new Intl.NumberFormat(locale, { minimumFractionDigits: 3 }).formatToParts(1.5).find((part) => part.type === "decimal")?.value;
      expect(formatMediaTime(1500, locale)).toBe(`0:01${mark}500`);
    }
  });

  for (const [index, row] of fixture.keys.entries()) {
    it(`maps key vector ${index} (${row.key} on ${row.target})`, () => {
      const command = mediaKeyCommand({ key: row.key, shiftKey: row.shift, ctrlKey: false, altKey: false, metaKey: false, targetTag: row.target, targetType: row.type }, row.duration);
      expect(command).toEqual(row.expect);
    });
  }

  it("never steals browser or assistive chords", () => {
    expect(mediaKeyCommand({ key: " ", shiftKey: false, ctrlKey: true, altKey: false, metaKey: false, targetTag: "DIV", targetType: null }, 1000)).toBeNull();
  });

  for (const [index, row] of fixture.seek.entries()) {
    it(`clamps seek vector ${index}`, () => {
      expect(seekedMediaPosition(row.command as Extract<MediaKeyCommand, { kind: "seek" | "seek-to" }>, row.position, row.duration)).toBe(row.expect);
    });
  }

  for (const row of fixture.plural) {
    it(`chooses singular and plural unit words for ${row.ms} ms from the label table`, () => {
      expect(mediaTimeValueText(row.ms, "en", mediaTransportLabels("en"))).toBe(row.enText);
      expect(mediaTimeValueText(row.ms, "de", mediaTransportLabels("de"))).toBe(row.deText);
    });
  }

  for (const [index, row] of fixture.trim.cases.entries()) {
    it(`maps selection vector ${index} to whole frames`, () => {
      expect(trimValueForSelection(fixture.trim.declared as Parameters<typeof trimValueForSelection>[0], row.startMs, row.endMs)).toBe(row.expect);
    });
  }

  it("keeps the contract parser exact over the new labels and trim", () => {
    const props = { schemaVersion: 1, kind: "audio", mediaType: "audio/mpeg", revision: "7", durationMs: 12000, positionMs: 0, selectionStartMs: null, selectionEndMs: null, locale: "en", labels: mediaTransportLabels("en"), resource: null, capability: { status: "unsupported", reason: null }, hostContentHeight: 88, trim: fixture.trim.declared };
    expect(parseMediaTransportProps(props).trim).toEqual(fixture.trim.declared);
    expect(() => parseMediaTransportProps({ ...props, trim: { ...fixture.trim.declared, unit: "byte" } })).toThrow();
    expect(() => parseMediaTransportProps({ ...props, trim: { ...fixture.trim.declared, extra: 1 } })).toThrow();
    const { trim: _omitted, ...withoutTrim } = props;
    expect(() => parseMediaTransportProps(withoutTrim)).toThrow();
  });
});
