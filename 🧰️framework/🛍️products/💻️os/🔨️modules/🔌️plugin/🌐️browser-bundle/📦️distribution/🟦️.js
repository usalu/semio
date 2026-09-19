"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.copyBrowserArtifacts = copyBrowserArtifacts;
var node_fs_1 = require("node:fs");
var promises_1 = require("node:fs/promises");
var node_path_1 = require("node:path");
var ____json_1 = require("../\uD83E\uDDEC\uFE0Fschema/\uD83D\uDD23\uFE0F.json");
var ____ts_1 = require("../\uD83D\uDD78\uFE0Fimports/\uD83D\uDFE6\uFE0F.ts");
var pathPattern = new RegExp(____json_1.default.$defs.BrowserArtifactDistributionV1.properties.files.items.pattern, "u");
/** 🛣️ Admits relative artifact paths with one canonical separator representation. */
function artifactPath(value) {
    if (typeof value !== "string" || !pathPattern.test(value))
        throw new Error("Invalid browser artifact path: ".concat(String(value)));
    return value.replaceAll("\\", "/");
}
/** 🗂️ Rejects symlink parents before reading or writing owned artifact trees. */
function regularParents(path) {
    return __awaiter(this, void 0, void 0, function () {
        var current, error_1;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    current = (0, node_path_1.resolve)(path);
                    _a.label = 1;
                case 1:
                    if (!((0, node_path_1.dirname)(current) !== current)) return [3 /*break*/, 6];
                    _a.label = 2;
                case 2:
                    _a.trys.push([2, 4, , 5]);
                    return [4 /*yield*/, (0, promises_1.lstat)(current)];
                case 3:
                    if ((_a.sent()).isSymbolicLink())
                        throw new Error("Symlink browser artifact path: ".concat(current));
                    return [3 /*break*/, 5];
                case 4:
                    error_1 = _a.sent();
                    if (error_1.code !== "ENOENT")
                        throw error_1;
                    return [3 /*break*/, 5];
                case 5:
                    current = (0, node_path_1.dirname)(current);
                    return [3 /*break*/, 1];
                case 6: return [2 /*return*/];
            }
        });
    });
}
/** 📦️ Copies declared immutable browser files into a caller-owned staging directory. */
function copyBrowserArtifacts(outputRoot_1, sources_1) {
    return __awaiter(this, arguments, void 0, function (outputRoot, sources, options) {
        var files, destinations, _i, sources_2, source, destination, manifestPath, stat, marker, _a, _b, _c, _d, value, name_1, input, output, key, error_2, shimPrefix, _e, files_1, file, _f, _g, _h;
        var _j, _k, _l, _m, _o;
        if (options === void 0) { options = {}; }
        return __generator(this, function (_p) {
            switch (_p.label) {
                case 0:
                    (_j = options.signal) === null || _j === void 0 ? void 0 : _j.throwIfAborted();
                    return [4 /*yield*/, regularParents(outputRoot)];
                case 1:
                    _p.sent();
                    files = [], destinations = new Set();
                    _i = 0, sources_2 = sources;
                    _p.label = 2;
                case 2:
                    if (!(_i < sources_2.length)) return [3 /*break*/, 16];
                    source = sources_2[_i];
                    (_k = options.signal) === null || _k === void 0 ? void 0 : _k.throwIfAborted();
                    destination = artifactPath(source.destination), manifestPath = (0, node_path_1.join)(source.root, ".nx-artifact.json");
                    return [4 /*yield*/, regularParents(manifestPath)];
                case 3:
                    _p.sent();
                    return [4 /*yield*/, (0, promises_1.lstat)(manifestPath)];
                case 4:
                    stat = _p.sent();
                    if (!stat.isFile() || stat.size > 1024 * 1024)
                        throw new Error("Invalid browser artifact manifest: ".concat(manifestPath));
                    _b = (_a = JSON).parse;
                    return [4 /*yield*/, (0, promises_1.readFile)(manifestPath, "utf8")];
                case 5:
                    marker = _b.apply(_a, [_p.sent()]);
                    if (Object.keys(marker).sort().join() !== "files,owner,version" || marker.version !== 1 || !source.owner || marker.owner !== source.owner)
                        throw new Error("Browser artifact owner mismatch: ".concat(source.root));
                    if (!Array.isArray(marker.files) || marker.files.length === 0)
                        throw new Error("Missing browser artifact paths: ".concat(source.root));
                    _c = 0, _d = marker.files;
                    _p.label = 6;
                case 6:
                    if (!(_c < _d.length)) return [3 /*break*/, 15];
                    value = _d[_c];
                    name_1 = artifactPath(value), input = (0, node_path_1.join)(source.root, name_1), output = (0, node_path_1.join)(outputRoot, destination, name_1), key = (0, node_path_1.resolve)(output);
                    if (destinations.has(key))
                        throw new Error("Browser artifact path collision: ".concat(key));
                    destinations.add(key);
                    return [4 /*yield*/, regularParents(input)];
                case 7:
                    _p.sent();
                    return [4 /*yield*/, regularParents(output)];
                case 8:
                    _p.sent();
                    return [4 /*yield*/, (0, promises_1.lstat)(input)];
                case 9:
                    if (!(_p.sent()).isFile())
                        throw new Error("Invalid browser artifact file: ".concat(input));
                    _p.label = 10;
                case 10:
                    _p.trys.push([10, 12, , 13]);
                    return [4 /*yield*/, (0, promises_1.lstat)(output)];
                case 11:
                    _p.sent();
                    throw new Error("Browser artifact destination exists: ".concat(output));
                case 12:
                    error_2 = _p.sent();
                    if (error_2.code !== "ENOENT")
                        throw error_2;
                    return [3 /*break*/, 13];
                case 13:
                    shimPrefix = source.shimDirectory === undefined ? undefined : (0, node_path_1.relative)((0, node_path_1.dirname)(output), (0, node_path_1.join)(outputRoot, artifactPath(source.shimDirectory))).replaceAll("\\", "/") + "/";
                    files.push({ source: input, output: output, name: destination + "/" + name_1, shimPrefix: shimPrefix });
                    _p.label = 14;
                case 14:
                    _c++;
                    return [3 /*break*/, 6];
                case 15:
                    _i++;
                    return [3 /*break*/, 2];
                case 16:
                    _e = 0, files_1 = files;
                    _p.label = 17;
                case 17:
                    if (!(_e < files_1.length)) return [3 /*break*/, 25];
                    file = files_1[_e];
                    (_l = options.signal) === null || _l === void 0 ? void 0 : _l.throwIfAborted();
                    return [4 /*yield*/, (0, promises_1.mkdir)((0, node_path_1.dirname)(file.output), { recursive: true })];
                case 18:
                    _p.sent();
                    if (!(file.shimPrefix !== undefined && file.source.endsWith(".js"))) return [3 /*break*/, 21];
                    _f = promises_1.writeFile;
                    _g = [file.output];
                    _h = ____ts_1.rewritePreview2ShimImportSource;
                    return [4 /*yield*/, (0, promises_1.readFile)(file.source, "utf8")];
                case 19: return [4 /*yield*/, _f.apply(void 0, _g.concat([_h.apply(void 0, [_p.sent(), file.shimPrefix]), { flag: "wx", signal: options.signal }]))];
                case 20:
                    _p.sent();
                    return [3 /*break*/, 23];
                case 21: return [4 /*yield*/, (0, promises_1.copyFile)(file.source, file.output, node_fs_1.constants.COPYFILE_EXCL)];
                case 22:
                    _p.sent();
                    _p.label = 23;
                case 23:
                    (_m = options.progress) === null || _m === void 0 ? void 0 : _m.call(options, file.name);
                    _p.label = 24;
                case 24:
                    _e++;
                    return [3 /*break*/, 17];
                case 25:
                    (_o = options.signal) === null || _o === void 0 ? void 0 : _o.throwIfAborted();
                    return [2 /*return*/, files.length];
            }
        });
    });
}
