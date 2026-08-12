import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const PROJECT_BASENAME = "📋️project.json";
const SCRIPT_BASENAME = "📜️script.ts";
const DEFAULT_EXECUTOR = "nx:run-commands";
const TEST_TARGET = "test";
const TEST_LEVELS = ["quick", "long", "exhaustive"];

/** @type {Map<string, string>} Prefer the first non-lossy root per project name across createNodes batches. */
const rootsByName = new Map();

/**
 * 🎚️ Applies the workspace target convention: every target runs through
 * {@link DEFAULT_EXECUTOR}, and a project that owns a `📜️script.ts` runs from
 * its own root, so a `📋️project.json` only spells out what actually differs.
 * Projects without one keep nx's workspace-root default and thereby delegate to
 * the root `📜️script.ts`.
 * @param {Record<string, any>} target
 * @param {string} root
 * @param {boolean} ownsScript
 */
function targetWithDefaults(target, root, ownsScript) {
  const executor = target.executor ?? DEFAULT_EXECUTOR;
  if (executor !== DEFAULT_EXECUTOR) return { ...target, executor };
  if (!ownsScript) return { ...target, executor };
  return { ...target, executor, options: { cwd: root, ...(target.options ?? {}) } };
}

/**
 * 🧪 Derives the `test-quick` / `test-long` / `test-exhaustive` siblings from a
 * project's base `test` target, leaving any explicitly declared level untouched.
 * @param {Record<string, any>} targets
 */
function withLeveledTestTargets(targets) {
  const base = targets[TEST_TARGET];
  if (typeof base?.options?.command !== "string") return targets;
  const leveled = { ...targets };
  for (const level of TEST_LEVELS) {
    const name = `${TEST_TARGET}-${level}`;
    if (leveled[name]) continue;
    leveled[name] = { ...base, options: { ...base.options, command: `${base.options.command} ${level}` } };
  }
  return leveled;
}

/**
 * @param {Record<string, any>} json
 * @param {string} root
 * @param {string} projectDir
 */
function projectWithDefaults(json, root, projectDir) {
  const ownsScript = existsSync(join(projectDir, SCRIPT_BASENAME));
  const declared = json.targets ?? {};
  const normalized = {};
  for (const [name, target] of Object.entries(declared)) normalized[name] = targetWithDefaults(target, root, ownsScript);
  return { ...json, name: json.name, root, targets: withLeveledTestTargets(normalized) };
}

/**
 * @param {string[]} configFiles
 * @param {unknown} _options
 * @param {{ workspaceRoot: string }} context
 */
function emojiProjectJsonNodes(configFiles, _options, context) {
  const { workspaceRoot } = context;
  /** @param {string} p */
  const nxPath = (p) => p.split("\\").join("/");

  return configFiles
    .filter((configFile) => {
      // 🛡️ Nx's native walker sometimes hands back lossy paths with U+FFFD where a
      // multi-byte emoji used to be; those are unopenable duplicates of a real file.
      if (configFile.includes("\uFFFD")) return false;
      if (configFile.includes("node_modules") || configFile.includes(".🦑️repo") || configFile.includes("/dist/")) {
        return false;
      }
      return true;
    })
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      let json;
      try {
        json = JSON.parse(readFileSync(abs, "utf8"));
      } catch {
        return null;
      }
      const name = json.name;
      if (!name) return null;
      const projectDir = dirname(abs);
      const root = nxPath(relative(workspaceRoot, projectDir)).normalize("NFC") || ".";
      const prior = rootsByName.get(name);
      if (prior !== undefined && prior !== root) return null;
      if (prior === undefined) rootsByName.set(name, root);
      return [configFile, { projects: { [name]: projectWithDefaults(json, root, projectDir) } }];
    })
    .filter(Boolean);
}

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: [`**/${PROJECT_BASENAME}`, emojiProjectJsonNodes],
};
