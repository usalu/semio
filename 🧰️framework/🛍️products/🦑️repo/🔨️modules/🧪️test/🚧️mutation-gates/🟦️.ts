/** 🚧️ Mutation gates
 *
 * Gates over mutation inventories, oracle requirements, stubs, fixtures and provenance.
 */

//#region 🔌️Adapters
import { type EngineFamily } from "@semio-tech/framework/test/adapter";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { type BreachRecord } from "@semio-tech/repo-lib/policy";
import { isExcludedTestPath } from "../🗂️taxonomy/🟦️.ts";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
import { walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { isJustifiedMutationDecision } from "../🗃️oracle-registry/🟦️.ts";
import { declaredSubsets, isWildcardSubset, subsetCoordinate, subsetPolicyIsSingle, surfaceCoordinate, surfaceProblem } from "../🪆️subset-policy/🟦️.ts";
import { compareInventories, engineFamilyId, isQualifyingOracleKind, isSemioNativeArtifact, MANIFEST_MUTATION_ID_RE, type ManifestMutation, manifestTarget, type MutationManifest, mutationManifestProblems, owningSubsetOf, QUALIFYING_ORACLE_KINDS, readRuntimeInventory } from "../📑️mutation-manifest/🟦️.ts";
import { isQualifiedProbe } from "../🔬️probes/🟦️.ts";
import { type TestResult } from "../📤️results/🟦️.ts";
import { type OracleRegistry, probeTable, profileTable, toleranceProfileTable } from "../🤝️contributions/🟦️.ts";
import { testEvidenceProblems, verifyFixture } from "../🧷️fixture-evidence/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧾️ContractV2
/**
 * 🚨️ The v2 mutation-completeness gate. It compares PRODUCTION DISPATCH, the owner manifest and the
 * claimed test inventory for exact equality, which is the thing v1 could not do: v1's audit compared
 * a catalog with checked-in physical evidence and said so in its own comment, so a mutation reachable
 * through dispatch but missing from the catalog left no trace anywhere.
 *
 * A state-lane manifest (`surface` set) measures an editor or viewer surface mounted in the subset that hosts it: its
 * mutations change that surface's own config, presence or transient record, never a document subset, so the
 * smallest-document-subset rules (`wildcard-subset-owner`, `unsplit-artifact-subset`) do not apply to it.
 */
export function mutationInventoryBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const ownerOf = new Map<string, string[]>();

  for (const contribution of registry.contributions) {
    for (const [index, manifest] of contribution.mutationManifests.entries()) {
      const surfaceIssue = manifest.surface === undefined ? null : surfaceProblem(repoRoot, contribution.owner, manifest.surface);
      const problems = [...mutationManifestProblems(manifest, contribution.owner), ...(surfaceIssue === null ? [] : [surfaceIssue])];
      for (const problem of problems) {
        breaches.push(breach("testing/contract", "mutation-manifest-invalid", contribution.manifestPath, `mutationManifests[${index}] ${problem}`, "A manifest is the authority the runtime and the tests are both measured against; a malformed one measures nothing.", "Correct the manifest record."));
      }
      if (problems.length > 0) continue;

      const claimed = registry.mutationCatalogs.filter((catalog) => catalog.capability !== "" && manifest.mutations.some((mutation) => mutation.capability === catalog.capability)).flatMap((catalog) => catalog.kinds);
      const runtime = readRuntimeInventory(repoRoot, manifest);
      const equality = compareInventories(manifest, runtime, claimed);

      if (equality.runtimeMissing) {
        breaches.push(breach("testing/contract", "runtime-inventory-missing", contribution.manifestPath, `No runtime inventory has been produced for ${equality.target}`, "Runtime completeness is a measurement, not a claim. Without the production bridge's own answer the manifest is unverified.", "Run `test inventory --artifact <id> --standard <v> --subset <s>` so the production bridge emits its inventory."));
      }
      for (const id of equality.runtimeOnly) {
        breaches.push(breach("testing/contract", "runtime-only-mutation", contribution.manifestPath, `${equality.target}: mutation ${id} is reachable through production dispatch but appears in no owner manifest`, "An unowned executable mutation is untested surface that no coverage number can see.", "Add it to the owning subset's manifest with its outcomes, dispatch and oracle requirement."));
      }
      for (const id of equality.manifestOnly) {
        breaches.push(breach("testing/contract", "manifest-only-mutation", contribution.manifestPath, `${equality.target}: mutation ${id} is declared but production dispatch does not offer it`, "A manifest row with no dispatch behind it inflates coverage with a mutation nobody can invoke.", "Remove the row, or implement and register the dispatch variant."));
      }
      for (const id of equality.testOnly) {
        breaches.push(breach("testing/contract", "test-only-mutation", contribution.manifestPath, `${equality.target}: the test catalog claims mutation ${id}, which no manifest owns`, "A test-only kind measures the test suite against itself.", "Add the mutation to its owning manifest, or drop the catalog kind."));
      }
      for (const mismatch of equality.outcomeMismatches) {
        breaches.push(breach("testing/contract", "mutation-outcome-mismatch", contribution.manifestPath, `${equality.target}: mutation ${mismatch.mutation} declares outcomes [${mismatch.manifest.join(", ")}] but dispatch reports [${mismatch.runtime.join(", ")}]`, "A fixture declares one expected outcome class; if the declared set disagrees with dispatch, some reachable outcome has no fixture at all.", "Align the manifest's outcomes with the dispatch descriptor."));
      }
      for (const mismatch of equality.variantMismatches) {
        breaches.push(breach("testing/contract", "mutation-variant-mismatch", contribution.manifestPath, `${equality.target}: mutation ${mismatch.mutation} names dispatch variant ${mismatch.manifest} but the runtime reports ${mismatch.runtime}`, "Name-checked dispatch is what keeps manifest↔runtime equality from silently degrading into an order comparison.", "Correct productionDispatch.variant."));
      }

      for (const mutation of manifest.mutations) {
        const subset = owningSubsetOf(manifest, mutation);
        const coordinate = subsetCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset });
        const key = `${coordinate}${manifest.surface === undefined ? "" : `#${manifest.surface}`}::${mutation.id}`;
        ownerOf.set(key, [...(ownerOf.get(key) ?? []), `${contribution.manifestPath}#${coordinate}`]);
        if (manifest.surface === undefined && isWildcardSubset(subset)) {
          const siblings = (declaredSubsets(repoRoot).get(`${manifest.artifact}@${manifest.standard}`) ?? []).filter((candidate) => !isWildcardSubset(candidate));
          if (siblings.length > 0) {
            breaches.push(breach("testing/contract", "wildcard-subset-owner", contribution.manifestPath, `Mutation ${mutation.id} is owned by wildcard subset ${JSON.stringify(subset)} while ${manifest.artifact}@${manifest.standard} declares ${siblings.length} real subset(s): ${siblings.join(", ")}`, "Testing at artifact level hides which part of the artifact a mutation actually changed, and lets one broad case stand in for every narrow one.", `Give the mutation its smallest owner among ${siblings.join(", ")}, or declare an explicit typed compound.`));
          } else if (!subsetPolicyIsSingle(repoRoot, manifest.artifact, manifest.standard)) {
            breaches.push(breach("testing/contract", "unsplit-artifact-subset", contribution.manifestPath, `Mutation ${mutation.id} is owned by ${JSON.stringify(subset)} and ${manifest.artifact}@${manifest.standard} declares no narrower subset at all`, "A mutation is owned by the SMALLEST semantic subset. An artifact with a single catch-all subset is either a format that genuinely has no narrower scope, or one that has not been split yet — and until the owner says which, every mutation on it is scoped to the whole artifact.", `Declare the real subsets of ${manifest.artifact}@${manifest.standard} and give this mutation its smallest owner, or record "subsetPolicy": "single" with a rationale in that artifact's 🪆️subsets component.`, "medium"));
          }
        }
        breaches.push(...oracleRequirementBreaches(registry, contribution.manifestPath, manifest, mutation));
      }
    }
  }

  for (const [key, owners] of ownerOf) {
    if (owners.length > 1) {
      breaches.push(breach("testing/contract", "duplicate-mutation-owner", owners[0]!, `Mutation ${key} is owned by ${owners.length} manifests: ${owners.join(", ")}`, "Exactly one smallest semantic subset owns a mutation; two owners means two coverage numbers for one behaviour and no way to say which is right.", "Keep the narrowest owner and remove the others."));
    }
  }
  return breaches;
}

