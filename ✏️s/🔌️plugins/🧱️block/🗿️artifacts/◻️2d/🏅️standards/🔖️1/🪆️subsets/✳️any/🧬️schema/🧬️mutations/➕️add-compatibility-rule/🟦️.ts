/** ➕️ `AddCompatibilityRule` mutation payload — mirrors `🦀️.rs`. */

import type { BlockCompatibilityRule } from "../../../../../../../../../🟦️";

export interface AddCompatibilityRule {
  rule: BlockCompatibilityRule;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
