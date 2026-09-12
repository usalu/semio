// 🧪️ Nx plugin: one virtual project per discovered test case.
//
// Discovery is every taxonomy-declared kind-only feature beneath `🧪️tests/*`. There are no hand-authored `📋️project.json` files
// for tests, so a case can never be silently omitted from a higher level, and `checkLeveledTestTargets`
// style scanners become unnecessary — the four level targets are generated, always, for every case.
//
// The exclusion set, the case slug rule, the adapter filenames and the location of the testing
// domain are all TAXONOMY DATA (`🔣️taxonomy.json`). This plugin declares none of them, so marking
// another area exempt or relocating the domain is a vocabulary edit, never a code edit here.

import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { createHash } from "node:crypto";
const dependencyModule = new URL("./🕸️dependencies/🟨️.mjs", import.meta.url);
const dependencyRevision = createHash("sha256").update(readFileSync(dependencyModule)).digest("hex");
const { cargoPackage, localPackagePath, rustSubjectPackage, ownerContributions, packagesForOwner } = await import(`${dependencyModule.href}?revision=${dependencyRevision}`);
const implementationRevision = () => createHash("sha256").update(readFileSync(new URL(import.meta.url))).update(readFileSync(dependencyModule)).digest("hex");
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
  return JSON.parse(readFileSync(join(workspaceRoot, TAXONOMY_REL), "utf8"));
}

/** @param {string} p */
const nxPath = (p) => p.split("\\").join("/");

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
  const featureFilename = filenameForKind(vocabulary, vocabulary.testFeatureFileKindId);
  const results = [];
  const libraryModule = new URL("../📚️library/🟨️.mjs", import.meta.url);
  const libraryRevision = createHash("sha256").update(readFileSync(libraryModule)).digest("hex");
  const { cacheInternals: native } = await import(`${libraryModule.href}?revision=${libraryRevision}`);
  const policy = JSON.parse(readFileSync(join(workspaceRoot, dirname(TAXONOMY_REL), "⚡️caching/🔣️policy.json"), "utf8"));
  const state = { manifests: new Map(), closures: new Map(), names: new Map(), plans: new Map() };
  const javascript = policy.toolchains.javascript;
  let commandInputs;

  for (const configFile of configFiles) {
    if (configFile.includes("\uFFFD")) continue;
    const rel = nxPath(configFile);
    if (isExcluded(vocabulary, rel)) continue;
    const caseRel = dirname(rel);
    const testsRel = dirname(caseRel);
    if (basename(testsRel) !== vocabulary.testsDirName) continue;
    if (basename(rel) !== featureFilename) continue;
    const ownerRel = dirname(testsRel);
    const caseSlug = basename(caseRel);
    if (!canonicalCase(vocabulary, ownerRel, caseSlug)) continue;
    commandInputs ??= [...native.relativeScriptInputs([join(workspaceRoot, vocabulary.testDomainPath, "📜️script.ts")], workspaceRoot), ...javascript.files.map((path) => `{workspaceRoot}/${path}`), ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime }))];

    const adapters = [];
    for (const fileKindId of Object.values(vocabulary.testAdapterFileKinds)) {
      const filename = filenameForKind(vocabulary, fileKindId);
      const adapterRel = `${caseRel}/${filename}`;
      if (existsSync(join(workspaceRoot, adapterRel))) adapters.push(adapterRel);
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
    const sourceFile = `${project.root}/${filenameForKind(vocabulary, vocabulary.testFeatureFileKindId)}`;
    return [...targets].map((target) => {
      if (!context.projects[target]) throw new Error(`Test dependency has no Nx project: ${source} → ${target}`);
      return { source, target, sourceFile, type: "static" };
    });
  });
}

/** ♻️ Refreshes resident inference after authored source changes without discarding Nx state. */
async function invokeCurrentImplementation(kind, args) {
  const revision = implementationRevision();
  if (revision === loadedRevision) return kind === "nodes" ? testCaseProjects(...args) : testCaseDependencies(...args);
  const url = new URL(import.meta.url);
  url.searchParams.set("revision", revision);
  const current = await import(url.href);
  if (current.default === plugin) throw new Error("The graph runtime retained stale ESM code; run inference through native Nx on Node.js");
  return kind === "nodes" ? current.default.createNodesV2[1](...args) : current.createDependencies(...args);
}

export function createDependencies(...args) { return invokeCurrentImplementation("dependencies", args); }

const plugin = {
  createDependencies,
  name: "@repo/test-cases",
  createNodesV2: ["**/*.feature", (...args) => invokeCurrentImplementation("nodes", args)],
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
    for (const entry of readdirSync(dir)) {
      const abs = join(dir, entry);
      let stats;
      try {
        stats = statSync(abs);
      } catch {
        continue;
      }
      if (!stats.isDirectory()) continue;
      const rel = nxPath(relative(workspaceRoot, abs));
      if (isExcluded(vocabulary, rel) || entry === "node_modules" || entry === ".git") continue;
      if (entry === vocabulary.testsDirName) {
        for (const child of readdirSync(abs)) {
          if (canonicalCase(vocabulary, nxPath(relative(workspaceRoot, dir)), child) && existsSync(join(abs, child, featureFilename))) found.push(nxPath(relative(workspaceRoot, join(abs, child))));
        }
        continue;
      }
      walk(abs);
    }
  };
  walk(workspaceRoot);
  return found.sort();
}
