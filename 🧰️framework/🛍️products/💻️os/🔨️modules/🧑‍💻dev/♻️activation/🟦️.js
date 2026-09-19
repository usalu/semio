"use strict";
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
exports.COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = exports.UNWATCHED_COMPONENT_SOURCE_DIRECTORIES = exports.PLAYGROUND_SESSION_VITE_SPECIFIER = exports.PLAYGROUND_SESSION_OUTPUT_ROOT_ENV = exports.PLAYGROUND_SESSION_ARTIFACT_KEY = exports.ACTIVATION_RECEIPT_FILE = void 0;
exports.playgroundSessionOutputPath = playgroundSessionOutputPath;
exports.playgroundSessionStagedOutputPath = playgroundSessionStagedOutputPath;
exports.playgroundSessionViteAlias = playgroundSessionViteAlias;
exports.parseActivationReceipt = parseActivationReceipt;
exports.nextActivationReceipt = nextActivationReceipt;
exports.readActivationReceipt = readActivationReceipt;
exports.developmentRuntimeRoot = developmentRuntimeRoot;
exports.pluginModulesRoot = pluginModulesRoot;
exports.pluginModulesRootIn = pluginModulesRootIn;
exports.publishActivationReceipt = publishActivationReceipt;
exports.observeActivationReceipts = observeActivationReceipts;
exports.stagedModuleVerdict = stagedModuleVerdict;
exports.stagedModuleReportLines = stagedModuleReportLines;
exports.newestComponentSourceMtime = newestComponentSourceMtime;
exports.stagedModuleMtime = stagedModuleMtime;
var node_fs_1 = require("node:fs");
var node_crypto_1 = require("node:crypto");
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
exports.ACTIVATION_RECEIPT_FILE = "🔣️receipt.json";
exports.PLAYGROUND_SESSION_ARTIFACT_KEY = "🎮️playground-session/🟦️.ts";
exports.PLAYGROUND_SESSION_OUTPUT_ROOT_ENV = "SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT";
exports.PLAYGROUND_SESSION_VITE_SPECIFIER = "virtual:semio-playground-session";
/** 🎮️ Resolves one semantic session source below an explicit generated-output root. */
function playgroundSessionOutputPath(outputRoot) {
    return node_path_1.join.apply(void 0, __spreadArray([outputRoot], exports.PLAYGROUND_SESSION_ARTIFACT_KEY.split("/"), false));
}
/** 🎮️ Resolves one variant source below an explicit staging root. */
function playgroundSessionStagedOutputPath(stagingRoot, variant) {
    return playgroundSessionOutputPath((0, node_path_1.join)(stagingRoot, variant));
}
/** 🎮️ Gives Vite and native tests the same pure virtual-session alias. */
function playgroundSessionViteAlias(stagingRoot, variant) {
    return { find: exports.PLAYGROUND_SESSION_VITE_SPECIFIER, replacement: playgroundSessionStagedOutputPath(stagingRoot, variant) };
}
var identity = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
var keys = function (value, expected) { return value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === __spreadArray([], expected, true).sort().join(); };
/** 🧾️ Validates the completed runtime receipt defined by {@link ./🧬️schema/🔣️.json} `#/$defs/DevActivationV1`. */
function parseActivationReceipt(value) {
    if (!keys(value, ["schema", "variant", "profile", "plugins"]) || value.schema !== "semio.dev.activation/v1" || typeof value.variant !== "string" || !identity.test(value.variant) || !["dev", "release"].includes(String(value.profile)) || !Array.isArray(value.plugins))
        throw new Error("Invalid activation receipt");
    var seen = new Set();
    for (var _i = 0, _a = value.plugins; _i < _a.length; _i++) {
        var row = _a[_i];
        if (!keys(row, ["pluginId", "artifactSha256", "rebuiltAt"]) || typeof row.pluginId !== "string" || !identity.test(row.pluginId) || typeof row.artifactSha256 !== "string" || !/^[a-f0-9]{64}$/.test(row.artifactSha256) || !Number.isSafeInteger(row.rebuiltAt) || Number(row.rebuiltAt) < 1)
            throw new Error("Invalid activation plugin");
        if (seen.has(row.pluginId))
            throw new Error("Duplicate activation plugin: ".concat(row.pluginId));
        seen.add(row.pluginId);
    }
    return value;
}
/** 🕰️ Keeps warm activations unchanged and advances changed content despite clock rollback. */
function nextActivationReceipt(variant, profile, completed, previous, now) {
    if (now === void 0) { now = Date.now(); }
    if (previous)
        parseActivationReceipt(previous);
    if (previous && (previous.variant !== variant || previous.profile !== profile))
        throw new Error("Activation receipt identity mismatch");
    if (!Number.isSafeInteger(now))
        throw new Error("Invalid activation clock");
    var prior = new Map(previous === null || previous === void 0 ? void 0 : previous.plugins.map(function (row) { return [row.pluginId, row]; }));
    var timestamp = Math.max.apply(Math, __spreadArray([now, 1], __spreadArray([], prior.values(), true).map(function (row) { return row.rebuiltAt + 1; }), false));
    return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant: variant, profile: profile, plugins: __spreadArray([], completed, true).sort(function (a, b) { return a.pluginId < b.pluginId ? -1 : a.pluginId > b.pluginId ? 1 : 0; }).map(function (row) { var _a; return (__assign(__assign({}, row), { rebuiltAt: ((_a = prior.get(row.pluginId)) === null || _a === void 0 ? void 0 : _a.artifactSha256) === row.artifactSha256 ? prior.get(row.pluginId).rebuiltAt : timestamp })); }) });
}
/** 📖️ Reads only explicit activation completion, never the existence or mtime of cached outputs. */
function readActivationReceipt(directory) {
    return parseActivationReceipt(JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.join)(directory, exports.ACTIVATION_RECEIPT_FILE), "utf8")));
}
/** 🗂️ Separates each renderer, variant and profile's activation and installations from compiled artifacts. */
function developmentRuntimeRoot(packageRoot, variant, profile, renderer) {
    if (!["react", "wgpu"].includes(renderer))
        throw new Error("Select a development renderer: react or wgpu");
    parseActivationReceipt({ schema: "semio.dev.activation/v1", variant: variant, profile: profile, plugins: [] });
    return (0, node_path_1.join)(packageRoot, "dist", "runtime", renderer, profile, variant);
}
/** 🔌️ THE staging root for a profile — the ONE directory every producer writes and every consumer
 * reads: `@semio-tech/framework-plugin-web:support-<profile>`, every crate's `materialize-<profile>`
 * and `@semio-tech/framework-os-dev:plugin` write it; the react Vite `/🔌️plugin-modules` mount, the
 * WGPU browser host, the native runner's `SEMIO_PLUGIN_MODULES`, `prepare`/`activate`
 * and the production distribution copy read it. Derived from this module's own location — never from a
 * workspace walk — so Vite's config bundler resolves it without pulling repository discovery in, and so
 * no caller can pick a second root. Two roots is what this function replaces: a `🧑‍💻dev/🔌️plugin-modules`
 * written only by the catalog builder while `materialize-*` wrote here drifted silently for two days
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-playground-boot-2026-09-12.md` §2.1). */
function pluginModulesRoot(profile) {
    return pluginModulesRootIn((0, node_path_1.resolve)((0, node_path_1.dirname)((0, node_url_1.fileURLToPath)(import.meta.url)), "../../../../../.."), profile);
}
/** 🗂️ The same staging root inside an EXPLICIT workspace — the one form a sandboxed consumer (the
 * production distribution copy, driven against a throwaway workspace in its own tests) may use. The
 * repository-relative path lives here once so no caller ever spells a second one. */
