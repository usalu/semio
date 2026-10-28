/** 🚪️ IFC4 STEP export through the validated native boundary. */
import type {IfcSnapshot} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {TxtSnapshot} from "../../../../../../../../../../../../🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {encodeIfc4, type Ifc4IoOptions} from "../../../../../../🟦️.ts";
export function serialize(snapshot: IfcSnapshot, options: Ifc4IoOptions = {}): TxtSnapshot {
  const text = new TextDecoder().decode(encodeIfc4(snapshot, options));
  return {schema: "stdio.txt", lines: text.slice(0, -1).split("\n"), trailingNewline: true, lineEnding: "lf"};
}
