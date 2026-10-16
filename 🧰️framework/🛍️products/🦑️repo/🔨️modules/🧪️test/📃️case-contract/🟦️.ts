/** 📃️ Case contract
 *
 * The contract of one test case and its mutation-vector coverage.
 */

//#region 🔌️Adapters
import { parseFeature, type ParsedFeature } from "@semio-tech/framework/test/gherkin";
import { type Implementation } from "@semio-tech/framework/test/adapter";
import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { type BreachRecord } from "../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { mutationCatalogSourceOwner, mutationOwnerRelativePath } from "../../📚️library/🔍️discovery/🟦️.ts";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { testAdapterFilenames, testFeatureFilename, testTaxonomy, type TestTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { type DiscoveredCase } from "../🧭️case-discovery/🟦️.ts";
import { ADAPTER_ENTRY_POINTS, oracleImplementation } from "../🗃️oracle-registry/🟦️.ts";
import { mutationCatalogProblems } from "../🧮️mutation-catalog/🟦️.ts";
import { subsetCoordinatesOfOwner } from "../🪆️subset-policy/🟦️.ts";
import { type OracleRegistry, profileTable } from "../🤝️contributions/🟦️.ts";
import { resolveTestInputs, testInputUrisIn } from "../📥️fixture-inputs/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
import { canonicalTestCaseName, testImplementationFilenames } from "../🗺️test-layout/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧾️Contract
const MUTATION_FIXTURE_DIRECTORIES = ["🦠️mutation", "📸️snapshot", "📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🔺️diff", "🎯️outcome"] as const;
const MUTATION_FIXTURE_FILES = ["🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🎯️outcome/🔣️.json"] as const;

function childDirectories(abs: string): string[] {
  if (!existsSync(abs)) return [];
  return readdirSync(abs, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink()).map((entry) => entry.name).sort();
}

/** 🧩️ Whether a case child directory is a split piece of the case's own adapters: a tree that holds only taxonomy-declared adapter files and such directories. */
function adapterModuleTree(abs: string, adapterNames: ReadonlySet<string>): boolean {
  const entries = readdirSync(abs, { withFileTypes: true });
  return entries.length > 0 && entries.every((entry) => !entry.isSymbolicLink() && (entry.isDirectory() ? adapterModuleTree(join(abs, entry.name), adapterNames) : adapterNames.has(entry.name)));
}

function vectorBundleNodes(abs: string): string[] {
  if (!existsSync(abs) || !lstatSync(abs).isDirectory()) return [];
  const nodes: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      const rel = relative(abs, full).split(sep).join("/");
      if (entry.isDirectory() && !entry.isSymbolicLink()) {
        nodes.push(`${rel}/`);
        walk(full);
      } else nodes.push(entry.isSymbolicLink() ? `${rel}@symlink` : rel);
    }
  };
  walk(abs);
  return nodes.sort();
}

/** 🧫️ Describes a closed example bundle with exactly one diff representation. */
function expectedVectorBundle(abs: string): string[] | null {
  const variants = ["🔺️diff/🔣️.json", "🔺️diff/🚫️.absent"].filter(path => existsSync(join(abs, path)) && lstatSync(join(abs, path)).isFile());
  return variants.length === 1 ? [...MUTATION_FIXTURE_DIRECTORIES.map(path => `${path}/`), ...MUTATION_FIXTURE_FILES, variants[0]!].sort() : null;
}

/** 📦️ Validates data-only mutation evidence independently of its executable test case. */
function bundleBreach(scope: string): BreachRecord | null {
  const expected = expectedVectorBundle(scope), actual = vectorBundleNodes(scope);
  if (expected !== null && JSON.stringify(actual) === JSON.stringify(expected)) return null;
  return breach("testing/contract", "mutation-vector-bundle-invalid", scope, "Mutation fixture does not have the closed data-only bundle shape", "Mutation, snapshots, diff and outcome are examples; implementations belong in a separate canonical test case.", "Restore the exact fixture leaves with one diff alternative and no source, extra children or symlinks.");
}

