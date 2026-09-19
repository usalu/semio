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
exports.DAEMON_BUDGET_MS = exports.ORCHESTRATOR_BUDGET_MS = exports.CMD_BUDGET_MS = exports.BUILD_BUDGET_MS = void 0;
exports.buildBudgetMs = buildBudgetMs;
exports.cmdBudgetMs = cmdBudgetMs;
exports.orchestratorBudgetMs = orchestratorBudgetMs;
exports.daemonBudgetMs = daemonBudgetMs;
exports.defaultBudgetMs = defaultBudgetMs;
exports.budgetTimeoutHint = budgetTimeoutHint;
exports.terminateOwnedProcessTree = terminateOwnedProcessTree;
exports.orchestratorBudgetOpts = orchestratorBudgetOpts;
exports.daemonBudgetOpts = daemonBudgetOpts;
exports.runCmd = runCmd;
exports.runCmdStatus = runCmdStatus;
exports.tryRun = tryRun;
exports.resolveWorkspaceBin = resolveWorkspaceBin;
exports.workspaceScriptExists = workspaceScriptExists;
exports.runNodeBinStatus = runNodeBinStatus;
exports.runNodeBin = runNodeBin;
exports.semioBuildMode = semioBuildMode;
exports.semioShipEnv = semioShipEnv;
exports.cargoProfileDir = cargoProfileDir;
/** @emoji 🏃️ Process execution with opt-in wall-clock budgets for repository commands and builds,
 * the `spawnSync` runners built on them, workspace-aware `.bin`
 * resolution and the dev/ship build-mode switch. Split out of `📦️packages/🟦️typescript/🟦️.ts` so a
 * consumer that only spawns a tool (the plugin package's jco/wasm-opt steps, and through them the
 * extension store and `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery` taxonomy
 * walk into its module graph. */
