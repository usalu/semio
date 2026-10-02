// 🧪️ Nx plugin: one virtual project per discovered test case.
//
// Discovery is every taxonomy-declared kind-only feature beneath `🧪️tests/*`. There are no hand-authored `📋️project.json` files
// for tests, so a case can never be silently omitted from a higher level, and `checkLeveledTestTargets`
// style scanners become unnecessary — the four level targets are generated, always, for every case.
//
// The exclusion set, the case slug rule, the adapter filenames and the location of the testing
// domain are all TAXONOMY DATA (`🔣️taxonomy.json`). This plugin declares none of them, so marking
// another area exempt or relocating the domain is a vocabulary edit, never a code edit here.

import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
const nxPath = (p) => p.split("\\").join("/");
const dependencyModule = new URL("./🕸️dependencies/🟨️.mjs", import.meta.url);
const dependencyRevision = createHash("sha256").update(readAuthoritySource(dependencyModule)).digest("hex");
let cargoPackage, localPackagePath, rustSubjectPackage, ownerContributions, packagesForOwner;
const testBootstrap = import(`${dependencyModule.href}?revision=${dependencyRevision}`).then((deps) => {
  cargoPackage = deps.cargoPackage;
  localPackagePath = deps.localPackagePath;
  rustSubjectPackage = deps.rustSubjectPackage;
  ownerContributions = deps.ownerContributions;
  packagesForOwner = deps.packagesForOwner;
});
const implementationRevision = () => createHash("sha256").update(readAuthoritySource(new URL(import.meta.url))).update(readAuthoritySource(dependencyModule)).digest("hex");
const loadedRevision = implementationRevision();

const TAXONOMY_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const LEVELS = ["quick", "long", "exhaustive"];
const CASE_SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 🏷️ Resolves a single presented emoji and the taxonomy-defined case identifier. */
function canonicalCase(vocabulary, owner, name) {
  const first = CASE_SEGMENTER.segment(name)[Symbol.iterator]().next().value?.segment ?? "";
  const fold = value => value.normalize("NFC").replaceAll("\uFE0E", "").replaceAll("\uFE0F", "");
  return name === name.normalize("NFC") && /[\p{Extended_Pictographic}\p{Emoji_Presentation}\u20E3]/u.test(first)
    && (/\p{Emoji_Presentation}/u.test([...first][0] ?? "") || first.includes("\uFE0F"))
    && !vocabulary.pathEmojiPolicy.genericEmojiIdentities.some(emoji => fold(emoji) === fold(first))
    && new RegExp(vocabulary.testCaseSlugPattern, "u").test(name.slice(first.length))
    && !owner.split("/").some(segment => segment === vocabulary.testsDirName || vocabulary.testDeliveryScopeDirectoryNames.includes(segment));
}

/** 🔣️ Reads the frozen test vocabulary; the plugin never re-declares taxonomy strings. */
function taxonomy(workspaceRoot) {
  const authority = workspaceAdmission(workspaceRoot);
  if (!authority.file(TAXONOMY_REL)) throw new Error("Test discovery requires real no-follow taxonomy authority");
  return JSON.parse(readFileSync(join(authority.root, TAXONOMY_REL), "utf8"));
}

