/** 🏷️ Dependency types of the GIS service-composition suites: the worker test bag extended by the GIS inference worker's own test seams and models. */
import type { BackboneWorkerTestDependencies } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts";
import type { createGisMapInferenceWorkerV1 } from "../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts";
import type { idleGisMapInferencePortStatusV1, parseGisMapInferenceApprovalReceiptV1 } from "../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts";

export type OwnerDriverV1 = ReturnType<typeof createGisMapInferenceWorkerV1>;
export type InferenceOperationV1 = NonNullable<OwnerDriverV1["test"]["inferencePort"]>;
export type ServiceCompositionTestDependenciesV1 = Omit<BackboneWorkerTestDependencies, "testSeams"> & OwnerDriverV1["test"] & { testSeams: BackboneWorkerTestDependencies["testSeams"] & OwnerDriverV1["test"]; idleGisMapInferencePortStatusV1: typeof idleGisMapInferencePortStatusV1; parseGisMapInferenceApprovalReceiptV1: typeof parseGisMapInferenceApprovalReceiptV1 };
