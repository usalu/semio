/** 🚗️ Language-neutral oracle for the seven-axis driver draft lifecycle. */
import { describe, expect, test } from "vitest";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const frameworkRoot = join(engineRoot, "..", "..", "..", "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🚗️driver-editor", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🚗️driver-editor", "🔣️.json"), "utf8"));

type Driver = Record<string, string>;
type State = { selected: string; drivers: Record<string, Driver>; draft: Driver | null; saveLabel: string };

const slug = (label: string): string => label.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-+|-+$)/g, "");
const live = (state: State): Driver => state.draft ?? state.drivers[state.selected] ?? state.drivers.default;
const edit = (state: State, key: string, value: string): State => ({ ...state, draft: { ...live(state), [key]: value } });
const select = (state: State, selected: string): State => ({ ...state, selected, draft: null });
const save = (state: State, label: string): State => {
  const id = `custom.${slug(label)}`;
  const saved = { ...live(state), id, label: label.trim() };
  return { ...state, selected: id, drivers: { ...state.drivers, [id]: saved }, draft: null, saveLabel: "" };
};
const remove = (state: State, id: string): State => {
  const drivers = { ...state.drivers };
  delete drivers[id];
  return { ...state, drivers, selected: state.selected === id ? "default" : state.selected, draft: null };
};

describe("🚗️ driver editor contract", () => {
  test("the shared fixture satisfies the Ajv schema oracle", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("the independent reducer matches edit, selection, save and delete vectors", () => {
    let state: State = { selected: "default", drivers: structuredClone(fixture.builtins), draft: null, saveLabel: "" };
    const editVector = fixture.transitions.edit;
    state = edit(state, editVector.key, editVector.value);
    expect(state.draft !== null).toBe(editVector.dirty);
    expect(live(state)[editVector.key]).toBe(editVector.liveValue);
    expect(state.drivers.default[editVector.key]).toBe(editVector.persistedValue);

    state = select(state, fixture.transitions.select.selected);
    expect(state.draft).toBeNull();
    expect(live(state).tooltips).toBe(fixture.transitions.select.liveValue);

    state = edit(state, "labelTier", "beginner");
    state = save(state, fixture.transitions.save.label);
    expect(state.selected).toBe(fixture.transitions.save.selected);
    expect(state.drivers[fixture.transitions.save.id].labelTier).toBe("beginner");
    expect(state.draft).toBeNull();
    expect(state.saveLabel).toBe(fixture.transitions.save.saveLabel);

    state = remove(state, fixture.transitions.delete.id);
    expect(state.selected).toBe(fixture.transitions.delete.selected);
    expect(state.drivers[fixture.transitions.delete.id]).toBeUndefined();
    expect(state.draft).toBeNull();
  });

  test("React publishes the same axes, ids and canonical lifecycle", () => {
    const driver = readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🧱️elements", "🚗️UiDriver", "🟦️.tsx"), "utf8");
    const panel = readFileSync(join(engineRoot, "🧱️elements", "📌️ChromePanels", "🟦️.tsx"), "utf8");
    const host = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    for (const axis of fixture.axes as Array<{ key: string; options: string[] }>) {
      expect(panel).toContain(`driverAxisSelectRow("${axis.key}"`);
      for (const option of axis.options) expect(driver).toContain(`"${option}"`);
    }
    for (const id of fixture.controls as string[]) {
      if (!fixture.axes.some((axis: { key: string }) => id === `framework.settings.driver.${axis.key}`)) expect(panel).toContain(id);
    }
    expect(host).toContain("uiDriverBase = uiDriverDraft ?? uiDriver");
    expect(host).toContain('dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });');
    expect(host).toContain("commitUiPreference(setCustomDriver(id, canonicalUiDriver(saved)))");
    expect(host).toContain('commitUiPreference(setDriver(DEFAULT_UI_DRIVER.id))');
  });
});
