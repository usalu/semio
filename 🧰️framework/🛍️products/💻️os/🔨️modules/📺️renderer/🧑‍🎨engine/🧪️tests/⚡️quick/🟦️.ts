import { isDeepStrictEqual } from "node:util";
import Ajv from "ajv";
import React from "react";
import { describe, expect, it } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { rendererResidentLedger } from "../../💾️resident/🟦️.ts";
import residentFixture from "../../💾️resident/🧫️fixtures/🔣️.json";
import {
  BootstrapStatusNotice,
  reduceBootstrapUiState,
  resolveRequiredHostApps,
  type BootstrapUiStatus,
} from "../../🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx";
import hostBootstrapFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🪪️host-bootstrap/🔣️.json";
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
});
