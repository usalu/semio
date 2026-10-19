import { describe } from "vitest";
import { registerSchedulerContractAndOwnershipTests } from "./🧮️scheduler-contract-and-ownership/🟦️.ts";
import { registerNativeResponseAndDecodeRetirementTests } from "./📦️native-response-and-decode-retirement/🟦️.ts";
import { registerAssetDecodeAndPresenterWakeTests } from "./🖼️asset-decode-and-presenter-wakes/🟦️.ts";
import { registerWorkerTaskAndIngressTests } from "./🧵️worker-task-and-ingress/🟦️.ts";
import { registerBootStepBudgetTests } from "./⏳️boot-step-budgets/🟦️.ts";

describe("wgpu frame-Worker step budget", () => {
  registerSchedulerContractAndOwnershipTests();
  registerNativeResponseAndDecodeRetirementTests();
  registerAssetDecodeAndPresenterWakeTests();
  registerWorkerTaskAndIngressTests();
  registerBootStepBudgetTests();
});
