import { iconRenderPort, type IconRenderRequest } from "@semio-tech/ui-react";
import { meshAssetTransportUrl } from "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts";

/** 🖼️ Renders preview and exported shots through one asset transport boundary. */
export async function renderIconRequest(request: IconRenderRequest) {
  return iconRenderPort.render({ ...request, assetUrl: meshAssetTransportUrl(request.assetUrl) });
}
