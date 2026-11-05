/** 🏛️ Actual close hook admission and destruction boundaries. */
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { useCloseDocumentState } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts";
import { createSealedInstanceLedgerV1 } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import type { ProgramRetirementLeaseV1, ProgramRetirementRequestV1 } from "../../🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts";
afterEach(cleanup);
function setup() {
  const events: string[] = [], ledger = createSealedInstanceLedgerV1();
  const handle = { pluginId: "stdio", destroyApp: vi.fn(async (id: number) => { events.push("destroy:" + id); }) };
  const owners = [1, 2].map(instanceId => ({ handle, name: "Draft " + instanceId, session: { pluginId: "stdio", instanceId, app: { id: "txt" }, viewState: {} } }));
  const request = { current: null as unknown as ProgramRetirementRequestV1 };
  const persist = vi.fn(async (owner: typeof owners[number], _signal: AbortSignal) => { events.push("persist:" + owner.session.instanceId); });
  const core = { admitProgramRetirementRef: request, persistProgramDocumentRef: { current: persist }, loadedPluginsRef: { current: [{ handle }] }, sessionWorkRef: { current: { pending: () => 0 } }, sealedInstancesRef: { current: ledger }, documentAttachmentLanesRef: { current: new Map([[handle, new Map(owners.map(owner => [owner.session.instanceId, { drain: async () => { events.push("drain:" + owner.session.instanceId); } }]))]]) } };
  const hook = renderHook(() => useCloseDocumentState({ core, scope: { storage: createMemoryStoragePort() }, openDocumentSessionsRef: { current: new Map() }, shellStateRef: { current: { sync: { syncStatusByDocumentId: {} } } }, closeDocumentRef: { current: () => {} }, resolveSyncTargetSession: () => null, loadedPlugins: [], syncBackboneUri: null } as unknown as Parameters<typeof useCloseDocumentState>[0]));
  const admit = (current = () => true) => request.current(owners as never, current);
  return { ...hook, events, ledger, handle, owners, persist, admit, core };
}
it("keeps every program when the second captured draft cannot save and the person keeps it", async () => {
  const view = setup(); view.persist.mockImplementation(async owner => { view.events.push("persist:" + owner.session.instanceId); if (owner.session.instanceId === 2) throw Error("disk-refused"); });
  let pending!: Promise<ProgramRetirementLeaseV1 | null>;
  await act(async () => { pending = view.admit(); });
  expect(view.result.current.closePrompt?.phase).toBe("confirm"); expect(view.handle.destroyApp).not.toHaveBeenCalled(); expect(view.ledger.size()).toBe(2);
  await act(async () => { view.result.current.closePrompt!.onKeep(); await pending; });
  expect(await pending).toBeNull(); expect(view.ledger.size()).toBe(0); expect(view.events).toEqual(["persist:1", "persist:2"]);
});
it("waits for all approvals before draining or destroying either program", async () => {
  const view = setup(); let admission: ProgramRetirementLeaseV1 | null = null;
  await act(async () => { admission = await view.admit(); });
  expect(view.events).toEqual(["persist:1", "persist:2"]); expect(view.handle.destroyApp).not.toHaveBeenCalled();
  expect(await admission!.retire()).toBe(true);
  expect(view.events).toEqual(["persist:1", "persist:2", "drain:1", "destroy:1", "drain:2", "destroy:2"]);
  expect(await admission!.retire()).toBe(false);
});
it("cancels a pending archive read without waiting for the plugin to answer", async () => {
  const view = setup(); view.persist.mockImplementation(async () => new Promise<void>(() => {}));
  let pending!: Promise<ProgramRetirementLeaseV1 | null>;
  await act(async () => { pending = view.admit(); });
  expect(view.result.current.closePrompt?.phase).toBe("saving");
  await act(async () => { view.result.current.closePrompt!.onKeep(); await pending; });
  expect(await pending).toBeNull(); expect(view.ledger.size()).toBe(0); expect(view.handle.destroyApp).not.toHaveBeenCalled();
});
it("refuses a stale owner after discard and releases its seals", async () => {
  const view = setup(); let held = true;
  view.persist.mockRejectedValue(Error("write-failed"));
  let pending!: Promise<ProgramRetirementLeaseV1 | null>;
  await act(async () => { pending = view.admit(() => held); }); held = false;
  await act(async () => { view.result.current.closePrompt!.onDiscard(); await pending; });
  expect(await pending).toBeNull(); expect(view.handle.destroyApp).not.toHaveBeenCalled(); expect(view.ledger.size()).toBe(0);
});

it("a drain failure releases the still-live owners instead of freezing every editor", async () => {
  const view = setup(); view.core.documentAttachmentLanesRef.current.get(view.handle)!.set(1, { drain: async () => { throw Error("drain-refused"); } });
  let admission: ProgramRetirementLeaseV1 | null = null; await act(async () => { admission = await view.admit(); });
  await expect(admission!.retire()).rejects.toThrow("drain-refused");
  expect(view.handle.destroyApp).not.toHaveBeenCalled(); expect(view.ledger.size()).toBe(0);
});
it("reserves a captured owner across routes until the asynchronous destruction finishes", async () => {
  const view = setup(); let resolve!: () => void;
  view.core.documentAttachmentLanesRef.current.get(view.handle)!.set(1, { drain: async () => new Promise<void>(done => { resolve = done; }) });
  let admission: ProgramRetirementLeaseV1 | null = null; await act(async () => { admission = await view.admit(); });
  expect(await view.admit()).toBeNull(); expect(admission!.current()).toBe(true);
  let retiring!: Promise<boolean>; await act(async () => { retiring = admission!.retire(); });
  admission!.release(); expect(view.ledger.size()).toBe(2); expect(await view.admit()).toBeNull();
  resolve(); expect(await retiring).toBe(true); expect(view.handle.destroyApp).toHaveBeenCalledTimes(2);
});
