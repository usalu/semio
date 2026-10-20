/** 🎓️ The introduction replay, ephemeral brand and os command lock tests. */
import { shouldStartIntroduction } from "@semio-tech/ui/introduction";
import { expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { initialShellState, buildOsCommands, dispatchOsCommand, shouldPersistIntroductionSeen, shouldReplayIntroductionOnLoad, isEphemeralShellBrand, clearDurableShellStorage } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🛂 Registers the introduction replay and start, ephemeral brand, os command omission and locked dispatch tests. */
export function registerIntroductionAndOsCommandTests(): void {
  it("shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh", () => {
    expect(shouldReplayIntroductionOnLoad(undefined)).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain", replayIntroductionOnLoad: false })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "plain", windowTitle: "Plain" })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(false);
  });
  it("shouldStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedInSession: false };
    expect(shouldStartIntroduction(base)).toBe(true);
    const dismissedAppIds = new Set(["puzzle-3d-play"]);
    const dismissed = { ...base, dismissedInSession: dismissedAppIds.has(base.appId) };
    expect(shouldStartIntroduction(dismissed)).toBe(false);
    expect(shouldStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    expect(shouldStartIntroduction({ ...dismissed, appId: "puzzle-3d-view", dismissedInSession: dismissedAppIds.has("puzzle-3d-view") })).toBe(true);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    expect(shouldStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });
  it("shouldStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedInSession:false };
    expect(shouldStartIntroduction(base)).toBe(true);
    const dismissed = { ...base, dismissedInSession:true };
    expect(shouldStartIntroduction(dismissed)).toBe(false);
    expect(shouldStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    expect(shouldStartIntroduction({ ...dismissed, appId: "puzzle-3d-view", dismissedInSession:false })).toBe(true);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    expect(shouldStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });
it("isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only", () => {
    expect(isEphemeralShellBrand(undefined)).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain", ephemeral: true })).toBe(true);
    const ephemeralState = initialShellState({
      locale: "en",
      plugins: [],
      locks: { locale: "de", terminology: "reuse", themeId: "semio" },
      defaults: { exampleId: "concrete-forest" },
      storage: createMemoryStoragePort(),
    });
    expect(ephemeralState.layout.activeExampleId).toBe("concrete-forest");
    expect(ephemeralState.uiPrefs.uiLocale).toBe("de");
    expect(ephemeralState.uiPrefs.uiTerminology).toBe("reuse");
    expect(ephemeralState.uiPrefs.uiThemeId).toBe("semio");
    expect(ephemeralState.uiPrefs.uiAppearance).toBe("system");
    expect(ephemeralState.uiPrefs.uiLayout).toBe("desktop");
    expect(ephemeralState.uiPrefs.uiCustomThemes).toEqual({});
    expect(ephemeralState.layout.dockOverride).toBeNull();
    expect(ephemeralState.layout.shellLayout).toBeNull();
    localStorage.setItem("ui.chrome.appearance", "dark");
    localStorage.setItem("semio.os.dock", "{}");
    localStorage.setItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play", "true");
    clearDurableShellStorage();
    expect(localStorage.getItem("ui.chrome.appearance")).toBeNull();
    expect(localStorage.getItem("semio.os.dock")).toBeNull();
    expect(localStorage.getItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play")).toBeNull();
  });
  it("buildOsCommands omits only the commands for locked prefs", () => {
    const ids = buildOsCommands([], [], false, { locale: "de", appearance: "dark" }).map((c) => c.id);
    expect(ids).not.toContain("os.setLocale");
    expect(ids).not.toContain("os.setAppearance");
    expect(ids).toContain("os.setTerminology");
    expect(ids).toContain("os.setThemeId");
  });
  it("dispatchOsCommand is a no-operation for a locked pref even if invoked directly", () => {
    const dispatch = vi.fn();
    const commit = vi.fn();
    dispatchOsCommand("os.setLocale", { locale: "de" }, commit, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never, { locale: "en" });
    expect(commit).not.toHaveBeenCalled();
    expect(dispatch).not.toHaveBeenCalled();
  });
}
