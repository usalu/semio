import { existsSync } from "node:fs";
import { join } from "node:path";
import { policyListArtifactDialectDirs } from "../../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { POLICY_STDIO_ARTIFACTS_REL } from "../../🚪️artifact-io/📋️owner-table/🟦️.ts";
import { policyStripEmoji } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";

/** 🗂️One (artifact, standard, subset) triple under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, restricted to real on-disk dirs (dynamic — NOT a hardcoded "31/32" count, so a future new standard/subset is picked up automatically). */
type PolicyStdioStandardEntry = { artRel: string; artifactId: string; standardSlug: string; subsetRel: string; subsetId: string };

function policyListStdioStandardEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  const out: PolicyStdioStandardEntry[] = [];
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialect.artRel.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    out.push({
      artRel: dialect.artRel,
      artifactId: policyStripEmoji(dialect.artRel.slice(POLICY_STDIO_ARTIFACTS_REL.length + 1)),
      standardSlug: dialect.standardSlug,
      subsetRel: dialect.subsetRel,
      subsetId: dialect.subsetId,
    });
  }
  return out;
}

/**
 * 🔎️Only the SCHEMA-OWNING subset entries (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES,
 * replaces the old exact-string `subsetId === "✳️any"` filter). A subset is schema-owning iff its
 * `🧬️schema/📸️snapshot/` dir exists on disk — a real conformance subset (pdf 1.7's `✳️a`, step's
 * `✳️cc6`, …) is a validation-gated DELEGATING stamp on top of its standard's owning subset (pure
 * `pub use …::subsets::any::schema::*`, per the pilot), never a new snapshot type, so it never has
 * its own `📸️snapshot/` dir. `✳️any` is always schema-owning by construction. This is structural
 * rather than name-based so a future new subset never needs an allowlist/filter edit here — it is
 * automatically excluded from schema-internal rules (DiffAlgebra, field-sweep, grammar-honesty,
 * facet-mirror-drift) the moment it's created as a delegating subset, and automatically INCLUDED in
 * builder/analyzer/composer/io rules (those already iterate the full `policyListStdioStandardEntries`
 * result, unfiltered).
 */
export function policyListStdioSchemaOwningEntries(repoRoot: string): PolicyStdioStandardEntry[] {
  return policyListStdioStandardEntries(repoRoot).filter((e) => existsSync(join(repoRoot, e.subsetRel, "🧬️schema", "📸️snapshot")));
}

export function policyStdioStandardKey(artifactId: string, standardSlug: string): string {
  return `stdio/${artifactId}/standards#${standardSlug}`;
}
