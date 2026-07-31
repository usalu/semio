import { readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const SCRIPT_BASENAME = "📜script.ts";

/** @param {string} absScript */
function scriptExportsPolicy(absScript) {
  const text = readFileSync(absScript, "utf8");
  return /\bexport\s+(const|function)\s+policy\b/.test(text);
}

/**
 * @param {string} workspaceRoot
 * @param {string} absScript
 */
function extraPolicyInputs(workspaceRoot, absScript) {
  const text = readFileSync(absScript, "utf8");
  const m = text.match(/export\s+const\s+policyFile\s*=\s*["']([^"']+)["']/);
  if (!m) return [];
  const targetAbs = join(dirname(absScript), m[1]);
  const tr = relative(workspaceRoot, targetAbs).split("\\").join("/");
  return [`{workspaceRoot}/${tr}`];
}

/**
 * @param {string[]} configFiles
 * @param {unknown} _options
 * @param {{ workspaceRoot: string }} context
 */
function policyScriptProjects(configFiles, _options, context) {
  const { workspaceRoot } = context;
  /** @param {string} p */
  const nxPath = (p) => p.split("\\").join("/");

  return configFiles
    .filter((configFile) => {
      if (configFile.includes("node_modules") || configFile.includes(".🦑repo") || configFile.includes("/dist/")) {
        return false;
      }
      const abs = join(workspaceRoot, configFile);
      return scriptExportsPolicy(abs);
    })
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      const rel = nxPath(relative(workspaceRoot, abs));
      const projectRoot = dirname(abs);
      const slug = rel.replace(/[^a-zA-Z0-9_-]+/g, "_");
      const projectName = `breach-${slug}`;
      const extraInputs = extraPolicyInputs(workspaceRoot, abs);
      return [
        configFile,
        {
          projects: {
            [projectName]: {
              name: projectName,
              root: nxPath(relative(workspaceRoot, projectRoot)) || ".",
              targets: {
                lint: {
                  executor: "nx:run-commands",
                  options: {
                    cwd: workspaceRoot,
                    command: `bun "${rel}" policy`,
                  },
                  inputs: [`{workspaceRoot}/${rel}`, ...extraInputs],
                  outputs: [`{workspaceRoot}/.🦑repo/⚡cache/breaches`],
                  cache: true,
                },
              },
            },
          },
        },
      ];
    });
}

function emojiProjectJsonNodes(configFiles, _options, context) {
  const { workspaceRoot } = context;
  /** @param {string} p */
  const nxPath = (p) => p.split("\\").join("/");

  return configFiles
    .filter((configFile) => {
      if (configFile.includes("node_modules") || configFile.includes(".🦑repo") || configFile.includes("/dist/")) {
        return false;
      }
      return true;
    })
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      const json = JSON.parse(readFileSync(abs, "utf8"));
      const name = json.name;
      if (!name) return null;
      const projectRoot = dirname(abs);
      const root = nxPath(relative(workspaceRoot, projectRoot)) || ".";
      return [
        configFile,
        {
          projects: {
            [name]: {
              ...json,
              name,
              root,
            },
          },
        },
      ];
    })
    .filter(Boolean);
}

export default {
  name: "@repo/policy-scripts-file",
  createNodesV2: [`**/${SCRIPT_BASENAME}`, policyScriptProjects],
};
