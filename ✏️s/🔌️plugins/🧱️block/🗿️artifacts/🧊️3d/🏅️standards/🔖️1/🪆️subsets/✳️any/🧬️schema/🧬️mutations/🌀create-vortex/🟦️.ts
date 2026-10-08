/** 🌀 `CreateVortex` mutation payload — mirrors `🦀️.rs`. */

import type { Block3dVortexTemplate } from "../../../../../../../🟦️";

export interface CreateVortex {
  vortex: Block3dVortexTemplate;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
