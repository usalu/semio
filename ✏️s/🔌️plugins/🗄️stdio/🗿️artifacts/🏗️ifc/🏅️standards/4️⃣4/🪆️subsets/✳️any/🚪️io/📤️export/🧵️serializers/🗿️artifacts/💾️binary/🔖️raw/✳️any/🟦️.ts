/** 🚪️ IFC4 STEP export through the validated native boundary. */
import type {IfcSnapshot} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {BinarySnapshot} from "../../../../../../../../../../../../💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import {encodeIfc4, type Ifc4IoOptions} from "../../../../../../🟦️.ts";
export function serialize(snapshot: IfcSnapshot, options: Ifc4IoOptions = {}): BinarySnapshot {return {schema: "stdio.binary", bytes: [...encodeIfc4(snapshot, options)]};}
