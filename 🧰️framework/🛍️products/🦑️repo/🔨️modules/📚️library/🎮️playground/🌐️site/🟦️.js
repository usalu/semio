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
exports.playgroundVariantsFromCrateManifest = playgroundVariantsFromCrateManifest;
exports.registerPlaygroundSiteBuildCommands = registerPlaygroundSiteBuildCommands;
/** @emoji 🌐️ Builds CDN-ready standalone playground sites from plugin `📜️script.ts build`. */
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDED\uFE0Frouting/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../\uD83D\uDFE6\uFE0F.ts");
var DISTRIBUTION_BUILD_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts";
/** @emoji 🎮️ Playground variant ids declared on the crate next to this package's `📜️script.ts`. */
function playgroundVariantsFromCrateManifest(crateManifestPath) {
    var _a;
    var text = (0, node_fs_1.readFileSync)(crateManifestPath, "utf8");
    var variants = [];
    for (var _i = 0, _b = text.split("[[package.metadata.semio.playground]]").slice(1); _i < _b.length; _i++) {
        var block = _b[_i];
        var variant = (_a = block.match(/^variant\s*=\s*"([^"]+)"/m)) === null || _a === void 0 ? void 0 : _a[1];
        if (!variant)
            throw new Error("Invalid playground block in ".concat(crateManifestPath));
        variants.push(variant);
    }
    return variants;
}
/** @emoji 🏗️ Registers `build [<variant>|all]` to publish the react release distribution for this plugin crate. */
function registerPlaygroundSiteBuildCommands(router) {
    var BuildScript = /** @class */ (function (_super) {
        __extends(BuildScript, _super);
        function BuildScript() {
            return _super !== null && _super.apply(this, arguments) || this;
        }
        BuildScript.prototype.run = function (segments) {
            return __awaiter(this, void 0, void 0, function () {
                var manifestPath, variants, mode, selected, buildScript, _i, selected_1, variant;
                var _a;
                return __generator(this, function (_b) {
                    manifestPath = (0, node_path_1.join)(this.root, "Cargo.toml");
                    variants = __spreadArray([], playgroundVariantsFromCrateManifest(manifestPath), true);
                    if (variants.length === 0)
                        throw new Error("This crate declares no [[package.metadata.semio.playground]] rows");
                    mode = (_a = segments[0]) !== null && _a !== void 0 ? _a : (variants.length === 1 ? variants[0] : undefined);
                    if (!mode)
                        throw new Error("Usage: build <".concat(variants.join("|"), "|all>"));
                    selected = mode === "all" ? variants : variants.includes(mode) ? [mode] : undefined;
                    if (!selected)
                        throw new Error("Unknown playground variant ".concat(JSON.stringify(mode), " \u2014 expected ").concat(variants.join(", "), " or all"));
                    buildScript = (0, node_path_1.join)(this.repoRoot, DISTRIBUTION_BUILD_SCRIPT);
                    for (_i = 0, selected_1 = selected; _i < selected_1.length; _i++) {
                        variant = selected_1[_i];
                        (0, ____ts_2.runBun)([buildScript, "build", variant, "react", "release"], this.repoRoot);
                    }
                    return [2 /*return*/];
                });
            });
        };
        return BuildScript;
    }(____ts_1.BundleScript));
    router.register("build", BuildScript);
}
