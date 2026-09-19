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
exports.writeGeneratedFileIfChanged = writeGeneratedFileIfChanged;
exports.collectArtifactFiles = collectArtifactFiles;
var promises_1 = require("node:fs/promises");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
/** 🪶️ Writes generated text only when its bytes changed, preserving no-op prerequisite mtimes. */
function writeGeneratedFileIfChanged(path, content) {
    if ((0, node_fs_1.existsSync)(path)) {
        var metadata = (0, node_fs_1.lstatSync)(path);
        if (!metadata.isFile() || metadata.isSymbolicLink())
            throw new Error("Invalid generated file: ".concat(path));
        if ((0, node_fs_1.readFileSync)(path, "utf8") === content)
            return false;
    }
    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(path), { recursive: true });
    (0, node_fs_1.writeFileSync)(path, content, "utf8");
    return true;
}
/** 🗂️ Collects regular staged files without following links or retaining compiler directory state. */
function collectArtifactFiles(root, signal) {
    return __awaiter(this, void 0, void 0, function () {
        var metadata, files, pending, directory, _i, _a, entry, path;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    signal === null || signal === void 0 ? void 0 : signal.throwIfAborted();
                    return [4 /*yield*/, (0, promises_1.lstat)(root)];
                case 1:
                    metadata = _b.sent();
                    if (!metadata.isDirectory() || metadata.isSymbolicLink())
                        throw new Error("Invalid artifact root: ".concat(root));
                    files = new Map(), pending = [root];
                    _b.label = 2;
                case 2:
                    if (!pending.length) return [3 /*break*/, 7];
                    signal === null || signal === void 0 ? void 0 : signal.throwIfAborted();
                    directory = pending.pop();
                    _i = 0;
                    return [4 /*yield*/, (0, promises_1.readdir)(directory, { withFileTypes: true })];
                case 3:
                    _a = _b.sent();
                    _b.label = 4;
                case 4:
                    if (!(_i < _a.length)) return [3 /*break*/, 6];
                    entry = _a[_i];
                    signal === null || signal === void 0 ? void 0 : signal.throwIfAborted();
                    path = (0, node_path_1.join)(directory, entry.name);
                    if (entry.isDirectory())
                        pending.push(path);
                    else if (entry.isFile())
                        files.set((0, node_path_1.relative)(root, path).replaceAll("\\", "/"), path);
                    else
                        throw new Error("Unsupported artifact file: ".concat(path));
                    _b.label = 5;
                case 5:
                    _i++;
                    return [3 /*break*/, 4];
                case 6: return [3 /*break*/, 2];
                case 7: return [2 /*return*/, new Map(__spreadArray([], files, true).sort(function (_a, _b) {
                        var a = _a[0];
                        var b = _b[0];
                        return a < b ? -1 : a > b ? 1 : 0;
                    }))];
            }
        });
    });
}
