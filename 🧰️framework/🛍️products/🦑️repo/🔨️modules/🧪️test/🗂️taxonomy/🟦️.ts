/** 🗂️ Test taxonomy
 *
 * The frozen test vocabulary read out of the SSOT `🔣️taxonomy.json`: file kinds, path exclusions, test locations and the comparison-profile spec shape.
 */

//#region 🔌️Adapters
import { type Implementation, IMPLEMENTATIONS } from "@semio-tech/framework/test/adapter";
import { readFileSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { getSemioRoot } from "../../📚️library/🗂️workspaces/🧬️metadata/🟦️.ts";
import { type Taxonomy } from "../../📚️library/🔍️discovery/📑️taxonomy-document/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔣️Contract
/** 🔣️ Repo-relative path of the SSOT taxonomy the whole test platform reads its vocabulary from. */
export const TAXONOMY_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

/** 🧪️ Repo-relative root of the testing domain that owns this contract. */
export const TEST_DOMAIN_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";

/** 🧪️ Implementations a test case may claim an adapter for. */



/** 🎭️ The two roles one scenario is executed in — the third-party reference and this repository's code. */



/** 🎯️ Execution modes a scenario declares with `@mode-…`. */



/**
 * ⚖️ A comparison profile id. Deliberately an OPEN string rather than a closed union: the framework
 * owns the comparison MECHANISM and its domain-neutral profiles, while a profile that knows a file
 * format is contributed by the owner of that format. Adding one must never require editing this file.
 */


/**
 * ⚖️ The declarative policy of one profile. Everything a profile can say is data, so a new profile is
 * a manifest entry — never new framework code.
 */
export type ComparisonProfileSpec = Readonly<{
  id: string;
  /** 📖️ Why this profile exists and what it deliberately treats as producer freedom. */
  description?: string;
  /** 🔢️ `ordered` keeps array order significant; `set` compares arrays as multisets. */
  arrays?: "ordered" | "set";
  /** 🚫️ Keys stripped before comparison — the producer freedom this format allows. */
  ignoreKeys?: readonly string[];
  /** 📏️ Numeric tolerance, and the grid values are rounded onto before comparison. */
  tolerance?: number;
  /** 🔤️ `utf8` normalizes line endings, trailing whitespace and Unicode form. */
  text?: "none" | "utf8";
  /** 💾️ Compare the whole projection as an opaque byte string. */
  bytes?: boolean;
  /** ⚖️ The multi-artifact, externally-probed pipeline this profile delegates to. A profile that names one compares an artifact BUNDLE; the structural fields above then describe only its projection half. */
  pipeline?: string;
}>;

/** ⚖️ The domain-neutral profiles the framework itself owns. Nothing here knows a file format. */
export const CORE_COMPARISON_PROFILES: readonly ComparisonProfileSpec[] = [
  { id: "exact-bytes-v1", description: "Byte-for-byte identity — used only where byte determinism is itself the requirement.", bytes: true },
  { id: "utf8-text-v1", description: "Text identity after normalizing line endings, trailing whitespace and Unicode form.", text: "utf8" },
  { id: "ordered-json-v1", description: "Structural identity with array order significant; key order never is." },
  { id: "unordered-json-v1", description: "Structural identity with arrays compared as multisets.", arrays: "set" },
  { id: "floating-point-v1", description: "Structural identity with a numeric tolerance for representation noise.", tolerance: 1e-9 },
  { id: "filesystem-tree-v1", description: "A directory listing, where enumeration order is never normative.", arrays: "set" },
  { id: "diagnostic-v1", description: "Diagnostics compared by kind and message, not by position or rendering.", arrays: "set", ignoreKeys: ["line", "column", "offset", "span", "detail"] },
  { id: "event-stream-v1", description: "An event stream compared by content, not by wall-clock or process identity.", ignoreKeys: ["timestamp", "durationMs", "elapsedMs", "pid", "threadId"] },
];

type TestFileKind = Readonly<{ emoji: string; extensionChains: readonly string[] }>;
type TestPathExclusion = Readonly<{ path: string; mode: "opaque"; reason: string }>;
type TestLocation = Readonly<{ directoryPath: string; fileKindId: string }>;
/** 🐹️ A toolchain convention that compiles a test only beside the package it tests, e.g. Go's `_test.go`. */
export type TestInPackageImplementation = Readonly<{ id: string; implementation: string; fileKindId: string; filenameSuffix: string; rationale: string }>;
export type MutationCatalogAuthority = Pick<Taxonomy, "mutationDomainOwners" | "mutationCatalogSourceOwners" | "pathEmojiPolicy" | "schemaScopeOwnerLevels">;

/** 🗂️ Canonical v7 test taxonomy, mirrored from `🔣️taxonomy.json` so drift fails closed. */
export type TestTaxonomy = MutationCatalogAuthority & Readonly<{
  fileKinds: Readonly<Record<string, TestFileKind>>;
  pathExclusions: Readonly<Record<string, TestPathExclusion>>;
  testsDirName: string;
  testFixturesDirName: string;
  testExamplesDirName: string;
  testOraclesDirName: string;
  testObsoleteCategoryStems: readonly string[];
  testObsoleteTestEmojiCategoryStems: readonly string[];
  testFixtureLegacyDirectoryNames: readonly string[];
  exampleAssetsDirName: string;
  testFeatureFileKindId: string;
  testCaseSlugPattern: string;
  testAdapterFileKinds: Readonly<Record<string, string>>;
  testImplementationIds: Readonly<Record<string, string>>;
  testImplementationFileKindIds: readonly string[];
  testLegacyDirectoryNames: readonly string[];
  testLegacyFilenamePatterns: readonly Readonly<{ id: string; pattern: string }>[];
  testInPackageImplementations: readonly TestInPackageImplementation[];
  testRunnerConfigurationCaseName: string;
  /** 🔬️ Owner collections whose members bound the search for a case's subject package. */
  testSubjectBoundaryOwnerKinds: readonly string[];
  testDeliveryScopeDirectoryNames: readonly string[];
  testJavaScriptFrameworkModules: readonly string[];
  testAssertionModules: readonly string[];
  testSelfTestDeclarationPattern: string;
  testOutputCacheDirName: string;
  testOutputMarkerFileKindId: string;
  testOutputMarkerKind: string;
  testOutputChildDirs: readonly string[];
  testOracleRegistryLocation: TestLocation;
  testSchemaLocation: TestLocation;
  testContributionFileKindId: string;
  /** 🔬️ Directory name of an owner's external measurement probes — test-owned by what it is. */
  testProbeDirName: string;
  /** 🏭️ Directory name of an owner's third-party fixture generator — test-owned by what it is. */
  testGeneratorDirName: string;
  /** 🏭️ Directory name of an owner's production mutation bridge. Production-side by design: it links the real implementation so `listMutations` answers from dispatch. */
  testBridgeDirName: string;
  testDomainPath: string;
  testPhases: readonly string[];
  testLevellessPhases: readonly string[];
  testMutationVocabularyDirName: string;
}>;

let taxonomyCache: { root: string; value: TestTaxonomy } | null = null;

/** 🔣️ Loads the frozen test vocabulary out of the SSOT taxonomy. Never re-declare these strings locally. */
export function testTaxonomy(repoRoot: string): TestTaxonomy {
  if (taxonomyCache && taxonomyCache.root === repoRoot) return taxonomyCache.value;
  const parsed = JSON.parse(readFileSync(join(repoRoot, TAXONOMY_REL_PATH), "utf8")) as Record<string, unknown>;
  const required = ["fileKinds", "pathExclusions", "testsDirName", "testFixturesDirName", "testExamplesDirName", "testOraclesDirName", "testObsoleteCategoryStems", "testObsoleteTestEmojiCategoryStems", "testFixtureLegacyDirectoryNames", "exampleAssetsDirName", "testFeatureFileKindId", "testCaseSlugPattern", "testAdapterFileKinds", "testImplementationIds", "testImplementationFileKindIds", "testLegacyDirectoryNames", "testLegacyFilenamePatterns", "testInPackageImplementations", "testRunnerConfigurationCaseName", "testSubjectBoundaryOwnerKinds", "testDeliveryScopeDirectoryNames", "testJavaScriptFrameworkModules", "testAssertionModules", "testSelfTestDeclarationPattern", "testOutputCacheDirName", "testOutputMarkerFileKindId", "testOutputMarkerKind", "testOutputChildDirs", "testOracleRegistryLocation", "testSchemaLocation", "testContributionFileKindId", "testProbeDirName", "testGeneratorDirName", "testBridgeDirName", "testDomainPath", "testPhases", "testLevellessPhases", "testMutationVocabularyDirName"];
  required.push("mutationDomainOwners", "mutationCatalogSourceOwners", "pathEmojiPolicy", "schemaScopeOwnerLevels");
  const moduleDirectory = (parsed.schemaScopeOwnerLevels as { moduleMemberDirName?: unknown } | undefined)?.moduleMemberDirName;
  if (typeof moduleDirectory !== "string" || !moduleDirectory || moduleDirectory.includes("/") || moduleDirectory.includes("\\")) throw Error("Declared module collection vocabulary required");
  const missing = required.filter((key) => parsed[key] === undefined);
  if (missing.length > 0) throw new Error(`🔣️taxonomy.json is missing the test contract keys: ${missing.join(", ")}`);
  const value = Object.fromEntries(required.map((key) => [key, parsed[key]])) as unknown as TestTaxonomy;
  taxonomyCache = { root: repoRoot, value };
  return value;
}

/** 🔮️ Recognizes canonical oracle ownership by exact ancestor scope, independently of manifest validity. */
export function isTestOraclePath(taxonomy: TestTaxonomy, rel: string): boolean {
  return rel.split("/").includes(taxonomy.testOraclesDirName);
}

/** 📄️ Resolves the taxonomy-ordered kind-only filename for a test contract. */
export function testFilenameForKind(taxonomy: TestTaxonomy, kindId: string): string {
  const kind = taxonomy.fileKinds[kindId];
  if (!kind || kind.extensionChains.length === 0) throw new Error(`🔣️taxonomy.json file kind "${kindId}" has no canonical filename`);
  return `${kind.emoji}${kind.extensionChains[0]}`;
}

/** 📍️ Resolves a taxonomy location to its canonical repository-relative file path. */
export function testLocationPath(taxonomy: TestTaxonomy, location: TestLocation): string {
  return `${location.directoryPath}/${testFilenameForKind(taxonomy, location.fileKindId)}`;
}

/** 🥒️ Canonical feature filename. */
export function testFeatureFilename(taxonomy: TestTaxonomy): string {
  return testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
}

/** 🔌️ Canonical implementation adapter filenames keyed by language identifier. */
export function testAdapterFilenames(taxonomy: TestTaxonomy): Readonly<Record<string, string>> {
  return Object.fromEntries(Object.entries(taxonomy.testAdapterFileKinds).map(([key, kindId]) => [key, testFilenameForKind(taxonomy, kindId)]));
}

/** 🚫️ Excluded paths come from the taxonomy and are applied HERE, in the discovery library — never
 * only by a CI path filter. This function names no area; which ones are excluded is vocabulary. */
export function isExcludedTestPath(repoRoot: string, relPath: string): boolean {
  const normalized = relPath.split(sep).join("/");
  // 🚫️The repository's OWN semio root is excluded structurally, not by a listed path: it holds
  // tickets, caches, notes, metrics and hub runtime data (whose trusted-catalog builds carry cargo
  // `examples/` output), and a ticket folder routinely contains a SCRATCH COPY of a plugin subtree.
  // Those copies were discovered as real owners and real test cases. It is derived from
  // `getSemioRoot` rather than spelled here so relocating it is still a vocabulary change.
  const metaRoot = relative(repoRoot, getSemioRoot(repoRoot)).split(sep).join("/");
  if (metaRoot.length > 0 && !metaRoot.startsWith("..") && (normalized === metaRoot || normalized.startsWith(`${metaRoot}/`))) return true;
  return Object.values(testTaxonomy(repoRoot).pathExclusions).some(({ path }) => normalized === path.replace(/\/$/, "") || normalized.startsWith(path) || normalized.includes(`/${path}`));
}

/** 🧭️ Maps a taxonomy adapter filename key (`🦀️rust`) to its stable implementation id (`rust`). */
export function implementationOfAdapterKey(repoRoot: string, key: string): Implementation | null {
  const id = testTaxonomy(repoRoot).testImplementationIds[key];
  return (IMPLEMENTATIONS as readonly string[]).includes(id ?? "") ? (id as Implementation) : null;
}
//#endregion 🔣️Contract