/**
 * ✅️ Every mutation needs a QUALIFYING third-party oracle. A second implementation written inside this
 * repository from this repository's own schemas is registered as `cross-semio-implementation` and is a
 * required supplement, never a substitute: both halves read the same specification, so a misreading
 * of it produces two agreeing wrong answers.
 */
export function oracleRequirementBreaches(registry: OracleRegistry, scope: string, manifest: MutationManifest, mutation: ManifestMutation): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const coordinate = subsetCoordinate(manifestTarget(manifest, mutation));
  for (const requirement of mutation.oracleRequirements) {
    const supplying = registry.oracles.filter((oracle) => oracle.capabilities.includes(requirement.capability));
    const qualifying = supplying.filter((oracle) => isQualifyingOracleKind(oracle.kind));
    const justified = registry.noOracleDecisions.find((decision) => decision.capabilities.includes(requirement.capability) && isJustifiedMutationDecision(decision));
    if (qualifying.length === 0 && justified !== undefined) {
      breaches.push(
        breach(
          "testing/oracle",
          "justified-reference-gap",
          scope,
          `${coordinate}: mutation ${mutation.id} has no ${requirement.qualifyingKind} for capability ${requirement.capability}; decision ${justified.id} records a surveyed negative search (${justified.referenceSurvey!.candidatesConsidered.map((candidate) => `${candidate.package}: ${candidate.verdict}`).join(", ")})`,
          "A surveyed gap is accepted so it does not block, and stays listed so it is revisited when a reference appears.",
          `Register a ${QUALIFYING_ORACLE_KINDS.join(" / ")} for ${requirement.capability} when one exists, then drop the capability from ${justified.id}.`,
          "medium",
        ),
      );
      continue;
    }
    if (qualifying.length === 0) {
      const supplemental = supplying.map((oracle) => `${oracle.id} (${oracle.kind ?? "unclassified"})`);
      breaches.push(
        breach(
          "testing/oracle",
          "missing-external-oracle",
          scope,
          `${coordinate}: mutation ${mutation.id} requires a ${requirement.qualifyingKind} for capability ${requirement.capability}, and none is registered${supplemental.length > 0 ? ` (only ${supplemental.join(", ")})` : ""}`,
          "An expected result produced by reimplementing our own semantics proves the two implementations agree, not that either is right. Metamorphic, inverse, round-trip and property tests are required supplements and none of them discharges this.",
          `Register a ${QUALIFYING_ORACLE_KINDS.join(" / ")} that declares capability ${requirement.capability}, or block the mutation's release until one exists.`,
        ),
      );
      continue;
    }
    const families = new Set(qualifying.map((oracle) => engineFamilyId(oracle.engine)));
    const required = requirement.distinctEngineFamilies ?? 1;
    if (families.size < required) {
      breaches.push(
        breach(
          "testing/oracle",
          "insufficient-engine-independence",
          scope,
          `${coordinate}: mutation ${mutation.id} requires ${required} independent engine families for ${requirement.capability} but the registered oracles span ${families.size} (${[...families].sort().join(", ")})`,
          "Two wrappers around one kernel are one oracle. Counting them as two turns a shared-kernel bug into a passing comparison.",
          "Qualify a reference on a different engine family, or lower the requirement deliberately with a recorded rationale.",
          "medium",
        ),
      );
    }
  }
  return breaches;
}

/**
 * 🌱️ `verified-native-second-implementation` is EARNED, never asserted. It may discharge a mutation's
 * external-oracle requirement ONLY when the registered entry carries, and this function verifies, every
 * one of: a semio-native artifact (`isSemioNativeArtifact` — never `s.stdio.*` other than `s.stdio.semio`
 * itself, so a format with a real third-party option is refused however good the entry's survey reads);
 * a structured negative search naming what was checked, not free rationale prose; a second-implementation
 * language that differs from the subject's own (a same-language or transliterated reference catches a
 * typo and nothing else — a misread specification still produces two agreeing wrong answers); a
 * specification source distinct from the subject's own code; fixture-backed vectors; and 100% capability
 * coverage of every manifest this same contribution owns for that format — a PARTIAL second
 * implementation must stay `cross-semio-implementation` and discharge nothing, exactly as it did before
 * this kind existed. Any entry missing or failing one of these fires a distinct, actionable breach rather
 * than a generic rejection, so a lazy or false claim fails loudly instead of silently passing.
 */
