"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MODULE_HOT_SWAP_FILE = exports.MODULE_SHARD_DIRECTORY = exports.MODULE_VENDOR_DIRECTORY = exports.MODULE_BRIDGE_FILE = exports.MODULE_DIRECTORIES = exports.MODULE_EXTENSION_ROUTE = exports.MODULE_PLUGIN_ROUTE = exports.MODULE_ROUTES = void 0;
exports.parseModuleRoutes = parseModuleRoutes;
exports.moduleRoutePath = moduleRoutePath;
exports.parseModuleDirectories = parseModuleDirectories;
exports.moduleStaticDirectoryNames = moduleStaticDirectoryNames;
exports.moduleDirectoryName = moduleDirectoryName;
exports.moduleIdForDirectoryName = moduleIdForDirectoryName;
var ___catalog_json_1 = require("./\uD83D\uDDFA\uFE0Fcatalog.json");
var ____json_1 = require("./\uD83E\uDDEC\uFE0Fschema/\uD83D\uDD23\uFE0F.json");
var ___routes_json_1 = require("./\uD83D\uDEE3\uFE0Froutes.json");
var ____ts_1 = require("../../../\uD83E\uDDE9\uFE0Fextension/\uD83D\uDFE6\uFE0F.ts");
var schema = ____json_1.default.$defs.DeploymentCatalogV1;
var idSpec = schema.properties.modules.items.properties.pluginId;
var idPattern = new RegExp(idSpec.pattern, "u");
/** 🛣️Admits only the two explicitly selected distribution route owners. */
function parseModuleRoutes(input) {
    if (!input || typeof input !== "object" || Array.isArray(input))
        throw new Error("Invalid module routes");
    var value = input, properties = ____json_1.default.$defs.DeploymentModuleRoutesV1.properties;
    if (Object.keys(value).sort().join(",") !== "extension,plugin" || value.plugin !== properties.plugin.const || value.extension !== properties.extension.const)
        throw new Error("Module routes must match their exact schema authority");
    return Object.freeze({ plugin: value.plugin, extension: value.extension });
}
exports.MODULE_ROUTES = parseModuleRoutes(___routes_json_1.default);
exports.MODULE_PLUGIN_ROUTE = exports.MODULE_ROUTES.plugin;
exports.MODULE_EXTENSION_ROUTE = exports.MODULE_ROUTES.extension;
/** 🚏️Decodes one canonical module request path without accepting obsolete routes or traversal aliases. */
function moduleRoutePath(rawUrl) {
    var _a;
    var encoded = (_a = rawUrl.split(/[?#]/, 1)[0]) !== null && _a !== void 0 ? _a : "";
    if (/%(?:2f|5c)/iu.test(encoded))
        return null;
    var path;
    try {
        path = decodeURIComponent(encoded);
    }
    catch (_b) {
        return null;
    }
    if (/[\\\u0000-\u001F\u007F]/u.test(path) || path.split("/").slice(1).some(function (part) { return part === "" || part === "." || part === ".."; }))
        return null;
    return Object.values(exports.MODULE_ROUTES).some(function (route) { return path === route || path.startsWith(route + "/"); }) ? path : null;
}
/** 📦️Validates the hand-authored deployment authority without deriving any name from an ID. */
function parseModuleDirectories(input) {
    if (!input || typeof input !== "object" || Array.isArray(input))
        throw new Error("Invalid module deployment catalog");
    var value = input;
    if (Object.keys(value).sort().join(",") !== "modules,version" || value.version !== 1 || !Array.isArray(value.modules) || value.modules.length < 1 || value.modules.length > 256)
        throw new Error("Invalid module deployment catalog fields");
    var ids = new Set(), emojis = new Set();
    return Object.freeze(value.modules.map(function (entry) {
        if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).sort().join(",") !== "directoryName,pluginId")
            throw new Error("Invalid module deployment row");
        if (typeof entry.pluginId !== "string" || entry.pluginId.length > idSpec.maxLength || !idPattern.test(entry.pluginId))
            throw new Error("Invalid public module identity");
        var emoji = (0, ____ts_1.installationDirectoryEmoji)(entry.directoryName);
        if (ids.has(entry.pluginId) || emojis.has(emoji))
            throw new Error("Duplicate module identity or sibling emoji");
        ids.add(entry.pluginId);
        emojis.add(emoji);
        return Object.freeze({ pluginId: entry.pluginId, directoryName: entry.directoryName });
    }));
}
exports.MODULE_DIRECTORIES = parseModuleDirectories(___catalog_json_1.default);
exports.MODULE_BRIDGE_FILE = "🌉️bridge.js";
exports.MODULE_VENDOR_DIRECTORY = "🪞️vendor";
exports.MODULE_SHARD_DIRECTORY = "🧵️shard";
exports.MODULE_HOT_SWAP_FILE = "♻️hot-swap.json";
/** 🚚️Selects only declared physical module directories for a production copy. */
function moduleStaticDirectoryNames(pluginId, hostMode) {
    var directoryName = moduleDirectoryName(pluginId);
    return hostMode ? undefined : [exports.MODULE_VENDOR_DIRECTORY, exports.MODULE_SHARD_DIRECTORY, directoryName];
}
/** 🧭️Resolves only an explicitly declared physical directory for a public plugin ID. */
function moduleDirectoryName(pluginId) {
    var row = exports.MODULE_DIRECTORIES.find(function (entry) { return entry.pluginId === pluginId; });
    if (!row)
        throw new Error("No hand-authored module directory for ".concat(JSON.stringify(pluginId)));
    return row.directoryName;
}
/** 🔎️Maps a materialized basename back to its declared public identity. */
function moduleIdForDirectoryName(directoryName) {
    var _a;
    return (_a = exports.MODULE_DIRECTORIES.find(function (entry) { return entry.directoryName === directoryName; })) === null || _a === void 0 ? void 0 : _a.pluginId;
}
