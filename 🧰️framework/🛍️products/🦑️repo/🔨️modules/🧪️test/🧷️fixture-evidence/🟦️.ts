/** 🧷️ Fixture evidence
 *
 * Fixture content digests, evidence problems and fixture verification.
 */

//#region 🔌️Adapters
import { type TestEvidence, EVIDENCE_CLASSES, MUTATION_OUTCOME_CLASSES } from "@semio-tech/framework/test/adapter";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { setDigest } from "../🔢️digests/🟦️.ts";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
import { isWildcardSubset, isWildcardSubsetFor } from "../🪆️subset-policy/🟦️.ts";
import { MANIFEST_MUTATION_ID_RE } from "../📑️mutation-manifest/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧫️Fixture
/** 🧫️ What a fixture IS. Every fixture is exactly one of these — there is no fourth, unlabelled kind. */



/** 🧫️ One file inside a fixture bundle, addressed by ROLE so nothing downstream names a path. */


/** 🏭️ Exactly how a third-party-generated fixture was produced, in enough detail to re-run it. */


/** 📜️ Where a fixture came from and under what licence it may be committed. */


/** 📐️ The units and frame a fixture's numbers are expressed in. Absent units make every metric meaningless. */


/** ⚠️ A fixture-level loosening of its tolerance profile — every field mandatory, and always reported. */


/** 🧫️ One immutable fixture bundle with complete provenance. */


export const SHA256_RE = /^sha256:[0-9a-f]{64}$/;

/** #⃣ The canonical `sha256:<hex>` digest of a file's bytes, as every manifest and blob id spells it. */
export function contentDigest(bytes: Uint8Array): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

/** #⃣ The canonical content digest of a file on disk. */
export function contentDigestOf(absPath: string): string {
  return contentDigest(readFileSync(absPath));
}

/**
 * 🧾️ Validates one fixture manifest strictly. A fixture with incomplete provenance is a contract failure.
 *
 * 🎚️ A fixture whose target names a `surface` measures an editor or viewer state lane hosted by its subset rather
 * than a document scope, so the subset's siblings never make it a wildcard — the exemption its mutation manifest has.
 */
