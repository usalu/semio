/** 🧊️ The interactive 3D model view: mode switching between drawings and the model, the seven representations as variants of one model, the shared clock driving the clip time, camera presets by keyboard and buttons, reduced motion, the software fallback when WebGL2 is missing or lost, progress and cancellation, the text alternative, and the custom element.
 * The viewer runs on its software rasteriser here (jsdom has no WebGL2), which draws the same scene; the WebGL2 path is checked in a real browser (see the report).
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { VIEWS } from "@teaching/staffage";
import { StaffageClock, StaffageFigure, defineStaffageElement } from "@teaching/staffage-react";
import { SoftwareBackend } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { preparedOf, prepareModel } from "../../🔨️modules/🧊️spatial/🏭️prepare/🟦️.ts";
import { ModelStage, type StageOptions, type StageState } from "../../🔨️modules/🧊️spatial/🖥️stage/🟦️.ts";
import { CAMERA_PRESETS, describeModel, modelKey, qualityFor, variantOf } from "../../🔨️modules/🧊️spatial/🟦️.ts";
import { sampleFigure } from "../🧰️support/🧰️.ts";

const pose = { kind: "pose", pose: "stand" } as const;
const clip = { kind: "clip", clip: "walk", timeS: 0 } as const;
const base = { asset: sampleFigure, query: pose, view: "light-solid", scale: 100, language: "en", preset: "perspective", settleMs: 0 } as const satisfies StageOptions;
const hosts: HTMLElement[] = [];
const stages: ModelStage[] = [];

function mount(options: Partial<StageOptions> = {}): { readonly host: HTMLElement; readonly stage: ModelStage; readonly root: () => HTMLElement } {
  const host = document.createElement("div");
  document.body.append(host);
  hosts.push(host);
  const stage = new ModelStage(host, { ...base, ...options } as StageOptions);
  stages.push(stage);
  return { host, stage, root: () => host.querySelector<HTMLElement>("[data-model-stage]")! };
}

const phase = (host: HTMLElement) => host.querySelector("[data-model-stage]")!.getAttribute("data-state");
const ready = (host: HTMLElement) => waitFor(() => expect(phase(host)).toBe("ready"), { timeout: 60000 });
const sum = (pixels: Uint8ClampedArray): string => {
  let hash = 2166136261;
  for (const value of pixels) hash = Math.imul(hash ^ value, 16777619) >>> 0;
  return String(hash);
};

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

describe("🔀 mode switching", () => {
  it("shows a drawing for the planar viewpoints and the axonometric view and the interactive model for the viewpoint model, and back", async () => {
    const figure = (viewpoint: "elevation" | "plan" | "axonometric" | "model") => <StaffageFigure asset={sampleFigure} view="light-solid" viewpoint={viewpoint} scale={{ denominator: 100 }} language="en" query={pose} />;
    const { container, rerender } = render(figure("elevation"));
    expect(container.querySelector("svg")).not.toBeNull();
    expect(container.querySelector("[data-model-stage]")).toBeNull();
    rerender(figure("axonometric"));
    expect(container.querySelector("[data-loading='axonometric']")).not.toBeNull();
    expect(container.querySelector("[role=status]")!.textContent).toBe(translator("en")("viewpoints.loading"));
    await waitFor(() => expect(container.querySelector("svg")).not.toBeNull(), { timeout: 60000 });
    expect(container.querySelector("[data-refusal]")).toBeNull();
    expect(container.querySelector("[data-viewpoint]")!.getAttribute("data-viewpoint")).toBe("axonometric");
    rerender(figure("model"));
    await waitFor(() => expect(container.querySelector("[data-model-stage]")).not.toBeNull());
    expect(container.querySelector("svg")).toBeNull();
    await waitFor(() => expect(container.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    rerender(figure("plan"));
    expect(container.querySelector("[data-model-stage]")).toBeNull();
    expect(container.querySelector("svg")).not.toBeNull();
  });

  it("names the refusal of the axonometric view for an asset without volume, in both languages, and draws nothing instead", async () => {
    const bare = { ...sampleFigure, volume: undefined } as never;
    for (const language of ["en", "de"] as const) {
      const { container, unmount } = render(<StaffageFigure asset={bare} view="light-solid" viewpoint="axonometric" scale={{ denominator: 100 }} language={language} query={pose} />);
      await waitFor(() => expect(container.querySelector("[data-refusal='axonometric']")).not.toBeNull(), { timeout: 60000 });
      expect(container.querySelector("[data-refusal='axonometric']")!.textContent).toContain(translator(language)("viewpoints.missing.axonometric"));
      expect(container.querySelector("svg")).toBeNull();
      unmount();
    }
  });
});

describe("🎭️ representations are variants of one model", () => {
  it("builds every one of the seven views as a variant, rebuilds only for scale, pose or clip and analytic settings, never for view or camera", async () => {
    const { host, stage } = mount();
    const scenes: { variants: ReadonlyMap<string, unknown> }[] = [];
    stage.viewer!.on("ready", ({ scene }) => scenes.push(scene));
    await ready(host);
    expect([...scenes[0]!.variants.keys()].sort()).toEqual([...VIEWS].sort());
    for (const view of VIEWS) expect(scenes[0]!.variants.has(variantOf(view))).toBe(true);
    const document = preparedOf(sampleFigure, { ...base });
    for (const view of VIEWS) {
      stage.update({ view });
      expect(modelKey(sampleFigure, { ...base, view })).toBe(modelKey(sampleFigure, base));
    }
    stage.update({ preset: "plan" });
    expect(preparedOf(sampleFigure, { ...base })).toBe(document);
    expect(scenes).toHaveLength(1);
    expect(modelKey(sampleFigure, { ...base, scale: 50 })).not.toBe(modelKey(sampleFigure, base));
    expect(modelKey(sampleFigure, { ...base, query: clip })).not.toBe(modelKey(sampleFigure, base));
    expect(modelKey(sampleFigure, { ...base, thermalQuantity: "skin-temperature" })).not.toBe(modelKey(sampleFigure, base));
  });

  it("paints the representations differently: solid, line, thermal, forces and moments pictures are not the same picture", async () => {
    const { host, stage } = mount();
    await ready(host);
    const pictures = new Map<string, string>();
    for (const view of VIEWS) {
      stage.update({ view });
      pictures.set(view, sum(stage.viewer!.readPixels()));
    }
    expect(new Set(pictures.values()).size).toBeGreaterThanOrEqual(6);
    for (const [a, b] of [["light-solid", "dark-solid"], ["light-solid", "thermal"], ["light-solid", "forces"], ["dark-line", "light-line"]] as const) expect(pictures.get(a)).not.toBe(pictures.get(b));
  });

  it("follows the level of detail of the scale: a coarser scale tessellates coarser", () => {
    const chords = [20, 50, 100, 200, 500].map((scale) => qualityFor(sampleFigure, scale).chordM);
    expect([...chords].sort((x, y) => x - y)).toEqual(chords);
    expect(chords[0]).toBeLessThan(chords[4]!);
    expect(qualityFor(sampleFigure, 500).maxSegments).toBeLessThan(qualityFor(sampleFigure, 20).maxSegments);
  });
});

describe("⏱️ the shared transport drives the clip time", () => {
  it("shows the clock time in the viewer, follows scrub and step, and a pose has no clip time", async () => {
    const duration = sampleFigure.clips.find((candidate) => candidate.id === "walk")!.durationS;
    const clock = new StaffageClock({ fps: 30, durationS: duration, playing: false, reducedMotion: true });
    const { host, stage } = mount({ query: clip, clock });
    await ready(host);
    expect(stage.viewer!.clock.animation).toBe("walk");
    expect(stage.viewer!.clock.timeS).toBeCloseTo(0, 9);
    const first = sum(stage.viewer!.readPixels());
    clock.seek(0.5);
    expect(stage.viewer!.clock.timeS).toBeCloseTo(clock.state.frame / 30, 9);
    expect(stage.viewer!.clock.timeS).toBeCloseTo(0.5, 9);
    clock.step(3);
    expect(stage.viewer!.clock.timeS).toBeCloseTo(18 / 30, 9);
    expect(sum(stage.viewer!.readPixels())).not.toBe(first);
    clock.dispose();
    const still = mount({ query: pose });
    await ready(still.host);
    expect(still.stage.viewer!.clock.animation).toBeNull();
  });

  it("toggles the clock with Space on the canvas and never plays by itself", async () => {
    const clock = new StaffageClock({ fps: 30, durationS: 2, playing: false, reducedMotion: true });
    const { host, stage } = mount({ query: clip, clock });
    await ready(host);
    const canvas = host.querySelector("canvas")!;
    fireEvent.keyDown(canvas, { key: " " });
    expect(clock.state.playing).toBe(true);
    expect(stage.viewer!.clock.playing).toBe(false);
    fireEvent.keyDown(canvas, { key: " " });
    expect(clock.state.playing).toBe(false);
    clock.dispose();
  });

  it("starts paused and shows the pose of the first frame under prefers-reduced-motion", async () => {
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query.includes("reduce"), media: query, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, onchange: null, dispatchEvent: () => false }));
    const clockUnderReducedMotion = new StaffageClock({ fps: 30, durationS: 2 });
    expect(clockUnderReducedMotion.state.playing).toBe(false);
    const { host, stage } = mount({ query: clip, clock: clockUnderReducedMotion });
    await ready(host);
    expect(stage.viewer!.clock.playing).toBe(false);
    expect(stage.viewer!.clock.timeS).toBe(0);
    clockUnderReducedMotion.dispose();
    vi.unstubAllGlobals();
  });
});

describe("⌨️ camera, keyboard and accessibility", () => {
  it("operates the camera with the keyboard: keys 1 to 5 choose a preset, arrows leave it, the buttons name the preset and say which is on", async () => {
    const presets: string[] = [];
    const { host, root } = mount({ onPreset: (preset) => presets.push(preset) });
    await ready(host);
    const canvas = host.querySelector("canvas")!;
    const pressed = () => [...host.querySelectorAll<HTMLButtonElement>("[data-preset]")].filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.getAttribute("data-preset"));
    expect(pressed()).toEqual(["perspective"]);
    fireEvent.keyDown(canvas, { key: "2" });
    expect(root().getAttribute("data-preset")).toBe("front");
    expect(pressed()).toEqual(["front"]);
    fireEvent.keyDown(canvas, { key: "4" });
    expect(pressed()).toEqual(["axonometric"]);
    fireEvent.keyDown(canvas, { key: "ArrowLeft" });
    expect(pressed()).toEqual([]);
    expect(root().getAttribute("data-preset")).toBe("free");
    fireEvent.click(host.querySelector("[data-preset='plan']")!);
    expect(pressed()).toEqual(["plan"]);
    expect(presets).toEqual(["front", "axonometric", "plan"]);
    fireEvent.keyDown(canvas, { key: "ArrowUp" });
    fireEvent.click(host.querySelector("[data-action='reset']")!);
    expect(pressed()).toEqual(["plan"]);
  });

  it("zooms with the buttons and the keys and offers every control as a labelled native button that can take focus", async () => {
    const { host, stage } = mount();
    await ready(host);
    const height = () => stage.viewer!.getCamera().orthoHeight + stage.viewer!.getCamera().distance;
    const before = height();
    fireEvent.click(host.querySelector("[data-action='zoom-in']")!);
    const closer = height();
    expect(closer).toBeLessThan(before);
    fireEvent.keyDown(host.querySelector("canvas")!, { key: "-" });
    expect(height()).toBeGreaterThan(closer);
    const buttons = [...host.querySelectorAll("button")].filter((button) => !button.closest("[hidden]"));
    expect(buttons.length).toBeGreaterThanOrEqual(CAMERA_PRESETS.length + 3);
    for (const button of buttons) {
      expect(button.tagName).toBe("BUTTON");
      expect(button.tabIndex).toBeGreaterThanOrEqual(0);
      expect((button.getAttribute("aria-label") ?? button.textContent ?? "").trim().length).toBeGreaterThan(0);
    }
    const canvas = host.querySelector("canvas")!;
    expect(canvas.tabIndex).toBe(0);
    expect(canvas.getAttribute("role")).toBe("application");
    expect(canvas.getAttribute("aria-label")).toBe("3D model of Sample figure");
    expect(host.querySelector("[data-model-stage]")!.getAttribute("part")).toBe("stage");
  });

  it("describes the state in words in the reader's language and updates the description with representation, scale and clip time", async () => {
    const { host, stage } = mount({ view: "thermal", scale: 50 });
    await ready(host);
    const describedBy = host.querySelector("canvas")!.getAttribute("aria-describedby")!.split(" ");
    const text = () => describedBy.map((id) => host.querySelector(`#${id}`)?.textContent ?? "").join(" ");
    expect(text()).toContain(ENGLISH["model.instructions"]);
    expect(text()).toContain("Interactive 3D model of Sample figure");
    expect(text()).toContain("pose");
    expect(text()).toMatch(/thermal view/iu);
    expect(text()).toContain("scale 1:50");
    stage.update({ view: "forces", scale: 50 });
    expect(text()).toMatch(/force view/iu);
    expect(describeModel(sampleFigure, { ...base, query: { kind: "clip", clip: "walk", timeS: 1.5 }, timeS: 1.5, language: "de" }, "front", "de")).toMatch(/3D-Modell von .*Clip .* bei 1,50 s.*Maßstab 1:100.*Kamera Vorderansicht/u);
  });

  it("speaks German when asked: names, notice, instructions, no English left in the controls", async () => {
    const { host } = mount({ language: "de" });
    await ready(host);
    const names = [...host.querySelectorAll("button")].map((button) => (button.getAttribute("aria-label") ?? button.textContent ?? "").trim());
    expect(names).toContain("Seitenansicht");
    expect(names).toContain("Perspektive");
    expect(names).toContain("Ansicht zurücksetzen");
    expect(names).not.toContain("Side elevation");
    expect(host.querySelector("canvas")!.getAttribute("aria-label")).toBe("3D-Modell von Sample figure".replace("Sample figure", sampleFigure.title.de));
    expect(host.querySelector("[data-model-stage]")!.getAttribute("lang")).toBe("de");
  });

  it("marks forced-colours and contrast support in its style and gives the customisation properties", async () => {
    const { host } = mount();
    await ready(host);
    const css = [...(document.adoptedStyleSheets ?? [])].flatMap((sheet) => [...sheet.cssRules].map((rule) => rule.cssText)).concat([...host.querySelectorAll("style")].map((style) => style.textContent ?? "")).join("\n");
    for (const needle of ["forced-colors", "prefers-contrast", "prefers-reduced-motion", "--staffage-model-background", "--staffage-model-focus", "--staffage-model-accent", "--staffage-model-min-height", ":focus-visible"]) expect(css).toContain(needle);
  });
});

describe("🛟 fallbacks, progress and cancellation", () => {
  it("draws in software with a translated note when WebGL2 is not available, never a blank box", async () => {
    for (const language of ["en", "de"] as const) {
      const { host, stage } = mount({ language });
      await ready(host);
      expect(stage.state.backend).toBe("software");
      expect(host.querySelector("[data-model-stage]")!.getAttribute("data-backend")).toBe("software");
      expect(host.querySelector(".semio-stage-note")!.textContent).toBe(translator(language)("model.backend.software"));
      const pixels = stage.viewer!.readPixels();
      expect(new Set(pixels).size).toBeGreaterThan(8);
    }
  });

  it("falls back to software, and says so, when the graphics context is lost and does not come back", async () => {
    let report: ((status: "ready" | "lost" | "restored") => void) | null = null;
    const backend: NonNullable<StageOptions["backend"]> = (_canvas, onStatus) => {
      report = onStatus;
      const stub = new SoftwareBackend(1, 1);
      Object.defineProperty(stub, "kind", { value: "webgl2" });
      return stub;
    };
    const { host, stage } = mount({ backend, restoreMs: 30 });
    await ready(host);
    expect(stage.state.backend).toBe("webgl2");
    report!("lost");
    expect(host.querySelector(".semio-stage-note")!.textContent).toBe(ENGLISH["model.contextLost"]);
    await waitFor(() => expect(stage.state.backend).toBe("software"), { timeout: 5000 });
    await ready(host);
    expect(host.querySelector(".semio-stage-note")!.textContent).toBe(ENGLISH["model.contextSwitched"]);
  });

  it("keeps the model when the graphics context comes back in time", async () => {
    let report: ((status: "ready" | "lost" | "restored") => void) | null = null;
    const backend: NonNullable<StageOptions["backend"]> = (_canvas, onStatus) => {
      report = onStatus;
      const stub = new SoftwareBackend(1, 1);
      Object.defineProperty(stub, "kind", { value: "webgl2" });
      return stub;
    };
    const { host, stage } = mount({ backend, restoreMs: 60 });
    await ready(host);
    report!("lost");
    report!("restored");
    await new Promise((resolve) => setTimeout(resolve, 120));
    expect(stage.state.backend).toBe("webgl2");
    expect(host.querySelector(".semio-stage-note")!.textContent).toBe(ENGLISH["model.contextRestored"]);
  });

  it("reports progress from 0 to 1 over the four phases and can be cancelled and retried", async () => {
    const seen: StageState[] = [];
    const holder: { stage: ModelStage | null } = { stage: null };
    let cancelled = false;
    const { host, stage } = mount({
      query: { kind: "clip", clip: "idle-breathing", timeS: 0 },
      onState: (state) => {
        seen.push(state);
        if (!cancelled && state.phase === "preparing" && state.fraction > 0.2) {
          cancelled = true;
          holder.stage!.cancel();
        }
      },
    });
    holder.stage = stage;
    await waitFor(() => expect(phase(host)).toBe("cancelled"), { timeout: 60000 });
    expect(host.querySelector(".semio-stage-notice")!.textContent).toContain(ENGLISH["model.cancelled"]);
    const retry = host.querySelector<HTMLButtonElement>("[data-action='retry']")!;
    expect(retry.textContent).toBe(ENGLISH["model.retry"]);
    fireEvent.click(retry);
    await ready(host);
    const fractions = seen.filter((state) => state.phase === "preparing").map((state) => state.fraction);
    expect(fractions[0]).toBeLessThan(0.3);
    expect(Math.max(...fractions)).toBeGreaterThan(0.9);
    expect(seen.at(-1)!.phase).toBe("ready");
    expect(host.querySelector("progress")!.hasAttribute("aria-label")).toBe(true);
  });

  it("cancels a preparation the viewer is not showing yet through the abort signal and keeps nothing", async () => {
    const control = new AbortController();
    const outcome = prepareModel(sampleFigure, { ...base, scale: 77 }, { signal: control.signal, progress: (fraction) => fraction > 0.1 && control.abort() });
    await expect(outcome).rejects.toMatchObject({ name: "AbortError" });
    expect(preparedOf(sampleFigure, { ...base, scale: 77 })).toBeUndefined();
  });

  it("names an asset without volume in words in both languages and shows no viewer", async () => {
    for (const language of ["en", "de"] as const) {
      const { host, root } = mount({ asset: { ...sampleFigure, volume: undefined } as never, language });
      await waitFor(() => expect(root().getAttribute("data-state")).toBe("unavailable"));
      expect(host.querySelector(".semio-stage-notice")!.textContent).toContain(translator(language)("model.noVolume"));
    }
  });

  it("explains a model that cannot be built as an alert instead of a blank box", async () => {
    const { host, root } = mount({ asset: { ...sampleFigure, volume: { ...sampleFigure.volume!, solids: [{ ...sampleFigure.volume!.solids[0]!, bone: "no-such-bone" } as never] } } as never });
    await waitFor(() => expect(root().getAttribute("data-state")).toBe("error"), { timeout: 60000 });
    expect(host.querySelector("[role='alert']")!.textContent).toContain(ENGLISH["model.error"]);
  });
});

describe("🧩️ <staffage-figure> with the model viewpoint", () => {
  defineStaffageElement([sampleFigure]);
  const flush = (ms = 0) => new Promise<void>((resolve) => setTimeout(resolve, ms));
  type Figure = HTMLElement & { stage: ModelStage | null };
  const make = (attributes: Record<string, string>): Figure => {
    const element = document.createElement("staffage-figure") as Figure;
    for (const [key, value] of Object.entries(attributes)) element.setAttribute(key, value);
    return element;
  };
  const attributes = { asset: "sample-figure", view: "light-solid", scale: "100", lang: "en", pose: "stand" };

  it("mounts the stage inside the shadow root, reports its state and keeps the camera attribute in step with the reader's choice", async () => {
    const element = make({ ...attributes, viewpoint: "model", camera: "front" });
    const states: string[] = [];
    element.addEventListener("staffage-model", (event) => states.push((event as CustomEvent<StageState>).detail.phase));
    document.body.append(element);
    await waitFor(() => expect(element.stage).not.toBeNull());
    await waitFor(() => expect(element.shadowRoot!.querySelector("[data-model-stage]")!.getAttribute("data-state")).toBe("ready"), { timeout: 60000 });
    expect(element.shadowRoot!.querySelector("[data-model-stage]")!.getAttribute("data-preset")).toBe("front");
    expect(states).toContain("ready");
    fireEvent.keyDown(element.shadowRoot!.querySelector("canvas")!, { key: "3" });
    expect(element.getAttribute("camera")).toBe("plan");
    element.setAttribute("view", "thermal");
    await flush(10);
    expect(element.shadowRoot!.querySelector("[data-model-stage]")!.getAttribute("data-view")).toBe("thermal");
    element.setAttribute("no-toolbar", "");
    await flush(10);
    expect(element.shadowRoot!.querySelector<HTMLElement>("[part='toolbar']")!.hidden).toBe(true);
    element.remove();
    expect(element.stage).toBeNull();
  });

  it("switches between the model, the axonometric drawing and a planar drawing by the viewpoint attribute", async () => {
    const element = make({ ...attributes, viewpoint: "axonometric" });
    document.body.append(element);
    await flush(20);
    expect(element.shadowRoot!.querySelector("svg")).not.toBeNull();
    expect(element.shadowRoot!.querySelector("[role=alert]")).toBeNull();
    element.setAttribute("viewpoint", "model");
    await waitFor(() => expect(element.shadowRoot!.querySelector("[data-model-stage]")).not.toBeNull());
    element.setAttribute("viewpoint", "plan");
    await flush(20);
    expect(element.shadowRoot!.querySelector("[data-model-stage]")).toBeNull();
    expect(element.shadowRoot!.querySelector("svg")).not.toBeNull();
    element.remove();
  });

  it("asks for a viewpoint in words that name the model, and says nothing renders without a language", async () => {
    const element = make({ ...attributes, viewpoint: "sideways" });
    document.body.append(element);
    await flush(20);
    expect(element.shadowRoot!.querySelector("[role=alert]")!.textContent).toMatch(/elevation, front, plan, axonometric, model or all/u);
    element.setAttribute("lang", "de");
    await flush(20);
    expect(element.shadowRoot!.querySelector("[role=alert]")!.textContent).toMatch(/axonometric, model oder all/u);
    element.remove();
  });
});

describe("🌐️ both languages are complete", () => {
  it("has every 3D key in English and German with the same placeholders and no empty text", () => {
    const keys = Object.keys(ENGLISH).filter((key) => /^(model\.|viewpoints\.(name|phrase|missing|announce|helpSpatial)\.?(axonometric|model|Spatial)?|detail\.format\.(glb|fbx|model))/u.test(key));
    expect(keys.length).toBeGreaterThan(60);
    const placeholders = (text: string) => [...text.matchAll(/\{(\w+)\}/gu)].map((match) => match[1]).sort();
    for (const key of keys) {
      const en = (ENGLISH as Record<string, string>)[key]!;
      const de = (GERMAN as Record<string, string>)[key]!;
      expect(en.trim().length, key).toBeGreaterThan(0);
      expect(de.trim().length, key).toBeGreaterThan(0);
      expect(placeholders(de), key).toEqual(placeholders(en));
    }
    const identical = keys.filter((key) => (ENGLISH as Record<string, string>)[key] === (GERMAN as Record<string, string>)[key]);
    expect(identical.sort()).toEqual(["model.errorBody", "model.exports.clip", "model.exports.phase", "model.exports.pose", "model.progressValue", "model.status.animation", "model.status.pose"]);
    expect(Object.keys(GERMAN).sort()).toEqual(Object.keys(ENGLISH).sort());
  });
});
