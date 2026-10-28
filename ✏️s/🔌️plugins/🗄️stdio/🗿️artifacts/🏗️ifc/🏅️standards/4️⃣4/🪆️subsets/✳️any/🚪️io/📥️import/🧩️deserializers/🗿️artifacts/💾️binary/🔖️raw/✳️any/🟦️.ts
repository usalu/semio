/** 🚪️ IFC4 STEP import through the validated native boundary. */
import type {IfcSnapshot} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {BinarySnapshot} from "../../../../../../../../../../../../💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {decodeIfc4, type Ifc4IoOptions} from "../../../../../../🟦️.ts";
export function deserialize(snapshot: BinarySnapshot, options: Ifc4IoOptions = {}): IfcSnapshot {
  if (snapshot.bytes.some(byte => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error("ifc4: invalid byte");
  return decodeIfc4(Uint8Array.from(snapshot.bytes), options);
}
