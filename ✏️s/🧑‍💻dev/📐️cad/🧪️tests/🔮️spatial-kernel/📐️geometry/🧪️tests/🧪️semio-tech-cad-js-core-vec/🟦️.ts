import type { GeometryTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";
import { createVecTestKit } from "./🧰️vec-test-kit/🟦️.ts";
import type { TestSource } from "./🧰️vec-test-kit/🟦️.ts";
import { registerVecAndModelDefinitionCatalogTests } from "./📏️vec-and-model-definition-catalogs/🟦️.ts";
import { registerModelSpaceAndTransformationTests } from "./🔁️model-space-and-transformations/🟦️.ts";
import { registerValidationGeometryAndMetadataTests } from "./🧊️validation-geometry-and-metadata/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: GeometryTestDependencies, source: TestSource): Promise<void> {
  const kit = createVecTestKit(vitest, dependencies, source);
  registerVecAndModelDefinitionCatalogTests(kit);
  registerModelSpaceAndTransformationTests(kit);
  registerValidationGeometryAndMetadataTests(kit);
}
