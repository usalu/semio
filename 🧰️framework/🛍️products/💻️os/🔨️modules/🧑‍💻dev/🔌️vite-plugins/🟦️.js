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
exports.AGENT_BRIDGE_RENDEZVOUS_DIR_ENV = exports.AGENT_BRIDGE_OFFER_ENDPOINT_PATH = exports.AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION = exports.UNWATCHED_REPOSITORY_SEGMENTS = exports.PLUGIN_SOURCE_WATCH_PATH = exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES = exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = exports.REPO_ROOT = void 0;
exports.descriptorRouteDecision = descriptorRouteDecision;
exports.semioDescriptorRouteGuardVitePlugin = semioDescriptorRouteGuardVitePlugin;
exports.reserveCanonicalBootstrapFolderMirror = reserveCanonicalBootstrapFolderMirror;
exports.stageCanonicalBootstrapFolderMirror = stageCanonicalBootstrapFolderMirror;
exports.publishCanonicalBootstrapFolderMirror = publishCanonicalBootstrapFolderMirror;
exports.retireCanonicalBootstrapFolderMirror = retireCanonicalBootstrapFolderMirror;
exports.backboneDbHandleFor = backboneDbHandleFor;
exports.readBackbonePayload = readBackbonePayload;
exports.writeBackbonePayload = writeBackbonePayload;
exports.semioProductionTestBoundaryVitePlugin = semioProductionTestBoundaryVitePlugin;
exports.semioBackboneVitePlugin = semioBackboneVitePlugin;
exports.scanBuiltPluginModules = scanBuiltPluginModules;
exports.semioPluginHotSwapVitePlugin = semioPluginHotSwapVitePlugin;
exports.reportActivationFreshness = reportActivationFreshness;
exports.semioActivationVitePlugin = semioActivationVitePlugin;
exports.semioBlobVitePlugin = semioBlobVitePlugin;
exports.repositorySourceWatchRoots = repositorySourceWatchRoots;
exports.unwatchedRepositoryPathMatcher = unwatchedRepositoryPathMatcher;
exports.semioPlaygroundReactRefreshCoherenceVitePlugin = semioPlaygroundReactRefreshCoherenceVitePlugin;
exports.createSourceFreshnessRegistry = createSourceFreshnessRegistry;
exports.semioSourceWatchVitePlugin = semioSourceWatchVitePlugin;
exports.requestedTransformFile = requestedTransformFile;
exports.semioTransformFreshnessVitePlugin = semioTransformFreshnessVitePlugin;
exports.semioSourceFreshnessVitePlugins = semioSourceFreshnessVitePlugins;
exports.newestLiveAgentBridgeOffer = newestLiveAgentBridgeOffer;
exports.semioAgentBridgeRendezvousVitePlugin = semioAgentBridgeRendezvousVitePlugin;
/** @emoji 🔌️ The dev server's own Vite plugins — backbone document IO, the content-addressed blob
 * endpoint, the plugin hot-swap SSE stream and the production test boundary — kept in a module of
 * their own so `⚙️vite.config.ts` can mount them without pulling `📜️script.ts`'s task router (and
 * through it the repository library's discovery walk) into Vite's config bundle. `bun:sqlite` stays a
 * lazy dynamic import: Vite loads this module's exports under Node before the dev server's Bun
 * runtime exists. */
var node_crypto_1 = require("node:crypto");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
var framework_os_1 = require("@semio-tech/framework-os");
var ____ts_1 = require("../../\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../\u267B\uFE0Factivation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../\uD83D\uDD0C\uFE0Fplugin/\uD83C\uDFEA\uFE0Fstore/\uD83D\uDCE5\uFE0Finstallation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../../../../../\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0F\uFE0Fhash/\uD83D\uDFE6\uFE0F.ts");
/** @emoji 🗂️ Repository root derived from this module's own location — the config bundler must not
 * reach `getWorkspaceRoot` (and the discovery walk behind it) just to place two dev databases. */
