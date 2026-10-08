/** 🧩 `AddRepresentationAttribute` mutation payload — mirrors `🦀️.rs`. */

import type { BlockAttribute } from "../../../../../../../../../🟦️";

export interface AddRepresentationAttribute {
  id: string;
  attribute: BlockAttribute;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
