import { installDocumentServiceWorkerV1, documentWorkerTestDependenciesV1, documentWorkerTestSourceV1 } from "@semio-tech/framework-os/modules/store/worker";
import { createGisMapInferenceWorkerV1 } from "@semio-tech/gis-gismap-js/inference/worker";

/** 🧩️ Merges two seam objects keeping accessors live. */
const mergeSeams = <A extends object, B extends object>(first: A, second: B): A & B =>
  Object.defineProperties<A & B>(Object.create(Object.prototype), { ...Object.getOwnPropertyDescriptors(first), ...Object.getOwnPropertyDescriptors(second) });
const worker: { driver: ReturnType<typeof createGisMapInferenceWorkerV1> | null } = { driver: null };
const retire = installDocumentServiceWorkerV1("gis", "s.gis.gismap.inference", (host) => worker.driver = createGisMapInferenceWorkerV1(host));
if (import.meta.hot) import.meta.hot.dispose(retire);
if (import.meta.vitest && worker.driver) {
  const driver = worker.driver;
  const { registerGisInferencePortTests } = await import("../🧪️tests/💡️inference-port/🟦️.ts");
  const { registerGisDocumentOpenTests } = await import("../🧪️tests/🗺️document-open/🟦️.ts");
  const { registerGisPeerRebootstrapTests } = await import("../🧪️tests/🔁️peer-rebootstrap/🟦️.ts");
  const { registerGisDirectoryLaneTests } = await import("../🧪️tests/🛤️directory-lane/🟦️.ts");
  const { workerLawSourceV1 } = await import("@semio-tech/framework-os/testing/worker-law-fakes");
  const dependencies = documentWorkerTestDependenciesV1();
  const models = await import("@semio-tech/gis-gismap-js/inference/schema");
  const testSeams = mergeSeams(dependencies.testSeams, driver.test);
  const source = workerLawSourceV1(documentWorkerTestSourceV1);
  const gisDependencies: Parameters<typeof registerGisInferencePortTests>[1] = { ...dependencies, ...driver.test, ...models, testSeams };
  await registerGisInferencePortTests(import.meta.vitest, gisDependencies, source);
  await registerGisDocumentOpenTests(import.meta.vitest, gisDependencies, source);
  await registerGisPeerRebootstrapTests(import.meta.vitest, gisDependencies, source);
  await registerGisDirectoryLaneTests(import.meta.vitest, gisDependencies, source);
}

