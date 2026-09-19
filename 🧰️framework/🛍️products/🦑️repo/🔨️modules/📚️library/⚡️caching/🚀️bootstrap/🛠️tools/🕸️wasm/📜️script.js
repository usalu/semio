"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
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
var __asyncValues = (this && this.__asyncValues) || function (o) {
    if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
    var m = o[Symbol.asyncIterator], i;
    return m ? m.call(o) : (o = typeof __values === "function" ? __values(o) : o[Symbol.iterator](), i = {}, verb("next"), verb("throw"), verb("return"), i[Symbol.asyncIterator] = function () { return this; }, i);
    function verb(n) { i[n] = o[n] && function (v) { return new Promise(function (resolve, reject) { v = o[n](v), settle(resolve, reject, v.done, v.value); }); }; }
    function settle(resolve, reject, d, v) { Promise.resolve(v).then(function(v) { resolve({ value: v, done: d }); }, reject); }
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
exports.binaryenDistribution = binaryenDistribution;
exports.binaryenIdentity = binaryenIdentity;
exports.binaryenMembers = binaryenMembers;
exports.binaryenDirectory = binaryenDirectory;
exports.preparedBinaryen = preparedBinaryen;
exports.prepareBinaryen = prepareBinaryen;
var node_crypto_1 = require("node:crypto");
var node_fs_1 = require("node:fs");
var promises_1 = require("node:fs/promises");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../../../\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDED\uFE0Frouting/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../\uD83D\uDD12\uFE0Fleases/\uD83D\uDFE6\uFE0F.ts");
var ___script_ts_1 = require("../../\uD83D\uDCE6\uFE0Fdependencies/\uD83D\uDCDC\uFE0Fscript.ts");
var ____json_1 = require("./\uD83D\uDD23\uFE0F.json");
var manifest = ____json_1.default;
var store = ".🧬semio/🦑️repo/⚡️cache/tools/binaryen", owner = "workspace:deps-wasm-opt";
var digest = function (path) { return (0, node_crypto_1.createHash)("sha256").update((0, node_fs_1.readFileSync)(path)).digest("hex"); };
/** 🧭️ Selects a published, checksum-pinned native distribution. */
function binaryenDistribution(platform, architecture) {
    if (platform === void 0) { platform = process.platform; }
    if (architecture === void 0) { architecture = process.arch; }
    var row = manifest.platforms.find(function (row) { return row.platform === platform && row.architecture === architecture; });
    if (!row)
        throw new Error("Unsupported Binaryen host: ".concat(platform, "/").concat(architecture));
    return row;
}
/** 🪪️ Hashes the intended toolchain independently of installation state. */
function binaryenIdentity(platform, architecture) {
    if (platform === void 0) { platform = process.platform; }
    if (architecture === void 0) { architecture = process.arch; }
    return "binaryen:".concat(manifest.version, ":").concat(platform, "/").concat(architecture, ":").concat(binaryenDistribution(platform, architecture).sha256);
}
/** 📦️ Retains the optimizer and its dynamic libraries while rejecting nonportable archive paths. */
function binaryenMembers(members, platform) {
    var prefix = "binaryen-version_".concat(manifest.version, "/"), selected = [];
    for (var _i = 0, members_1 = members; _i < members_1.length; _i++) {
        var member = members_1[_i];
        if (!member.startsWith(prefix) || /[\\:\x00-\x1f]/.test(member) || member.replace(/\/$/, "").split("/").some(function (part) { return !part || part === "." || part === ".."; }))
            throw new Error("Invalid Binaryen archive member: ".concat(member));
        var path = member.slice(prefix.length);
        if (path === "bin/wasm-opt".concat(platform === "win32" ? ".exe" : "") || /^(?:bin|lib)\/[a-zA-Z0-9_.+-]+(?:\.dylib|\.dll|\.so(?:\.[0-9]+)*)$/.test(path))
            selected.push(path);
    }
    if (new Set(selected).size !== selected.length)
        throw new Error("Invalid duplicate Binaryen archive members");
    return selected;
}
/** 🛣️ Keeps immutable native tools outside compiler and task-result stores. */
function binaryenDirectory(workspace) {
    var row = binaryenDistribution();
    return (0, node_path_1.join)(workspace, store, manifest.version, "".concat(row.platform, "-").concat(row.architecture, "-").concat(row.sha256));
}
/** 🔒️ Rejects substituted directories before reading or publishing tool state. */
function regularDirectory(path) {
    for (var current = (0, node_path_1.resolve)(path);; current = (0, node_path_1.dirname)(current)) {
        try {
            var stat = (0, node_fs_1.lstatSync)(current);
            if (!stat.isDirectory() || stat.isSymbolicLink())
                throw new Error("Invalid Binaryen directory: ".concat(current));
        }
        catch (error) {
            if (error.code !== "ENOENT")
                throw error;
        }
        if ((0, node_path_1.dirname)(current) === current)
            return;
    }
}
/** 🔐️ Checks the optimizer and every required library against the verified archive receipt. */
function preparedBinaryen(workspace) {
    var _a;
    var directory = binaryenDirectory(workspace), executable = "bin/wasm-opt".concat(process.platform === "win32" ? ".exe" : "");
    regularDirectory(directory);
    var receipt = JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.join)(directory, ".toolchain.json"), "utf8"));
    if (receipt.owner !== owner || receipt.identity !== binaryenIdentity() || !((_a = receipt.files) === null || _a === void 0 ? void 0 : _a[executable]))
        throw new Error("Binaryen installation identity mismatch");
    var names = Object.keys(receipt.files);
    if (binaryenMembers(names.map(function (path) { return "binaryen-version_".concat(manifest.version, "/").concat(path); }), process.platform).length !== names.length)
        throw new Error("Invalid Binaryen receipt paths");
    for (var _i = 0, names_1 = names; _i < names_1.length; _i++) {
        var name_1 = names_1[_i];
        var path = (0, node_path_1.join)(directory, name_1);
        regularDirectory((0, node_path_1.dirname)(path));
        var stat = (0, node_fs_1.lstatSync)(path);
        if (!stat.isFile() || stat.isSymbolicLink() || digest(path) !== receipt.files[name_1])
            throw new Error("Binaryen installation checksum mismatch: ".concat(name_1));
    }
    return (0, node_path_1.join)(directory, executable);
}
/** 📥️ Acquires one pinned optimizer in an isolated directory and publishes it atomically. */
function prepareBinaryen(workspace, signal) {
    return __awaiter(this, void 0, void 0, function () {
        var row, directory, parent;
        var _this = this;
        return __generator(this, function (_a) {
            signal.throwIfAborted();
            row = binaryenDistribution(), directory = binaryenDirectory(workspace), parent = (0, node_path_1.dirname)(directory);
            regularDirectory(parent);
            (0, node_fs_1.mkdirSync)(parent, { recursive: true });
            return [2 /*return*/, (0, ____ts_3.withResourceLeases)({ directory: (0, node_path_1.join)(workspace, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), signal: signal, resources: [{ resource: binaryenIdentity(), mode: "exclusive" }] }, function () { return __awaiter(_this, void 0, void 0, function () {
                    var present, temporary, progress, response, archive, file, hash, bytes, _a, _b, _c, chunk, e_1_1, members, selected, prefix_1, executable, payload, files, _i, selected_1, name_2, path, version;
                    var _d, e_1, _e, _f;
                    return __generator(this, function (_g) {
                        switch (_g.label) {
                            case 0:
                                present = true;
                                try {
                                    (0, node_fs_1.lstatSync)(directory);
                                }
                                catch (error) {
                                    if (error.code !== "ENOENT")
                                        throw error;
                                    present = false;
                                }
                                if (present)
                                    return [2 /*return*/, preparedBinaryen(workspace)];
                                temporary = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(parent, ".prepare-"));
                                progress = setInterval(function () { return console.log("Preparing Binaryen ".concat(manifest.version, "\u2026")); }, 10000);
                                _g.label = 1;
                            case 1:
                                _g.trys.push([1, , 24, 25]);
                                console.log("Downloading pinned Binaryen ".concat(manifest.version, " for ").concat(row.platform, "/").concat(row.architecture, "\u2026"));
                                return [4 /*yield*/, fetch("".concat(manifest.release, "/").concat(row.archive), { signal: signal })];
                            case 2:
                                response = _g.sent();
                                if (!response.ok || !response.body)
                                    throw new Error("Binaryen download failed: ".concat(response.status));
                                archive = (0, node_path_1.join)(temporary, "binaryen.tar.gz");
                                return [4 /*yield*/, (0, promises_1.open)(archive, "wx")];
                            case 3:
                                file = _g.sent(), hash = (0, node_crypto_1.createHash)("sha256");
                                bytes = 0;
                                _g.label = 4;
                            case 4:
                                _g.trys.push([4, , 18, 20]);
                                _g.label = 5;
                            case 5:
                                _g.trys.push([5, 11, 12, 17]);
                                _a = true, _b = __asyncValues(response.body);
                                _g.label = 6;
                            case 6: return [4 /*yield*/, _b.next()];
                            case 7:
                                if (!(_c = _g.sent(), _d = _c.done, !_d)) return [3 /*break*/, 10];
                                _f = _c.value;
                                _a = false;
                                chunk = _f;
                                signal.throwIfAborted();
                                bytes += chunk.byteLength;
                                if (bytes > row.bytes)
                                    throw new Error("Binaryen archive exceeds its pinned size");
                                hash.update(chunk);
                                return [4 /*yield*/, file.writeFile(chunk)];
                            case 8:
                                _g.sent();
                                _g.label = 9;
                            case 9:
                                _a = true;
                                return [3 /*break*/, 6];
                            case 10: return [3 /*break*/, 17];
                            case 11:
                                e_1_1 = _g.sent();
                                e_1 = { error: e_1_1 };
                                return [3 /*break*/, 17];
                            case 12:
                                _g.trys.push([12, , 15, 16]);
                                if (!(!_a && !_d && (_e = _b.return))) return [3 /*break*/, 14];
                                return [4 /*yield*/, _e.call(_b)];
                            case 13:
                                _g.sent();
                                _g.label = 14;
                            case 14: return [3 /*break*/, 16];
                            case 15:
                                if (e_1) throw e_1.error;
                                return [7 /*endfinally*/];
                            case 16: return [7 /*endfinally*/];
                            case 17: return [3 /*break*/, 20];
                            case 18: return [4 /*yield*/, file.close()];
                            case 19:
                                _g.sent();
                                return [7 /*endfinally*/];
                            case 20:
                                if (bytes !== row.bytes || hash.digest("hex") !== row.sha256)
                                    throw new Error("Binaryen archive checksum mismatch");
                                return [4 /*yield*/, (0, ___script_ts_1.runTool)("tar", ["-tzf", archive], temporary, signal, true)];
                            case 21:
                                members = (_g.sent()).trim().split(/\r?\n/);
                                selected = binaryenMembers(members, process.platform), prefix_1 = "binaryen-version_".concat(manifest.version);
                                executable = "bin/wasm-opt".concat(process.platform === "win32" ? ".exe" : "");
                                if (!selected.includes(executable))
                                    throw new Error("Binaryen archive has no optimizer");
                                return [4 /*yield*/, (0, ___script_ts_1.runTool)("tar", __spreadArray(["-xzf", archive, "-C", temporary], selected.map(function (path) { return "".concat(prefix_1, "/").concat(path); }), true), temporary, signal)];
                            case 22:
                                _g.sent();
                                payload = (0, node_path_1.join)(temporary, prefix_1), files = {};
                                for (_i = 0, selected_1 = selected; _i < selected_1.length; _i++) {
                                    name_2 = selected_1[_i];
                                    path = (0, node_path_1.join)(payload, name_2);
                                    regularDirectory((0, node_path_1.dirname)(path));
                                    if (!(0, node_fs_1.lstatSync)(path).isFile() || (0, node_fs_1.lstatSync)(path).isSymbolicLink())
                                        throw new Error("Invalid Binaryen payload: ".concat(name_2));
                                    files[name_2] = digest(path);
                                }
                                return [4 /*yield*/, (0, ___script_ts_1.runTool)((0, node_path_1.join)(payload, executable), ["--version"], temporary, signal, true)];
                            case 23:
                                version = _g.sent();
                                if (version.trim() !== "wasm-opt version ".concat(manifest.version, " (version_").concat(manifest.version, ")"))
                                    throw new Error("Binaryen version mismatch: ".concat(version.trim()));
                                (0, node_fs_1.writeFileSync)((0, node_path_1.join)(payload, ".toolchain.json"), JSON.stringify({ owner: owner, identity: binaryenIdentity(), files: files }) + "\n");
                                signal.throwIfAborted();
                                (0, node_fs_1.renameSync)(payload, directory);
                                return [2 /*return*/, preparedBinaryen(workspace)];
                            case 24:
                                clearInterval(progress);
                                (0, node_fs_1.rmSync)(temporary, { recursive: true, force: true });
                                return [7 /*endfinally*/];
                            case 25: return [2 /*return*/];
                        }
                    });
                }); })];
        });
    });
}
var PrepareScript = /** @class */ (function (_super) {
    __extends(PrepareScript, _super);
    function PrepareScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    PrepareScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var controller, stop, _a, _b, _c;
            return __generator(this, function (_d) {
                switch (_d.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Optimizer preparation accepts no arguments");
                        controller = new AbortController(), stop = function () { return controller.abort(new Error("Optimizer preparation cancelled")); };
                        process.once("SIGINT", stop);
                        process.once("SIGTERM", stop);
                        _d.label = 1;
                    case 1:
                        _d.trys.push([1, , 3, 4]);
                        _b = (_a = console).log;
                        _c = "Prepared ".concat;
                        return [4 /*yield*/, prepareBinaryen(this.root, AbortSignal.any([controller.signal, AbortSignal.timeout(300000)]))];
                    case 2:
                        _b.apply(_a, [_c.apply("Prepared ", [_d.sent()])]);
                        return [3 /*break*/, 4];
                    case 3:
                        process.removeListener("SIGINT", stop);
                        process.removeListener("SIGTERM", stop);
                        return [7 /*endfinally*/];
                    case 4: return [2 /*return*/];
                }
            });
        });
    };
    return PrepareScript;
}(____ts_1.Script));
/** 🔏️ Hashes tool versions without application imports or acquisition side effects. */
var FingerprintScript = /** @class */ (function (_super) {
    __extends(FingerprintScript, _super);
    function FingerprintScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    FingerprintScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var controller, stop, versions, _i, _a, tool, override, path, version;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Tool fingerprint accepts no arguments");
                        controller = new AbortController(), stop = function () { return controller.abort(new Error("Tool fingerprint cancelled")); };
                        process.once("SIGINT", stop);
                        process.once("SIGTERM", stop);
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, , 6, 7]);
                        versions = {};
                        _i = 0, _a = ["wasm-pack", "wasm-bindgen", "wasm-opt", "trunk"];
                        _b.label = 2;
                    case 2:
                        if (!(_i < _a.length)) return [3 /*break*/, 5];
                        tool = _a[_i];
                        controller.signal.throwIfAborted();
                        override = tool === "wasm-bindgen" ? process.env.SEMIO_WASM_BINDGEN_BIN : tool === "wasm-opt" ? process.env.SEMIO_WASM_OPT_BIN : undefined;
                        if (tool === "wasm-opt" && !override) {
                            versions[tool] = binaryenIdentity();
                            return [3 /*break*/, 4];
                        }
                        path = override ? (0, node_path_1.resolve)(this.root, override) : Bun.which(tool, { PATH: process.env.PATH });
                        if (!path) {
                            versions[tool] = "unavailable";
                            return [3 /*break*/, 4];
                        }
                        return [4 /*yield*/, (0, ___script_ts_1.runTool)(path, ["--version"], this.root, AbortSignal.any([controller.signal, AbortSignal.timeout(10000)]), true)];
                    case 3:
                        version = (_b.sent()).trim();
                        if (!version)
                            throw new Error("Cannot fingerprint ".concat(tool));
                        versions[tool] = version;
                        _b.label = 4;
                    case 4:
                        _i++;
                        return [3 /*break*/, 2];
                    case 5:
                        console.log(JSON.stringify(versions));
                        return [3 /*break*/, 7];
                    case 6:
                        process.removeListener("SIGINT", stop);
                        process.removeListener("SIGTERM", stop);
                        return [7 /*endfinally*/];
                    case 7: return [2 /*return*/];
                }
            });
        });
    };
    return FingerprintScript;
}(____ts_1.Script));
if (import.meta.main)
    await new ____ts_1.ScriptRouter((0, ____ts_2.getWorkspaceRoot)()).register("prepare", PrepareScript).register("fingerprint", FingerprintScript).run(process.argv.slice(2));
