/** 🌿 `CreateGrip` mutation payload — mirrors `🦀️.rs`. */

import type { Block5dGripTemplate } from "../../../../../../../🟦️";

export interface CreateGrip {
  grip: Block5dGripTemplate;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
