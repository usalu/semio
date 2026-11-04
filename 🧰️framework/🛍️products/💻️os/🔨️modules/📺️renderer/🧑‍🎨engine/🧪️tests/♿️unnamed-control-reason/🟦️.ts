/** ♿️ A surface patch rejected for an unnamed control names that control's key, and a store that rejects it keeps its previous good state byte-identical. */
import { describe, expect, it } from "vitest";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { patchRejectionReasonV1 } from "../../🧱️elements/🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";

const layout = { kind: "leaf", width: "hug", height: "hug" } as const;
const node = (id: number, key: string, component: object, children: number[] = []) => ({ type: "upsert", id, key, component, layout, style: {}, activity: "idle", disabled: false, accessibility: {}, bindings: [], children }) as never;

const patch = (base: number, ops: unknown[]) => ({ surface: "s", baseRevision: base, revision: base + 1, ops: [...ops, { type: "setRoot", id: 0 }] }) as never;
const withAccessibility = (record: Record<string, unknown>, label: string) => ({ ...record, accessibility: { label } });

describe("unnamed control rejection", () => {
  it("names the control key and leaves the previous document untouched", () => {
    const store = new UiDocumentStore("s");
    const good = store.applyPatch(patch(0, [node(0, "root", { type: "container", role: "group" }, [1]), withAccessibility(node(1, "title", { type: "input", value: "x" }) as never, "Title")]));
    expect(good.ok).toBe(true);
    const before = store.getRevisionSnapshot();
    const bad = patch(before, [node(1, "width-control", { type: "slider", value: 1, min: 0, max: 2, step: 1 })]);
    const result = store.applyPatch(bad);
    expect(result.ok).toBe(false);
    if (!result.ok) expect(patchRejectionReasonV1(bad, result.rejection as never)).toBe("invariantViolated:unnamedControl:width-control");
    expect(store.getRevisionSnapshot()).toBe(before);
  });
});
