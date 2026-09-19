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
Object.defineProperty(exports, "__esModule", { value: true });
exports.PLAYGROUND_LOCKED_EXAMPLE_ENV = exports.OS_HUB_PORT_ENV = exports.OS_HUB_PORT = exports.PLAYGROUND_PORTS = void 0;
exports.loadFrameworkOsPlaygroundCatalog = loadFrameworkOsPlaygroundCatalog;
exports.playgroundDevPort = playgroundDevPort;
exports.playgroundDevPortString = playgroundDevPortString;
exports.playgroundTestPort = playgroundTestPort;
exports.playgroundTestPortString = playgroundTestPortString;
exports.playgroundPortEnv = playgroundPortEnv;
exports.allPlaygroundReservedPorts = allPlaygroundReservedPorts;
exports.playgroundLockedExampleIdFromEnv = playgroundLockedExampleIdFromEnv;
exports.playgroundPlayViteDefine = playgroundPlayViteDefine;
var ____ts_1 = require("./\uD83D\uDD12\uFE0Fpreferences/\uD83D\uDFE6\uFE0F.ts");
/** @emoji 🎮️ Playground identity for the whole repository: the generated OS playground catalog, the
 * dev/test port table every host binds, and the locked-example Vite define. Split out of
 * `📦️packages/🟦️typescript/🟦️.ts` so a consumer that only needs a port (the styling package's dev
 * servers, and through them `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery`
 * taxonomy walk into its module graph. */
