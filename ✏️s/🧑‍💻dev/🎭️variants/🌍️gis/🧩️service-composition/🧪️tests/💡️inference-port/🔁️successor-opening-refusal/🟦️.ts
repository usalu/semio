/** 🔁️ The retained inference job successor opening refusal test. */
import { idleGisMapInferencePortStatusV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import type { InferenceOperationV1 } from "../../🧰️dependencies/🟦️.ts";
import type { GisInferencePortTestKit } from "../🧰️inference-port-test-kit/🟦️.ts";

/** 🧬 Registers the test that retains the preceding inference job when a successor opening is refused. */
export function registerSuccessorOpeningRefusalTests(kit: GisInferencePortTestKit): void {
  const { dependencies, dispatchBackboneWorkerRequest, handleTsRequest, idleGisMapInferencePortStatusV1, parseDocumentExecutionTargetLeaseFieldsV1, resetWorkerLaw, source, testSeams, vitest } = kit;
  vitest.it("retains the preceding inference job when a successor opening is refused", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    for (const field of ["closed", "request", "opened", "refused", "authorityRefused", "indeterminate"]) Object.assign(fixture[field], { owner: "gis", serviceId: "s.gis.gismap.inference" });
    for (const kind of fixture.routes) {
      const routes: string[] = [];
      const request = fixture.request;
      dispatchBackboneWorkerRequest(request, { handleRequestBytes: () => { routes.push("rust"); }, postReady: () => { routes.push("rust-ready"); } }, () => { routes.push("typescript"); });
      vitest.expect(equal(routes, ["typescript"])).toBe(true);
    }
    const seams = dependencies.testSeams;
    const previous = seams.inferencePort;
    const sink = seams.workerPostTestSink;
    const messages: unknown[] = [];
    const leaseFields = parseDocumentExecutionTargetLeaseFieldsV1(
      (JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", source.url), "utf8")) as { installedTarget: unknown }).installedTarget,
    );
    const retained: InferenceOperationV1 = {
      transport: { call: async () => { throw new Error("capacity law must not perform transport"); } },
      operationEpoch: 20,
      scope: fixture.request.payload.scope,
      abort: new AbortController(),
      sessionEpoch: testSeams.directorySessionEpoch,
      sessionFence: { token: {}, sessionBindingSha256: "0".repeat(64), authorizationGeneration: 1 },
      clientInstanceId: "12345678-1234-4123-8123-123456789abc",
      leaseFields,
      request: null,
      closeRequested: false,
      reconcileRequired: false,
      status: { ...idleGisMapInferencePortStatusV1(), phase: "running", jobId: "1".repeat(32) },
      turns: 0,
      pollTimer: null,
      inFlight: false,
      cancelSent: false,
      closed: false,
    };
    try {
      seams.inferencePort = retained;
      seams.workerPostTestSink = (message: unknown) => { messages.push(message); };
      await handleTsRequest(fixture.request);
      vitest.expect(seams.inferencePort).toBe(retained);
      vitest.expect(retained.abort.signal.aborted).toBe(false);
      vitest.expect(equal(messages, [fixture.refused])).toBe(true);
    } finally {
      seams.inferencePort = previous;
      seams.workerPostTestSink = sink;
    }
  });
}