/** 🧬️ Audits each registered mutation's canonical implementation case and separately owned fixture bundle. */
export function mutationVectorRegistryBreaches(repoRoot: string, registry: OracleRegistry, taxonomy: TestTaxonomy = testTaxonomy(repoRoot)): BreachRecord[] {
  const implementationNames = new Set(testImplementationFilenames(taxonomy));
  const breaches: BreachRecord[] = [], represented = new Set<string>(), sweeps = new Map<string, { sourceRoot: string; fixtureRoot: string; grouped: boolean }>();
  for (const contribution of registry.contributions) for (const catalog of contribution.mutationCatalogs) {
    const problems = mutationCatalogProblems(catalog, contribution.owner, taxonomy);
    for (const problem of problems) breaches.push(breach("testing/contract", "mutation-vector-catalog-invalid", contribution.manifestPath, problem, "Every mutation example and test has one exact semantic owner.", "Correct the catalog's current identities and paths."));
    if (problems.length) continue;
    const sourceOwner = mutationCatalogSourceOwner(contribution.owner, taxonomy)!;
    const relativeRoot = `${sourceOwner}/🧬️schema/🧬️mutations`, sourceRoot = join(repoRoot, relativeRoot), fixtureRoot = join(repoRoot, sourceOwner, taxonomy.testsDirName, taxonomy.testFixturesDirName, "🧬️mutations");
    const grouped = Object.hasOwn(taxonomy.mutationDomainOwners, relativeRoot);
    sweeps.set(sourceRoot, { sourceRoot, fixtureRoot, grouped });
    for (const vector of catalog.vectors) {
      const owner = grouped ? mutationOwnerRelativePath(relativeRoot, vector.mutationId, taxonomy)! : vector.sourceMutationDirectoryName;
      for (const scenario of vector.scenarios) {
        const casePath = join(sourceRoot, owner, "🧪️tests", scenario.directoryName), fixturePath = join(fixtureRoot, owner, scenario.directoryName);
        const key = relative(repoRoot, casePath).split(sep).join("/");
        if (represented.has(key)) breaches.push(breach("testing/contract", "mutation-vector-duplicate-owner", contribution.manifestPath, `Mutation test ${key} is claimed by more than one catalog`, "Each test and fixture pair has one catalog owner.", "Remove the duplicate declaration."));
        represented.add(key);
        const implementations = existsSync(casePath) && lstatSync(casePath).isDirectory() ? readdirSync(casePath, { withFileTypes: true }).filter(entry => entry.isFile() && implementationNames.has(entry.name)) : [];
        if (!implementations.length || !existsSync(fixturePath)) {
          breaches.push(breach("testing/contract", "mutation-vector-missing", contribution.manifestPath, `Mutation ${key} requires both its canonical implementation and ${relative(repoRoot, fixturePath).split(sep).join("/")}`, "Fixture data and executable tests are separate, required parts of the same evidence.", "Restore the missing canonical case or owner fixture bundle."));
        }
        if (existsSync(fixturePath)) { const finding = bundleBreach(fixturePath); if (finding) breaches.push(finding); }
      }
    }
  }
  for (const { sourceRoot, fixtureRoot, grouped } of sweeps.values()) {
    const owners = (root: string): string[] => childDirectories(root).flatMap(name => grouped ? childDirectories(join(root, name)).map(operation => `${name}/${operation}`) : [name]);
    for (const owner of owners(fixtureRoot)) {
      const cases = join(sourceRoot, owner, "🧪️tests");
      for (const scenario of childDirectories(join(fixtureRoot, owner))) {
        const key = relative(repoRoot, join(cases, scenario)).split(sep).join("/");
        if (!represented.has(key)) breaches.push(breach("testing/contract", "mutation-vector-unregistered", key, `Mutation fixture for ${key} is not registered`, "Every physical mutation fixture example has an explicit catalog identity and matching implementation.", "Declare its current mutation and scenario identity."));
      }
    }
  }
  return breaches;
}