export function nativeSecondImplementationBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    for (const oracle of contribution.oracles) {
      if (oracle.kind !== "verified-native-second-implementation") continue;
      const scope = `${contribution.owner}#${oracle.id}`;
      const evidence = oracle.nativeSecondImplementation;
      if (evidence === undefined) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} claims verified-native-second-implementation with no nativeSecondImplementation evidence recorded`,
            "This kind discharges an external-oracle requirement exactly as a real third party would; the claim must be earned with recorded evidence, never merely asserted by the kind field alone.",
            "Record format, noThirdPartySurvey, subjectImplementationLanguage, secondImplementationLanguage, specificationSource and fixtureCoverage, or reclassify as cross-semio-implementation.",
          ),
        );
        continue;
      }
      if (!isPlainObject(evidence) || typeof evidence.format !== "string" || evidence.format.trim().length === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records nativeSecondImplementation evidence without the schema-required non-empty format`,
            "The format joins this evidence to the owner's authoritative mutation manifest. Missing or malformed metadata cannot earn a discharge and must remain an explicit contract breach.",
            "Record nativeSecondImplementation.format as the artifact id declared by this contribution's mutationManifests entry.",
          ),
        );
        continue;
      }
      if (!isSemioNativeArtifact(evidence.format)) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-not-native",
            scope,
            `${oracle.id} claims native status for ${JSON.stringify(evidence.format)}, which names a real interchange format`,
            "A format under s.stdio.* other than s.stdio.semio is one a genuine third party can implement, whatever this entry's survey argues; this kind is categorically unavailable there.",
            "Register a real third-party-library / third-party-cli / standards-reference-tool reference instead, or narrow the mutation's requirement.",
          ),
        );
        continue;
      }
      const owningManifests = contribution.mutationManifests.filter((manifest) => manifest.artifact === evidence.format);
      const manifestCapabilities = new Set(owningManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
      if (owningManifests.length === 0 || manifestCapabilities.size === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} claims format ${JSON.stringify(evidence.format)}, but this owner's own contribution manifests none of it`,
            "The claim must tie to a real, owned mutation vocabulary; a format nothing here manifests cannot be verified as fully covered.",
            "Point format at the artifact id this same contribution's own mutationManifests entry declares.",
          ),
        );
        continue;
      }
      const ownCapabilities = new Set(oracle.capabilities);
      const uncovered = [...manifestCapabilities].filter((capability) => !ownCapabilities.has(capability));
      if (uncovered.length > 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-partial-coverage",
            scope,
            `${oracle.id} covers ${ownCapabilities.size} of ${manifestCapabilities.size} capabilities ${JSON.stringify(evidence.format)} manifests; uncovered: ${uncovered.join(", ")}`,
            "A partial second implementation must stay cross-semio-implementation and discharge nothing — the 100% requirement exists precisely so this kind cannot be used to quietly cover part of a vocabulary while looking complete.",
            "Extend the reference's capabilities to the full manifest vocabulary, or reclassify as cross-semio-implementation.",
          ),
        );
        continue;
      }
      const survey = evidence.noThirdPartySurvey;
      const candidatesProblem = survey === undefined || survey.ecosystemsSearched.length === 0 || survey.candidatesConsidered.length === 0 || survey.candidatesConsidered.some((candidate) => candidate.package.trim().length === 0 || candidate.reason.trim().length < 10);
      if (candidatesProblem) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no credible noThirdPartySurvey`,
            "A negative search is not credible unless it names the ecosystems it looked in and, for each candidate it declined, a real structural reason — a rationale-free claim that 'nothing exists' is exactly the unfalsifiable assertion this field exists to rule out.",
            "Record ecosystemsSearched and at least one candidatesConsidered entry with a real package name and a structural reason (10+ characters) it does not qualify.",
          ),
        );
        continue;
      }
      const subjectLanguage = evidence.subjectImplementationLanguage.trim().toLowerCase();
      const secondLanguage = evidence.secondImplementationLanguage.trim().toLowerCase();
      if (subjectLanguage.length === 0 || secondLanguage.length === 0 || subjectLanguage === secondLanguage) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-same-language",
            scope,
            `${oracle.id} declares subjectImplementationLanguage ${JSON.stringify(evidence.subjectImplementationLanguage)} and secondImplementationLanguage ${JSON.stringify(evidence.secondImplementationLanguage)}`,
            "A reference written in the subject's own language, or transliterated from it, catches a typo and nothing else: both halves would still read the same specification, so a misreading of it produces two agreeing wrong answers.",
            "Write the reference in a genuinely different language from the subject, and record both truthfully.",
          ),
        );
        continue;
      }
      if (evidence.specificationSource.trim().length === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no specificationSource`,
            "The whole discharge depends on this reference being read from a written specification, not from the subject's own source — an empty field cannot demonstrate that.",
            "Name the specification document(s) this reference was authored from.",
          ),
        );
        continue;
      }
      if (evidence.fixtureCoverage === undefined || evidence.fixtureCoverage.vectors <= 0 || evidence.fixtureCoverage.capabilitiesCovered.length === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no fixture-backed vectors`,
            "Every mutation this kind discharges must be tested over fixtures — a real-world example, a handcrafted vector, or one generated by a qualifying oracle — never merely a language claim with no committed evidence behind it.",
            "Record how many committed vectors back this reference and which capabilities they exercise.",
          ),
        );
      }
    }
  }
  return breaches;
}

/**
 * 🦑️ Every mutation CAPABILITY the repository declares must be owned by a v2 mutation manifest.
 *
 * `mutationCatalogs` (v1's kind/vector vocabulary) and `mutationManifests` (v2's oracle/dispatch/outcome
 * vocabulary) were two independent registries with no cross-check, and `buildCoverageMatrix` reads only
 * the second. An owner could therefore be 100% v1-complete — every `mutate-<kind>` and `inverse-<kind>`
 * scenario present — while contributing ZERO rows to `test matrix --enforce`, the actual release gate.
 * Because the gated denominators pool across the whole registry, one properly-manifested owner kept
 * them non-empty and the omission never even tripped the empty-denominator guard: a whole capability
 * with un-oracled, un-inventoried mutations was not "missing", it was invisible.
 */
export function capabilityManifestBreaches(registry: OracleRegistry): BreachRecord[] {
  const manifested = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    for (const catalog of contribution.mutationCatalogs) {
      if (catalog.capability === "" || manifested.has(catalog.capability)) continue;
      breaches.push(
        breach(
          "testing/contract",
          "capability-without-manifest",
          contribution.manifestPath,
          `Catalog ${catalog.id} declares capability ${catalog.capability} (${catalog.kinds.length} kind(s)) and no mutation manifest owns it`,
          "The coverage matrix is enumerated from manifests. A capability with no manifest contributes no rows at all, so its mutations are not reported as uncovered — they are absent from the denominator and invisible to every release gate.",
          `Add a mutationManifests entry for ${catalog.capability} declaring each kind's subset, outcomes, production dispatch and oracle requirement.`,
        ),
      );
    }
  }
  return breaches;
}

