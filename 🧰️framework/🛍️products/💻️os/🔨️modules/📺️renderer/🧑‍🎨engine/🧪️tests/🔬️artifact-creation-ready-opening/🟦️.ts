import { handleSpaceArtifactCreationStatusMessage } from "../../🧱️elements/🏛️ShellHost/🧵️backbone-worker/🌱️artifact-creation-messages/🟦️.ts";
import workflowCreationFixture from "./workflow-creation.json";
import { openStudioWorkflowV1, workflowArtifactCreationRequestV1, workflowArtifactDropV1 } from "../../🧱️elements/🏛️ShellHost/🧾️space-artifact-creation/🔁️workflow/🟦️.ts";
import { dialectCoordinate } from "@semio-tech/framework";
import { decodeBackboneWorkerRequest, encodeBackboneWorkerResponse, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import type { SpaceArtifactCreationOwnerV1 } from "../../🧱️elements/🏛️ShellHost/🧾️space-artifact-creation/🟦️.ts";
import { createElement } from "react";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🔣️.json";
import artifactCreationCatalogAuthorityFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority/🔣️.json";
import { directorySchema } from "../../../../📇️directory/🧪️tests/🧫️fixtures/🟦️.ts";
import artifactCreationReadyOpeningFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🌱️artifact-creation/🚪️ready-opening/🔣️.json";

import { runArtifactCreationReadyOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import {
  ARTIFACT_CREATION_PROGRESS_TEXT_V1,
  ArtifactCreationCatalogNotice,
  ArtifactCreationProgressNotice,
  artifactCreationProgressRoleV1,
  type ArtifactCreationProgressStateV1,
} from "../../🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx";

const owner = {
  requestId: "1".repeat(32),
  spaceId: "space-a",
  expectedCatalogGenerationId: artifactCreationCatalogAuthorityFixture.catalog.catalogGenerationId,
  kindId: "fixture.neutral-host-fixture.counter",
  name: "Shared Map",
  runtimeKey: "hub:space-a:index",
  clientInstanceId: "client-a",
  sessionInstanceId: 7,
} as const;

describe("artifact creation Ready opening", () => {
  it("publishes only a committed current target and releases every rejected private target at most once", async () => {
    
    
    for (const row of artifactCreationReadyOpeningFixture.cases) {
      let currentIndex = 0;
      let releases = 0;
      let publishes = 0;
      let failures = 0;
      const outcome = await runArtifactCreationReadyOpeningV1({
        current: () => row.current[Math.min(currentIndex++, row.current.length - 1)]!,
        prepare: async () => {
          if (row.prepare === "unused") throw new Error("unexpected prepare");
          return row.prepare === "missing" ? null : { id: row.id };
        },
        open: async () => {
          if (row.open === "unused") throw new Error("unexpected open");
          if (row.open === "failed") throw new Error("attach failed");
          return row.open === "committed" ? { committed: true as const, runtimeKey: row.id, clientInstanceId: "client" } : null;
        },
        publish: () => { publishes += 1; },
        release: async () => {
          releases += 1;
          if (row.release === "failed") throw new Error("release failed");
        },
        failed: () => { failures += 1; },
      });
      expect(outcome, row.id).toBe(row.outcome);
      expect(releases, row.id).toBe(row.release === "unused" ? 0 : 1);
      expect(publishes, row.id).toBe(row.publish ? 1 : 0);
      expect(failures, row.id).toBe(row.failure ? 1 : 0);
    }
  });

  it("renders the schema-owned bilingual failure and normalizes impossible empty Ready catalogs to unavailable", () => {
    
    
    expect(deepEqual(artifactCreationProgressFixture.locales, ARTIFACT_CREATION_PROGRESS_TEXT_V1)).toBe(true);
    for (const row of artifactCreationProgressFixture.catalogCases) {
      const locale = row.locale as "en" | "de";
      const phase = row.phase as "loading" | "ready" | "unavailable";
      const effectivePhase = row.effectivePhase as "loading" | "ready" | "unavailable";
      const view = render(createElement(ArtifactCreationCatalogNotice, { status: { phase }, hasChoices: row.hasChoices, locale }));
      const notice = view.getByRole(row.role, { name: ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].catalog[effectivePhase] });
      expect(notice.getAttribute("aria-live"), row.id).toBe(row.live);
      expect(notice.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(effectivePhase);
      view.unmount();
    }
  });

  it("offers a bilingual retry for durable Ready without issuing cancellation", () => {
    for (const locale of ["en", "de"] as const) {
      const onCancel = vi.fn();
      const onOpen = vi.fn();
      const state: ArtifactCreationProgressStateV1 = { ...owner, issuedAtMs: 0, phase: "ready", cancelRequested: false, openingDisposition: "failed" };
      expect(artifactCreationProgressRoleV1(state.phase, state.openingDisposition)).toBe("alert");
      const view = render(createElement(ArtifactCreationProgressNotice, { state, locale, onCancel, onOpen }));
      const retry = ARTIFACT_CREATION_PROGRESS_TEXT_V1[locale].opening.retry;
      fireEvent.click(view.getByRole("button", { name: `${retry}: ${owner.name}` }));
      expect(onOpen).toHaveBeenCalledExactlyOnceWith(owner.requestId);
      expect(onCancel).not.toHaveBeenCalled();
      view.unmount();
    }
  });
});
describe("Workflow Catalogue artifact creation", () => {
  it("validates the drop shape and sends only the matching trusted creation kind to its Space", () => {
    const { catalog, drop, target, expected } = workflowCreationFixture;
    const oracle = new Ajv({ strict: true }).compile({
      type: "object",
      additionalProperties: false,
      required: ["pluginId", "appId", "nodeId", "x", "y"],
      properties: { pluginId: { type: "string", minLength: 1 }, appId: { type: "string", minLength: 1 }, nodeId: { type: "string", minLength: 1 }, x: { type: "number" }, y: { type: "number" } },
    });
    for (const args of [drop, { ...drop, spaceId: "other-space" }, { ...drop, x: "12" }, { ...drop, nodeId: "" }]) {
      expect(workflowArtifactDropV1(args)).toEqual(oracle(args) ? args : null);
    }
    expect(deepEqual(workflowArtifactCreationRequestV1(catalog, target, expected.spaceId, expected.requestId, expected.name), expected)).toBe(true);
    expect(workflowArtifactCreationRequestV1(catalog, target, "other-space", expected.requestId, expected.name)).toBeNull();
    expect(workflowArtifactCreationRequestV1({ ...catalog, kinds: [...catalog.kinds, catalog.kinds[1]!] }, target, expected.spaceId, expected.requestId, expected.name)).toBeNull();
  });

  it("resumes just once when the hub commits the exact artifact to the owning Space", () => {
    const { expected, target, ready, unrelatedStatuses } = workflowCreationFixture;
    const resume = vi.fn();
    const postMessage = vi.fn();
    let current = true;
    const owner: SpaceArtifactCreationOwnerV1 = {
      ...expected,
      runtimeKey: "fixture-runtime",
      clientInstanceId: "fixture-client",
      sessionInstanceId: 7,
      opening: false,
      cancelRequested: false,
      ready: null,
      workflow: { current: () => current, expectedSchema: target.io.artifactSchema, expectedDialect: dialectCoordinate(target.dialect), finish: async () => true },
    };
    const owners = new Map([[owner.requestId, owner]]);
    const send = (message: Extract<BackboneWorkerResponse, { kind: "space-artifact-creation-status" }>) => handleSpaceArtifactCreationStatusMessage({
      spaceArtifactCreationOwnersRef: { current: owners },
      openDocumentSessionsRef: { current: new Map() },
      setSpaceArtifactCreationUi: vi.fn(),
      openReadySpaceArtifactCreationRef: { current: resume },
      message,
      worker: { postMessage } as unknown as Worker,
    });
    const exact = { ...ready, kind: "space-artifact-creation-status", phase: "ready" } as const;
    send({ kind: exact.kind, requestId: exact.requestId, spaceId: exact.spaceId, catalogGenerationId: exact.catalogGenerationId, phase: "preparing" });
    for (const change of unrelatedStatuses) {
      const { artifactSchema, standard, ...other } = change;
      send({ ...exact, ...other, ready: { ...exact.ready, artifactSchema: artifactSchema ?? exact.ready.artifactSchema, parentDialect: { ...exact.ready.parentDialect, standard: standard ?? exact.ready.parentDialect.standard } } });
    }
    expect(resume).not.toHaveBeenCalled();
    send(exact);
    send(exact);
    expect(resume).toHaveBeenCalledExactlyOnceWith(owner.requestId);
    expect(owners.get(owner.requestId)?.ready).toEqual(exact);
    expect(postMessage).not.toHaveBeenCalled();
    current = false;
    send(exact);
    expect(resume).toHaveBeenCalledTimes(1);
    expect(owners.has(owner.requestId)).toBe(false);
  });

  it("waits for the sealed Directory history and reopens the winner if another user created the Workflow", async () => {
    const { catalog, target, expected, ready, studioEvents, foreignClientInstanceId } = workflowCreationFixture;
    const worker = new EventTarget() as EventTarget & { postMessage: ReturnType<typeof vi.fn> };
    worker.postMessage = vi.fn();
    const create = vi.fn();
    const lookup = openStudioWorkflowV1({
      worker: worker as unknown as Worker, app: target, baseUrl: "https://fixture.invalid", spaceId: expected.spaceId,
      requestId: expected.requestId, current: () => true, name: expected.name, create,
    });
    const request = (index: number) => decodeBackboneWorkerRequest(worker.postMessage.mock.calls[index]![0].wire);
    const indexOpen = request(0);
    if (indexOpen.kind !== "directory-space-open" || indexOpen.clientInstanceId === undefined) throw new Error("Expected an independently owned Directory lookup");
    const clientInstanceId = indexOpen.clientInstanceId;
    const send = (data: unknown): void => { worker.dispatchEvent(new MessageEvent("message", { data })); };
    const page = (events: readonly unknown[], owner = clientInstanceId): void => send({ kind: "directory-space-events", spaceId: expected.spaceId, events, clientInstanceId: owner });
    const historyReady = (owner = clientInstanceId) => send({ wire: encodeBackboneWorkerResponse({ kind: "directory-space-history-ready", spaceId: expected.spaceId, clientInstanceId: owner }) });
    expect(create).not.toHaveBeenCalled();
    send({ kind: "directory-space-events", spaceId: expected.spaceId, events: studioEvents });
    page(studioEvents, foreignClientInstanceId);
    historyReady(foreignClientInstanceId);
    page(studioEvents.slice(0, 1));
    expect(worker.postMessage).toHaveBeenCalledTimes(1);
    historyReady();
    const catalogOpen = decodeBackboneWorkerRequest(worker.postMessage.mock.calls[1]![0].wire);
    if (catalogOpen.kind !== "space-artifact-creation-catalog-open") throw new Error("Expected the scoped catalog query");
    send({ kind: "space-artifact-creation-catalog", clientInstanceId: catalogOpen.clientInstanceId, ...catalog });
    send({ kind: "space-artifact-creation-catalog-status", clientInstanceId: catalogOpen.clientInstanceId, spaceId: expected.spaceId, phase: "ready" });
    await Promise.resolve();
    expect(create).toHaveBeenCalledExactlyOnceWith(expected);
    send({ kind: "space-artifact-creation-status", requestId: expected.requestId, catalogGenerationId: expected.expectedCatalogGenerationId, spaceId: expected.spaceId, phase: "failed" });
    page(studioEvents);
    historyReady();
    await expect(lookup).resolves.toEqual({ artifactId: ready.ready.artifactId });
    expect(request(2)).toEqual(indexOpen);
    expect(request(3)).toEqual({ kind: "directory-space-close", spaceId: expected.spaceId, clientInstanceId });
    expect(worker.postMessage.mock.calls.map(([message]) => decodeBackboneWorkerRequest(message.wire).kind)).toEqual([
      "directory-space-open", "space-artifact-creation-catalog-open", "directory-space-open", "directory-space-close",
    ]);
  });
});
