/** 🗺️ Test layout
 *
 * Placement of tests and fixtures: path assessment, source signals and the layout scan.
 */

//#region 🔌️Adapters
import { opendir, readFile as readFileAsync } from "node:fs/promises";
import { basename, join, posix, relative, sep } from "node:path";
import ts from "typescript";
import { type BreachRecord } from "../../📚️library/🚨️policy/🔖️breach/🟦️.ts";
import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { isExcludedTestPath, testFeatureFilename, type TestInPackageImplementation, testTaxonomy, type TestTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { SKIP_DIR_NAMES, testCollectionIndexes, testFixtureCollectionIndex } from "../🧭️case-discovery/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧾️Contract
export const TEST_LAYOUT_FINDING_CODES = ["legacy-test-directory", "legacy-test-filename", "test-implementation-depth", "test-implementation-filename", "test-case-name", "test-owner-delivery-scope", "inline-test-body", "inline-self-test-declaration", "invalid-test-module-wiring", "fixture-outside-tests", "fixture-owner-delivery-scope", "test-data-in-case", "production-fixture-dependency", "legacy-fixture-directory", "obsolete-testing-category"] as const;
export type TestLayoutFindingCode = (typeof TEST_LAYOUT_FINDING_CODES)[number];
export type TestLayoutFinding = Readonly<{ code: TestLayoutFindingCode; path: string; line: number | null; detail: string }>;
export type TestLayoutSource = Readonly<{ path: string; source: string }>;
export type TestLayoutProgress = Readonly<{ phase: "discover" | "inspect"; completed: number; total: number }>;
export type TestLayoutScanOptions = Readonly<{ signal?: AbortSignal; progress?: (progress: TestLayoutProgress) => void; concurrency?: number }>;

type TestSourceSignal = Readonly<{ code: "inline-test-body" | "inline-self-test-declaration" | "invalid-test-module-wiring"; line: number; detail: string }>;
type TestPathAssessment = Readonly<{ canonical: boolean; finding: TestLayoutFinding | null }>;

/** 🧪️ All canonical implementation filenames, including every executable extension chain of each declared file kind. */
export function testImplementationFilenames(taxonomy: TestTaxonomy): readonly string[] {
  return [...new Set(taxonomy.testImplementationFileKindIds.flatMap((kindId) => {
    const kind = taxonomy.fileKinds[kindId];
    if (!kind) return [];
    return kind.extensionChains.filter((extension) => extension !== ".d.ts").map((extension) => `${kind.emoji}${extension}`);
  }))].sort();
}

/** 🧭️ Identifies source extensions without treating their legacy basename as authority. */
function testSourceExtension(name: string, taxonomy: TestTaxonomy): string | null {
  const chains = taxonomy.testImplementationFileKindIds.flatMap((kindId) => taxonomy.fileKinds[kindId]?.extensionChains ?? []).filter((extension) => extension !== ".d.ts").sort((a, b) => b.length - a.length);
  return chains.find((extension) => name.endsWith(extension)) ?? null;
}

/** 🏷️ Recognizes the shared feature-contract and implementation-directory identity. */
export function canonicalTestCaseName(name: string, taxonomy: TestTaxonomy): boolean {
  const identity = leadingEmojiIdentity(name);
  return !!identity.first && new RegExp(taxonomy.testCaseSlugPattern, "u").test(identity.rest) && pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length === 0;
}

const registeredFileKindFilenamesCache = new WeakMap<TestTaxonomy, ReadonlySet<string>>();

/** 🏷️ Every canonical kind-only filename the taxonomy registers; a registered file kind is never a legacy test filename. */
function registeredFileKindFilenames(taxonomy: TestTaxonomy): ReadonlySet<string> {
  let names = registeredFileKindFilenamesCache.get(taxonomy);
  if (!names) registeredFileKindFilenamesCache.set(taxonomy, names = new Set(Object.values(taxonomy.fileKinds).flatMap((kind) => kind.extensionChains.map((extension) => `${kind.emoji}${extension}`))));
  return names;
}

/** 🐹️ Names the in-package convention a test source follows when a production source of the same file kind sits beside it. */
export function inPackageTestImplementation(name: string, siblingNames: Iterable<string>, taxonomy: TestTaxonomy): TestInPackageImplementation | null {
  for (const entry of taxonomy.testInPackageImplementations) {
    if (!name.endsWith(entry.filenameSuffix) || name.length === entry.filenameSuffix.length) continue;
    const extensions = taxonomy.fileKinds[entry.fileKindId]?.extensionChains ?? [];
    for (const sibling of siblingNames) if (!sibling.endsWith(entry.filenameSuffix) && extensions.some((extension) => sibling.endsWith(extension))) return entry;
  }
  return null;
}

const siblingNamesCache = new WeakMap<ReadonlySet<string>, ReadonlyMap<string, readonly string[]>>();

/** 🗂️ Groups an inventory's file names by directory once per inventory. */
function siblingNamesOf(path: string, availablePaths: ReadonlySet<string>): readonly string[] {
  let byDirectory = siblingNamesCache.get(availablePaths);
  if (!byDirectory) {
    const grouped = new Map<string, string[]>();
    for (const entry of availablePaths) {
      const directory = posix.dirname(entry);
      const names = grouped.get(directory);
      if (names) names.push(posix.basename(entry));
      else grouped.set(directory, [posix.basename(entry)]);
    }
    siblingNamesCache.set(availablePaths, byDirectory = grouped);
  }
  return byDirectory.get(posix.dirname(path)) ?? [];
}

/** 📐️ Measures one source path against `<semantic owner>/🧪️tests/<case>/<implementation>`. */
export function assessTestImplementationPath(path: string, taxonomy: TestTaxonomy): TestPathAssessment {
  const segments = path.split("/");
  const tests = testCollectionIndexes(path, taxonomy.testsDirName, taxonomy);
  if (tests.length === 0) return { canonical: false, finding: null };
  const index = tests.at(-1)!;
  if (tests.length !== 1 || index !== segments.length - 3) return { canonical: false, finding: { code: "test-implementation-depth", path, line: null, detail: `Executable test sources must be direct children of ${taxonomy.testsDirName}/<test-name>.` } };
  const owner = segments.slice(0, index);
  const delivery = segments[index + 1] === taxonomy.testRunnerConfigurationCaseName ? undefined : owner.find((segment) => taxonomy.testDeliveryScopeDirectoryNames.includes(segment));
  if (delivery) return { canonical: false, finding: { code: "test-owner-delivery-scope", path, line: null, detail: `The test owner is below delivery scope ${delivery}; move the case to its nearest language-neutral semantic owner.` } };
  if (!canonicalTestCaseName(segments[index + 1]!, taxonomy)) return { canonical: false, finding: { code: "test-case-name", path, line: null, detail: "The test case directory must use one canonical emoji followed by a kebab-case name." } };
  if (!testImplementationFilenames(taxonomy).includes(segments.at(-1)!)) return { canonical: false, finding: { code: "test-implementation-filename", path, line: null, detail: `The implementation filename must be one of ${testImplementationFilenames(taxonomy).join(", ")}.` } };
  return { canonical: true, finding: null };
}

/** 🗂️ Rejects explicit testing buckets while preserving semantic test names and ordinary production domains. */
function obsoleteTestingCategoryFinding(path: string, taxonomy: TestTaxonomy, nodeKind: "file" | "directory" = "file"): TestLayoutFinding | null {
  const segments = path.split("/"), directories = nodeKind === "directory" ? segments : segments.slice(0, -1);
  if (directories.includes(taxonomy.testFixturesDirName) || directories.includes(taxonomy.exampleAssetsDirName)) return null;
  const testEmoji = leadingEmojiIdentity(taxonomy.testsDirName).first;
  const canonical = new Map([taxonomy.testsDirName, taxonomy.testFixturesDirName, taxonomy.testExamplesDirName, taxonomy.testOraclesDirName].map((name) => [leadingEmojiIdentity(name).rest, name]));
  for (let index = 0; index < directories.length; index++) {
    const segment = directories[index]!;
    if (index > 0 && directories[index - 1] === taxonomy.testsDirName) continue;
    if (taxonomy.testLegacyDirectoryNames.includes(segment) || taxonomy.testFixtureLegacyDirectoryNames.includes(segment)) continue;
    const identity = leadingEmojiIdentity(segment), stem = identity.rest.toLocaleLowerCase("und");
    const canonicalName = canonical.get(stem);
    const obsolete = taxonomy.testObsoleteCategoryStems.includes(stem) || identity.first === testEmoji && taxonomy.testObsoleteTestEmojiCategoryStems.includes(stem) || canonicalName !== undefined && segment !== canonicalName;
    if (obsolete) return { code: "obsolete-testing-category", path, line: null, detail: `Obsolete testing category ${segment} is forbidden; use ${taxonomy.testsDirName}, ${taxonomy.testFixturesDirName}, ${taxonomy.testExamplesDirName}, or ${taxonomy.testOraclesDirName} at the semantic owner.` };
  }
  return null;
}

/** 🗂️ Applies category rules to a physical directory so empty legacy buckets cannot evade the file scan. */
function inspectTestLayoutDirectory(taxonomy: TestTaxonomy, input: string): TestLayoutFinding[] {
  const path = input.split("\\").join("/"), segments = path.split("/"), findings: TestLayoutFinding[] = [];
  const obsoleteCategory = obsoleteTestingCategoryFinding(path, taxonomy, "directory");
  if (obsoleteCategory) findings.push(obsoleteCategory);
  if (segments.includes(taxonomy.exampleAssetsDirName)) return findings;
  const fixtures = testFixtureCollectionIndex(path, taxonomy), tests = testCollectionIndexes(path, taxonomy.testsDirName, taxonomy)[0] ?? -1;
  if (fixtures >= 0) {
    if (tests < 0 || tests > fixtures) findings.push({ code: "fixture-outside-tests", path, line: null, detail: "Fixture examples require a testing collection under their language-neutral semantic owner." });
    if (segments.slice(0, fixtures).some((segment) => taxonomy.testDeliveryScopeDirectoryNames.includes(segment))) findings.push({ code: "fixture-owner-delivery-scope", path, line: null, detail: "Fixtures require a language-neutral semantic owner outside package, target, and implementation delivery folders." });
    return findings;
  }
  const legacyTest = segments.find((segment) => taxonomy.testLegacyDirectoryNames.includes(segment));
  if (legacyTest) findings.push({ code: "legacy-test-directory", path, line: null, detail: `Legacy test directory ${legacyTest} is forbidden; use ${taxonomy.testsDirName}/<test-name>/<implementation>.` });
  const legacyFixture = segments.find((segment, index) => taxonomy.testFixtureLegacyDirectoryNames.includes(segment) && !(tests >= 0 && index === tests + 1));
  if (legacyFixture) findings.push({ code: "legacy-fixture-directory", path, line: null, detail: `Fixture examples must use the canonical ${taxonomy.testFixturesDirName} directory. Test implementations and executable support must use their own semantic scopes.` });
  return findings;
}

/** 🧫️ Checks physical fixture ownership and excludes example trees from executable-test interpretation. */
function fixtureLayoutFindings(path: string, taxonomy: TestTaxonomy): TestLayoutFinding[] {
  const segments = path.split("/"), fixtures = testFixtureCollectionIndex(path, taxonomy), tests = testCollectionIndexes(path, taxonomy.testsDirName, taxonomy)[0] ?? -1, findings: TestLayoutFinding[] = [];
  if (fixtures >= 0) {
    if (tests < 0 || tests > fixtures) findings.push({ code: "fixture-outside-tests", path, line: null, detail: "Fixture examples require a testing collection under their language-neutral semantic owner." });
    if (segments.slice(0, fixtures).some(segment => taxonomy.testDeliveryScopeDirectoryNames.includes(segment))) findings.push({ code: "fixture-owner-delivery-scope", path, line: null, detail: "Fixtures require a language-neutral semantic owner outside package, target, and implementation delivery folders." });
    return findings;
  }
  if (segments.slice(0, -1).some((segment, index) => taxonomy.testFixtureLegacyDirectoryNames.includes(segment) && !(tests >= 0 && index === tests + 1))) findings.push({ code: "legacy-fixture-directory", path, line: null, detail: `Fixture examples must use the canonical ${taxonomy.testFixturesDirName} directory. Test implementations and executable support must use their own semantic scopes.` });
  if (tests >= 0 && !testSourceExtension(segments.at(-1)!, taxonomy) && segments.at(-1) !== testFeatureFilename(taxonomy)) findings.push({ code: "test-data-in-case", path, line: null, detail: "Test folders contain executable implementations and feature contracts; example inputs and expected outputs belong in owner fixtures." });
  return findings;
}

/** 🎭️ Masks C-family comments and literals while preserving offsets, newlines, and literal delimiters. */
function maskCStyleSource(source: string, rust: boolean): string {
  const out = source.split("");
  const blank = (index: number): void => { if (out[index] !== "\n" && out[index] !== "\r") out[index] = " "; };
  let index = 0;
  while (index < source.length) {
    if (source.startsWith("//", index)) {
      while (index < source.length && source[index] !== "\n") blank(index++);
      continue;
    }
    if (source.startsWith("/*", index)) {
      let depth = 1;
      blank(index++); blank(index++);
      while (index < source.length && depth > 0) {
        if (source.startsWith("/*", index) && rust) { depth++; blank(index++); blank(index++); }
        else if (source.startsWith("*/", index)) { depth--; blank(index++); blank(index++); }
        else blank(index++);
      }
      continue;
    }
    const raw = rust ? /^(?:br|rb|r)(#*)"/u.exec(source.slice(index)) : null;
    if (raw) {
      const opening = raw[0], hashes = raw[1] ?? "", closing = `"${hashes}`;
      index += opening.length;
      const end = source.indexOf(closing, index);
      const limit = end < 0 ? source.length : end;
      while (index < limit) blank(index++);
      index = end < 0 ? source.length : end + closing.length;
      continue;
    }
    const quote = source[index], rustCharacter = rust && quote === "'" && /^'(?:\\(?:x[0-9A-Fa-f]{2}|u\{[0-9A-Fa-f_]+\}|.)|[^'\\\r\n])'/u.test(source.slice(index));
    if (quote === '"' || !rust && (quote === "'" || quote === "`") || rustCharacter) {
      index++;
      while (index < source.length) {
        if (source[index] === "\\") { blank(index++); if (index < source.length) blank(index++); continue; }
        if (source[index] === quote) { index++; break; }
        blank(index++);
      }
      continue;
    }
    index++;
  }
  return out.join("");
}

