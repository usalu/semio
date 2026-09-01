//#region 🪪️FixedMetadataEnvelope
import type { ResidentResources } from "../../../../../🌱️value/💾️resident/🟦️component.ts";

export type UiResidentMetadataKind = "pool" | "instance" | "payload" | "builder" | "reader" | "page" | "evidence";
const ENVELOPES = Object.freeze({
  pool: Object.freeze({ bytes: 264, slots: 5, owners: 5 }),
  instance: Object.freeze({ bytes: 376, slots: 6, owners: 6 }),
  payload: Object.freeze({ bytes: 312, slots: 4, owners: 4 }),
  builder: Object.freeze({ bytes: 296, slots: 3, owners: 3 }),
  reader: Object.freeze({ bytes: 160, slots: 3, owners: 3 }),
  page: Object.freeze({ bytes: 160, slots: 3, owners: 3 }),
  evidence: Object.freeze({ bytes: 192, slots: 4, owners: 4 }),
});
/** 🪪️ Fixed UI metadata allowance; neutral registration and intrinsic storage remain separately charged. */
export function uiResidentMetadataEnvelope(kind: UiResidentMetadataKind): ResidentResources {
  if (typeof kind !== "string" || !Object.hasOwn(ENVELOPES, kind)) throw new Error("Invalid UI resident metadata kind");
  return ENVELOPES[kind];
}
//#endregion 🪪️FixedMetadataEnvelope
