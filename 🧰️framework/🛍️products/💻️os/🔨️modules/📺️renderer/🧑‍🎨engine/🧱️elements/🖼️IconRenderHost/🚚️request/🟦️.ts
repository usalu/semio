import { publishedPageUrl } from "../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { iconRenderPort, type IconRenderRequest } from "@semio-tech/ui-react";
import { meshAssetTransportUrl } from "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts";
import { MESH_DELIVERY_CATALOG } from "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/📇️catalog/🟦️.ts";

/** 🖼️ Renders preview and exported shots through one asset transport boundary. */
export async function renderIconRequest(request: IconRenderRequest) {
  return iconRenderPort.render({ ...request, assetUrl: publishedPageUrl(meshAssetTransportUrl(request.assetUrl, MESH_DELIVERY_CATALOG)) });
}
