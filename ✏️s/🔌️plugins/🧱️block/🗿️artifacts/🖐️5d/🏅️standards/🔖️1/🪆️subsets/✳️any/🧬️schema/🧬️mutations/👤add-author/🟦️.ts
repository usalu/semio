/** 👤 `AddAuthor` mutation payload — mirrors `🦀️.rs`. */

import type { BlockAuthor } from "../../../../../../../../../🟦️";

export interface AddAuthor {
  author: BlockAuthor;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
