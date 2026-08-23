// 🧪️ Nx plugin: one virtual project per discovered test case.
//
// Discovery is `**/🧪️tests/*/component.feature`. There are no hand-authored `📋️project.json` files
// for tests, so a case can never be silently omitted from a higher level, and `checkLeveledTestTargets`
// style scanners become unnecessary — the four level targets are generated, always, for every case.
//
// The exclusion set, the case slug rule, the adapter filenames and the location of the testing
// domain are all TAXONOMY DATA (`🔣️taxonomy.json`). This plugin declares none of them, so marking
// another area exempt or relocating the domain is a vocabulary edit, never a code edit here.

import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const TAXONOMY_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const LEVELS = ["quick", "long", "exhaustive"];

/** 🔣️ Reads the frozen test vocabulary; the plugin never re-declares taxonomy strings. */
function taxonomy(workspaceRoot) {
  return JSON.parse(readFileSync(join(workspaceRoot, TAXONOMY_REL), "utf8"));
}

/** @param {string} p */
const nxPath = (p) => p.split("\\").join("/");

/** 🚫️ The hard discovery exclusion — `compose/` first among them. */
function isExcluded(vocabulary, relPath) {
  return vocabulary.testExcludedPathPrefixes.some((prefix) => relPath === prefix.replace(/\/$/, "") || relPath.startsWith(prefix) || relPath.includes(`/${prefix}`));
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

/** 📥️ Cache inputs of one case: the feature, its fixtures, its adapters, the claimed sources, the contract. */
function inputsFor(workspaceRoot, vocabulary, ownerRel, caseRel, adapters) {
  const sharedFixtures = `${ownerRel}/${vocabulary.testFixturesDirName}`;
  const domain = vocabulary.testDomainPath;
  const inputs = [
    `{workspaceRoot}/${caseRel}/${vocabulary.testFeatureFilename}`,
    `{workspaceRoot}/${caseRel}/${vocabulary.testFixturesDirName}/**/*`,
    ...(existsSync(join(workspaceRoot, sharedFixtures)) ? [`{workspaceRoot}/${sharedFixtures}/**/*`] : []),
    ...adapters.map((adapter) => `{workspaceRoot}/${adapter}`),
    `{workspaceRoot}/${vocabulary.testOracleRegistryPath}`,
    `{workspaceRoot}/${vocabulary.testSchemaPath}`,
    `{workspaceRoot}/${TAXONOMY_REL}`,
    // 🧩️Whatever the platform itself is made of, wherever the taxonomy says it lives.
    `{workspaceRoot}/${domain}/**/*`,
    // 🧩️And every owner contribution, so adding or changing an oracle invalidates the cases that use it.
    `{workspaceRoot}/**/${vocabulary.testContributionDirName}/**/*`,
    "sharedGlobals",
  ];
  // 🧭️ A change to the owner's own sources must invalidate the case, or a subject regression would
  // be served from cache as a pass. Fixture directories are excluded here because the two globs
  // above already cover them: a real-world fixture is megabytes, and Nx hashes file CONTENT, so
  // counting it twice per target doubles the hashing cost of every case that owns one.
  inputs.push(`{workspaceRoot}/${ownerRel}/**/*`);
  inputs.push(`!{workspaceRoot}/${ownerRel}/**/${vocabulary.testFixturesDirName}/**/*`);
  return inputs;
}

/** 🎚️ One generated target routed through the testing domain's own router. */
function target(domain, command, inputs, cacheable = true) {
  return {
    executor: "nx:run-commands",
    options: { cwd: domain, command: `bun ./📜️script.ts ${command}`, forwardAllArgs: false },
    inputs,
    // 📤️Only the durable products of a run are cache outputs. The work directory holds each case's
    // mutable fixture copies, which are large, regenerated on every run and meaningless to restore.
    outputs: ["results", "reports", "diffs"].map((child) => `{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tests/${child}`),
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
  const results = [];

  for (const configFile of configFiles) {
    if (configFile.includes("\uFFFD")) continue;
    const rel = nxPath(configFile);
    if (isExcluded(vocabulary, rel)) continue;
    const caseRel = dirname(rel);
    const testsRel = dirname(caseRel);
    if (basename(testsRel) !== vocabulary.testsDirName) continue;
    const ownerRel = dirname(testsRel);
    const caseSlug = basename(caseRel);
    if (!new RegExp(vocabulary.testCaseSlugPattern).test(caseSlug)) continue;

    const adapters = [];
    for (const filename of Object.values(vocabulary.testAdapterFilenames)) {
      const adapterRel = `${caseRel}/${filename}`;
      if (existsSync(join(workspaceRoot, adapterRel))) adapters.push(adapterRel);
    }

    const name = projectNameFor(ownerRel, caseSlug, await ownerHash(ownerRel));
    const inputs = inputsFor(workspaceRoot, vocabulary, ownerRel, caseRel, adapters);
    const domain = vocabulary.testDomainPath;
    const select = `--owner ${JSON.stringify(ownerRel)} --case ${caseSlug}`;

    results.push([
      configFile,
      {
        projects: {
          [name]: {
            name,
            root: caseRel,
            projectType: "application",
            tags: ["type:test", `owner:${ownerRel}`, ...adapters.map((adapter) => `impl:${basename(adapter)}`)],
            targets: {
              lint: target(domain, `contract ${select}`, inputs),
              "test-contract": target(domain, `contract ${select}`, inputs),
              "test-oracle": target(domain, `oracle ${select}`, inputs),
              "test-subject": target(domain, `subject ${select}`, inputs),
              "test-parity": target(domain, `parity ${select}`, inputs),
              test: target(domain, `run ${select}`, inputs),
              ...Object.fromEntries(LEVELS.map((level) => [`test-${level}`, target(domain, `run ${level} ${select}`, inputs, level !== "exhaustive")])),
            },
          },
        },
      },
    ]);
  }

  return results;
}

export default {
  name: "@repo/test-cases",
  createNodesV2: ["**/component.feature", testCaseProjects],
};

/** 🧪️ Exposed for the domain's own self-tests: the pure parts of the generation above. */
export const internals = { isExcluded, projectNameFor, inputsFor, taxonomy };

/** 📁️ Convenience for tools that need the case directories without loading Nx. */
export function discoverCaseDirs(workspaceRoot) {
  const vocabulary = taxonomy(workspaceRoot);
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
          if (existsSync(join(abs, child, vocabulary.testFeatureFilename))) found.push(nxPath(relative(workspaceRoot, join(abs, child))));
        }
        continue;
      }
      walk(abs);
    }
  };
  walk(workspaceRoot);
  return found.sort();
}
