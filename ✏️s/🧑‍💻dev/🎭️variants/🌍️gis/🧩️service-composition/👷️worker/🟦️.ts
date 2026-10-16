import { installDocumentServiceWorkerV1, documentWorkerTestDependenciesV1, documentWorkerTestSourceV1 } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { createGisMapInferenceWorkerV1 } from "../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts";

let driver: ReturnType<typeof createGisMapInferenceWorkerV1> | null = null;
const retire = installDocumentServiceWorkerV1("gis", "s.gis.gismap.inference", (host) => driver = createGisMapInferenceWorkerV1(host));
if (import.meta.hot) import.meta.hot.dispose(retire);
if (import.meta.vitest && driver) {
  const { registerGisInferencePortTests } = await import("../🧪️tests/💡️inference-port/🟦️.ts");
  const { registerGisDocumentOpenTests } = await import("../🧪️tests/🗺️document-open/🟦️.ts");
  const { registerGisPeerRebootstrapTests } = await import("../🧪️tests/🔁️peer-rebootstrap/🟦️.ts");
  const { registerGisDirectoryLaneTests } = await import("../🧪️tests/🛤️directory-lane/🟦️.ts");
  const { workerLawSourceV1 } = await import("../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites/🟦️.ts");
  const dependencies = documentWorkerTestDependenciesV1();
  const models = await import("../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts");
  const testSeams = Object.defineProperties({}, { ...Object.getOwnPropertyDescriptors(dependencies.testSeams), ...Object.getOwnPropertyDescriptors(driver.test) });
  const source = workerLawSourceV1(documentWorkerTestSourceV1);
  const gisDependencies = { ...dependencies, ...driver.test, ...models, testSeams } as Parameters<typeof registerGisInferencePortTests>[1];
  await registerGisInferencePortTests(import.meta.vitest, gisDependencies, source);
  await registerGisDocumentOpenTests(import.meta.vitest, gisDependencies, source);
  await registerGisPeerRebootstrapTests(import.meta.vitest, gisDependencies, source);
  await registerGisDirectoryLaneTests(import.meta.vitest, gisDependencies, source);
}

