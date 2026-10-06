/** 📝️ Canonical sparse diff text beside its decoded value and the derived inverse/touched regions.
 * @see ./🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfDiff, parseGltfDiffDerivation, type GltfDiff, type GltfDiffDerivation } from "../../../🧬️schema/🔺️diff/🟦️.ts";

export interface GltfDiffTextDocument {
  text: string;
  value: GltfDiff;
  derivation: GltfDiffDerivation;
}
export type GltfDiffText = string;

export const parseGltfDiffTextDocument = gltfWireObject<GltfDiffTextDocument>({ text: gltfWireRequired(gltfWireString), value: gltfWireRequired(parseGltfDiff), derivation: gltfWireRequired(parseGltfDiffDerivation) });
