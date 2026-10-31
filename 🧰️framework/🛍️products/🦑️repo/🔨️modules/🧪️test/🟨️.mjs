import{staleImplementationV1}from"../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
// 🧪️ Nx plugin: one virtual project per test OWNER (the directory holding a `🧪️tests` directory of discovered cases).
//
// Discovery is every taxonomy-declared kind-only feature beneath `🧪️tests/*`. There are no hand-authored `📋️project.json` files
// for tests, so a case can never be silently omitted. An owner project carries exactly two targets: `lint` (the contract phase
// over every case) and `test`, whose level, facet and case are ARGUMENTS (`repo run <owner>:test -- long --facet oracle --case x`),
// never separate targets; Nx caches every argument combination on its own.
//
// The exclusion set, the case slug rule, the adapter filenames and the location of the testing
// domain are all TAXONOMY DATA (`🔣️taxonomy.json`). This plugin declares none of them, so marking
// another area exempt or relocating the domain is a vocabulary edit, never a code edit here.

import { observeDependencies, observedFs, observeInference } from "../📚️library/📦️packages/🟦️typescript/🏗️build/🟨️.mjs";
import { targetBuildModes } from "../📚️library/📦️packages/🟦️typescript/🏗️build/🟨️.mjs";
import {repositoryBuildGraphDataDirectoryV1,repositoryBuildGraphWriterDirectoryV1,withRepositoryBuildGraphFactStoreV1} from "../📚️library/📦️packages/🟦️typescript/🏗️build/🟨️.mjs";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const { existsSync, lstatSync, readFileSync, readdirSync } = observedFs;
const nxPath = (p) => p.split("\\").join("/");
const dependencyModule = new URL("./🕸️dependencies/🟨️.mjs", import.meta.url);
const inputModule = new URL("../📚️library/🏗️build/📥️inputs/🟨️.mjs", import.meta.url);
/** 🧲️ Loads the inference producer: a literal import in a captured closure so the capture reaches it, the plain URL otherwise. */
const loadInferenceModuleV1 = () => import.meta.semioCapturedPhysicalSources ? import("../📚️library/📦️packages/🟦️typescript/🏗️build/🕸️graph/📥️inference/🟨️.mjs") : import(new URL("../📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs", import.meta.url).href);
const dependencyRevision = createHash("sha256").update(readAuthoritySource(dependencyModule)).digest("hex");
const inputRevision = createHash("sha256").update(readAuthoritySource(inputModule)).digest("hex");
let cargoPackage, localPackagePath, rustSubjectPackage, ownerContributions, packagesForOwner, compactSourceInputs;
const testBootstrap = Promise.all(import.meta.semioCapturedPhysicalSources ? [import("./🕸️dependencies/🟨️.mjs"), import("../📚️library/📦️packages/🟦️typescript/🏗️build/📥️inputs/🟨️.mjs")] : [import(`${dependencyModule.href}?revision=${dependencyRevision}`), import(`${inputModule.href}?revision=${inputRevision}`)]).then(([deps, inputs]) => {
  cargoPackage = deps.cargoPackage;
  localPackagePath = deps.localPackagePath;
  rustSubjectPackage = deps.rustSubjectPackage;
  ownerContributions = deps.ownerContributions;
  packagesForOwner = deps.packagesForOwner;
  compactSourceInputs = inputs.compactSourceInputs;
});
const implementationRevision = () => createHash("sha256").update(readAuthoritySource(new URL(import.meta.url))).update(readAuthoritySource(dependencyModule)).update(readAuthoritySource(inputModule)).digest("hex");
const loadedRevision = implementationRevision();