/** 🛡️ Admits fresh workspace ancestry and relative real-file candidates without normalization. */
function workspaceAdmission(input) {
  if (typeof input !== "string" || !input || input.includes("\0") || input.split(/[\\/]/u).some(part => part === "." || part === "..")) throw new Error("Test discovery requires an unnormalized real workspace root");
  const root = resolve(input), ancestry = [];
  for (let current = root; ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
  for (const path of ancestry.reverse()) { const value = lstatSync(path); if (!value.isDirectory() || value.isSymbolicLink()) throw new Error(`Test discovery requires real workspace ancestry: ${path}`); }
  const directories = new Map([[root, true]]);
  const file = value => {
    if (typeof value !== "string" || !value || value.includes("\0")) return false;
    const path = nxPath(value), parts = path.split("/");
    if (isAbsolute(path) || /^[A-Za-z]:/u.test(path) || parts.some(part => !part || part === "." || part === "..")) return false;
    let current = root;
    for (const part of parts.slice(0, -1)) {
      current = join(current, part);
      if (!directories.has(current)) { try { const entry = lstatSync(current); directories.set(current, entry.isDirectory() && !entry.isSymbolicLink()); } catch (error) { if (error.code !== "ENOENT" && error.code !== "ENOTDIR") throw error; directories.set(current, false); } }
      if (!directories.get(current)) return false;
    }
    try { const entry = lstatSync(join(current, parts.at(-1))); return entry.isFile() && !entry.isSymbolicLink(); } catch (error) { if (error.code !== "ENOENT" && error.code !== "ENOTDIR") throw error; return false; }
  };
  return { root, file };
}

/** 📄️ Reads implementation bytes only through fresh physical source authority. */
function readAuthoritySource(url) {
  const path = fileURLToPath(url), authority = workspaceAdmission(dirname(path));
  if (!authority.file(basename(path))) throw new Error(`Test discovery implementation must be a real file: ${path}`);
  return readFileSync(path);
}

/** 📋️ Selects only the complete current Nx inventory admitted by owned physical case rules. */
function candidateFiles(configFiles, vocabulary, authority) {
  const filename = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
  return [...new Set(configFiles.filter(value => typeof value === "string").map(nxPath))].filter(path => {
    if (path.includes("\uFFFD") || isExcluded(vocabulary, path) || !authority.file(path)) return false;
    const casePath = dirname(path), testsPath = dirname(casePath);
    return basename(path) === filename && basename(testsPath) === vocabulary.testsDirName && canonicalCase(vocabulary, dirname(testsPath), basename(casePath));
  }).sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 📄️ Resolves the taxonomy-ordered primary kind-only filename from taxonomy v7. */
function filenameForKind(vocabulary, fileKindId) {
  const kind = vocabulary.fileKinds[fileKindId];
  if (!kind || kind.extensionChains.length === 0) throw new Error(`test Nx plugin requires a canonical filename for file kind ${JSON.stringify(fileKindId)}`);
  return `${kind.emoji}${kind.extensionChains[0]}`;
}

/** 📍️ Resolves a taxonomy v7 directory+file-kind location. */
function locationPath(vocabulary, location) {
  return `${location.directoryPath}/${filenameForKind(vocabulary, location.fileKindId)}`;
}

/** 🚫️ The hard discovery exclusion — `compose/` first among them. */
function isExcluded(vocabulary, relPath) {
  const segments = nxPath(relPath).split("/");
  if (segments.some((segment) => vocabulary.pathEmojiPolicy.reservedSubtreeDirectoryNames.includes(segment))) return true;
  if (vocabulary.implementationLeafPolicy.ignoredPathPatterns.some((pattern) => {
    const suffix = pattern.replace(/^\*\*\//u, "");
    return relPath === suffix || relPath.startsWith(`${suffix}/`) || relPath.endsWith(`/${suffix}`) || relPath.includes(`/${suffix}/`);
  })) return true;
  return Object.values(vocabulary.pathExclusions).some(({ path }) => {
    const prefix = path.replace(/\/$/, "");
    return relPath === prefix || relPath.startsWith(`${prefix}/`);
  });
}

/** 🏷️ Same deterministic project name the coordinator derives, so both agree without coordination. */
function projectNameFor(ownerRel, caseSlug, hash) {
  const slug = ownerRel
    .split("/")
    .map((segment) => segment.replace(/[^a-zA-Z0-9]+/g, "").toLowerCase())
    .filter(Boolean)
    .join("-");
  return `test-${slug || "root"}-${hash}-${caseSlug}`;
}

/** #⃣ The coordinator's owner-path hash, recomputed here so the two never drift. */
async function ownerHash(ownerRel) {
  const { createHash } = await import("node:crypto");
  return createHash("sha256").update(ownerRel).digest("hex").slice(0, 6);
}

/** 🦀️ Resolves the subject selected by the generated host. */
function rustSutCratePath(workspaceRoot, ownerRel) {
  return rustSubjectPackage(workspaceRoot, ownerRel)?.path ?? null;
}

/** 🔮️ Resolves local oracle packages from the owner's applicable contributions. */
function oracleContributionPaths(workspaceRoot, vocabulary, ownerRel) {
  return packagesForOwner(ownerContributions(workspaceRoot, vocabulary, ownerRel), ownerRel)
    .filter((entry) => typeof entry.path === "string").map((entry) => localPackagePath(workspaceRoot, entry.path));
}

/** 🦀️ Attaches Cargo source ownership and generator tasks to the phases that compile a native host. */
function nativeInputsFor(workspaceRoot, vocabulary, ownerRel, adapters, phase, native, policy, state) {
  const filename = filenameForKind(vocabulary, vocabulary.testAdapterFileKinds["🦀️rust"]);
  if (["lint", "test-contract"].includes(phase) || !adapters.some((path) => path.endsWith(`/${filename}`))) return { inputs: [], dependsOn: [] };
  const key = `${ownerRel}\0${phase === "test-oracle"}`;
  if (state.plans.has(key)) return state.plans.get(key);
  const subject = phase === "test-oracle" ? null : rustSubjectPackage(workspaceRoot, ownerRel);
  const packages = packagesForOwner(ownerContributions(workspaceRoot, vocabulary, ownerRel), ownerRel, "rust");
  const roots = [...new Set([`${vocabulary.testDomainPath}/📦️packages/🦀️rust`, ...(subject ? [subject.path] : []), ...packages.filter((entry) => typeof entry.path === "string").map((entry) => localPackagePath(workspaceRoot, entry.path))])];
  const dependencyRoots = [...new Set(roots.flatMap((root) => native.nativeDependencyRoots(root, workspaceRoot, false, state.manifests, state.closures)))];
  const projects = dependencyRoots.map((root) => {
    if (!state.names.has(root)) {
      const authored = join(workspaceRoot, root, "📋️project.json");
      const name = existsSync(authored) ? JSON.parse(readFileSync(authored, "utf8")).name : cargoPackage(workspaceRoot, root)?.name;
      if (!name) throw new Error(`Test dependency has no Nx project identity: ${root}`);
      state.names.set(root, name);
    }
    return state.names.get(root);
  }).sort();
  const dependsOn = [...new Set(roots.flatMap((root) => native.nativePreparation(root, workspaceRoot, vocabulary.generatorContracts, false, state.manifests, state.closures)).map((contract) => contract.target))].sort();
  const cargo = policy.toolchains.cargo;
  const plan = {
    dependsOn,
    inputs: [{ input: "nativeSources", projects }, ...cargo.files.map((path) => `{workspaceRoot}/${path}`), ...cargo.environment.map((env) => ({ env })), ...cargo.commands.map((runtime) => ({ runtime })), ...(dependsOn.length ? [{ dependentTasksOutputFiles: "**/*", transitive: true }] : [])],
  };
  state.plans.set(key, plan);
  return plan;
}

/** 📥️ Cache inputs of one case: the feature, its fixtures, its adapters, the claimed sources, the contract. */
function inputsFor(workspaceRoot, vocabulary, ownerRel, caseRel, adapters) {
  const sharedFixtures = `${ownerRel}/${vocabulary.testFixturesDirName}`;
  const domain = vocabulary.testDomainPath;
  const featureFilename = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
  const rustAdapterFilename = filenameForKind(vocabulary, vocabulary.testAdapterFileKinds["🦀️rust"]);
  const rustSutCrate = adapters.some((adapter) => adapter.endsWith(`/${rustAdapterFilename}`)) ? rustSutCratePath(workspaceRoot, ownerRel) : null;
  const inputs = [
    `{workspaceRoot}/${caseRel}/${featureFilename}`,
    ...(existsSync(join(workspaceRoot, sharedFixtures)) ? [`{workspaceRoot}/${sharedFixtures}/**/*`] : []),
    ...adapters.map((adapter) => `{workspaceRoot}/${adapter}`),
    `{workspaceRoot}/${locationPath(vocabulary, vocabulary.testOracleRegistryLocation)}`,
    `{workspaceRoot}/${locationPath(vocabulary, vocabulary.testSchemaLocation)}`,
    `{workspaceRoot}/${TAXONOMY_REL}`,
    // 🧩️Whatever the platform itself is made of, wherever the taxonomy says it lives.
    `{workspaceRoot}/${domain}/**/*`,
    // 🧩️And every owner contribution, so adding or changing an oracle invalidates the cases that use it.
    `{workspaceRoot}/**/${vocabulary.testOraclesDirName}/**/*`,
    // 🦀️The rust adapter's own crate root, wherever it actually sits — often an ancestor of the owner.
    ...(rustSutCrate === null ? [] : [`{workspaceRoot}/${rustSutCrate}/**/*`]),
    // 🔮️Every path-based oracle host package an ancestor (or the owner itself) contributes: the crate
    // or module a generated host actually links, not just the manifest that names it.
    ...oracleContributionPaths(workspaceRoot, vocabulary, ownerRel).map((path) => `{workspaceRoot}/${path}/**/*`),
    "sharedGlobals",
  ];
  // 🧭️ A change to the owner's own sources must invalidate the case, or a subject regression would
  // be served from cache as a pass. Fixture directories are excluded here because the owner fixture glob
  // above already covers them: a real-world fixture is megabytes, and Nx hashes file CONTENT, so
  // counting it twice per target doubles the hashing cost of every case that owns one.
  inputs.push(`{workspaceRoot}/${ownerRel}/**/*`);
  inputs.push(`!{workspaceRoot}/${ownerRel}/**/${vocabulary.testFixturesDirName}/**/*`);
  return inputs;
}

/** 🎚️ One generated target routed through the testing domain's own router. */
function target(domain, command, inputs, cacheable = true, scope) {
  return {
    executor: "nx:run-commands",
    options: { cwd: domain, command: `bun ./📜️script.ts ${command}`, forwardAllArgs: false, env: { SEMIO_TEST_OUTPUT_SCOPE: scope } },
    inputs,
    // 📤️Only the durable products of a run are cache outputs. The work directory holds each case's
    // mutable fixture copies, which are large, regenerated on every run and meaningless to restore.
    outputs: ["results", "reports", "diffs"].map((child) => `{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tests/tasks/${scope}/${child}`),
    cache: cacheable,
  };
}

/**
 * 🕸️ Generates one project per test case with the full phase and level target set. Level targets are
 * cumulative: `test-long` selects every scenario tagged `fundamental`, `quick` or `long`.
 */
async function testCaseProjects(configFiles, _options, context) {
  const { workspaceRoot } = context;
  const vocabulary = taxonomy(workspaceRoot);
  const authority = workspaceAdmission(workspaceRoot);
  const featureCandidates = candidateFiles(configFiles, vocabulary, authority);
  const results = [];
  const policyPath = `${dirname(TAXONOMY_REL)}/⚡️caching/🔣️policy.json`;
  if (!authority.file(policyPath)) throw new Error("Test discovery requires real no-follow cache policy authority");
  const policy = JSON.parse(readFileSync(join(workspaceRoot, policyPath), "utf8"));
  if (featureCandidates.length === 0) return results;
  const libraryModule = new URL("../📚️library/🟨️.mjs", import.meta.url);
  const libraryRevision = createHash("sha256").update(readAuthoritySource(libraryModule)).digest("hex");
  const { cacheInternals: native } = await import(`${libraryModule.href}?revision=${libraryRevision}`);
  const state = { manifests: new Map(), closures: new Map(), names: new Map(), plans: new Map() };
  const javascript = policy.toolchains.javascript;
  let commandInputs;

  for (const configFile of featureCandidates) {
    if (configFile.includes("\uFFFD")) continue;
    const rel = nxPath(configFile);
    if (rel.startsWith(".generation") && rel.includes("-link/")) continue;
    if (!existsSync(join(workspaceRoot, rel))) continue;
    if (isExcluded(vocabulary, rel)) continue;
    const caseRel = dirname(rel);
    const testsRel = dirname(caseRel);
    if (basename(testsRel) !== vocabulary.testsDirName) continue;

    const ownerRel = dirname(testsRel);
    const caseSlug = basename(caseRel);
    if (!canonicalCase(vocabulary, ownerRel, caseSlug)) continue;
    commandInputs ??= [...native.relativeScriptInputs([join(workspaceRoot, vocabulary.testDomainPath, "📜️script.ts")], workspaceRoot), ...javascript.files.map((path) => `{workspaceRoot}/${path}`), ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime }))];

    const adapters = [];
    for (const fileKindId of Object.values(vocabulary.testAdapterFileKinds)) {
      const filename = filenameForKind(vocabulary, fileKindId);
      const adapterRel = `${caseRel}/${filename}`;
      if (authority.file(adapterRel)) adapters.push(adapterRel);
    }

    const name = projectNameFor(ownerRel, caseSlug, await ownerHash(ownerRel));
    const inputs = inputsFor(workspaceRoot, vocabulary, ownerRel, caseRel, adapters);
    const domain = vocabulary.testDomainPath;
    const select = `--owner ${JSON.stringify(ownerRel)} --case ${caseSlug}`;
    const scoped = (phase, command, cacheable = true) => {
      const nativeInputs = nativeInputsFor(workspaceRoot, vocabulary, ownerRel, adapters, phase, native, policy, state);
      return { ...target(domain, command, [...inputs, "testCommandSources", ...nativeInputs.inputs, { env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }], cacheable, `${name}/${phase}`), dependsOn: nativeInputs.dependsOn };
    };

    results.push([
      configFile,
      {
        projects: {
          [name]: {
            name,
            root: caseRel,
            projectType: "application",
            namedInputs: { testCommandSources: commandInputs },
            tags: ["type:test", `owner:${ownerRel}`, ...adapters.map((adapter) => `impl:${basename(adapter)}`)],
            targets: {
              lint: scoped("lint", `contract ${select}`),
              "test-contract": scoped("test-contract", `contract ${select}`),
              "test-oracle": scoped("test-oracle", `oracle ${select}`),
              "test-subject": scoped("test-subject", `subject ${select}`),
              "test-parity": scoped("test-parity", `parity ${select}`),
              test: scoped("test", `run ${select}`),
              // 🎚️Every level is cached alike: `inputsFor` already hashes the case's full owner-and-oracle
              // closure, `SEMIO_TEST_LEVEL`/`SEMIO_TEST_BUDGET_MS` are hashed env, and outputs are declared —
              // exhaustive is the most expensive level, not a less deterministic one.
              ...Object.fromEntries(LEVELS.map((level) => [`test-${level}`, scoped(`test-${level}`, `run ${level} ${select}`)])),
            },
          },
        },
      },
    ]);
  }

  return results;
}

