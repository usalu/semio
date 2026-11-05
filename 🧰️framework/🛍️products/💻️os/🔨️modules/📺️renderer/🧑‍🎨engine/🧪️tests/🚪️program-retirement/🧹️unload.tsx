/** 🧹️ Native artifact drafts participate in the actual browser unload guard. */
import { cleanup, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import Ajv from "ajv";
import fixture from "./🧹️unload.json";
import { useShellProgramUnloadGuardV1 } from "../../🧱️elements/🏛️ShellHost/🚪️program-retirement/🧹️unload/🟦️.tsx";
import { panelJsonFromState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
afterEach(cleanup);
it("validates neutral unload scenarios", () => expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true));
for (const row of fixture.cases) it(row.id, () => {
  const app = { id: "txt", io: { artifactSchema: row.artifact ? "stdio.txt" : "" } }, handle = { pluginId: "stdio" };
  const session = { pluginId: "stdio", instanceId: 7, app, viewState: {} };
  const statuses = row.status === "missing" ? {} : { document: { persisted: row.status === "saved", pendingMutations: row.status === "pending" ? 1 : 0, ...(row.status === "failed" ? { writeFailure: { code: "folder-write-failed" } } : {}) } };
  const attachedSession = { ...session, ...(row.attachmentMismatch === "app" ? { app: { id: "other" } } : {}), ...(row.attachmentMismatch === "instance" ? { instanceId: 8 } : {}) };
  const options = { loaded: [{ handle, manifest: { apps: [app] } }], session, attachments: new Map(row.attached ? [["document", { plugin: handle, session: attachedSession }]] : []), statuses };
  renderHook(() => useShellProgramUnloadGuardV1(options as unknown as Parameters<typeof useShellProgramUnloadGuardV1>[0]));
  const event = new Event("beforeunload", { cancelable: true }); window.dispatchEvent(event);
  expect(event.defaultPrevented).toBe(row.blocked);
  const oracle = row.artifact && (!row.attached || row.attachmentMismatch !== "none") || row.attached && row.status !== "saved";
  expect(new Ajv().validate({ const: oracle }, row.blocked)).toBe(true);
});
it("protects an unbound background editor and removes its listener after it is saved", () => {
  const hostApp = { id: "home" }, app = { id: "txt", io: { artifactSchema: "stdio.txt" } }, hostHandle = { pluginId: "host" }, handle = { pluginId: "stdio" };
  const host = { pluginId: "host", instanceId: 1, app: hostApp, viewState: { panelJson: panelJsonFromState({ activePanelTab: "main", spawnedApps: [{ id: "guest", pluginId: "stdio", appId: "txt", instanceId: 7, label: "Draft", breadcrumb: [] }] }) } };
  const target = { pluginId: "stdio", instanceId: 7, app, viewState: {} };
  const options = { loaded: [{ handle: hostHandle, manifest: { apps: [hostApp] } }, { handle, manifest: { apps: [app] } }], session: host, attachments: new Map(), statuses: {} as Record<string, unknown> };
  const view = renderHook(() => useShellProgramUnloadGuardV1(options as unknown as Parameters<typeof useShellProgramUnloadGuardV1>[0]));
  let event = new Event("beforeunload", { cancelable: true }); window.dispatchEvent(event); expect(event.defaultPrevented).toBe(true);
  options.attachments.set("document", { plugin: handle, session: target }); options.statuses.document = { persisted: true, pendingMutations: 0 }; view.rerender();
  event = new Event("beforeunload", { cancelable: true }); window.dispatchEvent(event); expect(event.defaultPrevented).toBe(false);
});
it("does not mistake the same numeric instance on another handle for an attachment", () => {
  const app = { id: "txt", io: { artifactSchema: "stdio.txt" } }, handle = { pluginId: "stdio" }, other = { pluginId: "stdio" }, session = { pluginId: "stdio", instanceId: 7, app, viewState: {} };
  renderHook(() => useShellProgramUnloadGuardV1({ loaded: [{ handle, manifest: { apps: [app] } }], session, attachments: new Map([["document", { plugin: other, session }]]), statuses: { document: { persisted: true, pendingMutations: 0 } } } as unknown as Parameters<typeof useShellProgramUnloadGuardV1>[0]));
  const event = new Event("beforeunload", { cancelable: true }); window.dispatchEvent(event); expect(event.defaultPrevented).toBe(true);
});
