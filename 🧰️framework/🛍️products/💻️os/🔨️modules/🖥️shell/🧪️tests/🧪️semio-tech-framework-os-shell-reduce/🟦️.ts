type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { reduce } = dependencies;
  type ShellState = any;

  const { describe, expect, it } = vitest;

  const defaultState = (): ShellState => ({
    revision: 0,
    loadedPlugins: [],
    pluginStatusById: {},
    pluginSupervisorById: {},
    activeSession: null,
    sessionError: null,
    appLabelsOverlay: {},
    actionPaneFoldedByWindow: {},
    actionPaneExpandedByWindow: {},
    stagedActionArgs: {},
    activeUtilityByWindow: {},
    activeToolId: null,
    commandPanelExpanded: null,
    stagedCommandArgs: {},
    panelsVisible: { left: false, right: false, top: false, bottom: false },
    panelsSize: { left: 280, right: 280, top: 280, bottom: 280 },
    panelsPath: { left: [], right: [], top: [], bottom: [] },
    dockOverride: null,
    panelPathMemory: {},
    treeOpenStates: {},
    activeWindowId: null,
    shellLayout: null,
    activeExampleId: "",
    mobilePanelPath: [],
    mobilePanelVisible: false,
    extraWindows: [],
    windowTitlesById: {},
    windowIconsById: {},
    searchOpen: false,
    findOpen: false,
    introductionStepIndex: null,
    introductionAutoStartedKeys: [],
    introductionCompletedInteractions: [],
    dialogStack: [],
    transientNotice: null,
    openWithFocusRole: null,
    activeTutorialId: null,
    uiAppearance: "system",
    uiLayout: "default",
    uiDriverId: "",
    uiCustomDrivers: {},
    uiDriverDraft: null,
    uiLocale: "en",
    uiTerminology: "",
    uiThemeId: "",
    uiCustomThemes: {},
    uiThemeDraft: null,
    uiKeybindingOverrides: {},
    syncBackboneUri: null,
    syncCardKind: null,
    syncDraftPath: "",
    syncStatusByDocument: {},
    inferencePortByDocument: {},
    mergePolicy: "manual",
    conflicts: [],
    selectedConflictId: null,
    storageScope: "memory",
    openingPreferences: {},
  });

  describe("@semio-tech/framework-os-shell reduce", () => {
    it("is pure and increments revision", () => {
      const state = defaultState();
      const result = reduce(state, { type: "setSearchOpen", open: true }, 1000);
      expect(result.ok).toBe(true);
      if (!result.ok) throw new Error("unreachable");
      expect(result.state.revision).toBe(state.revision + 1);
      expect(result.state.searchOpen).toBe(true);
      expect(state.searchOpen).toBe(false); // input untouched
    });

    it("rejects leave state untouched and report a typed error", () => {
      const state = defaultState();
      const result = reduce(state, { type: "selectConflict", conflictId: "missing" }, 1000);
      expect(result.ok).toBe(false);
      if (result.ok) throw new Error("unreachable");
      expect(result.error).toEqual({ kind: "unknownConflict", conflictId: "missing" });
    });

    it("re-derives every shared fixture (Rust/TS parity)", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const fixturesDir = join(here, "🧫️fixtures");
      const files = readdirSync(fixturesDir).filter((name) => name.endsWith(".json"));
      expect(files.length).toBeGreaterThanOrEqual(65);

      for (const file of files) {
        const fixture = JSON.parse(readFileSync(join(fixturesDir, file), "utf8"));
        const result = reduce(fixture.state, fixture.command, 1_700_000_000_000);
        if ("error" in fixture.expected) {
          expect(result.ok, `${fixture.name}: expected an error`).toBe(false);
          if (!result.ok) expect(result.error, `${fixture.name} error mismatch`).toEqual(fixture.expected.error);
        } else {
          expect(result.ok, `${fixture.name}: expected ok`).toBe(true);
          if (result.ok) {
            expect(result.state, `${fixture.name} state mismatch`).toEqual(fixture.expected.state);
            expect(result.events, `${fixture.name} events mismatch`).toEqual(fixture.expected.events);
          }
        }
      }
    });
  });

}