exports.REPO_ROOT = (0, node_path_1.resolve)((0, node_path_1.dirname)((0, node_url_1.fileURLToPath)(import.meta.url)), "../../../../../../..");
/** @emoji 🛂️ Resolves only canonical declared module descriptor requests, without decoding arbitrary filesystem paths. */
function descriptorRouteDecision(url, specs) {
    var _a;
    if (!url)
        return { kind: "pass" };
    var pathname;
    try {
        pathname = decodeURIComponent(new URL(url, "http://127.0.0.1").pathname);
    }
    catch (_b) {
        return { kind: "pass" };
    }
    for (var _i = 0, specs_1 = specs; _i < specs_1.length; _i++) {
        var spec = specs_1[_i];
        var route = spec.route.endsWith("/") ? spec.route : "".concat(spec.route, "/");
        if (!pathname.startsWith(route))
            continue;
        var parts = pathname.slice(route.length).split("/");
        if (parts.length !== 2 || parts[1] !== "🔣️.json")
            return { kind: "pass" };
        var moduleDirectory = (_a = parts[0]) !== null && _a !== void 0 ? _a : "";
        if (!spec.directoryNames.has(moduleDirectory) || !(0, node_fs_1.existsSync)((0, node_path_1.join)(spec.root, moduleDirectory, "🔣️.json")))
            return { kind: "missing", moduleDirectory: moduleDirectory };
        return { kind: "pass" };
    }
    return { kind: "pass" };
}
/** @emoji 🚫️ Prevents a missing plugin descriptor from falling through to Vite's HTML SPA response. */
function semioDescriptorRouteGuardVitePlugin(specs) {
    return {
        name: "semio-descriptor-route-guard",
        enforce: "pre",
        configureServer: function (server) {
            server.middlewares.use(function (req, res, next) {
                var decision = descriptorRouteDecision(req.url, specs);
                if (decision.kind === "pass")
                    return next();
                res.statusCode = 404;
                res.setHeader("content-type", "application/json");
                res.end("".concat(JSON.stringify({ error: "descriptor-not-found", moduleDirectory: decision.moduleDirectory }), "\n"));
            });
        },
    };
}
//#region BackboneVitePlugin
/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
var backboneDatabaseCtor;
function backboneDatabaseCtorLazy() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!!backboneDatabaseCtor) return [3 /*break*/, 2];
                    return [4 /*yield*/, Promise.resolve().then(function () { return require("bun:sqlite"); })];
                case 1:
                    (backboneDatabaseCtor = (_a.sent()).Database);
                    _a.label = 2;
                case 2: return [2 /*return*/, backboneDatabaseCtor];
            }
        });
    });
}
exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = "".concat(framework_os_1.BACKBONE_ENDPOINT_PATH, "/canonical-bootstrap");
exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES = framework_os_1.DOCUMENT_ARCHIVE_MAXIMUM_BYTES;
var CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES = exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES + 10;
var SHA256_HEX = /^[0-9a-f]{64}$/u;
function canonicalBootstrapMirrorError(code) {
    return Object.assign(new Error("canonical bootstrap folder mirror ".concat(code)), { code: code });
}
function canonicalBootstrapFolderMirrorDbPath(uri) {
    var folder = uri.slice("folder://".length);
    if ((0, framework_os_1.backboneKindFromUri)(uri) !== "folder" || !(0, node_path_1.isAbsolute)(folder))
        throw canonicalBootstrapMirrorError("invalid");
    return (0, node_path_1.join)(folder, ".semio", "documents.db");
}
function validateCanonicalBootstrapDocumentId(documentId) {
    if (documentId.length === 0 || new TextEncoder().encode(documentId).byteLength > 512 || documentId.includes("\0"))
        throw canonicalBootstrapMirrorError("invalid");
}
function validateCanonicalBootstrapFrontier(frontier, documentId) {
    var _a;
    var chainIsCanonical = SHA256_HEX.test((_a = frontier === null || frontier === void 0 ? void 0 : frontier.chainSha256) !== null && _a !== void 0 ? _a : "");
    var chainIsZero = chainIsCanonical && frontier.chainSha256 === "0".repeat(64);
    var genesis = (frontier === null || frontier === void 0 ? void 0 : frontier.documentId) === documentId &&
        frontier.headEditOrdinal === 0 &&
        frontier.headEditId === "" &&
        frontier.lastCommitSeq === 0 &&
        chainIsZero;
    var edited = (frontier === null || frontier === void 0 ? void 0 : frontier.documentId) === documentId &&
        Number.isSafeInteger(frontier.headEditOrdinal) &&
        frontier.headEditOrdinal > 0 &&
        Number.isSafeInteger(frontier.lastCommitSeq) &&
        frontier.lastCommitSeq > 0 &&
        typeof frontier.headEditId === "string" &&
        frontier.headEditId.length > 0 &&
        new TextEncoder().encode(frontier.headEditId).byteLength <= 512 &&
        !/\p{Cc}/u.test(frontier.headEditId) &&
        chainIsCanonical &&
        !chainIsZero;
    if (!genesis && !edited)
        throw canonicalBootstrapMirrorError("invalid");
}
function validateCanonicalBootstrapReserve(request, documentId) {
    if ((request === null || request === void 0 ? void 0 : request.schema) !== "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1" ||
        typeof request.artifactSchema !== "string" ||
        request.artifactSchema.length === 0 ||
        !SHA256_HEX.test(request.descriptorDigestV1) ||
        !SHA256_HEX.test(request.aggregateSha256))
        throw canonicalBootstrapMirrorError("invalid");
    validateCanonicalBootstrapFrontier(request.baselineFrontier, documentId);
}
function validateCanonicalBootstrapControl(control) {
    if (!Number.isSafeInteger(control === null || control === void 0 ? void 0 : control.epoch) || control.epoch < 1 || typeof control.capability !== "string" || !SHA256_HEX.test(control.capability))
        throw canonicalBootstrapMirrorError("invalid");
}
function canonicalBootstrapFolderMirrorDb(uri) {
    return __awaiter(this, void 0, void 0, function () {
        var dbPath;
        return __generator(this, function (_a) {
            dbPath = canonicalBootstrapFolderMirrorDbPath(uri);
            (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(dbPath), { recursive: true });
            return [2 /*return*/, backboneDbHandleFor(dbPath)];
        });
    });
}
/** 🪪️ Mints and persists the sole current folder-mirror epoch; reserving a successor makes every prior stage and published pair non-current in the same transaction. */
function reserveCanonicalBootstrapFolderMirror(uri, documentId, request) {
    return __awaiter(this, void 0, void 0, function () {
        var db, capability, reserve;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    validateCanonicalBootstrapDocumentId(documentId);
                    validateCanonicalBootstrapReserve(request, documentId);
                    return [4 /*yield*/, canonicalBootstrapFolderMirrorDb(uri)];
                case 1:
                    db = _a.sent();
                    capability = (0, node_crypto_1.randomBytes)(32).toString("hex");
                    reserve = db.transaction(function () {
                        var _a;
                        var prior = db.query("SELECT epoch FROM canonical_bootstrap_owner WHERE document_id = ?1").get(documentId);
                        var epoch = Number((_a = prior === null || prior === void 0 ? void 0 : prior.epoch) !== null && _a !== void 0 ? _a : 0) + 1;
                        if (!Number.isSafeInteger(epoch))
                            throw canonicalBootstrapMirrorError("conflict");
                        db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1", [documentId]);
                        db.run("INSERT INTO canonical_bootstrap_owner (document_id, epoch, capability, state, artifact_schema, descriptor_digest_v1, aggregate_sha256, baseline_frontier_json, updated_at) VALUES (?1, ?2, ?3, 'reserved', ?4, ?5, ?6, ?7, ?8) ON CONFLICT(document_id) DO UPDATE SET epoch = excluded.epoch, capability = excluded.capability, state = excluded.state, artifact_schema = excluded.artifact_schema, descriptor_digest_v1 = excluded.descriptor_digest_v1, aggregate_sha256 = excluded.aggregate_sha256, baseline_frontier_json = excluded.baseline_frontier_json, updated_at = excluded.updated_at", [documentId, epoch, capability, request.artifactSchema, request.descriptorDigestV1, request.aggregateSha256, JSON.stringify(request.baselineFrontier), Date.now()]);
                        return epoch;
                    });
                    return [2 /*return*/, { schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: reserve(), capability: capability }];
            }
        });
    });
}
/** 🧱️ Stages one bounded, exactly framed canonical recursive archive without changing folder-visible state. */
function stageCanonicalBootstrapFolderMirror(uri, documentId, control, payload) {
    return __awaiter(this, void 0, void 0, function () {
        var pack, spr, archive, aggregate, db, stage;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    validateCanonicalBootstrapDocumentId(documentId);
                    validateCanonicalBootstrapControl(control);
                    if (payload.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES)
                        throw canonicalBootstrapMirrorError("too-large");
                    try {
                        archive = (0, framework_os_1.decodeDocumentArchiveBytes)(payload);
                        pack = Uint8Array.from(archive.parent_pack);
                        spr = Uint8Array.from(archive.parent_spr);
                    }
                    catch (_b) {
                        throw canonicalBootstrapMirrorError("invalid");
                    }
                    if (pack.byteLength + spr.byteLength > exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES)
                        throw canonicalBootstrapMirrorError("too-large");
                    aggregate = (0, node_crypto_1.createHash)("sha256").update(pack).update(spr).digest("hex");
                    return [4 /*yield*/, canonicalBootstrapFolderMirrorDb(uri)];
                case 1:
                    db = _a.sent();
                    stage = db.transaction(function () {
                        var owner = db.query("SELECT aggregate_sha256 AS aggregateSha256 FROM canonical_bootstrap_owner WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'").get(documentId, control.epoch, control.capability);
                        if ((owner === null || owner === void 0 ? void 0 : owner.aggregateSha256) !== aggregate)
                            throw canonicalBootstrapMirrorError("conflict");
                        db.run("INSERT INTO canonical_bootstrap_archive_stage (document_id, epoch, archive, aggregate_sha256) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(document_id, epoch) DO UPDATE SET archive = excluded.archive, aggregate_sha256 = excluded.aggregate_sha256", [documentId, control.epoch, payload, aggregate]);
                    });
                    stage();
                    return [2 /*return*/];
            }
        });
    });
}
/** 📣️ Makes a staged pair current only while the exact server-minted epoch remains reserved. */
function publishCanonicalBootstrapFolderMirror(uri, documentId, control) {
    return __awaiter(this, void 0, void 0, function () {
        var db, publish;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    validateCanonicalBootstrapDocumentId(documentId);
                    validateCanonicalBootstrapControl(control);
                    return [4 /*yield*/, canonicalBootstrapFolderMirrorDb(uri)];
                case 1:
                    db = _a.sent();
                    publish = db.transaction(function () {
                        var stage = db
                            .query("SELECT 1 AS present FROM canonical_bootstrap_owner owner JOIN canonical_bootstrap_archive_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1 AND owner.epoch = ?2 AND owner.capability = ?3 AND owner.state = 'reserved' AND stage.aggregate_sha256 = owner.aggregate_sha256")
                            .get(documentId, control.epoch, control.capability);
                        if ((stage === null || stage === void 0 ? void 0 : stage.present) !== 1)
                            throw canonicalBootstrapMirrorError("conflict");
                        var changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'published', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'", [documentId, control.epoch, control.capability, Date.now()]);
                        if (changed.changes !== 1)
                            throw canonicalBootstrapMirrorError("conflict");
                    });
                    publish();
                    return [2 /*return*/];
            }
        });
    });
}
/** 🧹️ Retires only the exact current epoch; a stale owner cannot hide or alter its successor. */
function retireCanonicalBootstrapFolderMirror(uri, documentId, control) {
    return __awaiter(this, void 0, void 0, function () {
        var db, retire;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    validateCanonicalBootstrapDocumentId(documentId);
                    validateCanonicalBootstrapControl(control);
                    return [4 /*yield*/, canonicalBootstrapFolderMirrorDb(uri)];
                case 1:
                    db = _a.sent();
                    retire = db.transaction(function () {
                        var changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'retired', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state IN ('reserved', 'published')", [documentId, control.epoch, control.capability, Date.now()]);
                        if (changed.changes !== 1)
                            throw canonicalBootstrapMirrorError("conflict");
                        db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1 AND epoch = ?2", [documentId, control.epoch]);
                    });
                    retire();
                    return [2 /*return*/];
            }
        });
    });
}
/** @emoji 🗄️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): per-path `bun:sqlite` handle cache.
 * `readBackbonePayload`/`writeBackbonePayload` used to `new Database(dbPath)` — and re-run the
 * (idempotent but non-free) `CREATE TABLE IF NOT EXISTS` — on EVERY single read/write request, so a
 * hot dev-editing loop against one folder-backed document reopened the same file every keystroke's
 * autosave. Lifetime: opened once, then held open for the lifetime of THIS dev-server process — never
 * explicitly closed or evicted. A dev session only ever touches a handful of distinct folder URIs (the
 * open studio, plus maybe one or two app documents), so the cache's total size is bounded by session
 * variety, not by request volume; there is no observed need for a size/idle eviction policy for that few
 * long-lived, cheap-to-hold connections. If that assumption ever stops holding (e.g. a scripted session
 * that iterates many distinct folders), add one then — not speculatively here. */
var backboneDbHandles = new Map();
function backboneDbHandleFor(dbPath) {
    return __awaiter(this, void 0, void 0, function () {
        var existing, Database, db;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    existing = backboneDbHandles.get(dbPath);
                    if (existing)
                        return [2 /*return*/, existing];
                    return [4 /*yield*/, backboneDatabaseCtorLazy()];
                case 1:
                    Database = _a.sent();
                    db = new Database(dbPath);
                    db.run("CREATE TABLE IF NOT EXISTS document_archive (id TEXT PRIMARY KEY, schema TEXT, archive BLOB NOT NULL, updated_at INTEGER NOT NULL)");
                    db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_owner (document_id TEXT PRIMARY KEY, epoch INTEGER NOT NULL, capability TEXT NOT NULL, state TEXT NOT NULL CHECK (state IN ('reserved', 'published', 'retired', 'generic')), artifact_schema TEXT NOT NULL, descriptor_digest_v1 TEXT NOT NULL, aggregate_sha256 TEXT NOT NULL, baseline_frontier_json TEXT NOT NULL, updated_at INTEGER NOT NULL)");
                    db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_archive_stage (document_id TEXT NOT NULL, epoch INTEGER NOT NULL, archive BLOB NOT NULL, aggregate_sha256 TEXT NOT NULL, PRIMARY KEY (document_id, epoch))");
                    backboneDbHandles.set(dbPath, db);
                    return [2 /*return*/, db];
            }
        });
    });
}
/** @emoji 🗂️ Same `.semio/documents.db` convention as `vcs::FolderSqliteStorage` so a folder-bound studio opened by the browser dev path and a
 * native (wgpu) reader agree on the same file. `documentId` defaults to the studio's own
 * single-document convention (mirrors os-core's `SPACE_FOLDER_DOCUMENT_ID`) when the caller doesn't
 * pass one — app documents (per `OsDocumentRef`) always pass their own id explicitly. */
