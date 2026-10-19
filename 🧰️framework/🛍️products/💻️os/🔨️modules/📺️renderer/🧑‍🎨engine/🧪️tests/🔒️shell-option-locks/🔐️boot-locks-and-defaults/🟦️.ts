/** 🔐️ The boot lock resolution, merge and default tests. */
import { expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { initialShellState, mergeShellLockSources, resolveBootExampleId, resolveShellDefaults, resolveShellLocks } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🧷 Registers the locale, appearance and terminology resolution, lock merge, env default and boot example tests. */
export function registerBootLocksAndDefaultsTests(): void {
  it("resolves valid locale/appearance and falls back with a warning on invalid values while staying locked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    expect(resolveShellLocks({ locale: "de" })).toMatchObject({ locale: "de" });
    expect(resolveShellLocks({ locale: "fr" })).toMatchObject({ locale: "en" });
    expect(resolveShellLocks({ appearance: "dark" })).toMatchObject({ appearance: "dark" });
    expect(resolveShellLocks({ appearance: "bogus" })).toMatchObject({ appearance: "system" });
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });
  it("accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot)", () => {
    expect(resolveShellLocks({ terminology: "reuse" })).toMatchObject({ terminology: "reuse" });
    expect(resolveShellLocks({ terminology: "some-app-declared-id" })).toMatchObject({ terminology: "some-app-declared-id" });
    expect(resolveShellLocks({ terminology: "" })).toEqual({});
  });
  it("returns an empty object for undefined locks", () => {
    expect(resolveShellLocks(undefined)).toEqual({});
  });
  it("initialShellState applies locked values over stored/default prefs", () => {
    const state = initialShellState({ locale: "en", plugins: [], locks: { exampleId: "concrete-forest", locale: "de", terminology: "reuse", themeId: "semio", appearance: "dark" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    expect(state.uiPrefs.uiLocale).toBe("de");
    expect(state.uiPrefs.uiTerminology).toBe("reuse");
    expect(state.uiPrefs.uiThemeId).toBe("semio");
    expect(state.uiPrefs.uiAppearance).toBe("dark");
  });
  it("mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key", () => {
    expect(mergeShellLockSources({ locale: "de", terminology: "reuse" }, { locale: "en", themeId: undefined })).toEqual({ locale: "en", terminology: "reuse" });
    expect(mergeShellLockSources({ locale: "de" }, undefined)).toEqual({ locale: "de" });
    expect(mergeShellLockSources(undefined, { locale: "en" })).toEqual({ locale: "en" });
    expect(mergeShellLockSources(undefined, undefined)).toBeUndefined();
  });
  it("resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking", () => {
    const brand = { id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", defaults: { exampleId: "concrete-forest" } };
    expect(resolveShellDefaults(brand, { exampleId: "nakagin-capsule-tower" })).toEqual({ exampleId: "nakagin-capsule-tower" });
    expect(resolveShellDefaults(brand, undefined)).toEqual({ exampleId: "concrete-forest" });
    expect(resolveShellDefaults(undefined, undefined)).toEqual({ exampleId: undefined });
    const state = initialShellState({ locale: "en", plugins: [], defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    const locked = initialShellState({ locale: "en", plugins: [], locks: { exampleId: "nakagin-capsule-tower" }, defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(locked.layout.activeExampleId).toBe("nakagin-capsule-tower");
  });
  it("resolveBootExampleId seeds the first registered example when nothing is active or defaulted", () => {
    const options = [{ id: "hexagonal-mushroom-column" }, { id: "rectangle-extrude-volume" }, { id: "sphere-cut-with-torus" }];
    expect(resolveBootExampleId("", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", options, "sphere-cut-with-torus")).toBe("sphere-cut-with-torus");
    expect(resolveBootExampleId("rectangle-extrude-volume", options, "sphere-cut-with-torus")).toBe("rectangle-extrude-volume");
    expect(resolveBootExampleId("missing", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", [])).toBe("");
  });
}
