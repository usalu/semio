"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MESH_DELIVERY_CATALOG = void 0;
exports.parseMeshDeliveryCatalog = parseMeshDeliveryCatalog;
exports.resolveMeshAsset = resolveMeshAsset;
exports.meshAssetTransportUrl = meshAssetTransportUrl;
var ___catalog_json_1 = require("./\uD83D\uDCC7\uFE0Fcatalog.json");
var ___catalog_json_2 = require("../\uD83C\uDF31\uFE0Fmetabolism/\uD83C\uDFA8\uFE0Frepresentation/\uD83D\uDCC7\uFE0Fcatalog.json");
function object(value, keys) {
    if (!value || typeof value !== "object" || Array.isArray(value))
        throw new Error("Mesh catalog object required");
    var row = value;
    if (Object.keys(row).length !== keys.length || keys.some(function (key) { return !(key in row); }))
        throw new Error("Mesh catalog fields do not match the schema");
    return row;
}
function path(value, extension) {
    if (extension === void 0) { extension = ""; }
    if (typeof value !== "string" || !value.endsWith(extension))
        throw new Error("Mesh catalog path required");
    var stem = extension ? value.slice(0, -extension.length) : value;
    if (!stem || stem.split("/").some(function (part) { return !part || /[.\\%?#\u0000-\u001f]/u.test(part); }))
        throw new Error("Unsafe mesh catalog path: ".concat(value));
    return value;
}
function publicUrl(value) {
    if (typeof value !== "string" || !value.startsWith("/mesh/"))
        throw new Error("Mesh public URL required");
    var leaf = path(value.slice("/mesh/".length), ".glb");
    if (leaf.includes("/"))
        throw new Error("Mesh public identity must be a single explicit URL key");
    return value;
}
function rows(value) {
    if (!Array.isArray(value))
        throw new Error("Mesh catalog rows required");
    return value;
}
/** 🧭️ Resolves schema-owned public identities to explicit source and delivery paths without aliases. */
function parseMeshDeliveryCatalog(input, readCatalog) {
    var authority = object(input, ["$schema", "version", "collections", "entries"]);
    if (authority.version !== 1 || typeof authority.$schema !== "string")
        throw new Error("Unsupported mesh delivery schema");
    var result = [];
    var urls = new Set();
    var sources = new Set();
    var paths = new Set();
    var catalogs = new Set();
    var admit = function (entry) {
        if (urls.has(entry.url) || sources.has(entry.source) || paths.has(entry.path))
            throw new Error("Duplicate mesh identity: ".concat(entry.url));
        urls.add(entry.url);
        sources.add(entry.source);
        paths.add(entry.path);
        result.push(Object.freeze(entry));
    };
    for (var _i = 0, _a = rows(authority.collections); _i < _a.length; _i++) {
        var value = _a[_i];
        var collection = object(value, ["catalog", "root", "output"]);
        var catalogPath = path(collection.catalog, ".json");
        if (catalogs.has(catalogPath))
            throw new Error("Duplicate mesh source catalog: ".concat(catalogPath));
        catalogs.add(catalogPath);
        var root = path(collection.root);
        var output = path(collection.output);
        var source = object(readCatalog(catalogPath), ["$schema", "version", "entries"]);
        if (source.version !== 1 || typeof source.$schema !== "string" || rows(source.entries).length === 0)
            throw new Error("Unsupported mesh source schema");
        for (var _b = 0, _c = rows(source.entries); _b < _c.length; _b++) {
            var value_1 = _c[_b];
            var entry = object(value_1, ["url", "path"]);
            var leaf = path(entry.path, ".glb");
            admit({ url: publicUrl(entry.url), source: "".concat(root, "/").concat(leaf), path: "".concat(output, "/").concat(leaf) });
        }
    }
    for (var _d = 0, _e = rows(authority.entries); _d < _e.length; _d++) {
        var value = _e[_d];
        var entry = object(value, ["url", "source", "path"]);
        admit({ url: publicUrl(entry.url), source: path(entry.source, ".glb"), path: path(entry.path, ".glb") });
    }
    return Object.freeze(result);
}
exports.MESH_DELIVERY_CATALOG = parseMeshDeliveryCatalog(___catalog_json_1.default, function (path) {
    if (path === "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json")
        return ___catalog_json_2.default;
    throw new Error("Unknown mesh source catalog: ".concat(path));
});
var indexes = new WeakMap();
/** 🔎️ Unknown and corrupted public mesh IDs are errors, never filename fallbacks. */
function resolveMeshAsset(url, catalog) {
    if (catalog === void 0) { catalog = exports.MESH_DELIVERY_CATALOG; }
    var index = indexes.get(catalog);
    if (!index) {
        index = new Map(catalog.map(function (entry) { return [entry.url, entry]; }));
        indexes.set(catalog, index);
    }
    var entry = index.get(url);
    if (!entry)
        throw new Error("Unknown mesh asset: ".concat(url));
    return entry;
}
/** 🌐️ Rewrites only the mesh namespace at the transport boundary; other asset domains retain ownership. */
function meshAssetTransportUrl(url, catalog) {
    if (catalog === void 0) { catalog = exports.MESH_DELIVERY_CATALOG; }
    return url.startsWith("/mesh/") ? "/mesh/".concat(resolveMeshAsset(url, catalog).path) : url;
}
