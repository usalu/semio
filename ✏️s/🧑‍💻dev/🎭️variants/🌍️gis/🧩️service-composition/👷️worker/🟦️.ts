import { installDocumentServiceWorkerV1, documentWorkerTestDependenciesV1, documentWorkerTestSourceV1 } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { createGisMapInferenceWorkerV1 } from "../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts";

let driver: ReturnType<typeof createGisMapInferenceWorkerV1> | null = null;
const retire = installDocumentServiceWorkerV1("gis", "s.gis.gismap.inference", (host) => driver = createGisMapInferenceWorkerV1(host));
if (import.meta.hot) import.meta.hot.dispose(retire);
if (import.meta.vitest && driver) {
  const dependencies = documentWorkerTestDependenciesV1();
  const { registerTests1 } = await import("../🧪️tests/🟦️.ts");
  const models = await import("../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts");
  const testSeams = Object.defineProperties({}, { ...Object.getOwnPropertyDescriptors(dependencies.testSeams), ...Object.getOwnPropertyDescriptors(driver.test) });
  await registerTests1(import.meta.vitest, { ...dependencies, ...driver.test, ...models, testSeams } as Parameters<typeof registerTests1>[1], documentWorkerTestSourceV1);
}