var SPACE_FOLDER_DOCUMENT_ID = "studio";
function readBackbonePayload(uri, documentId) {
    return __awaiter(this, void 0, void 0, function () {
        var kind, path, folder, dbPath, db, canonical, bytes_1, archive, pack, spr, row, bytes;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    kind = (0, framework_os_1.backboneKindFromUri)(uri);
                    if (kind === "file") {
                        path = uri.slice("file://".length);
                        if (!(0, node_fs_1.existsSync)(path))
                            return [2 /*return*/, null];
                        return [2 /*return*/, new Uint8Array((0, node_fs_1.readFileSync)(path))];
                    }
                    if (!(kind === "folder")) return [3 /*break*/, 2];
                    folder = uri.slice("folder://".length);
                    dbPath = (0, node_path_1.join)(folder, ".semio", "documents.db");
                    if (!(0, node_fs_1.existsSync)(dbPath))
                        return [2 /*return*/, null];
                    return [4 /*yield*/, backboneDbHandleFor(dbPath)];
                case 1:
                    db = _a.sent();
                    canonical = db
                        .query("SELECT owner.state, owner.aggregate_sha256 AS expectedAggregate, stage.aggregate_sha256 AS stagedAggregate, stage.archive FROM canonical_bootstrap_owner owner LEFT JOIN canonical_bootstrap_archive_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1")
                        .get(documentId !== null && documentId !== void 0 ? documentId : SPACE_FOLDER_DOCUMENT_ID);
                    if (canonical && canonical.state !== "generic") {
                        if (canonical.state !== "published" || !canonical.archive)
                            return [2 /*return*/, null];
                        bytes_1 = canonical.archive instanceof Uint8Array ? canonical.archive : new Uint8Array(canonical.archive);
                        archive = (0, framework_os_1.decodeDocumentArchiveBytes)(bytes_1);
                        pack = Uint8Array.from(archive.parent_pack), spr = Uint8Array.from(archive.parent_spr);
                        if (canonical.stagedAggregate !== canonical.expectedAggregate || (0, node_crypto_1.createHash)("sha256").update(pack).update(spr).digest("hex") !== canonical.expectedAggregate)
                            return [2 /*return*/, null];
                        return [2 /*return*/, bytes_1];
                    }
                    row = db.query("SELECT archive FROM document_archive WHERE id = ?1").get(documentId !== null && documentId !== void 0 ? documentId : SPACE_FOLDER_DOCUMENT_ID);
                    if (!(row === null || row === void 0 ? void 0 : row.archive))
                        return [2 /*return*/, null];
                    bytes = row.archive instanceof Uint8Array ? row.archive : new Uint8Array(row.archive);
                    (0, framework_os_1.decodeDocumentArchiveBytes)(bytes);
                    return [2 /*return*/, bytes];
                case 2: return [2 /*return*/, null];
            }
        });
    });
}
function writeBackbonePayload(uri, documentId, schema, payload) {
    return __awaiter(this, void 0, void 0, function () {
        var kind, path, folder, dbPath, db_1, id_1, write;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    kind = (0, framework_os_1.backboneKindFromUri)(uri);
                    (0, framework_os_1.decodeDocumentArchiveBytes)(payload);
                    if (kind === "file") {
                        path = uri.slice("file://".length);
                        (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(path), { recursive: true });
                        (0, node_fs_1.writeFileSync)(path, payload);
                        return [2 /*return*/];
                    }
                    if (!(kind === "folder")) return [3 /*break*/, 2];
                    folder = uri.slice("folder://".length);
                    dbPath = (0, node_path_1.join)(folder, ".semio", "documents.db");
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(dbPath), { recursive: true });
                    return [4 /*yield*/, backboneDbHandleFor(dbPath)];
                case 1:
                    db_1 = _a.sent();
                    id_1 = documentId !== null && documentId !== void 0 ? documentId : SPACE_FOLDER_DOCUMENT_ID;
                    write = db_1.transaction(function () {
                        var canonical = db_1.query("SELECT state FROM canonical_bootstrap_owner WHERE document_id = ?1").get(id_1);
                        if ((canonical === null || canonical === void 0 ? void 0 : canonical.state) === "reserved" || (canonical === null || canonical === void 0 ? void 0 : canonical.state) === "published")
                            throw canonicalBootstrapMirrorError("conflict");
                        if ((canonical === null || canonical === void 0 ? void 0 : canonical.state) === "retired") {
                            db_1.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1", [id_1]);
                            db_1.run("UPDATE canonical_bootstrap_owner SET state = 'generic', updated_at = ?2 WHERE document_id = ?1 AND state = 'retired'", [id_1, Date.now()]);
                        }
                        db_1.run("INSERT INTO document_archive (id, schema, archive, updated_at) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, archive = excluded.archive, updated_at = excluded.updated_at", [id_1, schema !== null && schema !== void 0 ? schema : "", payload, Date.now()]);
                    });
                    write();
                    return [2 /*return*/];
                case 2: throw new Error("unsupported backbone uri: ".concat(uri));
            }
        });
    });
}
/** 👁️ Per-folder-uri debounced watchers feeding every subscribed SSE response for that uri — one
 * `node:fs.watch` per folder regardless of subscriber count. Mirrors `store_sync`'s native
 * `notify` watcher (200ms debounce) so both the dev-browser and native paths agree on cadence. */
var folderWatchSubscribers = new Map();
var folderWatchHandles = new Map();
var FOLDER_WATCH_DEBOUNCE_MS = 200;
function subscribeFolderWatch(uri, subscriber) {
    if (!folderWatchSubscribers.has(uri))
        folderWatchSubscribers.set(uri, new Set());
    var subscribers = folderWatchSubscribers.get(uri);
    subscribers.add(subscriber);
    if (!folderWatchHandles.has(uri) && (0, framework_os_1.backboneKindFromUri)(uri) === "folder") {
        var folder = uri.slice("folder://".length);
        (0, node_fs_1.mkdirSync)((0, node_path_1.join)(folder, ".semio"), { recursive: true });
        var debounceTimer_1;
        var handle = (0, node_fs_1.watch)((0, node_path_1.join)(folder, ".semio"), { persistent: false }, function () {
            if (debounceTimer_1)
                clearTimeout(debounceTimer_1);
            debounceTimer_1 = setTimeout(function () {
                var _a;
                for (var _i = 0, _b = (_a = folderWatchSubscribers.get(uri)) !== null && _a !== void 0 ? _a : []; _i < _b.length; _i++) {
                    var sub = _b[_i];
                    sub.write("data: changed\n\n");
                }
            }, FOLDER_WATCH_DEBOUNCE_MS);
        });
        folderWatchHandles.set(uri, handle);
    }
    return function () {
        var _a;
        subscribers.delete(subscriber);
        if (subscribers.size === 0) {
            (_a = folderWatchHandles.get(uri)) === null || _a === void 0 ? void 0 : _a.close();
            folderWatchHandles.delete(uri);
            folderWatchSubscribers.delete(uri);
        }
    };
}
function canonicalBootstrapMirrorControlFromHeaders(headers) {
    var epochSource = headers === null || headers === void 0 ? void 0 : headers["x-semio-canonical-bootstrap-epoch"];
    var authorization = headers === null || headers === void 0 ? void 0 : headers.authorization;
    if (Array.isArray(epochSource) || Array.isArray(authorization) || !/^[1-9][0-9]*$/u.test(epochSource !== null && epochSource !== void 0 ? epochSource : "") || !(authorization === null || authorization === void 0 ? void 0 : authorization.startsWith("SemioFolderBootstrap ")))
        throw canonicalBootstrapMirrorError("invalid");
    var epoch = Number(epochSource);
    var control = { epoch: epoch, capability: authorization.slice("SemioFolderBootstrap ".length) };
    validateCanonicalBootstrapControl(control);
    return control;
}
function canonicalBootstrapMirrorContentType(headers) {
    var source = headers === null || headers === void 0 ? void 0 : headers["content-type"];
    return typeof source === "string" ? source.trim().toLowerCase() : null;
}
function canonicalBootstrapMirrorStatus(error) {
    var code = typeof error === "object" && error !== null && "code" in error ? error.code : undefined;
    return code === "conflict" ? 409 : code === "too-large" ? 413 : 400;
}
function collectBackboneRequestBody(req, limit, complete) {
    var chunks = [];
    var length = 0;
    var exceeded = false;
    req.on("data", function (chunk) {
        if (exceeded)
            return;
        var bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(String(chunk));
        length += bytes.byteLength;
        if (length > limit) {
            exceeded = true;
            chunks.length = 0;
            return;
        }
        chunks.push(bytes);
    });
    req.on("end", function () { return complete(exceeded ? null : new Uint8Array(Buffer.concat(chunks))); });
}
/** @emoji 💓️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): both dev SSE endpoints below previously
 * wrote `: connected\n\n` once on connect and nothing else until a real event fired — a quiet dev
 * session (no file edits, no plugin rebuild) could sit for minutes with nothing crossing the wire, which
 * is exactly the shape a browser or an intermediary dev proxy's idle-connection timeout (commonly in the
 * 30-60s range) silently kills with no client-visible `close`/`error` event, leaving the tab's
 * `EventSource` looking "connected" while actually dead. Periodic `: keepalive\n\n` SSE comments (valid
 * per the SSE spec — a line starting with `:` is ignored by `EventSource` but still resets any
 * intermediary's idle timer) fix that. `req.on("close")` already fires reliably on a real disconnect, so
 * clearing this timer there is the only cleanup needed. */
