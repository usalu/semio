/** 🧩️ `AddAttribute` mutation payload — mirrors `🦀️.rs`. */

import type { BlockAttribute } from "../../../../../../../../../🟦️";

export interface AddAttribute {
  attribute: BlockAttribute;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
