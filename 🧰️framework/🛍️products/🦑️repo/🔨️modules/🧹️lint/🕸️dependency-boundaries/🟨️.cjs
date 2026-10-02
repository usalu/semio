const { resolve } = require("node:path");
const { loadDependencyDirectionPolicy } = require("../../📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs");

/** 🧱️ Dependency-cruiser uses the canonical current no-follow authority snapshot. */
module.exports = loadDependencyDirectionPolicy(resolve(__dirname, "../../../../../..")).policy;
