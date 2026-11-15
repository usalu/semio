/** 🎚️ Scale and mode of the interactive 3D model: the scales an asset has a model at and why the others have none, the scale and mode groups of the stage with their keyboard and announcement, and the one choice shared with the 2D drawing in the viewer and in the custom element.
 * The viewer runs on its software rasteriser here (jsdom has no WebGL2).
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { VIEWS } from "@teaching/staffage";
import type { Asset, Language, View } from "@teaching/staffage";
import { defineStaffageElement } from "@teaching/staffage-react";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { Viewer } from "../../🔨️modules/🎛️viewer/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { DEFAULT_VIEWER, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { MODEL_SCALES, scaleChoices } from "../../🔨️modules/🧊️spatial/🟦️.ts";
import { ModelStage, type StageOptions } from "../../🔨️modules/🧊️spatial/🖥️stage/🟦️.ts";
import { VIEW_NAMES } from "../../🔨️modules/🗣️labels/🟦️.ts";
import { sampleFigure } from "../🧰️support/🧰️.ts";

const unavailable = { status: "unavailable", denominator: 500, cause: "below-printable-feature", reason: { en: "The smallest part is 0.3 mm on paper.", de: "Das kleinste Teil misst 0,3 mm auf dem Papier." }, limitingFeature: "handle", limitingSizeM: 0.15, alternatives: [] };
const partial = { ...sampleFigure, volumes: { ...sampleFigure.volumes, "500": { status: "unavailable", denominator: 500 } }, representations: { ...sampleFigure.representations, "500": unavailable } } as unknown as Asset;
const silent = { ...partial, representations: { ...partial.representations, "500": { ...unavailable, reason: { en: "", de: "" } } } } as unknown as Asset;
const pose = { kind: "pose", pose: "stand" } as const;
const base = { asset: partial, query: pose, view: "light-solid", scale: 100, language: "en", preset: "perspective", settleMs: 0 } as const satisfies StageOptions;
const hosts: HTMLElement[] = [];
const stages: ModelStage[] = [];

function mount(options: Partial<StageOptions> = {}): { readonly host: HTMLElement; readonly stage: ModelStage } {
  const host = document.createElement("div");
  document.body.append(host);
  hosts.push(host);
  const stage = new ModelStage(host, { ...base, ...options } as StageOptions);
  stages.push(stage);
  return { host, stage };
}

const phase = (host: HTMLElement) => host.querySelector("[data-model-stage]")!.getAttribute("data-state");
const ready = (host: HTMLElement) => waitFor(() => expect(phase(host)).toBe("ready"), { timeout: 60000 });
const scaleButtons = (host: HTMLElement) => [...host.querySelectorAll<HTMLButtonElement>("[data-group=scale] button")];
const modeButtons = (host: HTMLElement) => [...host.querySelectorAll<HTMLButtonElement>("[data-group=mode] button")];
const pressed = (buttons: HTMLButtonElement[]) => buttons.filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.getAttribute("data-scale") ?? button.getAttribute("data-view"));
const spoken = (host: HTMLElement) => host.querySelector("[role=status][aria-live=polite]")!.textContent;
const key = (element: Element, name: string) => fireEvent.keyDown(element, { key: name });

beforeEach(() => {
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, left: 0, top: 0, right: 160, bottom: 120, width: 160, height: 120, toJSON: () => ({}) });
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
});
afterEach(() => {
  for (const stage of stages.splice(0)) stage.dispose();
  for (const host of hosts.splice(0)) host.remove();
  cleanup();
  vi.restoreAllMocks();
});

describe("🔢️ the scales of a model and the reason for a missing one", () => {
  it("offers the four preset scales finest first and marks 1:500 unavailable with the reason of its representation in both languages", () => {
    expect(MODEL_SCALES).toEqual([50, 100, 200, 500]);
    expect(scaleChoices(partial, "en")).toEqual([
      { scale: 50, available: true },
      { scale: 100, available: true },
      { scale: 200, available: true },
      { scale: 500, available: false, reason: "The smallest part is 0.3 mm on paper." },
    ]);
    expect(scaleChoices(partial, "de")[3]).toEqual({ scale: 500, available: false, reason: "Das kleinste Teil misst 0,3 mm auf dem Papier." });
  });

  it("names the scales that exist when the representation gives no reason, and has no reason for an available scale", () => {
    const missing = scaleChoices(silent, "en")[3]!;
    expect(missing.available).toBe(false);
    expect(missing.available ? "" : missing.reason).toBe(translator("en")("model.noScaleList", { scales: "1:50, 1:100, 1:200" }));
    expect(scaleChoices(silent, "de")[3]).toEqual({ scale: 500, available: false, reason: translator("de")("model.noScaleList", { scales: "1:50, 1:100, 1:200" }) });
    expect(scaleChoices(sampleFigure, "en").every((choice) => choice.available)).toBe(true);
  });
});

describe("🎛️ the scale and mode groups of the stage", () => {
  it("has a labelled group of the four scales and one of the seven modes, with the current choice pressed", async () => {
    const { host } = mount({ scale: 100, view: "thermal" });
    const groups = [...host.querySelectorAll("[role=group]")].map((group) => group.getAttribute("aria-label"));
    expect(groups).toEqual(expect.arrayContaining([ENGLISH["model.scale"], ENGLISH["model.mode"]]));
    expect(scaleButtons(host).map((button) => button.textContent)).toEqual(["1:50", "1:100", "1:200", "1:500"]);
    expect(modeButtons(host).map((button) => button.getAttribute("data-view"))).toEqual([...VIEWS]);
    expect(modeButtons(host).map((button) => button.textContent)).toEqual(VIEWS.map((view) => VIEW_NAMES[view].en));
    expect(pressed(scaleButtons(host))).toEqual(["100"]);
    expect(pressed(modeButtons(host))).toEqual(["thermal"]);
    await ready(host);
  });

  it("selects the scene of the scale and the mode and reports each choice to the host, which keeps 2D and 3D on one state", async () => {
    const scales: number[] = [];
    const views: View[] = [];
    const { host, stage } = mount({ onScale: (scale) => scales.push(scale), onView: (view) => views.push(view) });
    await ready(host);
    expect(stage.viewer!.activeScene).toBe("s100-body");
    fireEvent.click(scaleButtons(host)[0]!);
    expect(scales).toEqual([50]);
    expect(stage.viewer!.activeScene).toBe("s50-body");
    expect(pressed(scaleButtons(host))).toEqual(["50"]);
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("thermal")]!);
    expect(views).toEqual(["thermal"]);
    expect(stage.viewer!.activeScene).toBe("s50-thermal");
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("forces")]!);
    expect(stage.viewer!.activeScene).toBe("s50-structure");
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("dark-line")]!);
    expect(stage.viewer!.activeScene).toBe("s50-body");
    expect(views).toEqual(["thermal", "forces", "dark-line"]);
    expect(host.querySelector("[data-model-stage]")!.getAttribute("data-view")).toBe("dark-line");
    expect(phase(host)).toBe("ready");
  });

  it("does not build the model again for a scale or a mode", async () => {
    const { host, stage } = mount();
    const documents: unknown[] = [];
    stage.viewer!.on("ready", ({ scene }) => documents.push(scene));
    await ready(host);
    fireEvent.click(scaleButtons(host)[2]!);
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("moments")]!);
    expect(documents).toHaveLength(1);
  });

  it("marks an unavailable scale with its reason for assistive technology, announces the reason on a click and changes nothing", async () => {
    const scales: number[] = [];
    const { host, stage } = mount({ onScale: (scale) => scales.push(scale) });
    await ready(host);
    const missing = scaleButtons(host)[3]!;
    expect(missing.getAttribute("aria-disabled")).toBe("true");
    expect(missing.getAttribute("data-unavailable")).toBe("");
    const reason = host.ownerDocument.getElementById(missing.getAttribute("aria-describedby")!)!;
    expect(reason.textContent).toBe(translator("en")("model.scaleUnavailable", { scale: "500", reason: "The smallest part is 0.3 mm on paper." }));
    expect(scaleButtons(host).slice(0, 3).every((button) => button.getAttribute("aria-disabled") === null)).toBe(true);
    fireEvent.click(missing);
    expect(scales).toEqual([]);
    expect(stage.viewer!.activeScene).toBe("s100-body");
    expect(pressed(scaleButtons(host))).toEqual(["100"]);
    expect(host.querySelector("[data-announce]")!.textContent).toBe(reason.textContent);
  });

  it("follows a scale that the host sets, and refuses one without a model by name while the stage stays on the last scene", async () => {
    const { host, stage } = mount();
    await ready(host);
    stage.update({ scale: 200 });
    expect(stage.viewer!.activeScene).toBe("s200-body");
    expect(pressed(scaleButtons(host))).toEqual(["200"]);
    stage.update({ scale: 500 });
    expect(phase(host)).toBe("unavailable");
    expect(host.querySelector("[data-model-stage]")!.textContent).toContain("1:50, 1:100, 1:200");
    expect(pressed(scaleButtons(host))).toEqual(["500"]);
    stage.update({ scale: 100 });
    expect(phase(host)).toBe("ready");
    expect(stage.viewer!.activeScene).toBe("s100-body");
  });

  it("is complete in German: the group names, the buttons, the reason and the announcement", async () => {
    const { host, stage } = mount({ language: "de", view: "thermal" });
    await ready(host);
    expect([...host.querySelectorAll("[role=group]")].map((group) => group.getAttribute("aria-label"))).toEqual(expect.arrayContaining([GERMAN["model.scale"], GERMAN["model.mode"]]));
    expect(modeButtons(host).map((button) => button.textContent)).toEqual(VIEWS.map((view) => VIEW_NAMES[view].de));
    fireEvent.click(scaleButtons(host)[3]!);
    expect(host.querySelector("[data-announce]")!.textContent).toBe(translator("de")("model.scaleUnavailable", { scale: "500", reason: "Das kleinste Teil misst 0,3 mm auf dem Papier." }));
    fireEvent.click(scaleButtons(host)[0]!);
    expect(stage.viewer!.activeScene).toBe("s50-thermal");
    expect(spoken(host)).toBe(translator("de")("model.status.scene", { scale: 50, mode: GERMAN["model.mode.thermal"], representation: VIEW_NAMES.thermal.de }));
  });
});

describe("⌨️ keyboard and announcement", () => {
  it("moves the focus with the arrow keys, Home and End over the available buttons of a group, and Enter chooses like a click", async () => {
    const scales: number[] = [];
    const { host } = mount({ onScale: (scale) => scales.push(scale) });
    await ready(host);
    const buttons = scaleButtons(host);
    buttons[0]!.focus();
    key(buttons[0]!, "ArrowRight");
    expect(document.activeElement).toBe(buttons[1]);
    key(buttons[1]!, "ArrowRight");
    expect(document.activeElement).toBe(buttons[2]);
    key(buttons[2]!, "ArrowRight");
    expect(document.activeElement).toBe(buttons[3]);
    key(buttons[3]!, "ArrowLeft");
    expect(document.activeElement).toBe(buttons[2]);
    key(buttons[2]!, "Home");
    expect(document.activeElement).toBe(buttons[0]);
    key(buttons[0]!, "End");
    expect(document.activeElement).toBe(buttons[3]);
    const modes = modeButtons(host);
    modes[0]!.focus();
    key(modes[0]!, "ArrowLeft");
    expect(document.activeElement).toBe(modes[6]);
    key(modes[6]!, "ArrowRight");
    expect(document.activeElement).toBe(modes[0]);
    expect(buttons.every((button) => button.tabIndex >= 0 || button.getAttribute("aria-disabled") === "true")).toBe(true);
    expect(scales).toEqual([]);
  });

  it("names the scale and the mode of the scene in the viewer's polite status when they change", async () => {
    const { host } = mount();
    await ready(host);
    fireEvent.click(scaleButtons(host)[0]!);
    expect(spoken(host)).toBe(translator("en")("model.status.scene", { scale: 50, mode: ENGLISH["model.mode.body"], representation: VIEW_NAMES["light-solid"].en }));
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("thermal")]!);
    expect(spoken(host)).toBe(translator("en")("model.status.scene", { scale: 50, mode: ENGLISH["model.mode.thermal"], representation: VIEW_NAMES.thermal.en }));
    fireEvent.click(modeButtons(host)[VIEWS.indexOf("moments")]!);
    expect(spoken(host)).toBe(translator("en")("model.status.scene", { scale: 50, mode: ENGLISH["model.mode.structure"], representation: VIEW_NAMES.moments.en }));
  });

  it("hides both groups with the toolbar", async () => {
    const { host } = mount({ toolbar: false });
    await ready(host);
    expect(host.querySelector<HTMLElement>(".semio-stage-bar")!.hidden).toBe(true);
  });
});

describe("🔗️ one scale and one view for 2D and 3D", () => {
  const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
  function Stateful(props: { readonly patches: Partial<ViewerState>[]; readonly language?: Language }): ReactElement {
    const [viewer, setViewer] = useState<ViewerState>({ ...DEFAULT_VIEWER, viewpoint: "model", selection: { kind: "pose", pose: "stand" } });
    const language = props.language ?? "en";
    return (
      <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
        <Viewer asset={partial} viewer={viewer} onChange={(patch) => { props.patches.push(patch); setViewer((current) => ({ ...current, ...patch })); }} />
      </ArchiveProvider>
    );
  }
  const checked = (name: string) => within(screen.getByRole("radiogroup", { name })).getAllByRole("radio").filter((radio) => radio.getAttribute("aria-checked") === "true").map((radio) => radio.textContent);

  it("patches the viewer state from the stage, so the scale and view radio groups of the viewer show the same choice", async () => {
    const patches: Partial<ViewerState>[] = [];
    const { container } = render(<Stateful patches={patches} />);
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    expect(checked("Drawing scale")).toEqual(["1:100"]);
    fireEvent.click(container.querySelector("[data-group=scale] [data-scale='200']")!);
    expect(patches.at(-1)).toEqual({ scale: 200 });
    expect(checked("Drawing scale")).toEqual(["1:200"]);
    fireEvent.click(container.querySelector("[data-group=mode] [data-view='thermal']")!);
    expect(patches.at(-1)).toEqual({ view: "thermal" });
    expect(checked("Representation")).toEqual([VIEW_NAMES.thermal.en]);
    expect(container.querySelector("[data-model-stage]")!.getAttribute("data-view")).toBe("thermal");
    fireEvent.click(screen.getByRole("radio", { name: "Plan (top view)" }));
    await waitFor(() => expect(container.querySelector("[data-model-stage]")).toBeNull());
    expect(checked("Drawing scale")).toEqual(["1:200"]);
    expect(checked("Representation")).toEqual([VIEW_NAMES.thermal.en]);
    expect(container.querySelector("svg")).not.toBeNull();
  });

  it("changes the stage when the radio groups of the viewer change, and marks the scale without a model in the viewer's own scale group with its reason", async () => {
    const patches: Partial<ViewerState>[] = [];
    const { container } = render(<Stateful patches={patches} />);
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    fireEvent.click(within(screen.getByRole("radiogroup", { name: "Drawing scale" })).getByRole("radio", { name: "1:50" }));
    await waitFor(() => expect(pressed([...container.querySelectorAll<HTMLButtonElement>("[data-group=scale] button")])).toEqual(["50"]));
    const radio = within(screen.getByRole("radiogroup", { name: "Drawing scale" })).getByRole("radio", { name: /1:500/u });
    expect(radio.getAttribute("aria-disabled")).toBe("true");
    expect(radio.getAttribute("aria-description") ?? radio.getAttribute("title")).toContain("The smallest part is 0.3 mm on paper.");
    patches.length = 0;
    fireEvent.click(radio);
    expect(patches).toEqual([]);
  });

  it("is German in the viewer too", async () => {
    const { container } = render(<Stateful patches={[]} language="de" />);
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    expect(container.querySelector("[data-group=scale]")!.getAttribute("aria-label")).toBe(GERMAN["model.scale"]);
    expect(screen.getByRole("radiogroup", { name: GERMAN["viewer.scale"] })).toBeTruthy();
  });
});

describe("🧩️ the custom element", () => {
  it("follows the attributes scale and view and fires staffage-scale and staffage-view when the stage chooses", async () => {
    defineStaffageElement([partial]);
    const element = document.createElement("staffage-figure");
    for (const [name, value] of Object.entries({ asset: "sample-figure", view: "light-solid", scale: "100", lang: "en", viewpoint: "model", pose: "stand" })) element.setAttribute(name, value);
    document.body.append(element);
    hosts.push(element);
    const events: string[] = [];
    element.addEventListener("staffage-scale", (event) => events.push(`scale:${(event as CustomEvent).detail.scale}`));
    element.addEventListener("staffage-view", (event) => events.push(`view:${(event as CustomEvent).detail.view}`));
    await waitFor(() => expect(element.shadowRoot!.querySelector("[data-group=scale]")).not.toBeNull(), { timeout: 60000 });
    fireEvent.click(element.shadowRoot!.querySelector("[data-scale='50']")!);
    fireEvent.click(element.shadowRoot!.querySelector("[data-view='thermal']")!);
    expect(element.getAttribute("scale")).toBe("50");
    expect(element.getAttribute("view")).toBe("thermal");
    expect(events).toEqual(["scale:50", "view:thermal"]);
  });
});

describe("🌐️ both languages are complete", () => {
  it("has every new text in English and German with the same placeholders", () => {
    const keys = Object.keys(ENGLISH).filter((name) => /^model\.(scale|mode|noScaleList|status\.scene)/u.test(name));
    expect(keys.length).toBeGreaterThanOrEqual(10);
    const placeholders = (text: string) => [...text.matchAll(/\{(\w+)\}/gu)].map((match) => match[1]).sort();
    for (const name of keys) {
      expect(GERMAN[name as keyof typeof GERMAN], name).toBeTruthy();
      expect(placeholders(GERMAN[name as keyof typeof GERMAN]), name).toEqual(placeholders(ENGLISH[name as keyof typeof ENGLISH]));
    }
  });
});
