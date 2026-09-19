#!/usr/bin/env bun
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
var __rest = (this && this.__rest) || function (s, e) {
    var t = {};
    for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p) && e.indexOf(p) < 0)
        t[p] = s[p];
    if (s != null && typeof Object.getOwnPropertySymbols === "function")
        for (var i = 0, p = Object.getOwnPropertySymbols(s); i < p.length; i++) {
            if (e.indexOf(p[i]) < 0 && Object.prototype.propertyIsEnumerable.call(s, p[i]))
                t[p[i]] = s[p[i]];
        }
    return t;
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
exports.webMaterialize = exports.nativeMaterialize = exports.EXTENSION_PACKAGE_ENVELOPE_TOKEN = exports.EXTENSION_PACKAGE_FORMAT = exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI = exports.EXTENSION_MANIFEST_ZIP_ENTRY = exports.EXTENSION_COMPONENT_FILE = exports.EXTENSION_INSTALL_META = exports.EXTENSION_WATCH_MARKER = exports.EXTENSION_WATCH_PATH = exports.EXTENSION_INSTALL_PATH = exports.EXTENSION_STATIC_ROUTE = void 0;
exports.wrapExtensionPackageEnvelope = wrapExtensionPackageEnvelope;
exports.packExtensionPackage = packExtensionPackage;
exports.extensionPackageContentHash = extensionPackageContentHash;
exports.unpackExtensionPackage = unpackExtensionPackage;
exports.createExtensionStore = createExtensionStore;
exports.defaultExtensionInstallRoot = defaultExtensionInstallRoot;
exports.semioExtensionStoreVitePlugin = semioExtensionStoreVitePlugin;
/** @emoji 🏪 Runtime-installable extension store — unpack `.semio` packages, materialize for native/web, dev-server install + SSE. */
var node_crypto_1 = require("node:crypto");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var node_os_1 = require("node:os");
var framework_os_1 = require("@semio-tech/framework-os");
var ____ts_1 = require("../\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../\uD83E\uDDE9\uFE0Fextension/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../../\uD83C\uDF10\uFE0Fbrowser-bundle/\uD83C\uDFD7\uFE0Fmaterialization/\uD83D\uDFE6\uFE0F.ts");
//#region 🔖️Constants
exports.EXTENSION_STATIC_ROUTE = ____ts_3.MODULE_EXTENSION_ROUTE;
exports.EXTENSION_INSTALL_PATH = "".concat(exports.EXTENSION_STATIC_ROUTE, "/install");
exports.EXTENSION_WATCH_PATH = "".concat(exports.EXTENSION_STATIC_ROUTE, "/watch");
exports.EXTENSION_WATCH_MARKER = "👀️extension-watch.json";
exports.EXTENSION_INSTALL_META = "📥️install.json";
exports.EXTENSION_COMPONENT_FILE = "component.wasm";
exports.EXTENSION_MANIFEST_ZIP_ENTRY = "manifest.semio";
exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI = "🛂️manifest.semio";
exports.EXTENSION_PACKAGE_FORMAT = 1;
exports.EXTENSION_PACKAGE_ENVELOPE_TOKEN = "os.extension.pack v1";
var SEMIO_BINARY_MAGIC = new Uint8Array([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
var FOLDER_WATCH_DEBOUNCE_MS = 200;
//#endregion 🔖️Types
//#region 🔖️Package
function packageContentHash(bytes) {
    return (0, node_crypto_1.createHash)("sha256").update(bytes).digest("hex");
}
function unwrapSemioEnvelope(bytes) {
    if (bytes.length < 12 || !SEMIO_BINARY_MAGIC.every(function (value, index) { return bytes[index] === value; })) {
        return bytes;
    }
    var tokenLen = new DataView(bytes.buffer, bytes.byteOffset + 8, 4).getUint32(0, true);
    var payloadStart = 12 + tokenLen;
    if (payloadStart > bytes.length)
        throw new Error("truncated semio extension package envelope");
    return bytes.subarray(payloadStart);
}
/** @emoji 📨 Wraps deflate zip bytes in the Wave-1.A semio binary envelope (`os.extension.pack v1`). */
function wrapExtensionPackageEnvelope(zipBytes) {
    var tokenBytes = new TextEncoder().encode(exports.EXTENSION_PACKAGE_ENVELOPE_TOKEN);
    var out = new Uint8Array(SEMIO_BINARY_MAGIC.length + 4 + tokenBytes.length + zipBytes.length);
    out.set(SEMIO_BINARY_MAGIC, 0);
    new DataView(out.buffer, out.byteOffset + 8, 4).setUint32(0, tokenBytes.length, true);
    out.set(tokenBytes, 12);
    out.set(zipBytes, 12 + tokenBytes.length);
    return out;
}
function buildExtensionZipPayload(manifest, componentWasm, assets) {
    (0, ____ts_2.installationDirectoryEmoji)(manifest.directoryName);
    if (componentWasm.length === 0)
        throw new Error("extension component.wasm is empty");
    if (manifest.packageFormat !== exports.EXTENSION_PACKAGE_FORMAT)
        throw new Error("invalid extension package format ".concat(manifest.packageFormat));
    var manifestBytes = (0, framework_os_1.encodePackValue)(manifest);
    var files = new Map([
        [exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, manifestBytes],
        [exports.EXTENSION_COMPONENT_FILE, componentWasm],
    ]);
    var assetNames = __spreadArray([], assets.keys(), true).sort();
    for (var _i = 0, assetNames_1 = assetNames; _i < assetNames_1.length; _i++) {
        var name_1 = assetNames_1[_i];
        var payload = assets.get(name_1);
        if (!payload)
            continue;
        files.set(name_1.startsWith("assets/") ? name_1 : "assets/".concat(name_1), payload);
    }
    return (0, ____ts_1.encodeOwnedZip)(files);
}
/** @emoji 📦 Packs manifest + wasip2 component bytes into a `.sxt` stream (semio envelope + deterministic deflate zip). */
function packExtensionPackage(input) {
    var _a;
    var zipBytes = buildExtensionZipPayload(input.manifest, input.componentWasm, (_a = input.assets) !== null && _a !== void 0 ? _a : new Map());
    return wrapExtensionPackageEnvelope(zipBytes);
}
/** @emoji 🔓️ SHA-256 hex digest of the full `.sxt` bytes (matches install-store dedup). */
function extensionPackageContentHash(bytes) {
    return packageContentHash(bytes);
}
function zipEntryBytes(files, predicate) {
    for (var _i = 0, files_1 = files; _i < files_1.length; _i++) {
        var _a = files_1[_i], name_2 = _a[0], payload = _a[1];
        if (predicate(name_2))
            return payload;
    }
    return undefined;
}
function decodeExtensionManifest(manifestBytes) {
    var decoded;
    try {
        decoded = (0, framework_os_1.decodePackValue)(manifestBytes);
    }
    catch (_a) {
        decoded = JSON.parse(new TextDecoder().decode(manifestBytes));
    }
    if (!decoded || typeof decoded !== "object")
        throw new Error("extension manifest is not a pack object");
    var row = decoded;
    var extensionId = row.extensionId;
    (0, ____ts_2.installationDirectoryEmoji)(row.directoryName);
    var label = row.label;
    var version = row.version;
    var extendsHost = row.extends;
    if (typeof extensionId !== "string" || !extensionId)
        throw new Error("extension manifest missing extensionId");
    if (typeof label !== "string")
        throw new Error("extension manifest missing label");
    if (typeof version !== "string" || !version)
        throw new Error("extension manifest missing version");
    return {
        extensionId: extensionId,
        directoryName: row.directoryName,
        label: label,
        version: version,
        extends: typeof extendsHost === "string" ? extendsHost : "",
        capabilities: Array.isArray(row.capabilities) ? row.capabilities : undefined,
        contributions: Array.isArray(row.contributions) ? row.contributions : undefined,
    };
}
/** @emoji 📦 Unpacks a Wave-1.A extension package (semio envelope + deflate zip) into wasm bytes, manifest, and optional assets. */
function unpackExtensionPackage(bytes) {
    var _a, _b;
    var packageHash = packageContentHash(bytes);
    var zipBytes = unwrapSemioEnvelope(bytes);
    var files = (0, ____ts_1.decodeOwnedZip)(zipBytes);
    var manifestBytes = (_a = zipEntryBytes(files, function (name) { return name === exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI || name.endsWith(exports.EXTENSION_MANIFEST_ZIP_ENTRY); })) !== null && _a !== void 0 ? _a : (function () {
        throw new Error("extension package missing ".concat(exports.EXTENSION_MANIFEST_ZIP_ENTRY));
    })();
    var wasmBytes = (_b = zipEntryBytes(files, function (name) { return name === exports.EXTENSION_COMPONENT_FILE || name.endsWith("/".concat(exports.EXTENSION_COMPONENT_FILE)); })) !== null && _b !== void 0 ? _b : (function () {
        throw new Error("extension package missing ".concat(exports.EXTENSION_COMPONENT_FILE));
    })();
    var assets = new Map();
    for (var _i = 0, files_2 = files; _i < files_2.length; _i++) {
        var _c = files_2[_i], name_3 = _c[0], payload = _c[1];
        if (name_3 === exports.EXTENSION_COMPONENT_FILE || name_3.endsWith(exports.EXTENSION_MANIFEST_ZIP_ENTRY) || name_3.endsWith(exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI))
            continue;
        if (name_3.startsWith("assets/"))
            assets.set(name_3.slice("assets/".length), payload);
    }
    return { manifest: decodeExtensionManifest(manifestBytes), wasmBytes: wasmBytes, assets: assets, packageHash: packageHash };
}
//#endregion 🔖️Package
//#region 🔖️Materializers
/** @emoji 🦀 Native host materializer — keeps raw `component.wasm` on disk for wasmtime `Component::from_binary`. */
var nativeMaterialize = function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
    var wasmBytes = _b.wasmBytes, outDir = _b.outDir, directoryName = _b.directoryName;
    return __generator(this, function (_c) {
        (0, ____ts_2.installationDirectoryEmoji)(directoryName);
        (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, exports.EXTENSION_COMPONENT_FILE), wasmBytes);
        return [2 /*return*/, { moduleUrl: "".concat(exports.EXTENSION_STATIC_ROUTE, "/").concat(directoryName, "/").concat(exports.EXTENSION_COMPONENT_FILE) }];
    });
}); };
exports.nativeMaterialize = nativeMaterialize;
/** @emoji 🌐 Web materializer — jco transpile + bridge (see `🟦️.ts`). */
var webMaterialize = function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
    var _i, assets_1, _c, rel, payload, assetPath, jsBase, componentBase, artifactDir, artifactPath;
    var wasmBytes = _b.wasmBytes, assets = _b.assets, outDir = _b.outDir, directoryName = _b.directoryName, materializeCtx = _b.materializeCtx;
    return __generator(this, function (_d) {
        (0, ____ts_2.installationDirectoryEmoji)(directoryName);
        (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, exports.EXTENSION_COMPONENT_FILE), wasmBytes);
        for (_i = 0, assets_1 = assets; _i < assets_1.length; _i++) {
            _c = assets_1[_i], rel = _c[0], payload = _c[1];
            assetPath = (0, node_path_1.join)(outDir, "assets", rel);
            (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(assetPath), { recursive: true });
            (0, node_fs_1.writeFileSync)(assetPath, payload);
        }
        (0, ____ts_4.ensurePreview2ShimVendorAt)(materializeCtx.preview2VendorDir, materializeCtx.repoRoot);
        jsBase = exports.EXTENSION_COMPONENT_FILE.replace(/\.wasm$/, "");
        componentBase = "".concat(jsBase, "_component");
        artifactDir = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)((0, node_os_1.tmpdir)(), "semio-ext-jco-"));
        artifactPath = (0, node_path_1.join)(artifactDir, exports.EXTENSION_COMPONENT_FILE);
        try {
            (0, node_fs_1.writeFileSync)(artifactPath, wasmBytes);
            (0, ____ts_4.transpilePluginComponent)(artifactPath, outDir, componentBase, materializeCtx);
            (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, ____ts_4.PLUGIN_HOST_SHIM_FILE), (0, ____ts_4.hostShimSource)());
            (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, ____ts_4.SHARD_WORKER_FILE), (0, ____ts_4.shardWorkerSource)());
            (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, ____ts_3.MODULE_BRIDGE_FILE), (0, ____ts_4.pluginComponentBridgeSource)(componentBase, exports.EXTENSION_COMPONENT_FILE));
        }
        finally {
            (0, node_fs_1.rmSync)(artifactDir, { recursive: true, force: true });
        }
        return [2 /*return*/, { moduleUrl: "".concat(exports.EXTENSION_STATIC_ROUTE, "/").concat(directoryName, "/").concat(____ts_3.MODULE_BRIDGE_FILE) }];
    });
}); };
exports.webMaterialize = webMaterialize;
//#endregion 🔖️Materializers
//#region 🔖️Store
function readInstallMeta(dir) {
    var metaPath = (0, node_path_1.join)(dir, exports.EXTENSION_INSTALL_META);
    if (!(0, node_fs_1.existsSync)(metaPath))
        return undefined;
    if ((0, node_fs_1.lstatSync)(dir).isSymbolicLink() || (0, node_fs_1.lstatSync)(metaPath).isSymbolicLink())
        throw new Error("Extension installation metadata must not follow a symlink");
    var meta = JSON.parse((0, node_fs_1.readFileSync)(metaPath, "utf8"));
    (0, ____ts_2.installationDirectoryEmoji)(meta.directoryName);
    if (meta.directoryName !== (0, node_path_1.basename)(dir) || typeof meta.extensionId !== "string" || !meta.extensionId)
        throw new Error("Extension installation metadata identity mismatch");
    return meta;
}
function scanInstalledExtensions(installRoot) {
    if (!(0, node_fs_1.existsSync)(installRoot))
        return [];
    var rows = [];
    for (var _i = 0, _a = (0, node_fs_1.readdirSync)(installRoot, { withFileTypes: true }); _i < _a.length; _i++) {
        var entry = _a[_i];
        if (!entry.isDirectory() || entry.name.startsWith("_") || entry.name.startsWith("."))
            continue;
        var meta = readInstallMeta((0, node_path_1.join)(installRoot, entry.name));
        if (meta)
            rows.push(meta);
    }
    rows.sort(function (a, b) { return a.extensionId.localeCompare(b.extensionId); });
    return rows;
}
function writeWatchMarker(installRoot, event) {
    (0, node_fs_1.mkdirSync)(installRoot, { recursive: true });
    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(installRoot, exports.EXTENSION_WATCH_MARKER), "".concat(JSON.stringify(__assign(__assign({}, event), { emittedAt: Date.now() })), "\n"));
}
/** @emoji 🏪 Creates an extension store rooted at `installRoot`, using `materializer` for browser or native layouts. */
function createExtensionStore(options) {
    var installRoot = options.installRoot, repoRoot = options.repoRoot, materializer = options.materializer;
    var preview2VendorDir = (0, node_path_1.join)(installRoot, ____ts_4.PREVIEW2_VENDOR_RELATIVE);
    var materializeCtx = { repoRoot: repoRoot, preview2VendorDir: preview2VendorDir };
    function materializeInstalled(manifest, wasmBytes, assets, packageHash) {
        return __awaiter(this, void 0, void 0, function () {
            var outDir, existing, siblings, moduleUrl, installedAt, record;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        (0, ____ts_2.installationDirectoryEmoji)(manifest.directoryName);
                        if ((0, node_fs_1.existsSync)(installRoot) && (0, node_fs_1.lstatSync)(installRoot).isSymbolicLink())
                            throw new Error("Extension install root must not be a symlink");
                        outDir = (0, node_path_1.join)(installRoot, manifest.directoryName);
                        existing = (0, node_fs_1.existsSync)(outDir) ? readInstallMeta(outDir) : undefined;
                        if ((0, node_fs_1.existsSync)(outDir) && (existing === null || existing === void 0 ? void 0 : existing.extensionId) !== manifest.extensionId)
                            throw new Error("Extension directory is not owned by this public identity");
                        siblings = (0, node_fs_1.existsSync)(installRoot) ? (0, node_fs_1.readdirSync)(installRoot).filter(function (name) { return name !== manifest.directoryName; }) : [];
                        if ((0, ____ts_2.installationDirectoryCollision)(manifest.directoryName, siblings))
                            throw new Error("Extension directory conflicts with a sibling emoji");
                        if (scanInstalledExtensions(installRoot).some(function (entry) { return entry.extensionId === manifest.extensionId && entry.directoryName !== manifest.directoryName; }))
                            throw new Error("Extension identity already owns a different directory");
                        if (existing)
                            (0, node_fs_1.rmSync)(outDir, { recursive: true, force: true });
                        (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
                        return [4 /*yield*/, materializer({ extensionId: manifest.extensionId, directoryName: manifest.directoryName, version: manifest.version, wasmBytes: wasmBytes, assets: assets, outDir: outDir, materializeCtx: materializeCtx })];
                    case 1:
                        moduleUrl = (_a.sent()).moduleUrl;
                        installedAt = Date.now();
                        record = {
                            extensionId: manifest.extensionId,
                            directoryName: manifest.directoryName,
                            version: manifest.version,
                            label: manifest.label,
                            extends: manifest.extends,
                            moduleUrl: moduleUrl,
                            packageHash: packageHash,
                            installedAt: installedAt,
                        };
                        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(outDir, exports.EXTENSION_INSTALL_META), "".concat(JSON.stringify(record, null, 2), "\n"));
                        writeWatchMarker(installRoot, { kind: "installed", extensionId: manifest.extensionId, version: manifest.version, installedAt: installedAt });
                        return [2 /*return*/, { extensionId: manifest.extensionId, version: manifest.version, moduleUrl: moduleUrl }];
                }
            });
        });
    }
    return {
        installRoot: installRoot,
        installFromBytes: function (bytes) {
            return __awaiter(this, void 0, void 0, function () {
                var unpacked;
                return __generator(this, function (_a) {
                    unpacked = unpackExtensionPackage(bytes);
                    return [2 /*return*/, materializeInstalled(unpacked.manifest, unpacked.wasmBytes, unpacked.assets, unpacked.packageHash)];
                });
            });
        },
        installFromUrl: function (url) {
            return __awaiter(this, void 0, void 0, function () {
                var response, bytes, _a;
                return __generator(this, function (_b) {
                    switch (_b.label) {
                        case 0: return [4 /*yield*/, fetch(url)];
                        case 1:
                            response = _b.sent();
                            if (!response.ok)
                                throw new Error("extension download failed (".concat(response.status, ") for ").concat(url));
                            _a = Uint8Array.bind;
                            return [4 /*yield*/, response.arrayBuffer()];
                        case 2:
                            bytes = new (_a.apply(Uint8Array, [void 0, _b.sent()]))();
                            return [2 /*return*/, this.installFromBytes(bytes)];
                    }
                });
            });
        },
        uninstall: function (extensionId) {
            return __awaiter(this, void 0, void 0, function () {
                var installed;
                return __generator(this, function (_a) {
                    if ((0, node_fs_1.existsSync)(installRoot) && (0, node_fs_1.lstatSync)(installRoot).isSymbolicLink())
                        throw new Error("Extension install root must not be a symlink");
                    installed = scanInstalledExtensions(installRoot).filter(function (entry) { return entry.extensionId === extensionId; });
                    if (installed.length > 1)
                        throw new Error("Extension identity owns multiple directories");
                    if (installed[0])
                        (0, node_fs_1.rmSync)((0, node_path_1.join)(installRoot, installed[0].directoryName), { recursive: true, force: true });
                    writeWatchMarker(installRoot, { kind: "uninstalled", extensionId: extensionId });
                    return [2 /*return*/];
                });
            });
        },
        listInstalled: function () {
            return __awaiter(this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, scanInstalledExtensions(installRoot)];
                });
            });
        },
    };
}
/** @emoji 🗂️ Default dev install directory beside `🔌️plugin-modules`. */
function defaultExtensionInstallRoot(repoRoot) {
    return (0, node_path_1.join)(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules");
}
//#endregion 🔖️Store
function readRequestBody(req) {
    return new Promise(function (resolve, reject) {
        var chunks = [];
        req.on("data", function (chunk) { return chunks.push(Buffer.from(chunk)); });
        req.on("end", function () { return resolve(Buffer.concat(chunks)); });
        req.on("error", function (error) { return reject(error); });
    });
}
//#region 🔌️ExtensionStoreVitePlugin
/** @emoji 🔌 Vite middleware: `POST /🧩️extension-modules/install`, `GET /🧩️extension-modules/watch` SSE (mirrors plugin hot-swap). */
function semioExtensionStoreVitePlugin(options) {
    var _a;
    var store = createExtensionStore({
        installRoot: options.installRoot,
        repoRoot: options.repoRoot,
        materializer: (_a = options.materializer) !== null && _a !== void 0 ? _a : exports.webMaterialize,
    });
    return {
        name: "semio-extension-store",
        configureServer: function (server) {
            var _this = this;
            var subscribers = new Set();
            (0, node_fs_1.mkdirSync)(store.installRoot, { recursive: true });
            var markerPath = (0, node_path_1.join)(store.installRoot, exports.EXTENSION_WATCH_MARKER);
            var debounceTimer;
            (0, node_fs_1.watch)(store.installRoot, function (_eventType, filename) {
                if (filename !== exports.EXTENSION_WATCH_MARKER)
                    return;
                if (debounceTimer)
                    clearTimeout(debounceTimer);
                debounceTimer = setTimeout(function () {
                    if (!(0, node_fs_1.existsSync)(markerPath))
                        return;
                    var marker;
                    try {
                        marker = JSON.parse((0, node_fs_1.readFileSync)(markerPath, "utf8"));
                    }
                    catch (_a) {
                        return;
                    }
                    var _ignored = marker.emittedAt, event = __rest(marker, ["emittedAt"]);
                    var payload = "data: ".concat(JSON.stringify(event), "\n\n");
                    for (var _i = 0, subscribers_1 = subscribers; _i < subscribers_1.length; _i++) {
                        var sub = subscribers_1[_i];
                        sub.write(payload);
                    }
                }, FOLDER_WATCH_DEBOUNCE_MS);
            });
            server.middlewares.use(function (req, res, next) { return __awaiter(_this, void 0, void 0, function () {
                var requestPath, snapshot, contentType, result, body, _a, _b, bytes, error_1;
                var _c;
                var _d, _e, _f;
                return __generator(this, function (_g) {
                    switch (_g.label) {
                        case 0:
                            requestPath = (0, ____ts_3.moduleRoutePath)((_d = req.url) !== null && _d !== void 0 ? _d : "");
                            if (!(requestPath === exports.EXTENSION_WATCH_PATH && req.method === "GET")) return [3 /*break*/, 2];
                            res.statusCode = 200;
                            res.setHeader("content-type", "text/event-stream");
                            res.setHeader("cache-control", "no-cache");
                            res.setHeader("connection", "keep-alive");
                            res.write(": connected\n\n");
                            _c = { kind: "snapshot" };
                            return [4 /*yield*/, store.listInstalled()];
                        case 1:
                            snapshot = (_c.extensions = _g.sent(), _c);
                            res.write("data: ".concat(JSON.stringify(snapshot), "\n\n"));
                            subscribers.add(res);
                            req.on("close", function () { return subscribers.delete(res); });
                            return [2 /*return*/];
                        case 2:
                            if (requestPath !== exports.EXTENSION_INSTALL_PATH || req.method !== "POST")
                                return [2 /*return*/, next()];
                            _g.label = 3;
                        case 3:
                            _g.trys.push([3, 10, , 11]);
                            contentType = (_f = (_e = req.headers) === null || _e === void 0 ? void 0 : _e["content-type"]) !== null && _f !== void 0 ? _f : "";
                            result = void 0;
                            if (!contentType.includes("application/json")) return [3 /*break*/, 6];
                            _b = (_a = JSON).parse;
                            return [4 /*yield*/, readRequestBody(req)];
                        case 4:
                            body = _b.apply(_a, [(_g.sent()).toString("utf8")]);
                            if (!body.url)
                                throw new Error("JSON body must include { url }");
                            return [4 /*yield*/, store.installFromUrl(body.url)];
                        case 5:
                            result = _g.sent();
                            return [3 /*break*/, 9];
                        case 6: return [4 /*yield*/, readRequestBody(req)];
                        case 7:
                            bytes = _g.sent();
                            if (bytes.length === 0)
                                throw new Error("empty extension package body");
                            return [4 /*yield*/, store.installFromBytes(new Uint8Array(bytes))];
                        case 8:
                            result = _g.sent();
                            _g.label = 9;
                        case 9:
                            res.statusCode = 200;
                            res.setHeader("content-type", "application/json");
                            res.end(JSON.stringify(result));
                            return [3 /*break*/, 11];
                        case 10:
                            error_1 = _g.sent();
                            res.statusCode = 400;
                            res.setHeader("content-type", "application/json");
                            res.end(JSON.stringify({ error: error_1 instanceof Error ? error_1.message : String(error_1) }));
                            return [3 /*break*/, 11];
                        case 11: return [2 /*return*/];
                    }
                });
            }); });
        },
    };
}
//#endregion 🔌️ExtensionStoreVitePlugin
//#region 🧪️Tests
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("../🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { EXTENSION_COMPONENT_FILE: exports.EXTENSION_COMPONENT_FILE, EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI: exports.EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, EXTENSION_PACKAGE_ENVELOPE_TOKEN: exports.EXTENSION_PACKAGE_ENVELOPE_TOKEN, MODULE_EXTENSION_ROUTE: ____ts_3.MODULE_EXTENSION_ROUTE, createExtensionStore: createExtensionStore, decodeOwnedZip: ____ts_1.decodeOwnedZip, decodePackValue: framework_os_1.decodePackValue, existsSync: node_fs_1.existsSync, extensionPackageContentHash: extensionPackageContentHash, installationDirectoryCollision: ____ts_2.installationDirectoryCollision, installationDirectoryEmoji: ____ts_2.installationDirectoryEmoji, join: node_path_1.join, mkdtempSync: node_fs_1.mkdtempSync, packExtensionPackage: packExtensionPackage, readFileSync: node_fs_1.readFileSync, rmSync: node_fs_1.rmSync, tmpdir: node_os_1.tmpdir, unpackExtensionPackage: unpackExtensionPackage, wrapExtensionPackageEnvelope: wrapExtensionPackageEnvelope }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
