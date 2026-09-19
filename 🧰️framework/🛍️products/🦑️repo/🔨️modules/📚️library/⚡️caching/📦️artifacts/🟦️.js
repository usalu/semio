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
exports.stageArtifacts = stageArtifacts;
var node_fs_1 = require("node:fs");
var promises_1 = require("node:fs/promises");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../\uD83D\uDD12\uFE0Fleases/\uD83D\uDFE6\uFE0F.ts");
/** 🧱️ Replaces only a previously owned deliverable tree and restores it if publication fails. */
function stageArtifacts(staging_1, owner_1, files_1) {
    return __awaiter(this, arguments, void 0, function (staging, owner, files, options) {
        var signal, lease, temporary, marker, parent_1, previous, _i, files_2, _a, name_1, source, destination;
        var _b, _c;
        if (options === void 0) { options = {}; }
        return __generator(this, function (_d) {
            switch (_d.label) {
                case 0:
                    signal = (_b = options.signal) !== null && _b !== void 0 ? _b : new AbortController().signal;
                    return [4 /*yield*/, (0, ____ts_2.acquireResourceLease)({ directory: (_c = options.leaseDirectory) !== null && _c !== void 0 ? _c : (0, node_path_1.join)((0, ____ts_1.getWorkspaceRoot)(), ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), resource: "artifact:".concat((0, node_path_1.resolve)(staging)), mode: "exclusive", signal: signal, onWait: options.onWait })];
                case 1:
                    lease = _d.sent();
                    _d.label = 2;
                case 2:
                    _d.trys.push([2, , 7, 8]);
                    marker = ".nx-artifact.json";
                    for (parent_1 = (0, node_path_1.resolve)(staging); (0, node_path_1.dirname)(parent_1) !== parent_1; parent_1 = (0, node_path_1.dirname)(parent_1))
                        if ((0, node_fs_1.existsSync)(parent_1) && (0, node_fs_1.lstatSync)(parent_1).isSymbolicLink())
                            throw new Error("Symlink artifact destination: ".concat(parent_1));
                    if ((0, node_fs_1.existsSync)(staging) && (!(0, node_fs_1.existsSync)((0, node_path_1.join)(staging, marker)) || JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.join)(staging, marker), "utf8")).owner !== owner))
                        throw new Error("Unowned artifact directory: ".concat(staging));
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(staging), { recursive: true });
                    temporary = (0, node_fs_1.mkdtempSync)("".concat(staging, ".stage-"));
                    previous = "".concat(temporary, ".previous");
                    _i = 0, files_2 = files;
                    _d.label = 3;
                case 3:
                    if (!(_i < files_2.length)) return [3 /*break*/, 6];
                    _a = files_2[_i], name_1 = _a[0], source = _a[1];
                    signal.throwIfAborted();
                    if (name_1.startsWith("/") || name_1.split(/[\\/]/).includes(".."))
                        throw new Error("Invalid artifact path ".concat(name_1));
                    destination = (0, node_path_1.join)(temporary, name_1);
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(destination), { recursive: true });
                    return [4 /*yield*/, (0, promises_1.copyFile)(source, destination)];
                case 4:
                    _d.sent();
                    (0, node_fs_1.chmodSync)(destination, (0, node_fs_1.lstatSync)(source).mode & 511);
                    _d.label = 5;
                case 5:
                    _i++;
                    return [3 /*break*/, 3];
                case 6:
                    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(temporary, marker), JSON.stringify({ version: 1, owner: owner, files: __spreadArray([], files.keys(), true).sort() }) + "\n");
                    signal.throwIfAborted();
                    if ((0, node_fs_1.existsSync)(staging))
                        (0, node_fs_1.renameSync)(staging, previous);
                    try {
                        (0, node_fs_1.renameSync)(temporary, staging);
                    }
                    catch (error) {
                        if ((0, node_fs_1.existsSync)(previous))
                            (0, node_fs_1.renameSync)(previous, staging);
                        throw error;
                    }
                    (0, node_fs_1.rmSync)(previous, { recursive: true, force: true });
                    return [3 /*break*/, 8];
                case 7:
                    try {
                        if (temporary)
                            (0, node_fs_1.rmSync)(temporary, { recursive: true, force: true });
                    }
                    finally {
                        lease.release();
                    }
                    return [7 /*endfinally*/];
                case 8: return [2 /*return*/];
            }
        });
    });
}