/** 🕸️ Makes native source changes select their inferred consumer cases in Nx affected runs. */
function testCaseDependencies(_options, context) {
  const vocabulary = taxonomy(context.workspaceRoot);
  return Object.entries(context.projects).flatMap(([source, project]) => {
    if (!project.tags?.includes("type:test")) return [];
    const targets = new Set(Object.values(project.targets ?? {}).flatMap((target) => (target.inputs ?? []).flatMap((input) => input.input === "nativeSources" ? input.projects : [])));
    const featureName = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
    const preferred = `${project.root}/${featureName}`;
    const files = context.fileMap?.projectFileMap?.[source];
    const sourceFile = files?.find((entry) => entry.file === preferred)?.file
      ?? files?.find((entry) => entry.file.endsWith(`/${featureName}`))?.file
      ?? preferred;
    if (!existsSync(join(context.workspaceRoot, sourceFile))) return [];
    return [...targets].map((target) => {
      if (!context.projects[target]) throw new Error(`Test dependency has no Nx project: ${source} → ${target}`);
      return { source, target, sourceFile, type: "static" };
    });
  });
}

/** ♻️ Refreshes resident inference after authored source changes without discarding Nx state. */
async function invokeCurrentImplementation(kind, args) {
  await testBootstrap;
  const revision = implementationRevision();
  if (revision === loadedRevision) return kind === "nodes" ? testCaseProjects(...args) : testCaseDependencies(...args);
  const url = new URL(import.meta.url);
  url.searchParams.set("revision", revision);
  const current = await import(url.href);
  if (current.default === plugin) throw new Error("The graph runtime retained stale ESM code; run inference through native Nx on Node.js");
  return kind === "nodes" ? current.default.createNodesV2[1](...args) : current.createDependencies(...args);
}

