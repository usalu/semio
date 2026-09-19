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
exports.semioNxParallel = semioNxParallel;
exports.semioNxParallelFlag = semioNxParallelFlag;
exports.devToolingEnv = devToolingEnv;
exports.repoToolCacheEnv = repoToolCacheEnv;
var node_os_1 = require("node:os");
var ____ts_1 = require("../../\u26A1\uFE0Fcaching/\uD83D\uDFE6\uFE0F.ts");
/** 🧵️ Resolves how many Nx task slots and bounded worker pools should use on this machine. */
function semioNxParallel() {
    var _a;
    var override = (_a = process.env.SEMIO_NX_PARALLEL) === null || _a === void 0 ? void 0 : _a.trim();
    if (override) {
        var parsed = Number.parseInt(override, 10);
        if (Number.isFinite(parsed) && parsed > 0)
            return parsed;
    }
    return Math.max(1, (0, node_os_1.availableParallelism)());
}
/** 🧵️ Nx `run-many`/`affected` flag pair sized for this machine. */
function semioNxParallelFlag() {
    return ["--parallel", String(semioNxParallel())];
}
/** 🧰️Dev tooling env without IDE-injected node options. Plugin isolation is deliberately left at Nx's
 * own default: this workspace's inference plugin
 * (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`) is an async ES module — it top-level
 * `await import()`s the runtime-component closure under a revision query — so `NX_ISOLATE_PLUGINS=false`
 * makes Nx's in-process `require()` path (`runPreTasksExecution` → `getPluginsSeparated`) reject it with
 * "require() async module … is unsupported", which fails every `nx run` spawned with this env even
 * though the daemon-served project graph itself resolves. */
function devToolingEnv(extra) {
    var _a, _b, _c, _d, _e, _f, _g;
    if (extra === void 0) { extra = {}; }
    var env = __assign(__assign({}, process.env), extra);
    delete env.NODE_OPTIONS;
    delete env.VSCODE_INSPECTOR_OPTIONS;
    (_a = env.NX_NATIVE_COMMAND_RUNNER) !== null && _a !== void 0 ? _a : (env.NX_NATIVE_COMMAND_RUNNER = "false");
    (_b = env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT) !== null && _b !== void 0 ? _b : (env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT = "false");
    (_c = env.NX_TUI) !== null && _c !== void 0 ? _c : (env.NX_TUI = "false");
    (_d = env.NX_VERBOSE_LOGGING) !== null && _d !== void 0 ? _d : (env.NX_VERBOSE_LOGGING = "false");
    (_e = env.NX_PERF_LOGGING) !== null && _e !== void 0 ? _e : (env.NX_PERF_LOGGING = "false");
    (_f = env.NX_NATIVE_LOGGING) !== null && _f !== void 0 ? _f : (env.NX_NATIVE_LOGGING = "nx=warn");
    (_g = env.NX_PARALLEL) !== null && _g !== void 0 ? _g : (env.NX_PARALLEL = String(semioNxParallel()));
    return env;
}
/** ⚡️ Routes Go's build cache and Playwright's browser downloads into the shared cache root; never overrides a value the caller set explicitly. */
function repoToolCacheEnv(repoRoot, extra) {
    var _a, _b;
    if (extra === void 0) { extra = {}; }
    var env = __assign({}, extra);
    (_a = env.GOCACHE) !== null && _a !== void 0 ? _a : (env.GOCACHE = (0, ____ts_1.repoCacheDirectory)(repoRoot, "go"));
    (_b = env.PLAYWRIGHT_BROWSERS_PATH) !== null && _b !== void 0 ? _b : (env.PLAYWRIGHT_BROWSERS_PATH = (0, ____ts_1.repoCacheDirectory)(repoRoot, "tools", "ms-playwright"));
    return env;
}