/** 🧾️ Validators for the v2 registry records that shipped with none — a record nothing checks is a record that can say anything. */
export function registryRecordBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const probes = probeTable(registry);
  const tolerances = toleranceProfileTable(registry);
  for (const probe of registry.probes) {
    if (!MANIFEST_MUTATION_ID_RE.test(probe.id)) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe id ${JSON.stringify(probe.id)} is not kebab-case`, "Probe ids are referenced by every pipeline stage; a free-form id cannot be joined reliably.", "Rename the probe."));
    if (probe.capabilities.length === 0) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} declares no capabilities`, "A probe that claims no capability can never be selected deliberately.", "Declare what it measures."));
    if (probe.outputSchema !== "semio.repository-test.probe-report/v2") breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} declares outputSchema ${JSON.stringify(probe.outputSchema)}`, "The orchestrator only evaluates the typed report shape it knows.", 'Emit "semio.repository-test.probe-report/v2".'));
    if (probe.qualification !== undefined && probe.qualification.evidence.trim().length === 0) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} claims qualification ${probe.qualification.status} with no evidence`, "A qualification status is a claim about a measurement somebody made; without the evidence it is an assertion.", "Record what was measured, and where."));
  }
  for (const profile of registry.toleranceProfiles) {
    if (!MANIFEST_MUTATION_ID_RE.test(profile.id)) breaches.push(breach("testing/contract", "tolerance-record-invalid", profile.id, `Tolerance profile id ${JSON.stringify(profile.id)} is not kebab-case`, "Profile ids are joined from fixtures and pipelines.", "Rename the profile."));
    for (const [key, value] of Object.entries(profile)) {
      if (typeof value === "number" && !Number.isFinite(value)) breaches.push(breach("testing/contract", "tolerance-record-invalid", profile.id, `Tolerance profile ${profile.id} field ${key} is not finite`, "A non-finite tolerance accepts everything.", "Give it a finite value."));
    }
  }
  for (const pipeline of registry.comparisonPipelines) {
    if (pipeline.stages.length === 0) breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} declares no stages`, "A pipeline with no stage compares nothing and reports equal.", "Declare at least one stage."));
    if (pipeline.toleranceProfile !== undefined && !tolerances.has(pipeline.toleranceProfile)) {
      breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} names unknown tolerance profile ${pipeline.toleranceProfile}`, "An unresolved profile means every dimensional assertion is resolved against nothing.", `Use one of ${[...tolerances.keys()].sort().join(", ")}.`));
    }
    if (pipeline.stages.every((stage) => stage.optional === true)) {
      breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Every stage of pipeline ${pipeline.id} is optional`, "A pipeline whose every stage is excusable gates nothing while reporting a verdict.", "Make at least one stage gating."));
    }
    for (const [index, stage] of pipeline.stages.entries()) {
      const probe = probes.get(stage.probe);
      if (probe === undefined) {
        breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} stage ${index} names unregistered probe ${stage.probe}`, "An unregistered probe has no licence, pin, engine family or qualification on record, and the stage cannot run.", "Register the probe, or remove the stage."));
        continue;
      }
      if (stage.optional === true && isQualifiedProbe(probe)) {
        breaches.push(breach("testing/contract", "optional-stage-overclaimed", pipeline.id, `Pipeline ${pipeline.id} stage ${index} marks QUALIFIED probe ${stage.probe} optional`, "`optional` exists for a probe whose qualification spike has not passed. On a qualified probe it would excuse that probe's own status: failed, which is the one thing this protocol must never do.", "Remove `optional`, or downgrade the probe's qualification honestly."));
      }
      if (stage.inputs.length === 0) breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} stage ${index} reads no inputs`, "A stage with no input measures nothing.", "Name the artifact roles it reads."));
    }
  }
  return breaches;
}

/**
 * 🎭️ A declared export dialect whose serializer does not actually emit that format.
 *
 * This is the root cause beneath a large share of "no qualifying oracle": a carrier oracle works by
 * having a third-party reader of a STANDARD FORMAT verify what a mutation produced — `brepjs` reads the
 * STEP a Boolean should have written. That is impossible when the exporter writes the artifact's own
 * internal DSL text under the standard format's extension, because no third-party reader of PNG, OBJ,
 * STL or Markdown can parse it. The artifact declares an export capability it does not have.
 *
 * 45% of the repository's serializers are in this state (85 of 187), and Markdown and DXF have no real
 * exporter at all. The finding is severity-graded rather than uniform: for a BINARY format, DSL text is
 * unambiguously not that format; for a text format it is still wrong but a reader may at least not
 * crash, so it is reported one level lower rather than asserted with false confidence.
 */
export const BINARY_CARRIER_FORMATS: readonly string[] = ["png", "jpg", "jpeg", "gif", "tiff", "bmp", "stl", "ply", "las", "gltf", "glb", "zip", "pdf", "dwg", "docx", "xlsx", "pptx", "mp3", "mp4", "wav"];

/** 🧪️ Strip `#[cfg(test)]` modules before judging a serializer. Their round-trip PROOFS legitimately call
 *  the very functions a stub detector watches for, so scanning them flagged two genuinely real carriers —
 *  `semio@v1/cad → step` and `semio@v1/drawing → svg` — as stubs. A gate must be wrong in neither
 *  direction: a false stub hides a usable carrier exactly as a missed stub invents one. */
function withoutTestModules(text: string): string {
  let out = "";
  let at = 0;
  for (;;) {
    const found = text.indexOf("#[cfg(test)]", at);
    if (found === -1) return out + text.slice(at);
    out += text.slice(at, found);
    const open = text.indexOf("{", found);
    if (open === -1) return out;
    let depth = 0;
    let i = open;
    for (; i < text.length; i += 1) {
      if (text[i] === "{") depth += 1;
      else if (text[i] === "}") {
        depth -= 1;
        if (depth === 0) break;
      }
    }
    at = i + 1;
  }
}

/** 💬️ Drop whole-line `//` and `//!`/`///` comments before judging a serializer. A leaf that documents the
 *  stub it REPLACED ("this file used to hand back `print_dsl(...)`") is the opposite of a stub, and matching
 *  the phrase in prose flagged real zip, stl and dwg writers as internal-DSL emitters. */
function withoutLineComments(text: string): string {
  return text.replace(/^[ \t]*\/\/.*$/gm, "");
}

/** 🪞️ Whether a serializer prints ITS OWN INPUT as DSL text. A hop that builds the target format's
 *  snapshot and hands that snapshot's own wire text onward (`print_dsl(&md)` for an `MdSnapshot`) is the
 *  io-mechanism's designed shape — the format's own codec reaches the carrier on the next hop — and
 *  flagging the call by name alone reported real markdown writers as internal-DSL emitters. */
function printsItsInputAsDsl(text: string): boolean {
  for (const match of text.matchAll(/fn\s+serialize(?:_bytes|_text)?\s*(?:<[^>]*>)?\s*\(([^)]*)\)/g)) {
    const name = ((match[1] ?? "").split(",")[0] ?? "").split(":")[0]?.trim().replace(/^(mut|&)\s*/, "") ?? "";
    if (!name || name === "self" || name === "&self") continue;
    const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    if (new RegExp(`print_dsl\\s*\\(\\s*&?${escaped}\\b|\\b${escaped}\\s*\\.\\s*print_dsl\\s*\\(`).test(text)) return true;
  }
  return false;
}

/** 📥️ Whether a deserializer parses this artifact's own DSL out of the file. The one legitimate shape
 *  is the shared zip document archive, whose declared content IS the `snapshot.<ext>.semio` member; any
 *  other DSL parse means a real file of the declared format cannot be imported. */
