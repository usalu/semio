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
exports.acquireResourceLease = acquireResourceLease;
exports.withResourceLeases = withResourceLeases;
var node_crypto_1 = require("node:crypto");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var promises_1 = require("node:timers/promises");
var APPLICATION_ID = 0x534d4c53;
/** 🗃️ Uses each runtime's system SQLite behind the same local interface. */
function openDatabase(path) {
    return __awaiter(this, void 0, void 0, function () {
        var Database, database_1, DatabaseSync, database;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!process.versions.bun) return [3 /*break*/, 2];
                    return [4 /*yield*/, Promise.resolve().then(function () { return require("bun:sqlite"); })];
                case 1:
                    Database = (_a.sent()).Database, database_1 = new Database(path, { create: true });
                    return [2 /*return*/, { exec: function (sql) { return database_1.exec(sql); }, read: function (sql) {
                                var _a;
                                var values = [];
                                for (var _i = 1; _i < arguments.length; _i++) {
                                    values[_i - 1] = arguments[_i];
                                }
                                return (_a = database_1.query(sql)).get.apply(_a, values);
                            }, close: function () { return database_1.close(); } }];
                case 2: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:sqlite"); })];
                case 3:
                    DatabaseSync = (_a.sent()).DatabaseSync, database = new DatabaseSync(path);
                    return [2 /*return*/, { exec: function (sql) { return database.exec(sql); }, read: function (sql) {
                                var _a;
                                var values = [];
                                for (var _i = 1; _i < arguments.length; _i++) {
                                    values[_i - 1] = arguments[_i];
                                }
                                return (_a = database.prepare(sql)).get.apply(_a, values);
                            }, close: function () { return database.close(); } }];
            }
        });
    });
}
/** 🛣️ Keeps lock identities on regular paths without following a substituted store. */
function databasePath(directory, resource) {
    for (var current = (0, node_path_1.resolve)(directory);; current = (0, node_path_1.dirname)(current)) {
        try {
            var stat = (0, node_fs_1.lstatSync)(current);
            if (!stat.isDirectory() || stat.isSymbolicLink())
                throw new Error("Invalid lease store directory: ".concat(current));
        }
        catch (error) {
            if (error.code !== "ENOENT")
                throw error;
        }
        if ((0, node_path_1.dirname)(current) === current)
            break;
    }
    (0, node_fs_1.mkdirSync)(directory, { recursive: true });
    var path = (0, node_path_1.join)((0, node_fs_1.realpathSync)(directory), (0, node_crypto_1.createHash)("sha256").update(resource).digest("hex") + ".sqlite");
    try {
        var stat = (0, node_fs_1.lstatSync)(path);
        if (!stat.isFile() || stat.isSymbolicLink() || stat.nlink !== 1)
            throw new Error("Invalid resource lease database: ".concat(path));
    }
    catch (error) {
        if (error.code !== "ENOENT")
            throw error;
    }
    return path;
}
/** 🪪️ Initializes one immutable resource identity under SQLite's exclusive transaction. */
function initialize(database, resource) {
    var _a, _b, _c;
    var identity = (_a = database.read("PRAGMA application_id")) === null || _a === void 0 ? void 0 : _a.application_id;
    if (identity === APPLICATION_ID)
        return;
    if (identity !== 0)
        throw new Error("Foreign resource lease database");
    database.exec("BEGIN EXCLUSIVE");
    try {
        var current = (_b = database.read("PRAGMA application_id")) === null || _b === void 0 ? void 0 : _b.application_id;
        if (current !== APPLICATION_ID) {
            if (current !== 0)
                throw new Error("Foreign resource lease database");
            if (((_c = database.read("SELECT count(*) AS count FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%'")) === null || _c === void 0 ? void 0 : _c.count) !== 0)
                throw new Error("Unowned resource lease database");
            database.exec("CREATE TABLE semio_resource_lease (id INTEGER PRIMARY KEY CHECK (id = 1), version INTEGER NOT NULL CHECK (version = 1), resource TEXT NOT NULL); PRAGMA application_id = ".concat(APPLICATION_ID));
            database.read("INSERT INTO semio_resource_lease VALUES (1, 1, ?) RETURNING resource", resource);
        }
        database.exec("COMMIT");
    }
    catch (error) {
        database.exec("ROLLBACK");
        throw error;
    }
}
/** 🔒️ Waits cancellably for shared or exclusive access; release only after the protected operation stops. */
function acquireResourceLease(options) {
    return __awaiter(this, void 0, void 0, function () {
        var database, started, nextProgress, acquired, _loop_1, state_1;
        var _a, _b, _c, _d;
        return __generator(this, function (_e) {
            switch (_e.label) {
                case 0:
                    options.signal.throwIfAborted();
                    if (typeof options.resource !== "string" || !options.resource.length || options.resource.length > 4096 || options.resource.includes("\0"))
                        throw new Error("Invalid lease resource");
                    if (!["shared", "exclusive"].includes(options.mode))
                        throw new Error("Invalid resource access mode");
                    if (options.timeoutMs !== undefined && (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 0))
                        throw new Error("Invalid resource lease timeout");
                    return [4 /*yield*/, openDatabase(databasePath(options.directory, options.resource))];
                case 1:
                    database = _e.sent(), started = Date.now();
                    nextProgress = 0, acquired = false;
                    _e.label = 2;
                case 2:
                    _e.trys.push([2, , 7, 8]);
                    database.exec("PRAGMA busy_timeout = 0; PRAGMA trusted_schema = OFF");
                    _loop_1 = function () {
                        var row, released_1, code, elapsedMs, progress;
                        return __generator(this, function (_f) {
                            switch (_f.label) {
                                case 0:
                                    options.signal.throwIfAborted();
                                    try {
                                        if (((_a = database.read("PRAGMA journal_mode")) === null || _a === void 0 ? void 0 : _a.journal_mode) !== "delete")
                                            throw new Error("Resource lease requires rollback journal mode");
                                        initialize(database, options.resource);
                                        database.exec(options.mode === "exclusive" ? "BEGIN EXCLUSIVE" : "BEGIN");
                                        row = database.read("SELECT version, resource FROM semio_resource_lease WHERE id = 1");
                                        if ((row === null || row === void 0 ? void 0 : row.version) !== 1 || row.resource !== options.resource || ((_b = database.read("PRAGMA application_id")) === null || _b === void 0 ? void 0 : _b.application_id) !== APPLICATION_ID)
                                            throw new Error("Resource lease identity mismatch");
                                        options.signal.throwIfAborted();
                                        acquired = true;
                                        released_1 = false;
                                        return [2 /*return*/, { value: { resource: options.resource, mode: options.mode, release: function () {
                                                        if (released_1)
                                                            return;
                                                        released_1 = true;
                                                        try {
                                                            database.exec("ROLLBACK");
                                                        }
                                                        finally {
                                                            database.close();
                                                        }
                                                    } } }];
                                    }
                                    catch (error) {
                                        try {
                                            database.exec("ROLLBACK");
                                        }
                                        catch (_g) { }
                                        code = error;
                                        if (code.code !== "SQLITE_BUSY" && code.errcode !== 5 && code.errno !== 5)
                                            throw error;
                                    }
                                    elapsedMs = Date.now() - started;
                                    if (elapsedMs >= ((_c = options.timeoutMs) !== null && _c !== void 0 ? _c : Infinity))
                                        throw new Error("Resource lease timed out: ".concat(options.resource));
                                    if (elapsedMs >= nextProgress) {
                                        progress = { resource: options.resource, mode: options.mode, elapsedMs: elapsedMs };
                                        if (options.onWait)
                                            options.onWait(progress);
                                        else
                                            console.log("Waiting for ".concat(options.mode, " access to ").concat(options.resource));
                                        nextProgress = elapsedMs + 1000;
                                    }
                                    return [4 /*yield*/, (0, promises_1.setTimeout)(Math.min(40, ((_d = options.timeoutMs) !== null && _d !== void 0 ? _d : Infinity) - elapsedMs), undefined, { signal: options.signal })];
                                case 1:
                                    _f.sent();
                                    return [2 /*return*/];
                            }
                        });
                    };
                    _e.label = 3;
                case 3: return [5 /*yield**/, _loop_1()];
                case 4:
                    state_1 = _e.sent();
                    if (typeof state_1 === "object")
                        return [2 /*return*/, state_1.value];
                    _e.label = 5;
                case 5: return [3 /*break*/, 3];
                case 6: return [3 /*break*/, 8];
                case 7:
                    if (!acquired)
                        database.close();
                    return [7 /*endfinally*/];
                case 8: return [2 /*return*/];
            }
        });
    });
}
/** 🪢️ Orders a resource set consistently and releases it in reverse order on every exit. */
function withResourceLeases(options, operation) {
    return __awaiter(this, void 0, void 0, function () {
        var resources, leases, _i, _a, entry, _b, _c, resource, _d, _e, _f, _g, lease;
        return __generator(this, function (_h) {
            switch (_h.label) {
                case 0:
                    resources = new Map(), leases = [];
                    for (_i = 0, _a = options.resources; _i < _a.length; _i++) {
                        entry = _a[_i];
                        resources.set(entry.resource, resources.get(entry.resource) === "exclusive" ? "exclusive" : entry.mode);
                    }
                    _h.label = 1;
                case 1:
                    _h.trys.push([1, , 7, 8]);
                    _b = 0, _c = __spreadArray([], resources.keys(), true).sort();
                    _h.label = 2;
                case 2:
                    if (!(_b < _c.length)) return [3 /*break*/, 5];
                    resource = _c[_b];
                    _e = (_d = leases).push;
                    return [4 /*yield*/, acquireResourceLease(__assign(__assign({}, options), { resource: resource, mode: resources.get(resource) }))];
                case 3:
                    _e.apply(_d, [_h.sent()]);
                    _h.label = 4;
                case 4:
                    _b++;
                    return [3 /*break*/, 2];
                case 5:
                    options.signal.throwIfAborted();
                    return [4 /*yield*/, operation()];
                case 6: return [2 /*return*/, _h.sent()];
                case 7:
                    for (_f = 0, _g = leases.reverse(); _f < _g.length; _f++) {
                        lease = _g[_f];
                        lease.release();
                    }
                    return [7 /*endfinally*/];
                case 8: return [2 /*return*/];
            }
        });
    });
}
