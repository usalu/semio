/** 🗜️ The shipped form of a catalog asset and the asset read from it: the header is decoded at once, the drawing geometry of each scale and the volume of each scale stay compressed until something reads them.
 *
 * An asset read here remembers the payload it came from ({@link packedAsset}), so it can travel to a worker as it was shipped: a structured clone of the asset itself visits every property, which would decode the geometry of every scale on the page and copy all of it.
 *
 * @see ../../../🌐️vite/🟦️.ts — `StaffageServedPayload`, the module the build and the development server ship
 * @see ../../🧮️pipeline/🏁️drawing/queue.ts — the drawing worker's mailbox
 * @see ../../🌊️flow/🧵️server/wire.ts — the air flow worker's wire
 */
import { inflateZlib } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🗜️deflate/🟦️.ts";
import type { Asset } from "@teaching/staffage";
import type { StaffageServedPayload } from "../../../🌐️vite/🟦️.ts";

/** 📦️ The compressed parts of a shipped asset: its header, the drawing geometry per scale and the volume per scale. */
export type PackedAsset = Pick<StaffageServedPayload, "header" | "scales" | "drawings">;

const packed = new WeakMap<Asset, PackedAsset>();

function decodeGeometry(text: string): unknown {
  const bytes = Uint8Array.from(atob(text), (char) => char.charCodeAt(0));
  return JSON.parse(new TextDecoder().decode(inflateZlib(bytes)));
}

function deferGeometry(target: object, key: string, text: string): void {
  Object.defineProperty(target, key, { enumerable: true, configurable: true, get() {
    const value = decodeGeometry(text);
    Object.defineProperty(target, key, { enumerable: true, value });
    return value;
  } });
}

/** 📖️ Reads the asset of a shipped payload; the geometry of a scale is decoded when it is first read. */
export function unpackAsset(payload: PackedAsset): Asset {
  const asset = decodeGeometry(payload.header) as Asset;
  if (payload.scales) {
    const volumes = {};
    for (const [scale, text] of Object.entries(payload.scales)) deferGeometry(volumes, scale, text);
    Object.defineProperty(asset, "volumes", { enumerable: true, value: volumes });
  }
  for (const [scale, text] of Object.entries(payload.drawings ?? {})) deferGeometry(asset.representations[scale as keyof Asset["representations"]], "geometry", text);
  packed.set(asset, { header: payload.header, ...(payload.scales && { scales: payload.scales }), ...(payload.drawings && { drawings: payload.drawings }) });
  return asset;
}

/** 📮️ The payload an asset was read from, or nothing for an asset that was not shipped compressed. */
export const packedAsset = (asset: Asset): PackedAsset | undefined => packed.get(asset);