/**
 * 🦠️ The completeness gate. A feature that tags `@mutations-<catalog>` claims to exercise the WHOLE
 * declared mutation vocabulary of an artifact; this turns that claim into arithmetic. Every kind the
 * owner declares must appear as both a `mutate-<kind>` scenario (the mutation actually applied and
 * compared against the reference implementation) and an `inverse-<kind>` scenario (the algebraic law
 * that the mutation is undoable), or the case is not exhaustive and says so.
 *
 * The framework reads a DECLARED list, never implementation source: which language the artifact is
 * written in, and whether its vocabulary is an enum, a descriptor table or a directory of leaves, is
 * the owner's business. The owner proves the declaration matches its own code with its own test.
 *
 * 🧫️ A case that runs one scenario per committed VECTOR instead of per kind names its rows by the vector's
 * registered scenario id (`mutate-<scenario>`, `inverse-<scenario>`). Such a row covers — and declares —
 * exactly the kind the catalog's `vectors[]` registers that scenario under, so the arithmetic stays exact:
 * nothing is matched by prefix, and an unregistered row is still a stray.
 */
export function mutationCoverageBreaches(discovered: DiscoveredCase, feature: ParsedFeature, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  if (feature.mutationCatalog === null) return breaches;
  const catalog = registry.mutationCatalogs.find((entry) => entry.id === feature.mutationCatalog);
  if (catalog === undefined) {
    breaches.push(breach("testing/contract", "unknown-mutation-catalog", discovered.featurePath, `Unknown mutation catalog @mutations-${feature.mutationCatalog}`, "A catalog is the declared vocabulary a feature is measured against; claiming one that is not declared measures the feature against nothing.", "Declare the catalog in the owner's taxonomy-declared oracle contribution manifest."));
    return breaches;
  }
  if (catalog.capability !== "" && feature.capability !== null && catalog.capability !== feature.capability) {
    breaches.push(breach("testing/contract", "mutation-catalog-capability-mismatch", discovered.featurePath, `Catalog ${catalog.id} declares capability ${catalog.capability} but the feature declares ${feature.capability}`, "A vocabulary belongs to the capability it mutates; pointing a feature at another capability's catalog would report coverage of behaviour it never exercises.", "Align the feature's @capability- tag with the catalog, or claim the matching catalog."));
  }
  const ids = new Set(feature.scenarios.map((scenario) => scenario.id));
  const vectorKind = new Map(catalog.vectors.flatMap((vector) => vector.scenarios.map((scenario) => [scenario.id, vector.mutationId] as const)));
  const coveredBy = (role: "mutate" | "inverse"): Set<string> => new Set([...ids].filter((id) => id.startsWith(`${role}-`)).map((id) => id.slice(role.length + 1)).map((row) => (catalog.kinds.includes(row) ? row : vectorKind.get(row))).filter((kind): kind is string => kind !== undefined));
  const mutated = coveredBy("mutate");
  const inverted = coveredBy("inverse");
  const missingMutate = catalog.kinds.filter((kind) => !mutated.has(kind));
  const missingInverse = catalog.kinds.filter((kind) => !inverted.has(kind));
  if (missingMutate.length > 0) {
    breaches.push(breach("testing/contract", "mutation-kind-uncovered", discovered.featurePath, `${missingMutate.length} of ${catalog.kinds.length} mutation kind(s) in catalog ${catalog.id} have no mutate scenario: ${missingMutate.join(", ")}`, "Exhaustive coverage is the claim this feature makes by tagging the catalog. An unexercised kind is a mutation the implementation may corrupt with nothing to notice.", `Add a row to the Examples table for each kind so the scenario id ${"mutate-<kind>"} exists.`));
  }
  if (missingInverse.length > 0) {
    breaches.push(breach("testing/contract", "mutation-inverse-uncovered", discovered.featurePath, `${missingInverse.length} mutation kind(s) in catalog ${catalog.id} have no inverse scenario: ${missingInverse.join(", ")}`, "A mutation that cannot be undone breaks the undo history the whole event-sourced runtime rests on, and the failure only ever shows up in a user's session.", `Add the kind to the inverse-law Examples table so the scenario id ${"inverse-<kind>"} exists.`));
  }
  const stray = [...ids].filter((id) => id.startsWith("mutate-") && !catalog.kinds.includes(id.slice("mutate-".length)) && !vectorKind.has(id.slice("mutate-".length)));
  if (stray.length > 0) {
    breaches.push(breach("testing/contract", "mutation-kind-undeclared", discovered.featurePath, `Scenario(s) ${stray.join(", ")} exercise mutation kinds the catalog does not declare`, "The catalog is what the completeness gate counts against; a kind exercised but not declared means the declared vocabulary is out of date and the gate is measuring the wrong set.", "Add the kind to the catalog, or rename the scenario if it is not a mutation case."));
  }
  if ((catalog.deferredKinds ?? []).length > 0) {
    breaches.push(breach("testing/contract", "mutation-kinds-deferred", discovered.featurePath, `Catalog ${catalog.id} defers ${(catalog.deferredKinds ?? []).length} kind(s): ${(catalog.deferredKinds ?? []).join(", ")}`, "A deferred kind is untested surface that no longer shows up as missing. Recording it keeps the debt visible instead of letting the gate report green over it.", "Cover the kinds and empty deferredKinds, or state why they cannot exist.", "medium"));
  }
  return breaches;
}

