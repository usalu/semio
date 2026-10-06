import type {Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 📍 `MoveVortex` mutation payload — mirrors `🦀️.rs`. */

export interface MoveVortex {
  id: string;
  newPosition: [Binary64, Binary64, Binary64];
  newDirection: [Binary64, Binary64, Binary64];
}
