/** 🌱 `CreateGripKind` mutation payload — mirrors `🦀️.rs`. */

import type { Block5dGripKind } from "../../../../../../../🟦️";

export interface CreateGripKind {
  gripKind: Block5dGripKind;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
