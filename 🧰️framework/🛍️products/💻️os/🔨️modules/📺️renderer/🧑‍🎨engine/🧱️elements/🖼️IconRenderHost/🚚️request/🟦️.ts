import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/📤️export-request/🔣️.json" with { type: "json" };
import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import { iconRenderPort, type IconRenderRequest } from "@semio-tech/ui-react";
import { meshAssetTransportUrl } from "@semio-tech/framework/assets/mesh";
import { MESH_DELIVERY_CATALOG } from "@semio-tech/framework/assets/mesh";

/** 🖼️ Renders preview and exported shots through one asset transport boundary. */
export async function renderIconRequest(request: IconRenderRequest, signal?: AbortSignal) {
  signal?.throwIfAborted();
  const errors = validateJsonSchemaSubset(schema, request);
  if (errors.length) throw new Error("Icon render request does not match the export schema");
  return iconRenderPort.render({ ...request, assetUrl: publishedPageUrl(meshAssetTransportUrl(request.assetUrl, MESH_DELIVERY_CATALOG)) }, signal);
}