const TAXONOMY_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const CASE_SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 🏷️ Resolves a single presented emoji and the taxonomy-defined case identifier. */
function canonicalCase(vocabulary, owner, name) {
  const first = CASE_SEGMENTER.segment(name)[Symbol.iterator]().next().value?.segment ?? "";
  const fold = value => value.normalize("NFC").replaceAll("\uFE0E", "").replaceAll("\uFE0F", "");
  return name === name.normalize("NFC") && /[\p{Extended_Pictographic}\p{Emoji_Presentation}\u20E3]/u.test(first)
    && (/\p{Emoji_Presentation}/u.test([...first][0] ?? "") || first.includes("\uFE0F"))
    && !vocabulary.pathEmojiPolicy.genericEmojiIdentities.some(emoji => fold(emoji) === fold(first))
    && new RegExp(vocabulary.testCaseSlugPattern, "u").test(name.slice(first.length))
    && name !== vocabulary.testFixturesDirName
    && !owner.split("/").some((segment, index, parts) => (segment === vocabulary.testsDirName || segment === vocabulary.testFixturesDirName) && parts[index - 1] !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName || vocabulary.testDeliveryScopeDirectoryNames.includes(segment));
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
    return basename(path) === filename && basename(testsPath) === vocabulary.testsDirName && basename(dirname(testsPath)) !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName && canonicalCase(vocabulary, dirname(testsPath), basename(casePath));
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

/** 🏷️ The deterministic project name of one test owner: its path slug and the coordinator's owner-path hash. */
function projectNameFor(ownerRel, hash) {
  const slug = ownerRel
    .split("/")
    .map((segment) => segment.replace(/[^a-zA-Z0-9]+/g, "").toLowerCase())
    .filter(Boolean)
    .join("-");
  return `test-${slug || "root"}-${hash}`;
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

/** 🏷️ The build project identity of one native dependency root, remembered in the discovery state. */
function projectNameOfRoot(workspaceRoot, root, state) {
  if (!state.names.has(root)) {
    const authored = join(workspaceRoot, root, "📋️project.json");
    const name = existsSync(authored) ? JSON.parse(readFileSync(authored, "utf8")).name : cargoPackage(workspaceRoot, root)?.name;
    if (!name) throw new Error(`Test dependency has no build project identity: ${root}`);
    state.names.set(root, name);
  }
  return state.names.get(root);
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
  const projects = dependencyRoots.map((root) => projectNameOfRoot(workspaceRoot, root, state)).sort();
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
    ...adapters.map((adapter) => `{workspaceRoot}/${dirname(adapter)}/**/${basename(adapter)}`),
    `{workspaceRoot}/${locationPath(vocabulary, vocabulary.testOracleRegistryLocation)}`,
    `{workspaceRoot}/${locationPath(vocabulary, vocabulary.testSchemaLocation)}`,
    `{workspaceRoot}/${TAXONOMY_REL}`,
    `{workspaceRoot}/${domain}/**/*`,
    `{workspaceRoot}/**/${vocabulary.testOraclesDirName}/**/*`,
    ...(rustSutCrate === null ? [] : [`{workspaceRoot}/${rustSutCrate}/**/*`]),
    ...oracleContributionPaths(workspaceRoot, vocabulary, ownerRel).map((path) => `{workspaceRoot}/${path}/**/*`),
    "sharedGlobals",
  ];
  inputs.push(`{workspaceRoot}/${ownerRel}/**/*`);
  inputs.push(`!{workspaceRoot}/${ownerRel}/**/${vocabulary.testFixturesDirName}/**/*`);
  return inputs;
}

/** 🎚️ One generated target routed through the testing domain's own router. */
function target(domain, command, inputs, cacheable = true, scope, forwardAllArgs = false) {
  return targetBuildModes(scope.split("/").at(-1), {
    executor: "repo:script",
    options: { cwd: domain, command: `repo ./📜️script.ts ${command}`, forwardAllArgs, env: { SEMIO_TEST_OUTPUT_SCOPE: scope } },
    inputs,
    configurations: Object.fromEntries(["prototype", "release"].map(mode => [mode, { env: { SEMIO_TEST_OUTPUT_SCOPE: `${scope}/${mode}` } }])),
    outputs: ["results", "reports", "diffs"].map((child) => `{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tests/tasks/${scope}/{options.env.SEMIO_BUILD_MODE}/${child}`),
    cache: cacheable,
  });
}

/** 🧬️ The inputs every test project shares: the command closure of the testing domain's router and the toolchain files, environment and runtimes of the JavaScript policy. */
async function commandInputsOf(context, vocabulary, native, javascript, createScriptInputCache) {
  const { workspaceRoot } = context;
  return commandInputsFrom(await native.relativeScriptInputs([join(workspaceRoot, vocabulary.testDomainPath, "📜️script.ts")], workspaceRoot, createScriptInputCache(context)), javascript, workspaceRoot);
}

/** 🧬️ The shared inputs of the test projects from the roster of the command walk of the testing domain. */
function commandInputsFrom(roster, javascript, workspaceRoot) {
  return compactSourceInputs([...roster, ...javascript.files.map((path) => `{workspaceRoot}/${path}`), ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime }))], workspaceRoot);
}

