/** 🟣️ The shared declarations of the artifact path projection authority suite tests. */
import { loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { semanticPathProjectionAuthority } from "../../../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type Taxonomy } from "../../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { type ArtifactProjectionGoldenEntry, drawScenarioTaxonomy, projectionAuthorityNodes } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🗺️ Selects the authority a golden projection is planned against: the live taxonomy for CAD, the scenario overlay for the authored Draw bundle. */
export function projectionTaxonomy(projection: ArtifactProjectionGoldenEntry): Taxonomy {
  return projection.contractId === "artifact-editor-command-bundle-v1" ? drawScenarioTaxonomy(loadTaxonomy()) : loadTaxonomy();
}

export function projectionAuthority(projection: ArtifactProjectionGoldenEntry, nodes = projectionAuthorityNodes(projection), occupiedPaths: readonly string[] = []) {
  return semanticPathProjectionAuthority({
    artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")),
    contractId: projection.contractId,
    nodes,
    occupiedPaths,
    sourceRoot: projection.sourceRoot,
  }, projectionTaxonomy(projection));
}
