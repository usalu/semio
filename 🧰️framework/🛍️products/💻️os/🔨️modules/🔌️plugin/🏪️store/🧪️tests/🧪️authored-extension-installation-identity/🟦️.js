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
exports.registerTests1 = registerTests1;
function registerTests1(vitest, dependencies, source) {
    return __awaiter(this, void 0, void 0, function () {
        var EXTENSION_COMPONENT_FILE, EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, EXTENSION_PACKAGE_ENVELOPE_TOKEN, MODULE_EXTENSION_ROUTE, createExtensionStore, decodeOwnedZip, decodePackValue, existsSync, extensionPackageContentHash, installationDirectoryCollision, installationDirectoryEmoji, join, mkdtempSync, packExtensionPackage, readFileSync, rmSync, tmpdir, unpackExtensionPackage, wrapExtensionPackageEnvelope, describe, expect, it, legacyFflateZip, fixtureManifest, fixtureWasm, fixtureAsset;
        var _this = this;
        return __generator(this, function (_a) {
            EXTENSION_COMPONENT_FILE = dependencies.EXTENSION_COMPONENT_FILE, EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI = dependencies.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, EXTENSION_PACKAGE_ENVELOPE_TOKEN = dependencies.EXTENSION_PACKAGE_ENVELOPE_TOKEN, MODULE_EXTENSION_ROUTE = dependencies.MODULE_EXTENSION_ROUTE, createExtensionStore = dependencies.createExtensionStore, decodeOwnedZip = dependencies.decodeOwnedZip, decodePackValue = dependencies.decodePackValue, existsSync = dependencies.existsSync, extensionPackageContentHash = dependencies.extensionPackageContentHash, installationDirectoryCollision = dependencies.installationDirectoryCollision, installationDirectoryEmoji = dependencies.installationDirectoryEmoji, join = dependencies.join, mkdtempSync = dependencies.mkdtempSync, packExtensionPackage = dependencies.packExtensionPackage, readFileSync = dependencies.readFileSync, rmSync = dependencies.rmSync, tmpdir = dependencies.tmpdir, unpackExtensionPackage = dependencies.unpackExtensionPackage, wrapExtensionPackageEnvelope = dependencies.wrapExtensionPackageEnvelope;
            describe = vitest.describe, expect = vitest.expect, it = vitest.it;
            legacyFflateZip = Uint8Array.from(Buffer.from("UEsDBBQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAA8J+bgu+4j21hbmlmZXN0LnNlbWlvY2U11DPSM+ZPy6woKS1K1Tu8JzcnsbSEpSg1MYWxmP/D/OUr3+/oV3CDSDMyCgqws/MkJxYkJmXmZJZkphbzMLIxsfMm5+eVFGUmlZZk5ucV8zCws6dWlKTmpRSzMbNzg5nFQAnPFDZGdtacxKTUHDYWdt6CxOTsxPRUt/yi3MQSVgYw+GDPzl6WWgRSzcYAAFBLAwQUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAGNvbXBvbmVudC53YXNtY0gszmVkYGAAAFBLAwQUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAGFzc2V0cy9pY29ucy/wn6ep77iPLnR4dHMvOrwntSpT4cP8nt73O/q5AFBLAQIUABQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAAAAAAAAAAAAAAAAAAAADwn5uC77iPbWFuaWZlc3Quc2VtaW9QSwECFAAUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAAAAAAAAAAAAAADEAAAAY29tcG9uZW50Lndhc21QSwECFAAUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAAAAAAAAAAAAAAD6AAAAYXNzZXRzL2ljb25zL/Cfp6nvuI8udHh0UEsFBgAAAAADAAMAxQAAAEIBAAAAAA==", "base64"));
            fixtureManifest = {
                extensionId: "fixture.ümlaut",
                directoryName: "🧩️fixture-umlaut",
                label: "🧩️ Fixture",
                version: "1.2.3",
                extends: "s",
                capabilities: ["read"],
                contributions: [],
                packageFormat: 1,
            };
            fixtureWasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);
            fixtureAsset = new TextEncoder().encode("Grüezi 🌍️\n");
            describe("authored extension installation identity", function () {
                it("retains the declared physical name independently of the public extension ID", function () { return __awaiter(_this, void 0, void 0, function () {
                    var vector, root, writes, store, packed, installed, _a, collision;
                    var _this = this;
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0:
                                vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", source.url), "utf8"));
                                root = mkdtempSync(join(tmpdir(), "semio-authored-install-"));
                                writes = [];
                                _b.label = 1;
                            case 1:
                                _b.trys.push([1, , 6, 7]);
                                store = createExtensionStore({ installRoot: root, repoRoot: root, materializer: function (input) { return __awaiter(_this, void 0, void 0, function () {
                                        return __generator(this, function (_a) {
                                            writes.push(input.outDir);
                                            return [2 /*return*/, { moduleUrl: "".concat(MODULE_EXTENSION_ROUTE, "/").concat(input.directoryName, "/module.js") }];
                                        });
                                    }); } });
                                packed = packExtensionPackage({ manifest: vector.manifest, componentWasm: fixtureWasm });
                                return [4 /*yield*/, store.installFromBytes(packed)];
                            case 2:
                                installed = _b.sent();
                                expect(installed.extensionId).toBe(vector.manifest.extensionId);
                                expect(writes).toEqual([join(root, vector.manifest.directoryName)]);
                                _a = expect;
                                return [4 /*yield*/, store.listInstalled()];
                            case 3:
                                _a.apply(void 0, [(_b.sent())[0].directoryName]).toBe(vector.manifest.directoryName);
                                expect(existsSync(join(root, vector.manifest.extensionId))).toBe(false);
                                collision = __assign(__assign({}, vector.manifest), { extensionId: "other-id", directoryName: "🧩️another" });
                                return [4 /*yield*/, expect(store.installFromBytes(packExtensionPackage({ manifest: collision, componentWasm: fixtureWasm }))).rejects.toThrow(/sibling emoji/)];
                            case 4:
                                _b.sent();
                                expect(writes).toHaveLength(1);
                                return [4 /*yield*/, store.uninstall(vector.manifest.extensionId)];
                            case 5:
                                _b.sent();
                                expect(existsSync(join(root, vector.manifest.directoryName))).toBe(false);
                                return [3 /*break*/, 7];
                            case 6:
                                rmSync(root, { recursive: true, force: true });
                                return [7 /*endfinally*/];
                            case 7: return [2 /*return*/];
                        }
                    });
                }); });
                it("agrees with JSON Schema and independent emoji identity checks", function () { return __awaiter(_this, void 0, void 0, function () {
                    var Ajv, emojiRegex, _a, installationDirectoryEmoji, installationDirectoryCollision, schema, vector, cases, validate, _loop_1, _i, _b, name_1, _c, _d, row;
                    var _e;
                    return __generator(this, function (_f) {
                        switch (_f.label) {
                            case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("ajv"); })];
                            case 1:
                                Ajv = (_f.sent()).default;
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("emoji-regex"); })];
                            case 2:
                                emojiRegex = (_f.sent()).default;
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("../../../../🧩️extension/🟦️.ts"); })];
                            case 3:
                                _a = _f.sent(), installationDirectoryEmoji = _a.installationDirectoryEmoji, installationDirectoryCollision = _a.installationDirectoryCollision;
                                schema = JSON.parse(readFileSync(new URL("../../🧩️extension/📐️directory.schema.json", source.url), "utf8"));
                                vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", source.url), "utf8"));
                                cases = JSON.parse(readFileSync(new URL("../📇️registry/🧫️fixtures/📦️deployment/🧪️cases.json", source.url), "utf8"));
                                validate = new Ajv({ strict: true }).compile(schema);
                                _loop_1 = function (name_1) {
                                    var valid = cases.validDirectories.includes(name_1);
                                    expect(validate(name_1), name_1).toBe(valid);
                                    if (valid)
                                        expect(installationDirectoryEmoji(name_1)).toBe(__spreadArray([], name_1.replaceAll("\uFE0F", "").matchAll(emojiRegex()), true)[0][0]);
                                    else
                                        expect(function () { return installationDirectoryEmoji(name_1); }).toThrow();
                                };
                                for (_i = 0, _b = __spreadArray(__spreadArray([], cases.validDirectories, true), cases.invalidDirectories, true); _i < _b.length; _i++) {
                                    name_1 = _b[_i];
                                    _loop_1(name_1);
                                }
                                for (_c = 0, _d = vector.collisions; _c < _d.length; _c++) {
                                    row = _d[_c];
                                    expect((_e = installationDirectoryCollision(row.directoryName, row.siblings)) !== null && _e !== void 0 ? _e : null).toBe(row.conflict);
                                }
                                return [2 /*return*/];
                        }
                    });
                }); });
            });
            describe("extension package ZIP ownership", function () {
                it("decodes the pinned fflate UTF-8/DEFLATE fixture and preserves its hash", function () {
                    expect(extensionPackageContentHash(legacyFflateZip)).toBe("43675c79f03ba52f45cc57eecabee2a9334e93957128e7975a2979528d14efa9");
                    var decoded = decodeOwnedZip(legacyFflateZip);
                    expect(decodePackValue(decoded.get(EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI))).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
                    expect(decoded.get(EXTENSION_COMPONENT_FILE)).toEqual(fixtureWasm);
                    expect(decoded.get("assets/icons/🧩️.txt")).toEqual(fixtureAsset);
                    expect(function () { return unpackExtensionPackage(wrapExtensionPackageEnvelope(legacyFflateZip)); }).toThrow(/Installation directory/);
                });
                it("encodes deterministic synchronous packages with UTF-8 asset names", function () {
                    var input = { manifest: fixtureManifest, componentWasm: fixtureWasm, assets: new Map([["icons/🧩️.txt", fixtureAsset]]) };
                    var first = packExtensionPackage(input);
                    var second = packExtensionPackage(input);
                    expect(first).toEqual(second);
                    var unpacked = unpackExtensionPackage(first);
                    expect(unpacked.packageHash).toBe(extensionPackageContentHash(first));
                    expect(unpacked.manifest).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
                    expect(unpacked.wasmBytes).toEqual(fixtureWasm);
                    expect(unpacked.assets.get("icons/🧩️.txt")).toEqual(fixtureAsset);
                });
                it("rejects an entry whose declared expansion exceeds the owned bound", function () {
                    var packed = packExtensionPackage({ manifest: fixtureManifest, componentWasm: fixtureWasm });
                    var zipStart = 12 + new TextEncoder().encode(EXTENSION_PACKAGE_ENVELOPE_TOKEN).length;
                    var corrupted = packed.slice(zipStart);
                    var data = new DataView(corrupted.buffer, corrupted.byteOffset, corrupted.byteLength);
                    var end = corrupted.length - 22;
                    var central = data.getUint32(end + 16, true);
                    data.setUint32(central + 24, 256 * 1024 * 1024 + 1, true);
                    expect(function () { return unpackExtensionPackage(wrapExtensionPackageEnvelope(corrupted)); }).toThrow("decoded size limit");
                });
            });
            return [2 /*return*/];
        });
    });
}
