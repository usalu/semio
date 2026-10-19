/** 🧰️ The shared GIS Map inference port test kit: the destructured vitest, the worker law fakes and the production seams. */
import { idleGisMapInferencePortStatusV1 } from "@semio-tech/gis-gismap-js/inference/schema";
import { workerLawFakesV1 } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";

export type BackboneWorkerResponse = import("@semio-tech/framework-os").BackboneWorkerResponse;
export type DocumentExecutionTargetLeaseFieldsV1 = import("@semio-tech/framework/directory/schema").DocumentExecutionTargetLeaseFieldsV1;
export type GisMapInferencePortStatusV1 = import("@semio-tech/gis-gismap-js/inference/schema").GisMapInferencePortStatusV1;
export type GisMapInferencePreviewV1 = import("@semio-tech/gis-gismap-js/inference/schema").GisMapInferencePreviewV1;
export type Vitest = NonNullable<ImportMeta["vitest"]>;

/** ⏳️ Settles every turn one bounded request/response leg needs, then four quiet ones. */
export async function settleInferenceTurns(): Promise<void> {
  for (let turn = 0; turn < 40; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
}

/** 🏭 Creates the shared inference port test kit of one registration from the injected dependencies. */
export function createGisInferencePortTestKit(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource) {
  const { beforeEach, describe, expect, it } = vitest;
  const { testSeams, dispatchBackboneWorkerRequest, parseDocumentExecutionTargetLeaseFieldsV1, idleGisMapInferencePortStatusV1, handleTsRequest, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, openArtifact, artifactState, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, encodePackValue, closeArtifact, driveInferencePort, DirectoryClient } = dependencies;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  return { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DirectoryClient, DocumentExecutionTargetLease, artifactState, beforeEach, clearHubSessionCapability, closeArtifact, dependencies, describe, dispatchBackboneWorkerRequest, documentExecutionTargetLeaseMintToken, driveInferencePort, encodePackValue, expect, handleTsRequest, hubSessionFetch, idleGisMapInferencePortStatusV1, installHubSessionCapability, it, openArtifact, parseDocumentExecutionTargetLeaseFieldsV1, resetWorkerLaw, source, testSeams, vitest };
}
export type GisInferencePortTestKit = ReturnType<typeof createGisInferencePortTestKit>;