var SSE_KEEPALIVE_INTERVAL_MS = 15000;
function startSseKeepalive(res) {
    var timer = setInterval(function () {
        try {
            res.write(": keepalive\n\n");
        }
        catch (_a) {
            clearInterval(timer);
        }
    }, SSE_KEEPALIVE_INTERVAL_MS);
    return function () { return clearInterval(timer); };
}
/** 🧹️ Eliminates in-source test branches before production asset URL collection. */
function semioProductionTestBoundaryVitePlugin() {
    return {
        name: "semio-production-test-boundary",
        enforce: "pre",
        apply: "build",
        transform: function (source, id) {
            return __awaiter(this, void 0, void 0, function () {
                var transformWithEsbuild, result;
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0:
                            if (!source.includes("import.meta.vitest") || !/\.[cm]?[jt]sx?(?:[?#].*)?$/u.test(id))
                                return [2 /*return*/, null];
                            return [4 /*yield*/, Promise.resolve().then(function () { return require("vite"); })];
                        case 1:
                            transformWithEsbuild = (_a.sent()).transformWithEsbuild;
                            return [4 /*yield*/, transformWithEsbuild(source, id, { define: { "import.meta.vitest": "undefined" }, minifySyntax: true, target: "esnext", charset: "utf8", jsx: "preserve", sourcemap: true })];
                        case 2:
                            result = _a.sent();
                            return [2 /*return*/, { code: result.code, map: JSON.stringify(result.map) }];
                    }
                });
            });
        },
    };
}
/** @emoji 💾️ Vite middleware for browser file/folder backbone IO: `GET|PUT ${BACKBONE_ENDPOINT_PATH}?uri=&documentId=&schema=`
 * for read/write, plus `GET ${BACKBONE_ENDPOINT_PATH}/watch?uri=` (SSE) for external-edit notification —
 * `🏪️store/👷️worker/🟦️.ts`'s folder transport degrades to polling if this endpoint isn't reachable. */
function semioBackboneVitePlugin() {
    return {
        name: "semio-backbone",
        configureServer: function (server) {
            var _this = this;
            server.middlewares.use(function (req, res, next) {
                var _a;
                if (!((_a = req.url) === null || _a === void 0 ? void 0 : _a.startsWith(framework_os_1.BACKBONE_ENDPOINT_PATH)))
                    return next();
                var requestUrl = new URL(req.url, "http://127.0.0.1");
                var uri = requestUrl.searchParams.get("uri");
                if (!uri) {
                    res.statusCode = 400;
                    res.end("missing uri");
                    return;
                }
                if (requestUrl.pathname === "".concat(framework_os_1.BACKBONE_ENDPOINT_PATH, "/watch")) {
                    if (req.method !== "GET") {
                        res.statusCode = 405;
                        res.end("method not allowed");
                        return;
                    }
                    res.statusCode = 200;
                    res.setHeader("content-type", "text/event-stream");
                    res.setHeader("cache-control", "no-cache");
                    res.setHeader("connection", "keep-alive");
                    res.write(": connected\n\n");
                    var stopKeepalive_1 = startSseKeepalive(res);
                    var unsubscribe_1 = subscribeFolderWatch(uri, res);
                    req.on("close", function () {
                        stopKeepalive_1();
                        unsubscribe_1();
                    });
                    return;
                }
                var documentId = requestUrl.searchParams.get("documentId");
                var schema = requestUrl.searchParams.get("schema");
                if (requestUrl.pathname.startsWith("".concat(exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH, "/"))) {
                    var action_1 = requestUrl.pathname.slice(exports.CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH.length + 1);
                    if (documentId === null || (0, framework_os_1.backboneKindFromUri)(uri) !== "folder" || requestUrl.searchParams.size !== 2 || !["reserve", "stage", "publish", "retire"].includes(action_1)) {
                        res.statusCode = 400;
                        res.end("invalid canonical bootstrap folder mirror request");
                        return;
                    }
                    if ((action_1 === "stage" && req.method !== "PUT") || (action_1 !== "stage" && req.method !== "POST")) {
                        res.statusCode = 405;
                        res.end("method not allowed");
                        return;
                    }
                    var expectedContentType = action_1 === "reserve" ? "application/json" : action_1 === "stage" ? "application/octet-stream" : null;
                    if (canonicalBootstrapMirrorContentType(req.headers) !== expectedContentType) {
                        res.statusCode = 415;
                        res.end("unsupported media type");
                        return;
                    }
                    var limit = action_1 === "reserve" ? 8192 : action_1 === "stage" ? CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES : 0;
                    collectBackboneRequestBody(req, limit, function (body) {
                        void (function () { return __awaiter(_this, void 0, void 0, function () {
                            var source, parsed, owner, control;
                            return __generator(this, function (_a) {
                                switch (_a.label) {
                                    case 0:
                                        if (body === null)
                                            throw canonicalBootstrapMirrorError("too-large");
                                        if (!(action_1 === "reserve")) return [3 /*break*/, 2];
                                        source = new TextDecoder("utf-8", { fatal: true }).decode(body);
                                        parsed = JSON.parse(source);
                                        if (JSON.stringify(parsed) !== source)
                                            throw canonicalBootstrapMirrorError("invalid");
                                        return [4 /*yield*/, reserveCanonicalBootstrapFolderMirror(uri, documentId, parsed)];
                                    case 1:
                                        owner = _a.sent();
                                        res.statusCode = 201;
                                        res.setHeader("content-type", "application/json");
                                        res.setHeader("cache-control", "no-store");
                                        res.end("".concat(JSON.stringify(owner), "\n"));
                                        return [2 /*return*/];
                                    case 2:
                                        control = canonicalBootstrapMirrorControlFromHeaders(req.headers);
                                        if (!(action_1 === "stage")) return [3 /*break*/, 4];
                                        return [4 /*yield*/, stageCanonicalBootstrapFolderMirror(uri, documentId, control, body)];
                                    case 3:
                                        _a.sent();
                                        return [3 /*break*/, 8];
                                    case 4:
                                        if (!(action_1 === "publish")) return [3 /*break*/, 6];
                                        return [4 /*yield*/, publishCanonicalBootstrapFolderMirror(uri, documentId, control)];
                                    case 5:
                                        _a.sent();
                                        return [3 /*break*/, 8];
                                    case 6: return [4 /*yield*/, retireCanonicalBootstrapFolderMirror(uri, documentId, control)];
                                    case 7:
                                        _a.sent();
                                        _a.label = 8;
                                    case 8:
                                        res.statusCode = 204;
                                        res.end();
                                        return [2 /*return*/];
                                }
                            });
                        }); })().catch(function (error) {
                            res.statusCode = canonicalBootstrapMirrorStatus(error);
                            res.setHeader("content-type", "application/json");
                            res.end("".concat(JSON.stringify({ error: error instanceof Error ? error.message : "canonical bootstrap folder mirror invalid" }), "\n"));
                        });
                    });
                    return;
                }
                if (req.method === "GET") {
                    readBackbonePayload(uri, documentId)
                        .then(function (payload) {
                        if (payload == null) {
                            res.statusCode = 404;
                            res.end("");
                            return;
                        }
                        res.statusCode = 200;
                        res.setHeader("content-type", "application/octet-stream");
                        res.end(Buffer.from(payload));
                    })
                        .catch(function (error) {
                        res.statusCode = 500;
                        res.end(String(error));
                    });
                    return;
                }
                if (req.method === "PUT") {
                    collectBackboneRequestBody(req, CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES, function (body) {
                        if (body === null) {
                            res.statusCode = 413;
                            res.end("payload too large");
                            return;
                        }
                        writeBackbonePayload(uri, documentId, schema, body)
                            .then(function () {
                            res.statusCode = 200;
                            res.setHeader("content-type", "application/octet-stream");
                            res.end(new Uint8Array());
                        })
                            .catch(function (error) {
                            res.statusCode = 500;
                            res.end(String(error));
                        });
                    });
                    return;
                }
                res.statusCode = 405;
                res.end("method not allowed");
            });
        },
    };
}
/** @emoji 🔌️ Every plugin dir under `root` that has a completed build right now (a `.core*.wasm`
 * present — same convention `collectPluginWasmSizeRows` walks), newest core-wasm mtime as `rebuiltAt`.
 * Backs the SSE endpoint's connect-time `snapshot` event: a browser that connects (or reconnects) after
 * some builds already finished must still learn about them — `♻️hot-swap.json` alone only ever holds the
 * single most recent build, not the full history. `root` is REQUIRED (never defaulted): the one staging
 * root is `pluginModulesRoot(profile)` in `♻️activation/🟦️.ts`, and a default here was how a second,
 * silently drifting module tree stayed alive. */