/**
 * 🪆️ Law 1's placement half, made mechanical. `discoverTestCases` takes a case's PARENT as its
 * owner, so a case sitting one level above a subset directory is discovered and measured against
 * that wrong owner — its `sharedFixtureDir` resolves to the ARTIFACT's fixtures, not the subset's,
 * silently defeating the very placement this ticket spent 642 `unsplit-artifact-subset` closures on.
 *
 * The one signal this trusts is the catalog's own `subsetDirectoryName` — `mutationCatalogProblems`
 * already REQUIRES and VALIDATES it (`catalog profile does not match its contribution owner`)
 * whenever the catalog's own contribution carries standards/subsets coordinates, so a bare
 * `@mutations-<id>` tag resolves to exactly one real, already-audited subset id; nothing here
 * re-derives it from an adapter import or a fixture URI, both of which name a real subset for a pure
 * container/round-trip case too (confirmed live: `gif/🐯️create-and-round-trip-gif`,
 * `jpg/📖️create-and-read-jpeg` and `zip/✏️create-and-edit-archive` each import exactly one subset's `io`
 * while genuinely spanning or exceeding it) and would misfire on precisely the three cases C4 confirmed
 * must stay put. A catalog with no `subsetDirectoryName` — unprofiled, not a standards/subsets
 * vocabulary — yields no verdict rather than a guess, and a feature with no `@mutations-` tag at all
 * is never a candidate, mirroring the same early return `mutationCoverageBreaches` itself takes.
 */
export function caseAboveSubsetBreaches(discovered: DiscoveredCase, feature: ParsedFeature, registry: OracleRegistry): BreachRecord[] {
  if (feature.mutationCatalog === null) return [];
  const catalog = registry.mutationCatalogs.find((entry) => entry.id === feature.mutationCatalog);
  if (catalog?.subsetDirectoryName === undefined || catalog.subsetDirectoryName.length === 0) return [];
  if (subsetCoordinatesOfOwner(discovered.owner) !== null) return [];
  const subset = leadingEmojiIdentity(catalog.subsetDirectoryName).rest;
  return [
    breach(
      "testing/taxonomy",
      "case-above-subset",
      discovered.caseDir,
      `Case claims @mutations-${feature.mutationCatalog}, owned by exactly one subset (${subset}), but sits at ${discovered.owner}, above every subset`,
      "A mutation is owned by the smallest semantic subset. A case sitting above that subset is discovered with the ARTIFACT as its owner, so its sharedFixtureDir resolves against the artifact's fixtures rather than the subset's, and every reader of the tree sees an artifact-wide case for evidence that actually proves one narrow scope.",
      `Move 🧪️tests/${discovered.case} under the ${catalog.subsetDirectoryName} subset's own 🧪️tests, beside catalog ${catalog.id}.`,
    ),
  ];
}