function pluginModulesRootIn(workspace, profile) {
    if (profile !== "dev" && profile !== "release")
        throw new Error("Select a plugin staging profile: dev or release");
    return (0, node_path_1.join)(workspace, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "📦️packages", "🟦️typescript", "dist", profile, "🔌️plugin-modules");
}
/** 📬️ Atomically announces completed preparation without rewriting a warm receipt. */
function publishActivationReceipt(directory, receipt) {
    var text = JSON.stringify(parseActivationReceipt(receipt)) + "\n", destination = (0, node_path_1.join)(directory, exports.ACTIVATION_RECEIPT_FILE);
    if ((0, node_fs_1.existsSync)(destination)) {
        if ((0, node_fs_1.lstatSync)(destination).isSymbolicLink())
            throw new Error("Invalid activation receipt path");
        var previous = readActivationReceipt(directory);
        if (previous.variant !== receipt.variant || previous.profile !== receipt.profile)
            throw new Error("Activation receipt identity mismatch");
        if (JSON.stringify(previous) + "\n" === text)
            return false;
    }
    (0, node_fs_1.mkdirSync)(directory, { recursive: true });
    var temporary = (0, node_path_1.join)(directory, ".receipt-".concat((0, node_crypto_1.randomUUID)(), ".stage"));
    try {
        (0, node_fs_1.writeFileSync)(temporary, text, { flag: "wx" });
        (0, node_fs_1.renameSync)(temporary, destination);
    }
    finally {
        (0, node_fs_1.rmSync)(temporary, { force: true });
    }
    return true;
}
/** 👀️ Observes atomic completion receipts and owns its filesystem subscription. */
function observeActivationReceipts(directory, listener, onError) {
    var current, serialized, closed = false;
    var refresh = function () {
        if (closed)
            return;
        var next = readActivationReceipt(directory), text = JSON.stringify(next);
        if (serialized === text)
            return;
        current = next;
        serialized = text;
        listener(next);
    };
    var watcher = (0, node_fs_1.watch)(directory, function () {
        try {
            refresh();
        }
        catch (error) {
            onError(error);
        }
    });
    watcher.on("error", onError);
    var close = function () { if (!closed) {
        closed = true;
        watcher.close();
    } };
    try {
        refresh();
    }
    catch (error) {
        close();
        throw error;
    }
    return { snapshot: function () { return current; }, close: close };
}
//#region 🔖️StagedModuleFreshness
/** 🗑️ Directory names inside a component's owner tree that hold BUILD OUTPUT, never the sources whose
 * mtime decides whether the staged module is behind. Walking them would make every crate permanently
 * "stale" the moment its own `dist/component-dev/*.wasm` lands. */
