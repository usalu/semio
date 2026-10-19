const { resolve } = require("node:path");
const { loadDependencyDirectionPolicy } = require("../../📚️library/📦️packages/🟦️typescript/📇️inventory/🟨️.cjs");

/** 🧱️ Dependency-cruiser uses the canonical current no-follow authority snapshot. */
module.exports = loadDependencyDirectionPolicy(resolve(__dirname, "../../../../../..")).policy;
