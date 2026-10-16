import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** 🧊 `UpdatePart3d` mutation payload — mirrors `🦀️.rs`. */

export interface UpdatePart3d {
  newOrientation: [Binary64, Binary64, Binary64, Binary64]|null;
  newScale: [Binary64, Binary64, Binary64]|null;
}
