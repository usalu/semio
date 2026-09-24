// #region 🧲️Header
/** @emoji 🎭️ Per-surface verdicts of one browser-actor patch offer on the shell (🎫️ 26/09/23 C10, audit G-P1-4): the
 * neutral corpus drives `browserActorPanelKeysV1` and `applyBrowserActorUiPatchesV1` step by step, and Ajv checks
 * every verdict the shell answers against the private patch-handoff schema the worker's reader enforces. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import type { AppPanelTabDefinition, UiPatch } from "@semio-tech/framework";
import { applyBrowserActorUiPatchesV1, browserActorPanelKeysV1, type BrowserActorUiStoresV1 } from "../../🟦️.tsx";
import corpus from "../../🧫️fixtures/🎭️browser-actor-panels/🔣️.json";
import schema from "../../../../../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧬️schema/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Tests
type Step = { readonly name: string; readonly retained: boolean; readonly patches: readonly UiPatch[]; readonly verdicts: readonly unknown[]; readonly stored: boolean; readonly panelsAdded: boolean; readonly revisions: Readonly<Record<string, number>> };
const fixture = corpus as unknown as { readonly windowKey: string; readonly panelTabs: readonly AppPanelTabDefinition[]; readonly panelKeys: readonly string[]; readonly steps: readonly Step[] };

describe("browser actor panels", () => {
  it("derives the rendered panel keys from the app's bodied panel-tab leaves", () => {
    expect([...browserActorPanelKeysV1({ panelTabs: [...fixture.panelTabs] })]).toEqual(fixture.panelKeys);
  });

  it("answers one schema-valid verdict per surface and keeps every other surface's frame", () => {
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    const validVerdict = ajv.getSchema(`${schema.$id}#/definitions/verdict`)!;
    const panelKeys = browserActorPanelKeysV1({ panelTabs: [...fixture.panelTabs] });
    let retained: BrowserActorUiStoresV1 | null = null;
    for (const step of fixture.steps) {
      const applied = applyBrowserActorUiPatchesV1(step.patches, fixture.windowKey, panelKeys, step.retained ? retained : null);
      expect(applied.verdicts, step.name).toEqual(step.verdicts);
      for (const verdict of applied.verdicts) expect(validVerdict(verdict), `${step.name}: ${JSON.stringify(validVerdict.errors)}`).toBe(true);
      expect(applied.stores !== null, step.name).toBe(step.stored);
      expect(applied.panelsAdded, step.name).toBe(step.panelsAdded);
      const stores = applied.stores;
      const revisions = stores === null ? {} : Object.fromEntries([[fixture.windowKey, stores.window.getRevisionSnapshot()], ...[...stores.panels].map(([key, store]) => [key, store.getRevisionSnapshot()])]);
      expect(revisions, step.name).toEqual(step.revisions);
      for (const [key, revision] of Object.entries(step.revisions)) if (revision === 0) expect((key === fixture.windowKey ? stores!.window : stores!.panels.get(key)!).getState().root, `${step.name}: ${key} reset`).toBeNull();
      if (stores !== null) retained = stores;
    }
  });
});
//#endregion 🧪️Tests
