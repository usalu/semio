/** 🌱 `CreateVortexKind` mutation payload — mirrors `🦀️.rs`. */

import type { Block3dVortexKind } from "../../../../../../../🟦️";

export interface CreateVortexKind {
  vortexKind: Block3dVortexKind;
  /** 📍️ Zero-based slot to insert at; appended when omitted. */
  index?: number;
}
