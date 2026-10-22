import { observedFs } from "../../../👁️observation/🟨️.mjs";
const {readFileSync}=observedFs;
import { dirname, join, relative } from "node:path";

const SCRIPT_BASENAME = "📜️script.ts";

/**
 * 🛡️ Reads a file, tolerating paths a workspace walker hands back with lossy U+FFFD
 * substitutions for multi-byte segments, which are unopenable and would
 * otherwise abort the whole project graph.
 * @param {string} absPath
 * @returns {string | null}
 */
function readTextOrNull(absPath) {
  try {
    return readFileSync(absPath, "utf8");
  } catch {
    return null;
  }
}

/** @param {string} absScript */
function scriptExportsPolicy(absScript) {
  const text = readTextOrNull(absScript);
  if (text === null) return false;
  return /\bexport\s+(?:(?:const|function)\s+policy\b|\{\s*policy\s*\}\s+from\b)/.test(text);
}

/**
 * @param {string} workspaceRoot
 * @param {string} absScript
 */
function extraPolicyInputs(workspaceRoot, absScript) {
  const text = readTextOrNull(absScript);
  const m = text?.match(/export\s+const\s+policyFile\s*=\s*["']([^"']+)["']/);
  if (!m) return [];
  const targetAbs = join(dirname(absScript), m[1]);
  const tr = relative(workspaceRoot, targetAbs).split("\\").join("/");
  return [`{workspaceRoot}/${tr}`];
}

/**
 * @param {string[]} configFiles
 * @param {unknown} _options
 * @param {{ workspaceRoot: string, projects?: Record<string,any> }} context
 */
function policyScriptProjects(configFiles, _options, context) {
  const { workspaceRoot } = context;
  /** @param {string} p */
  const buildPath = (p) => p.split("\\").join("/");

  return configFiles
    .filter((configFile) => {
      if (configFile.includes("node_modules") || configFile.includes(".🧬semio") || configFile.includes("/dist/")) {
        return false;
      }
      const abs = join(workspaceRoot, configFile);
      return scriptExportsPolicy(abs);
    })
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      const rel = buildPath(relative(workspaceRoot, abs));
      const projectRoot = dirname(abs);
      const slug = rel.replace(/[^a-zA-Z0-9_-]+/g, "_");
      const root=buildPath(relative(workspaceRoot,projectRoot))||".",owner=Object.values(context.projects??{}).find(project=>project.root===root),projectName=owner?.name??`breach-${slug}`,lint=owner?.targets?.lint;
      if(owner?.targets?.["lint-policy"])throw Error(`Policy target already declared: ${projectName}:lint-policy`);
      const extraInputs = extraPolicyInputs(workspaceRoot, abs);
      return [
        configFile,
        {
          projects: {
            [projectName]: {
              name: projectName,
              root,
              targets: {
                lint: {...(lint??{executor:"repo:noop"}),dependsOn:[...new Set([...(lint?.dependsOn??[]),"lint-policy"])]},
                "lint-policy": {
                  executor: "repo:script",
                  options: {
                    cwd: workspaceRoot,
                    command: `repo "${rel}" policy`,
                  },
                  inputs: [`{workspaceRoot}/${rel}`, ...extraInputs],
                  outputs: [`{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/breaches`],
                  cache: true,
                },
              },
            },
          },
        },
      ];
    });
}

export default {
  name: "@repo/policy-scripts-file",
  createNodesV2: [`**/${SCRIPT_BASENAME}`, policyScriptProjects],
};
