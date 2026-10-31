/** 📖️ Detail sections: size on paper, timeline and radio cards, source tags, provenance counts and honest export states.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import type { ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { Asset, Language } from "@teaching/staffage";
import walkerDocument from "../../../../🧫️fixtures/🚶️reference-walker/🔣️.json";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { DimensionsSection } from "../../🔨️modules/🔬️detail/📐️dimensions/🟦️.tsx";
import { ClipTimeline, PosesAndClips } from "../../🔨️modules/🔬️detail/🎞️clips/🟦️.tsx";
import { SourceTag } from "../../🔨️modules/🔬️detail/🧬️model/🟦️.tsx";
import { ProvenanceSection, referenceCounts } from "../../🔨️modules/🔬️detail/📚️provenance/🟦️.tsx";
import { ExportPanel } from "../../🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import type { ExportCapabilities, ExportResult } from "../../🔨️modules/📤️downloads/🟦️.ts";

const walker = walkerDocument as unknown as Asset;

const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
const row = { entry: { id: walker.id, category: "humans", title: walker.title, heightM: 1.73, widthM: 0.37, poses: [], clips: [], representations: { "500": "unavailable", "200": "authored", "100": "authored", "50": "authored" }, symbols: [], analytics: [], viewpoints: [], exports: [], distinction: walker.summary, completion: "authored" }, status: "ready", asset: walker } as const;

function wrap(node: ReactElement, language: Language = "en"): ReactElement {
  return <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>{node}</ArchiveProvider>;
}

afterEach(cleanup);

describe("📐️ dimensions", () => {
  it("reads the size on paper at the four presets: 1.73 m is 3.46, 8.65, 17.3, 34.6 mm", () => {
    render(wrap(<DimensionsSection asset={walker} row={row as never} />));
    const cell = (denominator: number): string => screen.getByText(`1:${denominator}`).closest("tr")!.querySelectorAll("td")[0]!.textContent!;
    expect([500, 200, 100, 50].map(cell)).toEqual(["3.46 mm", "8.65 mm", "17.3 mm", "34.6 mm"]);
  });
  it("uses the German decimal comma", () => {
    render(wrap(<DimensionsSection asset={walker} row={row as never} />, "de"));
    expect(screen.getByText("1:100").closest("tr")!.querySelectorAll("td")[0]!.textContent).toBe("17,3 mm");
  });
});

describe("🎞️ clips", () => {
  const clip = walker.clips[0]!;
  it("seeks to an event marker and moves one frame on its native slider", () => {
    const onSeek = vi.fn();
    render(wrap(<ClipTimeline asset={walker} clip={clip} timeS={0} onSeek={onSeek} />));
    fireEvent.click(document.querySelectorAll("[data-event]")[1]!);
    expect(onSeek).toHaveBeenLastCalledWith(clip.events[1]!.t);
    const slider = screen.getByRole("slider", { name: ENGLISH["detail.timeline.label"] }) as HTMLInputElement;
    expect(Number(slider.step)).toBeCloseTo(1 / clip.fps, 9);
    fireEvent.change(slider, { target: { value: String(1 / clip.fps) } });
    expect(onSeek).toHaveBeenLastCalledWith(1 / clip.fps);
  });
  it("marks the default clip checked without a selection and moves with arrow keys", () => {
    const onSelect = vi.fn();
    render(wrap(<PosesAndClips asset={walker} selection={null} onSelect={onSelect} />));
    const clips = within(screen.getByRole("radiogroup", { name: ENGLISH["detail.clips.clips"] }));
    expect(clips.getByRole("radio", { checked: true }).getAttribute("data-card")).toBe(walker.presentation.defaultClip);
    fireEvent.keyDown(clips.getByRole("radio", { checked: true }), { key: "ArrowRight" });
    expect(onSelect).toHaveBeenCalledWith({ kind: "clip", clip: walker.clips[1]!.id });
  });
});

describe("🏷️ source tags and provenance", () => {
  it("names every source in both languages with a citation tooltip", () => {
    for (const language of ["en", "de"] as const) {
      const t = translator(language);
      for (const source of ["measured", "literature", "authored-estimate", "illustrative"] as const) {
        const { container, unmount } = render(wrap(<SourceTag source={source} reference="winter-2009" asset={walker} />, language));
        expect(container.textContent).toContain(t(`detail.source.${source}`));
        expect(container.querySelector("[data-source]")!.getAttribute("title")).toContain("Winter");
        unmount();
      }
    }
    expect(GERMAN["detail.source.measured"]).not.toBe(ENGLISH["detail.source.measured"]);
  });
  it("counts the citations of every reference", () => {
    expect(referenceCounts(walker)).toEqual({ "winter-2009": 11, "ashrae-2021": 2 });
    render(wrap(<ProvenanceSection asset={walker} />));
    expect(screen.getByText("winter-2009").closest("tr")!.querySelectorAll("td")[1]!.textContent).toBe("11");
  });
});

describe("📤️ export panel", () => {
  const capabilities: ExportCapabilities = { svg: { available: true }, pdf: { available: true }, dxf: { available: true }, dwg: { available: false, reason: { en: "The DWG writer (Rust, WebAssembly) is not available in this build.", de: "Der DWG-Schreiber fehlt." } }, "png-sequence": { available: true }, mp4: { available: true, codec: "video/mp4" }, webm: { available: false, reason: { en: "This browser's MediaRecorder cannot produce WebM.", de: "Der MediaRecorder kann kein WebM." } } };
  const request = { view: "dark-line", scale: { denominator: 100 }, language: "en" } as const;
  const query = { kind: "clip", clip: "walk", timeS: 0 } as const;
  it("shows DWG as unavailable, WebM as unsupported and SVG as available", () => {
    render(wrap(<ExportPanel asset={walker} query={query} request={request} language="en" capabilities={capabilities} />));
    const state = (format: string): string | null => document.querySelector(`[data-format="${format}"]`)!.getAttribute("data-state");
    expect([state("dwg"), state("webm"), state("svg")]).toEqual(["unavailable", "unsupported", "available"]);
    expect(within(document.querySelector('[data-format="dwg"]') as HTMLElement).getByText(/Rust, WebAssembly/u)).toBeTruthy();
    expect((within(document.querySelector('[data-format="webm"]') as HTMLElement).getAllByRole("button")[0] as HTMLButtonElement).disabled).toBe(true);
  });
  it("runs with progress, saves the result and reports it", async () => {
    const save = vi.fn();
    const bytes = new Uint8Array(2048);
    const exporter = vi.fn(async (_a, _q, _o, format, control): Promise<ExportResult> => {
      control?.progress?.(1, 2, { en: "Half", de: "Halb" });
      return { status: "done", format, filename: "x.svg", mime: "image/svg+xml", bytes, blob: new Blob([bytes]), details: { metadataEmbedded: true, notes: [] } };
    });
    render(wrap(<ExportPanel asset={walker} query={query} request={request} language="en" capabilities={capabilities} exporter={exporter as never} save={save} />));
    fireEvent.click(within(document.querySelector('[data-format="svg"]') as HTMLElement).getByRole("button"));
    await waitFor(() => expect(save).toHaveBeenCalledTimes(1));
    expect(document.querySelector("[data-result]")!.textContent).toContain("x.svg");
    expect(document.querySelector("[data-result]")!.textContent).toContain("2.0 kB");
  });
  it("cancels a running export through its signal and blocks a second run meanwhile", async () => {
    const exporter = vi.fn((_a, _q, _o, format, control) => new Promise<ExportResult>((resolve) => control.signal.addEventListener("abort", () => resolve({ status: "cancelled", format }))));
    render(wrap(<ExportPanel asset={walker} query={query} request={request} language="en" capabilities={capabilities} exporter={exporter as never} />));
    fireEvent.click(within(document.querySelector('[data-format="svg"]') as HTMLElement).getByRole("button"));
    await screen.findByRole("progressbar", { hidden: true }).catch(() => undefined);
    expect((within(document.querySelector('[data-format="pdf"]') as HTMLElement).getByRole("button") as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(await screen.findByRole("button", { name: ENGLISH["action.cancel"] }));
    await waitFor(() => expect(document.querySelector("[data-result]")!.textContent).toContain(ENGLISH["detail.export.cancelled"]));
    expect(exporter).toHaveBeenCalledTimes(1);
  });
});
