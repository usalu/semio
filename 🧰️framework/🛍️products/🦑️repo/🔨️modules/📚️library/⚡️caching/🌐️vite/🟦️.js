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
Object.defineProperty(exports, "__esModule", { value: true });
exports.buildViteArtifact = buildViteArtifact;
exports.serveVite = serveVite;
var promises_1 = require("node:fs/promises");
var node_http_1 = require("node:http");
var node_module_1 = require("node:module");
var node_path_1 = require("node:path");
var ____ts_1 = require("../\uD83D\uDCE6\uFE0Fartifacts/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../\uD83D\uDCE6\uFE0Fartifacts/\uD83D\uDDC2\uFE0Ffiles/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("./\uD83E\uDDFE\uFE0Fsession/\uD83D\uDFE6\uFE0F.ts");
/** 🌐️ Bundles one site and publishes its complete deliverable only after Vite succeeds. */
function buildViteArtifact(options) {
    return __awaiter(this, void 0, void 0, function () {
        var temporaryRoot, temporary, cli, child_1, force_1, cancel, status_1, files;
        var _a, _b, _c, _d, _e, _f, _g, _h;
        return __generator(this, function (_j) {
            switch (_j.label) {
                case 0:
                    (_a = options.signal) === null || _a === void 0 ? void 0 : _a.throwIfAborted();
                    temporaryRoot = (_b = options.temporaryRoot) !== null && _b !== void 0 ? _b : (0, node_path_1.join)((0, node_path_1.dirname)(options.output), ".staging");
                    return [4 /*yield*/, (0, promises_1.mkdir)(temporaryRoot, { recursive: true })];
                case 1:
                    _j.sent();
                    return [4 /*yield*/, (0, promises_1.mkdtemp)((0, node_path_1.join)(temporaryRoot, "vite-build-"))];
                case 2:
                    temporary = _j.sent();
                    _j.label = 3;
                case 3:
                    _j.trys.push([3, , 10, 12]);
                    cli = (0, node_path_1.join)((0, node_path_1.dirname)((0, node_module_1.createRequire)((0, node_path_1.join)(options.workspace, "package.json")).resolve("vite/package.json")), "bin/vite.js");
                    (_c = options.signal) === null || _c === void 0 ? void 0 : _c.throwIfAborted();
                    console.log("Bundling ".concat(options.owner));
                    child_1 = Bun.spawn([process.execPath, cli, "build", "--config", options.config, "--configLoader", "bundle", "--outDir", temporary, "--emptyOutDir"], { cwd: options.root, env: __assign(__assign({}, process.env), options.environment), stdout: "inherit", stderr: "inherit", stdin: "ignore" });
                    cancel = function () { child_1.kill("SIGTERM"); force_1 !== null && force_1 !== void 0 ? force_1 : (force_1 = setTimeout(function () { return child_1.kill("SIGKILL"); }, 2000)); };
                    (_d = options.signal) === null || _d === void 0 ? void 0 : _d.addEventListener("abort", cancel, { once: true });
                    if ((_e = options.signal) === null || _e === void 0 ? void 0 : _e.aborted)
                        cancel();
                    _j.label = 4;
                case 4:
                    _j.trys.push([4, , 6, 7]);
                    return [4 /*yield*/, child_1.exited];
                case 5:
                    status_1 = _j.sent();
                    return [3 /*break*/, 7];
                case 6:
                    (_f = options.signal) === null || _f === void 0 ? void 0 : _f.removeEventListener("abort", cancel);
                    clearTimeout(force_1);
                    return [7 /*endfinally*/];
                case 7:
                    (_g = options.signal) === null || _g === void 0 ? void 0 : _g.throwIfAborted();
                    if (status_1 !== 0)
                        throw new Error("Vite build failed for ".concat(options.owner, " (").concat(status_1, ")"));
                    return [4 /*yield*/, (0, ____ts_2.collectArtifactFiles)(temporary, options.signal)];
                case 8:
                    files = _j.sent();
                    (_h = options.signal) === null || _h === void 0 ? void 0 : _h.throwIfAborted();
                    return [4 /*yield*/, (0, ____ts_1.stageArtifacts)(options.output, options.owner, files, { signal: options.signal })];
                case 9:
                    _j.sent();
                    console.log("Published ".concat(options.owner, ": ").concat(files.size, " files at ").concat(options.output));
                    return [3 /*break*/, 12];
                case 10: return [4 /*yield*/, (0, promises_1.rm)(temporary, { recursive: true, force: true })];
                case 11:
                    _j.sent();
                    return [7 /*endfinally*/];
                case 12: return [2 /*return*/];
            }
        });
    });
}
/** 🖥️ Owns one Vite listener through readiness and cancellation without reusing ambient services. */
function serveVite(options) {
    return __awaiter(this, void 0, void 0, function () {
        var session, createServer, listener, sockets, server, address, host, closed_1, _i, sockets_1, socket;
        var _a;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    options.signal.throwIfAborted();
                    if (!Number.isInteger(options.port) || options.port < 0 || options.port > 65535)
                        throw new Error("Invalid Vite service port");
                    session = options.session ? (0, ____ts_3.parseServiceSession)(options.session) : undefined;
                    return [4 /*yield*/, Promise.resolve().then(function () { return require("vite"); })];
                case 1:
                    createServer = (_b.sent()).createServer;
                    listener = (0, node_http_1.createServer)(), sockets = new Set();
                    listener.on("connection", function (socket) { sockets.add(socket); socket.once("close", function () { return sockets.delete(socket); }); });
                    _b.label = 2;
                case 2:
                    _b.trys.push([2, , 7, 9]);
                    return [4 /*yield*/, createServer({ root: options.root, configFile: options.config, configLoader: (_a = options.configLoader) !== null && _a !== void 0 ? _a : "bundle", server: { host: options.host, port: options.port, strictPort: true, open: false, middlewareMode: { server: listener }, hmr: { server: listener } }, plugins: session ? [{
                                    name: "semio-service-readiness",
                                    configureServer: function (server) {
                                        server.middlewares.use(function (request, response, next) {
                                            if (request.url !== ____ts_3.SERVICE_READY_ENDPOINT || request.method !== "GET")
                                                return next();
                                            response.setHeader("content-type", "application/json");
                                            response.setHeader("cache-control", "no-store");
                                            response.end(JSON.stringify(session));
                                        });
                                    },
                                }] : [] })];
                case 3:
                    server = _b.sent();
                    options.signal.throwIfAborted();
                    listener.on("request", function (request, response) { return server.middlewares(request, response); });
                    return [4 /*yield*/, new Promise(function (resolve, reject) {
                            listener.once("error", reject);
                            listener.listen(options.port, options.host, function () { listener.removeListener("error", reject); resolve(); });
                        })];
                case 4:
                    _b.sent();
                    options.signal.throwIfAborted();
                    address = listener.address();
                    if (!address || typeof address === "string")
                        throw new Error("Vite did not bind a TCP listener");
                    host = options.host === "0.0.0.0" ? "127.0.0.1" : options.host.includes(":") ? "[".concat(options.host, "]") : options.host;
                    return [4 /*yield*/, options.ready("http://".concat(host, ":").concat(address.port, "/"))];
                case 5:
                    _b.sent();
                    return [4 /*yield*/, new Promise(function (resolve) {
                            if (options.signal.aborted)
                                resolve();
                            else
                                options.signal.addEventListener("abort", function () { return resolve(); }, { once: true });
                        })];
                case 6:
                    _b.sent();
                    return [3 /*break*/, 9];
                case 7:
                    closed_1 = new Promise(function (resolve) { return listener.close(function () { return resolve(); }); });
                    for (_i = 0, sockets_1 = sockets; _i < sockets_1.length; _i++) {
                        socket = sockets_1[_i];
                        socket.destroy();
                    }
                    return [4 /*yield*/, Promise.all([closed_1, server === null || server === void 0 ? void 0 : server.close()])];
                case 8:
                    _b.sent();
                    return [7 /*endfinally*/];
                case 9: return [2 /*return*/];
            }
        });
    });
}
