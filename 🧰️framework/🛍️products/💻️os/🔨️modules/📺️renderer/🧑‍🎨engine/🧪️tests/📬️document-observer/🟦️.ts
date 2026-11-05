/** 📬️ Observed local readback also reaches the actual active document handler. */
import Ajv from "ajv";
import { expect, it, vi } from "vitest";
import { encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { handleDocumentEventMessage, routeLocalDocumentObserverV1 } from "../../🧱️elements/🏛️ShellHost/🧵️backbone-worker/📬️document-event-messages/🟦️.ts";
import fixture from "./🔣️.json";

it("validates the neutral observer routing corpus", () => { expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true); });
for (const row of fixture.cases) it(row.name, () => {
  const status = vi.fn(), archive = vi.fn(), dispatch = vi.fn();
  const owners = { current: new Map([[row.observerClient, { status, archive }]]) };
  const sessions = { current: new Map(row.activeClient === null ? [] : [["document", { clientInstanceId: row.activeClient }]]) };
  const message = { kind: "event", documentId: "document", clientInstanceId: row.messageClient, event: { kind: "status", ...row.status } };
  const consumed = routeLocalDocumentObserverV1({ localDocumentOwnersRef: owners, openDocumentSessionsRef: sessions, message } as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0]);
  expect(consumed).toBe(row.consumed);
  expect(status.mock.calls).toEqual(row.observed ? [[row.persisted]] : []);
  if (!consumed) handleDocumentEventMessage({ openDocumentSessionsRef: sessions, dispatch, message } as unknown as Parameters<typeof handleDocumentEventMessage>[0]);
  if (row.dispatched) {
    expect(dispatch).toHaveBeenCalledWith({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: "document", status: row.status });
    expect(Object.hasOwn(dispatch.mock.calls[0]![0].status, "writeFailure")).toBe(Object.hasOwn(row.status, "writeFailure"));
  } else expect(dispatch).not.toHaveBeenCalled();
});
it("delivers unchanged archive bytes to the observer and real ordinary archive bootstrap", async () => {
  const pack = { parent_pack: [1, 2], parent_spr: [3, 4], members: [] };
  const bytes = encodeDocumentArchiveBytes(pack), observed = vi.fn(), restored = vi.fn(async (..._args: unknown[]) => "held" as const);
  const bootstrap = vi.fn(), dispatch = vi.fn();
  const entry = { documentId: "document", clientInstanceId: "live", plugin: {}, session: { pluginId: "stdio", instanceId: 7, app: { id: "text", label: { en: "Text" } } }, readBack: { archive: restored }, creationMount: null, rejectReady: vi.fn() };
  const sessions = { current: new Map([["document", entry]]) };
  const message = { kind: "event", documentId: "document", clientInstanceId: "live", event: { kind: "documentArchiveReplaced", archive: Array.from(bytes) } };
  const consumed = routeLocalDocumentObserverV1({ localDocumentOwnersRef: { current: new Map([["live", { archive: observed }]]) }, openDocumentSessionsRef: sessions, message } as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0]);
  expect(consumed).toBe(false); expect(observed).toHaveBeenCalledWith(bytes);
  handleDocumentEventMessage({
    openDocumentSessionsRef: sessions, dispatch, message, uiTerminologyRef: { current: "native" }, uiLocaleRef: { current: "en" },
    core: { localDocumentOwnerRegistry: { acquire: () => ({}) }, localSourceRevisions: { acquire: () => null } },
    documentArchivePorts: () => ({}), rebootstrapDiscardedSessionsRef: { current: new Map() }, setBootstrapUiByDocument: bootstrap,
  } as unknown as Parameters<typeof handleDocumentEventMessage>[0]);
  await vi.waitFor(() => expect(bootstrap).toHaveBeenCalledTimes(1));
  expect(restored.mock.calls[0]?.[0]).toEqual(pack);
  expect(entry.rejectReady).not.toHaveBeenCalled();
});
it("leaves non-event messages and unrelated clients to the normal route", () => {
  const status = vi.fn();
  const port = { localDocumentOwnersRef: { current: new Map([["live", { status }]]) }, openDocumentSessionsRef: { current: new Map() }, message: { kind: "stopped" } };
  expect(routeLocalDocumentObserverV1(port as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0])).toBe(false);
  expect(status).not.toHaveBeenCalled();
});

it("requires the scoped runtime key rather than a same-client unscoped entry", () => {
  const message = { kind: "event", documentId: "document", clientInstanceId: "live", scope: { spaceId: "space", documentId: "document" }, event: { kind: "status", persisted: true, pendingMutations: 0, remote: null } };
  const status = vi.fn();
  const port = { localDocumentOwnersRef: { current: new Map([["live", { status }]]) }, openDocumentSessionsRef: { current: new Map([["document", { clientInstanceId: "live" }]]) }, message };
  expect(routeLocalDocumentObserverV1(port as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0])).toBe(true);
  expect(status).toHaveBeenCalledWith(true);
  message.scope.documentId = "different";
  expect(routeLocalDocumentObserverV1(port as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0])).toBe(true);
});
it("accepts status-only observers without consuming active archive delivery", () => {
  const message = { kind: "event", documentId: "document", clientInstanceId: "live", event: { kind: "documentArchiveReplaced", archive: [1, 2] } };
  expect(routeLocalDocumentObserverV1({ localDocumentOwnersRef: { current: new Map([["live", { status: vi.fn() }]]) }, openDocumentSessionsRef: { current: new Map([["document", { clientInstanceId: "live" }]]) }, message } as unknown as Parameters<typeof routeLocalDocumentObserverV1>[0])).toBe(false);
});
