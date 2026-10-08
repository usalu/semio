/** 🔖 `AddRepresentationTag` mutation payload — mirrors `🦀️.rs`. */

export interface AddRepresentationTag {
  id: string;
  tag: string;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