exports.UNWATCHED_COMPONENT_SOURCE_DIRECTORIES = ["dist", "target", "node_modules", "pkg", ".git"];
/** 🔒️ Bound on one component's source walk so a serve-start freshness pass over ~20 crates stays a
 * few milliseconds and can never be turned into an unbounded repository scan by a stray symlink. */
exports.COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20000;
/** 🕰️ Formats an epoch millisecond for a `[stale]` line — UTC ISO so two machines print the same text. */
function stagedInstant(value) {
    return new Date(Math.round(value)).toISOString();
}
/** 🔎️ Decides one staged component's freshness from already-collected facts — pure, so the serve-start
 * pass and the activation-receipt watcher share ONE rule and a fixture can drive every outcome.
 * Precedence is most-fundamental-first: nothing staged beats no receipt row, which beats an extension
 * that was materialized but never published, which beats sources newer than the staged bytes. */
function stagedModuleVerdict(facts) {
    var _a, _b;
    if (facts.stagedAtMs === undefined)
        return { pluginId: facts.pluginId, kind: "unstaged", detail: "no staged module directory" };
    if (facts.activationTracked) {
        if (facts.receiptArtifactSha256 === undefined)
            return { pluginId: facts.pluginId, kind: "unactivated", detail: "staged ".concat(stagedInstant(facts.stagedAtMs), " but absent from the activation receipt") };
        if (facts.role === "extension" && facts.installedPackageHash !== facts.receiptArtifactSha256) {
            return { pluginId: facts.pluginId, kind: "unpublished", detail: "installed ".concat((_a = facts.installedPackageHash) !== null && _a !== void 0 ? _a : "(nothing)", " \u2260 activated ").concat(facts.receiptArtifactSha256) };
        }
    }
    if (facts.newestSourceMs !== undefined && facts.newestSourceMs > facts.stagedAtMs) {
        return { pluginId: facts.pluginId, kind: "source-newer", detail: "staged ".concat(stagedInstant(facts.stagedAtMs), " < ").concat((_b = facts.newestSourcePath) !== null && _b !== void 0 ? _b : "source", " ").concat(stagedInstant(facts.newestSourceMs)) };
    }
    return { pluginId: facts.pluginId, kind: "fresh" };
}
/** 📣️ Renders one `[stale]` line per non-fresh component, each ending in the exact command that fixes
 * it — a served module that is behind its own crate must never be a silent no-op. */
