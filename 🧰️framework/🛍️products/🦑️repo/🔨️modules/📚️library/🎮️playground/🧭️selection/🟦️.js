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
exports.loadFrameworkOsPlaygroundSelections = loadFrameworkOsPlaygroundSelections;
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../\uD83D\uDD0D\uFE0Fdiscovery/\uD83D\uDFE6\uFE0F.ts");
/** 🧭️ Resolves public selections from authored manifests before any generated output exists. */
function loadFrameworkOsPlaygroundSelections(repoRoot, manifestPaths) {
    var _a, _b, _c, _d, _e, _f, _g, _h;
    if (repoRoot === void 0) { repoRoot = (0, ____ts_1.getWorkspaceRoot)(); }
    var paths = manifestPaths !== null && manifestPaths !== void 0 ? manifestPaths : (0, ____ts_2.discoverCatalogPackages)(repoRoot, (0, ____ts_2.loadCatalogTaxonomy)()).filter(function (entry) { return entry.lang === "🦀️rust" && ["plugin", "extension"].includes(entry.role); }).map(function (entry) { return entry.manifestPath; });
    var selections = [], identities = new Set();
    for (var _i = 0, paths_1 = paths; _i < paths_1.length; _i++) {
        var path = paths_1[_i];
        var absolute = (0, node_path_1.resolve)(repoRoot, path), local = (0, node_path_1.relative)(repoRoot, absolute).replaceAll("\\", "/");
        if ((0, node_path_1.isAbsolute)(path) || local !== path || local.startsWith("../") || !local.endsWith("/Cargo.toml"))
            throw new Error("Playground manifest is outside its source owner: ".concat(path));
        for (var node = absolute; node !== (0, node_path_1.resolve)(repoRoot); node = (0, node_path_1.dirname)(node))
            if ((0, node_fs_1.lstatSync)(node).isSymbolicLink())
                throw new Error("Playground manifest source is a symlink: ".concat(path));
        var metadata = (_a = ____ts_2.cargoProviderTomlParser.parse((0, node_fs_1.readFileSync)(absolute, "utf8")).package) === null || _a === void 0 ? void 0 : _a.metadata;
        if (!((_b = metadata === null || metadata === void 0 ? void 0 : metadata.component) === null || _b === void 0 ? void 0 : _b.package) || !["plugin", "extension"].includes((_d = (_c = metadata.semio) === null || _c === void 0 ? void 0 : _c.role) !== null && _d !== void 0 ? _d : ""))
            continue;
        if (!/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/.test(metadata.component.package))
            throw new Error("Invalid playground component identity: ".concat(path));
        var _loop_1 = function (row) {
            if (typeof row.variant !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant))
                throw new Error("Invalid playground variant: ".concat(path));
            var aliases = (_g = row.aliases) !== null && _g !== void 0 ? _g : [];
            if (!Array.isArray(aliases) || aliases.some(function (alias) { return typeof alias !== "string" || !alias.trim(); }))
                throw new Error("Invalid playground aliases: ".concat(path));
            for (var _l = 0, _m = __spreadArray([row.variant], aliases.filter(function (alias) { return alias !== row.variant; }), true); _l < _m.length; _l++) {
                var identity = _m[_l];
                if (identities.has(identity))
                    throw new Error("Duplicate playground selection: ".concat(identity));
                identities.add(identity);
            }
            for (var _o = 0, _p = ["react", "wgpu"]; _o < _p.length; _o++) {
                var renderer = _p[_o];
                if (!Number.isSafeInteger((_h = row.ports) === null || _h === void 0 ? void 0 : _h[renderer]) || row.ports[renderer] < 1 || row.ports[renderer] > 65535)
                    throw new Error("Invalid playground ".concat(renderer, " port: ").concat(path));
            }
            selections.push({ variant: row.variant, aliases: aliases, ports: row.ports, pluginId: metadata.component.package.slice(6), cratePath: (0, node_path_1.dirname)(local).replaceAll("\\", "/") });
        };
        for (var _j = 0, _k = (_f = (_e = metadata.semio) === null || _e === void 0 ? void 0 : _e.playground) !== null && _f !== void 0 ? _f : []; _j < _k.length; _j++) {
            var row = _k[_j];
            _loop_1(row);
        }
    }
    return selections.sort(function (a, b) { return a.variant.localeCompare(b.variant); });
}