/**
 * 🧾️ The contract phase: everything checkable without executing a single test. A case that fails
 * here can never be reported as passing, because the plan the hosts would receive is not well formed.
 */
export function validateCaseContract(repoRoot: string, discovered: DiscoveredCase, registry: OracleRegistry): BreachRecord[] {
  const taxonomy = testTaxonomy(repoRoot);
  const breaches: BreachRecord[] = [];
  const scope = discovered.caseDir;

  if (!canonicalTestCaseName(discovered.case, taxonomy)) {
    breaches.push(breach("testing/taxonomy", "case-slug", scope, `Test case directory ${JSON.stringify(discovered.case)} needs one canonical emoji and a kebab-case name`, `The name after the leading emoji must match ${taxonomy.testCaseSlugPattern} so a stable identifier can be derived from the path.`, "Use one canonical emoji followed by a kebab-case name."));
  }
  if (discovered.owner.split("/").some(segment => taxonomy.testDeliveryScopeDirectoryNames.includes(segment))) {
    breaches.push(breach("testing/taxonomy", "case-in-delivery-scope", scope, "Test case lives below a delivery scope", "A behaviour is owned by the nearest language-neutral domain owner, before implementation packages and targets.", `Move ${taxonomy.testsDirName}/${discovered.case} up to the owning domain root.`));
  }
  if (Object.keys(discovered.adapters).length === 0) {
    breaches.push(breach("testing/contract", "no-adapter", scope, "Test case has no implementation adapter", "A feature with no adapter can never execute, so it silently contributes zero coverage.", `Add at least one ${Object.values(testAdapterFilenames(taxonomy)).join(" / ")} adapter.`));
  }
  for (const [implementation, adapterPath] of Object.entries(discovered.adapters) as [Implementation, string][]) {
    const entry = ADAPTER_ENTRY_POINTS[implementation];
    if (entry.pattern.test(readFileSync(join(repoRoot, adapterPath), "utf8"))) continue;
    breaches.push(breach("testing/contract", "adapter-entry-point-missing", adapterPath, `The ${implementation} adapter defines no ${entry.signature}`, "A host loads an adapter only through its language's entry point. A file without one is a standalone script: every row it was meant to answer is reported as a host problem instead of a result, in both roles, and the case contributes zero parity.", `Define ${entry.signature} and register a handler for every scenario the feature declares in that language's role.`));
  }

  const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
  for (const error of feature.errors) breaches.push(breach("testing/contract", "feature-syntax", discovered.featurePath, error, "The feature file must parse under the repository's restricted Gherkin profile.", "Fix the feature file syntax."));
  if (feature.capability === null) breaches.push(breach("testing/contract", "missing-capability", discovered.featurePath, "Feature is missing its @capability-<id> tag", "Every feature declares the capability it specifies so owners and implementations can be matched to it.", "Add a feature-level @capability-<id> tag."));
  const knownProfiles = profileTable(registry);
  if (feature.comparison === null) breaches.push(breach("testing/contract", "missing-comparison", discovered.featurePath, "Feature is missing a @comparison-<profile> tag", "Comparison belongs to an owned, tested profile — never to an adapter.", `Add one of ${[...knownProfiles.keys()].sort().join(", ")}.`));
  else if (!knownProfiles.has(feature.comparison)) breaches.push(breach("testing/contract", "unknown-comparison", discovered.featurePath, `Unknown comparison profile @comparison-${feature.comparison}`, "A profile is either one of the framework's domain-neutral profiles or one an owner contributes through its declared oracle manifest.", `Add it to this owner's oracle contribution manifest, or use one of ${[...knownProfiles.keys()].sort().join(", ")}.`));
  if (feature.oracle === null && feature.noOracleDecision === null) {
    breaches.push(breach("testing/oracle", "missing-oracle", discovered.featurePath, "Feature declares neither @oracle-<id> nor @no-oracle-<decision-id>", "A test without a reference implementation or an explicitly recorded no-oracle decision proves only that the code agrees with itself.", "Register an oracle in the oracle registry, or record an approved no-oracle decision."));
  }
  if (feature.oracleInput !== null && feature.oracle === null) {
    breaches.push(breach("testing/oracle", "oracle-input-without-oracle", discovered.featurePath, `@oracle-input-${feature.oracleInput} needs an @oracle-<id>`, "A subject artifact is evidence for an external decoder, never a substitute for one.", "Register the external oracle that consumes the subject artifact."));
  }
  if (feature.oracle !== null && !registry.oracles.some((entry) => entry.id === feature.oracle)) {
    breaches.push(breach("testing/oracle", "unknown-oracle", discovered.featurePath, `Unknown oracle id @oracle-${feature.oracle}`, "Oracles must be centrally registered so their license, test-only status and capabilities are auditable.", "Add the oracle to the registry, or reference an existing id."));
  }
  if (feature.noOracleDecision !== null && !registry.noOracleDecisions.some((entry) => entry.id === feature.noOracleDecision)) {
    breaches.push(breach("testing/oracle", "unknown-no-oracle-decision", discovered.featurePath, `Unknown no-oracle decision @no-oracle-${feature.noOracleDecision}`, "A no-oracle decision must be recorded with its rationale and substitutes, not asserted inline.", "Add the decision to the registry's noOracleDecisions."));
  }
  const oracleEntry = registry.oracles.find((entry) => entry.id === feature.oracle);
  const oracleLanguage = oracleEntry === undefined ? undefined : oracleImplementation(oracleEntry);
  if (oracleEntry && oracleLanguage !== undefined && discovered.adapters[oracleLanguage] === undefined) {
    const differential = feature.scenarios.filter((scenario) => scenario.mode === "differential").length;
    const adapterKey = Object.entries(taxonomy.testImplementationIds).find(([, id]) => id === oracleLanguage)?.[0];
    const adapterFilename = adapterKey === undefined ? oracleLanguage : (testAdapterFilenames(taxonomy)[adapterKey] ?? oracleLanguage);
    breaches.push(breach("testing/oracle", "oracle-adapter-missing", discovered.featurePath, `Oracle ${oracleEntry.id} runs in the ${oracleLanguage} adapter, which this case does not have — ${differential} @mode-differential scenario(s) are compared against nothing`, "The oracle role is dispatched to the adapter of the oracle's own language. Without that adapter the phase records a problem instead of a result, so every differential row stays unexecuted while the case keeps claiming the reference.", `Add the ${adapterFilename} adapter registering the oracle handlers, or declare the oracle that actually runs.`));
  }
  if (oracleEntry && feature.comparison && !oracleEntry.comparisonProfiles.includes(feature.comparison)) {
    breaches.push(breach("testing/oracle", "oracle-profile-mismatch", discovered.featurePath, `Oracle ${oracleEntry.id} does not declare comparison profile ${feature.comparison}`, "An oracle can only be trusted for the projections it was surveyed against.", `Use one of ${oracleEntry.comparisonProfiles.join(", ")}, or extend the registry entry deliberately.`));
  }
  if (oracleEntry && feature.capability && !oracleEntry.capabilities.includes(feature.capability)) {
    breaches.push(breach("testing/oracle", "oracle-capability-mismatch", discovered.featurePath, `Oracle ${oracleEntry.id} does not declare capability ${feature.capability}`, "Reusing an approved oracle outside its surveyed capability set silently skips the research step.", "Extend the registry entry after re-surveying, or pick a different oracle."));
  }
  if (feature.scenarios.length === 0) breaches.push(breach("testing/contract", "no-scenarios", discovered.featurePath, "Feature declares no scenarios", "An empty feature reports as green while testing nothing.", "Add at least one scenario with @id, @level and @mode tags."));

  // 🧭️A differential scenario compares two producers. Without an oracle it therefore needs a second
  // independently written implementation, and the recorded decision has to actually claim that
  // substitute — otherwise the scenario would be comparing one implementation with nothing.
  const decision = registry.noOracleDecisions.find((entry) => entry.id === feature.noOracleDecision);
  if (feature.oracle === null && decision !== undefined) {
    const differential = feature.scenarios.filter((scenario) => scenario.mode === "differential");
    const claimsImplementations = decision.substitutes.includes("independent-implementations");
    if (differential.length > 0 && Object.keys(discovered.adapters).length < 2 && !claimsImplementations) {
      breaches.push(breach("testing/oracle", "differential-without-evidence", discovered.featurePath, `${differential.length} @mode-differential scenario(s) have neither an oracle nor a second implementation`, `The no-oracle decision ${decision.id} rests on ${decision.substitutes.join(", ")}, none of which discharge a differential comparison.`, "Register an oracle, add a second implementation, or restate the scenarios as @mode-conformance / @mode-property with their vectors in the feature."));
    }
    if (claimsImplementations && Object.keys(discovered.adapters).length < 2) {
      breaches.push(breach("testing/oracle", "claimed-implementations-missing", discovered.caseDir, `No-oracle decision ${decision.id} claims the independent-implementations substitute but the case has ${Object.keys(discovered.adapters).length} adapter(s)`, "A substitute that is claimed but not present is the same as having no evidence at all.", "Add the second implementation's adapter, or record a decision that rests on the substitutes actually in place."));
    }
  }

  breaches.push(...mutationCoverageBreaches(discovered, feature, registry));
  breaches.push(...caseAboveSubsetBreaches(discovered, feature, registry));

  const uris = testInputUrisIn(feature);
  for (const uri of resolveTestInputs(repoRoot, discovered, uris).missing) {
    breaches.push(breach("testing/fixture", "missing-fixture", discovered.featurePath, `Fixture ${uri} does not resolve`, "Fixture lookup is explicit, so an unresolved URI is a contract error rather than a runtime surprise.", `Add the file under ${uri.startsWith("shared://") ? `${discovered.owner}/${taxonomy.testsDirName}/${taxonomy.testFixturesDirName}` : uri.startsWith("asset://") ? `${discovered.owner}/${taxonomy.exampleAssetsDirName}` : `${discovered.owner}/${taxonomy.testsDirName}/${taxonomy.testFixturesDirName}`}.`));
  }

  for (const entry of readdirSync(join(repoRoot, discovered.caseDir), { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (entry.name === taxonomy.testFixturesDirName) continue;
      if (adapterModuleTree(join(repoRoot, discovered.caseDir, entry.name), new Set(Object.values(testAdapterFilenames(taxonomy))))) continue;
      breaches.push(breach("testing/taxonomy", "unknown-case-child", `${discovered.caseDir}/${entry.name}`, `Unexpected directory ${entry.name} inside a test case`, `A case holds one ${testFeatureFilename(taxonomy)}, its adapters, and optional ${taxonomy.testFixturesDirName} example data.`, "Move the directory to its owner, or delete it."));
      continue;
    }
    if (entry.name === testFeatureFilename(taxonomy)) continue;
    if (Object.values(testAdapterFilenames(taxonomy)).includes(entry.name)) continue;
    breaches.push(breach("testing/taxonomy", "unknown-adapter-filename", `${discovered.caseDir}/${entry.name}`, `Unknown file ${entry.name} inside a test case`, "Only the feature file and taxonomy-declared adapters may live in a case directory; anything else is a committed generated wrapper or a stray scratch file.", `Rename it to one of ${Object.values(testAdapterFilenames(taxonomy)).join(", ")}, or delete it.`));
  }

  return breaches;
}
//#endregion 🧾️Contract
