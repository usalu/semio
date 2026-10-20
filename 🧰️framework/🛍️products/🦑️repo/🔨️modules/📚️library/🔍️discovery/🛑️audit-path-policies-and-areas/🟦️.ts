import { foldPathEmojiIdentity } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🛣️path/🟦️.ts";
import { OPAQUE_PATH_EXCLUSIONS } from "../👥️mutation-owner-rules/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🛑️ Audits the opaque path exclusions, the Unicode, variation selector, path emoji and collision policies and the area enforcement and area states. */
export function auditPathPoliciesAndAreas(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, ids } = audit;
  if (record(taxonomy.pathExclusions, "pathExclusions")) {
    const entries = Object.entries(taxonomy.pathExclusions);
    if (entries.map(([id]) => id).join("\0") !== OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join("\0")) problems.push(`pathExclusions must contain exactly ordered ${OPAQUE_PATH_EXCLUSIONS.map(([id]) => JSON.stringify(id)).join(", ")} contracts.`);
    for (const [id, path] of OPAQUE_PATH_EXCLUSIONS) {
      const contract = taxonomy.pathExclusions[id];
      if (!contract || contract.path !== path || contract.mode !== "opaque" || !contract.reason) problems.push(`pathExclusions.${id} must be the exact opaque ${JSON.stringify(path)} contract.`);
    }
  }
  if (taxonomy.unicodeNormalization?.form !== "NFC" || taxonomy.unicodeNormalization?.caseFold !== "lower" || taxonomy.unicodeNormalization?.locale !== "und") problems.push("unicodeNormalization must be NFC/lower/und.");
  if (taxonomy.variationSelectorPolicy?.selector !== "\uFE0F" || taxonomy.variationSelectorPolicy?.requiredAfterEmoji !== true || taxonomy.variationSelectorPolicy?.comparison !== "ignore-selector") problems.push("variationSelectorPolicy is invalid.");
  if (taxonomy.pathEmojiPolicy?.inventory !== "git-visible" || taxonomy.pathEmojiPolicy?.identity !== "single-emoji-grapheme" || taxonomy.pathEmojiPolicy?.siblingNamespace !== "files-and-directories") problems.push("pathEmojiPolicy inventory, identity, and sibling namespace are invalid.");
  if (taxonomy.pathEmojiPolicy?.genericEmojiIdentities?.map(foldPathEmojiIdentity).join("\0") !== "📁\0📂\0📄") problems.push("pathEmojiPolicy.genericEmojiIdentities must reject the three generic file/folder glyphs.");
  if (!Array.isArray(taxonomy.pathEmojiPolicy?.reservedSubtreeDirectoryNames) || new Set(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map(foldPathEmojiIdentity)).size !== taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.length) problems.push("pathEmojiPolicy.reservedSubtreeDirectoryNames must be a unique array.");
  const comparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (taxonomy.collisionPolicy?.comparisons?.join("\0") !== comparisons.join("\0")) problems.push(`collisionPolicy.comparisons must be exactly ${comparisons.join(", ")}.`);
  if (taxonomy.collisionPolicy?.maxPathUtf16Units !== 256 || taxonomy.collisionPolicy?.maxComponentUtf8Bytes !== 255 || taxonomy.collisionPolicy?.rejectWindowsReservedNames !== true || taxonomy.collisionPolicy?.rejectTrailingDotsAndSpaces !== true) problems.push("collisionPolicy platform constraints must retain maxPathUtf16Units 256, maxComponentUtf8Bytes 255 and reject reserved/trailing names.");
  if (taxonomy.areaEnforcement?.requiredState !== "clean" || taxonomy.areaEnforcement?.undeclaredAreas !== "enforce") problems.push("areaEnforcement must enforce clean declared and undeclared areas.");
  ids(taxonomy.areaEnforcement?.opaquePathExclusionIds, taxonomy.pathExclusions, "areaEnforcement.opaquePathExclusionIds");
  if (taxonomy.areaEnforcement?.opaquePathExclusionIds?.join("\0") !== OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join("\0")) problems.push(`areaEnforcement.opaquePathExclusionIds must be exactly [${OPAQUE_PATH_EXCLUSIONS.map(([id]) => JSON.stringify(id)).join(", ")}].`);
  const exemptAreaRoots = [...(taxonomy.pathEmojiPolicy?.reservedSubtreeDirectoryNames ?? []), ...Object.values(taxonomy.pathExclusions ?? {}).map((exclusion) => exclusion.path.replaceAll("\\", "/").replace(/\/+$/u, ""))];
  if (record(taxonomy.areas, "areas")) for (const [area, state] of Object.entries(taxonomy.areas)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must exist only in pathExclusions.");
    if (state === "exempt") {
      if (!exemptAreaRoots.some((root) => root.length > 0 && (area === root || area.startsWith(`${root}/`)))) problems.push(`areas[${JSON.stringify(area)}] may only be "exempt" inside a reserved subtree directory or a declared path exclusion, which discovery already skips.`);
    } else if (state !== "clean") problems.push(`areas[${JSON.stringify(area)}] must be "clean" or "exempt".`);
  }
}
