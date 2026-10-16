import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** 📍 `MoveVortex` mutation payload — mirrors `🦀️.rs`. */

export interface MoveVortex {
  id: string;
  newPosition: [Binary64, Binary64, Binary64];
  newDirection: [Binary64, Binary64, Binary64];
}
