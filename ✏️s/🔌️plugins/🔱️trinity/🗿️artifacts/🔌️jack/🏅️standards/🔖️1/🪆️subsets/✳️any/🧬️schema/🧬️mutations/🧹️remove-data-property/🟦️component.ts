/** 🧹️ jack direct `remove-data-property` payload mirror of `RemoveDataProperty`. */
import type { JackEntityRef } from "../🔧️change-data-property/🟦️component.ts";

export interface RemoveDataProperty {
  entity: JackEntityRef;
  key: string;
}
