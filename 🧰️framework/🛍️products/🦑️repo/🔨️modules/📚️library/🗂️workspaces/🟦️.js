"use strict";
//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js
// Physical package membership and explicit package payload ownership.
//#endregion 🧲️Header
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getWorkspaceRoot = getWorkspaceRoot;
exports.computeWorkspaces = computeWorkspaces;
exports.diffWorkspaces = diffWorkspaces;
//#region 🔌️Adapters
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
//#endregion 🔌️Adapters
//#region 🔎️WorkspaceRoot
/** 🔎️Uses Nx's execution workspace before standalone hints and workspace-manifest discovery. Owned here
 * rather than in the repository library barrel so a consumer that only needs the root path never pulls
 * the barrel (and its taxonomy discovery walk) into its module graph. */
function getWorkspaceRoot() {
    var _a, _b;
    var fromNx = (_a = process.env.NX_WORKSPACE_ROOT) === null || _a === void 0 ? void 0 : _a.trim();
    if (fromNx)
        return (0, node_path_1.resolve)(fromNx);
    var fromEnv = (_b = process.env.REPO_ROOT) === null || _b === void 0 ? void 0 : _b.trim();
    if (fromEnv)
        return (0, node_path_1.resolve)(fromEnv);
    var dir = process.cwd();
    for (var i = 0; i < 30; i++) {
        var pkg = (0, node_path_1.join)(dir, "package.json");
        if ((0, node_fs_1.existsSync)(pkg)) {
            try {
                var j = JSON.parse((0, node_fs_1.readFileSync)(pkg, "utf8"));
                if (j.name === "workspace")
                    return dir;
            }
            catch (_c) {
                /* ignore */
            }
        }
        var up = (0, node_path_1.dirname)(dir);
        if (up === dir)
            break;
        dir = up;
    }
    return process.cwd();
}
//#endregion 🔎️WorkspaceRoot
//#region 🔣️Constants
var MANIFEST_FILENAME = "package.json";
/** 🧺️ Directory names never descended into — build/vendor/scratch trees, never real workspace source.
 * Includes the schema-owned opaque `compose` boundary (same isolation as `DISCOVERY_SKIP_DIRS`) so
 * workspace generation cannot reintroduce its intentionally deleted memberships. */
