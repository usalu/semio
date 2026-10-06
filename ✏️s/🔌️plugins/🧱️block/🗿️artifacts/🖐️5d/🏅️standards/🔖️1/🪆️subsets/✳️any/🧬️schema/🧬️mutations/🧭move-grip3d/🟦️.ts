import type {Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🧭 `MoveGrip3d` mutation payload — mirrors `🦀️.rs`. */

export interface MoveGrip3d {
  id: string;
  newPosition: [Binary64, Binary64, Binary64];
  newDirection: [Binary64, Binary64, Binary64];
}