var node_child_process_1 = require("node:child_process");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("./\uD83C\uDF3F\uFE0Fenvironment/\uD83D\uDFE6\uFE0F.ts");
/** 🏗️Default build budget (ms): zero leaves compilation and Cargo lock waits unlimited. Opt in via `SEMIO_BUILD_BUDGET_MS`. */
exports.BUILD_BUDGET_MS = 0;
/** ⏱️Resolves the active build-class budget: `SEMIO_BUILD_BUDGET_MS` env override, else [[BUILD_BUDGET_MS]]. */
function buildBudgetMs() {
    var _a;
    return Number((_a = process.env.SEMIO_BUILD_BUDGET_MS) !== null && _a !== void 0 ? _a : exports.BUILD_BUDGET_MS);
}
/** 🛠️Default command budget (ms): zero lets build tools and script wrappers finish. Opt in via `SEMIO_CMD_BUDGET_MS`. */
exports.CMD_BUDGET_MS = 0;
/** ⏱️Resolves the active generic-command budget: `SEMIO_CMD_BUDGET_MS` env override, else [[CMD_BUDGET_MS]]. */
function cmdBudgetMs() {
    var _a;
    return Number((_a = process.env.SEMIO_CMD_BUDGET_MS) !== null && _a !== void 0 ? _a : exports.CMD_BUDGET_MS);
}
/** 🎛️Default Nx/script orchestrator budget (ms): zero avoids imposing a parent build deadline. Opt in via `SEMIO_ORCHESTRATOR_BUDGET_MS`. */
exports.ORCHESTRATOR_BUDGET_MS = 0;
/** 🖥️Default dev-server and daemon budget (ms): zero permits long-lived sessions and their builds. Opt in via `SEMIO_DAEMON_BUDGET_MS`. */
exports.DAEMON_BUDGET_MS = 0;
/** ⏱️Resolves the active orchestrator budget: `SEMIO_ORCHESTRATOR_BUDGET_MS` env override, else [[ORCHESTRATOR_BUDGET_MS]]. */
function orchestratorBudgetMs() {
    var _a;
    return Number((_a = process.env.SEMIO_ORCHESTRATOR_BUDGET_MS) !== null && _a !== void 0 ? _a : exports.ORCHESTRATOR_BUDGET_MS);
}
/** ⏱️Resolves the active daemon budget: `SEMIO_DAEMON_BUDGET_MS` env override, else [[DAEMON_BUDGET_MS]]. */
function daemonBudgetMs() {
    var _a;
    return Number((_a = process.env.SEMIO_DAEMON_BUDGET_MS) !== null && _a !== void 0 ? _a : exports.DAEMON_BUDGET_MS);
}
/** 🧭️Selects the opt-in build budget for Cargo and command budget for other executables; both default to unlimited. */
function defaultBudgetMs(cmd) {
    return cmd === "cargo" ? buildBudgetMs() : cmdBudgetMs();
}
/** ⏱️Timeout hint for a budget-exceeded message; `cargo` commands default to the shared target-dir lock-contention hint (by far the most common real cause), everything else to a generic budget-tuning hint. An explicit `override` always wins. */
function budgetTimeoutHint(cmd, override) {
    if (override)
        return override;
    return cmd === "cargo"
        ? "Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying."
        : "Trim it, or raise its budget (`budgetMs`, `SEMIO_CMD_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`).";
}
/** 🪓️Terminates one owned subprocess tree, including descendant-created POSIX groups, before ancestry is lost. */
function terminateOwnedProcessTree(rootPid) {
    var _a;
    if (!Number.isSafeInteger(rootPid) || rootPid <= 0)
        return;
    if (process.platform === "win32") {
        (0, node_child_process_1.spawnSync)("taskkill", ["/pid", String(rootPid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
        return;
    }
    var snapshot = (0, node_child_process_1.spawnSync)("ps", ["-axo", "pid=,ppid=,pgid="], { encoding: "utf8", windowsHide: true });
    if (snapshot.status !== 0 || typeof snapshot.stdout !== "string") {
        try {
            process.kill(-rootPid, "SIGKILL");
        }
        catch (_b) { }
        try {
            process.kill(rootPid, "SIGKILL");
        }
        catch (_c) { }
        return;
    }
    var rows = snapshot.stdout.split("\n").flatMap(function (line) {
        var fields = line.trim().split(/\s+/).map(Number);
        return fields.length === 3 && fields.every(function (field) { return Number.isSafeInteger(field) && field > 0; })
            ? [{ pid: fields[0], parent: fields[1], group: fields[2] }]
            : [];
    });
    var depth = new Map([[rootPid, 0]]);
    for (var changed = true; changed;) {
        changed = false;
        for (var _i = 0, rows_1 = rows; _i < rows_1.length; _i++) {
            var row = rows_1[_i];
            if (depth.has(row.pid))
                continue;
            var parentDepth = depth.get(row.parent);
            if (parentDepth === undefined)
                continue;
            depth.set(row.pid, parentDepth + 1);
            changed = true;
        }
    }
    var owned = rows.filter(function (row) { return depth.has(row.pid); }).map(function (row) { return (__assign(__assign({}, row), { depth: depth.get(row.pid) })); });
    if (!owned.some(function (row) { return row.pid === rootPid; }))
        owned.push({ pid: rootPid, parent: 0, group: rootPid, depth: 0 });
    var ownedPids = new Set(owned.map(function (row) { return row.pid; }));
    var groups = new Map();
    for (var _d = 0, owned_1 = owned; _d < owned_1.length; _d++) {
        var row = owned_1[_d];
        if (ownedPids.has(row.group))
            groups.set(row.group, Math.max((_a = groups.get(row.group)) !== null && _a !== void 0 ? _a : 0, row.depth));
    }
    for (var _e = 0, _f = __spreadArray([], groups, true).sort(function (left, right) { return right[1] - left[1]; }); _e < _f.length; _e++) {
        var group = _f[_e][0];
        try {
            process.kill(-group, "SIGKILL");
        }
        catch (_g) { }
    }
    for (var _h = 0, _j = owned.sort(function (left, right) { return right.depth - left.depth; }); _h < _j.length; _h++) {
        var row = _j[_h];
        try {
            process.kill(row.pid, "SIGKILL");
        }
        catch (_k) { }
    }
}
/** ⏱️[[RunCmdOpts]] preset for nx/script orchestrators — [[orchestratorBudgetMs]] and full CPU [[devToolingEnv]]. */
function orchestratorBudgetOpts(extra) {
    if (extra === void 0) { extra = {}; }
    return { budgetMs: orchestratorBudgetMs(), env: (0, ____ts_2.devToolingEnv)(extra) };
}
/** ⏱️[[RunCmdOpts]] preset for dev servers and long-lived daemons — [[daemonBudgetMs]] and [[devToolingEnv]]. */
function daemonBudgetOpts(extra) {
    if (extra === void 0) { extra = {}; }
    return { budgetMs: daemonBudgetMs(), env: (0, ____ts_2.devToolingEnv)(extra) };
}
/** ⏱️Shared `spawnSync` core for [[runCmd]]/[[runCmdStatus]]: throws on spawn error, budget timeout, or signal kill (printing `[budget]` first on timeout); otherwise returns the exit status. */
function runCmdInternal(cmd, args, opts) {
    var _a, _b, _c, _d, _e;
    var budgetMs = (_a = opts.budgetMs) !== null && _a !== void 0 ? _a : defaultBudgetMs(cmd);
    var formattedArgs = __spreadArray([], args, true);
    if (cmd === "bun" || cmd === process.execPath) {
        if (formattedArgs[0] && !formattedArgs[0].startsWith("-") && !formattedArgs[0].includes("/") && !formattedArgs[0].includes("\\") && !workspaceScriptExists(formattedArgs[0])) {
            var resolved = resolveWorkspaceBin(formattedArgs[0], (_b = opts.cwd) !== null && _b !== void 0 ? _b : process.cwd());
            if (resolved) {
                formattedArgs[0] = resolved;
            }
        }
    }
    var result = (0, node_child_process_1.spawnSync)(cmd, formattedArgs, {
        stdio: "inherit",
        cwd: opts.cwd,
        env: (_c = opts.env) !== null && _c !== void 0 ? _c : process.env,
        timeout: budgetMs,
        killSignal: "SIGKILL",
        shell: (_d = opts.shell) !== null && _d !== void 0 ? _d : false,
    });
    if (result.error) {
        if (result.error.code === "ETIMEDOUT") {
            console.error("[budget] ".concat(cmd, " ").concat(args.join(" "), " exceeded ").concat(budgetMs, "ms \u2014 killed. ").concat(budgetTimeoutHint(cmd, opts.onTimeoutHint)));
        }
        throw result.error;
    }
    if (result.signal)
        throw new Error("".concat(cmd, " ").concat(args.join(" "), " killed by signal ").concat(result.signal));
    return (_e = result.status) !== null && _e !== void 0 ? _e : 1;
}
/**
 * 🏃️Runs a subprocess with inherited stdio and an opt-in wall-clock budget (default [[defaultBudgetMs]]);
 * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
 * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
 */
function runCmd(cmd, args, opts) {
    if (opts === void 0) { opts = {}; }
    var status = runCmdInternal(cmd, args, opts);
    if (status !== 0)
        throw new Error("".concat(cmd, " ").concat(args.join(" "), " exited with status ").concat(status));
}
/** 🏃️Like [[runCmd]] but returns the exit status instead of throwing on non-zero exit — for call sites
 *  that branch on it. Budget exceed still prints `[budget]` and throws (never silently returns a status). */
function runCmdStatus(cmd, args, opts) {
    if (opts === void 0) { opts = {}; }
    return runCmdInternal(cmd, args, opts);
}
/** 🏃️Like [[runCmd]] but ignores failures, including timeouts from an explicitly configured budget. */
function tryRun(cmd, args, opts) {
    if (opts === void 0) { opts = {}; }
    try {
        runCmd(cmd, args, opts);
    }
    catch (_a) {
        /* optional */
    }
}
/** 🔍️ Resolves a CLI executable in `cwd` or workspace root's `node_modules/.bin` to avoid `bun x` cwd resolution bugs on emoji/ZWJ paths. */
function resolveWorkspaceBin(binName, cwd) {
    if (cwd === void 0) { cwd = process.cwd(); }
    var shortName = binName.includes("/") ? binName.split("/").pop() : binName;
    var localBin = (0, node_path_1.join)(cwd, "node_modules", ".bin", shortName);
    if ((0, node_fs_1.existsSync)(localBin))
        return localBin;
    var rootBin = (0, node_path_1.join)((0, ____ts_1.getWorkspaceRoot)(), "node_modules", ".bin", shortName);
    if ((0, node_fs_1.existsSync)(rootBin))
        return rootBin;
    return null;
}
var workspaceScriptNames = null;
/** 📜️Whether the workspace `package.json` declares a script under `name`. A declared script is the
 * workspace's deliberate wrapper for that tool (`nx` routes through the caching bootstrap, which
 * owns the daemon-served project graph the async ES-module inference plugin needs), so `bun <name>`
 * must reach the script and never the bare `node_modules/.bin` entry of the same name. */
function workspaceScriptExists(name) {
    var _a;
    if (workspaceScriptNames === null) {
        var manifest = (0, node_path_1.join)((0, ____ts_1.getWorkspaceRoot)(), "package.json");
        var scripts = (0, node_fs_1.existsSync)(manifest) ? (_a = JSON.parse((0, node_fs_1.readFileSync)(manifest, "utf8")).scripts) !== null && _a !== void 0 ? _a : {} : {};
        workspaceScriptNames = new Set(Object.keys(scripts));
    }
    return workspaceScriptNames.has(name);
}
/** 🟢️Runs a CLI tool using `node` synchronously in `cwd`, returning status code. */
function runNodeBinStatus(args, cwd, env) {
    var _a;
    if (cwd === void 0) { cwd = process.cwd(); }
    if (env === void 0) { env = process.env; }
    var binName = args[0];
    var resolved = resolveWorkspaceBin(binName, cwd);
    var executable = resolved !== null && resolved !== void 0 ? resolved : binName;
    var result = (0, node_child_process_1.spawnSync)("node", __spreadArray([executable], args.slice(1), true), { cwd: cwd, env: env, shell: false, stdio: "inherit" });
    if (result.error) {
        console.error(result.error);
        return 1;
    }
    return (_a = result.status) !== null && _a !== void 0 ? _a : 1;
}
/** 🟢️Runs a CLI tool using `node` synchronously in `cwd`. */
function runNodeBin(args, cwd, env) {
    if (cwd === void 0) { cwd = process.cwd(); }
    if (env === void 0) { env = process.env; }
    var status = runNodeBinStatus(args, cwd, env);
    if (status !== 0)
        process.exit(status);
}
/** @emoji 🚦️ `ship` only when `SEMIO_BUILD_MODE=ship`; default is dev for local/agent loops. */
function semioBuildMode() {
    return process.env.SEMIO_BUILD_MODE === "ship" ? "ship" : "dev";
}
/** @emoji 🚀️ Env for nx/build orchestrators so spawned crate `wasm` scripts inherit ship mode. */
function semioShipEnv() {
    return __assign(__assign({}, process.env), { SEMIO_BUILD_MODE: "ship" });
}
/** @emoji 📂 Cargo output directory name for a profile (`dev` → `debug`). */
function cargoProfileDir(profile) {
    return profile === "dev" ? "debug" : profile;
}