var WORKSPACE_SCAN_SKIP_DIR_NAMES = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "storybook-static", "temp", "coverage", "🔌️plugin-modules", ".🧬semio", "compose"]);
function errorCode(error) {
    return typeof error === "object" && error !== null && "code" in error ? String(error.code) : undefined;
}
function nativeState(path) {
    try {
        var state = (0, node_fs_1.lstatSync)(path);
        if (state.isSymbolicLink())
            return "symlink";
        if (state.isDirectory())
            return "directory";
        if (state.isFile())
            return "file";
        return "other";
    }
    catch (error) {
        if (errorCode(error) === "ENOENT")
            return "missing";
        throw new Error("Workspace source is unreadable: ".concat(path), { cause: error });
    }
}
var NATIVE_DISCOVERY_OPERATIONS = {
    list: function (path) {
        try {
            return (0, node_fs_1.readdirSync)(path, { withFileTypes: true }).map(function (entry) { return ({
                kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other",
                name: entry.name,
            }); });
        }
        catch (error) {
            if (errorCode(error) === "ENOENT")
                return [];
            throw new Error("Workspace directory is unreadable: ".concat(path), { cause: error });
        }
    },
    readText: function (path) {
        try {
            return (0, node_fs_1.readFileSync)(path, "utf8");
        }
        catch (error) {
            throw new Error("Workspace manifest is unreadable: ".concat(path), { cause: error });
        }
    },
    state: nativeState,
};
function checkCancellation(options) {
    var _a;
    if ((_a = options.signal) === null || _a === void 0 ? void 0 : _a.aborted)
        throw new Error("Workspace discovery cancelled");
}
function readManifest(manifestPath, operations) {
    var state = operations.state(manifestPath);
    if (state === "missing")
        return {};
    if (state !== "file")
        throw new Error("Workspace manifest must be a regular file: ".concat(manifestPath, " (").concat(state, ")"));
    var source = operations.readText(manifestPath);
    var document;
    try {
        document = JSON.parse(source);
    }
    catch (error) {
        throw new Error("Workspace manifest is malformed: ".concat(manifestPath), { cause: error });
    }
    if (!document || typeof document !== "object" || Array.isArray(document))
        throw new Error("Workspace manifest must contain an object: ".concat(manifestPath));
    var manifest = document;
    return { name: typeof manifest.name === "string" ? manifest.name : undefined, exports: manifest.exports };
}
/** 🗺️ Discovers physical package manifests without language or output-directory assumptions. */
function walk(absDir, repoRoot, results, options, operations, progress) {
    var _a;
    checkCancellation(options);
    var entries = operations.list(absDir);
    progress.directoriesScanned += 1;
    (_a = options.onProgress) === null || _a === void 0 ? void 0 : _a.call(options, { candidatesDiscovered: results.length, directoriesScanned: progress.directoriesScanned, relativeDirectory: (0, node_path_1.relative)(repoRoot, absDir).replaceAll("\\", "/") });
    checkCancellation(options);
    for (var _i = 0, entries_1 = entries; _i < entries_1.length; _i++) {
        var entry = entries_1[_i];
        checkCancellation(options);
        if (entry.kind !== "directory" || entry.name.startsWith(".") || WORKSPACE_SCAN_SKIP_DIR_NAMES.has(entry.name))
            continue;
        var absChild = (0, node_path_1.join)(absDir, entry.name);
        var manifestPath = (0, node_path_1.join)(absChild, MANIFEST_FILENAME);
        if (operations.state(manifestPath) !== "missing") {
            results.push(__assign({ relDir: (0, node_path_1.relative)(repoRoot, absChild).replaceAll("\\", "/"), absDir: absChild }, readManifest(manifestPath, operations)));
        }
        walk(absChild, repoRoot, results, options, operations, progress);
    }
}
/** 📦️ Enumerates explicit export targets across package subpaths and conditions. */
function exportTargets(value, subpaths) {
    if (subpaths === void 0) { subpaths = true; }
    if (typeof value === "string")
        return [value];
    if (Array.isArray(value))
        return value.flatMap(function (entry) { return exportTargets(entry, false); });
    if (!value || typeof value !== "object")
        return [];
    var entries = Object.entries(value);
    if (entries.some(function (_a) {
        var key = _a[0];
        return key.startsWith(".");
    })) {
        if (!subpaths || entries.some(function (_a) {
            var key = _a[0];
            return key !== "." && (!key.startsWith("./") || key.includes("*"));
        }))
            return [];
        return entries.flatMap(function (_a) {
            var entry = _a[1];
            return exportTargets(entry, false);
        });
    }
    if (entries.some(function (_a) {
        var key = _a[0];
        return !key || /^\d+$/u.test(key);
    }))
        return [];
    var targets = [];
    for (var _i = 0, entries_2 = entries; _i < entries_2.length; _i++) {
        var _a = entries_2[_i], condition = _a[0], entry = _a[1];
        targets.push.apply(targets, exportTargets(entry, false));
        if (condition === "default")
            break;
    }
    return targets;
}
/** 🔗️ Binds a payload to its nearest package owner through a concrete physical export. */
function ownsPayload(owner, payload, operations) {
    if (!owner.name || owner.name !== payload.name)
        return false;
    var prefix = (0, node_path_1.relative)(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
    return exportTargets(owner.exports).some(function (target) {
        if (!target.startsWith("./") || /[\\:*?%#\u0000]/u.test(target))
            return false;
        var segments = target.slice(2).split("/");
        if (segments.some(function (segment) { return !segment || segment === "." || segment === ".." || segment === "node_modules"; }))
            return false;
        if (!segments.join("/").startsWith(prefix))
            return false;
        var path = owner.absDir;
        return segments.every(function (segment, index) {
            path = (0, node_path_1.join)(path, segment);
            return operations.state(path) === (index === segments.length - 1 ? "file" : "directory");
        });
    });
}
//#endregion 🔍️Scan
//#region 🏗️Generate
/** 🏗️ Emits each independent package once and rejects unbound duplicate identities. */
function computeWorkspaces(repoRoot, options) {
    var _a;
    if (options === void 0) { options = {}; }
    var operations = (_a = options.operations) !== null && _a !== void 0 ? _a : NATIVE_DISCOVERY_OPERATIONS;
    var rootState = operations.state(repoRoot);
    if (rootState !== "directory")
        throw new Error("Workspace root must be a physical directory: ".concat(repoRoot, " (").concat(rootState, ")"));
    var candidates = [];
    walk(repoRoot, repoRoot, candidates, options, operations, { directoriesScanned: 0 });
    var byDirectory = new Map(candidates.map(function (candidate) { return [candidate.absDir, candidate]; }));
    var results = candidates.filter(function (candidate) {
        checkCancellation(options);
        var parent = (0, node_path_1.dirname)(candidate.absDir);
        while (parent !== repoRoot && parent !== (0, node_path_1.dirname)(parent)) {
            var owner = byDirectory.get(parent);
            if (owner)
                return !ownsPayload(owner, candidate, operations);
            parent = (0, node_path_1.dirname)(parent);
        }
        return true;
    });
    var dirByName = new Map();
    for (var _i = 0, results_1 = results; _i < results_1.length; _i++) {
        var _b = results_1[_i], relDir = _b.relDir, name_1 = _b.name;
        if (!name_1)
            continue;
        var existing = dirByName.get(name_1);
        if (existing && existing !== relDir) {
            throw new Error("Workspace discovery: duplicate package name \"".concat(name_1, "\" at both \"").concat(existing, "\" and \"").concat(relDir, "\" \u2014 bun install would not resolve this unambiguously."));
        }
        dirByName.set(name_1, relDir);
    }
    return results.map(function (r) { return r.relDir; }).sort(function (a, b) { return a.localeCompare(b); });
}
/** 🔎️ Diagnostic split for `--check`: entries `computeWorkspaces` wants that root `package.json` is
 * missing, and entries root `package.json` still lists that no longer resolve to a real package. */
function diffWorkspaces(repoRoot, current, options) {
    if (options === void 0) { options = {}; }
    var expected = computeWorkspaces(repoRoot, options);
    var expectedSet = new Set(expected);
    var currentSet = new Set(current);
    return {
        expected: expected,
        missing: expected.filter(function (entry) { return !currentSet.has(entry); }),
        stale: current.filter(function (entry) { return !expectedSet.has(entry); }),
    };
}
//#endregion 🏗️Generate
