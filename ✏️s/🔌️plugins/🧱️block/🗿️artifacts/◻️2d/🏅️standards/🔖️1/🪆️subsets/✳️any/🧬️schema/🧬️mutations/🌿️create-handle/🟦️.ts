/** 🌿️ `CreateHandle` mutation payload — mirrors `🦀️.rs`. */

import type { Block2dHandleTemplate } from "../../../../../../../🟦️";

export interface CreateHandle {
  handle: Block2dHandleTemplate;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
