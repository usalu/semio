/** 🔺️ IFC2x3 domain diff fields with exact instance identity and optional EDM change. */
import type {Ifc2x3EdmPreamble,Part21Header,Part21Instance} from "../📸️snapshot/🟦️.ts";
export interface Ifc2x3Diff {
  schema?: string;
  header?: Part21Header;
  removedInstances?: Part21Instance["id"][];
  upsertedInstances?: Part21Instance[];
  edmPreamble?: Ifc2x3EdmPreamble | null;
  instanceOrder?: Part21Instance["id"][];
}
