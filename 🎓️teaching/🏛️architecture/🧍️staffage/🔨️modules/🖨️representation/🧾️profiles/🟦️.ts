/** 🧾️ The print profiles of the archive, bundled from `🗂️catalog/🖨️print-profiles/🔣️.json`, so browsers and command-line tools read the same values.
 *
 * @see ../../../🗂️catalog/🖨️print-profiles/🔣️.json — the profile document
 * @see ../../../🧬️schema/🟦️.ts — `PrintProfile`
 */
import type { PrintProfile, PrintProfileDocument, Slug } from "../../../🧬️schema/🟦️.ts";
import document from "../../../🗂️catalog/🖨️print-profiles/🔣️.json";

/** 🖨️ The bundled print profile document. */
export const PRINT_PROFILES: PrintProfileDocument = document as unknown as PrintProfileDocument;

/** 🖨️ The profile with the id, or an error naming the profiles there are. */
export function printProfile(id: Slug): PrintProfile {
  const profile = PRINT_PROFILES.profiles.find((candidate) => candidate.id === id);
  if (profile === undefined) throw new Error(`no print profile "${id}"; profiles: ${PRINT_PROFILES.profiles.map((p) => p.id).join(", ")}`);
  return profile;
}

/** 🖨️ The default profile of the archive (`defaultProfile` of the document). */
export const DEFAULT_PRINT_PROFILE: PrintProfile = printProfile(PRINT_PROFILES.defaultProfile);