function stagedModuleReportLines(verdicts, command) {
    return verdicts.filter(function (row) { return row.kind !== "fresh"; }).map(function (row) { return "[stale] ".concat(row.pluginId, ": ").concat(row.kind).concat(row.detail ? " \u2014 ".concat(row.detail) : "", " \u2014 run: ").concat(command); });
}
/** 📂️ One directory's entries, or none when it cannot be read — structurally typed so this module keeps
 * its node-builtin-only import surface (`⚙️vite.config.ts` bundles it on every dev-server boot). */
function readableDirectoryEntries(directory) {
    try {
        return (0, node_fs_1.readdirSync)(directory, { withFileTypes: true });
    }
    catch (_a) {
        return [];
    }
}
/** 🕰️ Newest regular-file mtime under one component's source tree, output directories excluded and the
 * walk bounded. `undefined` when the tree is absent or holds no readable source file. */
function newestComponentSourceMtime(sourceRoot, maximumEntries) {
    if (maximumEntries === void 0) { maximumEntries = exports.COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES; }
    if (!(0, node_fs_1.existsSync)(sourceRoot))
        return undefined;
    var newest, visited = 0;
    var pending = [sourceRoot];
    while (pending.length > 0) {
        var directory = pending.pop();
        for (var _i = 0, _a = readableDirectoryEntries(directory); _i < _a.length; _i++) {
            var entry = _a[_i];
            if (++visited > maximumEntries)
                return newest;
            if (entry.isSymbolicLink())
                continue;
            var path = (0, node_path_1.join)(directory, entry.name);
            if (entry.isDirectory()) {
                if (!exports.UNWATCHED_COMPONENT_SOURCE_DIRECTORIES.includes(entry.name))
                    pending.push(path);
                continue;
            }
            if (!entry.isFile())
                continue;
            var mtimeMs = void 0;
            try {
                mtimeMs = (0, node_fs_1.statSync)(path).mtimeMs;
            }
            catch (_b) {
                continue;
            }
            if (!newest || mtimeMs > newest.mtimeMs)
                newest = { mtimeMs: mtimeMs, path: path };
        }
    }
    return newest;
}
/** 🕰️ Newest regular-file mtime among one staged module directory's own files. */
function stagedModuleMtime(moduleDirectory) {
    if (!(0, node_fs_1.existsSync)(moduleDirectory))
        return undefined;
    var newest;
    for (var _i = 0, _a = readableDirectoryEntries(moduleDirectory); _i < _a.length; _i++) {
        var entry = _a[_i];
        if (!entry.isFile() || entry.name === ".nx-artifact.json")
            continue;
        try {
            var mtimeMs = (0, node_fs_1.statSync)((0, node_path_1.join)(moduleDirectory, entry.name)).mtimeMs;
            if (newest === undefined || mtimeMs > newest)
                newest = mtimeMs;
        }
        catch (_b) {
            continue;
        }
    }
    return newest;
}
//#endregion 🔖️StagedModuleFreshness
