// 🧪️ Nx plugin: one virtual project per discovered test case.
//
// Discovery is `**/🧪️tests/*/component.feature`. There are no hand-authored `📋️project.json` files
// for tests, so a case can never be silently omitted from a higher level, and `checkLeveledTestTargets`
// style scanners become unnecessary — the four level targets are generated, always, for every case.
//
// `compose/**` is excluded HERE, in the discovery library, not by a workflow path filter.

import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const TAXONOMY_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const DOMAIN_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";
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
  const inputs = [
    `{workspaceRoot}/${caseRel}/${vocabulary.testFeatureFilename}`,
    `{workspaceRoot}/${caseRel}/${vocabulary.testFixturesDirName}/**/*`,
    ...(existsSync(join(workspaceRoot, sharedFixtures)) ? [`{workspaceRoot}/${sharedFixtures}/**/*`] : []),
    ...adapters.map((adapter) => `{workspaceRoot}/${adapter}`),
    `{workspaceRoot}/${vocabulary.testOracleRegistryPath}`,
    `{workspaceRoot}/${vocabulary.testSchemaPath}`,
    `{workspaceRoot}/${TAXONOMY_REL}`,
    `{workspaceRoot}/${DOMAIN_REL}/📦️packages/**/*`,
    `{workspaceRoot}/${DOMAIN_REL}/🧬️protocol/**/*`,
    `{workspaceRoot}/${DOMAIN_REL}/🏃️runner/**/*`,
    `{workspaceRoot}/${DOMAIN_REL}/🔮️oracle/**/*`,
    `{workspaceRoot}/${DOMAIN_REL}/📜️script.ts`,
    "sharedGlobals",
  ];
  // 🧭️ A change to the owner's own sources must invalidate the case, or a subject regression would
  // be served from cache as a pass.
  inputs.push(`{workspaceRoot}/${ownerRel}/**/*`);
  return inputs;
}

/** 🎚️ One generated target routed through the testing domain's own router. */
function target(command, inputs, cacheable = true) {
  return {
    executor: "nx:run-commands",
    options: { cwd: DOMAIN_REL, command: `bun ./📜️script.ts ${command}`, forwardAllArgs: false },
    inputs,
    outputs: [`{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tests`],
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
              lint: target(`contract ${select}`, inputs),
              "test-contract": target(`contract ${select}`, inputs),
              "test-oracle": target(`oracle ${select}`, inputs),
              "test-subject": target(`subject ${select}`, inputs),
              "test-parity": target(`parity ${select}`, inputs),
              test: target(`run ${select}`, inputs),
              ...Object.fromEntries(LEVELS.map((level) => [`test-${level}`, target(`run ${level} ${select}`, inputs, level !== "exhaustive")])),
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