/** 🧬️ Computes what every test owner shares once, so a discovery of single owners neither repeats it nor hides the facts it read: the caller captures the observations of this call and holds them for every owner it discovers afterwards. The command walk of the testing domain (about 20 thousand rows, the same for every owner) is not part of those observations: `walk` carries the rows that support it and the derived fact that stands for it, so an owner holds one row for it and an edit of a file of the walk that keeps its answer leaves every test owner valid. */
async function testShared(context) {
  const { workspaceRoot } = context;
  const vocabulary = taxonomy(workspaceRoot);
  const authority = workspaceAdmission(workspaceRoot);
  const policyPath = `${dirname(TAXONOMY_REL)}/⚡️caching/🔣️policy.json`;
  if (!authority.file(policyPath)) throw new Error("Test discovery requires real no-follow cache policy authority");
  const policy = JSON.parse(readFileSync(join(workspaceRoot, policyPath), "utf8"));
  const { cacheInternals: native, libraryBootstrap, createScriptInputCache, resolveCommandWalkV1 } = await (await loadInferenceModuleV1()).loadCurrentImplementationV1();
  await libraryBootstrap;
  const walk = await resolveCommandWalkV1(workspaceRoot, [join(workspaceRoot, vocabulary.testDomainPath, "📜️script.ts")], createScriptInputCache({ ...context, lateClosures: true }));
  const commandInputs = commandInputsFrom(walk.value, policy.toolchains.javascript, workspaceRoot);
  const state = { manifests: new Map(), closures: new Map(), names: new Map() }, host = `${vocabulary.testDomainPath}/📦️packages/🦀️rust`;
  if (existsSync(join(workspaceRoot, host))) {
    for (const root of native.nativeDependencyRoots(host, workspaceRoot, false, state.manifests, state.closures)) projectNameOfRoot(workspaceRoot, root, state);
    native.nativePreparation(host, workspaceRoot, vocabulary.generatorContracts, false, state.manifests, state.closures);
  }
  return Object.freeze({ commandInputs, state: Object.freeze(state), walk: Object.freeze({ rows: walk.rows, derived: walk.derived }) });
}

/**
 * 🕸️ Generates one project per test owner: `lint` runs the contract phase over every case, `test` runs the owner's cases with the level, facet and case
 * selection passed as arguments. Its inputs are the union of every case's inputs, so changing one case invalidates its owner's cached runs.
 */
