import {readFileSync} from "node:fs";
import {parseCatalogContributionV1,parseCatalogPublicationV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🟦️.ts";
/** 🧩️ The application selects its plugins and the independent Hub contribution. */
export function specificCatalogPublicationV1(){const specific=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")),hub=JSON.parse(readFileSync(new URL("../../../../🌎️hub/📇️catalog/🔣️.json",import.meta.url),"utf8"));return parseCatalogPublicationV1({...specific,contributions:[...specific.contributions,parseCatalogContributionV1(hub)]});}

/** 🔌️ The workspace composes this optional application provider through an explicit Nx port. */
export function catalogNxContributionV1(){return {version:1 as const,publication:specificCatalogPublicationV1(),projectRoot:"✏️s/🧑‍💻dev",project:"@semio-tech/s-dev-composition",generateTarget:"generate",inputs:["✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json","🌎️hub/📇️catalog/🔣️.json"]};}