function parsesItsInputAsDsl(text: string): boolean {
  return /parse_dsl\s*\(/.test(text) && !/\b(document_archive_member|decode_document_archive)\b/.test(text);
}

/** 🕳️ A serializer whose body NEVER READS ITS INPUT cannot represent a mutation, whatever else it does.
 *  This one predicate subsumes several shapes that were each hiding separately: an `Err("not
 *  implemented")` stub, a bridge returning `XmlDocument::default()` regardless of argument, and a DWG
 *  leaf that ignores its parameter and returns a hardcoded empty document. Detecting the CAUSE rather
 *  than each spelling is what keeps the gate from needing a new rule per author. */
function ignoresItsInput(text: string, entry: "serialize" | "deserialize" = "serialize"): boolean {
  for (const match of text.matchAll(new RegExp(`fn\\s+${entry}(?:_bytes|_text)?\\s*(?:<[^>]*>)?\\s*\\(([^)]*)\\)`, "g"))) {
    const parameter = (match[1] ?? "").split(",")[0]?.trim() ?? "";
    const name = parameter.split(":")[0]?.trim().replace(/^(mut|&)\s*/, "") ?? "";
    if (!name || name === "self" || name === "&self") continue;
    const open = text.indexOf("{", (match.index ?? 0) + match[0].length);
    if (open === -1) continue;
    let depth = 0;
    let i = open;
    for (; i < text.length; i += 1) {
      if (text[i] === "{") depth += 1;
      else if (text[i] === "}") {
        depth -= 1;
        if (depth === 0) break;
      }
    }
    const body = text.slice(open + 1, i);
    const reads = new RegExp(`\\b${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`).test(body.replace(new RegExp(`let\\s+_\\s*=\\s*${name}\\s*;`, "g"), ""));
    if (!reads) return true;
  }
  return false;
}

/**
 * 🫥️ A QUALIFYING ORACLE MUST NOT BE THIS REPOSITORY'S OWN SECOND IMPLEMENTATION WEARING A LIBRARY'S NAME.
 *
 * The shape this catches, and it was found in five owners covering 156 mutations: an entry registered
 * `third-party-library` on the strength of a real crate — `json`, `png`, `image` — where the crate only
 * parses or encodes, and WHAT THE MUTATION SHOULD PRODUCE is computed by the owner's own
 * `🦀️oracle.rs`. `gltf`'s implemented seven kinds and refused the other 113 while the
 * manifest named it against all 120; `png`'s says in its own doc comment that it "deliberately mirrors"
 * the production `diff`. Both halves then read the same specification, so a misreading of it yields two
 * agreeing wrong answers — which is the one failure a differential test exists to prevent.
 *
 * A crate that decodes the artifact discharges "the result is a well-formed file of this format". It does
 * not discharge "the mutation computed the right answer". Only the second is what a mutation oracle is for.
 *
 * 🎯 ENTRY-GRANULAR, NOT FILE-GRANULAR. This owner's shared `🦀️oracle.rs` is exactly one Rust
 * compilation unit; a `verified-native-second-implementation`/`third-party-library`/… entry can only be
 * THE CODE this file's predicting dispatch belongs to when that entry's own `ecosystem` is `"rust"` —
 * anything else (`python`'s `ifcopenshell`, `javascript`'s `yauzl`, …) executes in an entirely different
 * runtime the file never touches, so the file's content is not evidence about it, however incriminating
 * that same file is about a Rust sibling entry sitting in the very same registry contribution. Filtering
 * to `implicable` BEFORE reading the file — rather than reading it once and then deciding which of
 * `qualifying`'s ids to name in the message — is what keeps a genuine third-party reader from being swept
 * in with a mislabelled Rust reimplementation merely for sharing a directory: the file is not even opened
 * unless some entry could actually be the code it describes.
 */
export function reimplementationOracleBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    const qualifying = contribution.oracles.filter((oracle) => isQualifyingOracleKind(oracle.kind));
    const implicable = qualifying.filter((oracle) => oracle.ecosystem === "rust");
    if (implicable.length === 0) continue;
    const own = join(repoRoot, dirname(contribution.manifestPath), "🦀️.rs");
    if (!existsSync(own)) continue;
    let text: string;
    try {
      text = readFileSync(own, "utf8");
    } catch {
      continue;
    }
    const predicts = /fn\s+(apply_kind|apply)\s*\(/.test(text) && /has no oracle implementation/.test(text);
    const admits = /independent implementation|second implementation|deliberately mirrors|reimplemented from scratch/i.test(text);
    if (!predicts && !admits) continue;
    const judgedByProbes =
      contribution.comparisonPipelines.length > 0 &&
      contribution.probes.length > 0 &&
      contribution.probes.some((probe) => probe.qualification?.status === "qualified" || (probe as { qualified?: boolean }).qualified === true);
    if (judgedByProbes) continue;
    breaches.push(
      breach(
        "testing/oracle",
        "reimplementation-registered-as-third-party",
        `${contribution.owner}/🦀️oracle.rs`,
        `${implicable.map((oracle) => oracle.id).join(", ")} is registered as a qualifying third-party oracle, but this owner predicts mutation output in its own Rust`,
        `The crate parses or encodes; the expected RESULT of each mutation is computed here. Both halves of the comparison then read the same specification, so a misreading of it produces two agreeing wrong answers — the one failure mode a differential test exists to prevent.`,
        `Reclassify the entry as cross-semio-implementation (a required supplement, never a substitute) and register a domain-aware third-party reader for the mutation semantics, or narrow the crate's capability to file well-formedness only.`,
        "high",
      ),
    );
  }
  return breaches;
}

/** 🏭️ A fixture whose `after` half was written by OUR code is not evidence, whatever the registration says.
 *
 * A generator may legitimately link a third-party crate to lay out a seed document and to serialise.
 * What it may never do is obtain the MUTATED bytes, or the expected PROJECTION of them, from this
 * repository's own mutation engine: then both halves of the comparison descend from one implementation
 * and a shared misreading of the specification yields two agreeing wrong answers. The third-party
 * library is reduced to a codec for our own answer.
 *
 * Keyed on the disqualifying SYMBOLS rather than on any `semio-*` dependency, because a generator that
 * merely borrows a JSON helper from the harness is not writing fixtures with our semantics, and a gate
 * that cannot tell those apart is one people learn to ignore.
 *
 * @see ../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pdf-fixtures-are-not-admissible-evidence.md
 */
export function fixtureWriterProvenanceBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const disqualifyingAll = /\b(oracle_apply_mutation|oracle_apply|apply_mutation|project_conformance|oracle_inverse_spec|oracle_round_trip)\b/g;
  for (const contribution of registry.contributions) {
    if (contribution.testEvidence.length === 0) continue;
    const generator = join(repoRoot, contribution.owner, "🏭️generator");
    if (!existsSync(generator)) continue;
    const offenders = new Set<string>();
    walkDirectories(generator, (abs, rel) => {
      if (rel.includes("target")) return "skip";
      let names: string[];
      try {
        names = readdirSync(abs);
      } catch {
        return "enter";
      }
      for (const name of names) {
        if (!name.endsWith(".rs")) continue;
        let text: string;
        try {
          text = readFileSync(join(abs, name), "utf8");
        } catch {
          continue;
        }
        for (const line of text.split("\n")) {
          if (!line.startsWith("use ") || !/\bsemio_/.test(line)) continue;
          for (const found of line.matchAll(disqualifyingAll)) offenders.add(found[0]);
        }
      }
      return "enter";
    });
    if (offenders.size === 0) continue;
    breaches.push(
      breach(
        "testing/fixture",
        "fixture-after-state-written-by-our-own-code",
        `${contribution.owner}/🏭️generator`,
        `${contribution.testEvidence.length} committed fixture(s) obtain their mutated state from this repository's own engine (${[...offenders].sort().join(", ")})`,
        `The generator imports the mutation application and/or the expected projection from a semio crate, so the \`after\` half of every pair here is our own computation wearing a third-party crate's name. A reader registered over these fixtures would be judging a state we predicted, which is the exact substitution Protocol v2 exists to forbid.`,
        `Apply each mutation through the third-party library's OWN public API in the generator and drop the semio import, so the after bytes are written by something other than us; only then register a reader oracle over them. Writer and reader being the same third-party library is fine and is the established precedent.`,
        "high",
      ),
    );
  }
  return breaches;
}

/**
 * 🧩️ The import half of the same lie: a declared import dialect whose deserializer never reads the
 * file it is handed, or parses it as this artifact's own DSL instead of the declared format. The
 * common spelling is `let _ = bytes; Ok(Snapshot::default())` — every import "succeeds" and yields an
 * empty document, so a user importing a real STL or DXF silently gets nothing. A format the artifact
 * cannot represent must not be declared at all; one it can must be read.
 */