var framework_1 = require("@semio-tech/framework");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_2 = require("../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
/**
 * 📚️ Loads the generated framework OS playground catalog (variant/plugin/aliases/ports rows).
 * Reads the registry owner's `🤖️generated/🎠️playgrounds.json` directly (rather than a static
 * TS import of the gitignored generated module) so this shared kernel never fails to load on a
 * fresh clone before `bun nx run @semio-tech/plugin-registry:generate` has ever run — callers get
 * an empty catalog in that case instead of a hard module-resolution error.
 */
function loadFrameworkOsPlaygroundCatalog() {
    var catalogPath = (0, node_path_1.join)((0, ____ts_2.getWorkspaceRoot)(), "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json");
    if (!(0, node_fs_1.existsSync)(catalogPath))
        return [];
    return JSON.parse((0, node_fs_1.readFileSync)(catalogPath, "utf8"));
}
/** @emoji 🔌️ Builds playground port table from semio.app manifests plus non-app hosts. */
function buildPlaygroundPortsFromManifests() {
    var ports = {
        storybook: { dev: 6010, env: "STORYBOOK_PORT" },
    };
    for (var _i = 0, _a = loadFrameworkOsPlaygroundCatalog(); _i < _a.length; _i++) {
        var row = _a[_i];
        ports[row.variant] = { dev: row.ports.react, test: row.ports.wgpu, env: "S_OS_PORT" };
    }
    var walk = function (directory) {
        var _a, _b, _c;
        for (var _i = 0, _d = (0, node_fs_1.readdirSync)(directory, { withFileTypes: true }); _i < _d.length; _i++) {
            var entry = _d[_i];
            if (entry.name === "node_modules" || entry.name === "target" || entry.name === "🎫️tickets" || entry.name.startsWith("."))
                continue;
            var path = (0, node_path_1.join)(directory, entry.name);
            if (entry.isDirectory()) {
                walk(path);
                continue;
            }
            if (entry.name !== "package.json")
                continue;
            try {
                var manifest = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
                var app = (_a = manifest.semio) === null || _a === void 0 ? void 0 : _a.app;
                if ((app === null || app === void 0 ? void 0 : app.hostKind) && Number.isSafeInteger((_b = app.port) === null || _b === void 0 ? void 0 : _b.dev) && ((_c = app.port) === null || _c === void 0 ? void 0 : _c.env))
                    ports[app.hostKind] = { dev: app.port.dev, test: app.port.test, env: app.port.env };
            }
            catch (_e) { }
        }
    };
    walk((0, ____ts_2.getWorkspaceRoot)());
    return ports;
}
var playgroundPortsCache = (0, framework_1.ephemeralBox)("framework.products.repo.modules.library.playground.index.ts.playgroundPortsCache", undefined);
function resolvePlaygroundPorts() {
    var _a;
    (_a = playgroundPortsCache.current) !== null && _a !== void 0 ? _a : (playgroundPortsCache.current = buildPlaygroundPortsFromManifests());
    return playgroundPortsCache.current;
}
exports.PLAYGROUND_PORTS = new Proxy({}, {
    get: function (_target, prop) {
        return resolvePlaygroundPorts()[prop];
    },
    ownKeys: function () {
        return Reflect.ownKeys(resolvePlaygroundPorts());
    },
    getOwnPropertyDescriptor: function (_target, prop) {
        var value = resolvePlaygroundPorts()[prop];
        if (value === undefined)
            return undefined;
        return { configurable: true, enumerable: true, value: value };
    },
});
/** @emoji 🔌️ Local dev port for a playground host. */
function playgroundDevPort(kind) {
    var spec = resolvePlaygroundPorts()[kind];
    if (!spec)
        throw new Error("unknown playground host kind: ".concat(kind));
    return spec.dev;
}
/** @emoji 🔌️ String dev port (vite `--port`, nx `env`). */
function playgroundDevPortString(kind) {
    return String(playgroundDevPort(kind));
}
/** @emoji 🧪️ Vitest/playwright port when set; otherwise `undefined`. */
function playgroundTestPort(kind) {
    var _a;
    return (_a = resolvePlaygroundPorts()[kind]) === null || _a === void 0 ? void 0 : _a.test;
}
/** @emoji 🧪️ String test port for nx `env` / playwright. */
function playgroundTestPortString(kind) {
    var port = playgroundTestPort(kind);
    return port === undefined ? undefined : String(port);
}
/** @emoji 🔌️ Process env var holding the dev port override. */
function playgroundPortEnv(kind) {
    var spec = resolvePlaygroundPorts()[kind];
    if (!spec)
        throw new Error("unknown playground host kind: ".concat(kind));
    return spec.env;
}
/** @emoji 🚧️ Every assigned playground dev + test port (for strict binding). */
function allPlaygroundReservedPorts() {
    var ports = new Set();
    for (var _i = 0, _a = Object.values(resolvePlaygroundPorts()); _i < _a.length; _i++) {
        var spec = _a[_i];
        ports.add(spec.dev);
        if (spec.test !== undefined)
            ports.add(spec.test);
    }
    return ports;
}
/** @emoji 🔌️ OS hub service dev port. 8787, not 6070 — 6070 is the `s` react playground's port,
 * see `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` `[[package.metadata.semio.playground]]`. */
exports.OS_HUB_PORT = 8787;
/** @emoji 🔌️ Process env var for {@link OS_HUB_PORT}. */
exports.OS_HUB_PORT_ENV = "OS_HUB_PORT";
/** @emoji 🔒️ Process env var locking a playground to one example (hides navbar dropdown). */
var ____ts_3 = require("./\uD83D\uDD12\uFE0Fpreferences/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "PLAYGROUND_LOCKED_EXAMPLE_ENV", { enumerable: true, get: function () { return ____ts_3.PLAYGROUND_LOCKED_EXAMPLE_ENV; } });
/** @emoji 🔒️ Locked example id from process env, if any. */
function playgroundLockedExampleIdFromEnv(env) {
    var _a;
    if (env === void 0) { env = process.env; }
    var raw = (_a = env[____ts_1.PLAYGROUND_LOCKED_EXAMPLE_ENV]) === null || _a === void 0 ? void 0 : _a.trim();
    return raw || undefined;
}
/** @emoji 🔌️ Vite `define` entries for playground play bundles. */
function playgroundPlayViteDefine(extra) {
    var _a;
    if (extra === void 0) { extra = {}; }
    return __assign({ "import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID": JSON.stringify((_a = playgroundLockedExampleIdFromEnv()) !== null && _a !== void 0 ? _a : ""), "import.meta.vitest": "undefined" }, extra);
}
