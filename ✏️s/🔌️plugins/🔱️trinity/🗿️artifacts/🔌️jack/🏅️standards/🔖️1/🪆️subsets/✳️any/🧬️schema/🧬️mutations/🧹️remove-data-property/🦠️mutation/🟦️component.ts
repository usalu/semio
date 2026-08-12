/** 🧹️ jack remove-data-property/🦠️mutation — payload mirror of `RemoveDataProperty`. */
import type { JackEntityRef } from "../../🔧️change-data-property/🦠️mutation/🟦️component.ts";

export interface RemoveDataProperty {
  entity: JackEntityRef;
  key: string;
}