async function testCaseProjects(configFiles, _options, context) {
  const { workspaceRoot } = context;
  const shared = context.testShared;
  const vocabulary = taxonomy(workspaceRoot);
  const authority = workspaceAdmission(workspaceRoot);
  const featureCandidates = candidateFiles(configFiles, vocabulary, authority);
  const results = [];
  const policyPath = `${dirname(TAXONOMY_REL)}/⚡️caching/🔣️policy.json`;
  if (!authority.file(policyPath)) throw new Error("Test discovery requires real no-follow cache policy authority");
  const policy = JSON.parse(readFileSync(join(workspaceRoot, policyPath), "utf8"));
  if (featureCandidates.length === 0) return results;
  const { cacheInternals: native, libraryBootstrap, createScriptInputCache } = await (await loadInferenceModuleV1()).loadCurrentImplementationV1();
  await libraryBootstrap;
  const state = shared ? { manifests: new Map(shared.state.manifests), closures: new Map(shared.state.closures), names: new Map(shared.state.names), plans: new Map() } : { manifests: new Map(), closures: new Map(), names: new Map(), plans: new Map() };
  const javascript = policy.toolchains.javascript;
  const owners = new Map();
  let commandInputs = shared?.commandInputs;

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
    if (!canonicalCase(vocabulary, ownerRel, basename(caseRel))) continue;
    const adapters = [];
    for (const fileKindId of Object.values(vocabulary.testAdapterFileKinds)) {
      const adapterRel = `${caseRel}/${filenameForKind(vocabulary, fileKindId)}`;
      if (authority.file(adapterRel)) adapters.push(adapterRel);
    }
    const owner = owners.get(ownerRel) ?? { configFile, testsRel, cases: [] };
    owner.cases.push({ caseRel, adapters });
    owners.set(ownerRel, owner);
  }

  for (const [ownerRel, owner] of [...owners].sort(([left], [right]) => Buffer.compare(Buffer.from(left), Buffer.from(right)))) {
    commandInputs ??= await commandInputsOf(context, vocabulary, native, javascript, createScriptInputCache);
    const name = projectNameFor(ownerRel, await ownerHash(ownerRel));
    const adapters = [...new Set(owner.cases.flatMap((row) => row.adapters))].sort();
    const inputs = [...new Map(owner.cases.flatMap((row) => inputsFor(workspaceRoot, vocabulary, ownerRel, row.caseRel, row.adapters)).map((input) => [JSON.stringify(input), input])).values()];
    const domain = vocabulary.testDomainPath;
    const scoped = (phase, command, forwardAllArgs) => {
      const nativeInputs = nativeInputsFor(workspaceRoot, vocabulary, ownerRel, adapters, phase, native, policy, state);
      return { ...target(domain, command, compactSourceInputs([...inputs, "testCommandSources", ...nativeInputs.inputs, { env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }], workspaceRoot), true, `${name}/${phase}`, forwardAllArgs), dependsOn: nativeInputs.dependsOn };
    };
    results.push([
      owner.configFile,
      {
        projects: {
          [name]: {
            name,
            root: owner.testsRel,
            projectType: "application",
            namedInputs: { testCommandSources: commandInputs },
            tags: ["type:test", `owner:${ownerRel}`, ...[...new Set(adapters.map((adapter) => basename(adapter)))].sort().map((adapter) => `impl:${adapter}`)],
            metadata: { semio: { testCases: owner.cases.map((row) => basename(row.caseRel)).sort() } },
            targets: {
              lint: scoped("lint", `contract --owner-root ${JSON.stringify(ownerRel)}`, false),
              test: scoped("test", `cases ${JSON.stringify(ownerRel)}`, true),
            },
          },
        },
      },
    ]);
  }

  return results;
}

/**
 * 🚧️ Names the test projects whose `nativeSources` input names a project the given inventory does not hold, with the missing names in code point order.
 * Test inference reads project names from the live disk while the inventory is the snapshot of one graph run, so a project created or removed during the run is a dangling name here, never a reason to abort the graph.
 */
export function danglingTestDependenciesV1(projects) {
  return Object.entries(projects).flatMap(([source, project]) => {
    if (!project.tags?.includes("type:test")) return [];
    const names = new Set(Object.values(project.targets ?? {}).flatMap((target) => (target.inputs ?? []).flatMap((input) => input.input === "nativeSources" ? input.projects : [])));
    const missing = [...names].filter((name) => !Object.hasOwn(projects, name)).sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
    return missing.length ? [{ source, root: project.root, missing }] : [];
  });
}

/** 🕸️ Makes native source changes select their inferred consumer cases in Nx affected runs; a name the inventory does not hold yields no edge, the portable assembly withholds that test project through {@link danglingTestDependenciesV1}. */
function testCaseDependencies(_options, context) {
  const vocabulary = taxonomy(context.workspaceRoot);
  return Object.entries(context.projects).flatMap(([source, project]) => {
    if (!project.tags?.includes("type:test")) return [];
    const targets = new Set(Object.values(project.targets ?? {}).flatMap((target) => (target.inputs ?? []).flatMap((input) => input.input === "nativeSources" ? input.projects : [])).filter((target) => Object.hasOwn(context.projects, target)));
    const featureName = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
    const preferred = `${project.root}/${featureName}`;
    const files = context.fileMap?.projectFileMap?.[source];
    const sourceFile = files?.find((entry) => entry.file === preferred)?.file
      ?? files?.find((entry) => entry.file.endsWith(`/${featureName}`))?.file
      ?? preferred;
    if (!existsSync(join(context.workspaceRoot, sourceFile))) return [];
    return [...targets].map((target) => ({ source, target, sourceFile, type: "static" }));
  });
}