export function createDependencies(...args) {
  if (process.platform === "win32") return [];
  return invokeCurrentImplementation("dependencies", args);
}

/**
 * 🧩️ One Nx project configuration this plugin generates, as its consumers read it.
 * @typedef {{ targets: Record<string, { cache?: boolean, continuous?: boolean, dependsOn?: string[], outputs?: string[], inputs?: unknown[], namedInputs?: Record<string, unknown[]>, options?: Record<string, unknown>, metadata?: Record<string, unknown> }>, root?: string, name?: string, projectType?: string, tags?: string[], sourceRoot?: string }} GeneratedProject
 */

/**
 * 🧩️ The `createNodesV2` result: one entry per matched file, each carrying the projects it contributes.
 * @typedef {[string, { projects: Record<string, GeneratedProject> }][]} GeneratedNodes
 */

const plugin = {
  createDependencies,
  name: "@repo/test-cases",
  createNodesV2: /** @type {[string, (files: readonly string[], options: unknown, context: { workspaceRoot: string }) => Promise<GeneratedNodes>]} */ (["**/*.feature", (...args) => invokeCurrentImplementation("nodes", args)]),
};

export default plugin;

/** 🧪️ Exposed for the domain's own self-tests: the pure parts of the generation above. */
export const internals = { isExcluded, projectNameFor, inputsFor, taxonomy, rustSutCratePath, oracleContributionPaths };

/** 📁️ Convenience for tools that need the case directories without loading Nx. */
export function discoverCaseDirs(workspaceRoot) {
  const vocabulary = taxonomy(workspaceRoot);
  const featureFilename = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
  const found = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isSymbolicLink() || !entry.isDirectory()) continue;
      const abs = join(dir, entry.name);
      const rel = nxPath(relative(workspaceRoot, abs));
      if (isExcluded(vocabulary, rel)) continue;
      if (entry.name === vocabulary.testsDirName) {
        for (const child of readdirSync(abs, { withFileTypes: true })) {
          if (!child.isDirectory() || child.isSymbolicLink() || !canonicalCase(vocabulary, nxPath(relative(workspaceRoot, dir)), child.name)) continue;
          if (readdirSync(join(abs, child.name), { withFileTypes: true }).some((file) => file.name === featureFilename && file.isFile() && !file.isSymbolicLink())) found.push(nxPath(relative(workspaceRoot, join(abs, child.name))));
        }
        continue;
      }
      walk(abs);
    }
  };
  walk(workspaceRoot);
  return found.sort();
}
