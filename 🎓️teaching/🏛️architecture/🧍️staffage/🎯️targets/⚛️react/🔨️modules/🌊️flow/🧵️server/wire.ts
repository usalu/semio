/** 🔌️ What crosses to the air flow worker: the messages of the solver, with a catalog asset as the compressed payload it was shipped as instead of the decoded asset.
 * @see ./🟦️.ts — `FlowMessage`
 * @see ../../📚️catalog/🗜️payload/🟦️.ts — why an asset travels compressed */
import { packedAsset, unpackAsset, type PackedAsset } from "../../📚️catalog/🗜️payload/🟦️.ts";
import type { FlowMessage } from "./🟦️.ts";

/** 📨️ A solver message as it travels. */
export type FlowWire = FlowMessage | { readonly kind: "packed"; readonly id: number; readonly packed: PackedAsset };

/** 📤️ Puts a message on the wire. */
export function flowWireOut(message: FlowMessage): FlowWire {
  const packed = message.kind === "asset" ? packedAsset(message.asset) : undefined;
  return packed === undefined ? message : { kind: "packed", id: (message as { readonly id: number }).id, packed };
}

/** 📥️ Takes a message off the wire. */
export const flowWireIn = (wire: FlowWire): FlowMessage => (wire.kind === "packed" ? { kind: "asset", id: wire.id, asset: unpackAsset(wire.packed) } : wire);
