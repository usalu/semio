// #region 🧲️Header
/** 🎭️ Per-surface verdicts of one browser-actor patch offer on the shell (🎫️ 26/09/23 C10, audit G-P1-4 and
 * S15 finding b): the neutral corpus drives `browserActorPanelKeysV1` and `applyBrowserActorUiPatchesV1` step by step
 * over a document shown in two windows plus its panels and reserved refresh sections, and Ajv checks every verdict the
 * shell answers against the private patch-handoff schema the worker's reader enforces. The section values the shell
 * decodes (C13 P1) are compared with fast-deep-equal, and the host's section keys with the worker's visible-surface
 * contract, so the two ends of the actor can never disagree about which sections exist. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import type { AppPanelTabDefinition, UiPatch } from "@semio-tech/framework";
import equal from "fast-deep-equal";
import type { PluginUiRefreshRequest, PluginViewState } from "@semio-tech/framework";
import { applyBrowserActorUiPatchesV1, BROWSER_ACTOR_SECTION_KEYS, browserActorPanelKeysV1, browserActorSectionValuesV1, withoutUiRefreshSectionsV1, type BrowserActorUiStoresV1 } from "../../🟦️.tsx";
import visibleSurfaces from "../../../../../../🏪️store/👷️worker/🪟️visible-surfaces/🔣️.json";
import corpus from "../../🧫️fixtures/🎭️browser-actor-panels/🔣️.json";
import schema from "../../../../../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧬️schema/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Tests
type Step = { readonly name: string; readonly retained: boolean; readonly patches: readonly UiPatch[]; readonly verdicts: readonly unknown[]; readonly stored: boolean; readonly surfacesAdded: boolean; readonly revisions: Readonly<Record<string, number>>; readonly sectionsChanged: boolean; readonly sections: Readonly<Record<string, unknown>> };
const fixture = corpus as unknown as { readonly windowKeys: readonly string[]; readonly panelTabs: readonly AppPanelTabDefinition[]; readonly panelKeys: readonly string[]; readonly steps: readonly Step[] };

describe("browser actor panels", () => {
  it("derives the rendered panel keys from the app's bodied panel-tab leaves", () => {
    expect([...browserActorPanelKeysV1({ panelTabs: [...fixture.panelTabs] })]).toEqual(fixture.panelKeys);
  });

  it("answers one schema-valid verdict per surface of every window and panel and keeps every other surface's frame", () => {
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    const validVerdict = ajv.getSchema(`${schema.$id}#/definitions/verdict`)!;
    const panelKeys = browserActorPanelKeysV1({ panelTabs: [...fixture.panelTabs] }),
      windowKeys = new Set(fixture.windowKeys);
    let retained: BrowserActorUiStoresV1 | null = null;
    for (const step of fixture.steps) {
      const applied = applyBrowserActorUiPatchesV1(step.patches, windowKeys, panelKeys, step.retained ? retained : null);
      expect(applied.verdicts, step.name).toEqual(step.verdicts);
      for (const verdict of applied.verdicts) expect(validVerdict(verdict), `${step.name}: ${JSON.stringify(validVerdict.errors)}`).toBe(true);
      expect(applied.stores !== null, step.name).toBe(step.stored);
      expect(applied.surfacesAdded, step.name).toBe(step.surfacesAdded);
      expect(applied.sectionsChanged, step.name).toBe(step.sectionsChanged);
      const stores = applied.stores;
      const revisions = stores === null ? {} : Object.fromEntries([...stores.windows, ...stores.panels, ...stores.sections].map(([key, store]) => [key, store.getRevisionSnapshot()]));
      expect(revisions, step.name).toEqual(step.revisions);
      for (const [key, revision] of Object.entries(step.revisions)) if (revision === 0) expect((stores!.windows.get(key) ?? stores!.panels.get(key) ?? stores!.sections.get(key)!).getState().root, `${step.name}: ${key} reset`).toBeNull();
      const decoded = stores === null ? {} : Object.fromEntries([...browserActorSectionValuesV1(stores.sections, "corpus")].map(([key, { value }]) => [key, value]));
      expect(equal(decoded, step.sections), `${step.name}: ${JSON.stringify(decoded)}`).toBe(true);
      if (stores !== null) retained = stores;
    }
  });

  it("renders exactly the sections the worker's visible-surface contract announces", () => {
    expect([...BROWSER_ACTOR_SECTION_KEYS].sort()).toEqual(visibleSurfaces.sections.map(({ bodyKey }) => bodyKey).sort());
  });

  it("strips the reserved sections from an actor-served session's local refresh, and asks nothing when nothing else is left", () => {
    const viewState = {} as PluginViewState;
    const request: PluginUiRefreshRequest = { viewState, windows: [{ key: "note-navigator", bodyKey: "navigator" }], panels: [], engagements: { hash: "e" }, measures: {}, tools: {}, catalogue: {}, labels: { hash: "l" } };
    const stripped = withoutUiRefreshSectionsV1(request);
    expect(stripped === null ? null : Object.fromEntries(Object.entries(stripped).filter(([, value]) => value !== undefined))).toEqual({ viewState, windows: request.windows, panels: [], labels: { hash: "l" } });
    expect(withoutUiRefreshSectionsV1({ viewState, windows: [], panels: [], engagements: {}, measures: {}, tools: {}, catalogue: {} })).toBeNull();
    expect(withoutUiRefreshSectionsV1(null)).toBeNull();
  });
});
//#endregion 🧪️Tests
