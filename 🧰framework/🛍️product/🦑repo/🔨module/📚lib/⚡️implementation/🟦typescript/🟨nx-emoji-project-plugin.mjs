import { readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const PROJECT_BASENAME = "📋project.json";

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
  name: "@repo/emoji-project-json",
  createNodesV2: [`**/${PROJECT_BASENAME}`, emojiProjectJsonNodes],
};
