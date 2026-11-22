/** 🌬️ Full control over the air: the wind dial against its numpy oracle, keyboard operation, labels with units in both languages and the reader's number format, presets applied whole, conditions that solve again against mark detail that only redraws, the viewer's controls reaching the solver, the custom element's attributes on its drawing, sheet and 3D model, and the export request carrying the same air.
 * @see ./🥒️.feature — the scenarios
 * @see ./🐍️.py — the numpy oracle of the fixture */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { useState, type ReactElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { DEFAULT_TURBULENCE, flowDetailOf, formatNumber, type Environment, type FlowField, type Language } from "@teaching/staffage";
import { defineStaffageElement } from "@teaching/staffage-react";
import { AirControls, type AirState } from "../../🔨️modules/🎛️viewer/🌬️air/🟦️.tsx";
import { DIAL_MAX_MS, dialPoint, dialWind, groundBasis, windWords, type DialView } from "../../🔨️modules/🎛️viewer/🧭️wind/🟦️.tsx";
import { Viewer } from "../../🔨️modules/🎛️viewer/🟦️.tsx";
import { ENVIRONMENT_PRESETS, ENVIRONMENT_QUANTITIES, environmentOf } from "../../🔨️modules/🌤️environments/🟦️.ts";
import { ALL_FLOW_LAYERS, FLOW_DEFAULTS, FLOW_DEVELOP_S, FLOW_QUANTITIES, MARK_DEFAULTS, MARK_QUANTITIES, WIND_SECTORS, flowConditionsOf, windSector } from "../../🔨️modules/🌊️flow/🟦️.ts";
import { FlowStage, type FlowStageOptions } from "../../🔨️modules/🌊️flow/🖥️stage/🟦️.ts";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { GERMAN } from "../../🔨️modules/🌐️i18n/🔡️german/🟦️.ts";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { DEFAULT_VIEWER, type ViewerState } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { depictionOptions } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

defineStaffageElement([walker]);
afterEach(() => {
  cleanup();
  document.body.replaceChildren();
});

type DialCase = { readonly view: DialView; readonly bearingDeg: number; readonly speedMS: number; readonly point: readonly [number, number]; readonly solvedBearingDeg: number; readonly solvedSpeedMS: number; readonly sector: string };
const fixture: { readonly depth: number; readonly rimMS: number; readonly sectors: readonly string[]; readonly cases: readonly DialCase[] } = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../../../🧫️fixtures/🧭️wind-dial/🔣️.json"), "utf8"));
const own = walker.thermal.environment;
const EMPTY: AirState = { preset: null, environment: {}, flowSettings: {}, flowDetail: {} };
const LONG = 600_000;

function Controls(props: { readonly language: Language; readonly initial?: AirState; readonly log?: Partial<AirState>[]; readonly open?: boolean }): ReactElement {
  const [air, setAir] = useState<AirState>(props.initial ?? EMPTY);
  return (
    <form aria-label="air">
      <AirControls asset={walker} air={air} onChange={(patch) => { props.log?.push(patch); setAir((before) => ({ ...before, ...patch })); }} t={translator(props.language)} language={props.language} thermal flowable dial={{ view: "plan", name: "plan" }} {...(props.open === false ? {} : { open: { environment: true, wind: true, turbulence: true, marks: true } })} />
    </form>
  );
}

describe("🧭️ the wind dial", () => {
  it("places every wind of the fixture where numpy places it and reads it back", () => {
    expect(fixture.rimMS).toBe(DIAL_MAX_MS);
    expect(fixture.sectors).toEqual(WIND_SECTORS);
    expect(fixture.cases.length).toBe(30);
    for (const entry of fixture.cases) {
      const basis = groundBasis(entry.view);
      const [x, y] = dialPoint(basis, entry.bearingDeg, entry.speedMS);
      expect(x).toBeCloseTo(entry.point[0], 9);
      expect(y).toBeCloseTo(entry.point[1], 9);
      expect(dialWind(basis, x, y, 0)).toEqual({ bearingDeg: entry.solvedBearingDeg, speedMS: entry.solvedSpeedMS });
      expect(windSector(entry.bearingDeg)).toBe(entry.sector);
    }
  });

  it("draws the ground as each viewpoint shows it and inverts the axonometric dial too", () => {
    expect(groundBasis("plan")).toEqual({ ahead: [1, -0], left: [0, -1] });
    expect(groundBasis("elevation").left[1]).toBeCloseTo(-fixture.depth, 12);
    expect(groundBasis("front").ahead[1]).toBeCloseTo(fixture.depth, 12);
    const basis = groundBasis("axonometric");
    for (const [bearing, speed] of [[30, 2], [200, 11.5], [315, 0.4]] as const) expect(dialWind(basis, ...dialPoint(basis, bearing, speed), 0)).toEqual({ bearingDeg: bearing, speedMS: speed });
    expect(dialWind(basis, 0, 0, 77)).toEqual({ bearingDeg: 77, speedMS: 0 });
    expect(dialWind(basis, ...dialPoint(basis, 90, 500), 0).speedMS).toBe(DIAL_MAX_MS);
  });

  it("says where the air goes in words, in both languages", () => {
    expect(windWords(translator("en"), "en", 90, 1.5)).toBe("The air moves towards the asset's left (90°) at 1.5 m/s.");
    expect(windWords(translator("de"), "de", 90, 1.5)).toBe("Die Luft zieht zur linken Seite des Assets (90°) mit 1,5 m/s.");
    expect(windWords(translator("en"), "en", 0, 0)).toBe("Still air: no wind.");
    expect(windWords(translator("de"), "de", 270, 0)).toBe("Ruhende Luft: kein Wind.");
    expect(new Set(WIND_SECTORS.map((sector) => translator("en")(`wind.towards.${sector}`))).size).toBe(8);
  });

  it("is one slider operated by keyboard and by pointer, with the wind in words beside it", () => {
    const log: Partial<AirState>[] = [];
    render(<Controls language="en" log={log} />);
    const dial = screen.getByRole("slider", { name: "Wind direction and speed" });
    expect(dial.tabIndex).toBe(0);
    expect(dial.getAttribute("aria-valuenow")).toBe(String(own.windDirectionDeg));
    expect(dial.getAttribute("aria-valuetext")).toBe("180°, 0.2 m/s: backwards, behind the asset");
    expect(document.getElementById(dial.getAttribute("aria-describedby")!)!.textContent).toContain("Home points it straight ahead");
    fireEvent.keyDown(dial, { key: "ArrowRight" });
    expect(log.at(-1)).toEqual({ environment: { windDirectionDeg: 185 } });
    fireEvent.keyDown(dial, { key: "ArrowUp" });
    expect(log.at(-1)).toEqual({ environment: { windDirectionDeg: 185, windMS: 0.3 } });
    fireEvent.keyDown(dial, { key: "ArrowRight", shiftKey: true });
    expect(log.at(-1)).toEqual({ environment: { windDirectionDeg: 230, windMS: 0.3 } });
    fireEvent.keyDown(dial, { key: "ArrowLeft", shiftKey: true });
    fireEvent.keyDown(dial, { key: "ArrowLeft" });
    fireEvent.keyDown(dial, { key: "ArrowDown" });
    expect(log.at(-1)).toEqual({ environment: {} });
    fireEvent.keyDown(dial, { key: "Home" });
    expect(log.at(-1)).toEqual({ environment: { windDirectionDeg: 0 } });
    expect(document.querySelector("[data-wind-words]")!.textContent).toBe("The air moves straight ahead of the asset (0°) at 0.2 m/s.");
    fireEvent.keyDown(dial, { key: "End" });
    expect(log.at(-1)).toEqual({ environment: { windDirectionDeg: 0, windMS: 0 } });
    expect(document.querySelector("[data-wind-words]")!.textContent).toBe("Still air: no wind.");
    expect(document.querySelector("[data-wind-arrow]")!.getAttribute("data-wind-arrow")).toBe("still");
    fireEvent.keyDown(dial, { key: "a" });
    expect(log).toHaveLength(8);
  });
});

describe("🎚️ quantities, sections and presets", () => {
  it("labels every quantity with its unit in English and German, key for key", () => {
    for (const entry of [...ENVIRONMENT_QUANTITIES, ...FLOW_QUANTITIES, ...MARK_QUANTITIES]) {
      const key = `quantity.${entry.key}` as keyof typeof ENGLISH;
      expect(ENGLISH[key], entry.key).toBeTruthy();
      expect(GERMAN[key], entry.key).toBeTruthy();
      expect(GERMAN[key]).not.toBe(ENGLISH[key]);
      expect(entry.min).toBeLessThan(entry.max);
    }
    for (const name of ["param", "attribute"] as const) expect(new Set([...ENVIRONMENT_QUANTITIES, ...FLOW_QUANTITIES, ...MARK_QUANTITIES].map((entry) => entry[name])).size).toBe(ENVIRONMENT_QUANTITIES.length + FLOW_QUANTITIES.length + MARK_QUANTITIES.length);
  });

  it.each(["en", "de"] as const)("gives every quantity a field, a slider and a way back, named with its unit, in %s", (language) => {
    render(<Controls language={language} />);
    const t = translator(language);
    for (const entry of [...ENVIRONMENT_QUANTITIES, ...FLOW_QUANTITIES, ...MARK_QUANTITIES]) {
      const name = t(`quantity.${entry.key}` as keyof typeof ENGLISH);
      const label = entry.unit === "" ? name : `${name} (${entry.unit})`;
      const row = document.querySelector<HTMLElement>(`[data-quantity=${entry.key}]`)!;
      const field = row.querySelector<HTMLInputElement>("input[type=text]")!;
      expect(row.querySelector(`label[for="${field.id}"]`)!.textContent, label).toBe(label);
      expect(row.querySelector("input[type=range]")!.getAttribute("aria-label")).toBe(`${label}, ${language === "en" ? "slider" : "Schieberegler"}`);
      expect(within(row).getByRole("button", { name: t("quantity.reset", { name }) })).toBeTruthy();
    }
    expect(within(document.querySelector<HTMLElement>("[data-quantity=airTempC]")!).getByRole("textbox", { name: language === "en" ? "Air temperature (°C)" : "Lufttemperatur (°C)" })).toBeTruthy();
  });

  it("shows values in the reader's number format and reads a comma or a point", () => {
    const log: Partial<AirState>[] = [];
    render(<Controls language="de" log={log} initial={{ ...EMPTY, environment: { airTempC: -5.5 } }} />);
    const air = screen.getByRole("textbox", { name: /^Lufttemperatur \(°C\)/u }) as HTMLInputElement;
    expect(air.value).toBe("-5,5");
    expect((screen.getByRole("textbox", { name: /^Luftdruck \(hPa\)/u }) as HTMLInputElement).value).toBe("1013,25");
    expect(screen.getByRole("slider", { name: "Lufttemperatur (°C), Schieberegler" }).getAttribute("aria-valuetext")).toBe("-5,5 °C");
    fireEvent.change(air, { target: { value: "12,5" } });
    expect(log.at(-1)).toEqual({ environment: { airTempC: 12.5 } });
    fireEvent.change(air, { target: { value: "13.5" } });
    expect(log.at(-1)).toEqual({ environment: { airTempC: 13.5 } });
    const before = log.length;
    fireEvent.change(air, { target: { value: "900" } });
    expect(air.getAttribute("aria-invalid")).toBe("true");
    expect(log).toHaveLength(before);
    fireEvent.blur(air);
    expect(air.value).toBe("13,5");
    fireEvent.change(screen.getByRole("slider", { name: "Relative Luftfeuchte (%), Schieberegler" }), { target: { value: "85" } });
    expect(log.at(-1)).toEqual({ environment: { airTempC: 13.5, relativeHumidity: 0.85 } });
  });

  it("marks a changed value, counts it in its section and takes it back to its default", () => {
    const log: Partial<AirState>[] = [];
    render(<Controls language="en" log={log} />);
    const section = (name: string): HTMLElement => document.querySelector<HTMLElement>(`[data-air-section=${name}]`)!;
    expect(within(section("turbulence")).getByRole("button", { name: "Turbulence and wind profile" }).getAttribute("aria-expanded")).toBe("true");
    const seed = screen.getByRole("textbox", { name: /^Seed of the turbulent motion/u });
    expect(document.getElementById(seed.getAttribute("aria-describedby")!)!.textContent).toBe("Default: 1");
    fireEvent.change(seed, { target: { value: "7" } });
    expect(log.at(-1)).toEqual({ flowSettings: { seed: 7 } });
    expect(document.querySelector("[data-quantity=seed]")!.hasAttribute("data-changed")).toBe(true);
    expect(document.querySelector("[data-quantity=seed] label")!.textContent).toContain("changed");
    expect(within(section("turbulence")).getByRole("button", { name: "Turbulence and wind profile (changed: 1)" })).toBeTruthy();
    fireEvent.click(within(section("turbulence")).getByRole("radio", { name: "Logarithmic" }));
    expect(log.at(-1)).toEqual({ flowSettings: { seed: 7, profile: "logarithmic" } });
    expect((screen.getByRole("textbox", { name: /^Roughness length of the ground \(m\)/u }) as HTMLInputElement).disabled).toBe(false);
    fireEvent.click(screen.getByRole("button", { name: "Back to the default: Seed of the turbulent motion" }));
    expect(log.at(-1)).toEqual({ flowSettings: { profile: "logarithmic" } });
    fireEvent.change(seed, { target: { value: "1" } });
    expect(log.at(-1)).toEqual({ flowSettings: { profile: "logarithmic" } });
    fireEvent.click(within(section("turbulence")).getByRole("button", { name: "Back to the defaults" }));
    expect(log.at(-1)).toEqual({ flowSettings: {} });
    expect((screen.getByRole("textbox", { name: /^Roughness length of the ground \(m\)/u }) as HTMLInputElement).disabled).toBe(true);
    fireEvent.change(screen.getByRole("slider", { name: "Spacing of streams and clouds on paper (mm), slider" }), { target: { value: "4" } });
    expect(log.at(-1)).toEqual({ flowDetail: { spacingMm: 4 } });
    const toggle = within(section("marks")).getByRole("button", { name: "Flow marks (changed: 1)" });
    fireEvent.click(toggle);
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
    expect(section("marks").querySelector("[data-quantity]")).toBeNull();
    fireEvent.click(toggle);
    expect(section("marks").querySelectorAll("[data-quantity]")).toHaveLength(MARK_QUANTITIES.length);
  });

  it("keeps its defaults equal to the library's", () => {
    expect(flowConditionsOf({})).toEqual({});
    expect(flowConditionsOf({ turbulenceIntensity: 0.25 })).toEqual({ turbulence: { ...DEFAULT_TURBULENCE, intensity: 0.25 } });
    expect(flowConditionsOf({ profile: "logarithmic", referenceHeightM: 4, lifeS: 6, seed: 3 })).toEqual({ boundaryLayer: { z0M: FLOW_DEFAULTS.roughnessM, referenceHeightM: 4 }, maxAgeS: 6, seed: 3 });
    expect(flowConditionsOf({ roughnessM: 0.5 })).toEqual({});
    expect(FLOW_DEFAULTS.lifeS).toBe(FLOW_DEVELOP_S);
    const { trailS: _trail, strokeMm: _stroke, ...timing } = MARK_DEFAULTS;
    expect(flowDetailOf(100, timing)).toEqual({ ...flowDetailOf(100), periodS: MARK_DEFAULTS.periodS, exposureS: MARK_DEFAULTS.exposureS });
  });

  it("applies a reference environment whole, names it, and reads changed by hand afterwards", () => {
    const log: Partial<AirState>[] = [];
    render(<Controls language="en" log={log} />);
    const preset = ENVIRONMENT_PRESETS.find((candidate) => candidate.environment.windDirectionDeg !== own.windDirectionDeg || candidate.environment.pressurePa !== own.pressurePa) ?? ENVIRONMENT_PRESETS[0]!;
    const select = screen.getByRole("combobox", { name: "Reference environment" }) as HTMLSelectElement;
    expect(select.value).toBe("own");
    expect(document.querySelector("[data-environment-status]")!.textContent).toBe("Active environment: Reference case of the asset.");
    fireEvent.change(select, { target: { value: preset.id } });
    expect(log.at(-1)).toEqual({ preset: preset.id, environment: {} });
    expect(select.value).toBe(preset.id);
    expect(document.querySelector("[data-environment-status]")!.textContent).toBe(`Active environment: ${preset.title.en}.`);
    expect(environmentOf(walker, preset.id, {}).environment).toEqual(preset.environment);
    for (const entry of ENVIRONMENT_QUANTITIES) {
      const stored = (preset.environment as Environment)[entry.key] ?? preset.environment.meanRadiantTempC;
      expect((document.querySelector(`[data-quantity=${entry.key}] input[type=text]`) as HTMLInputElement).value, entry.key).toBe(formatNumber(entry.shown(stored), entry.digits, "en"));
    }
    fireEvent.change(screen.getByRole("slider", { name: "Solar irradiance (W/m²), slider" }), { target: { value: String(preset.environment.solarWM2 === 500 ? 510 : 500) } });
    expect(select.value).toBe("custom");
    expect(select.selectedOptions[0]!.textContent).toBe(`Changed by hand, starting from: ${preset.title.en}`);
    expect(document.querySelector("[data-environment-status]")!.getAttribute("data-environment-status")).toBe("custom");
    fireEvent.click(screen.getByRole("button", { name: "Back to the reference case" }));
    expect(log.at(-1)).toEqual({ preset: null, environment: {} });
    expect(select.value).toBe("own");
    expect(environmentOf(walker, null, {})).toMatchObject({ environment: own, own: true, changed: false });
  });
});

describe("🔁️ what solves again and what only redraws", () => {
  it("keeps the solve for a mark detail and reopens the session for wind and conditions, the field following the wind", async () => {
    const host = document.createElement("div");
    document.body.append(host);
    const phases: string[] = [];
    const flows: (FlowField | null)[] = [];
    const options: FlowStageOptions = { asset: walker, query: { kind: "pose", pose: "stand" }, environment: own, conditions: { maxAgeS: 2 }, detail: {}, clock: null, layers: ["streams", "tracers"], theme: "light", range: { mode: "auto" }, scale: 100, language: "en", projection: () => null, onFlow: (flow) => flows.push(flow), onStatus: (status) => phases.push(status.phase) };
    const stage = new FlowStage(host, options);
    const layer = host.querySelector<HTMLElement>("[data-flow]")!;
    const ready = (): Promise<void> => waitFor(() => expect(layer.getAttribute("data-flow")).toBe("ready"), { timeout: LONG, interval: 20 });
    await ready();
    expect(stage.flow!.ambient).toMatchObject({ windMS: own.windMS, windDirectionDeg: own.windDirectionDeg });
    expect(layer.getAttribute("data-wind")).toBe(`${own.windMS} ${own.windDirectionDeg}`);
    expect(layer.querySelector("[role=status]")!.textContent).toMatch(/ Wind 0\.2 m\/s, backwards, behind the asset \(180°\)\.$/u);
    const first = stage.flow;
    phases.length = 0;
    flows.length = 0;
    stage.update({ detail: { spacingMm: 6, exposureS: 1 } });
    stage.update({ layers: ["streams"], theme: "dark", scale: 50 });
    expect(layer.getAttribute("data-flow")).toBe("ready");
    expect(stage.flow).toBe(first);
    expect(phases).toEqual([]);
    expect(flows).toEqual([]);
    stage.update({ environment: { ...own, windMS: 1.5, windDirectionDeg: 90 } });
    expect(flows).toEqual([null]);
    expect(layer.getAttribute("data-flow")).toBe("developing");
    await ready();
    expect(stage.flow!.ambient).toMatchObject({ windMS: 1.5, windDirectionDeg: 90 });
    expect(flows.at(-1)).toBe(stage.flow);
    expect(layer.querySelector("[role=status]")!.textContent).toMatch(/ Wind 1\.5 m\/s, towards the asset's left \(90°\)\.$/u);
    const windy = stage.flow;
    stage.update({ conditions: { maxAgeS: 2, seed: 5, turbulence: { ...DEFAULT_TURBULENCE, intensity: 0.4 } } });
    expect(layer.getAttribute("data-flow")).toBe("developing");
    await ready();
    expect(stage.flow).not.toBe(windy);
    expect(stage.flow!.ambient).toMatchObject({ windMS: 1.5, windDirectionDeg: 90 });
    stage.update({ environment: { ...own, windMS: 0 }, language: "de" });
    await ready();
    expect(layer.querySelector("[role=status]")!.textContent).toMatch(/ Kein Wind\.$/u);
    stage.dispose();
    expect(host.querySelector("[data-flow]")).toBeNull();
  }, LONG);

  it("turns the wind of the viewer's flow from the dial", async () => {
    const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
    const patches: Partial<ViewerState>[] = [];
    function Stateful(): ReactElement {
      const [viewer, setViewer] = useState<ViewerState>({ ...DEFAULT_VIEWER, view: "thermal", selection: { kind: "pose", pose: "stand" }, flowSettings: { lifeS: 2 } });
      return (
        <ArchiveProvider language="en" t={translator("en")} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>
          <Viewer asset={walker} viewer={viewer} onChange={(patch) => { patches.push(patch); setViewer((current) => ({ ...current, ...patch })); }} />
        </ArchiveProvider>
      );
    }
    const { container } = render(<Stateful />);
    const wind = (): string | null => container.querySelector("[data-flow]")?.getAttribute("data-wind") ?? null;
    await waitFor(() => expect(container.querySelector("[data-flow]")?.getAttribute("data-flow")).toBe("ready"), { timeout: LONG, interval: 50 });
    expect(wind()).toBe(`${own.windMS} ${own.windDirectionDeg}`);
    expect(screen.getByRole("form", { name: "Viewer controls" })).toBeTruthy();
    const dial = screen.getByRole("slider", { name: "Wind direction and speed" });
    expect(dial.getAttribute("data-wind-dial")).toBe("elevation");
    fireEvent.keyDown(dial, { key: "ArrowLeft", shiftKey: true });
    fireEvent.keyDown(dial, { key: "ArrowUp", shiftKey: true });
    expect(patches.at(-1)).toEqual({ environment: { windDirectionDeg: 135, windMS: 1.2 } });
    await waitFor(() => expect(wind()).toBe("1.2 135"), { timeout: LONG, interval: 50 });
    await waitFor(() => expect(container.querySelector("[data-flow]")!.getAttribute("data-flow")).toBe("ready"), { timeout: LONG, interval: 50 });
    expect(container.querySelector("[data-flow] [role=status]")!.textContent).toMatch(/ Wind 1\.2 m\/s, backwards and towards the asset's left \(135°\)\.$/u);
    expect(screen.getByRole("checkbox", { name: "Stream lines" })).toBeTruthy();
  }, LONG);
});

describe("🧩️ the custom element and the export request", () => {
  type Figure = HTMLElement & { readonly flow: FlowField | null };
  const make = (attributes: Record<string, string>): Figure => {
    const element = document.createElement("staffage-figure") as Figure;
    for (const [key, value] of Object.entries(attributes)) element.setAttribute(key, value);
    document.body.append(element);
    return element;
  };
  const base = { asset: "reference-walker", view: "thermal", scale: "100", lang: "de", pose: "stand", "flow-life": "2" };
  const readyIn = (element: Figure, count = 1): Promise<void> => waitFor(() => {
    const layers = [...element.shadowRoot!.querySelectorAll("[data-flow]")];
    expect(layers).toHaveLength(count);
    expect(layers.every((layer) => layer.getAttribute("data-flow") === "ready")).toBe(true);
  }, { timeout: LONG, interval: 50 });

  it("draws no flow without the attribute and the flow of its attributes with it, following a changed wind direction", async () => {
    const plain = make({ ...base, viewpoint: "elevation" });
    await new Promise((done) => setTimeout(done, 0));
    expect(plain.shadowRoot!.querySelector("[data-flow]")).toBeNull();
    const narrow = Number(plain.shadowRoot!.querySelector(".figure")!.getAttribute("data-width-mm"));
    const events: (FlowField | null)[] = [];
    const element = make({ ...base, viewpoint: "elevation", flow: "", "wind-speed": "1.5", "wind-direction": "90", humidity: "80", turbulence: "25", "flow-seed": "4", "flow-spacing": "5" });
    element.addEventListener("staffage-flow", (event) => events.push((event as CustomEvent<{ flow: FlowField | null }>).detail.flow));
    await readyIn(element);
    const root = element.shadowRoot!;
    expect(Number(root.querySelector(".figure")!.getAttribute("data-width-mm"))).toBeGreaterThan(narrow);
    expect(root.querySelector("#semio-flow-style")).not.toBeNull();
    expect(root.querySelector<HTMLElement>("[data-flow]")!.lang).toBe("de");
    expect(element.flow!.ambient).toMatchObject({ windMS: 1.5, windDirectionDeg: 90, relativeHumidity: 0.8, airTempC: own.airTempC });
    expect(events.at(-1)).toBe(element.flow);
    expect(root.querySelector("[data-flow] [role=status]")!.textContent).toMatch(/ Wind 1,5 m\/s, zur linken Seite des Assets \(90°\)\.$/u);
    const layer = root.querySelector("[data-flow]");
    element.setAttribute("flow-spacing", "3");
    await new Promise((done) => setTimeout(done, 0));
    expect(root.querySelector("[data-flow]")).toBe(layer);
    expect(layer!.getAttribute("data-flow")).toBe("ready");
    element.setAttribute("wind-direction", "270");
    await waitFor(() => expect(element.flow?.ambient.windDirectionDeg).toBe(270), { timeout: LONG, interval: 50 });
    expect(root.querySelector("[data-flow]")).toBe(layer);
    const preset = ENVIRONMENT_PRESETS[0]!;
    element.setAttribute("environment", preset.id);
    element.removeAttribute("humidity");
    await waitFor(() => expect(element.flow?.ambient.airTempC).toBe(preset.environment.airTempC), { timeout: LONG, interval: 50 });
    expect(element.flow!.ambient).toMatchObject({ relativeHumidity: preset.environment.relativeHumidity, pressurePa: preset.environment.pressurePa, windMS: 1.5, windDirectionDeg: 270 });
    element.setAttribute("flow", "none");
    await waitFor(() => expect(root.querySelector("[data-flow]")).toBeNull());
    expect(element.flow).toBeNull();
    expect(events.at(-1)).toBeNull();
  }, LONG);

  it("mounts the same flow layer in every cell of its three-view sheet and in its 3D model", async () => {
    const sheet = make({ ...base, viewpoint: "all", flow: "streams tracers", "wind-speed": "0.8", "wind-direction": "45" });
    await readyIn(sheet, 3);
    expect([...sheet.shadowRoot!.querySelectorAll("figure[data-viewpoint]")].map((cell) => cell.querySelector("[data-flow]") !== null)).toEqual([true, true, true]);
    expect(sheet.flow!.ambient).toMatchObject({ windMS: 0.8, windDirectionDeg: 45 });
    sheet.remove();
    const model = make({ ...base, viewpoint: "model", flow: "all", "wind-speed": "2", "wind-direction": "180", "wind-profile": "logarithmic", "reference-height": "2" }) as Figure & { readonly stage: { readonly options?: unknown } | null };
    await waitFor(() => expect(model.stage).not.toBeNull(), { timeout: LONG, interval: 50 });
  }, LONG);

  it("asks for a download with the conditions and the mark detail of the screen", () => {
    const request = { view: "thermal", viewpoint: "elevation", scale: { denominator: 100 }, language: "en", flow: ALL_FLOW_LAYERS } as const;
    expect(depictionOptions(request)).toMatchObject({ flow: ALL_FLOW_LAYERS, flowDevelopS: FLOW_DEVELOP_S });
    expect(depictionOptions(request).flowConditions).toBeUndefined();
    const air = depictionOptions({ ...request, flowConditions: flowConditionsOf({ lifeS: 6, seed: 3, profile: "logarithmic" }), flowDetail: { spacingMm: 4 } });
    expect(air).toMatchObject({ flowDevelopS: 6, flowConditions: { maxAgeS: 6, seed: 3, boundaryLayer: { z0M: 0.03, referenceHeightM: 10 } }, flowDetail: { spacingMm: 4 } });
    expect(depictionOptions({ ...request, view: "forces", flowConditions: { seed: 3 } }).flowConditions).toBeUndefined();
  });
});
