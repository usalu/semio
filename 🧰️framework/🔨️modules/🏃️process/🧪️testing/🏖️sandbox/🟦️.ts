/** 🏖️ Per-run test sandbox: one test-environment API for temp roots, image roots, port leases, reclaim, the runtime guard and the concurrency law scanner. */
export { managedNativeParentOfV1, requireArtifactOutsideManagedParentV1, safeTemporaryDirectoryV1, readTestSandboxProtocolV1, testSandboxParentV1, testPathInsideV1, testProcessAliveV1 } from "./📐️layout/🟦️.ts";
export type { TestSandboxProtocolV1 } from "./📐️layout/🟦️.ts";
export { attachedDiskImagesV1, detachDiskImagesUnderV1, mayHoldDiskImageV1 } from "./🖼️images/🟦️.ts";
export { createTestSandboxV1, withTestSandboxV1, testSandboxEnvironmentV1, testScratchRootV1, adoptedTestSandboxRootV1, liveTestSandboxesV1 } from "./📁️root/🟦️.ts";
export type { TestSandboxV1, TestSandboxOptionsV1, TestGuardModeV1 } from "./📁️root/🟦️.ts";
export { reserveTestPortV1, releaseTestPortV1, readTestPortLeaseV1 } from "./🔌️ports/🟦️.ts";
export type { TestPortLeaseV1 } from "./🔌️ports/🟦️.ts";
export { reclaimAbandonedTestSandboxesV1, reclaimAgentSessionSandboxesV1, abandonedTestSandboxV1, listTestSandboxesV1, listSessionSandboxesV1 } from "./♻️reclaim/🟦️.ts";
export { guardContextFromEnvironmentV1, decideWriteV1, decideSpawnV1, decideListenV1, summarizeGuardReportV1, guardArgumentsV1, testCheckoutRootV1 } from "./🛡️guard/🟦️.ts";
export type { GuardContextV1, GuardViolationV1, GuardViolationKindV1 } from "./🛡️guard/🟦️.ts";
export { compileLawV1, scanSourceV1, scanOwnerV1, scanTestsV1, scanLanguageV1, scanIsTestPathV1, scanIsSuiteV1 } from "./🔍️scan/🟦️.ts";
export type { ScanFindingV1, ScanLanguageV1, CompiledLawV1 } from "./🔍️scan/🟦️.ts";
