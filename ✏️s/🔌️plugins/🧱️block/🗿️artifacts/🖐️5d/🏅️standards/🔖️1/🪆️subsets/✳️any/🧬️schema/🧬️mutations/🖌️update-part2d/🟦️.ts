import type {Binary64} from "@semio-tech/framework/value/ieee754";
/** 🖌️ `UpdatePart2d` mutation payload — mirrors `🦀️.rs`. */

export interface UpdatePart2d {
  newShape: string|null;
  newRadius: Binary64|null;
  newWidth: Binary64|null;
  newHeight: Binary64|null;
  newColor: string|null;
  newIconKind: string|null;
}
