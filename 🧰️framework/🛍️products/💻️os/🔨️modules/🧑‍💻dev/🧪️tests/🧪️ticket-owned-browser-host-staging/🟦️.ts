import * as Vitest from "vitest";
import * as Path from "node:path";
import * as Url from "node:url";
import { registerFixtureAndPixelTests } from "./🌉️fixtures-and-pixels/🟦️.ts";
import { registerRouteAndBridgeTests } from "./🛂️routes-and-bridge/🟦️.ts";
import { registerJcoRewriteTests } from "./🧬️jco-rewrites/🟦️.ts";
import { registerVendorTransportTests } from "./🚚️vendor-transport/🟦️.ts";
import { registerProfileAndCatalogBuildTests } from "./🏗️profile-and-catalog-build/🟦️.ts";
import { registerHandleAndOutputTests } from "./🗃️handles-and-output/🟦️.ts";
import { registerPollAndSmokeTests } from "./📡️poll-and-smoke/🟦️.ts";
import { testDependencies, type TestSource } from "./🧰️test-kit/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof testDependencies, source: TestSource): Promise<void> {
  await registerFixtureAndPixelTests(vitest, dependencies, source);
  await registerRouteAndBridgeTests(vitest, dependencies, source);
  await registerJcoRewriteTests(vitest, dependencies, source);
  await registerVendorTransportTests(vitest, dependencies, source);
  await registerProfileAndCatalogBuildTests(vitest, dependencies, source);
  await registerHandleAndOutputTests(vitest, dependencies);
  await registerPollAndSmokeTests(vitest, dependencies, source);
}

const testPackageDirectory = Path.resolve(Path.dirname(Url.fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
await registerTests1(Vitest as unknown as NonNullable<ImportMeta["vitest"]>, testDependencies, { directory: testPackageDirectory, url: Url.pathToFileURL(Path.resolve(testPackageDirectory, "📜️script.ts")).href });
