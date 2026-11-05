/** 🗃️ Real catalog handler and recorder acknowledgment integration. */
import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import { encodePackValue } from "@semio-tech/framework/packs";
import { encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { decodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { admitLocalDocument, LOCAL_CATALOG_CONFIG_SCHEMA, type LocalCatalog } from "@semio-tech/framework/config/mutations";
import { handleLocalCatalogEventMessage } from "../../🧱️elements/🏛️ShellHost/🧵️backbone-worker/📬️document-event-messages/🟦️.ts";
import { useCatalogState } from "../../🧱️elements/🏛️ShellHost/🛂️directory-route/🔶️catalog/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

afterEach(cleanup);
const document = (id: string) => ({ documentId: id, schema: "stdio.txt", name: id, storage: "folder" as const, target: "/data/" + id, admittedAtMs: 1 });
const archive = (catalog: LocalCatalog) => encodeDocumentArchiveBytes({ parent_pack: Array.from(encodePackValue(catalog)), parent_spr: [], members: [] });

function setup() {
  const requests: ReturnType<typeof decodeBackboneWorkerRequest>[] = [];
  const worker = { postMessage: vi.fn((message: { wire: Uint8Array }) => { requests.push(decodeBackboneWorkerRequest(message.wire)); }) };
  const localCatalogRef = { current: { documents: [] } as LocalCatalog };
  const localCatalogClientInstanceIdRef = { current: null as string | null };
  const localDocumentOwnersRef = { current: new Map<string, { status?: (persisted: boolean) => void; archive: (bytes: Uint8Array) => void }>() };
  const setLocalCatalog = vi.fn();
  const core = {
    ensureBackboneWorker: () => worker, localDataDir: "/data", setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef, localDocumentOwnersRef,
    shellActorIdRef: { current: "test" }, handleDirectoryEventPageRef: { current: null }, handleDirectoryBootstrapFailureRef: { current: null },
    backboneWorkerRef: { current: worker }, spaceAdministrationEpochRef: { current: 0 }, setSpaceAdministration: vi.fn(), verifiedSessionAuthority: null, identity: null,
  } as unknown as Parameters<typeof useCatalogState>[0]["core"];
  const view = renderHook(({ dataDir }) => useCatalogState({ core: { ...core, localDataDir: dataDir } }), { initialProps: { dataDir: "/data" } });
  const deliver = (event: unknown, client = localCatalogClientInstanceIdRef.current!) => handleLocalCatalogEventMessage({
    localCatalogClientInstanceIdRef, localCatalogRef, localDocumentOwnersRef, setLocalCatalog,
    message: { kind: "event", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId: client, event },
  } as Parameters<typeof handleLocalCatalogEventMessage>[0]);
  const writes = () => requests.filter(request => request.kind === "send" && request.message.kind === "localMutations");
  const probes = () => requests.filter(request => request.kind === "send" && request.message.kind === "externalChanged");
  return { ...view, deliver, writes, probes, localCatalogRef, localCatalogClientInstanceIdRef, localDocumentOwnersRef, setLocalCatalog };
}

describe("catalog handler and durable recorder", () => {
  it("validates strict status cases with an independent schema", () => { expect(new Ajv().validate(fixture.schema, fixture.statuses)).toBe(true); });
  it("forwards only strict acknowledged status and refuses stale clients", () => {
    const view = setup(), status = vi.fn(), read = vi.fn();
    view.localDocumentOwnersRef.current.set(view.localCatalogClientInstanceIdRef.current!, { status, archive: read });
    for (const row of fixture.statuses) {
      view.deliver({ kind: "status", ...row.status });
      expect(status.mock.lastCall).toEqual([row.expected]);
    }
    const calls = status.mock.calls.length;
    view.deliver({ kind: "status", persisted: true, pendingMutations: 0 }, "stale");
    expect(status).toHaveBeenCalledTimes(calls);
    view.deliver({ kind: "documentArchiveReplaced", archive: Array.from(archive({ documents: [document("a")] })) }, "stale");
    expect(read).not.toHaveBeenCalled();
    expect(view.localCatalogRef.current.documents).toEqual([]);
  });
  it("publishes decoded readback before notifying the observer", () => {
    const view = setup(), catalog = { documents: [document("a")] };
    const read = vi.fn(() => { expect(view.localCatalogRef.current).toEqual(catalog); });
    view.localDocumentOwnersRef.current.set(view.localCatalogClientInstanceIdRef.current!, { archive: read });
    view.deliver({ kind: "documentArchiveReplaced", archive: Array.from(archive(catalog)) });
    expect(read).toHaveBeenCalledTimes(1);
    expect(view.setLocalCatalog).toHaveBeenCalledWith(catalog);
  });
  it("optimistic state and failed statuses cannot acknowledge; writes serialize through actual disk readback", async () => {
    const view = setup();
    let firstDone = false, secondDone = false;
    const first = view.result.current.recordLocalCatalogRef.current(admitLocalDocument(document("a"))).then(value => { firstDone = true; return value; });
    const second = view.result.current.recordLocalCatalogRef.current(admitLocalDocument(document("b"))).then(value => { secondDone = true; return value; });
    await vi.waitFor(() => expect(view.writes()).toHaveLength(1));
    expect(view.localCatalogRef.current.documents.map(row => row.documentId)).toEqual(["a"]);
    expect(firstDone).toBe(false); expect(secondDone).toBe(false);
    const probes = view.probes().length;
    for (const row of fixture.statuses.filter(row => !row.expected)) view.deliver({ kind: "status", ...row.status });
    expect(view.probes()).toHaveLength(probes);
    view.deliver({ kind: "remoteMutations", envelopes: [] });
    expect(firstDone).toBe(false);
    view.deliver({ kind: "documentArchiveReplaced", archive: Array.from(archive({ documents: [document("a"), document("concurrent")] })) });
    expect((await first)?.documents).toHaveLength(2);
    await vi.waitFor(() => expect(view.writes()).toHaveLength(2));
    expect(secondDone).toBe(false);
    view.deliver({ kind: "documentArchiveReplaced", archive: Array.from(archive({ documents: [document("a"), document("b"), document("concurrent")] })) });
    expect((await second)?.documents).toHaveLength(3);
  });
  it("does not move queued writes into a replacement catalog generation", async () => {
    const view = setup();
    const first = view.result.current.recordLocalCatalogRef.current(admitLocalDocument(document("a")));
    const queued = view.result.current.recordLocalCatalogRef.current(admitLocalDocument(document("b")));
    await vi.waitFor(() => expect(view.writes()).toHaveLength(1));
    const previous = view.localCatalogClientInstanceIdRef.current;
    act(() => view.rerender({ dataDir: "/replacement-data" }));
    expect(view.localCatalogClientInstanceIdRef.current).not.toBe(previous);
    expect(await first).toBeNull();
    expect(await queued).toBeNull();
    expect(view.writes()).toHaveLength(1);
    expect(view.localDocumentOwnersRef.current.size).toBe(0);
  });
  it("cancelled recorder never deletes a successor subscriber", async () => {
    const view = setup();
    const pending = view.result.current.recordLocalCatalogRef.current(admitLocalDocument(document("a")));
    await vi.waitFor(() => expect(view.writes()).toHaveLength(1));
    const successor = { archive: vi.fn() }, client = view.localCatalogClientInstanceIdRef.current!;
    view.localDocumentOwnersRef.current.set(client, successor);
    act(() => view.unmount());
    expect(await pending).toBeNull();
    expect(view.localDocumentOwnersRef.current.get(client)).toBe(successor);
  });
});
