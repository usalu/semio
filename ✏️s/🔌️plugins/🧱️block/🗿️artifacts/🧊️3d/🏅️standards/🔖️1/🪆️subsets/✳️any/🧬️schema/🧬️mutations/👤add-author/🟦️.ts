/** 👤 `AddAuthor` mutation payload — mirrors `🦀️.rs`. */

import type { BlockAuthor } from "@semio-tech/block-js";

export interface AddAuthor {
  author: BlockAuthor;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
