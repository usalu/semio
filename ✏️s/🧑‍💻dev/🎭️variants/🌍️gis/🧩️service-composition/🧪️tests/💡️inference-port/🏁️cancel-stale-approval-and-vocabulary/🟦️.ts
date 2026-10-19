/** 🏁️ The cancel, stale base, approval, preview and vocabulary tests. */
import { settleInferenceTurns } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { GisInferencePortTestKit, GisMapInferencePortStatusV1 } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { InferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";

/** 🧾 Registers the cancel click, stale base, exact hash approval, cross job preview, route rejection and bilingual vocabulary tests. */
export function registerCancelStaleApprovalTests(kit: GisInferencePortTestKit, helpers: InferencePortHelpers): void {
  const { driveInferencePort, expect, handleTsRequest, it } = kit;
  const { DOCUMENT, HASH, JOB, PREVIEW, SPACE, inferenceHarness, pageBody, receiptBody } = helpers;
  it("records a Cancel click as requested and reaches cancelled only on the server's own answer", async () => {
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 4, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(200);
      harness.bodies.push(receiptBody("accepted", "none", 0));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 4, payload: { requestId: "4".repeat(32) } });
      await settleInferenceTurns();
      harness.statuses.push(200);
      harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 1, total: 4, cancelRequested: true }));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 4, payload: {  } });
      const requested = harness.ports().at(-1)!;
      expect(requested.cancelRequested).toBe(true);
      expect(requested.phase).toBe("running");
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 4, payload: {  } });
      await settleInferenceTurns();
      expect(harness.ports().at(-1)?.phase).toBe("cancelled");
      expect(harness.requests.filter((entry) => entry.endsWith("/cancel"))).toEqual([`POST /_semio/hub/spaces/${SPACE}/documents/${DOCUMENT}/inference/gis-map/jobs/${JOB}/cancel`]);
    } finally {
      harness.release();
    }
  });
  it("reports a stale base as a hard terminal that no later answer can move", async () => {
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 5, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(200);
      harness.bodies.push(receiptBody("accepted", "none", 0));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 5, payload: { requestId: "5".repeat(32) } });
      await settleInferenceTurns();
      harness.statuses.push(200);
      harness.bodies.push(pageBody("running", "none", { nextCursor: 1, completed: 1, total: 4, stale: true }));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "cancel", operationEpoch: 5, payload: {  } });
      await settleInferenceTurns();
      expect(harness.ports().at(-1)?.phase).toBe("stale");
      const before = harness.ports().length;
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 5, payload: {  } });
      await settleInferenceTurns();
      expect(harness.ports().length).toBe(before);
    } finally {
      harness.release();
    }
  });
  it("approves exactly the offered hash once and applies only on a committed receipt", async () => {
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 6, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(200);
      harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 6, payload: { requestId: "6".repeat(32) } });
      await settleInferenceTurns();
      expect(harness.ports().at(-1)?.phase).toBe("offered");
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 6, payload: {  } });
      expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
      harness.statuses.push(200);
      harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 2, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
      await driveInferencePort(6);
      expect(harness.ports().at(-1)?.preview).toEqual(PREVIEW);
      harness.statuses.push(200);
      harness.bodies.push(JSON.stringify({
        schema: "semio.hub.inference-approval-receipt/v1",
        jobId: JOB,
        mutationId: JOB,
        commandHash: HASH,
        proposalHash: HASH,
        applied: true,
        undo: { targetId: "2".repeat(32), expectedCurrent: { documentId: DOCUMENT, headEditOrdinal: 2, headEditId: "edit-2", lastCommitSeq: 2, chainSha256: "3".repeat(64) } },
      }));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch: 6, payload: {  } });
      expect(harness.ports().at(-1)?.phase).toBe("approving");
      await settleInferenceTurns();
      expect(harness.ports().at(-1)?.phase).toBe("applied");
      const approvalRequests = harness.requests.filter((entry) => entry.endsWith("/approval"));
      expect(approvalRequests).toHaveLength(1);
      expect(harness.state.outbox).toEqual([]);
    } finally {
      harness.release();
    }
  });
  it("retains an indeterminate owner on cross-job or cross-hash previews without sending approval", async () => {
    const substituted = [
      { ...PREVIEW, jobId: "8".repeat(32), regionId: `inference-${"8".repeat(32)}` },
      { ...PREVIEW, proposalHash: "9".repeat(64) },
    ];
    for (const [index, preview] of substituted.entries()) {
      const operationEpoch = 8 + index;
    const harness = await inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch, payload: { requestId: `${operationEpoch}`.repeat(32) } });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview }));
        await driveInferencePort(operationEpoch);
        expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.transport" });
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch, payload: {  } });
        await settleInferenceTurns();
        expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
      } finally {
        harness.release();
      }
    }
  });
  it("maps a published route rejection onto the closed failure vocabulary", async () => {
    const harness = await inferenceHarness({ lease: "editor" });
    try {
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: 7, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
      harness.statuses.push(503);
      harness.bodies.push(JSON.stringify({ schema: "semio.hub.inference-error/v1", code: "inference.unavailable" }));
      handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: 7, payload: { requestId: "7".repeat(32) } });
      await settleInferenceTurns();
      expect(harness.ports().at(-1)).toMatchObject({ phase: "indeterminate", code: "inference.unavailable" });
    } finally {
      harness.release();
    }
  });
  it("keeps explicit English and German text for every phase, code and control with no default language", async () => {
    const { GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, gisMapInferencePortRoleV1 } = await import("@semio-tech/gis-gismap-js/inference/schema");
    const phases: readonly GisMapInferencePortStatusV1["phase"][] = ["idle", "submitting", "running", "offered", "approving", "indeterminate", "applied", "cancelled", "stale", "failed"];
    for (const phase of phases) {
      const row = GIS_MAP_INFERENCE_PORT_TEXT_V1[phase];
      expect(Object.keys(row).sort()).toEqual(["de", "en"]);
      expect(row.en.length).toBeGreaterThan(0);
      expect(row.de.length).toBeGreaterThan(0);
      expect(row.de).not.toBe(row.en);
    }
    for (const [code, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1)) {
      expect(Object.keys(row).sort(), code).toEqual(["de", "en"]);
      expect(row.de, code).not.toBe(row.en);
    }
    for (const [control, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1)) {
      expect(Object.keys(row).sort(), control).toEqual(["de", "en"]);
      expect(row.de, control).not.toBe(row.en);
    }
    expect(gisMapInferencePortRoleV1("running")).toBe("status");
    expect(gisMapInferencePortRoleV1("failed")).toBe("alert");
  });
}
