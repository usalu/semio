/** 🔒️ The sealed inference owner retention, Undo owner and successor proof tests. */
import { WORKER_LAW_CAPABILITY } from "@semio-tech/framework-os/testing/worker-law-fakes";
import { settleInferenceTurns } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";
import type { InferencePortHelpers } from "../🛠️inference-port-helpers/🟦️.ts";

/** 🧊 Registers the sealed owner retention, Undo owner, successor broker proof and canonical response tests. */
export function registerSealedOwnerRetentionTests(kit: GisInferencePortTestKit, helpers: InferencePortHelpers): void {
  const { DirectoryClient, clearHubSessionCapability, closeArtifact, driveInferencePort, expect, handleTsRequest, hubSessionFetch, installHubSessionCapability, it, source, testSeams } = kit;
  const { DOCUMENT, HASH, JOB, PREVIEW, SPACE, inferenceHarness, pageBody, receiptBody } = helpers;
  it("retains the sealed inference owner through uncertain submit and document retirement", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    for (const scenario of fixture.cases) {
    const harness = await inferenceHarness({ lease: "editor" });
      const sessionEpoch = testSeams.directorySessionEpoch;
      try {
        const scope = { spaceId: SPACE, documentId: DOCUMENT };
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: fixture.operationEpoch, payload: { scope } });
        harness.bodies.push(scenario === "retired-document" ? receiptBody("accepted", "none", 0) : "{");
        let releaseReceipt: (() => void) | undefined;
        if (scenario === "retired-document") harness.gates.push(new Promise<void>((resolve) => { releaseReceipt = resolve; }));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch: fixture.operationEpoch, payload: { requestId: fixture.requestId } });
        const original = testSeams.inferencePort;
        if (scenario === "retired-document") {
          for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
          expect(harness.requests).toHaveLength(1);
          closeArtifact(DOCUMENT, SPACE);
          releaseReceipt!();
        }
        else handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "close", operationEpoch: fixture.operationEpoch, payload: {  } });
        await settleInferenceTurns();
        expect(testSeams.inferencePort).toBe(original);
        expect(original?.request?.requestId).toBe(fixture.requestId);
        expect(harness.posted.some((message) => message.kind === fixture.closedKind)).toBe(false);
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch: fixture.operationEpoch + 1, payload: { scope } });
        expect(harness.posted.at(-1)).toMatchObject({ kind: "inference-port-opened", outcome: "refused", code: "inference.capacity" });
        if (scenario === "rotated-session") {
          testSeams.directorySessionEpoch += 1;
          const count = harness.requests.length;
          await driveInferencePort(fixture.operationEpoch);
          expect(harness.requests).toHaveLength(count);
          expect(harness.ports().at(-1)?.phase).toBe(fixture.unknownPhase);
          expect(testSeams.inferencePort).toBe(original);
          continue;
        }
        if (scenario === "lost-submit") {
          expect(harness.ports().at(-1)?.phase).toBe(fixture.unknownPhase);
          harness.bodies.push(JSON.stringify({
            schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
            job: {
              receipt: { schema: "semio.hub.inference-job-receipt/v1", jobId: JOB, state: "running", proposalState: "none", proposalHash: null, cursor: 0, expiresAtMs: 1_700_000_060_000 },
              page: { jobId: JOB, state: "running", proposalState: "none", cancelRequested: false, expired: false, proposalHash: null, events: [], progress: [], nextCursor: 0 },
              approval: null,
            },
          }));
          await driveInferencePort(fixture.operationEpoch);
          expect(harness.requests.at(-1)).toBe(fixture.reconcilePath);
          expect(original?.status.jobId).toBe(JOB);
        }
        harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 0, total: 4, cancelRequested: true }));
        await driveInferencePort(fixture.operationEpoch);
        expect(harness.ports().at(-1)?.phase).toBe(fixture.terminalPhase);
        expect(testSeams.inferencePort).toBeNull();
        const closed = harness.posted.filter((message) => message.kind === fixture.closedKind);
        expect(equal(closed, [{ kind: fixture.closedKind, owner:"gis", serviceId:"s.gis.gismap.inference", operationEpoch: fixture.operationEpoch, scope }])).toBe(true);
        expect(harness.requests.filter((request) => request.endsWith("/jobs"))).toHaveLength(1);
      } finally {
        testSeams.directorySessionEpoch = sessionEpoch;
        const retained = testSeams.inferencePort;
        if (retained?.pollTimer) clearTimeout(retained.pollTimer);
        retained?.abort.abort();
        testSeams.inferencePort = null;
        harness.release();
      }
    }
  });
  it("never restores an inference Undo owner after the document closes", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    for (const scenario of fixture.approvalClose) {
    const harness = await inferenceHarness({ lease: "editor" });
      try {
        const operationEpoch = fixture.operationEpoch + 2;
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "open", operationEpoch, payload: { scope: { spaceId: SPACE, documentId: DOCUMENT } } });
        harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "propose", operationEpoch, payload: { requestId: fixture.requestId } });
        await settleInferenceTurns();
        harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
        await driveInferencePort(operationEpoch);
        const receipt = { schema: "semio.hub.inference-approval-receipt/v1", jobId: JOB, mutationId: JOB, commandHash: HASH, proposalHash: HASH, applied: true, undo: { targetId: "2".repeat(32), expectedCurrent: { documentId: DOCUMENT, headEditOrdinal: 2, headEditId: "edit-2", lastCommitSeq: 2, chainSha256: "3".repeat(64) } } };
        let release: () => void = () => undefined;
        if (scenario === "approval-response") {
          harness.bodies.push(JSON.stringify(receipt));
          harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
        } else harness.bodies.push("{");
        handleTsRequest({ kind: "service-operation", owner: "gis", serviceId: "s.gis.gismap.inference", action: "approve", operationEpoch, payload: {  } });
        for (let turn = 0; harness.requests.length < 3 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
        expect(harness.requests).toHaveLength(3);
        if (scenario === "reconcile-response") await settleInferenceTurns();
        closeArtifact(DOCUMENT, SPACE);
        const statusCount = harness.posted.filter((message) => message.kind === "inference-history-status").length;
        if (scenario === "approval-response") release();
        else {
          harness.bodies.push(JSON.stringify({
            schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId: fixture.requestId, found: true,
            job: {
              receipt: { schema: "semio.hub.inference-job-receipt/v1", jobId: JOB, state: "succeeded", proposalState: "approved", proposalHash: HASH, cursor: 1, expiresAtMs: 1_700_000_060_000 },
              page: { jobId: JOB, state: "succeeded", proposalState: "approved", cancelRequested: false, expired: true, proposalHash: HASH, events: [], progress: [], nextCursor: 1 },
              approval: { state: "available", receipt },
            },
          }));
        }
        await settleInferenceTurns();
        if (scenario === "reconcile-response") await driveInferencePort(operationEpoch);
        expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
        expect(harness.posted.filter((message) => message.kind === "inference-history-status")).toHaveLength(statusCount);
        expect(equal(harness.ports().at(-1)?.phase, "applied")).toBe(true);
      } finally { harness.release(); }
    }
  });
  it("preserves the successor broker proof when an old inference owner responds", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json", import.meta.url), "utf8")).retainedClosing;
    const harness = await inferenceHarness({ lease: "editor" });
    const epoch = testSeams.directorySessionEpoch;
    try {
      let release: () => void = () => undefined;
      harness.gates.push(new Promise<void>((resolve) => { release = resolve; }));
      harness.bodies.push("{}");
      const pending = hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, admit: () => testSeams.directorySessionEpoch === epoch });
      const rejected = expect(pending).rejects.toThrow();
      for (let turn = 0; harness.requests.length === 0 && turn < 100; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
      expect(harness.requests).toHaveLength(1);
      testSeams.directorySessionEpoch += 1;
      clearHubSessionCapability();
      expect(installHubSessionCapability(fixture.successorCapability)).toBe(true);
      release();
      await rejected;
      harness.bodies.push("{}");
      await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000 });
      expect(harness.proofs).toHaveLength(2);
      expect(harness.proofs[0]).toBe(`Bearer ${WORKER_LAW_CAPABILITY}`);
      expect(equal(harness.proofs[1], `Bearer ${fixture.successorCapability}`)).toBe(true);
    } finally { testSeams.directorySessionEpoch = epoch; harness.release(); }
  });
  it("validates the exact canonical authenticated Directory client response", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    for (const row of fixture.rows) {
      const client = new DirectoryClient("http://hub.test", { request: async () => new Response(JSON.stringify(row.value), { status: 200 }) });
      if (row.accepted) expect(equal(await client.me(), row.value)).toBe(true);
      else await expect(client.me()).rejects.toThrow();
    }
    for (const row of fixture.raw) {
      const client = new DirectoryClient("http://hub.test", { request: async () => new Response(row.source, { status: 200 }) });
      if (row.accepted) expect(equal(await client.me(), JSON.parse(row.source))).toBe(true);
      else await expect(client.me()).rejects.toThrow();
    }
  });
}
