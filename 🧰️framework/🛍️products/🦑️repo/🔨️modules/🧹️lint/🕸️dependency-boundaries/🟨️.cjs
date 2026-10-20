const { resolve } = require("node:path");
const { loadDependencyDirectionPolicy } = require("@semio-tech/repo-lib/inventory/commonjs");

/** 🧱️ Dependency-cruiser uses the canonical current no-follow authority snapshot. */
module.exports = loadDependencyDirectionPolicy(resolve(__dirname, "../../../../../..")).policy;
