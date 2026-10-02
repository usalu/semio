import type { AssetDeliveryProviderV1 } from "../../../../../../../../🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts";
import { MESH_COLLECTION_ASSET_PROVIDER_V1, STATIC_DIRECTORY_ASSET_PROVIDER_V1, TILE_PROXY_ASSET_PROVIDER_V1 } from "../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

/** 🧩️ Registers the asset providers owned by the concrete playground composition. */
export const PLAYGROUND_ASSET_PROVIDERS_V1: readonly AssetDeliveryProviderV1[] = Object.freeze([TILE_PROXY_ASSET_PROVIDER_V1, MESH_COLLECTION_ASSET_PROVIDER_V1, STATIC_DIRECTORY_ASSET_PROVIDER_V1]);
