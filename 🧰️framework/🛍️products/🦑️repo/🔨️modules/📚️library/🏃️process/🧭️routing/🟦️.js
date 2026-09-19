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
exports.ScriptRouter = exports.BundleScript = exports.Script = void 0;
exports.findRepoRoot = findRepoRoot;
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
/** 🧭️Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
var Script = /** @class */ (function () {
    function Script(root, repoRoot) {
        this.root = root;
        this.repoRoot = repoRoot;
    }
    return Script;
}());
exports.Script = Script;
/** 📦️Bundle-scoped command with `root` at the package directory. */
var BundleScript = /** @class */ (function (_super) {
    __extends(BundleScript, _super);
    function BundleScript(bundleRoot, repoRoot) {
        return _super.call(this, bundleRoot, repoRoot !== null && repoRoot !== void 0 ? repoRoot : findRepoRoot(bundleRoot)) || this;
    }
    return BundleScript;
}(Script));
exports.BundleScript = BundleScript;
/** 🧭️Declarative subcommand registry for a single `script.ts`. */
var ScriptRouter = /** @class */ (function () {
    function ScriptRouter(bundleRoot, repoRoot) {
        if (repoRoot === void 0) { repoRoot = findRepoRoot(bundleRoot); }
        this.commands = new Map();
        this.bundleRoot = bundleRoot;
        this.repoRoot = repoRoot;
    }
    /** 📌️Registers a subcommand implemented by a `Script` subclass. */
    ScriptRouter.prototype.register = function (name, Command) {
        this.commands.set(name, Command);
        return this;
    };
    /** 📋️Human-readable usage line for this router. */
    ScriptRouter.prototype.usage = function () {
        var names = __spreadArray([], this.commands.keys(), true);
        if (names.length === 0)
            return "bun ./📜️script.ts policy";
        return "bun ./\uD83D\uDCDC\uFE0Fscript.ts <".concat(names.join("|"), "> [args\u2026]");
    };
    /** 📊️Whether any subcommands are registered (policy-only bundles may have none). */
    ScriptRouter.prototype.hasCommands = function () {
        return this.commands.size > 0;
    };
    /** ▶️Dispatches `segments[0]` to a registered command class. */
    ScriptRouter.prototype.run = function (segments) {
        return __awaiter(this, void 0, void 0, function () {
            var name, Command;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        name = segments[0];
                        if (!name) {
                            console.error("usage: ".concat(this.usage()));
                            process.exit(1);
                        }
                        Command = this.commands.get(name);
                        if (!Command) {
                            console.error("unknown command ".concat(JSON.stringify(name)));
                            console.error("usage: ".concat(this.usage()));
                            process.exit(1);
                        }
                        return [4 /*yield*/, Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)))];
                    case 1:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    return ScriptRouter;
}());
exports.ScriptRouter = ScriptRouter;
/** 📁️Walks parents until the monorepo root (`nx.json` + workspace `package.json`); starts from the Nx workspace root when no directory is given. */
function findRepoRoot(start) {
    var dir = (start === null || start === void 0 ? void 0 : start.trim()) ? start : (0, ____ts_1.getWorkspaceRoot)();
    for (var i = 0; i < 32; i++) {
        if ((0, node_fs_1.existsSync)((0, node_path_1.join)(dir, "nx.json")) && (0, node_fs_1.existsSync)((0, node_path_1.join)(dir, "package.json")))
            return dir;
        var parent_1 = (0, node_path_1.dirname)(dir);
        if (parent_1 === dir)
            break;
        dir = parent_1;
    }
    return (0, ____ts_1.getWorkspaceRoot)();
}