function scanBuiltPluginModules(root) {
    if (!(0, node_fs_1.existsSync)(root))
        return [];
    var rows = [];
    for (var _i = 0, _a = (0, node_fs_1.readdirSync)(root, { withFileTypes: true }); _i < _a.length; _i++) {
        var entry = _a[_i];
        if (!entry.isDirectory() || !(0, ____ts_1.moduleIdForDirectoryName)(entry.name))
            continue;
        var pluginDir = (0, node_path_1.join)(root, entry.name);
        var newestMs = 0;
        for (var _b = 0, _c = (0, node_fs_1.readdirSync)(pluginDir); _b < _c.length; _b++) {
            var file = _c[_b];
            if (!/\.core\d*\.wasm$/.test(file))
                continue;
            newestMs = Math.max(newestMs, (0, node_fs_1.statSync)((0, node_path_1.join)(pluginDir, file)).mtimeMs);
        }
        var pluginId = (0, ____ts_1.moduleIdForDirectoryName)(entry.name);
        if (newestMs > 0 && pluginId)
            rows.push({ pluginId: pluginId, rebuiltAt: Math.round(newestMs) });
    }
    return rows;
}
/** @emoji 🔌️ OS-owned watcher route supplied explicitly to the neutral kernel source adapter. */
exports.PLUGIN_SOURCE_WATCH_PATH = "".concat(____ts_1.MODULE_PLUGIN_ROUTE, "/watch");
/** @emoji 🔌️ Vite middleware backing the shell's `createDevPluginSource` (`@semio-tech/framework`):
 * SSE at `PLUGIN_SOURCE_WATCH_PATH`, mirroring `semioBackboneVitePlugin`'s `/watch` endpoint. Sends one
 * `snapshot` on connect ({@link scanBuiltPluginModules}), then a `built` event every time `buildPlugin`
 * overwrites the shared `♻️hot-swap.json` marker — `buildPlugin` writes it last, after every other output
 * file, so by the time this fires the plugin's module is actually fetchable. Debounced the same 200ms
 * as `subscribeFolderWatch` above (a burst of writes during one build collapses to a single event). One
 * `fs.watch` on `plugin-modules/` for the whole dev server's lifetime — unlike the backbone plugin's
 * per-uri watchers, there is exactly one watch target here, so it is never torn down. */
function semioPluginHotSwapVitePlugin(options) {
    return {
        name: "semio-plugin-hot-swap",
        configureServer: function (server) {
            var subscribers = new Set();
            (0, node_fs_1.mkdirSync)(options.moduleRoot, { recursive: true });
            var hotSwapMarker = (0, node_path_1.join)(options.moduleRoot, ____ts_1.MODULE_HOT_SWAP_FILE);
            var debounceTimer;
            (0, node_fs_1.watch)(options.moduleRoot, function (_eventType, filename) {
                if (filename !== ____ts_1.MODULE_HOT_SWAP_FILE)
                    return;
                if (debounceTimer)
                    clearTimeout(debounceTimer);
                debounceTimer = setTimeout(function () {
                    if (!(0, node_fs_1.existsSync)(hotSwapMarker))
                        return;
                    var marker;
                    try {
                        marker = JSON.parse((0, node_fs_1.readFileSync)(hotSwapMarker, "utf8"));
                    }
                    catch (_a) {
                        return;
                    }
                    var event = { kind: "built", pluginId: marker.pluginId, rebuiltAt: marker.rebuiltAt };
                    var payload = "data: ".concat(JSON.stringify(event), "\n\n");
                    for (var _i = 0, subscribers_1 = subscribers; _i < subscribers_1.length; _i++) {
                        var sub = subscribers_1[_i];
                        sub.write(payload);
                    }
                }, FOLDER_WATCH_DEBOUNCE_MS);
            });
            server.middlewares.use(function (req, res, next) {
                var _a;
                if ((0, ____ts_1.moduleRoutePath)((_a = req.url) !== null && _a !== void 0 ? _a : "") !== exports.PLUGIN_SOURCE_WATCH_PATH || req.method !== "GET")
                    return next();
                res.statusCode = 200;
                res.setHeader("content-type", "text/event-stream");
                res.setHeader("cache-control", "no-cache");
                res.setHeader("connection", "keep-alive");
                res.write(": connected\n\n");
                var snapshot = { kind: "snapshot", plugins: scanBuiltPluginModules(options.moduleRoot) };
                res.write("data: ".concat(JSON.stringify(snapshot), "\n\n"));
                subscribers.add(res);
                var stopKeepalive = startSseKeepalive(res);
                req.on("close", function () {
                    stopKeepalive();
                    subscribers.delete(res);
                });
            });
        },
    };
}
/** @emoji 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and
 * prints one `[stale]` line per component whose served bytes are behind — the live half of the serve-start
 * pass in `📜️script.ts`. A restage that lands while the server runs therefore retires its own warning
 * without a restart, and one that never lands keeps saying so. */
function reportActivationFreshness(receipt, options) {
    var activated = new Map(receipt.plugins.map(function (row) { return [row.pluginId, row.artifactSha256]; }));
    var facts = options.components.map(function (component) {
        var newest = (0, ____ts_2.newestComponentSourceMtime)(component.sourceRoot);
        var installedMeta = (0, node_path_1.join)(options.installRoot, component.directoryName, ____ts_3.EXTENSION_INSTALL_META);
        var installedPackageHash;
        if ((0, node_fs_1.existsSync)(installedMeta)) {
            try {
                installedPackageHash = JSON.parse((0, node_fs_1.readFileSync)(installedMeta, "utf8")).packageHash;
            }
            catch (_a) {
                installedPackageHash = undefined;
            }
        }
        return {
            pluginId: component.pluginId,
            role: component.role,
            activationTracked: true,
            stagedAtMs: (0, ____ts_2.stagedModuleMtime)((0, node_path_1.join)(options.moduleRoot, component.directoryName)),
            newestSourceMs: newest === null || newest === void 0 ? void 0 : newest.mtimeMs,
            newestSourcePath: newest ? (0, node_path_1.relative)(exports.REPO_ROOT, newest.path).split(/[\\/]/).join("/") : undefined,
            receiptArtifactSha256: activated.get(component.pluginId),
            installedPackageHash: installedPackageHash,
        };
    });
    return (0, ____ts_2.stagedModuleReportLines)(facts.map(____ts_2.stagedModuleVerdict), "bun nx run @semio-tech/framework-os-dev:activate-".concat(receipt.variant, "-react-").concat(receipt.profile));
}
/** 📡️ Announces explicit Nx activation completion and releases every server-owned subscription. */
function semioActivationVitePlugin(options) {
    var dispose = function () { };
    var staleness = [];
    return {
        name: "semio-activation",
        /** @emoji 📣️ The staged-module verdict belongs in the DEVELOPER's console, not only in the server log
         * they are not reading: a guest module staged behind its own source serves a wire contract the host
         * TypeScript in the same page no longer speaks, and the symptom (`actor-ui-patch.pairing`, a window
         * booting a fallback graph) never names its cause. */
        transformIndexHtml: {
            order: "post",
            handler: function () {
                if (staleness.length === 0)
                    return [];
                var message = "semio dev \u00B7 ".concat(staleness.length, " staged plugin module(s) are behind their source \u2014 the host in this page may speak a newer wire contract than the guest it is talking to:\n").concat(staleness.join("\n"));
                return [{ tag: "script", attrs: { type: "module" }, children: "console.warn(".concat(JSON.stringify(message), ");") }];
            },
        },
        configureServer: function (server) {
            var _a;
            dispose();
            var subscribers = new Map();
            var previous;
            var send = function (event) {
                var text = "data: ".concat(JSON.stringify(event), "\n\n");
                for (var _i = 0, subscribers_2 = subscribers; _i < subscribers_2.length; _i++) {
                    var _a = subscribers_2[_i], response = _a[0], stop_1 = _a[1];
                    try {
                        response.write(text);
                    }
                    catch (_b) {
                        stop_1();
                        subscribers.delete(response);
                    }
                }
            };
            var observer = (0, ____ts_2.observeActivationReceipts)(options.receiptDirectory, function (receipt) {
                var _a;
                staleness = reportActivationFreshness(receipt, options);
                for (var _i = 0, staleness_1 = staleness; _i < staleness_1.length; _i++) {
                    var line = staleness_1[_i];
                    console.warn(line);
                }
                if (previous) {
                    if (previous.plugins.map(function (row) { return row.pluginId; }).join() !== receipt.plugins.map(function (row) { return row.pluginId; }).join())
                        (_a = server.ws) === null || _a === void 0 ? void 0 : _a.send({ type: "full-reload" });
                    var prior = new Map(previous.plugins.map(function (row) { return [row.pluginId, row.artifactSha256]; }));
                    for (var _b = 0, _c = receipt.plugins; _b < _c.length; _b++) {
                        var row = _c[_b];
                        if (prior.get(row.pluginId) !== row.artifactSha256)
                            send({ kind: "built", pluginId: row.pluginId, rebuiltAt: row.rebuiltAt });
                    }
                }
                previous = receipt;
            }, function (error) { return console.error("Activation receipt failed:", error); });
            dispose = function () {
                observer.close();
                for (var _i = 0, subscribers_3 = subscribers; _i < subscribers_3.length; _i++) {
                    var _a = subscribers_3[_i], response = _a[0], stop_2 = _a[1];
                    stop_2();
                    response.end();
                }
                subscribers.clear();
            };
            (_a = server.httpServer) === null || _a === void 0 ? void 0 : _a.once("close", dispose);
            server.middlewares.use(function (req, res, next) {
                var _a;
                if ((0, ____ts_1.moduleRoutePath)((_a = req.url) !== null && _a !== void 0 ? _a : "") !== exports.PLUGIN_SOURCE_WATCH_PATH || req.method !== "GET")
                    return next();
                res.statusCode = 200;
                res.setHeader("content-type", "text/event-stream");
                res.setHeader("cache-control", "no-cache");
                res.setHeader("connection", "keep-alive");
                res.write(": connected\n\n");
                var event = { kind: "snapshot", plugins: observer.snapshot().plugins.map(function (_a) {
                        var pluginId = _a.pluginId, rebuiltAt = _a.rebuiltAt;
                        return ({ pluginId: pluginId, rebuiltAt: rebuiltAt });
                    }) };
                res.write("data: ".concat(JSON.stringify(event), "\n\n"));
                var stop = startSseKeepalive(res);
                subscribers.set(res, stop);
                req.on("close", function () { stop(); subscribers.delete(res); });
            });
        },
        closeBundle: function () { dispose(); },
    };
}
//#endregion 🔌️PluginHotSwapVitePlugin
//#region BlobVitePlugin
var blobDatabaseSingleton;
/** 🗄️ Lazily opens the dev-session-wide content-addressed blob store at `<repoRoot>/.🧬semio/🔗space/blobs.db` —
 * unlike backbone documents, blobs aren't scoped to a per-uri folder (there's no folder in the
 * `write-blob`/`read-blob` WIT signature), so this is one shared table for the whole dev server. */
