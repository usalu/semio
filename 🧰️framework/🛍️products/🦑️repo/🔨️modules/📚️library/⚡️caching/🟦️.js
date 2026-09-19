"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.repoCacheDirectory = repoCacheDirectory;
exports.isGeneratedPath = isGeneratedPath;
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
var generatedDirectories;
/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`). */
function repoCacheDirectory(repoRoot) {
    var segments = [];
    for (var _i = 1; _i < arguments.length; _i++) {
        segments[_i - 1] = arguments[_i];
    }
    return node_path_1.join.apply(void 0, __spreadArray([repoRoot, ".🧬semio", "🦑️repo", "⚡️cache"], segments, false));
}
/** 🗑️ True when a path lies inside a policy-declared generated directory (`dist`, `🗑️generated`, …): disposable output that never feeds source inputs. */
function isGeneratedPath(path) {
    generatedDirectories !== null && generatedDirectories !== void 0 ? generatedDirectories : (generatedDirectories = new Set(JSON.parse((0, node_fs_1.readFileSync)((0, node_url_1.fileURLToPath)(new URL("./🔣️policy.json", import.meta.url)), "utf8")).generatedDirectories));
    return path.split(/[\\/]/u).some(function (segment) { return generatedDirectories.has(segment); });
}