export function testEvidenceProblems(value: unknown, repoRoot?: string): string[] {
  if (!isPlainObject(value)) return ["test evidence configuration is not an object"];
  const problems: string[] = [];
  if (Object.hasOwn(value, "schema")) problems.push("examples cannot declare a separate fixture schema");
  if (typeof value.id !== "string" || !MANIFEST_MUTATION_ID_RE.test(value.id)) problems.push("id must be kebab-case");
  if (!(EVIDENCE_CLASSES as readonly string[]).includes(String(value.class))) problems.push(`class must be one of ${EVIDENCE_CLASSES.join("|")}`);
  if (!isPlainObject(value.target)) problems.push("target must name artifact, standard and subset");
  else {
    for (const key of ["artifact", "standard", "subset"] as const) if (typeof (value.target as Record<string, unknown>)[key] !== "string") problems.push(`target.${key} must be a string`);
    const subset = String((value.target as Record<string, unknown>).subset ?? "");
    // 🪆️Settled the same way a MUTATION's scope is, when the repository is available to ask. A subset
    // literally named `any` is a hard breach only when the artifact has real sibling subsets to be
    // scoped against; an artifact with exactly one subset is not "everything", it is a naming choice,
    // and `subsetPolicy: "single"` settles it. Judging fixtures by the bare spelling while judging
    // mutations by the resolved rule made 27 fixtures of genuinely single-subset owners unregisterable.
    const artifact = String((value.target as Record<string, unknown>).artifact ?? "");
    const standard = String((value.target as Record<string, unknown>).standard ?? "");
    const surface = (value.target as Record<string, unknown>).surface;
    if (surface !== undefined && (typeof surface !== "string" || surface.length === 0)) problems.push("target.surface must be a non-empty string when present");
    const wildcard = typeof surface === "string" ? false : repoRoot === undefined ? isWildcardSubset(subset) : isWildcardSubsetFor(repoRoot, artifact, standard, subset);
    if (wildcard) problems.push(`target.subset ${JSON.stringify(subset)} is a wildcard`);
  }
  if (value.outcome !== undefined && !(MUTATION_OUTCOME_CLASSES as readonly string[]).includes(String(value.outcome))) problems.push(`outcome must be one of ${MUTATION_OUTCOME_CLASSES.join("|")}`);
  if (!isPlainObject(value.units) || typeof value.units.length !== "string" || typeof value.units.angle !== "string") problems.push("units must declare length and angle");
  if (!Array.isArray(value.files) || value.files.length === 0) problems.push("files must be a non-empty array");
  else {
    const roles = new Set<string>();
    for (const [index, file] of value.files.entries()) {
      if (!isPlainObject(file)) {
        problems.push(`files[${index}] is not an object`);
        continue;
      }
      for (const key of ["role", "path", "mediaType"] as const) if (typeof file[key] !== "string" || (file[key] as string).length === 0) problems.push(`files[${index}].${key} must be a non-empty string`);
      if (typeof file.sha256 !== "string" || !SHA256_RE.test(file.sha256)) problems.push(`files[${index}].sha256 must be sha256:<64 hex>`);
      const role = String(file.role ?? "");
      if (roles.has(role)) problems.push(`files[${index}].role ${role} is duplicated — a role addresses exactly one file`);
      roles.add(role);
    }
  }
  if (!isPlainObject(value.provenance) || typeof value.provenance.license !== "string" || (value.provenance.license as string).length === 0) problems.push("provenance.license is required — a fixture without an acceptable licence is a contract failure, not an undocumented exception");
  if (typeof value.comparisonProfile !== "string" || value.comparisonProfile.length === 0) problems.push("comparisonProfile must be a non-empty string");
  if (typeof value.reproducible !== "boolean") problems.push("reproducible must be a boolean");
  if (value.class === "third-party-generated") {
    if (!isPlainObject(value.generator)) problems.push("a third-party-generated fixture must record its generator");
    else for (const key of ["oracle", "packageVersion", "engineFamily", "engineVersion", "command", "platform"] as const) {
      if (typeof (value.generator as Record<string, unknown>)[key] !== "string" || String((value.generator as Record<string, unknown>)[key]).length === 0) problems.push(`generator.${key} must be a non-empty string`);
    }
  }
  if (value.toleranceOverride !== undefined) {
    const override = value.toleranceOverride;
    if (!isPlainObject(override)) problems.push("toleranceOverride must be an object");
    else {
      if (typeof override.reason !== "string" || override.reason.length < 20) problems.push("toleranceOverride.reason must state WHY in at least 20 characters");
      if (typeof override.measuredBaseline !== "number") problems.push("toleranceOverride.measuredBaseline must be the measured value the override is sized against");
      if (typeof override.factor !== "number" || override.factor < 1) problems.push("toleranceOverride.factor must be at least 1");
      if (typeof override.approvedBy !== "string" || override.approvedBy.length === 0) problems.push("toleranceOverride.approvedBy must name the approving owner");
    }
  }
  return problems;
}

/** 🧫️ Absolute path of one role inside a fixture bundle. */
export function fixtureFilePath(repoRoot: string, manifest: TestEvidence, role: string): string {
  const file = manifest.files.find((entry) => entry.role === role);
  if (file === undefined) throw new Error(`fixture ${manifest.id} has no file for role ${JSON.stringify(role)}`);
  return isAbsolute(file.path) ? file.path : join(repoRoot, manifest.manifestDir ?? "", file.path);
}

/** 🧾️ One verified fixture file. `expected`/`actual` differ exactly when the committed bytes changed. */
export type FixtureVerification = Readonly<{ fixture: string; role: string; path: string; expected: string; actual: string; ok: boolean; missing: boolean }>;

/** 🧾️ Compares current example bytes with the recorded test evidence digest. */
export function verifyFixture(repoRoot: string, manifest: TestEvidence): FixtureVerification[] {
  return manifest.files.map((file) => {
    const abs = isAbsolute(file.path) ? file.path : join(repoRoot, manifest.manifestDir ?? "", file.path);
    if (!existsSync(abs)) return { fixture: manifest.id, role: file.role, path: file.path, expected: file.sha256, actual: "", ok: false, missing: true };
    const actual = contentDigestOf(abs);
    return { fixture: manifest.id, role: file.role, path: file.path, expected: file.sha256, actual, ok: actual === file.sha256, missing: false };
  });
}

/** #⃣ Order-independent digest of every file in a fixture bundle — one component of the run key. */
export function fixtureBundleDigest(manifest: TestEvidence): string {
  return setDigest(manifest.files.map((file) => [file.role, file.sha256] as const));
}
//#endregion 🧫️Fixture