function blobDatabase() {
    return __awaiter(this, void 0, void 0, function () {
        var Database, dbPath;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!!blobDatabaseSingleton) return [3 /*break*/, 2];
                    return [4 /*yield*/, backboneDatabaseCtorLazy()];
                case 1:
                    Database = _a.sent();
                    dbPath = (0, node_path_1.join)(exports.REPO_ROOT, ".🧬semio", "🔗space", "blobs.db");
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(dbPath), { recursive: true });
                    blobDatabaseSingleton = new Database(dbPath);
                    blobDatabaseSingleton.run("CREATE TABLE IF NOT EXISTS blob (hash TEXT PRIMARY KEY, media_type TEXT NOT NULL, size INTEGER NOT NULL, bytes BLOB NOT NULL)");
                    _a.label = 2;
                case 2: return [2 /*return*/, blobDatabaseSingleton];
            }
        });
    });
}
/** @emoji 📦️ Vite middleware for the dev-only content-addressed blob store: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=`
 * (raw bytes body, BLAKE3-hashed above, returns `{"hash":...}`, idempotent via `INSERT OR IGNORE`) and
 * `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response, 404 if absent). The browser host-shim's
 * `writeBlob`/`readBlob` (see `hostShimSource`) and `🟦️backbone-🟦️worker.ts`'s IndexedDB cache both talk to
 * this. Mirrors `vcs::FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table/shape. */
function semioBlobVitePlugin() {
    return {
        name: "semio-blob",
        configureServer: function (server) {
            var _this = this;
            server.middlewares.use(function (req, res, next) {
                var _a;
                if (!((_a = req.url) === null || _a === void 0 ? void 0 : _a.startsWith(framework_os_1.BLOB_ENDPOINT_PATH)))
                    return next();
                var requestUrl = new URL(req.url, "http://127.0.0.1");
                if (req.method === "PUT") {
                    var chunks_1 = [];
                    req.on("data", function (chunk) {
                        chunks_1.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
                    });
                    req.on("end", function () {
                        void (function () { return __awaiter(_this, void 0, void 0, function () {
                            var bytes, mediaType, hash, db;
                            var _a;
                            return __generator(this, function (_b) {
                                switch (_b.label) {
                                    case 0:
                                        bytes = Buffer.concat(chunks_1);
                                        mediaType = (_a = requestUrl.searchParams.get("mediaType")) !== null && _a !== void 0 ? _a : "application/octet-stream";
                                        hash = (0, ____ts_4.blake3Hex)(new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
                                        return [4 /*yield*/, blobDatabase()];
                                    case 1:
                                        db = _b.sent();
                                        db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, mediaType, bytes.length, bytes]);
                                        res.statusCode = 200;
                                        res.setHeader("content-type", "application/json");
                                        res.end(JSON.stringify({ hash: hash }));
                                        return [2 /*return*/];
                                }
                            });
                        }); })().catch(function (error) {
                            res.statusCode = 500;
                            res.end(String(error));
                        });
                    });
                    return;
                }
                if (req.method === "GET") {
                    var hash_1 = requestUrl.pathname.slice("".concat(framework_os_1.BLOB_ENDPOINT_PATH, "/").length);
                    if (!hash_1) {
                        res.statusCode = 400;
                        res.end("missing hash");
                        return;
                    }
                    void (function () { return __awaiter(_this, void 0, void 0, function () {
                        var db, row;
                        var _a, _b;
                        return __generator(this, function (_c) {
                            switch (_c.label) {
                                case 0: return [4 /*yield*/, blobDatabase()];
                                case 1:
                                    db = _c.sent();
                                    row = db.query("SELECT media_type, bytes FROM blob WHERE hash = ?1").get(hash_1);
                                    if (!row) {
                                        res.statusCode = 404;
                                        res.end("");
                                        return [2 /*return*/];
                                    }
                                    res.statusCode = 200;
                                    res.setHeader("content-type", (_a = row.media_type) !== null && _a !== void 0 ? _a : "application/octet-stream");
                                    res.end(Buffer.from((_b = row.bytes) !== null && _b !== void 0 ? _b : new Uint8Array()));
                                    return [2 /*return*/];
                            }
                        });
                    }); })().catch(function (error) {
                        res.statusCode = 500;
                        res.end(String(error));
                    });
                    return;
                }
                res.statusCode = 405;
                res.end("method not allowed");
            });
        },
    };
}
//#endregion BlobVitePlugin
//#region 🔖️SourceFreshnessVitePlugins
/** @emoji 🚫️ Repository directory names no dev server may ever watch: version control metadata, the Nx
 * workspace store, the package store, the shared build/cache root, compiled output and generated
 * sources. Tools rewrite millions of files inside them while a dev session is open, and every such write
 * would otherwise be delivered into the dev server's event loop.
 *
 * `🤖️generated` and `.vscode` stay listed for the reason Vite's own `server.watch.ignored` once carried:
 * those files are config dependencies, so reacting to a registry or launch-config rewrite restarts the
 * server in a loop.
 * @see https://github.com/paulmillr/chokidar/blob/3.6.0/lib/fsevents-handler.js */
exports.UNWATCHED_REPOSITORY_SEGMENTS = [".git", ".nx", ".vscode", ".🧬semio", "node_modules", "dist", "target", "🤖️generated", "🗑️generated"];
/** @emoji 👁️ The repository's top-level source directories — every place a dev server's module graph can
 * legitimately import from, with the unwatchable stores above removed. Read from disk rather than
 * hardcoded so a new top-level product directory is watched without touching this module. */
function repositorySourceWatchRoots(repoRoot) {
    var excluded = new Set(exports.UNWATCHED_REPOSITORY_SEGMENTS);
    return (0, node_fs_1.readdirSync)(repoRoot, { withFileTypes: true }).filter(function (entry) { return entry.isDirectory() && !excluded.has(entry.name); }).map(function (entry) { return (0, node_path_1.join)(repoRoot, entry.name); }).sort();
}
/** @emoji 🧹️ Matches any relative path that crosses an unwatched store, on both `/` and `\` separators.
 * One precompiled test per filesystem event is the whole per-event budget this watcher may spend. */
function unwatchedRepositoryPathMatcher() {
    var alternatives = exports.UNWATCHED_REPOSITORY_SEGMENTS.map(function (segment) { return segment.replaceAll(".", "\\."); }).join("|");
    return new RegExp("(?:^|[\\\\/])(?:".concat(alternatives, ")(?:[\\\\/]|$)"), "u");
}
/** @emoji 🛰️ Drives Vite's file-change pipeline from `node:fs` recursive watches over the repository's
 * source roots, so `⚙️vite.config.ts` can hand Vite `server.watch: null` and run no chokidar watcher of
 * its own.
 *
 * Vite watches its `root` plus — through `ensureWatchedFile` — every module-graph file outside it, which
 * in this repository is roughly a thousand individual paths spread across several top-level directories.
 * On macOS chokidar answers that by consolidating sibling FSEvents streams upward until ONE stream covers
 * the whole repository, then runs every watched path's prefix filter against every event that stream
 * delivers. A concurrent `cargo` build writing into the shared cache therefore costs the dev server
 * `events × watched paths` string comparisons — measured at 6 291 events per 2 s against 1 316 watched
 * paths, which blocks the event loop for ~2 s at a time, allocates ~600 MB per burst and, once resident
 * memory reaches the runtime's ceiling, wedges the server permanently. `server.watch.ignored` cannot undo
 * this: chokidar consults it only after those prefix filters have already run.
 *
 * Watching the source roots directly keeps the kernel from ever reporting cache, package-store or
 * generated-output writes, and reduces the per-event cost to one regular-expression test. Events are
 * replayed on Vite's own (no-op) watcher emitter, so module invalidation, HMR boundary computation and
 * config-dependency restarts behave exactly as they did with chokidar.
 *
 * 🛰️ An existing FILE is replayed as `add` AND `change`, because macOS reports every write to it —
 * in place and atomic (temp + rename) alike — as `eventType: "rename"`, while Vite invalidates a
 * transformed module only from its `change` handler (`moduleGraph.onFileChange`); its `add` handler
 * recovers previously failed resolves and never touches the module graph. Chokidar told the two apart
 * from its own directory snapshots, which this watcher deliberately does not keep — so it states both
 * facts, which are both true of an atomic save (a new inode appeared, and the module changed) and
 * idempotent for a genuinely new file (nothing imports it yet, so the `change` finds no module).
 * Emitting only `add` served the pre-edit transform for the life of the server, and
 * `SEMIO_VITE_HMR=0` (`hmr: false`) removes the HMR pass that would otherwise have hidden it
 * (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §4.2). */
