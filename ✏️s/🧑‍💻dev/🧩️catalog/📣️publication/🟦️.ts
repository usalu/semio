import {parseCatalogContributionV1,parseCatalogPublicationV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🟦️.ts";
import type {CatalogBuildSourceV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
/** 🧩️ The application selects its plugins and the independent Hub contribution. */
export function specificCatalogPublicationV1(readText:CatalogBuildSourceV1["readText"]){const specific=JSON.parse(readText("✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json")),hub=JSON.parse(readText("🌎️hub/📇️catalog/🔣️.json"));return parseCatalogPublicationV1({...specific,contributions:[...specific.contributions,parseCatalogContributionV1(hub)]});}

/** 🔌️ The workspace composes this optional application provider through an explicit build source port. */
export function catalogBuildContributionV1(sources:CatalogBuildSourceV1){return {version:1 as const,publication:specificCatalogPublicationV1(sources.readText),projectRoot:"✏️s/🧑‍💻dev",project:"@semio-tech/s-dev-composition",generateTarget:"generate",inputs:["✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json","🌎️hub/📇️catalog/🔣️.json"]};}
