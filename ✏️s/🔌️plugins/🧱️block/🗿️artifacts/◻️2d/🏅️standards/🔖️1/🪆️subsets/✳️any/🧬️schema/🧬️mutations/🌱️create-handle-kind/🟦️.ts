/** 🌱️ `CreateHandleKind` mutation payload — mirrors `🦀️.rs`. */

import type { Block2dHandleKind } from "../../../../../../../🟦️";

export interface CreateHandleKind {
  handleKind: Block2dHandleKind;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
