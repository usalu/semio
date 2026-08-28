//#region 🪪️FixedMetadataEnvelope
import type { ResidentResources } from "../../../../../🌱️value/💾️resident/🟦️component.ts";

export type UiResidentMetadataKind = "pool" | "instance" | "payload" | "builder" | "reader" | "page" | "evidence";
const ENVELOPES = Object.freeze({
  pool: Object.freeze({ bytes: 192, slots: 4, owners: 4 }),
  instance: Object.freeze({ bytes: 304, slots: 5, owners: 5 }),
  payload: Object.freeze({ bytes: 232, slots: 3, owners: 3 }),
  builder: Object.freeze({ bytes: 264, slots: 2, owners: 2 }),
  reader: Object.freeze({ bytes: 112, slots: 2, owners: 2 }),
  page: Object.freeze({ bytes: 232, slots: 4, owners: 4 }),
  evidence: Object.freeze({ bytes: 152, slots: 3, owners: 3 }),
});
/** 🪪️ Fixed UI metadata allowance; neutral registration and intrinsic storage remain separately charged. */
export function uiResidentMetadataEnvelope(kind: UiResidentMetadataKind): ResidentResources {
  if (typeof kind !== "string" || !Object.hasOwn(ENVELOPES, kind)) throw new Error("Invalid UI resident metadata kind");
  return ENVELOPES[kind];
}
//#endregion 🪪️FixedMetadataEnvelope
