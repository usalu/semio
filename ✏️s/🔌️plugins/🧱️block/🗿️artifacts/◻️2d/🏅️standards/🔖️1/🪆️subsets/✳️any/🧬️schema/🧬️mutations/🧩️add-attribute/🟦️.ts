/** 🧩️ `AddAttribute` mutation payload — mirrors `🦀️.rs`. */

import type { BlockAttribute } from "@semio-tech/block-js";

export interface AddAttribute {
  attribute: BlockAttribute;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
