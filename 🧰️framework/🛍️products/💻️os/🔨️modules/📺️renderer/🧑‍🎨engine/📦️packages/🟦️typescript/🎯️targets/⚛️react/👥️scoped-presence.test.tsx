import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  decodeBackboneWorkerResponse,
  documentRuntimeKeyV1,
  encodeBackboneWorkerResponse,
  type BackboneWorkerResponse,
  type DocumentScope,
  type GisMapInferencePortStatusV1,
} from "@semio-tech/framework-os";
import { scopedPresencePeersV1 } from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🟦️.ts";
import {
  InferencePortPanel,
  inferencePortStatusRuntimeKeyV1,
  reduceBootstrapUiState,
  reduceExecutionTargetUiState,
  retainInferencePortOwnerAfterCloseV1,
  type InferencePortOwnerV1,
} from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx";
import fixture from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🔣️.json";
import schema from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🧬️.schema.json";

type Case = (typeof fixture.cases)[number];

afterEach(cleanup);

const inferencePreview = {
  schema: "semio.hub.gis-map-inference-preview/v1",
  jobId: "0123456789abcdef0123456789abcdef",
  proposalHash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  regionId: "inference-0123456789abcdef0123456789abcdef",
  ring: [[-73.99, 40.71], [-73.97, 40.71], [-73.97, 40.73], [-73.99, 40.73], [-73.99, 40.71]],
} as const;

function offeredInferenceStatus(preview: typeof inferencePreview | undefined): GisMapInferencePortStatusV1 {
  return {
    phase: "offered",
    jobId: inferencePreview.jobId,
    cursor: 4,
    completed: 3,
    total: 3,
    proposalHash: inferencePreview.proposalHash,
    ...(preview === undefined ? {} : { preview }),
    cancelRequested: false,
    code: null,
  };
}

function event(row: Case): Extract<BackboneWorkerResponse, { readonly kind: "event" }> {
  return {
    kind: "event",
    documentId: row.scope.documentId,
    scope: row.scope,
    verifiedSurfaceId: row.surface,
    event: { kind: "presence", peers: [row.peer] },
  };
}