var REACT_REFRESH_RUNTIME = "/@react-refresh";
/** @emoji ⚛️ Preamble copied from `@vitejs/plugin-react` — semio-host-html replaces the whole document in
 * `transformIndexHtml` `order: "pre"`, so the react plugin's own preamble injection must be reinforced in
 * `order: "post"` or `@react-three/fiber` (and every other JSX dep) throws "can't detect preamble". */
function semioReactRefreshPreambleScript(base) {
    var root = base.endsWith("/") ? base.slice(0, -1) : base;
    return "import { injectIntoGlobalHook } from \"".concat(root).concat(REACT_REFRESH_RUNTIME, "\";\ninjectIntoGlobalHook(window);\nwindow.$RefreshReg$ = () => {};\nwindow.$RefreshSig$ = () => (type) => type;");
}
/** @emoji ⚛️ Aligns Vite 7 / Rolldown OXC JSX refresh with `server.hmr` — `SEMIO_VITE_HMR=0` must not emit
 * `$RefreshReg$` wrappers without the HTML preamble, and HMR-on serves must always ship that preamble even
 * after {@link semioHostHtmlVitePlugin} rebuilds `index.html`. */
function semioPlaygroundReactRefreshCoherenceVitePlugin() {
    return {
        name: "semio-playground-react-refresh-coherence",
        enforce: "post",
        config: function (userConfig, _a) {
            var _b;
            var command = _a.command;
            if (command !== "serve" || ((_b = userConfig.server) === null || _b === void 0 ? void 0 : _b.hmr) !== false)
                return;
            return {
                esbuild: { jsxDev: false },
                oxc: { jsx: { refresh: false } },
                optimizeDeps: { esbuildOptions: { jsxDev: false } },
            };
        },
        transformIndexHtml: {
            order: "post",
            handler: function (html, ctx) {
                var _a, _b, _c;
                if (((_a = ctx.server) === null || _a === void 0 ? void 0 : _a.config.server.hmr) === false)
                    return;
                if (html.includes("injectIntoGlobalHook"))
                    return;
                var base = (_c = (_b = ctx.server) === null || _b === void 0 ? void 0 : _b.config.base) !== null && _c !== void 0 ? _c : "/";
                return [{ tag: "script", attrs: { type: "module" }, children: semioReactRefreshPreambleScript(base) }];
            },
        },
    };
}
/** @emoji 🔍️ Remembers what every transformed module's file looked like on disk when its transform was
 * produced, and answers which of them have moved since.
 *
 * ONLY files the dev server has actually transformed are tracked, so "moved" is exactly "the cached
 * transform is out of date": a file the server never read has no cached transform to be stale. The
 * per-directory index makes the answer to "did anything in THIS directory change" cost one stat per
 * tracked sibling, which is what turns a filesystem event naming an editor's temporary file into the
 * invalidation of the module that temporary file was renamed onto. */
function createSourceFreshnessRegistry() {
    var stamps = new Map();
    var siblings = new Map();
    var readStamp = function (file) {
        try {
            var status_1 = (0, node_fs_1.statSync)(file);
            return { mtimeMs: status_1.mtimeMs, size: status_1.size };
        }
        catch (_a) {
            return null;
        }
    };
    var moved = function (file) {
        var previous = stamps.get(file);
        if (previous === undefined)
            return false;
        var current = readStamp(file);
        if (current === null)
            return false;
        if (current.mtimeMs === previous.mtimeMs && current.size === previous.size)
            return false;
        stamps.set(file, current);
        return true;
    };
    return {
        record: function (file) {
            var _a;
            var stamp = readStamp(file);
            if (stamp === null)
                return;
            var directory = (0, node_path_1.dirname)(file);
            var tracked = (_a = siblings.get(directory)) !== null && _a !== void 0 ? _a : new Set();
            tracked.add(file);
            siblings.set(directory, tracked);
            stamps.set(file, stamp);
        },
        trackedCount: function () { return stamps.size; },
        movedFile: function (file) { return moved(file); },
        movedInDirectory: function (directory) { var _a; return __spreadArray([], ((_a = siblings.get(directory)) !== null && _a !== void 0 ? _a : []), true).filter(moved); },
        movedEverywhere: function () { return __spreadArray([], stamps.keys(), true).filter(moved); },
    };
}
/** @emoji ♻️ Retires every cached transform of one file, synchronously for the request in flight and then
 * through Vite's own file-change pipeline for everything downstream of it (plugin `watchChange`, HMR
 * boundaries, config-dependency restarts). `onFileChange` walks importers, so an importer that inlined
 * the edited module's output is retired with it. */
function retireStaleModule(server, file) {
    var _a;
    for (var _i = 0, _b = Object.values((_a = server.environments) !== null && _a !== void 0 ? _a : {}); _i < _b.length; _i++) {
        var environment = _b[_i];
        environment.moduleGraph.onFileChange(file);
    }
    server.watcher.emit("change", file);
}
function semioSourceWatchVitePlugin(options) {
    return {
        name: "semio-source-watch",
        apply: "serve",
        configureServer: function (server) {
            var _a;
            var unwatched = unwatchedRepositoryPathMatcher();
            var freshness = options.freshness;
            var handles = repositorySourceWatchRoots(options.repoRoot).map(function (root) { return (0, node_fs_1.watch)(root, { recursive: true, persistent: false }, function (eventType, name) {
                var _a, _b, _c, _d;
                if (name === null || unwatched.test(name))
                    return;
                var path = (0, node_path_1.join)(root, name);
                var directory = (0, node_path_1.dirname)(path);
                if (eventType !== "rename") {
                    server.watcher.emit("change", path);
                    for (var _i = 0, _e = (_a = freshness === null || freshness === void 0 ? void 0 : freshness.movedInDirectory(directory)) !== null && _a !== void 0 ? _a : []; _i < _e.length; _i++) {
                        var moved = _e[_i];
                        if (moved !== path)
                            server.watcher.emit("change", moved);
                    }
                    return;
                }
                if (!(0, node_fs_1.existsSync)(path)) {
                    server.watcher.emit("unlink", path);
                    for (var _f = 0, _g = (_b = freshness === null || freshness === void 0 ? void 0 : freshness.movedInDirectory(directory)) !== null && _b !== void 0 ? _b : []; _f < _g.length; _f++) {
                        var moved = _g[_f];
                        server.watcher.emit("change", moved);
                    }
                    return;
                }
                if ((0, node_fs_1.statSync)(path).isDirectory()) {
                    server.watcher.emit("addDir", path);
                    for (var _h = 0, _j = (_c = freshness === null || freshness === void 0 ? void 0 : freshness.movedInDirectory(path)) !== null && _c !== void 0 ? _c : []; _h < _j.length; _h++) {
                        var moved = _j[_h];
                        server.watcher.emit("change", moved);
                    }
                    return;
                }
                server.watcher.emit("add", path);
                server.watcher.emit("change", path);
                for (var _k = 0, _l = (_d = freshness === null || freshness === void 0 ? void 0 : freshness.movedInDirectory(directory)) !== null && _d !== void 0 ? _d : []; _k < _l.length; _k++) {
                    var moved = _l[_k];
                    if (moved !== path)
                        server.watcher.emit("change", moved);
                }
            }); });
            (_a = server.httpServer) === null || _a === void 0 ? void 0 : _a.once("close", function () {
                for (var _i = 0, handles_1 = handles; _i < handles_1.length; _i++) {
                    var handle = handles_1[_i];
                    handle.close();
                }
            });
        },
    };
}
/** @emoji 🗺️ The absolute file a dev-server request would be transformed from, or `null` for a request no
 * module graph entry can back (virtual ids, client runtime, the index document). `/@fs/` carries the
 * absolute path the module graph is keyed by; everything else is relative to Vite's `root`. */
function requestedTransformFile(url, root) {
    var pathname = url.split("?")[0];
    if (pathname === "" || pathname === "/" || pathname.endsWith("/"))
        return null;
    var decoded;
    try {
        decoded = decodeURIComponent(pathname);
    }
    catch (_a) {
        return null;
    }
    if (decoded.startsWith("/@fs/"))
        return decoded.slice("/@fs".length);
    if (decoded.startsWith("/@") || decoded.startsWith("/\0") || decoded.includes("\0"))
        return null;
    return (0, node_path_1.join)(root, decoded);
}
/** @emoji 🛡️ Proves, at request time, that every transform this dev server is about to serve was produced
 * from the bytes currently on disk — the guarantee the filesystem watcher alone cannot give.
 *
 * macOS reports a recursive `fs.watch` event by the path whose directory entry changed, and an atomic
 * save (write a temporary file, `rename` it onto the target) changes the entry of the TEMPORARY file. The
 * edited module's own path is then never named by any event — measured 0 times in 5 at every module depth,
 * for `sed -i ''`, for a rename-into-place and for every editor that saves atomically, which is all of
 * them. `SEMIO_VITE_HMR=0` removes the HMR pass that would otherwise have papered over it, so the dev
 * server keeps serving the pre-edit transform until the process is recycled: a developer moves a slider
 * and the preview runs yesterday's module.
 *
 * {@link semioSourceWatchVitePlugin} now answers such an event by re-stating its whole directory, which
 * repairs the common case at edit time. This plugin is the guarantee underneath it, and it does not
 * depend on any event arriving at all: every module request re-stats the one file behind it, and every
 * document request re-stats the whole transformed set, so the very next request after any edit — by any
 * tool, through any write style, with the watcher armed or not — serves the current file.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts */
function semioTransformFreshnessVitePlugin(options) {
    var documentSweep = { verified: 0, retired: 0 };
    return {
        name: "semio-transform-freshness",
        apply: "serve",
        enforce: "pre",
        transform: function (_code, id) {
            var file = id.split("?")[0];
            if ((0, node_path_1.isAbsolute)(file))
                options.freshness.record(file);
            return null;
        },
        configureServer: function (server) {
            var _a, _b, _c;
            var root = (_b = (_a = server.config) === null || _a === void 0 ? void 0 : _a.root) !== null && _b !== void 0 ? _b : "";
            (_c = server.middlewares) === null || _c === void 0 ? void 0 : _c.use(function (request, _response, next) {
                var _a;
                var url = (_a = request.url) !== null && _a !== void 0 ? _a : "";
                var accept = request.headers.accept;
                if (typeof accept === "string" && accept.includes("text/html")) {
                    var stale = options.freshness.movedEverywhere();
                    for (var _i = 0, stale_1 = stale; _i < stale_1.length; _i++) {
                        var file_1 = stale_1[_i];
                        retireStaleModule(server, file_1);
                    }
                    documentSweep = { verified: options.freshness.trackedCount(), retired: stale.length };
                    next();
                    return;
                }
                var file = requestedTransformFile(url, root);
                if (file !== null && options.freshness.movedFile(file))
                    retireStaleModule(server, file);
                next();
            });
        },
        transformIndexHtml: {
            order: "post",
            handler: function (_html, context) {
                var _a;
                var hmr = ((_a = context.server) === null || _a === void 0 ? void 0 : _a.config.server.hmr) === false ? "off" : "on";
                var banner = "semio dev \u00B7 transform freshness: stat-guard (every module request + whole graph per document) \u00B7 hmr ".concat(hmr, " \u00B7 ").concat(documentSweep.verified, " modules verified, ").concat(documentSweep.retired, " stale transforms retired \u00B7 serve pid ").concat(process.pid, " \u00B7 document ").concat(new Date().toISOString());
                return [{ tag: "script", attrs: { type: "module" }, children: "console.info(".concat(JSON.stringify(banner), ");") }];
            },
        },
    };
}
/** @emoji 🛰️ The dev server's complete "never serve a stale module" contract: the source watcher that
 * pushes edits into Vite's module graph, and the request-time stat guard that verifies what the watcher
 * delivered. They share one {@link createSourceFreshnessRegistry}, so the watcher can resolve a
 * temporary-file event into the module it was renamed onto. Mount both or neither. */
function semioSourceFreshnessVitePlugins(options) {
    var freshness = createSourceFreshnessRegistry();
    return [semioTransformFreshnessVitePlugin({ freshness: freshness }), semioSourceWatchVitePlugin({ repoRoot: options.repoRoot, freshness: freshness })];
}
//#endregion SourceFreshnessVitePlugins
//#region 🛰️AgentBridgeRendezvous
/** @emoji 🛰️ Where a live os session and a `semio-os-mcp` stdio gateway find each other — the exact
 * layout the gateway's own `🌉️mcp/🛰️rendezvous` facet owns (`~/.semio/agent/bridge`). */
exports.AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION = 1;
exports.AGENT_BRIDGE_OFFER_ENDPOINT_PATH = "/__semio/agent-bridge";
/** @emoji 🏷️ The carrier that points this dev session and one `semio-os-mcp` gateway at a rendezvous
 * of their own instead of the per-user default — the exact twin of the gateway's own
 * `🛰️rendezvous::RENDEZVOUS_DIR_ENV`. It is a directory path, never a credential (the admission proof
 * stays in the owner-only offer file the supervisor reads), and it is spelled with the `S_` prefix
 * the gateway's process-entry seal admits. Without it, `newestLiveAgentBridgeOffer` hands a shell
 * whichever gateway published last, so two agents running at once cross-wire. */
exports.AGENT_BRIDGE_RENDEZVOUS_DIR_ENV = "S_AGENT_BRIDGE_DIR";
function agentBridgeRendezvousDir() {
    var _a, _b;
    var pinned = process.env[exports.AGENT_BRIDGE_RENDEZVOUS_DIR_ENV];
    if (pinned)
        return pinned;
    var home = (_b = (_a = process.env.HOME) !== null && _a !== void 0 ? _a : process.env.USERPROFILE) !== null && _b !== void 0 ? _b : ".";
    return (0, node_path_1.join)(home, ".semio", "agent", "bridge");
}
/** @emoji 📨️ The newest gateway offer whose publishing process is still alive — what the browser
 * shell dials. `null` means no MCP gateway is currently offering a bridge, which is an ordinary
 * state (nobody launched one), never an error. */
function newestLiveAgentBridgeOffer(root) {
    if (root === void 0) { root = agentBridgeRendezvousDir(); }
    var directory = (0, node_path_1.join)(root, "offers");
    if (!(0, node_fs_1.existsSync)(directory))
        return null;
    var offers = (0, node_fs_1.readdirSync)(directory)
        .filter(function (name) { return name.endsWith(".json"); })
        .flatMap(function (name) {
        var _a, _b;
        var path = (0, node_path_1.join)(directory, name);
        try {
            var offer = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
            if (offer.schemaVersion !== exports.AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION || typeof offer.url !== "string" || typeof offer.admissionProof !== "string" || typeof offer.pid !== "number")
                return [];
            try {
                process.kill(offer.pid, 0);
            }
            catch (_c) {
                return [];
            }
            return [{ url: offer.url, admissionProof: offer.admissionProof, principal: (_a = offer.principal) !== null && _a !== void 0 ? _a : "agent:local", pid: offer.pid, publishedAtMs: (_b = offer.publishedAtMs) !== null && _b !== void 0 ? _b : 0 }];
        }
        catch (_d) {
            return [];
        }
    });
    offers.sort(function (left, right) { return right.publishedAtMs - left.publishedAtMs; });
    var newest = offers[0];
    return newest ? { url: newest.url, admissionProof: newest.admissionProof, principal: newest.principal, pid: newest.pid } : null;
}
/** @emoji 🛰️ Publishes THIS dev session as a live os session the stdio MCP gateway can discover, and
 * serves the gateway's own offer back to the browser shell on
 * {@link AGENT_BRIDGE_OFFER_ENDPOINT_PATH}. The admission proof never travels through an environment
 * variable or a build-time define: the dev server reads the owner-only offer file and hands it over
 * loopback, on request, exactly like the local supervisor it is.
 *
 * Both halves are removed when the dev server closes, so a gateway that starts later never believes a
 * dead session. */
function semioAgentBridgeRendezvousVitePlugin(options) {
    var _a;
    if (options === void 0) { options = {}; }
    var root = (_a = options.rendezvousRoot) !== null && _a !== void 0 ? _a : agentBridgeRendezvousDir();
    var sessionsDir = (0, node_path_1.join)(root, "sessions");
    var recordPath = (0, node_path_1.join)(sessionsDir, "".concat(process.pid, ".json"));
    var removeRecord = function () {
        try {
            if ((0, node_fs_1.existsSync)(recordPath))
                (0, node_fs_1.rmSync)(recordPath, { force: true });
        }
        catch (_a) {
            // best effort — a stale record is swept by the gateway's own liveness probe
        }
    };
    return {
        name: "semio-agent-bridge-rendezvous",
        apply: "serve",
        configureServer: function (server) {
            var _a, _b;
            (0, node_fs_1.mkdirSync)(sessionsDir, { recursive: true });
            (0, node_fs_1.writeFileSync)(recordPath, JSON.stringify({ schemaVersion: exports.AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, sessionId: (_a = options.sessionId) !== null && _a !== void 0 ? _a : "dev-".concat(process.pid), pid: process.pid, shellKind: "react", startedAtMs: Date.now() }, null, 2), { mode: 384 });
            process.once("exit", removeRecord);
            (_b = server.httpServer) === null || _b === void 0 ? void 0 : _b.once("close", removeRecord);
            server.middlewares.use(function (req, res, next) {
                var _a;
                if (!((_a = req.url) === null || _a === void 0 ? void 0 : _a.startsWith(exports.AGENT_BRIDGE_OFFER_ENDPOINT_PATH)))
                    return next();
                var offer = newestLiveAgentBridgeOffer(root);
                res.statusCode = offer ? 200 : 404;
                res.setHeader("content-type", "application/json");
                res.setHeader("cache-control", "no-store");
                res.end(JSON.stringify(offer !== null && offer !== void 0 ? offer : { error: "no live semio-os-mcp gateway is offering a bridge" }));
            });
        },
        closeBundle: removeRecord,
    };
}
//#endregion 🛰️AgentBridgeRendezvous
