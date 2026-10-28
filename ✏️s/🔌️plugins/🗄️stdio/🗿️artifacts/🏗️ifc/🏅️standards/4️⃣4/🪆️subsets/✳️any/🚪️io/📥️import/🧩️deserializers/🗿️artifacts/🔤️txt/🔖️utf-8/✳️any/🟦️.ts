/** 🚪️ IFC4 STEP import through the validated native boundary. */
import type {IfcSnapshot} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {TxtSnapshot} from "../../../../../../../../../../../../🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {decodeIfc4, type Ifc4IoOptions} from "../../../../../../🟦️.ts";
export function deserialize(snapshot: TxtSnapshot, options: Ifc4IoOptions = {}): IfcSnapshot {
  const separator = snapshot.lineEnding === "crLf" ? "\r\n" : "\n";
  return decodeIfc4(snapshot.lines.join(separator) + (snapshot.trailingNewline ? separator : ""), options);
}
