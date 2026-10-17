import base from "@semio-tech/framework-os/testing/config";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { frameworkSourceUrl } from "@semio-tech/framework/test/source-text";
const frameworkWorker = fileURLToPath(frameworkSourceUrl("🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
const workerLawSuites = fileURLToPath(frameworkSourceUrl("🛍️products/💻️os/🧪️tests/📚️worker-law-suites/🟦️.ts"));
const worker = fileURLToPath(new URL("../👷️worker/🟦️.ts", import.meta.url));
export default defineConfig({ ...base, test: { ...base.test, name: "gis-gismap-service-composition", include: [workerLawSuites], includeSource: [frameworkWorker, worker], coverage: { include: [worker] }, passWithNoTests: false } });