/** 🗂️ Names test inference within the current declared graph fact store. */
export function testBuildInferenceDirectoryV1(workspace){return repositoryBuildGraphDataDirectoryV1(workspace,"inference");}

/** ♻️ Refreshes resident inference after authored source changes without discarding build state. */
async function withTestInferenceWriterV1(context,body){workspaceAdmission(context.workspaceRoot);const run=(invocation,phaseControl)=>withRepositoryBuildGraphFactStoreV1(context.workspaceRoot,invocation,phaseControl,(_store,actualControl)=>body(invocation,actualControl));return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(process.env,invocation=>run(invocation,createIssuedResourceControlV1(invocation,"graph:test-inference-entry",0)));}

async function invokeCurrentImplementation(kind,args){const index=kind==="nodes"?2:1,context=args[index];return withTestInferenceWriterV1(context,(invocation,phaseControl)=>{const original=[...args];original[index]={...context,invocation,phaseControl};return invokeCurrentImplementationV1(kind,original);});}

async function invokeCurrentImplementationV1(kind, args) {
  await testBootstrap;
  const revision = implementationRevision();
  if (revision === loadedRevision) {
    if (kind !== "nodes") return observeDependencies({ receiptPath: join(repositoryBuildGraphWriterDirectoryV1(resolve(args[1].workspaceRoot),"inference"), "test-cases.receipt.json") }, () => testCaseDependencies(...args));
    const workspaceRoot = resolve(args[2].workspaceRoot), libraryRevision = (await loadInferenceModuleV1()).currentImplementationRevisionV1();
    return observeInference({ cachePath: join(repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"inference"), "test-cases.json"), key: [revision, libraryRevision, args[0], args[1] ?? null, workspaceRoot] }, () => testCaseProjects(...args));
  }
  const url = new URL(import.meta.url);
  url.searchParams.set("revision", revision);
  const current = await import(url.href);
  if (current.default === plugin) throw staleImplementationV1("The build graph runtime retained stale source code; restart the owned build invocation");
  return kind === "nodes" ? current.default.createNodesV2[1](...args) : current.createDependencies(...args);
}

export function createDependencies(options, context) {
  return invokeCurrentImplementation("dependencies", [options, context]);
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
export const internals = { isExcluded, projectNameFor, inputsFor, taxonomy, rustSutCratePath, oracleContributionPaths, target };

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
      if (entry.name === vocabulary.testFixturesDirName && basename(dir) !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName) continue;
      if (entry.name === vocabulary.testsDirName && basename(dir) !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName) {
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

/** 🏗️ Supplies portable test-owner projects without acquiring a task-runner runtime. */
export async function discoverTestBuildProjectsV1(files,options,context) { return withTestInferenceWriterV1(context,async(invocation,phaseControl)=>{await testBootstrap;return testCaseProjects(files,options,{...context,invocation,phaseControl});}); }

/** 🧬️ Computes the inputs all test owners share, to be passed back as `context.testShared` when single owners are discovered; the observations of this call belong to every one of them. */
export async function discoverTestSharedV1(context) { return withTestInferenceWriterV1(context,async(invocation,phaseControl)=>{await testBootstrap;return testShared({...context,invocation,phaseControl});}); }

/** 🪢️ Supplies test-owner dependency edges from the portable project inventory. */
export async function discoverTestBuildDependenciesV1(options,context) { return withTestInferenceWriterV1(context,async(invocation,phaseControl)=>{await testBootstrap;return testCaseDependencies(options,{...context,invocation,phaseControl});}); }
