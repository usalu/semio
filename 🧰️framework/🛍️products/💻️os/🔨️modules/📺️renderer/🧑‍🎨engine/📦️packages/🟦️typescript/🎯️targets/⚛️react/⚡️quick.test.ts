import { isDeepStrictEqual } from "node:util";
import Ajv2020 from "ajv/dist/2020.js";
import React from "react";
import { describe, expect, it } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { rendererResidentLedger } from "../../../../💾️resident/🟦️.ts";
import residentFixture from "../../../../💾️resident/🧪️fixture/🔣️.json";
import {
  BootstrapStatusNotice,
  reduceBootstrapUiState,
  resolveRequiredHostApps,
  type BootstrapUiStatus,
} from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx";
import hostBootstrapFixture from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🧪️fixtures/🔣️.json";
import hostBootstrapSchema from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🔣️.schema.json";

describe("renderer quick contracts", () => {
  it("validates the language-neutral renderer resident capacity with the Node oracle", () => {
    const capacity = rendererResidentLedger().capacity;
    expect(isDeepStrictEqual(capacity, residentFixture.capacity)).toBe(true);
    expect(capacity.bytes - capacity.control.bytes).toBe(residentFixture.data.bytes);
    expect(capacity.slots - capacity.control.slots).toBe(residentFixture.data.slots);
    expect(capacity.owners - capacity.control.owners).toBe(residentFixture.data.owners);
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
    const validate = new Ajv2020({ strict: true }).compile(hostBootstrapSchema);
    expect(validate(hostBootstrapFixture), JSON.stringify(validate.errors)).toBe(true);
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