export function stubDeserializerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (!rel.includes("🧩️deserializers")) return "enter";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
      let text: string;
      try {
        text = withoutLineComments(withoutTestModules(readFileSync(join(abs, entry.name), "utf8")));
      } catch {
        continue;
      }
      if (!/fn\s+deserialize(_bytes|_text)?\s*[(<]/.test(text)) continue;
      const format = rel.match(/🗿️artifacts\/[^/]*?([a-z0-9]+)\/🔖️/)?.[1] ?? "";
      const formatType = format.replace(/^./, (c) => c.toUpperCase());
      const parsesDsl = parsesItsInputAsDsl(text.split("\n").filter((line) => !(formatType && new RegExp(`<\\s*${formatType}Snapshot\\s+as\\s+pack::snapshot::ArtifactDsl\\s*>::parse_dsl`).test(line))).join("\n"));
      const coerces = (/to_value\s*\(\s*&?[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text) || /\b[a-z_][a-z0-9_]*\s*\.\s*to_value\s*\(\s*\)/.test(text)) && /from_value\s*[:(<]/.test(text);
      const inert = ignoresItsInput(text, "deserialize");
      if (!parsesDsl && !coerces && !inert) continue;
      if (format === "txt" || format === "json" || format === "binary" || format === "") continue;
      breaches.push(
        breach(
          "testing/contract",
          "stub-deserializer",
          `${rel}/${entry.name}`,
          inert ? `The ${format} deserializer never reads its input` : coerces ? `The ${format} deserializer coerces the ${format} snapshot through serde into this artifact` : `The ${format} deserializer parses the artifact's internal DSL, not ${format}`,
          inert
            ? `Its deserialize function never mentions its own input parameter, so every ${format} file imports as the same document — usually an empty default reported as success.`
            : coerces
              ? `It serializes the decoded ${format} snapshot to a value tree and reads that tree as this artifact; the two shapes share no fields, so every real ${format} file imports as an empty or failing document.`
              : `It hands the ${format} bytes to this artifact's own DSL parser, so a real ${format} file from any other producer cannot be imported.`,
          `Read ${format} into this artifact's model, or remove ${format} from this subset's import dialects so the capability is not claimed.`,
          "high",
        ),
      );
    }
    return "enter";
  });
  return breaches;
}

export function stubSerializerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (!rel.includes("🧵️serializers")) return "enter";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
      let text: string;
      try {
        text = withoutLineComments(withoutTestModules(readFileSync(join(abs, entry.name), "utf8")));
      } catch {
        continue;
      }
      if (!/fn\s+serialize(_bytes|_text)?\s*[(<]/.test(text)) continue;
      const printsDsl = printsItsInputAsDsl(text);
      const transmutes = /encode_pack\s*\(\s*[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text) && /decode_pack\s*\(\s*&?[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text);
      const coerces = (/to_value\s*\(\s*&?[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text) || /\b[a-z_][a-z0-9_]*\s*\.\s*to_value\s*\(\s*\)/.test(text)) && /from_value\s*[:(<]/.test(text);
      const inert = ignoresItsInput(text);
      if (!printsDsl && !transmutes && !coerces && !inert) continue;
      const format = rel.match(/🗿️artifacts\/[^/]*?([a-z0-9]+)\/🔖️/)?.[1] ?? "";
      if (format === "txt" || format === "json" || format === "") continue;
      const binary = BINARY_CARRIER_FORMATS.includes(format);
      breaches.push(
        breach(
          "testing/contract",
          "stub-serializer",
          `${rel}/${entry.name}`,
          printsDsl
            ? `The ${format} serializer emits the artifact's internal DSL text, not ${format}`
            : transmutes
              ? `The ${format} serializer reinterprets this artifact's own pack bytes as a ${format} snapshot`
              : coerces
                ? `The ${format} serializer coerces this artifact through serde into an empty ${format} document`
                : `The ${format} serializer never reads its input`,
          inert && !printsDsl && !transmutes && !coerces
            ? `Its serialize function never mentions its own input parameter, so its output cannot depend on the snapshot and no mutation can ever be observable in it. This covers an explicit not-implemented stub, a bridge returning Default::default() whatever it is given, and a leaf that silences the unused-parameter warning with \`let _ = x;\` and then ignores it.`
            : coerces && !printsDsl && !transmutes
            ? `It serializes the source snapshot to a serde_json Value and deserializes that Value into the ${format} snapshot type. Because the target's fields carry #[serde(default)] and unknown keys are ignored, this SUCCEEDS and produces an empty ${format} document for every input — so no mutation is ever observable in the bytes, and the export reports Ok while carrying nothing.`
            : transmutes && !printsDsl
            ? `It encodes the source snapshot with encode_pack and decodes those same bytes as the ${format} snapshot type. That is not an export, it is a reinterpretation of one artifact's envelope as another's — it either fails on the envelope id or yields a structurally valid ${format} document whose content is unrelated to the source. No third-party ${format} reader can verify a mutation through it.`
            : binary
            ? `${format} is a binary format and DSL text is not it. The artifact declares an export dialect it does not implement, so no third-party reader of ${format} can verify what a mutation produced — which is exactly how a carrier oracle works.`
            : `The artifact declares a ${format} export dialect and writes its own DSL text instead. A third-party ${format} reader cannot verify what a mutation produced.`,
          `Implement the ${format} serializer, or remove ${format} from this subset's exportDialects so the capability is not claimed.`,
          transmutes || coerces || inert || binary ? "high" : "medium",
        ),
      );
    }
    return "enter";
  });
  return breaches;
}

/** 🚫️ A no-oracle decision may never stand in for a mutation's external-oracle requirement. */
export function noOracleMisuseBreaches(registry: OracleRegistry): BreachRecord[] {
  const mutationCapabilities = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.flatMap((mutation) => [mutation.capability, ...mutation.oracleRequirements.map((requirement) => requirement.capability)])));
  const breaches: BreachRecord[] = [];
  for (const decision of registry.noOracleDecisions) {
    if (!isPlainObject(decision) || !Array.isArray(decision.capabilities) || !decision.capabilities.every((capability) => typeof capability === "string")) {
      const id = isPlainObject(decision) && typeof decision.id === "string" ? decision.id : "unknown-no-oracle-decision";
      breaches.push(breach("testing/oracle", "no-oracle-decision-invalid", id, `No-oracle decision ${id} has no schema-required capabilities array`, "A malformed decision cannot be matched against mutation requirements, and crashing here hides every subsequent contract finding.", "Record capabilities as a non-empty string array and retain the decision's rationale and substitutes."));
      continue;
    }
    const covered = decision.capabilities.filter((capability) => mutationCapabilities.has(capability));
    if (covered.length > 0 && !isJustifiedMutationDecision(decision)) {
      breaches.push(breach("testing/oracle", "no-oracle-covers-mutation", decision.id, `No-oracle decision ${decision.id} claims mutation capability/capabilities ${covered.join(", ")}`, "A runtime mutation's oracle requirement is discharged only by a qualifying third-party reference; recording a decision instead would make the gap invisible rather than blocking.", "Register a qualifying oracle and remove the mutation capability from the decision; where no third party can express the mutation, record coversMutations: true with a referenceSurvey naming every candidate considered."));
    }
  }
  return breaches;
}

