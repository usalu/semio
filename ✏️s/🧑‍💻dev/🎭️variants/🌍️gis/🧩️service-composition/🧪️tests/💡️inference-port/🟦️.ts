/** 💡️ The host-owned GIS Map inference port driven through a fake authenticated transport, and the retained inference job across a refused successor opening. */
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../🧰️dependencies/🟦️.ts";
import { createGisInferencePortTestKit } from "./🧰️inference-port-test-kit/🟦️.ts";
import type { Vitest } from "./🧰️inference-port-test-kit/🟦️.ts";
import { registerSuccessorOpeningRefusalTests } from "./🔁️successor-opening-refusal/🟦️.ts";
import { registerGisMapInferencePortTests } from "./🗺️gis-map-inference-port/🟦️.ts";

export async function registerGisInferencePortTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const kit = createGisInferencePortTestKit(vitest, dependencies, source);
  //#region 💡️InferencePortTests
  registerSuccessorOpeningRefusalTests(kit);
  registerGisMapInferencePortTests(kit);
  //#endregion 💡️InferencePortTests
}
