import delivery from "../📇️catalog.json" with { type: "json" };
import metabolism from "../../🌱️metabolism/🎨️representation/📇️catalog.json" with { type: "json" };
import { parseMeshDeliveryCatalog } from "../🟦️.ts";

export const MESH_DELIVERY_CATALOG = parseMeshDeliveryCatalog(delivery, path => {
  if (path === "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json") return metabolism;
  throw new Error(`Unknown mesh source catalog: ${path}`);
});
