/** 🧱 `CreateRepresentation` mutation payload — mirrors `🦀️.rs`. */

import type { BlockRepresentation } from "../../../../../../../../../🟦️";

export interface CreateRepresentation {
  representation: BlockRepresentation;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