describe("scope-safe Shell presence", () => {
  it("validates the neutral two-space fixture and independent runtime-key derivation", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.cases.map((row) => documentRuntimeKeyV1({ kind: "hub", ...row.scope }))).toEqual(fixture.cases.map((row) => row.runtimeKey));
    expect(fixture.routes).toEqual(fixture.cases.map((row) => `actor://${row.runtimeKey}`));
  });

  it("keeps equal document ids isolated by verified scope and surface", () => {
    const [a, b] = fixture.cases;
    expect(scopedPresencePeersV1(event(a), a.scope)).toEqual([
      { actor: "actor-a", userId: "user-a", label: "Ada", role: "author", connectedAtMs: 101, color: 2 },
    ]);
    expect(scopedPresencePeersV1(event(b), b.scope)).toEqual([
      { actor: "actor-b", userId: "user-b", label: "Berta", role: "spectator", connectedAtMs: 202, color: 5 },
    ]);
    expect(scopedPresencePeersV1(event(a), b.scope)).toEqual([]);
    expect(scopedPresencePeersV1({ ...event(a), verifiedSurfaceId: "map@1/*#viewer" }, a.scope)).toEqual([]);
  });

  it("decodes missing or mismatched private authority as an empty roster", () => {
    const [a] = fixture.cases;
    const hostile: BackboneWorkerResponse[] = [
      { ...event(a), scope: undefined } as unknown as BackboneWorkerResponse,
      { ...event(a), scope: { spaceId: "space-a", documentId: "other" } } as BackboneWorkerResponse,
      { ...event(a), verifiedSurfaceId: undefined } as unknown as BackboneWorkerResponse,
    ];
    for (const row of hostile) {
      const decoded = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(row));
      expect(decoded.kind).toBe("event");
      if (decoded.kind === "event") {
        expect(decoded.event).toEqual({ kind: "presence", peers: [] });
        expect(scopedPresencePeersV1(decoded, a.scope)).toEqual([]);
      }
    }
  });

  it("clears only the revoked scope's bootstrap and execution-target rows", () => {
    const [a, b] = fixture.cases;
    const bootstrap = [a, b].reduce(
      (state, row) => reduceBootstrapUiState(state, { kind: "artifact-bootstrap-progress", documentId: row.scope.documentId, scope: row.scope, receivedBytes: 1, totalBytes: 2, receivedChunks: 1, totalChunks: 2 }),
      {},
    );
    const afterBootstrapClose = reduceBootstrapUiState(bootstrap, { kind: "detached", documentId: a.scope.documentId, scope: a.scope });
    expect(Object.keys(afterBootstrapClose)).toEqual([b.runtimeKey]);
    const execution = [a, b].reduce(
      (state, row) => reduceExecutionTargetUiState(state, { kind: "execution-target-status", documentId: row.scope.documentId, spaceId: row.scope.spaceId, scope: row.scope as DocumentScope, code: "verifying" }),
      {},
    );
    const afterExecutionClose = reduceExecutionTargetUiState(execution, { kind: "execution-target-cleared", documentId: a.scope.documentId, scope: a.scope });
    expect(Object.keys(afterExecutionClose)).toEqual([b.runtimeKey]);
  });

  it("renders only the Hub-validated bounds preview and withholds blind approval", () => {
    const onAction = vi.fn();
    const { container, rerender } = render(<InferencePortPanel status={offeredInferenceStatus(undefined)} locale="en" onAction={onAction} />);
    expect(Array.from(container.querySelectorAll("button"), (button) => button.textContent)).not.toContain("Approve proposal");
    expect(container.querySelector("[data-semio-inference-preview]")).toBeNull();

    rerender(<InferencePortPanel status={offeredInferenceStatus(inferencePreview)} locale="de" onAction={onAction} />);
    expect(screen.getByText("Gebiet")).toBeTruthy();
    expect(screen.getByText(inferencePreview.regionId)).toBeTruthy();
    expect(Array.from(container.querySelectorAll("dd"), (cell) => cell.textContent)).toEqual([
      inferencePreview.regionId,
      "-73.99–-73.97",
      "40.71–40.73",
    ]);
    fireEvent.click(screen.getByRole("button", { name: "Vorschlag freigeben" }));
    expect(onAction).toHaveBeenCalledWith({ kind: "approve" });
  });

  it("accepts inference status only for the exact runtime owner and preserves the sibling scope on close", () => {
    const [a, b] = fixture.cases;
    const owner: InferencePortOwnerV1 = { operationEpoch: 8, runtimeKey: b.runtimeKey, scope: b.scope };
    const status = offeredInferenceStatus(inferencePreview);
    const response = (scope: DocumentScope, operationEpoch = 8): Extract<BackboneWorkerResponse, { readonly kind: "inference-port-status" }> => ({
      kind: "inference-port-status",
      operationEpoch,
      scope,
      status,
    });
    const decodedA = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response(a.scope, 7)));
    const decodedB = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response(b.scope)));
    const decodedWrongScope = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response({ spaceId: "space-c", documentId: b.scope.documentId })));
    if (decodedA.kind !== "inference-port-status" || decodedB.kind !== "inference-port-status" || decodedWrongScope.kind !== "inference-port-status") throw new Error("inference response discriminator changed");
    expect(inferencePortStatusRuntimeKeyV1(owner, 8, decodedA)).toBeNull();
    expect(inferencePortStatusRuntimeKeyV1(owner, 8, decodedWrongScope)).toBeNull();
    expect(inferencePortStatusRuntimeKeyV1(owner, 8, decodedB)).toBe(b.runtimeKey);
    expect(retainInferencePortOwnerAfterCloseV1(owner, a.runtimeKey)).toBe(owner);
    expect(retainInferencePortOwnerAfterCloseV1(owner, b.runtimeKey)).toBeNull();
  });
});
