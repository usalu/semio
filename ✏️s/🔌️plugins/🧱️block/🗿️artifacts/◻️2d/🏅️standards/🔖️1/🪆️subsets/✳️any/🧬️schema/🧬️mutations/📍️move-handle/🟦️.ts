import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** 📍️ `MoveHandle` mutation payload — mirrors `🦀️.rs`. */

export interface MoveHandle {
  id: string;
  newAngle: Binary64;
  newRadius: Binary64;
}