/** 🧫️ Fixture provenance, digest, licence and reproducibility — every one a hard contract condition. */
export function fixtureProvenanceBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const oracles = new Map(registry.oracles.map((oracle) => [oracle.id, oracle]));
  const tolerances = toleranceProfileTable(registry);
  const profiles = profileTable(registry);
  for (const contribution of registry.contributions) {
    for (const [index, fixture] of contribution.testEvidence.entries()) {
      const scope = contribution.manifestPath;
      for (const problem of testEvidenceProblems(fixture, repoRoot)) {
        breaches.push(breach("testing/fixture", "fixture-manifest-invalid", scope, `testEvidence[${index}] ${problem}`, "A fixture is evidence; evidence with incomplete provenance cannot be audited or reproduced.", "Complete the fixture manifest record."));
      }
      for (const verification of verifyFixture(repoRoot, fixture)) {
        if (verification.missing) breaches.push(breach("testing/fixture", "fixture-file-missing", `${scope}#${fixture.id}/${verification.role}`, `Fixture file ${verification.path} does not exist`, "A manifest that names a file nobody can read is a coverage claim with no artifact behind it.", "Restore the file or remove the role."));
        else if (!verification.ok) breaches.push(breach("testing/fixture", "fixture-digest-mismatch", `${scope}#${fixture.id}/${verification.role}`, `Fixture file ${verification.path} hashes ${verification.actual}, manifest says ${verification.expected}`, "Source fixtures are immutable after review; a changed fixture is a NEW identity, never an edit in place.", "Restore the reviewed bytes, or mint a new fixture id and digest."));
      }
      if (fixture.class === "third-party-generated" && fixture.generator !== undefined) {
        const oracle = oracles.get(fixture.generator.oracle);
        if (oracle === undefined) breaches.push(breach("testing/fixture", "fixture-generator-unregistered", `${scope}#${fixture.id}`, `Fixture generator oracle ${fixture.generator.oracle} is not registered`, "A generated fixture's authority is the tool that generated it; an unregistered tool has no licence, pin or engine family on record.", "Register the generating oracle."));
        else if (!isQualifyingOracleKind(oracle.kind)) breaches.push(breach("testing/fixture", "fixture-generated-by-non-qualifying-oracle", `${scope}#${fixture.id}`, `Fixture ${fixture.id} is generated by ${oracle.id}, whose kind is ${oracle.kind ?? "unclassified"}`, "An expected result produced by our own second implementation is not third-party evidence.", `Regenerate it with a ${QUALIFYING_ORACLE_KINDS.join(" / ")}.`));
        if (!fixture.reproducible) breaches.push(breach("testing/fixture", "fixture-not-reproducible", `${scope}#${fixture.id}`, `Generated fixture ${fixture.id} is marked non-reproducible`, "A generated expectation nobody can regenerate is indistinguishable from a hand-edited one.", "Make generation deterministic, or document the external canonicalization that makes it reproducible.", "medium"));
      }
      if (fixture.toleranceProfile !== undefined && !tolerances.has(fixture.toleranceProfile)) {
        breaches.push(breach("testing/fixture", "fixture-tolerance-profile-unknown", `${scope}#${fixture.id}`, `Fixture ${fixture.id} names tolerance profile ${fixture.toleranceProfile}, which no profile defines`, "An unresolved profile means every dimensional assertion for this fixture is resolved against nothing, while the manifest reads as though a policy applies.", `Use one of ${[...tolerances.keys()].sort().join(", ")}, or contribute the profile.`));
      }
      if (fixture.comparisonProfile.length > 0 && !profiles.has(fixture.comparisonProfile)) {
        breaches.push(breach("testing/fixture", "fixture-comparison-profile-unknown", `${scope}#${fixture.id}`, `Fixture ${fixture.id} names comparison profile ${fixture.comparisonProfile}, which no profile defines`, "The comparison profile decides how this fixture is compared at all; an unresolved one silently falls back to a default nobody chose.", "Contribute the profile, or name one that exists."));
      }
      if (fixture.toleranceOverride !== undefined) {
        const profile = toleranceProfileTable(registry).get(fixture.toleranceProfile ?? "");
        const cap = profile?.maxOverrideFactor ?? 1;
        if (fixture.toleranceOverride.factor > cap) {
          breaches.push(breach("testing/fixture", "tolerance-override-exceeds-cap", `${scope}#${fixture.id}`, `Fixture ${fixture.id} overrides its tolerance by ${fixture.toleranceOverride.factor}×, above the profile cap of ${cap}×`, "An uncapped override turns a failing comparison into a passing one and nothing in the report would say so.", "Lower the override, or change the fixture's tolerance profile deliberately."));
        }
      }
    }
  }
  return breaches;
}

/**
 * 🛠️ Law 2, made mechanical for the v2 manifest. `fixtureProvenanceBreaches` above already audits
 * every REGISTERED fixture's own honesty — present on disk, correct digest, licensed, generated by a
 * qualifying oracle when generated at all — and `mutationVectorRegistryBreaches` already audits the
 * v1 physical `🦠️mutation`/`📸️snapshot`/`🔺️diff`/`🎯️outcome` bundle's own shape. What NEITHER can see
 * is a v2-declared mutation with NO evidence registered against it in EITHER form: the coordinate
 * `buildCoverageMatrix` reports as `status: "missing"` in a RUN's coverage report, but which this
 * static contract phase — computable with no execution at all — never previously failed on.
 * `mutation-kind-uncovered`/`mutation-inverse-uncovered` police the v1 Gherkin catalog's Examples
 * table, not whether either fixture form actually exists; none of the five rules named in this
 * function's own brief reads a v2 `mutationManifests` entry's fixture coverage at all.
 *
 * The minimum honest evidence this accepts is deliberately EITHER of the two forms already live in
 * this repository, not a mandate to adopt one: a `TestEvidence` configuration whose
 * `target` names this mutation's own artifact/standard/subset and whose `mutation` field names this
 * mutation's own id — exactly what the exemplar (`📷️png@1.2/✳️any`'s `change-background`) carries —
 * OR a v1 physical vector registered in a `mutationCatalogs[].vectors` entry sharing this mutation's
 * OWN `capability` (the same correlation `mutationInventoryBreaches` already uses to match a v1
 * catalog's claimed kinds against a v2 manifest) whose `mutationId` names this mutation's own id, as
 * `🏛️architect/🏛️program` carries for all 266 of its mutations. A survey of the live registry BEFORE
 * settling on this design found 1,650 "violations" against the v2-only form — investigating the
 * largest, `s.architect.program`, showed a real, checked-in, handcrafted before/after JSON bundle for
 * every one of its 266 mutations, registered only in the older v1 form: counting those as untested
 * would have been dishonest. Whichever form supplies the evidence, its OWN existing rule
 * (`fixtureProvenanceBreaches` / `mutationVectorRegistryBreaches`) is what proves the claim is not a
 * lie — this rule only proves a claim of SOME form exists at all. One fixture per mutation id, not one
 * per declared outcome class: an unfixtured OUTCOME of an otherwise-fixtured mutation is a
 * coverage-report gap (`measureCoverage`'s own `status: "missing"` rows), not a law-2 placement gap,
 * and duplicating that measurement here would contradict the run-dependent coverage dimension instead
 * of scoping cleanly around it.
 *
 * A kind a catalog sharing the mutation's capability lists in `deferredKinds` is not reported here:
 * that declaration IS the schema-first record that no wire vector can exist for it yet, and
 * `mutationCoverageBreaches` already keeps the debt visible as `mutation-kinds-deferred` for every
 * feature that claims the catalog. Reporting it twice would count one declared gap as two findings.
 */
