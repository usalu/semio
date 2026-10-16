import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import { iconRenderPort, type IconRenderRequest } from "@semio-tech/ui-react";
import { meshAssetTransportUrl } from "@semio-tech/framework/assets/mesh";
import { MESH_DELIVERY_CATALOG } from "@semio-tech/framework/assets/mesh";

/** 🖼️ Renders preview and exported shots through one asset transport boundary. */
export async function renderIconRequest(request: IconRenderRequest) {
  return iconRenderPort.render({ ...request, assetUrl: publishedPageUrl(meshAssetTransportUrl(request.assetUrl, MESH_DELIVERY_CATALOG)) });
}
