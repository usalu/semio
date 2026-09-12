import { canonicalPrimaryFilenameForKind } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { loadCatalogTaxonomy, taxonomyRelativePathIsExcluded } from "../../../🔍️discovery/🟦️.ts";
import { isAbsolute } from "node:path";

/** 🧹️Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"📐️cad"` -> `"cad"`, `"🗣️dsl"` -> `"dsl"`. */
export function policyStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

export const POLICY_MUTATIONS_FACET = "🧬️mutations";

export const POLICY_MUTATION_PLAN_DIR = "🧩️plan";

export const POLICY_TS_COMPONENT_LEAF = canonicalPrimaryFilenameForKind(loadCatalogTaxonomy().componentFileKinds["🟦️typescript"]!, loadCatalogTaxonomy());

export const POLICY_RS_COMPONENT_LEAF_NAME = canonicalPrimaryFilenameForKind(loadCatalogTaxonomy().componentFileKinds["🦀️rust"]!, loadCatalogTaxonomy());

//#endregion 🔧️PolicyRuleHandcraftedSpecP3

//#region 🔧️PolicyRuleMutationArtifactEngines
/**
 * 🧬️Wave 2b mutation / artifact-engine scanners (OPERATIONS-TO-MUTATIONS).
 * Missing `🧬️mutations` / triad / `⚙️engine` / `start mutation` report as breaches so Wave 2+ can track
 * unmigrated artifacts; dispatch-enum coverage stays a deliberate placeholder until Wave 3 pilot lands.
 */

/** 🏷️Leading emoji prefix of a taxonomy dir name (everything before the ASCII stem). */
export function policyLeadingEmojiPrefix(name: string): string {
  const ascii = policyStripEmoji(name);
  if (!ascii) return name;
  const idx = name.indexOf(ascii);
  return idx > 0 ? name.slice(0, idx) : "";
}


export function policyStructuralRelativeLocator(value: string): string | null {
  if (!value || /^[A-Za-z]:[\\/]/u.test(value) || /^\\\\/u.test(value) || isAbsolute(value) || /[\u0000-\u001F\u007F\u2028\u2029]/u.test(value)) return null;
  const normalized = value.replaceAll("\\", "/").replace(/^\.\//u, ""), segments = normalized.split("/");
  return segments.some((segment) => !segment || segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose") || taxonomyRelativePathIsExcluded(normalized) ? null : normalized;
}


/**
 * 🗿️The owning artifact root for a `🧬️mutations` facet dir — everything above
 * `🏅️standards/…`, else the dir's own parent. Used as the `scope` on mutation breaches so they
 * report against `✏️s/🔌️plugins/<p>/🗿️artifacts/<a>` rather than the full nested facet path.
 */
export function policyArtifactRootOfMutationsDir(mutationsRel: string): string {
  const marker = mutationsRel.indexOf("/🏅️standards/");
  if (marker > 0) return mutationsRel.slice(0, marker);
  const parts = mutationsRel.split("/");
  parts.pop();
  return parts.join("/");
}
