/** 📏️ The span mode of the display range: the third option of the range control, its field, its labels in both languages, keyboard and the address.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { useState, type ReactElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import type { DisplayRange, Language } from "@teaching/staffage";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { depictionOptions } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { DEFAULT_VIEWER, viewerFromParams, viewerToParams } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { DEFAULT_SPAN_K, RangeControl, rangeOfMode } from "../../🔨️modules/🎛️viewer/📏️range/🟦️.tsx";

const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });

let latest: DisplayRange = { mode: "auto" };

function Harness(props: { readonly initial: DisplayRange; readonly language: Language }): ReactElement {
  const [range, setRange] = useState<DisplayRange>(props.initial);
  latest = range;
  return (
    <ArchiveProvider language={props.language} t={translator(props.language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
      <RangeControl range={range} legend={{ min: -4, max: 46 }} onChange={(next) => { latest = next; setRange(next); }} />
    </ArchiveProvider>
  );
}

const open = (initial: DisplayRange, language: Language = "en"): void => void render(<Harness initial={initial} language={language} />);

afterEach(cleanup);

describe("📏️ span option", () => {
  it("offers automatic, fixed and span as one labelled radio group", () => {
    open({ mode: "auto" });
    const group = screen.getByRole("radiogroup", { name: ENGLISH["viewer.range"] });
    expect(group).toBeTruthy();
    expect(screen.getAllByRole("radio").map((radio) => radio.textContent)).toEqual(["Automatic", "Fixed", "Span"]);
    expect(screen.getByRole("radio", { name: "Span" }).getAttribute("title")).toBe(ENGLISH["viewer.range.spanHelp"]);
  });

  it("starts the span at the course span and shows its field with the unit", () => {
    open({ mode: "auto" });
    fireEvent.click(screen.getByRole("radio", { name: "Span" }));
    expect(latest).toEqual({ mode: "span", spanK: DEFAULT_SPAN_K });
    expect(DEFAULT_SPAN_K).toBe(25);
    const field = screen.getByLabelText(ENGLISH["viewer.range.spanK"]) as HTMLInputElement;
    expect(field.value).toBe("25");
    expect(screen.getByText("K")).toBeTruthy();
    expect(screen.queryByLabelText("Minimum")).toBeNull();
  });

  it("edits the span and rejects non-positive and non-numeric entries", () => {
    open({ mode: "span", spanK: 25 });
    const field = screen.getByLabelText(ENGLISH["viewer.range.spanK"]) as HTMLInputElement;
    fireEvent.change(field, { target: { value: "15" } });
    expect(latest).toEqual({ mode: "span", spanK: 15 });
    fireEvent.change(field, { target: { value: "0" } });
    fireEvent.change(field, { target: { value: "-3" } });
    fireEvent.change(field, { target: { value: "abc" } });
    expect(latest).toEqual({ mode: "span", spanK: 15 });
    fireEvent.change(field, { target: { value: "7,5" } });
    expect(latest).toEqual({ mode: "span", spanK: 7.5 });
  });

  it("marks an invalid entry for assistive technology", () => {
    open({ mode: "span", spanK: 25 });
    const field = screen.getByLabelText(ENGLISH["viewer.range.spanK"]) as HTMLInputElement;
    fireEvent.change(field, { target: { value: "-3" } });
    expect(field.getAttribute("aria-invalid")).toBe("true");
  });

  it("moves between the modes with the arrow keys", () => {
    open({ mode: "fixed", min: 0, max: 10 });
    fireEvent.keyDown(screen.getByRole("radio", { name: "Fixed" }), { key: "ArrowRight" });
    expect(latest).toEqual({ mode: "span", spanK: DEFAULT_SPAN_K });
    fireEvent.keyDown(screen.getByRole("radio", { name: "Span" }), { key: "ArrowLeft" });
    expect(latest).toEqual({ mode: "fixed", min: -4, max: 46 });
    fireEvent.keyDown(screen.getByRole("radio", { name: "Fixed" }), { key: "ArrowLeft" });
    expect(latest).toEqual({ mode: "auto" });
  });

  it("keeps the edited span when span is chosen again and the fixed range likewise", () => {
    expect(rangeOfMode("span", undefined, { mode: "span", spanK: 9 })).toEqual({ mode: "span", spanK: 9 });
    expect(rangeOfMode("fixed", undefined, { mode: "fixed", min: 1, max: 2 })).toEqual({ mode: "fixed", min: 1, max: 2 });
    expect(rangeOfMode("fixed", undefined, { mode: "span", spanK: 9 })).toEqual({ mode: "fixed", min: 0, max: 1 });
    expect(rangeOfMode("auto", undefined, { mode: "span", spanK: 9 })).toEqual({ mode: "auto" });
  });

  it("is labelled in German too, and both bundles carry the same keys", () => {
    open({ mode: "span", spanK: 25 }, "de");
    expect(screen.getByRole("radiogroup", { name: "Anzeigebereich" })).toBeTruthy();
    expect(screen.getByRole("radio", { name: "Spanne" })).toBeTruthy();
    expect(screen.getByLabelText(GERMAN["viewer.range.spanK"])).toBeTruthy();
    expect(GERMAN["viewer.range.spanK"]).not.toBe(ENGLISH["viewer.range.spanK"]);
    for (const key of ["viewer.range.span", "viewer.range.spanHelp", "viewer.range.spanK"] as const) expect(GERMAN[key].length).toBeGreaterThan(0);
  });
});

describe("📏️ span in the address", () => {
  it("writes span and reads it back to the same viewer state", () => {
    const state = { ...DEFAULT_VIEWER, view: "thermal" as const, range: { mode: "span", spanK: 15 } as const };
    const params = viewerToParams(state);
    expect(params.get("span")).toBe("15");
    expect(params.get("range")).toBeNull();
    expect(viewerFromParams(new URLSearchParams(params.toString()))).toEqual(state);
  });

  it("prefers a fixed range and ignores a non-positive or unreadable span", () => {
    expect(viewerFromParams(new URLSearchParams("range=0:10&span=5")).range).toEqual({ mode: "fixed", min: 0, max: 10 });
    expect(viewerFromParams(new URLSearchParams("span=0")).range).toEqual({ mode: "auto" });
    expect(viewerFromParams(new URLSearchParams("span=-4")).range).toEqual({ mode: "auto" });
    expect(viewerFromParams(new URLSearchParams("span=x")).range).toEqual({ mode: "auto" });
  });

  it("passes the span to the depiction options exactly as the cast exporter writes it", () => {
    const range = viewerFromParams(new URLSearchParams("span=25")).range;
    const options = depictionOptions({ view: "thermal", viewpoint: "elevation", scale: { denominator: 50 }, language: "en", range });
    expect(options.range).toEqual({ mode: "span", spanK: 25 });
    expect(depictionOptions({ view: "thermal", viewpoint: "elevation", scale: { denominator: 50 }, language: "en" }).range).not.toEqual(options.range);
  });
});