/** 🧭️ Resolves one relative wiring target and proves it names a discovered canonical implementation. */
function canonicalTestImport(path: string, declared: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>, inlineModuleAncestors: readonly string[] = []): boolean {
  const root = "/repo", target = posix.resolve(root, posix.dirname(path), ...inlineModuleAncestors, declared.split("\\").join("/"));
  if (target !== root && !target.startsWith(`${root}/`)) return false;
  const relativeTarget = posix.relative(root, target);
  return assessTestImplementationPath(relativeTarget, taxonomy).canonical && availablePaths.has(relativeTarget);
}

/** 🪧️ Reads an actual Rust path attribute without treating comments or string contents as attributes. */
function rustPathAttribute(source: string, masked: string, offset: number): string | undefined {
  const attribute = /#\s*\[\s*path\s*=\s*"[^"]*"\s*\]/u.exec(masked);
  return attribute ? /"([^"]*)"/u.exec(source.slice(offset + attribute.index, offset + attribute.index + attribute[0].length))?.[1] : undefined;
}

/** 🪆️ Resolves enclosing Rust inline-module directories, including explicit #[path] attributes. */
function rustInlineModuleAncestors(source: string, masked: string, offset: number): string[] {
  const stack: (string | null)[] = [];
  for (const match of masked.slice(0, offset).matchAll(/((?:#\s*\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{|[{}]/gu)) {
    if (match[2]) {
      stack.push(rustPathAttribute(source, match[1]!, match.index!) ?? match[2]);
    }
    else if (match[0] === "{") stack.push(null);
    else stack.pop();
  }
  return stack.filter((entry): entry is string => entry !== null);
}

/** ⚙️ Proves a cfg expression cannot enable its item unless the Rust test configuration is active. */
function rustCfgRequiresTest(expression: string): boolean {
  const value = expression.trim();
  if (value === "test") return true;
  const call = /^([A-Za-z_][A-Za-z0-9_]*)\((.*)\)$/su.exec(value);
  if (!call || call[1] === "not") return false;
  const arguments_: string[] = [];
  let depth = 0, start = 0;
  for (let index = 0; index <= call[2]!.length; index += 1) {
    const token = call[2]![index];
    if (token === "(") depth += 1;
    else if (token === ")") depth -= 1;
    else if ((token === "," || index === call[2]!.length) && depth === 0) { arguments_.push(call[2]!.slice(start, index)); start = index + 1; }
  }
  const requirements = arguments_.map(rustCfgRequiresTest);
  return call[1] === "all" ? requirements.some(Boolean) : call[1] === "any" && requirements.length > 0 && requirements.every(Boolean);
}

/** ⚙️ Reads a cfg attribute conservatively, leaving mixed production/test gates untouched. */
function rustAttributesRequireTest(attributes: string): boolean {
  for (const match of attributes.matchAll(/cfg\s*\((.*)\)/gu)) if (rustCfgRequiresTest(match[1]!)) return true;
  return false;
}

/** 🦀️ Finds executable Rust bodies and validates external test-module wiring. */
function rustTestSignals(path: string, source: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>): TestSourceSignal[] {
  const masked = maskCStyleSource(source, true), found: TestSourceSignal[] = [];
  const line = (offset: number): number => source.slice(0, offset).split("\n").length;
  const addMatches = (pattern: RegExp, code: TestSourceSignal["code"], detail: string): void => {
    for (const match of masked.matchAll(pattern)) found.push({ code, line: line(match.index!), detail });
  };
  addMatches(/#\s*\[\s*(?:(?:[A-Za-z_][A-Za-z0-9_]*::)*(?:test|async_test|wasm_bindgen_test))(?:\s*\([^\]]*\))?\s*\]/gu, "inline-test-body", "Rust test attribute is outside a canonical test implementation.");
  addMatches(/\bfn\s+[A-Za-z0-9_]*self_tests?\s*\(/giu, "inline-self-test-declaration", "Rust self-test declaration is outside a canonical test implementation.");
  const modulePattern = /((?:#\s*\[[^\]\n]+\]\s*)+)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])/gu;
  for (const match of masked.matchAll(modulePattern)) {
    const attributes = match[1]!, name = match[2]!, boundary = match[3]!;
    const hasCfg = /cfg\s*\(/u.test(attributes), requiresTest = rustAttributesRequireTest(attributes);
    if (!requiresTest && (hasCfg || name !== "tests")) continue;
    if (boundary === "{") {
      found.push({ code: "inline-test-body", line: line(match.index!), detail: "Inline Rust test modules must move to a canonical test implementation." });
      continue;
    }
    const declared = rustPathAttribute(source, attributes, match.index!);
    const ancestors = rustInlineModuleAncestors(source, masked, match.index!);
    const segments = declared?.split(/[\\/]/u) ?? [];
    const testModule = /(?:^|_)tests?$/u.test(name) || segments.some(segment => segment === taxonomy.testsDirName || taxonomy.testLegacyDirectoryNames.includes(segment)) || taxonomy.testLegacyFilenamePatterns.some(entry => new RegExp(entry.pattern, "u").test(segments.at(-1) ?? ""));
    const supportTarget = declared && posix.relative("/repo", posix.resolve("/repo", posix.dirname(path), ...ancestors, declared));
    if (!testModule && supportTarget && availablePaths.has(supportTarget)) continue;
    if (!declared || !canonicalTestImport(path, declared, taxonomy, availablePaths, ancestors)) found.push({ code: "invalid-test-module-wiring", line: line(match.index!), detail: "External Rust test modules must use #[path] to an existing canonical test implementation." });
  }
  const includePattern = /((?:#\s*\[[^\]\n]+\]\s*)*)include!\s*\(\s*"[^"]+"\s*\)\s*;/gu;
  for (const match of masked.matchAll(includePattern)) {
    const original = source.slice(match.index!, match.index! + match[0].length), declared = /include!\s*\(\s*"([^"]+)"/u.exec(original)?.[1];
    const testWiring = rustAttributesRequireTest(match[1]!) || declared?.includes(taxonomy.testsDirName) === true;
    if (testWiring && (!rustAttributesRequireTest(match[1]!) || !declared || !canonicalTestImport(path, declared, taxonomy, availablePaths))) found.push({ code: "invalid-test-module-wiring", line: line(match.index!), detail: "Rust test includes must use a test-only #[cfg] gate and target an existing canonical test implementation." });
  }
  return found;
}

/** 🛡️ Proves an enclosing Rust item is disabled in every build without cfg(test). */
function rustOffsetRequiresTest(masked: string, offset: number): boolean {
  const stack = [{ start: 0, test: false }];
  for (const match of masked.slice(0, offset).matchAll(/[{};]/gu)) {
    const current = stack.at(-1)!;
    if (match[0] === "{") stack.push({ start: match.index! + 1, test: current.test || rustAttributesRequireTest(masked.slice(current.start, match.index)) });
    else if (match[0] === "}") { if (stack.length > 1) stack.pop(); stack.at(-1)!.start = match.index! + 1; }
    else current.start = match.index! + 1;
  }
  const current = stack.at(-1)!;
  return current.test || rustAttributesRequireTest(masked.slice(current.start, offset));
}

/** 🚧️ Resolves actual language import/include edges before treating fixture source files as opaque examples. */
function fixtureSourceDependencies(path: string, source: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>): TestLayoutFinding[] {
  if (assessTestImplementationPath(path, taxonomy).canonical || testFixtureCollectionIndex(path, taxonomy) >= 0) return [];
  const findings: TestLayoutFinding[] = [];
  const add = (declared: string, offset: number, ancestors: readonly string[] = []): void => {
    const absolute = posix.resolve("/repo", posix.dirname(path), ...ancestors, declared);
    if (!absolute.startsWith("/repo/")) return;
    const target = posix.relative("/repo", absolute);
    if (testFixtureCollectionIndex(target, taxonomy) < 0) return;
    const resolved = [target, ...[".ts", ".tsx", ".js", ".mjs", ".rs", ".json"].map(extension => target + extension)].find(candidate => availablePaths.has(candidate));
    if (resolved) findings.push({ code: "production-fixture-dependency", path, line: source.slice(0, offset).split("\n").length, detail: `Non-test source resolves a dependency on fixture ${resolved}. Move executable support out of fixtures or place this read in a canonical test implementation.` });
  };
  if (path.endsWith(".rs")) {
    const masked = maskCStyleSource(source, true);
    for (const match of masked.matchAll(/\binclude(?:_str|_bytes)?!\s*\(\s*"[^"]*"\s*\)/gu)) {
      if (rustOffsetRequiresTest(masked, match.index!)) continue;
      const declared = /"([^"]*)"/u.exec(source.slice(match.index!, match.index! + match[0].length))?.[1];
      if (declared) add(declared, match.index!);
    }
    for (const match of masked.matchAll(/((?:#\s*\[[^\]]*\]\s*)+)(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*;/gu)) {
      if (rustOffsetRequiresTest(masked, match.index!) || rustAttributesRequireTest(match[1]!)) continue;
      const declared = rustPathAttribute(source, match[1]!, match.index!);
      if (declared) add(declared, match.index!, rustInlineModuleAncestors(source, masked, match.index!));
    }
  } else if (/\.[cm]?[jt]sx?$/u.test(path)) {
    const root = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, /[jt]sx$/u.test(path) ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
    const bound = javascriptTestBindings(root, new Set(), new Set(), new Set(), new Set());
    const readers = new Set(["readFile", "readFileSync", "createReadStream"]);
    const fileBinding = javascriptTestBindings(root, new Set(["node:fs", "fs", "node:fs/promises", "fs/promises"]), new Set(), readers, new Set());
    const moduleUrl = (node: ts.Expression | undefined): boolean => node !== undefined && ts.isPropertyAccessExpression(node) && node.name.text === "url" && ts.isMetaProperty(node.expression) && node.expression.keywordToken === ts.SyntaxKind.ImportKeyword;
    const readTarget = (node: ts.Expression | undefined): string | undefined => {
      if (!node || !ts.isNewExpression(node) || !ts.isIdentifier(node.expression) || node.expression.text !== "URL" || bound(node.expression) !== undefined || !moduleUrl(node.arguments?.[1])) return;
      const path = node.arguments?.[0];
      return path && ts.isStringLiteralLike(path) ? path.text : undefined;
    };
    const visit = (node: ts.Node): void => {
      if (ts.isImportDeclaration(node) && !node.importClause?.isTypeOnly && ts.isStringLiteralLike(node.moduleSpecifier) && node.moduleSpecifier.text.startsWith(".")) add(node.moduleSpecifier.text, node.getStart(root));
      if (ts.isExportDeclaration(node) && !node.isTypeOnly && node.moduleSpecifier && ts.isStringLiteralLike(node.moduleSpecifier) && node.moduleSpecifier.text.startsWith(".")) add(node.moduleSpecifier.text, node.getStart(root));
      if (ts.isCallExpression(node) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || ts.isIdentifier(node.expression) && node.expression.text === "require" && bound(node.expression) === undefined)) {
        const target = node.arguments[0];
        if (target && ts.isStringLiteralLike(target) && target.text.startsWith(".")) add(target.text, node.getStart(root));
      }
      if (ts.isCallExpression(node)) {
        const call = node.expression;
        const fileReader = ts.isIdentifier(call) && fileBinding(call) === 1 || ts.isPropertyAccessExpression(call) && ts.isIdentifier(call.expression) && (fileBinding(call.expression) === 4 && readers.has(call.name.text) || call.expression.text === "Bun" && bound(call.expression) === undefined && call.name.text === "file");
        const target = fileReader ? readTarget(node.arguments[0]) : undefined;
        if (target) add(target, node.getStart(root));
      }
      ts.forEachChild(node, visit);
    };
    visit(root);
  }
  return findings;
}

/** 📦️ Resolves package entry points, Cargo build nodes, and declared static asset roots. */
function fixtureManifestDependencies(path: string, source: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>): TestLayoutFinding[] {
  const name = posix.basename(path);
  if (!["Cargo.toml", "package.json"].includes(name) || testFixtureCollectionIndex(path, taxonomy) >= 0) return [];
  const record = (value: unknown): Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : {};
  let manifest: Record<string, unknown>;
  try { manifest = record(name === "Cargo.toml" ? Bun.TOML.parse(source) : JSON.parse(source)); } catch { return []; }
  const targets = new Set<string>();
  const add = (value: unknown, repoRelative = false): void => {
    if (typeof value !== "string") return;
    const target = posix.relative("/repo", posix.resolve("/repo", repoRelative ? "." : posix.dirname(path), value));
    if (target.startsWith("..") || testFixtureCollectionIndex(target, taxonomy) < 0) return;
    if (availablePaths.has(target) || [...availablePaths].some(candidate => candidate.startsWith(target + "/"))) targets.add(target);
  };
  const leaves = (value: unknown): unknown[] => typeof value === "string" ? [value] : Array.isArray(value) ? value.flatMap(leaves) : Object.values(record(value)).flatMap(leaves);
  if (name === "Cargo.toml") {
    for (const table of [manifest, ...Object.values(record(manifest.target)).map(record)]) {
      for (const key of ["dependencies", "dev-dependencies", "build-dependencies"]) for (const dependency of Object.values(record(table[key]))) add(record(dependency).path);
    }
    const semio = record(record(record(manifest.package).metadata).semio);
    for (const asset of Array.isArray(semio.assets) ? semio.assets : []) if (record(asset).kind === "static-dir") add(record(asset).root, true);
    for (const target of [manifest.lib, ...["bin", "test", "bench", "example"].flatMap(key => Array.isArray(manifest[key]) ? manifest[key] as unknown[] : [])]) add(record(target).path);
    add(record(manifest.package).build);
  } else {
    for (const key of ["main", "module", "types", "typings", "bin", "exports", "browser"]) for (const target of leaves(manifest[key])) add(target);
    for (const key of ["dependencies", "devDependencies", "optionalDependencies"]) for (const value of Object.values(record(manifest[key]))) if (typeof value === "string" && value.startsWith("file:")) add(value.slice(5));
  }
  return [...targets].sort().map(target => ({ code: "production-fixture-dependency", path, line: null, detail: `Package configuration resolves executable support or shipped static data from fixture ${target}. Executable support belongs outside fixtures; shipped static data belongs under ${taxonomy.exampleAssetsDirName}.` }));
}

/** 🌳️ Resolves JavaScript value bindings through lexical scopes before classifying harness calls. */
function javascriptTestBindings(root: ts.SourceFile, frameworks: ReadonlySet<string>, assertions: ReadonlySet<string>, registrars: ReadonlySet<string>, assertionNames: ReadonlySet<string>): (identifier: ts.Identifier) => number | undefined {
  type Scope = { parent?: Scope; function: boolean; bindings: Map<string, number> };
  const scopes = new WeakMap<ts.Node, Scope>(), top: Scope = { function: true, bindings: new Map() };
  const moduleKind = (name: string): number => frameworks.has(name) ? 4 : assertions.has(name) ? 2 : 0;
  const memberKind = (namespace: number, name: string): number => namespace === 4 ? registrars.has(name) ? 1 : assertionNames.has(name) ? 2 : 0 : namespace;
  const bind = (name: ts.BindingName, scope: Scope, kind = 0): void => {
    if (ts.isIdentifier(name)) scope.bindings.set(name.text, kind);
    else for (const element of name.elements) if (ts.isBindingElement(element)) bind(element.name, scope, memberKind(kind, element.propertyName && ts.isIdentifier(element.propertyName) ? element.propertyName.text : ts.isIdentifier(element.name) ? element.name.text : ""));
  };
  const importedKind = (node: ts.Expression | undefined): number => {
    if (!node) return 0;
    if (ts.isAwaitExpression(node) || ts.isParenthesizedExpression(node)) return importedKind(node.expression);
    if (ts.isPropertyAccessExpression(node) && node.name.text === "vitest" && ts.isMetaProperty(node.expression) && node.expression.keywordToken === ts.SyntaxKind.ImportKeyword) return 4;
    if (!ts.isCallExpression(node) || !node.arguments[0] || !ts.isStringLiteralLike(node.arguments[0])) return 0;
    return node.expression.kind === ts.SyntaxKind.ImportKeyword || ts.isIdentifier(node.expression) && node.expression.text === "require" ? moduleKind(node.arguments[0].text) : 0;
  };
  const visit = (node: ts.Node, parent: Scope): void => {
    if ((ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isEnumDeclaration(node)) && node.name) parent.bindings.set(node.name.text, 0);
    const functionScope = ts.isFunctionLike(node) || ts.isClassStaticBlockDeclaration(node);
    const boundary = functionScope || ts.isBlock(node) || ts.isCaseBlock(node) || ts.isCatchClause(node) || ts.isForStatement(node) || ts.isForInStatement(node) || ts.isForOfStatement(node) || ts.isClassExpression(node) || ts.isModuleBlock(node);
    const scope: Scope = boundary ? { parent, function: functionScope, bindings: new Map() } : parent;
    if (node === root || boundary) scopes.set(node, scope);
    if ((ts.isFunctionExpression(node) || ts.isClassExpression(node)) && node.name) scope.bindings.set(node.name.text, 0);
    if (ts.isParameter(node)) bind(node.name, scope);
    if (ts.isVariableDeclaration(node)) {
      let owner = scope;
      if (ts.isVariableDeclarationList(node.parent) && !(node.parent.flags & ts.NodeFlags.BlockScoped)) while (!owner.function && owner.parent) owner = owner.parent;
      const kind = importedKind(node.initializer);
      bind(node.name, owner, ts.isIdentifier(node.name) && kind === 4 ? 5 : kind);
    }
    if (ts.isImportDeclaration(node) && ts.isStringLiteralLike(node.moduleSpecifier) && node.importClause && !node.importClause.isTypeOnly) {
      const kind = moduleKind(node.moduleSpecifier.text), clause = node.importClause;
      if (clause.name) bind(clause.name, scope, kind === 4 ? 5 : kind);
      if (clause.namedBindings && ts.isNamespaceImport(clause.namedBindings)) bind(clause.namedBindings.name, scope, kind);
      if (clause.namedBindings && ts.isNamedImports(clause.namedBindings)) for (const element of clause.namedBindings.elements) if (!element.isTypeOnly) bind(element.name, scope, memberKind(kind, (element.propertyName ?? element.name).text));
    }
    ts.forEachChild(node, child => visit(child, scope));
  };
  visit(root, top);
  return identifier => {
    let node: ts.Node | undefined = identifier, scope: Scope | undefined;
    while (node && !scope) { scope = scopes.get(node); node = node.parent; }
    while (scope) {
      if (scope.bindings.has(identifier.text)) return scope.bindings.get(identifier.text);
      scope = scope.parent;
    }
    return undefined;
  };
}

/** 🟦️ Finds JavaScript-family test imports, inline bodies, and assertion-bearing self-test declarations through the TypeScript parser. */
function javascriptTestSignals(path: string, source: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>): TestSourceSignal[] {
  const kind = path.endsWith(".tsx") || path.endsWith(".jsx") ? ts.ScriptKind.TSX : path.endsWith(".js") || path.endsWith(".mjs") || path.endsWith(".cjs") ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  const root = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, kind), frameworks = new Set(taxonomy.testJavaScriptFrameworkModules), assertions = new Set(taxonomy.testAssertionModules);
  const registrarNames = new Set(["afterAll", "afterEach", "beforeAll", "beforeEach", "describe", "it", "suite", "test"]), assertionNames = new Set(["assert", "expect"]);
  const binding = javascriptTestBindings(root, frameworks, assertions, registrarNames, assertionNames), modifiers = new Set(["each", "for", "only", "skip", "todo", "concurrent", "sequential", "fails", "runIf", "skipIf", "fixme", "serial", "parallel"]);
  const importMeta: ts.Node[] = [], registrarCalls: ts.CallExpression[] = [], assertionCalls: ts.CallExpression[] = [], dynamicImports: string[] = [], declarations: { node: ts.Node; name: string; body: ts.Node }[] = [];
  const moduleName = (node: ts.Expression): string | null => ts.isStringLiteralLike(node) ? node.text : null;
  const negativelyGated = (node: ts.Node): boolean => {
    let current = node, negative = false;
    while (current.parent) {
      const parent = current.parent;
      if (ts.isParenthesizedExpression(parent) || ts.isAsExpression(parent) || ts.isTypeAssertionExpression(parent) || ts.isNonNullExpression(parent)) { current = parent; continue; }
      if (ts.isPrefixUnaryExpression(parent) && parent.operator === ts.SyntaxKind.ExclamationToken && parent.operand === current) { negative = !negative; current = parent; continue; }
      break;
    }
    return negative;
  };
  const expressionKind = (expression: ts.Expression): number => {
    if (ts.isIdentifier(expression)) return binding(expression) ?? (registrarNames.has(expression.text) ? 1 : expression.text === "expect" ? 2 : 0);
    if (ts.isParenthesizedExpression(expression) || ts.isCallExpression(expression)) return expressionKind(expression.expression);
    if (!ts.isPropertyAccessExpression(expression)) return 0;
    const kind = expressionKind(expression.expression);
    if (kind & 4 && registrarNames.has(expression.name.text)) return 1;
    if (kind & 4 && assertionNames.has(expression.name.text)) return 2;
    return kind & 2 ? 2 : kind & 1 && modifiers.has(expression.name.text) ? 1 : 0;
  };
  const visit = (node: ts.Node): void => {
    if (ts.isPropertyAccessExpression(node) && node.name.text === "vitest" && ts.isMetaProperty(node.expression) && node.expression.keywordToken === ts.SyntaxKind.ImportKeyword && !negativelyGated(node)) importMeta.push(node);
    if (ts.isCallExpression(node)) {
      const expression = node.expression;
      if (expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0]) {
        const specifier = moduleName(node.arguments[0]!);
        if (specifier) dynamicImports.push(specifier);
      }
      const kind = expressionKind(expression);
      if (kind & 1) registrarCalls.push(node);
      if (kind & 2) assertionCalls.push(node);
    }
    if (ts.isFunctionDeclaration(node) && node.name && node.body) declarations.push({ node, name: node.name.text, body: node.body });
    if (ts.isMethodDeclaration(node) && node.name && ts.isIdentifier(node.name) && node.body) declarations.push({ node, name: node.name.text, body: node.body });
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && (ts.isArrowFunction(node.initializer) || ts.isFunctionExpression(node.initializer))) declarations.push({ node, name: node.name.text, body: node.initializer.body });
    if (ts.isPropertyAssignment(node) && (ts.isIdentifier(node.name) || ts.isStringLiteralLike(node.name)) && (ts.isArrowFunction(node.initializer) || ts.isFunctionExpression(node.initializer))) declarations.push({ node, name: node.name.text, body: node.initializer.body });
    ts.forEachChild(node, visit);
  };
  visit(root);
  const line = (node: ts.Node): number => root.getLineAndCharacterOfPosition(node.getStart(root)).line + 1;
  const canonicalImport = dynamicImports.some((specifier) => {
    if (!specifier.startsWith(".")) return false;
    return canonicalTestImport(path, specifier, taxonomy, availablePaths);
  });
  const found: TestSourceSignal[] = [];
  if (registrarCalls.length > 0) found.push({ code: "inline-test-body", line: line(registrarCalls[0]!), detail: "Test registration is outside a canonical test implementation." });
  if (importMeta.length > 0 && (registrarCalls.length > 0 || assertionCalls.length > 0 || !canonicalImport)) found.push({ code: "inline-test-body", line: line(importMeta[0]!), detail: canonicalImport ? "import.meta.vitest wiring still contains inline test or assertion calls." : "import.meta.vitest wiring must dynamically import an existing canonical test implementation." });
  const selfPattern = new RegExp(taxonomy.testSelfTestDeclarationPattern, "u");
  for (const declaration of declarations) {
    const assertion = assertionCalls.some(call => call.pos >= declaration.body.pos && call.end <= declaration.body.end);
    if (/SelfTests?$/u.test(declaration.name) || selfPattern.test(declaration.name) && assertion) found.push({ code: "inline-self-test-declaration", line: line(declaration.node), detail: `Self-test declaration ${declaration.name} is outside a canonical test implementation.` });
    if (declaration.name === "play" && assertion) found.push({ code: "inline-test-body", line: line(declaration.node), detail: "Story interaction tests must move to a canonical test implementation." });
  }
  return found;
}

/** 🐍️ Masks Python comments and strings while preserving offsets and newlines. */
function maskPythonSource(source: string): string {
  const out = source.split("");
  const blank = (index: number): void => { if (out[index] !== "\n" && out[index] !== "\r") out[index] = " "; };
  let index = 0;
  while (index < source.length) {
    if (source[index] === "#") { while (index < source.length && source[index] !== "\n") blank(index++); continue; }
    const prefix = /^(?:[rubfRUBF]{0,2})("""|'''|"|')/u.exec(source.slice(index));
    if (!prefix) { index++; continue; }
    const quote = prefix[1]!, opening = prefix[0];
    index += opening.length;
    while (index < source.length) {
      if (source.startsWith(quote, index)) { index += quote.length; break; }
      if (quote.length === 1 && source[index] === "\\") { blank(index++); if (index < source.length) blank(index++); continue; }
      blank(index++);
    }
  }
  return out.join("");
}

/** 🔎️ Finds executable declarations for non-JavaScript languages after literal masking. */
function conventionalTestSignals(path: string, source: string, extension: string, taxonomy: TestTaxonomy, availablePaths: ReadonlySet<string>): TestSourceSignal[] {
  if (extension === ".rs") return rustTestSignals(path, source, taxonomy, availablePaths);
  const masked = extension === ".py" ? maskPythonSource(source) : maskCStyleSource(source, false), patterns: readonly (readonly [RegExp, string])[] = extension === ".go" ? [[/^[ \t]*func\s+(?:Test|Benchmark|Fuzz)(?![a-z])\w*\s*\(\s*\w+\s+\*testing\.[TBF]\s*\)/gmu, "Go test declaration"]] : extension === ".py" ? [[/^[ \t]*(?:async\s+)?def\s+test_[A-Za-z0-9_]*\s*\(/gmu, "Python test declaration"], [/^[ \t]*class\s+\w+\s*\([^\n)]*(?:unittest\.)?TestCase[^\n)]*\)\s*:/gmu, "Python TestCase declaration"]] : extension === ".cs" ? [[/^[ \t]*\[(?:Fact|Theory|Test|TestCase)(?:\([^\]]*\))?\]/gmu, ".NET test attribute"]] : [];
  const found: TestSourceSignal[] = [];
  for (const [pattern, detail] of patterns) for (const match of masked.matchAll(pattern)) found.push({ code: "inline-test-body", line: source.slice(0, match.index!).split("\n").length, detail: `${detail} is outside a canonical test implementation.` });
  return found;
}

/** 🧫️ Applies the complete path and source contract to an in-memory, language-neutral file set. */
function inspectTestLayoutSource(taxonomy: TestTaxonomy, entry: TestLayoutSource, availablePaths: ReadonlySet<string>, legacyPatterns: readonly Readonly<{ id: string; pattern: RegExp }>[]): TestLayoutFinding[] {
  const findings: TestLayoutFinding[] = [], path = entry.path.split("\\").join("/"), segments = path.split("/"), name = segments.at(-1)!, extension = testSourceExtension(name, taxonomy);
  const obsoleteCategory = obsoleteTestingCategoryFinding(path, taxonomy);
  if (obsoleteCategory) findings.push(obsoleteCategory);
  if (segments.includes(taxonomy.exampleAssetsDirName)) return findings;
  findings.push(...fixtureLayoutFindings(path, taxonomy));
  if (testFixtureCollectionIndex(path, taxonomy) >= 0) return findings;
  findings.push(...fixtureManifestDependencies(path, entry.source, taxonomy, availablePaths));
  if (!extension) return findings;
  findings.push(...fixtureSourceDependencies(path, entry.source, taxonomy, availablePaths));
  const legacyDirectory = segments.slice(0, -1).find((segment) => taxonomy.testLegacyDirectoryNames.includes(segment));
  if (legacyDirectory) findings.push({ code: "legacy-test-directory", path, line: null, detail: `Legacy test directory ${legacyDirectory} is forbidden; use ${taxonomy.testsDirName}/<test-name>/<implementation>.` });
  const legacyFilename = registeredFileKindFilenames(taxonomy).has(name) ? undefined : legacyPatterns.find((entry) => entry.pattern.test(name));
  if (legacyFilename) findings.push({ code: "legacy-test-filename", path, line: null, detail: `Legacy test filename pattern ${legacyFilename.id} is forbidden.` });
  const assessment = assessTestImplementationPath(path, taxonomy), inFixture = testFixtureCollectionIndex(path, taxonomy) >= 0;
  if (assessment.finding && !inFixture) findings.push(assessment.finding);
  if (assessment.canonical) {
    if (extension === ".rs") for (const signal of rustTestSignals(path, entry.source, taxonomy, availablePaths)) if (signal.code === "invalid-test-module-wiring") findings.push({ ...signal, path });
    return findings;
  }
  if (!assessment.finding && inPackageTestImplementation(name, siblingNamesOf(path, availablePaths), taxonomy)) return findings;
  const signals = [".ts", ".tsx", ".mts", ".cts", ".js", ".mjs", ".cjs"].includes(extension) ? javascriptTestSignals(path, entry.source, taxonomy, availablePaths) : conventionalTestSignals(path, entry.source, extension, taxonomy, availablePaths);
  return [...findings, ...signals.map((signal) => ({ ...signal, path }))];
}

/** 🧫️ Applies the complete path and source contract to an in-memory, language-neutral file set. */
export function inspectTestLayoutSources(taxonomy: TestTaxonomy, sources: readonly TestLayoutSource[], directories: readonly string[] = []): TestLayoutFinding[] {
  const authored = sources.filter(entry => !entry.path.split(/[\\/]/u).slice(0, -1).some(segment => SKIP_DIR_NAMES.has(segment)));
  const legacyPatterns = taxonomy.testLegacyFilenamePatterns.map((entry) => ({ id: entry.id, pattern: new RegExp(entry.pattern, "u") })), availablePaths = new Set(authored.map((entry) => entry.path.split("\\").join("/")));
  const authoredDirectories = directories.filter(path => !path.split(/[\\/]/u).some(segment => SKIP_DIR_NAMES.has(segment)));
  const findings = [...authored.flatMap((entry) => inspectTestLayoutSource(taxonomy, entry, availablePaths, legacyPatterns)), ...authoredDirectories.flatMap((path) => inspectTestLayoutDirectory(taxonomy, path))];
  const unique = new Map(findings.map((finding) => [`${finding.code}\0${finding.path}\0${finding.line ?? ""}`, finding]));
  return [...unique.values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || (a.line ?? 0) - (b.line ?? 0));
}

/** 🚶️ Discovers authored source files asynchronously without following symlinks or entering excluded/output trees. */
async function discoverTestLayoutInventory(repoRoot: string, taxonomy: TestTaxonomy, options: TestLayoutScanOptions): Promise<Readonly<{ files: string[]; directories: string[] }>> {
  const stack = [repoRoot], files: string[] = [], directories: string[] = [];
  while (stack.length > 0) {
    options.signal?.throwIfAborted();
    const directory = stack.pop()!, relDirectory = relative(repoRoot, directory).split(sep).join("/");
    if (relDirectory && isExcludedTestPath(repoRoot, relDirectory)) continue;
    let entries;
    try { entries = await opendir(directory); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") continue; throw error; }
    const children = [];
    for await (const entry of entries) children.push(entry);
    children.sort((a, b) => a.name.localeCompare(b.name));
    for (const entry of children) {
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name), rel = relative(repoRoot, path).split(sep).join("/");
      if (entry.isDirectory()) {
        if (!SKIP_DIR_NAMES.has(entry.name) && !isExcludedTestPath(repoRoot, rel)) {
          directories.push(rel);
          stack.push(path);
        }
      } else if (entry.isFile()) files.push(rel);
    }
    options.progress?.({ phase: "discover", completed: files.length + directories.length, total: 0 });
  }
  return { files: files.sort(), directories: directories.sort() };
}

/** ⚡️ Scans the repository with bounded async reads, cancellation, deterministic progress, and sorted findings. */
export async function scanTestLayout(repoRoot: string, options: TestLayoutScanOptions = {}): Promise<TestLayoutFinding[]> {
  const taxonomy = testTaxonomy(repoRoot), inventory = await discoverTestLayoutInventory(repoRoot, taxonomy, options), { files, directories } = inventory, availablePaths = new Set(files), findings: TestLayoutFinding[][] = new Array(files.length);
  const legacyPatterns = taxonomy.testLegacyFilenamePatterns.map((entry) => ({ id: entry.id, pattern: new RegExp(entry.pattern, "u") }));
  const directoryFindings = directories.flatMap((path) => inspectTestLayoutDirectory(taxonomy, path));
  let cursor = 0, completed = directories.length;
  options.progress?.({ phase: "inspect", completed, total: files.length + directories.length });
  const concurrency = Math.max(1, Math.min(options.concurrency ?? 16, 64, files.length || 1));
  await Promise.all(Array.from({ length: concurrency }, async () => {
    while (true) {
      options.signal?.throwIfAborted();
      const index = cursor++;
      if (index >= files.length) return;
      const path = files[index]!;
      try {
        let source: string;
        try { source = (testSourceExtension(basename(path), taxonomy) || ["Cargo.toml", "package.json"].includes(basename(path))) && testFixtureCollectionIndex(path, taxonomy) < 0 ? await readFileAsync(join(repoRoot, path), "utf8") : ""; }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; source = await readFileAsync(join(repoRoot, path), "utf8"); }
        options.signal?.throwIfAborted();
        findings[index] = inspectTestLayoutSource(taxonomy, { path, source }, availablePaths, legacyPatterns);
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
        findings[index] = [];
      }
      options.progress?.({ phase: "inspect", completed: ++completed, total: files.length + directories.length });
    }
  }));
  options.signal?.throwIfAborted();
  const unique = new Map([...directoryFindings, ...findings.flat()].map((finding) => [`${finding.code}\0${finding.path}\0${finding.line ?? ""}`, finding]));
  return [...unique.values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || (a.line ?? 0) - (b.line ?? 0));
}

/** 🚨️ Converts exact layout findings into the repository test contract's breach vocabulary. */
export async function testLayoutBreaches(repoRoot: string, options: TestLayoutScanOptions = {}): Promise<BreachRecord[]> {
  return (await scanTestLayout(repoRoot, options)).map((finding) => breach("testing/layout", finding.code, finding.path, `${finding.detail}${finding.line === null ? "" : ` (line ${finding.line})`}`, "Every executable test has one language-neutral owner and one canonical implementation leaf.", `Move it to <parent>/${testTaxonomy(repoRoot).testsDirName}/<test-name>/<implementation> and remove inline or legacy test declarations.`));
}
//#endregion 🧾️Contract
