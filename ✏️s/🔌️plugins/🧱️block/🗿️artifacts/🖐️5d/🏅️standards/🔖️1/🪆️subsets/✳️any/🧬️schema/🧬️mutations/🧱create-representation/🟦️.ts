/** 🧱 `CreateRepresentation` mutation payload — mirrors `🦀️.rs`. */

import type { BlockRepresentation } from "@semio-tech/block-js";

export interface CreateRepresentation {
  representation: BlockRepresentation;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