export function mutationFixtureBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const fixturedMutations = new Set<string>();
  for (const contribution of registry.contributions) {
    for (const fixture of contribution.testEvidence) {
      if (fixture?.target?.artifact === undefined || fixture.mutation === undefined) continue;
      fixturedMutations.add(`${surfaceCoordinate(fixture.target)}::${fixture.mutation}`);
    }
  }
  const accountedMutationsByCapability = new Map<string, Set<string>>();
  for (const contribution of registry.contributions) {
    for (const catalog of contribution.mutationCatalogs) {
      if (catalog.capability === "") continue;
      const ids = accountedMutationsByCapability.get(catalog.capability) ?? new Set<string>();
      for (const vector of catalog.vectors) ids.add(vector.mutationId);
      for (const kind of catalog.deferredKinds ?? []) ids.add(kind);
      accountedMutationsByCapability.set(catalog.capability, ids);
    }
  }
  for (const contribution of registry.contributions) {
    for (const manifest of contribution.mutationManifests) {
      if (mutationManifestProblems(manifest, contribution.owner).length > 0) continue;
      for (const mutation of manifest.mutations) {
        const subset = owningSubsetOf(manifest, mutation);
        if (fixturedMutations.has(`${surfaceCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset, surface: manifest.surface })}::${mutation.id}`)) continue;
        if (accountedMutationsByCapability.get(mutation.capability)?.has(mutation.id) === true) continue;
        breaches.push(
          breach(
            "testing/fixture",
            "mutation-without-fixture",
            contribution.manifestPath,
            `Mutation ${mutation.id} of ${subsetCoordinate(manifestTarget(manifest, mutation))} is declared with no fixture-backed vector`,
            "A mutation manifest declares what dispatch CLAIMS it can do; a fixture is the one thing that tests the claim against a real-world example, a handcrafted vector, or a qualifying-oracle-generated before/after pair, rather than the implementation grading its own homework.",
            `Add a testEvidence entry with target ${JSON.stringify({ artifact: manifest.artifact, standard: manifest.standard, subset })} and mutation ${JSON.stringify(mutation.id)}, or register a physical vector for it in a mutationCatalogs[] entry sharing capability ${JSON.stringify(mutation.capability)} — either way, backed by real-world/handcrafted/oracle-generated evidence.`,
          ),
        );
      }
    }
  }
  return breaches;
}

/** 🔒️ Oracle and probe packages must be unreachable from production, and offline during execution. */
export function isolationBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const oracle of registry.oracles) {
    if (oracle.productionReachable === true && oracle.productionDebt === undefined) {
      breaches.push(breach("testing/dependency", "oracle-production-reachable", oracle.id, `Oracle ${oracle.id} (${oracle.package}) is production-reachable and records no debt`, "A reference the production code can also reach is not an independent reference — the bug would be in both halves.", "Remove the production edge, or record it as shrink-only productionDebt with an owner and a plan."));
    }
    if (oracle.networkDuringExecution === true) {
      breaches.push(breach("testing/dependency", "oracle-needs-network", oracle.id, `Oracle ${oracle.id} requires network access during execution`, "Tests execute offline once dependencies and fixture blobs are provisioned; a network call makes a run non-reproducible and CI-flaky.", "Provision the data ahead of the run and read it from the fixture store."));
    }
  }
  for (const probe of registry.probes) {
    if (probe.productionReachable === true && probe.productionDebt === undefined) {
      breaches.push(breach("testing/dependency", "probe-production-reachable", probe.id, `Probe ${probe.id} (${probe.package}) is production-reachable and records no debt`, "A measurement tool linked into production stops being an independent measurement.", "Make the probe test-only, or record the reachability as shrink-only productionDebt with an owner and a plan."));
    }
    if (probe.productionDebt !== undefined && (probe.productionDebt.reachableFrom.length === 0 || probe.productionDebt.owner.length === 0 || probe.productionDebt.plan.length === 0)) {
      breaches.push(breach("testing/dependency", "probe-debt-incomplete", probe.id, `Probe ${probe.id} records productionDebt without a path, an owner or a retirement plan`, "Debt that names nobody and no plan is not a record, it is an excuse.", "Name the reachable path, the owning module and how the debt shrinks."));
    }
    if (probe.networkDuringExecution === true) {
      breaches.push(breach("testing/dependency", "probe-needs-network", probe.id, `Probe ${probe.id} requires network access during execution`, "A probe that phones home cannot produce a reproducible measurement.", "Vendor or pre-provision whatever it fetches."));
    }
    if (probe.deterministic === false && probe.seedRequired !== true) {
      breaches.push(breach("testing/contract", "probe-nondeterministic-unseeded", probe.id, `Probe ${probe.id} is non-deterministic and declares no seed requirement`, "Approximate measurements — sampled Hausdorff distance above all — depend on sampling. Without a recorded seed the same comparison can pass and fail on identical inputs.", "Declare seedRequired and take the seed from the stage."));
    }
  }
  return breaches;
}

/** 🔄️ A subject that replays committed vectors proves only that the vectors were copied correctly. */
export function vectorReplayBreaches(results: readonly TestResult[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const result of results) {
    if (result.role !== "subject" || result.status === "errored") continue;
    if (result.productionDispatch?.invoked !== true) {
      breaches.push(breach("testing/contract", "subject-without-production-dispatch", result.testId, `Subject result ${result.testId} carries no production-dispatch proof`, "A subject adapter that replays a committed output vector measures the vector, not the code. The dispatch record is the only thing that distinguishes them.", "Invoke the production mutation bridge from the adapter and report the operation and bridge version."));
    }
  }
  return breaches;
}

/** ⚙️ An oracle sharing the subject's engine family is an interoperability check, not an independence one. */
export function engineIndependenceBreaches(registry: OracleRegistry, subjectEngines: ReadonlyMap<string, EngineFamily>): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const manifest of registry.mutationManifests) {
    const subject = subjectEngines.get(manifest.artifact);
    if (subject === undefined) continue;
    for (const mutation of manifest.mutations) {
      for (const requirement of mutation.oracleRequirements) {
        const qualifying = registry.oracles.filter((oracle) => oracle.capabilities.includes(requirement.capability) && isQualifyingOracleKind(oracle.kind));
        const independent = qualifying.filter((oracle) => engineFamilyId(oracle.engine) !== subject.family);
        if (qualifying.length > 0 && independent.length === 0) {
          breaches.push(
            breach(
              "testing/oracle",
              "oracle-shares-subject-engine",
              `${manifest.artifact}@${manifest.standard}/${owningSubsetOf(manifest, mutation)}`,
              `Every qualifying oracle for ${requirement.capability} sits on engine family ${subject.family}, which is also the subject's`,
              "A shared kernel makes the comparison circular: the reference and the code under test would agree on the kernel's own defects.",
              "Qualify a second exact-kernel family, and keep the independent mesh-side probe in the pipeline meanwhile.",
              "medium",
            ),
          );
        }
      }
    }
  }
  return breaches;
}
//#endregion 🧾️ContractV2
