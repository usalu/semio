import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** 🎥 `MoveCamera3d` mutation payload — mirrors `🦀️.rs`. */

export interface MoveCamera3d {
  newPosition: [Binary64, Binary64, Binary64];
  newTarget: [Binary64, Binary64, Binary64];
}
