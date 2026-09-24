import { isDeepStrictEqual } from "node:util";
import Ajv from "ajv";
import React from "react";
import { describe, expect, it, vi } from "vitest";
import { createDevPluginSource } from "@semio-tech/framework";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { rendererResidentLedger } from "../../💾️resident/🟦️.ts";
import residentFixture from "../../💾️resident/🧫️fixtures/🔣️.json";
import {
  BootstrapStatusNotice,
  ExecutionTargetStatusNotice,
  reduceBootstrapUiState,
  resolveRequiredHostApps,
  type BootstrapUiStatus,
} from "../../🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx";
import hostBootstrapFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🪪️host-bootstrap/🔣️.json";
import { pluginAvailabilityRouteV1, pluginInstallBandTextV1, pluginInstallProgressTotalV1 } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { upsertLoadedProgramV1, type LoadedProgramState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import valueResidentSchema from "../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json" with { type: "json" };

describe("renderer quick contracts", () => {
  it("validates the language-neutral renderer resident capacity with the Node oracle", () => {
    const capacity = rendererResidentLedger().capacity;
    expect(isDeepStrictEqual(capacity, residentFixture.capacity)).toBe(true);
    expect(capacity.bytes - capacity.control.bytes).toBe(residentFixture.data.bytes);
    expect(capacity.slots - capacity.control.slots).toBe(residentFixture.data.slots);
    expect(capacity.owners - capacity.control.owners).toBe(residentFixture.data.owners);
  });

  it("validates the renderer resident capacity against the owned RendererResidentPolicyV1 export", () => {
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(valueResidentSchema).addSchema(rendererSchema);
    const policy = ajv.getSchema(`${rendererSchema.$id}#/$defs/RendererResidentPolicyV1`)!;
    expect(policy(rendererResidentLedger().capacity), JSON.stringify(policy.errors)).toBe(true);
    expect(policy(residentFixture.capacity), JSON.stringify(policy.errors)).toBe(true);
    expect(policy({ ...residentFixture.capacity, bytes: residentFixture.capacity.bytes + 1 })).toBe(false);
  });

  it("pins one resident ledger identity for the React and WGPU renderer consumers", () => {
    expect(rendererResidentLedger()).toBe(rendererResidentLedger());
    expect(residentFixture.consumers).toEqual(["react", "wgpu"]);
    expect(residentFixture.sameLedger).toBe(true);
    expect(residentFixture.replacesClosingLedger).toBe(false);
  });

  it("resolves required host aliases to exact canonical editor app objects without fallback", () => {
    const apps = hostBootstrapFixture.identity.apps;
    const resolved = resolveRequiredHostApps(apps, hostBootstrapFixture.identity.aliases);
    expect(resolveRequiredHostApps(apps, hostBootstrapFixture.identity.aliases)).toBe(resolved);
    expect(resolved.landing).toBe(apps[1]);
    expect(resolved.host).toBe(apps[2]);
    expect(resolved.landing.id).toBe(hostBootstrapFixture.identity.expected.landingAppId);
    expect(resolved.host.id).toBe(hostBootstrapFixture.identity.expected.hostAppId);
    expect(() => resolveRequiredHostApps(apps.filter((app) => app !== apps[2]), hostBootstrapFixture.identity.aliases)).toThrow(/host alias/);
    expect(() => resolveRequiredHostApps([...apps, { ...apps[2], id: "other.studio@1\/*#editor" }], hostBootstrapFixture.identity.aliases)).toThrow(/ambiguous/);
  });

  it("validates the neutral host/bootstrap contract with AJV and renders exact bilingual units and cancellation", () => {
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(rendererSchema);
    const identity = ajv.getSchema(`${rendererSchema.$id}#/$defs/HostIdentityResolutionV1`)!;
    const status = ajv.getSchema(`${rendererSchema.$id}#/$defs/BootstrapStatusV1`)!;
    expect(identity(hostBootstrapFixture.identity), JSON.stringify(identity.errors)).toBe(true);
    for (const notice of [hostBootstrapFixture.bootstrap.progress, hostBootstrapFixture.bootstrap.failure, hostBootstrapFixture.bootstrap.rebootstrap]) {
      expect(status(notice), JSON.stringify(status.errors)).toBe(true);
    }
    let cancelled = "";
    const progress = hostBootstrapFixture.bootstrap.progress as Extract<BootstrapUiStatus, { kind: "artifact-bootstrap-progress" }>;
    const view = render(React.createElement(BootstrapStatusNotice, { status: progress, locale: "de", onCancel: (documentId) => { cancelled = documentId; } }));
    expect(view.getByRole("status").textContent).toContain(hostBootstrapFixture.bootstrap.expected.de);
    const progressbar = view.getByRole("progressbar");
    expect(progressbar.getAttribute("value")).toBe(String(progress.receivedBytes));
    expect(progressbar.getAttribute("max")).toBe(String(progress.totalBytes));
    fireEvent.click(view.getByRole("button", { name: "Wiederherstellung abbrechen" }));
    expect(cancelled).toBe(progress.documentId);
    view.rerender(React.createElement(BootstrapStatusNotice, { status: hostBootstrapFixture.bootstrap.rebootstrap as Extract<BootstrapUiStatus, { kind: "artifact-rebootstrap-required" }>, locale: "en", onCancel: () => {} }));
    expect(view.getByRole("alert").textContent).toContain(hostBootstrapFixture.bootstrap.expected.rebootstrapEn);
    cleanup();

    const active = reduceBootstrapUiState({}, progress);
    expect(active[progress.documentId]).toEqual(progress);
    expect(reduceBootstrapUiState(active, { kind: "snapshot-replaced", documentId: progress.documentId })).toEqual({});
    expect(reduceBootstrapUiState(active, { kind: "detached", documentId: progress.documentId })).toEqual({});
  });

  it("offers one bilingual cancel control while a hub execution target is still being fetched, and none once it settled", () => {
    const verifying = { kind: "execution-target-status", documentId: "artifact-a", clientInstanceId: "33333333-3333-4333-8333-333333333333", spaceId: "space-a", code: "verifying", progress: { stage: "component", completedBytes: 392_140, totalBytes: 14_939_324 } } as const;
    let cancelled = 0;
    const view = render(React.createElement(ExecutionTargetStatusNotice, { status: verifying, locale: "de", onCancel: () => { cancelled += 1; } }));
    expect(view.getByRole("progressbar").getAttribute("max")).toBe("14939324");
    fireEvent.click(view.getByRole("button", { name: "Öffnen abbrechen" }));
    expect(cancelled).toBe(1);
    view.rerender(React.createElement(ExecutionTargetStatusNotice, { status: verifying, locale: "en", onCancel: () => {} }));
    expect(view.getByRole("button", { name: "Cancel opening" })).toBeTruthy();
    view.rerender(React.createElement(ExecutionTargetStatusNotice, { status: { ...verifying, code: "cancelled", progress: undefined }, locale: "en", onCancel: () => {} }));
    expect(view.container.querySelector("button")).toBe(null);
    cleanup();
  });

  it("stamps every boot install so the connect-time snapshot of the build it loaded is a replay, never a hot-swap", async () => {
    vi.stubGlobal("fetch", async () => new Response(null, { status: 200 }));
    try {
      const builtBeforeBoot = Date.now() - 1_000;
      const source = createDevPluginSource([{ pluginId: "note", moduleUrl: "/🔌️plugin-modules/🗒️note/🌉️bridge.js" }], "/🔌️plugin-modules/watch");
      const boot = await source.acquireModule("note", undefined, { signal: new AbortController().signal });
      expect(boot.rebuiltAt).toBeGreaterThanOrEqual(builtBeforeBoot);
      expect(new URL(boot.moduleUrl, "http://semio.test").searchParams.get("v")).toBe(String(boot.rebuiltAt));
      expect(pluginAvailabilityRouteV1(true, boot.rebuiltAt, builtBeforeBoot)).toBe("drop");
      expect(pluginAvailabilityRouteV1(true, boot.rebuiltAt, boot.rebuiltAt)).toBe("drop");
      expect(pluginAvailabilityRouteV1(true, boot.rebuiltAt, boot.rebuiltAt! + 1)).toBe("hot-swap");
      expect(pluginAvailabilityRouteV1(false, undefined, builtBeforeBoot)).toBe("install");
    } finally {
      vi.unstubAllGlobals();
    }
  });

  it("names every in-flight install's verified bytes in both languages beside one total", () => {
    const progress = pluginInstallProgressTotalV1({ note: { completedBytes: 12_345_678, totalBytes: 81_200_000 }, draw: { completedBytes: 0, totalBytes: 1_000_000 } }, ["draw", "note", "writer"]);
    expect(progress).toEqual({ completedBytes: 12_345_678, totalBytes: 82_200_000 });
    expect(pluginInstallProgressTotalV1({}, ["note"])).toBe(null);
    expect(pluginInstallBandTextV1(["note"], "en", { completedBytes: 12_345_678, totalBytes: 81_200_000 })).toBe("Loading plugin note · 12.3 of 81.2 MB verified");
    expect(pluginInstallBandTextV1(["note"], "de", { completedBytes: 12_345_678, totalBytes: 81_200_000 })).toBe("Plugin wird geladen note · 12,3 von 81,2 MB geprüft");
    expect(pluginInstallBandTextV1(["note"], "de", null)).toBe("Plugin wird geladen note");
  });

  it("upserts a freshly installed program in its plugin's place, so an awaited install routes over it before the next render", () => {
    const program = (pluginId: string, label: string) => ({ handle: { pluginId }, manifest: { label } }) as unknown as LoadedProgramState;
    const loaded = [program("space", "Space"), program("gis", "GIS")];
    expect(upsertLoadedProgramV1(loaded, program("note", "Note")).map((entry) => entry.handle.pluginId)).toEqual(["space", "gis", "note"]);
    const replaced = upsertLoadedProgramV1(loaded, program("gis", "GIS 2"));
    expect(replaced.map((entry) => (entry.manifest as unknown as { label: string }).label)).toEqual(["Space", "GIS 2"]);
    expect(loaded.map((entry) => (entry.manifest as unknown as { label: string }).label)).toEqual(["Space", "GIS"]);
  });
});
