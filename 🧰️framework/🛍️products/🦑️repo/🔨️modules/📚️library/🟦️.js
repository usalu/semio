"use strict";
//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: bundle scripts, policy runner, linters, dependency-boundary lint.
//#endregion 🧲️Header
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
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
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
exports.BUNDLE_WIP_SUBJECT_RE = exports.MICRO_COMMIT_ULOC_TOTAL_EMOJI = exports.MICRO_COMMIT_ULOC_HEADER = exports.COMMIT_METRIC_TOTAL_EMOJI = exports.COMMIT_METRIC_HEADER = exports.METRIC_KINDS = exports.METRIC_KIND_SIZE = exports.METRIC_KIND_ULOC = exports.WGPU_DEV_LEGACY_ENTRY_PATH = exports.SEMIO_ASSET_BASE_URL_ENV = exports.SEMIO_ASSET_SERVER_PORT = exports.frameworkOsLockedPrefsEnv = exports.SEMIO_DEFAULT_EXAMPLE_ENV = exports.SEMIO_BRAND_ENV = exports.SEMIO_LOCKED_APPEARANCE_ENV = exports.SEMIO_LOCKED_THEME_ENV = exports.SEMIO_LOCKED_TERMINOLOGY_ENV = exports.SEMIO_LOCKED_LOCALE_ENV = exports.COVERAGE_EXCLUDE_REASONS = exports.COVERAGE_EXCLUDE_GLOBS = exports.ExactCargoLawError = exports.EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS = exports.EXACT_CARGO_ACTIVE_LEASE_MANIFEST = exports.EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX = exports.DEFAULT_TEST_BUDGET_MS = exports.TEST_LEVEL_BUDGET_MS = exports.TEST_LEVELS = exports.findRepoRoot = exports.ScriptRouter = exports.BundleScript = exports.Script = exports.POLICY_ALLOWLIST_FILENAME = exports.LAYERING_BASELINE_REL_PATH = exports.DefinitionLinter = exports.SectionLinter = exports.FileLinter = exports.FolderLinter = exports.BundleLinter = exports.TechnologyLinter = exports.BaseLinter = exports.MAP_CACHE_DIR_NAME = exports.SPACE_DATA_DIR_NAME = exports.HUB_DATA_DIR_NAME = exports.REPO_META_DIR_NAME = exports.SEMIO_ROOT_DIR = exports.wasmBindgenVersion = exports.semioNxParallelFlag = exports.semioNxParallel = exports.repoToolCacheEnv = exports.devToolingEnv = void 0;
exports.registerPlaygroundSiteBuildCommands = exports.playgroundVariantsFromCrateManifest = exports.loadFrameworkOsPlaygroundSelections = exports.ArtifactScaffoldError = exports.authorArtifactScaffold = exports.resolveCargoProviderBinding = exports.projectCargoProviderManifest = exports.inspectMutationMetadataSource = exports.cargoProviderTomlParser = exports.BUNDLE_DATE_SECTION_RE = void 0;
exports.readStableBuildFile = readStableBuildFile;
exports.getSemioRoot = getSemioRoot;
exports.getRepoMetaDir = getRepoMetaDir;
exports.getHubDataDir = getHubDataDir;
exports.getSpaceDataDir = getSpaceDataDir;
exports.getMapCacheDir = getMapCacheDir;
exports.fixtureItemsOf = fixtureItemsOf;
exports.resolveCliBin = resolveCliBin;
exports.resolveMcpBin = resolveMcpBin;
exports.runCliGraphql = runCliGraphql;
exports.getLibRoot = getLibRoot;
exports.resolveFolderByPath = resolveFolderByPath;
exports.resolveBundleByName = resolveBundleByName;
exports.resolveTechnologyByName = resolveTechnologyByName;
exports.defineLint = defineLint;
exports.isAdapterBoundaryFile = isAdapterBoundaryFile;
exports.shouldSkipDependencyBoundaryFile = shouldSkipDependencyBoundaryFile;
exports.loadThirdPartyDeps = loadThirdPartyDeps;
exports.parseTsImportSpecs = parseTsImportSpecs;
exports.dependencyBoundaryBreachesForFile = dependencyBoundaryBreachesForFile;
exports.dependencyBoundaryBreachesForBundleDir = dependencyBoundaryBreachesForBundleDir;
exports.layeringReferences = layeringReferences;
exports.loadLayeringBaseline = loadLayeringBaseline;
exports.layeringCounts = layeringCounts;
exports.layeringBreaches = layeringBreaches;
exports.writeLayeringBaseline = writeLayeringBaseline;
exports.policyDiscoveredAllowlist = policyDiscoveredAllowlist;
exports.scriptExportsPolicy = scriptExportsPolicy;
exports.resolvePolicyScriptEntity = resolvePolicyScriptEntity;
exports.resolveLintScriptEntity = resolvePolicyScriptEntity;
exports.runPolicyScript = runPolicyScript;
exports.runLintScript = runPolicyScript;
exports.formatBreachReport = formatBreachReport;
exports.runPolicyExit = runPolicyExit;
exports.dispatchPolicyArgv = dispatchPolicyArgv;
exports.runPolicyOnlyMain = runPolicyOnlyMain;
exports.runBundleScriptMain = runBundleScriptMain;
exports.runWorkspaceScriptMain = runWorkspaceScriptMain;
exports.dispatchSubcommand = dispatchSubcommand;
exports.resolveTestLevel = resolveTestLevel;
exports.testLevelRank = testLevelRank;
exports.testLevelAtLeast = testLevelAtLeast;
exports.atTestLevel = atTestLevel;
exports.testLevelBudgetMs = testLevelBudgetMs;
exports.testLevelBudgetSeconds = testLevelBudgetSeconds;
exports.goLevelTestArgs = goLevelTestArgs;
exports.canonicalGoPlan = canonicalGoPlan;
exports.runCanonicalGoTests = runCanonicalGoTests;
exports.runCanonicalGoBuild = runCanonicalGoBuild;
exports.vitestLevelArgs = vitestLevelArgs;
exports.bunTestLevelArgs = bunTestLevelArgs;
exports.pytestLevelArgs = pytestLevelArgs;
exports.dotnetLevelArgs = dotnetLevelArgs;
exports.playwrightTestTimeoutMs = playwrightTestTimeoutMs;
exports.runTestBudgeted = runTestBudgeted;
exports.capturedTestFailureDiagnostics = capturedTestFailureDiagnostics;
exports.resolveCargoPackageName = resolveCargoPackageName;
exports.resolveCargoPackageNames = resolveCargoPackageNames;
exports.partitionNextestExecutionFilters = partitionNextestExecutionFilters;
exports.nextestArtifactLocation = nextestArtifactLocation;
exports.runCargoTestBudgeted = runCargoTestBudgeted;
exports.runProbe = runProbe;
exports.exactCargoGeneratedOutputHasLiveLease = exactCargoGeneratedOutputHasLiveLease;
exports.exactExecutableFingerprint = exactExecutableFingerprint;
exports.runExactCargoLawProcess = runExactCargoLawProcess;
exports.runExactCargoLaws = runExactCargoLaws;
exports.spawnDaemon = spawnDaemon;
exports.waitForHttpUrl = waitForHttpUrl;
exports.coverageEnabled = coverageEnabled;
exports.coverageDir = coverageDir;
exports.coverageSlug = coverageSlug;
exports.isCoverageExcluded = isCoverageExcluded;
exports.goCoverageArgs = goCoverageArgs;
exports.pytestCoverageArgs = pytestCoverageArgs;
exports.dotnetCoverageArgs = dotnetCoverageArgs;
exports.parseLcov = parseLcov;
exports.mergeLcov = mergeLcov;
exports.renderLcov = renderLcov;
exports.goProfileToLcov = goProfileToLcov;
exports.summarizeCoverage = summarizeCoverage;
exports.enforceCoverageThreshold = enforceCoverageThreshold;
exports.runCargoLint = runCargoLint;
exports.withViteConfigLoader = withViteConfigLoader;
exports.runBun = runBun;
exports.runBunxStatus = runBunxStatus;
exports.runBunx = runBunx;
exports.spawnBunx = spawnBunx;
exports.spawnBun = spawnBun;
exports.runViteDev = runViteDev;
exports.runViteBuild = runViteBuild;
exports.vitestRunArguments = vitestRunArguments;
exports.runVitest = runVitest;
exports.frameworkOsPlaygroundDefaultPort = frameworkOsPlaygroundDefaultPort;
exports.resolveFrameworkOsPlaygroundPlugin = resolveFrameworkOsPlaygroundPlugin;
exports.frameworkOsPlaygroundDevEnv = frameworkOsPlaygroundDevEnv;
exports.playPollingEnv = playPollingEnv;
exports.consumePlaygroundExampleArgv = consumePlaygroundExampleArgv;
exports.runPlaywright = runPlaywright;
exports.isDevPortInUse = isDevPortInUse;
exports.devServerUrl = devServerUrl;
exports.wgpuDevPlayUrl = wgpuDevPlayUrl;
exports.probeWgpuDevPort = probeWgpuDevPort;
exports.stopTrunkDevPort = stopTrunkDevPort;
exports.devServerPlayEntry = devServerPlayEntry;
exports.describeDevPortOccupant = describeDevPortOccupant;
exports.canReuseDevPort = canReuseDevPort;
exports.resolveDevPort = resolveDevPort;
exports.runViteBunxDev = runViteBunxDev;
exports.runViteBunxDevPlain = runViteBunxDevPlain;
exports.runCargo = runCargo;
exports.resolveWasmBindgenBin = resolveWasmBindgenBin;
exports.wasmBuildEnvironment = wasmBuildEnvironment;
exports.wasmPackEnvironment = wasmPackEnvironment;
exports.wasmOutputDirectory = wasmOutputDirectory;
exports.wasmBuildArguments = wasmBuildArguments;
exports.runWasmPackWebBuild = runWasmPackWebBuild;
exports.selectComponentWasmProfile = selectComponentWasmProfile;
exports.parseExtensionCargoManifest = parseExtensionCargoManifest;
exports.runExtensionComponentPackage = runExtensionComponentPackage;
exports.scriptPathFromUrl = scriptPathFromUrl;
exports.shouldSkipPathForUloc = shouldSkipPathForUloc;
exports.classifyPathForMetrics = classifyPathForMetrics;
exports.langMetricsEmoji = langMetricsEmoji;
exports.gitRepoRoot = gitRepoRoot;
exports.countJsonKeys = countJsonKeys;
exports.diffJsonUloc = diffJsonUloc;
exports.countUnifiedLocForFile = countUnifiedLocForFile;
exports.isUlocCachePlausible = isUlocCachePlausible;
exports.scanRepoUnifiedLocUncached = scanRepoUnifiedLocUncached;
exports.scanRepoSizeByLanguageUncached = scanRepoSizeByLanguageUncached;
exports.scanRepoUnifiedLoc = scanRepoUnifiedLoc;
exports.scanRepoSizeByLanguage = scanRepoSizeByLanguage;
exports.createDefaultUlocRunner = createDefaultUlocRunner;
exports.createDefaultMetricsRunner = createDefaultMetricsRunner;
exports.metricsRunnerFromUloc = metricsRunnerFromUloc;
exports.splitGitNumstatDelta = splitGitNumstatDelta;
exports.pathUnderPrefixes = pathUnderPrefixes;
exports.gitRangeNumstat = gitRangeNumstat;
exports.accumulateGitDeltasFromNumstat = accumulateGitDeltasFromNumstat;
exports.accumulateUlocDeltasFromPaths = accumulateUlocDeltasFromPaths;
exports.accumulateSizeDeltasFromPaths = accumulateSizeDeltasFromPaths;
exports.sumGitLangDeltas = sumGitLangDeltas;
exports.gitDeltaLineTotal = gitDeltaLineTotal;
exports.appendGitDeltaSuffix = appendGitDeltaSuffix;
exports.formatBundleMetricSuffixes = formatBundleMetricSuffixes;
exports.formatBundleUlocSuffix = formatBundleUlocSuffix;
exports.roundSignificant = roundSignificant;
exports.formatMetricLocCount = formatMetricLocCount;
exports.formatMetricRatio = formatMetricRatio;
exports.langMetricsSlug = langMetricsSlug;
exports.formatMetricSizeCount = formatMetricSizeCount;
exports.formatMetricBody = formatMetricBody;
exports.formatUlocMetricsBody = formatUlocMetricsBody;
exports.countUnifiedLocUnderPathPrefixes = countUnifiedLocUnderPathPrefixes;
exports.countBytesUnderPathPrefixes = countBytesUnderPathPrefixes;
exports.buildCommitMetrics = buildCommitMetrics;
exports.buildCommitMetricsForRange = buildCommitMetricsForRange;
exports.buildMicroCommitMetrics = buildMicroCommitMetrics;
exports.buildMicroCommitMetricsForRange = buildMicroCommitMetricsForRange;
exports.sumMicroCommitLangMetrics = sumMicroCommitLangMetrics;
exports.formatCommitMetricLine = formatCommitMetricLine;
exports.formatMicroCommitMetricLine = formatMicroCommitMetricLine;
exports.formatCommitMetricsLines = formatCommitMetricsLines;
exports.formatMicroCommitMetricsLines = formatMicroCommitMetricsLines;
exports.gitDeltaSumsEqual = gitDeltaSumsEqual;
exports.validateMicroCommitLangMetricsDeltaSum = validateMicroCommitLangMetricsDeltaSum;
exports.validateCommitMetricsDeltaSum = validateCommitMetricsDeltaSum;
exports.digestMicroCommitMessage = digestMicroCommitMessage;
exports.safeGitEnv = safeGitEnv;
exports.gitSpawnEnv = gitSpawnEnv;
exports.currentBranch = currentBranch;
exports.branchValidationError = branchValidationError;
exports.normalizeEmojiPresentation = normalizeEmojiPresentation;
exports.canonicalWipLine1Base = canonicalWipLine1Base;
exports.extractCounterFromSubject = extractCounterFromSubject;
exports.extractNumericCounterFromSubject = extractNumericCounterFromSubject;
exports.line1BaseFromBundleTag = line1BaseFromBundleTag;
exports.contributorWipLine1Base = contributorWipLine1Base;
exports.bumpCounterFromHistory = bumpCounterFromHistory;
exports.bumpCounterFromSubject = bumpCounterFromSubject;
exports.bulletLeadEmoji = bulletLeadEmoji;
exports.formatMicroCommitBulletLine = formatMicroCommitBulletLine;
exports.normalizeBulletLines = normalizeBulletLines;
exports.bulletEmojiValidationError = bulletEmojiValidationError;
exports.pathTokensForBulletCoverage = pathTokensForBulletCoverage;
exports.uncoveredStagedAreas = uncoveredStagedAreas;
exports.buildMicroCommitMessage = buildMicroCommitMessage;
exports.writeMicroCommitTemplates = writeMicroCommitTemplates;
exports.shouldRefreshPreparedCommitMessage = shouldRefreshPreparedCommitMessage;
exports.clearGitCommitDraftState = clearGitCommitDraftState;
exports.clearMicroCommitTemplatesOnly = clearMicroCommitTemplatesOnly;
exports.wipeAfterCommit = wipeAfterCommit;
exports.handlePrepareCommitMsg = handlePrepareCommitMsg;
exports.resolveMicroCommitBunBin = resolveMicroCommitBunBin;
exports.renderMicroCommitGitHook = renderMicroCommitGitHook;
exports.installMicroCommitGitHooks = installMicroCommitGitHooks;
exports.resetMicroCommitTemplates = resetMicroCommitTemplates;
exports.runMicroCommit = runMicroCommit;
exports.parseCommitSteps = parseCommitSteps;
exports.isCommitPrepareOnly = isCommitPrepareOnly;
exports.formatGitSignedTagCommand = formatGitSignedTagCommand;
exports.formatCommitPrepareCommands = formatCommitPrepareCommands;
exports.formatCommitPrepareAgentReply = formatCommitPrepareAgentReply;
exports.findLastBundleWipCommit = findLastBundleWipCommit;
exports.formatBundleTagName = formatBundleTagName;
exports.formatBundleSubject = formatBundleSubject;
exports.leadingEmojiClusterCount = leadingEmojiClusterCount;
exports.emojiClusterCountInLine = emojiClusterCountInLine;
exports.normalizeBundleScopeLabel = normalizeBundleScopeLabel;
exports.labelPathTokens = labelPathTokens;
exports.bundleScopeLabelError = bundleScopeLabelError;
exports.inferPathPrefixesForBundleLabel = inferPathPrefixesForBundleLabel;
exports.extractBundleDateLineFromSubject = extractBundleDateLineFromSubject;
exports.extractBundleDateLineFromCommitBody = extractBundleDateLineFromCommitBody;
exports.extractBundleDateLineFromCommit = extractBundleDateLineFromCommit;
exports.pathsFromNumstatRow = pathsFromNumstatRow;
exports.buildBundlePathPrefixSets = buildBundlePathPrefixSets;
exports.pathMatchesBundleIndex = pathMatchesBundleIndex;
exports.normalizeBundleDateLine = normalizeBundleDateLine;
exports.formatBundleDateLine = formatBundleDateLine;
exports.buildBundleDateDeltasMap = buildBundleDateDeltasMap;
exports.buildBundleDateSizeDeltasMap = buildBundleDateSizeDeltasMap;
exports.sortCommitBundlesByEditTotal = sortCommitBundlesByEditTotal;
exports.isBundleScopeLine = isBundleScopeLine;
exports.commitBundleBodyError = commitBundleBodyError;
exports.parseCommitBundleBody = parseCommitBundleBody;
exports.resolveBundleIndicesForNumstatRow = resolveBundleIndicesForNumstatRow;
exports.partitionRangeDeltasByBundle = partitionRangeDeltasByBundle;
exports.partitionRangeSizeDeltasByBundle = partitionRangeSizeDeltasByBundle;
exports.validateBundleDayDeltasAttribution = validateBundleDayDeltasAttribution;
exports.validateBundleCommitAttribution = validateBundleCommitAttribution;
exports.buildCommitMessage = buildCommitMessage;
exports.resolveCommitBundleRange = resolveCommitBundleRange;
exports.commitHistoryCompareLines = commitHistoryCompareLines;
exports.bulletMatchesCommitHistory = bulletMatchesCommitHistory;
exports.validateBundleBulletsFresh = validateBundleBulletsFresh;
exports.emitCommitLog = emitCommitLog;
exports.emitCommitDiff = emitCommitDiff;
exports.runCommit = runCommit;
exports.exportAnimatedSvgToMp4 = exportAnimatedSvgToMp4;
//#region 🔌️Adapters
var ____ts_1 = require("./\uD83C\uDFC3\uFE0Fprocess/\uD83C\uDF3F\uFE0Fenvironment/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "devToolingEnv", { enumerable: true, get: function () { return ____ts_1.devToolingEnv; } });
Object.defineProperty(exports, "repoToolCacheEnv", { enumerable: true, get: function () { return ____ts_1.repoToolCacheEnv; } });
Object.defineProperty(exports, "semioNxParallel", { enumerable: true, get: function () { return ____ts_1.semioNxParallel; } });
Object.defineProperty(exports, "semioNxParallelFlag", { enumerable: true, get: function () { return ____ts_1.semioNxParallelFlag; } });
var framework_1 = require("@semio-tech/framework");
var node_child_process_1 = require("node:child_process");
var node_fs_1 = require("node:fs");
var node_os_1 = require("node:os");
var node_path_1 = require("node:path");
var node_module_1 = require("node:module");
var node_url_1 = require("node:url");
var node_crypto_1 = require("node:crypto");
var ___script_ts_1 = require("./\u26A1\uFE0Fcaching/\uD83D\uDE80\uFE0Fbootstrap/\uD83D\uDEE0\uFE0Ftools/\uD83D\uDD78\uFE0Fwasm/\uD83D\uDCDC\uFE0Fscript.ts");
var ____ts_2 = require("./\u26A1\uFE0Fcaching/\uD83E\uDD80\uFE0Fcargo/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "wasmBindgenVersion", { enumerable: true, get: function () { return ____ts_2.wasmBindgenVersion; } });
var ____ts_3 = require("./\u26A1\uFE0Fcaching/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("./\uD83D\uDD0D\uFE0Fdiscovery/\uD83D\uDFE6\uFE0F.ts");
var ____ts_5 = require("./\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts");
var ____ts_6 = require("./\uD83C\uDFC3\uFE0Fprocess/\uD83D\uDFE6\uFE0F.ts");
exports.SEMIO_ROOT_DIR = ".🧬semio";
exports.REPO_META_DIR_NAME = "🦑️repo";
exports.HUB_DATA_DIR_NAME = "🌐hub";
exports.SPACE_DATA_DIR_NAME = "🔗space";
exports.MAP_CACHE_DIR_NAME = "🗺️map";
/** 🧊️ Captures one stable regular file through its retained descriptor and a shared byte admission. */
function readStableBuildFile(path, maximum, admission, check) {
    check();
    var info = (0, node_fs_1.lstatSync)(path);
    if (!info.isFile() || info.isSymbolicLink())
        throw new Error("build input: not a regular file (".concat(path, ")"));
    if (info.size > maximum)
        throw new Error("build input: ".concat(path, " is ").concat(info.size, " bytes, over the ").concat(maximum, "-byte bound"));
    var file = (0, node_fs_1.openSync)(path, "r");
    try {
        var before = (0, node_fs_1.fstatSync)(file);
        var same = function (left, right) { return left.isFile() && !left.isSymbolicLink() && left.dev === right.dev && left.ino === right.ino && left.size === right.size && left.mtimeMs === right.mtimeMs && left.ctimeMs === right.ctimeMs; };
        if (!same(before, info) || before.size > maximum || before.size > admission.remaining)
            throw new Error("build input: file changed or exceeds aggregate bound");
        admission.remaining -= before.size;
        var bytes = Buffer.alloc(before.size);
        for (var offset = 0; offset < bytes.byteLength;) {
            check();
            var count = (0, node_fs_1.readSync)(file, bytes, offset, Math.min(64 * 1024, bytes.byteLength - offset), offset);
            if (!count)
                throw new Error("build input: file shortened");
            offset += count;
        }
        if (!same((0, node_fs_1.fstatSync)(file), before) || !same((0, node_fs_1.lstatSync)(path), before))
            throw new Error("build input: file changed while reading");
        return bytes;
    }
    finally {
        (0, node_fs_1.closeSync)(file);
    }
}
/** 🧬️Workspace-local semio root (`.🧬semio/`). */
function getSemioRoot(repoRoot) {
    return (0, node_path_1.join)(repoRoot, exports.SEMIO_ROOT_DIR);
}
/** 🦑️Repo product meta directory (`.🧬semio/🦑️repo/`). */
function getRepoMetaDir(repoRoot) {
    return (0, node_path_1.join)(getSemioRoot(repoRoot), exports.REPO_META_DIR_NAME);
}
/** 🌐Hub data directory (`.🧬semio/🌐hub/`). */
function getHubDataDir(repoRoot) {
    return (0, node_path_1.join)(getSemioRoot(repoRoot), exports.HUB_DATA_DIR_NAME);
}
/** 🔗Space instance data directory (`.🧬semio/🔗space/<instance>/`). */
function getSpaceDataDir(repoRoot, instance) {
    var base = (0, node_path_1.join)(getSemioRoot(repoRoot), exports.SPACE_DATA_DIR_NAME);
    return instance ? (0, node_path_1.join)(base, instance) : base;
}
/** 🗺️Per-provider map tile cache (`.🧬semio/🗺️map/<provider>/`). */
function getMapCacheDir(repoRoot, provider) {
    return (0, node_path_1.join)(getSemioRoot(repoRoot), exports.MAP_CACHE_DIR_NAME, provider);
}
//#endregion 🔖️breach
//#region 🔖️cli
/** @emoji 📎️ Reads `{ hash, items }` collection blocks from kit snapshot JSON. */
function fixtureItemsOf(node) {
    if (node && typeof node === "object" && Array.isArray(node.items)) {
        return node.items;
    }
    return [];
}
function defaultCliBin(root) {
    var win = process.platform === "win32";
    return (0, node_path_1.join)(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client", win ? "client.exe" : "client");
}
function resolveCliBin(root) {
    var _a;
    if (root === void 0) { root = (0, ____ts_5.getWorkspaceRoot)(); }
    var fromEnv = (_a = process.env.REPO_CLI_BIN) === null || _a === void 0 ? void 0 : _a.trim();
    if (fromEnv)
        return (0, node_path_1.resolve)(fromEnv);
    return defaultCliBin(root);
}
/** 🗃️ The MCP server is build output, so it lives in the marked repository cache and never in the tree. */
function defaultMcpBin(root) {
    var win = process.platform === "win32";
    return (0, node_path_1.join)(root, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", win ? "repo.exe" : "repo");
}
/** 🔌️Resolves the native repo MCP binary without colliding with the repo CLI executable. */
function resolveMcpBin(root) {
    var _a;
    if (root === void 0) { root = (0, ____ts_5.getWorkspaceRoot)(); }
    var fromEnv = (_a = process.env.REPO_MCP_BIN) === null || _a === void 0 ? void 0 : _a.trim();
    if (fromEnv)
        return (0, node_path_1.resolve)(fromEnv);
    return defaultMcpBin(root);
}
/** 📡️Runs repo client with `--json` and returns parsed GraphQL payload (`data` object). */
function runCliGraphql(query, variables, options) {
    var _a, _b, _c, _d, _e, _f, _g, _h, _j;
    if (variables === void 0) { variables = {}; }
    var root = (_a = options === null || options === void 0 ? void 0 : options.repoRoot) !== null && _a !== void 0 ? _a : (0, ____ts_5.getWorkspaceRoot)();
    var cwd = (_b = options === null || options === void 0 ? void 0 : options.cwd) !== null && _b !== void 0 ? _b : root;
    var bin = resolveCliBin(root);
    var vars = JSON.stringify(variables !== null && variables !== void 0 ? variables : {});
    var args = ["--repo", root, "--json", "graphql", "--query", query, "-v", vars];
    var stdout;
    try {
        stdout = (0, node_child_process_1.execFileSync)(bin, args, {
            cwd: cwd,
            encoding: "utf8",
            maxBuffer: 64 * 1024 * 1024,
        });
    }
    catch (e) {
        var err = e;
        var msg = (_f = (_e = (_d = (_c = err.stderr) === null || _c === void 0 ? void 0 : _c.toString) === null || _d === void 0 ? void 0 : _d.call(_c)) !== null && _e !== void 0 ? _e : err.message) !== null && _f !== void 0 ? _f : String(e);
        throw new Error("[repo/cli] exit ".concat((_g = err.status) !== null && _g !== void 0 ? _g : "?", ": ").concat(msg));
    }
    var lines = stdout
        .split(/\r?\n/)
        .map(function (l) { return l.trim(); })
        .filter(Boolean);
    var last;
    for (var _i = 0, lines_1 = lines; _i < lines_1.length; _i++) {
        var line = lines_1[_i];
        try {
            last = JSON.parse(line);
        }
        catch (_k) {
            continue;
        }
    }
    if (!last || typeof last !== "object") {
        throw new Error("[repo/cli] no JSON lines in stdout: ".concat(stdout.slice(0, 500)));
    }
    var payload = last;
    if ((_h = payload.errors) === null || _h === void 0 ? void 0 : _h.length) {
        throw new Error("[repo/cli] graphql errors: ".concat(payload.errors.map(function (x) { return x.message; }).join("; ")));
    }
    return (_j = payload.data) !== null && _j !== void 0 ? _j : payload;
}
var __dirname = (0, node_path_1.dirname)((0, node_url_1.fileURLToPath)(import.meta.url));
/** 🧭️Package dir for `@semio-tech/repo-lib` (…/repo/lib/js). */
function getLibRoot() {
    return (0, node_path_1.resolve)(__dirname, "📦️packages");
}
//#endregion 🔖️cli
//#region 🔖️linter
var NODE_QUERY = "\nquery NodeQ($id: ID!) {\n  node(id: $id) {\n    __typename\n    ... on File {\n      id path name extension fileKind: kind\n      sections { id name path range { start end } }\n      definitions { id name kind range { start end } }\n    }\n    ... on Folder {\n      id path name\n    }\n    ... on Bundle {\n      id name root bundleKind: kind\n    }\n    ... on Section {\n      id name path\n      range { start end }\n      sectionFile: file { path }\n      definitions { id name kind range { start end } }\n    }\n    ... on Definition {\n      id name defKind: kind\n      range { start end }\n      defFile: file { path }\n    }\n  }\n}\n";
/** 🧷️BaseLinter holds shared repo-root + graphql helpers for lint scripts. */
var BaseLinter = /** @class */ (function () {
    function BaseLinter(entityId, repoRoot) {
        if (repoRoot === void 0) { repoRoot = (0, ____ts_5.getWorkspaceRoot)(); }
        this.entityId = entityId;
        this.repoRoot = repoRoot;
    }
    BaseLinter.prototype.gql = function (query, variables) {
        if (variables === void 0) { variables = {}; }
        return runCliGraphql(query, variables, { repoRoot: this.repoRoot });
    };
    BaseLinter.prototype.loadNode = function () {
        var data = this.gql(NODE_QUERY, { id: this.entityId });
        var n = data.node;
        if (!n || !n.__typename) {
            throw new Error("[linter] node not found for id ".concat(this.entityId));
        }
        return n;
    };
    /** 🚫️Builds a breach with default scope = entity id. */
    BaseLinter.prototype.breach = function (p) {
        var scope = p.scope, rest = __rest(p, ["scope"]);
        return __assign(__assign({}, rest), { scope: scope !== null && scope !== void 0 ? scope : this.entityId });
    };
    return BaseLinter;
}());
exports.BaseLinter = BaseLinter;
/** 🏗️TechnologyLinter queries a technology node by id. */
var TechnologyLinter = /** @class */ (function (_super) {
    __extends(TechnologyLinter, _super);
    function TechnologyLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    TechnologyLinter.prototype.load = function () {
        var _this = this;
        var _a;
        if (!this.node) {
            var data = this.gql("query T { technologies { id name kind root } }");
            var found = ((_a = data.technologies) !== null && _a !== void 0 ? _a : []).find(function (t) { return String(t.id) === _this.entityId; });
            if (!found) {
                throw new Error("[TechnologyLinter] technology not found for id ".concat(this.entityId));
            }
            this.node = __assign(__assign({}, found), { __typename: "Technology" });
        }
        return this.node;
    };
    TechnologyLinter.prototype.name = function () {
        var _a;
        return String((_a = this.load().name) !== null && _a !== void 0 ? _a : "");
    };
    TechnologyLinter.prototype.kind = function () {
        var _a;
        return String((_a = this.load().kind) !== null && _a !== void 0 ? _a : "");
    };
    TechnologyLinter.prototype.root = function () {
        var _a;
        return String((_a = this.load().root) !== null && _a !== void 0 ? _a : "");
    };
    /** 📦️Lists bundle rows for this technology. */
    TechnologyLinter.prototype.bundles = function () {
        var _a;
        var data = this.gql("query B { bundles { id name root kind } }");
        var tech = this.name();
        return ((_a = data.bundles) !== null && _a !== void 0 ? _a : []).filter(function (b) { var _a; return String((_a = b.name) !== null && _a !== void 0 ? _a : "").split("/")[0] === tech; });
    };
    return TechnologyLinter;
}(BaseLinter));
exports.TechnologyLinter = TechnologyLinter;
/** 📦️BundleLinter queries a bundle node by id. */
var BundleLinter = /** @class */ (function (_super) {
    __extends(BundleLinter, _super);
    function BundleLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    BundleLinter.prototype.load = function () {
        if (!this.node)
            this.node = this.loadNode();
        if (this.node.__typename !== "Bundle") {
            throw new Error("[BundleLinter] expected Bundle, got ".concat(this.node.__typename));
        }
        return this.node;
    };
    BundleLinter.prototype.name = function () {
        var _a;
        return String((_a = this.load().name) !== null && _a !== void 0 ? _a : "");
    };
    BundleLinter.prototype.root = function () {
        var _a;
        return String((_a = this.load().root) !== null && _a !== void 0 ? _a : "");
    };
    BundleLinter.prototype.kind = function () {
        var _a;
        return String((_a = this.load().bundleKind) !== null && _a !== void 0 ? _a : "");
    };
    BundleLinter.prototype.technologyName = function () {
        var _a;
        return (_a = this.name().split("/")[0]) !== null && _a !== void 0 ? _a : "";
    };
    return BundleLinter;
}(BaseLinter));
exports.BundleLinter = BundleLinter;
/** 📁️FolderLinter queries a folder node by id. */
var FolderLinter = /** @class */ (function (_super) {
    __extends(FolderLinter, _super);
    function FolderLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    FolderLinter.prototype.load = function () {
        if (!this.node)
            this.node = this.loadNode();
        if (this.node.__typename !== "Folder") {
            throw new Error("[FolderLinter] expected Folder, got ".concat(this.node.__typename));
        }
        return this.node;
    };
    FolderLinter.prototype.path = function () {
        var _a;
        return String((_a = this.load().path) !== null && _a !== void 0 ? _a : "").replaceAll("\\", "/");
    };
    FolderLinter.prototype.name = function () {
        var _a;
        return String((_a = this.load().name) !== null && _a !== void 0 ? _a : "");
    };
    return FolderLinter;
}(BaseLinter));
exports.FolderLinter = FolderLinter;
/** 📄️FileLinter queries a file node by id and reads bytes from disk. */
var FileLinter = /** @class */ (function (_super) {
    __extends(FileLinter, _super);
    function FileLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    FileLinter.prototype.load = function () {
        if (!this.node)
            this.node = this.loadNode();
        if (this.node.__typename !== "File") {
            throw new Error("[FileLinter] expected File, got ".concat(this.node.__typename));
        }
        return this.node;
    };
    FileLinter.prototype.path = function () {
        var _a;
        return String((_a = this.load().path) !== null && _a !== void 0 ? _a : "").replaceAll("\\", "/");
    };
    FileLinter.prototype.ext = function () {
        var _a;
        return String((_a = this.load().extension) !== null && _a !== void 0 ? _a : "");
    };
    FileLinter.prototype.kind = function () {
        var _a;
        return String((_a = this.load().fileKind) !== null && _a !== void 0 ? _a : "");
    };
    FileLinter.prototype.content = function () {
        var p = this.path();
        return (0, node_fs_1.readFileSync)((0, node_path_1.join)(this.repoRoot, p), "utf8");
    };
    FileLinter.prototype.lines = function () {
        return this.content().split(/\r?\n/);
    };
    FileLinter.prototype.sections = function () {
        var _a;
        return (_a = this.load().sections) !== null && _a !== void 0 ? _a : [];
    };
    FileLinter.prototype.definitions = function () {
        var _a;
        return (_a = this.load().definitions) !== null && _a !== void 0 ? _a : [];
    };
    return FileLinter;
}(BaseLinter));
exports.FileLinter = FileLinter;
/** 🔖️SectionLinter queries a section node by id. */
var SectionLinter = /** @class */ (function (_super) {
    __extends(SectionLinter, _super);
    function SectionLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    SectionLinter.prototype.load = function () {
        if (!this.node)
            this.node = this.loadNode();
        if (this.node.__typename !== "Section") {
            throw new Error("[SectionLinter] expected Section, got ".concat(this.node.__typename));
        }
        return this.node;
    };
    SectionLinter.prototype.filePath = function () {
        var _a;
        var f = this.load().sectionFile;
        return String((_a = f === null || f === void 0 ? void 0 : f.path) !== null && _a !== void 0 ? _a : "").replaceAll("\\", "/");
    };
    SectionLinter.prototype.sectionPath = function () {
        var _a;
        return String((_a = this.load().path) !== null && _a !== void 0 ? _a : "");
    };
    SectionLinter.prototype.startLine = function () {
        var _a;
        var r = this.load().range;
        return Number((_a = r === null || r === void 0 ? void 0 : r.start) !== null && _a !== void 0 ? _a : 0);
    };
    SectionLinter.prototype.endLine = function () {
        var _a;
        var r = this.load().range;
        return Number((_a = r === null || r === void 0 ? void 0 : r.end) !== null && _a !== void 0 ? _a : 0);
    };
    SectionLinter.prototype.content = function () {
        var full = (0, node_fs_1.readFileSync)((0, node_path_1.join)(this.repoRoot, this.filePath()), "utf8");
        var lines = full.split(/\r?\n/);
        var s = this.startLine();
        var e = this.endLine();
        if (s <= 0 || e < s)
            return "";
        return lines.slice(s - 1, e).join("\n");
    };
    SectionLinter.prototype.definitions = function () {
        var _a;
        return (_a = this.load().definitions) !== null && _a !== void 0 ? _a : [];
    };
    return SectionLinter;
}(BaseLinter));
exports.SectionLinter = SectionLinter;
/** 🏷️DefinitionLinter queries a definition node by id. */
var DefinitionLinter = /** @class */ (function (_super) {
    __extends(DefinitionLinter, _super);
    function DefinitionLinter() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    DefinitionLinter.prototype.load = function () {
        if (!this.node)
            this.node = this.loadNode();
        if (this.node.__typename !== "Definition") {
            throw new Error("[DefinitionLinter] expected Definition, got ".concat(this.node.__typename));
        }
        return this.node;
    };
    DefinitionLinter.prototype.filePath = function () {
        var _a;
        var f = this.load().defFile;
        return String((_a = f === null || f === void 0 ? void 0 : f.path) !== null && _a !== void 0 ? _a : "").replaceAll("\\", "/");
    };
    DefinitionLinter.prototype.name = function () {
        var _a;
        return String((_a = this.load().name) !== null && _a !== void 0 ? _a : "");
    };
    DefinitionLinter.prototype.kind = function () {
        var _a;
        return String((_a = this.load().defKind) !== null && _a !== void 0 ? _a : "");
    };
    DefinitionLinter.prototype.startLine = function () {
        var _a;
        var r = this.load().range;
        return Number((_a = r === null || r === void 0 ? void 0 : r.start) !== null && _a !== void 0 ? _a : 0);
    };
    DefinitionLinter.prototype.endLine = function () {
        var _a;
        var r = this.load().range;
        return Number((_a = r === null || r === void 0 ? void 0 : r.end) !== null && _a !== void 0 ? _a : 0);
    };
    DefinitionLinter.prototype.content = function () {
        var full = (0, node_fs_1.readFileSync)((0, node_path_1.join)(this.repoRoot, this.filePath()), "utf8");
        var lines = full.split(/\r?\n/);
        var s = this.startLine();
        var e = this.endLine();
        if (s <= 0 || e < s)
            return "";
        return lines.slice(s - 1, e).join("\n");
    };
    return DefinitionLinter;
}(BaseLinter));
exports.DefinitionLinter = DefinitionLinter;
/** 🔎️Resolves folder path to folder graphql row (for script.ts policy placement). */
function resolveFolderByPath(repoRoot, folderPath) {
    var _a;
    var rel = folderPath.replaceAll("\\", "/").replace(/^\/+/, "");
    var data = runCliGraphql("query F($p: String!) { folder(path: $p) { __typename id path name } }", { p: rel }, { repoRoot: repoRoot });
    if (!((_a = data.folder) === null || _a === void 0 ? void 0 : _a.id))
        throw new Error("[linter] folder not found for path ".concat(rel));
    return data.folder;
}
/** 🔎️Resolves bundle name like `repo/client` to bundle id. */
function resolveBundleByName(repoRoot, name) {
    var _a;
    var data = runCliGraphql("query B($n: String!) { bundle(name: $n) { __typename id name root kind } }", { n: name }, { repoRoot: repoRoot });
    if (!((_a = data.bundle) === null || _a === void 0 ? void 0 : _a.id))
        throw new Error("[linter] bundle not found for name ".concat(name));
    return data.bundle;
}
/** 🔎️Resolves technology folder name (e.g. `repo`) to technology id. */
function resolveTechnologyByName(repoRoot, name) {
    var _a;
    var data = runCliGraphql("query T { technologies { id name root kind } }", {}, { repoRoot: repoRoot });
    var hit = ((_a = data.technologies) !== null && _a !== void 0 ? _a : []).find(function (t) { var _a; return String((_a = t.name) !== null && _a !== void 0 ? _a : "") === name; });
    if (!(hit === null || hit === void 0 ? void 0 : hit.id))
        throw new Error("[linter] technology not found for name ".concat(name));
    return hit;
}
/** 📜️Tags a lint callback for tooling (runner unwraps default export). */
function defineLint(_tag, fn) {
    return fn;
}
//#endregion 🔖️script
//#region 🔖️dependency-boundary
var ADAPTER_MARKERS = [
    "//#region 🔌️adapter",
    "// #region 🔌️adapter",
    "# #region 🔌️adapter",
    "#region 🔌️adapter",
    "//#region 🔌️adapters",
    "// #region 🔌️adapters",
    "# #region 🔌️adapters",
    "//#region 🌐️rswasmtransport",
    "// #region 🌐️rswasmtransport",
    "pub mod adapters",
    "mod adapters ",
];
var INTERNAL_PREFIXES = ["@ui/", "@cad/", "@puzzle/", "@framework/", "@repo/", "@coda/"];
/** 🔌️Returns true when the file path or content marks an adapter boundary. */
function isAdapterBoundaryFile(filePath, content) {
    var n = (0, node_path_1.normalize)(filePath).replaceAll("\\", "/").toLowerCase();
    if (n.includes("/adapters/") || n.includes("/external_adapters"))
        return true;
    if (n.includes("-transport.") || n.endsWith(".🟦️worker.ts") || n.includes("kit-store.worker"))
        return true;
    if (n.includes("adapter"))
        return true;
    var lower = content.toLowerCase();
    return ADAPTER_MARKERS.some(function (m) { return lower.includes(m); });
}
/** 🔌️Skips generated, test, and non-source paths for dependency-boundary lint. */
function shouldSkipDependencyBoundaryFile(filePath) {
    var _a;
    var n = (0, node_path_1.normalize)(filePath).replaceAll("\\", "/");
    if (n.includes("/node_modules/") || n.includes("/.🧬semio/") || n.includes("/dist/") || n.includes("/target/")) {
        return true;
    }
    var base = (_a = n.split("/").pop()) !== null && _a !== void 0 ? _a : n;
    if (base.endsWith(".gen.ts"))
        return true;
    if (base.includes(".test.") || base.endsWith(".spec.ts"))
        return true;
    return /\.(json|md|ya?ml|lock|svg|png|jpe?g|woff2?)$/i.test(base);
}
function isInternalPackage(name, version) {
    if (INTERNAL_PREFIXES.some(function (p) { return name.startsWith(p); }))
        return true;
    return version.startsWith("workspace:") || version.startsWith("file:") || version.startsWith("link:");
}
/** 🔌️Loads third-party package names from the nearest manifest walking up from filePath. */
function loadThirdPartyDeps(repoRoot, filePath) {
    var deps = new Set();
    var dir = (0, node_path_1.normalize)((0, node_path_1.dirname)((0, node_path_1.join)(repoRoot, filePath))).replaceAll("\\", "/");
    var root = (0, node_path_1.normalize)(repoRoot).replaceAll("\\", "/");
    while (dir.startsWith(root) || dir === root) {
        var pkg = (0, node_path_1.join)(dir, "package.json");
        if ((0, node_fs_1.existsSync)(pkg)) {
            mergePackageJson(pkg, deps);
            break;
        }
        var parent_1 = (0, node_path_1.dirname)(dir);
        if (parent_1 === dir)
            break;
        dir = parent_1;
    }
    return deps;
}
function mergePackageJson(path, deps) {
    var raw = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
    for (var _i = 0, _a = ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"]; _i < _a.length; _i++) {
        var key = _a[_i];
        var block = raw[key];
        if (!block)
            continue;
        for (var _b = 0, _c = Object.entries(block); _b < _c.length; _b++) {
            var _d = _c[_b], name_1 = _d[0], ver = _d[1];
            if (isInternalPackage(name_1, ver))
                continue;
            deps.add(name_1);
        }
    }
}
/** 🔌️Parses import specifiers from a single import line (TS/JS). */
function parseTsImportSpecs(line) {
    var specs = [];
    var re = /(?:from|import)\s+['"]([^'"]+)['"]/g;
    var m;
    while ((m = re.exec(line)) !== null) {
        specs.push(m[1]);
    }
    return specs;
}
function isThirdPartySpec(spec, deps) {
    if (!spec || spec.startsWith(".") || spec.startsWith("/"))
        return false;
    if (deps.has(spec))
        return true;
    if (spec.startsWith("@")) {
        var parts = spec.split("/");
        if (parts.length >= 2 && deps.has("".concat(parts[0], "/").concat(parts[1])))
            return true;
    }
    for (var _i = 0, deps_1 = deps; _i < deps_1.length; _i++) {
        var dep = deps_1[_i];
        if (spec === dep || spec.startsWith("".concat(dep, "/")))
            return true;
    }
    return false;
}
/** 🔌️Builds breach records for direct third-party imports outside adapter boundaries. */
function dependencyBoundaryBreachesForFile(repoRoot, filePath, content, scope) {
    if (shouldSkipDependencyBoundaryFile(filePath))
        return [];
    if (isAdapterBoundaryFile(filePath, content))
        return [];
    var deps = loadThirdPartyDeps(repoRoot, filePath);
    if (deps.size === 0)
        return [];
    var breachs = [];
    var lines = content.split(/\r?\n/);
    for (var i = 0; i < lines.length; i++) {
        var line = lines[i];
        if (!line.includes("import "))
            continue;
        for (var _i = 0, _a = parseTsImportSpecs(line); _i < _a.length; _i++) {
            var spec = _a[_i];
            if (!isThirdPartySpec(spec, deps))
                continue;
            breachs.push({
                id: "dep-boundary-".concat(spec, "-").concat(i + 1),
                summary: "Direct import of third-party \"".concat(spec, "\" must live in an adapter module"),
                kind: "dependency-boundary/import/direct-third-party",
                priority: "high",
                reason: "Third-party packages must only be imported in adapter regions or adapter paths",
                solution: "Move the import into a //#region 🔌️Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
                scope: scope,
            });
        }
    }
    return breachs;
}
/** 🔌️Aggregates dependency-boundary breaches for every TS/TSX file under a bundle root (repo-relative). */
function dependencyBoundaryBreachesForBundleDir(repoRoot, bundleRootRel) {
    var rootAbs = (0, node_path_1.join)(repoRoot, bundleRootRel);
    if (!(0, node_fs_1.existsSync)(rootAbs))
        return [];
    var breachs = [];
    var walk = function (dir) {
        for (var _i = 0, _a = (0, node_fs_1.readdirSync)(dir, { withFileTypes: true }); _i < _a.length; _i++) {
            var ent = _a[_i];
            if (ent.name === "node_modules" || ent.name === "dist" || ent.name === ".🧬semio")
                continue;
            var abs = (0, node_path_1.join)(dir, ent.name);
            if (ent.isDirectory()) {
                walk(abs);
                continue;
            }
            if (!/\.tsx?$/i.test(ent.name))
                continue;
            var rel = (0, node_path_1.relative)(repoRoot, abs).replaceAll("\\", "/");
            breachs.push.apply(breachs, dependencyBoundaryBreachesForFile(repoRoot, rel, (0, node_fs_1.readFileSync)(abs, "utf8"), rel));
        }
    };
    walk(rootAbs);
    return breachs;
}
/** 🏛️ Repo-relative path of the committed layering baseline. */
exports.LAYERING_BASELINE_REL_PATH = "🧅️layering.json";
var LAYERING_SCANNED_EXTENSIONS = new Set([".ts", ".tsx", ".mts", ".cts", ".js", ".mjs", ".cjs", ".rs", ".go", ".py", ".cs", ".json", ".toml", ".yml", ".yaml"]);
var LAYERING_SKIPPED_DIRS = new Set(["node_modules", ".git", ".nx", ".venv", "target", "dist", "build", "out", "__pycache__", "obj", "bin", "storybook-static", "🎫️tickets", "⚡️cache", "🤖️generated"]);
function layeringTaxonomy() {
    var taxonomy = (0, ____ts_4.loadTaxonomy)();
    var rootContracts = function (contractIds, key) {
        return contractIds.map(function (id) {
            var contract = taxonomy.fixedFilenameContracts[id];
            if (!contract)
                throw new Error("".concat(key, " references missing fixed contract ").concat(JSON.stringify(id), "."));
            return (0, ____ts_4.fixedContractFilename)(contract);
        });
    };
    return {
        layers: __assign({}, taxonomy.areaLayers),
        repoWide: rootContracts(taxonomy.repoWideContractIds, "repoWideContractIds"),
        generated: rootContracts(taxonomy.layeringGeneratedContractIds, "layeringGeneratedContractIds"),
        banners: __spreadArray([], taxonomy.layeringGeneratedBanners, true),
    };
}
/**
 * 🏛️ Finds every reference from repo-wide or framework code to an implementation area.
 *
 * The rule this enforces is concrete: deleting an implementation area must leave every repo-wide and
 * framework file correct. A rule that only makes sense for one implementation belongs in that
 * implementation's own `📜️script.ts`, which the policy plugin already discovers by convention.
 *
 * Which areas are implementations is taxonomy data (`areaLayers`), so this function names none of
 * them itself and a new area needs no code change here.
 */
function layeringReferences(repoRoot) {
    var _a = layeringTaxonomy(), layers = _a.layers, repoWide = _a.repoWide, generated = _a.generated, banners = _a.banners;
    var isGenerated = function (relPath) { return generated.includes(relPath); };
    var implementations = Object.entries(layers)
        .filter(function (_a) {
        var layer = _a[1];
        return layer === "implementation";
    })
        .map(function (_a) {
        var area = _a[0];
        return area;
    });
    var frameworkRoots = Object.entries(layers)
        .filter(function (_a) {
        var layer = _a[1];
        return layer === "framework";
    })
        .map(function (_a) {
        var area = _a[0];
        return area;
    });
    if (implementations.length === 0)
        return [];
    var found = [];
    var scan = function (relPath) {
        if (isGenerated(relPath))
            return;
        if (!LAYERING_SCANNED_EXTENSIONS.has(relPath.slice(relPath.lastIndexOf("."))))
            return;
        var content;
        try {
            content = (0, node_fs_1.readFileSync)((0, node_path_1.join)(repoRoot, relPath), "utf8");
        }
        catch (_a) {
            return;
        }
        // 🏛️A bundled or generated file re-derives itself; only AUTHORED coupling can go stale.
        var head = content.slice(0, 512);
        if (banners.some(function (banner) { return head.includes(banner); }))
            return;
        for (var _i = 0, implementations_1 = implementations; _i < implementations_1.length; _i++) {
            var area = implementations_1[_i];
            var count = content.split("".concat(area, "/")).length - 1;
            if (count > 0)
                found.push({ file: relPath, area: area, count: count });
        }
    };
    for (var _i = 0, repoWide_1 = repoWide; _i < repoWide_1.length; _i++) {
        var file = repoWide_1[_i];
        if ((0, node_fs_1.existsSync)((0, node_path_1.join)(repoRoot, file)))
            scan(file);
    }
    var walk = function (relDir) {
        var entries;
        try {
            entries = (0, node_fs_1.readdirSync)((0, node_path_1.join)(repoRoot, relDir), { withFileTypes: true });
        }
        catch (_a) {
            return;
        }
        for (var _i = 0, entries_1 = entries; _i < entries_1.length; _i++) {
            var entry = entries_1[_i];
            var childRel = "".concat(relDir, "/").concat(entry.name);
            if (entry.isDirectory()) {
                if (!LAYERING_SKIPPED_DIRS.has(entry.name))
                    walk(childRel);
                continue;
            }
            scan(childRel);
        }
    };
    for (var _b = 0, frameworkRoots_1 = frameworkRoots; _b < frameworkRoots_1.length; _b++) {
        var root = frameworkRoots_1[_b];
        walk(root);
    }
    return found.sort(function (a, b) { return b.count - a.count || a.file.localeCompare(b.file) || a.area.localeCompare(b.area); });
}
/** 🏛️ Loads the committed layering baseline, or an empty one on first run. */
function loadLayeringBaseline(repoRoot) {
    var path = (0, node_path_1.join)(repoRoot, exports.LAYERING_BASELINE_REL_PATH);
    if (!(0, node_fs_1.existsSync)(path))
        return { schemaVersion: 1, allowed: {} };
    return JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
}
/** 🏛️ Total references per file, the unit the ratchet compares. */
function layeringCounts(references) {
    var _a;
    var counts = {};
    for (var _i = 0, references_1 = references; _i < references_1.length; _i++) {
        var reference = references_1[_i];
        counts[reference.file] = ((_a = counts[reference.file]) !== null && _a !== void 0 ? _a : 0) + reference.count;
    }
    return counts;
}
/** 🏛️ The shrink-only verdict: a file may reference fewer implementation paths than its baseline, never more. */
function layeringBreaches(repoRoot) {
    var _a;
    var baseline = loadLayeringBaseline(repoRoot);
    var counts = layeringCounts(layeringReferences(repoRoot));
    var breaches = [];
    for (var _i = 0, _b = Object.entries(counts).sort(function (a, b) { return b[1] - a[1]; }); _i < _b.length; _i++) {
        var _c = _b[_i], file = _c[0], count = _c[1];
        var allowed = (_a = baseline.allowed[file]) !== null && _a !== void 0 ? _a : 0;
        if (count <= allowed)
            continue;
        breaches.push({
            id: "implementation-reference",
            kind: "layering/implementation-reference",
            scope: file,
            summary: "".concat(count, " reference(s) to an implementation area, baseline allows ").concat(allowed),
            priority: "high",
            reason: "Repo-wide and framework code must stay correct when an implementation area is deleted. A rule that only makes sense for one implementation belongs to that implementation.",
            solution: "Move the implementation-specific logic into that implementation's own 📜️script.ts (the policy plugin discovers it), or express the dependency as taxonomy vocabulary instead of a literal path.",
        });
    }
    return breaches;
}
/** 🏛️ Rewrites the baseline from the current tree — the deliberate ratchet step after a migration. */
function writeLayeringBaseline(repoRoot) {
    var counts = layeringCounts(layeringReferences(repoRoot));
    var baseline = { schemaVersion: 1, allowed: Object.fromEntries(Object.entries(counts).sort(function (_a, _b) {
            var a = _a[0];
            var b = _b[0];
            return a.localeCompare(b);
        })) };
    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(repoRoot, exports.LAYERING_BASELINE_REL_PATH), "".concat(JSON.stringify(__assign({ _comment: "🏛️ Shrink-only layering ratchet. Each entry is how many references to an implementation area (`areaLayers` in 🔣️taxonomy.json) one repo-wide or framework file is still allowed. A count may fall, never rise; a file that reaches zero should be removed from this list. Regenerate deliberately with `bun ./📜️script.ts verify layering write-baseline` AFTER a migration, never to make a failure go away." }, baseline), null, 2), "\n"));
    return baseline;
}
/** 🔒️ Filename an area uses to contribute allowlist entries to repository policy rules. */
exports.POLICY_ALLOWLIST_FILENAME = "🔒️policy-allowlist.json";
/**
 * 🔒️ Merges one policy rule's allowlist from every area that contributes to it. A rule therefore
 * never names a path it exempts: the exemption lives with the code it describes, and deleting that
 * code deletes the exemption. Discovery is by convention, so a new area needs no code change.
 */
function policyDiscoveredAllowlist(repoRoot, key) {
    var merged = new Set();
    var walk = function (relDir) {
        var _a;
        var entries;
        try {
            entries = (0, node_fs_1.readdirSync)((0, node_path_1.join)(repoRoot, relDir || "."), { withFileTypes: true });
        }
        catch (_b) {
            return;
        }
        for (var _i = 0, entries_2 = entries; _i < entries_2.length; _i++) {
            var entry = entries_2[_i];
            var childRel = relDir ? "".concat(relDir, "/").concat(entry.name) : entry.name;
            if (entry.isDirectory()) {
                if (!LAYERING_SKIPPED_DIRS.has(entry.name))
                    walk(childRel);
                continue;
            }
            if (entry.name !== exports.POLICY_ALLOWLIST_FILENAME)
                continue;
            try {
                var parsed = JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.join)(repoRoot, childRel), "utf8"));
                for (var _c = 0, _d = (_a = parsed[key]) !== null && _a !== void 0 ? _a : []; _c < _d.length; _c++) {
                    var value = _d[_c];
                    merged.add(value);
                }
            }
            catch (_e) {
                /* an unreadable allowlist contributes nothing, which fails closed */
            }
        }
    };
    walk("");
    return merged;
}
/** 🔎️True when `script.ts` exports a repo policy lint callback. */
function scriptExportsPolicy(scriptPath) {
    var text = (0, node_fs_1.readFileSync)(scriptPath, "utf8");
    return /\bexport\s+(?:(?:const|function)\s+policy\b|\{\s*policy\s*\}\s+from\b)/.test(text);
}
function parsePolicyFileExport(scriptPath) {
    var text = (0, node_fs_1.readFileSync)(scriptPath, "utf8");
    var m = text.match(/export\s+const\s+policyFile\s*=\s*["']([^"']+)["']/);
    return m === null || m === void 0 ? void 0 : m[1];
}
function fileEntityId(repoRoot, fileRel) {
    var _a;
    var rel = (0, node_path_1.relative)(repoRoot, fileRel).replaceAll("\\", "/");
    var data = runCliGraphql("query F($p: String!) { file(path: $p) { id } }", { p: rel }, { repoRoot: repoRoot });
    if (!((_a = data.file) === null || _a === void 0 ? void 0 : _a.id))
        throw new Error("[policy-runner] file id not found for ".concat(rel));
    return data.file.id;
}
function norm(p) {
    return p.replaceAll("\\", "/").replace(/\/+$/, "");
}
/** 🔎️Maps `script.ts` directory to bundle, technology, or folder entity id. */
function resolvePolicyScriptEntity(repoRoot, scriptPath) {
    var _a, _b, _c, _d, _e;
    var dir = (0, node_path_1.dirname)(scriptPath);
    var relDir = norm((0, node_path_1.relative)(repoRoot, dir));
    var folder = runCliGraphql("query Fo($p: String!) { folder(path: $p) { id path } }", { p: relDir }, { repoRoot: repoRoot });
    // 🌱️ Workspace root: `folder.id` is `""` (falsy but valid — relative(repoRoot, repoRoot) === "").
    if (((_a = folder.folder) === null || _a === void 0 ? void 0 : _a.id) === undefined)
        throw new Error("[policy-runner] folder not resolved for ".concat(relDir));
    var meta = runCliGraphql("query M { bundles { id root name } technologies { id root name } }", {}, { repoRoot: repoRoot });
    var d = norm(relDir);
    for (var _i = 0, _f = (_b = meta.bundles) !== null && _b !== void 0 ? _b : []; _i < _f.length; _i++) {
        var b = _f[_i];
        if (norm(String((_c = b.root) !== null && _c !== void 0 ? _c : "")) === d) {
            return { kind: "bundle", id: String(b.id) };
        }
    }
    for (var _g = 0, _h = (_d = meta.technologies) !== null && _d !== void 0 ? _d : []; _g < _h.length; _g++) {
        var t = _h[_g];
        // 🌱️ Technology `root` echoes the absolute `--repo` path (unlike bundle `root`, which is repo-relative) —
        // the workspace-root script.ts's relDir is "", so match against the absolute repoRoot too.
        var tRoot = norm(String((_e = t.root) !== null && _e !== void 0 ? _e : ""));
        if (tRoot === d || tRoot === norm(repoRoot)) {
            return { kind: "technology", id: String(t.id) };
        }
    }
    return { kind: "folder", id: String(folder.folder.id) };
}
function runPolicyScript(scriptPath_1) {
    return __awaiter(this, arguments, void 0, function (scriptPath, repoRoot) {
        var absScript, base, policyFile, entity, target, href, mod, fn, breachs, _a, _b, mkdirSync, writeFileSync, sanitizeCacheKey, cacheDir, cacheName, cachePath, payload;
        if (repoRoot === void 0) { repoRoot = (0, ____ts_5.getWorkspaceRoot)(); }
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0:
                    absScript = scriptPath.includes(":") || scriptPath.startsWith("/") || /^[A-Za-z]:\\/.test(scriptPath) ? scriptPath : (0, node_path_1.join)(repoRoot, scriptPath);
                    base = (0, node_path_1.basename)(absScript);
                    if (base !== "📜️script.ts") {
                        throw new Error("[policy-runner] expected \uD83D\uDCDC\uFE0Fscript.ts, got ".concat(base));
                    }
                    policyFile = parsePolicyFileExport(absScript);
                    if (policyFile) {
                        target = (0, node_path_1.join)((0, node_path_1.dirname)(absScript), policyFile).replaceAll("\\", "/");
                        entity = { kind: "file", id: fileEntityId(repoRoot, target), path: target };
                    }
                    else {
                        entity = resolvePolicyScriptEntity(repoRoot, absScript);
                    }
                    href = (0, node_url_1.pathToFileURL)(absScript).href;
                    return [4 /*yield*/, Promise.resolve("".concat(href)).then(function (s) { return require(s); })];
                case 1:
                    mod = (_c.sent());
                    fn = mod.policy;
                    if (typeof fn !== "function") {
                        throw new Error("[policy-runner] ".concat(absScript, " must export const policy = defineLint(...)"));
                    }
                    _a = entity.kind;
                    switch (_a) {
                        case "file": return [3 /*break*/, 2];
                        case "folder": return [3 /*break*/, 4];
                        case "bundle": return [3 /*break*/, 6];
                        case "technology": return [3 /*break*/, 8];
                    }
                    return [3 /*break*/, 10];
                case 2: return [4 /*yield*/, fn(new FileLinter(entity.id, repoRoot))];
                case 3:
                    breachs = _c.sent();
                    return [3 /*break*/, 11];
                case 4: return [4 /*yield*/, fn(new FolderLinter(entity.id, repoRoot))];
                case 5:
                    breachs = _c.sent();
                    return [3 /*break*/, 11];
                case 6: return [4 /*yield*/, fn(new BundleLinter(entity.id, repoRoot))];
                case 7:
                    breachs = _c.sent();
                    return [3 /*break*/, 11];
                case 8: return [4 /*yield*/, fn(new TechnologyLinter(entity.id, repoRoot))];
                case 9:
                    breachs = _c.sent();
                    return [3 /*break*/, 11];
                case 10: throw new Error("[policy-runner] unreachable");
                case 11: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                case 12:
                    _b = _c.sent(), mkdirSync = _b.mkdirSync, writeFileSync = _b.writeFileSync;
                    sanitizeCacheKey = function (id) { return id.replace(/[^\w.-]+/g, "_").slice(0, 200); };
                    cacheDir = (0, ____ts_3.repoCacheDirectory)(repoRoot, "breaches");
                    mkdirSync(cacheDir, { recursive: true });
                    cacheName = "".concat(sanitizeCacheKey(entity.id), ".json");
                    cachePath = (0, node_path_1.join)(cacheDir, cacheName);
                    payload = {
                        entityId: entity.id,
                        script: (0, node_path_1.relative)(repoRoot, absScript).replaceAll("\\", "/"),
                        breachs: breachs,
                    };
                    writeFileSync(cachePath, JSON.stringify(payload, null, 2), "utf8");
                    return [2 /*return*/, { entityId: entity.id, breachs: breachs, cachePath: cachePath }];
            }
        });
    });
}
/** 🧾️Renders a breach set as a per-kind tally followed by one line per breach. */
function formatBreachReport(breachs, cachePath) {
    var _a;
    var tally = new Map();
    for (var _i = 0, breachs_1 = breachs; _i < breachs_1.length; _i++) {
        var b = breachs_1[_i];
        tally.set(b.kind, ((_a = tally.get(b.kind)) !== null && _a !== void 0 ? _a : 0) + 1);
    }
    var lines = ["".concat(breachs.length, " high-priority breach(es) across ").concat(tally.size, " rule(s):")];
    for (var _b = 0, _c = __spreadArray([], tally, true).sort(function (a, b) { return b[1] - a[1]; }); _b < _c.length; _b++) {
        var _d = _c[_b], kind = _d[0], count = _d[1];
        lines.push("  ".concat(String(count).padStart(5), "  ").concat(kind));
    }
    lines.push("");
    for (var _e = 0, breachs_2 = breachs; _e < breachs_2.length; _e++) {
        var b = breachs_2[_e];
        lines.push("  ".concat(b.kind, "  ").concat(b.scope).concat(b.line ? ":".concat(b.line) : "", "  ").concat(b.summary));
    }
    lines.push("", "full breach set (including non-blocking priorities): ".concat(cachePath));
    return lines.join("\n");
}
/**
 * 🚪️Runs `policy` on this `script.ts`, reports every high-priority breach, and exits 1 when any exists.
 * The report is mandatory: a gate that exits non-zero without naming what it rejected is unactionable.
 */
function runPolicyExit(scriptPath) {
    return __awaiter(this, void 0, void 0, function () {
        var _a, breachs, cachePath, high;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0: return [4 /*yield*/, runPolicyScript(scriptPath)];
                case 1:
                    _a = _b.sent(), breachs = _a.breachs, cachePath = _a.cachePath;
                    high = breachs.filter(function (b) { return b.priority === "high"; });
                    if (high.length === 0) {
                        console.log("policy: no high-priority breaches (".concat(breachs.length, " total recorded at ").concat(cachePath, ")"));
                        return [2 /*return*/];
                    }
                    console.error(formatBreachReport(high, cachePath));
                    process.exit(1);
                    return [2 /*return*/];
            }
        });
    });
}
//#endregion 🔖️runner
//#region 🔖️policy-cli
/** 🚪️When argv contains `policy`, runs this bundle's policy lint and exits. */
function dispatchPolicyArgv(segments, scriptUrl) {
    return __awaiter(this, void 0, void 0, function () {
        var _this = this;
        return __generator(this, function (_a) {
            if (segments[0] !== "policy")
                return [2 /*return*/, false];
            setTimeout(function () { return __awaiter(_this, void 0, void 0, function () {
                var e_1;
                return __generator(this, function (_a) {
                    switch (_a.label) {
                        case 0:
                            _a.trys.push([0, 2, , 3]);
                            return [4 /*yield*/, runPolicyExit((0, node_url_1.fileURLToPath)(scriptUrl))];
                        case 1:
                            _a.sent();
                            return [3 /*break*/, 3];
                        case 2:
                            e_1 = _a.sent();
                            console.error(e_1);
                            process.exit(1);
                            return [3 /*break*/, 3];
                        case 3: return [2 /*return*/];
                    }
                });
            }); }, 0);
            return [2 /*return*/, true];
        });
    });
}
//#endregion 🔖️policy-cli
//#region 🔖️bundle-script
//#region 🔖️Script
var ____ts_7 = require("./\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDED\uFE0Frouting/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "Script", { enumerable: true, get: function () { return ____ts_7.Script; } });
Object.defineProperty(exports, "BundleScript", { enumerable: true, get: function () { return ____ts_7.BundleScript; } });
Object.defineProperty(exports, "ScriptRouter", { enumerable: true, get: function () { return ____ts_7.ScriptRouter; } });
Object.defineProperty(exports, "findRepoRoot", { enumerable: true, get: function () { return ____ts_7.findRepoRoot; } });
/** 🚪️Policy-only bundle entry when no other subcommands are registered. */
function runPolicyOnlyMain(scriptUrl) {
    return __awaiter(this, void 0, void 0, function () {
        var segments;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    segments = process.argv.slice(2);
                    return [4 /*yield*/, dispatchPolicyArgv(segments, scriptUrl)];
                case 1:
                    if (_a.sent())
                        return [2 /*return*/];
                    console.error("usage: bun ./📜️script.ts policy");
                    process.exit(1);
                    return [2 /*return*/];
            }
        });
    });
}
/**
 * 🚪️Bundle `script.ts` entry: handles optional `policy`, then routes remaining argv through `router`.
 * Export `policy` / `policyFile` from the same file when policy lint applies.
 */
function runBundleScriptMain(router_1, scriptUrl_1) {
    return __awaiter(this, arguments, void 0, function (router, scriptUrl, opts) {
        var segments;
        if (opts === void 0) { opts = {}; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    segments = process.argv.slice(2);
                    return [4 /*yield*/, dispatchPolicyArgv(segments, scriptUrl)];
                case 1:
                    if (_a.sent())
                        return [2 /*return*/];
                    if (opts.defaultCommand && segments.length === 0) {
                        segments = [opts.defaultCommand];
                    }
                    if (!router.hasCommands()) {
                        console.error("usage: ".concat(router.usage()));
                        process.exit(1);
                    }
                    return [4 /*yield*/, router.run(segments)];
                case 2:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
/** 🚪️Workspace root `script.ts` entry (no policy dispatch). */
function runWorkspaceScriptMain(router) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, router.run(process.argv.slice(2))];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
/**
 * 🧭️Nested subcommand dispatch inside a `Script.run` implementation.
 * `handlers` keys are the first argv segment; `defaultKey` runs when argv is empty.
 */
function dispatchSubcommand(segments, handlers, usage, defaultKey) {
    var _a;
    var key = (_a = segments[0]) !== null && _a !== void 0 ? _a : defaultKey;
    var handler = key ? handlers[key] : undefined;
    if (!handler) {
        console.error("usage: ".concat(usage));
        process.exit(1);
    }
    return handler(segments.slice(1));
}
//#endregion 🔖️Router
//#region ⏱️Budget
/** ⏱️Ordered test levels — every test belongs to exactly one; running level L runs all levels ≤ L, budgeted at L's limit. */
exports.TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"];
/** ⏱️Hard wall-clock budget (ms) per test level. */
exports.TEST_LEVEL_BUDGET_MS = {
    fundamental: 15000,
    quick: 30000,
    long: 300000,
    exhaustive: 900000,
};
/** ⏱️Deprecated alias for the fundamental-level budget; kept for straggling call sites during the leveled-test migration. */
exports.DEFAULT_TEST_BUDGET_MS = exports.TEST_LEVEL_BUDGET_MS.fundamental;
function isTestLevel(value) {
    return !!value && exports.TEST_LEVELS.includes(value);
}
/** ⏱️Reads the active test level (`SEMIO_TEST_LEVEL`, defaulting to `fundamental`) — set by [[resolveTestLevel]]. */
function activeTestLevel() {
    return isTestLevel(process.env.SEMIO_TEST_LEVEL) ? process.env.SEMIO_TEST_LEVEL : "fundamental";
}
/**
 * 🎚️Resolves the test level from `segments[0]` (if it names a level) or `SEMIO_TEST_LEVEL`, else `fundamental`.
 * `minimum` is the floor a suite declares when its own fixed cost (independent oracles, generated-bundle
 * renders, taxonomy loads) already exceeds a lower level's budget, so the suite is levelled honestly
 * instead of being killed at every invocation. Sets `process.env.SEMIO_TEST_LEVEL` so every child process
 * spawned afterwards (vitest, cargo, go, pytest, dotnet) inherits it without explicit plumbing.
 * Returns the remaining segments.
 */
function resolveTestLevel(segments, minimum) {
    if (minimum === void 0) { minimum = "fundamental"; }
    var first = segments[0], restIfLevel = segments.slice(1);
    var requested = isTestLevel(first) ? first : activeTestLevel();
    var level = testLevelRank(requested) >= testLevelRank(minimum) ? requested : minimum;
    process.env.SEMIO_TEST_LEVEL = level;
    if (level === "exhaustive" && process.env.SEMIO_COVERAGE === undefined)
        process.env.SEMIO_COVERAGE = "1";
    return { level: level, rest: isTestLevel(first) ? restIfLevel : segments };
}
/** 🎚️Numeric rank of a test level (0=fundamental..3=exhaustive), for `if (testLevelRank() >= testLevelRank("long"))`-style gating in test files. */
function testLevelRank(level) {
    if (level === void 0) { level = process.env.SEMIO_TEST_LEVEL; }
    var idx = exports.TEST_LEVELS.indexOf((isTestLevel(level) ? level : "fundamental"));
    return idx === -1 ? 0 : idx;
}
/** 🎚️True when the active level reaches `level` — the predicate behind level-gated test cases and level-gated `includeSource` entries. */
function testLevelAtLeast(level) {
    return testLevelRank() >= testLevelRank(level);
}
/**
 * 🎚️Level-gates one Vitest case factory: `atTestLevel(it, "long")` runs the case from `long` upwards and
 * reports it as skipped below that, so a case that outgrows its level's wall-clock budget moves level
 * instead of being deleted or silently killed. Structurally typed on `runIf` so this library never
 * depends on Vitest's own types.
 */
function atTestLevel(factory, level) {
    return factory.runIf(testLevelAtLeast(level));
}
function levelsAbove(level) {
    return exports.TEST_LEVELS.slice(exports.TEST_LEVELS.indexOf(level) + 1);
}
/** ⏱️Wall-clock budget (ms) for the given test level — `SEMIO_TEST_BUDGET_MS` override, else [[TEST_LEVEL_BUDGET_MS]]. */
function testLevelBudgetMs(level) {
    var _a;
    if (level === void 0) { level = activeTestLevel(); }
    return Number((_a = process.env.SEMIO_TEST_BUDGET_MS) !== null && _a !== void 0 ? _a : exports.TEST_LEVEL_BUDGET_MS[level]);
}
/** ⏱️Wall-clock budget (seconds, rounded up) for the given test level — for toolchains that take second-granularity timeouts. */
function testLevelBudgetSeconds(level) {
    if (level === void 0) { level = activeTestLevel(); }
    return Math.ceil(testLevelBudgetMs(level) / 1000);
}
/** ⏱️Cumulative `go test` args for the active level: per-level `-timeout`, keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
function goLevelTestArgs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    var args = ["-timeout", "".concat(testLevelBudgetSeconds(level), "s")];
    if (exports.TEST_LEVELS.indexOf(level) <= exports.TEST_LEVELS.indexOf("quick"))
        args.push("-short");
    var skipped = levelsAbove(level).map(function (l) { return l[0].toUpperCase() + l.slice(1); });
    if (skipped.length)
        args.push("-skip", "^Test(".concat(skipped.join("|"), ")"));
    return args;
}
/** 🔣️Resolves Go compiler-input layout names from the repository taxonomy. */
function canonicalGoLayout() {
    var taxonomy = (0, ____ts_4.loadCatalogTaxonomy)();
    var goKind = taxonomy.testAdapterFileKinds["🐹️go"];
    var sourceDomains = taxonomy.semanticDirectoryMemberKinds["members-of-members-of-members-of-modules"];
    if (!taxonomy.testsDirName || !taxonomy.testFixturesDirName || !taxonomy.packagesDirName || !goKind || !sourceDomains)
        throw new Error("The taxonomy must define Go inputs, tests, packages and fixture ownership.");
    return { testsDirectory: taxonomy.testsDirName, implementationFilename: (0, ____ts_4.canonicalFilenameForKind)(goKind, taxonomy), opaqueDirectoryNames: [taxonomy.testFixturesDirName, taxonomy.packagesDirName], sourceDirectoryNames: sourceDomains.memberNames };
}
function canonicalGoModulePlan(moduleRoot, layout) {
    var root = (0, node_fs_1.realpathSync)(moduleRoot);
    var sourceReplacements = {};
    var testReplacements = {};
    var packages = new Set();
    var map = function (owner, source, test) {
        var ownerRelative = (0, node_path_1.relative)(root, owner).split(node_path_1.sep).join("/");
        if (ownerRelative === ".." || ownerRelative.startsWith("../"))
            throw new Error("Go input owner escapes its module: ".concat(owner));
        var id = (0, node_crypto_1.createHash)("sha256").update((0, node_path_1.relative)(root, source).split(node_path_1.sep).join("/")).digest("hex").slice(0, 16);
        var virtual = (0, node_path_1.join)(owner, "zz_semio_".concat(id).concat(test ? "_test" : "", ".go"));
        if ((0, node_fs_1.existsSync)(virtual))
            throw new Error("Go input overlay collides with an authored file: ".concat(virtual));
        (test ? testReplacements : sourceReplacements)[virtual] = source;
        packages.add(ownerRelative ? "./".concat(ownerRelative) : ".");
    };
    var walk = function (directory) {
        if (directory !== root && (0, node_fs_1.existsSync)((0, node_path_1.join)(directory, "go.mod")))
            return;
        var entries = (0, node_fs_1.readdirSync)(directory, { withFileTypes: true }).sort(function (left, right) { return left.name.localeCompare(right.name); });
        if (entries.some(function (entry) { return entry.isFile() && !entry.isSymbolicLink() && entry.name.endsWith(".go"); })) {
            var ownerRelative = (0, node_path_1.relative)(root, directory).split(node_path_1.sep).join("/");
            packages.add(ownerRelative ? "./".concat(ownerRelative) : ".");
        }
        for (var _i = 0, entries_3 = entries; _i < entries_3.length; _i++) {
            var entry = entries_3[_i];
            if (!entry.isDirectory() || entry.isSymbolicLink())
                continue;
            var path = (0, node_path_1.join)(directory, entry.name);
            if (layout.sourceDirectoryNames.includes(entry.name)) {
                var source = (0, node_path_1.join)(path, layout.implementationFilename);
                if ((0, node_fs_1.existsSync)(source)) {
                    var info = (0, node_fs_1.lstatSync)(source);
                    if (info.isFile() && !info.isSymbolicLink())
                        map(directory, source, false);
                }
                continue;
            }
            if (entry.name === layout.testsDirectory) {
                for (var _a = 0, _b = (0, node_fs_1.readdirSync)(path, { withFileTypes: true }).sort(function (left, right) { return left.name.localeCompare(right.name); }); _a < _b.length; _a++) {
                    var testCase = _b[_a];
                    if (!testCase.isDirectory() || testCase.isSymbolicLink())
                        continue;
                    var source = (0, node_path_1.join)(path, testCase.name, layout.implementationFilename);
                    if (!(0, node_fs_1.existsSync)(source))
                        continue;
                    var info = (0, node_fs_1.lstatSync)(source);
                    if (!info.isFile() || info.isSymbolicLink())
                        continue;
                    map(directory, source, true);
                }
                continue;
            }
            if (entry.name === ".git" || entry.name === "node_modules" || entry.name === "target" || entry.name === "vendor" || layout.opaqueDirectoryNames.includes(entry.name))
                continue;
            walk(path);
        }
    };
    walk(root);
    var sorted = function (values) { return Object.fromEntries(Object.entries(values).sort(function (_a, _b) {
        var left = _a[0];
        var right = _b[0];
        return left.localeCompare(right);
    })); };
    var sources = sorted(sourceReplacements), tests = sorted(testReplacements);
    return { packages: __spreadArray([], packages, true).sort(), replacements: sorted(__assign(__assign({}, sources), tests)), sourceReplacements: sources, testReplacements: tests };
}
/** 🔗️Resolves filesystem-backed Go module replacements without admitting registry dependencies. */
function localGoReplacementRoots(moduleRoot) {
    var goMod = (0, node_path_1.join)(moduleRoot, "go.mod");
    if (!(0, node_fs_1.existsSync)(goMod))
        return [];
    var replacements = [];
    var block = false;
    for (var _i = 0, _a = (0, node_fs_1.readFileSync)(goMod, "utf8").split(/\r?\n/u); _i < _a.length; _i++) {
        var raw = _a[_i];
        var line = raw.replace(/\/\/.*$/u, "").trim();
        if (!line)
            continue;
        if (/^replace\s*\($/u.test(line)) {
            block = true;
            continue;
        }
        if (block && line === ")") {
            block = false;
            continue;
        }
        var directive = block ? line : line.startsWith("replace ") ? line.slice("replace ".length).trim() : "";
        if (!directive)
            continue;
        var parts = directive.split(/\s+/u);
        var arrow = parts.indexOf("=>");
        var replacement = arrow < 0 ? undefined : parts[arrow + 1];
        if (!replacement || (!(0, node_path_1.isAbsolute)(replacement) && !replacement.startsWith(".")))
            continue;
        var candidate = (0, node_path_1.resolve)(moduleRoot, replacement);
        if (!(0, node_fs_1.existsSync)((0, node_path_1.join)(candidate, "go.mod")))
            continue;
        replacements.push((0, node_fs_1.realpathSync)(candidate));
    }
    return __spreadArray([], new Set(replacements), true).sort();
}
/** 🐹️Maps domain-owned Go sources and cases, including local module replacements, to virtual compiler inputs. */
function canonicalGoPlan(moduleRoot, layout) {
    if (layout === void 0) { layout = canonicalGoLayout(); }
    var validNames = function (names) { return names.length > 0 && new Set(names).size === names.length && names.every(function (name) { return !!name && !name.includes("/") && !name.includes("\\"); }); };
    if (!layout.testsDirectory || layout.testsDirectory.includes("/") || layout.testsDirectory.includes("\\") || !layout.implementationFilename.endsWith(".go") || layout.implementationFilename.includes("/") || layout.implementationFilename.includes("\\") || !validNames(layout.opaqueDirectoryNames) || !validNames(layout.sourceDirectoryNames) || layout.opaqueDirectoryNames.includes(layout.testsDirectory) || layout.sourceDirectoryNames.includes(layout.testsDirectory) || layout.sourceDirectoryNames.some(function (name) { return layout.opaqueDirectoryNames.includes(name); }))
        throw new Error("Invalid canonical Go compiler-input layout.");
    var root = (0, node_fs_1.realpathSync)(moduleRoot);
    var primary = canonicalGoModulePlan(root, layout);
    var sourceReplacements = __assign({}, primary.sourceReplacements);
    var visited = new Set([root]);
    var addDependencies = function (owner) {
        for (var _i = 0, _a = localGoReplacementRoots(owner); _i < _a.length; _i++) {
            var dependency = _a[_i];
            if (visited.has(dependency))
                continue;
            visited.add(dependency);
            var plan = canonicalGoModulePlan(dependency, layout);
            for (var _b = 0, _c = Object.entries(plan.sourceReplacements); _b < _c.length; _b++) {
                var _d = _c[_b], virtual = _d[0], source = _d[1];
                var existing = sourceReplacements[virtual];
                if (existing && existing !== source)
                    throw new Error("Go input overlay collision: ".concat(virtual));
                sourceReplacements[virtual] = source;
            }
            addDependencies(dependency);
        }
    };
    addDependencies(root);
    var sorted = function (values) { return Object.fromEntries(Object.entries(values).sort(function (_a, _b) {
        var left = _a[0];
        var right = _b[0];
        return left.localeCompare(right);
    })); };
    var sources = sorted(sourceReplacements);
    return { packages: primary.packages, replacements: sorted(__assign(__assign({}, sources), primary.testReplacements)), sourceReplacements: sources, testReplacements: primary.testReplacements };
}
/** 🧪️Runs canonical Go cases through the standard toolchain without authored legacy filenames. */
function runCanonicalGoTests(moduleRoot_1, args_1) {
    return __awaiter(this, arguments, void 0, function (moduleRoot, args, opts) {
        var plan, packages, _i, packages_1, selected, inheritedOwner, owner, temporary, overlay, cleanup;
        var _a, _b, _c;
        if (opts === void 0) { opts = {}; }
        return __generator(this, function (_d) {
            switch (_d.label) {
                case 0:
                    plan = canonicalGoPlan(moduleRoot);
                    packages = (_a = opts.packages) !== null && _a !== void 0 ? _a : plan.packages;
                    if (packages.length === 0)
                        throw new Error("No canonical Go inputs found below ".concat((0, node_fs_1.realpathSync)(moduleRoot)));
                    for (_i = 0, packages_1 = packages; _i < packages_1.length; _i++) {
                        selected = packages_1[_i];
                        if (!plan.packages.includes(selected))
                            throw new Error("Go compiler package is outside the canonical plan: ".concat(selected));
                    }
                    inheritedOwner = (_b = opts.env) === null || _b === void 0 ? void 0 : _b.SEMIO_GO_OVERLAY_OWNER;
                    owner = inheritedOwner ? (0, node_fs_1.realpathSync)(inheritedOwner) : (0, node_fs_1.mkdtempSync)((0, node_path_1.join)((0, node_os_1.tmpdir)(), "semio-go-tests-"));
                    temporary = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(owner, "run-"));
                    overlay = (0, node_path_1.join)(temporary, "overlay.json");
                    cleanup = function () { return (0, node_fs_1.rmSync)(inheritedOwner ? temporary : owner, { recursive: true, force: true }); };
                    (0, node_fs_1.writeFileSync)(overlay, "".concat(JSON.stringify({ Replace: plan.replacements }, null, 2), "\n"));
                    process.once("exit", cleanup);
                    _d.label = 1;
                case 1:
                    _d.trys.push([1, , 3, 4]);
                    return [4 /*yield*/, runTestBudgeted("go", __spreadArray(__spreadArray(["test", "-overlay=".concat(overlay)], args, true), packages, true), { cwd: moduleRoot, env: __assign(__assign({}, ((_c = opts.env) !== null && _c !== void 0 ? _c : process.env)), { SEMIO_GO_OVERLAY_OWNER: owner }), budgetMs: opts.budgetMs, throwOnFailure: true })];
                case 2:
                    _d.sent();
                    return [3 /*break*/, 4];
                case 3:
                    process.off("exit", cleanup);
                    cleanup();
                    return [7 /*endfinally*/];
                case 4: return [2 /*return*/];
            }
        });
    });
}
/** 🏗️Builds domain-owned Go sources through the same standard-toolchain projection used by tests. */
function runCanonicalGoBuild(moduleRoot, args, opts) {
    if (opts === void 0) { opts = {}; }
    var plan = canonicalGoPlan(moduleRoot);
    var temporary = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)((0, node_os_1.tmpdir)(), "semio-go-build-"));
    var overlay = (0, node_path_1.join)(temporary, "overlay.json");
    var cleanup = function () { return (0, node_fs_1.rmSync)(temporary, { recursive: true, force: true }); };
    (0, node_fs_1.writeFileSync)(overlay, "".concat(JSON.stringify({ Replace: plan.replacements }, null, 2), "\n"));
    process.once("exit", cleanup);
    try {
        (0, ____ts_6.runCmd)("go", __spreadArray(["build", "-overlay=".concat(overlay)], args, true), { cwd: moduleRoot, env: opts.env, budgetMs: opts.budgetMs });
    }
    finally {
        process.off("exit", cleanup);
        cleanup();
    }
}
/** ⏱️Vitest per-test/hook/teardown timeouts (ms) for the active level budget. */
function vitestLevelArgs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    var ms = String(testLevelBudgetMs(level));
    return ["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms];
}
/** ⏱️`bun test` `--timeout` (ms) for the active level budget. */
function bunTestLevelArgs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    return ["--timeout", String(testLevelBudgetMs(level))];
}
/** ⏱️Cumulative pytest args for the active level: `-m` marker filter, per-test `--timeout`, and `--timeout-method=thread`. */
function pytestLevelArgs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    var excluded = levelsAbove(level);
    var args = excluded.length ? ["-m", excluded.map(function (l) { return "not ".concat(l); }).join(" and ")] : [];
    args.push("--timeout=".concat(testLevelBudgetSeconds(level)), "--timeout-method=thread");
    return args;
}
/** ⏱️Cumulative dotnet xunit args for the active level: `--filter` for levels above it and `--blame-hang` with a per-test timeout (ms). */
function dotnetLevelArgs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    var excluded = levelsAbove(level);
    var args = excluded.length ? ["--filter", excluded.map(function (l) { return "Category!=".concat(l); }).join("&")] : [];
    args.push("--blame-hang", "--blame-hang-timeout", String(testLevelBudgetMs(level)));
    return args;
}
/** ⏱️Playwright per-spec timeout (ms) — active level budget via [[testLevelBudgetMs]]. */
function playwrightTestTimeoutMs(level) {
    if (level === void 0) { level = activeTestLevel(); }
    return testLevelBudgetMs(level);
}
/**
 * ⏱️SIGKILLs `pid`'s whole process tree, not just `pid` — a timed-out `spawnSync`/`execFileSync` only signals the
 * direct child, leaking forked worker pools (e.g. vitest's `workers/forks.js`) that keep burning CPU indefinitely.
 * Requires the child to have been spawned with `detached: true` on POSIX so `pid` is its own process-group leader.
 */
function killBudgetTree(pid) {
    (0, ____ts_6.terminateOwnedProcessTree)(pid);
}
/** @emoji 🎭️ Playwright is TEST-ONLY and loaded lazily. The specifier is indirected so no bundler can
 * statically resolve it: this module is reachable from `⚙️vite.config.ts`, and a literal
 * `import("playwright")` makes bun follow it into a browser build, failing on the uninstalled
 * optional `chromium-bidi`. Runtime behaviour is identical. */
var PLAYWRIGHT_MODULE_SPECIFIER = "playwright";
/**
 * ⏱️Runs a command under the test-level budget; an explicit zero permits unlimited build preparation.
 * SIGKILLs the whole process tree and fails loudly when a positive budget expires.
 * Deliberately async: Bun's `spawnSync`/`execFileSync` `detached` option does not put the child in its own
 * process group (verified — only the async `spawn` does), so tree-killing on timeout requires the async form.
 * Callers may fire-and-forget this from a synchronous `void`-returning context — the process stays alive on
 * the pending child/timer handles regardless, and the eventual `process.exit()` below still takes effect.
 */
function runTestBudgeted(cmd_1, args_1) {
    return __awaiter(this, arguments, void 0, function (cmd, args, opts) {
        var budgetMs, child, timedOut, timer, _a, code, signal, hint;
        var _b, _c, _d;
        if (opts === void 0) { opts = {}; }
        return __generator(this, function (_e) {
            switch (_e.label) {
                case 0:
                    budgetMs = (_b = opts.budgetMs) !== null && _b !== void 0 ? _b : testLevelBudgetMs();
                    child = (0, node_child_process_1.spawn)(cmd, args, { stdio: "inherit", cwd: opts.cwd, env: (_c = opts.env) !== null && _c !== void 0 ? _c : process.env, detached: process.platform !== "win32" });
                    timedOut = false;
                    timer = budgetMs > 0 ? setTimeout(function () {
                        timedOut = true;
                        if (child.pid)
                            killBudgetTree(child.pid);
                    }, budgetMs) : undefined;
                    return [4 /*yield*/, new Promise(function (resolveExit, rejectExit) {
                            child.on("error", rejectExit);
                            child.on("exit", function (exitCode, exitSignal) { return resolveExit({ code: exitCode, signal: exitSignal }); });
                        }).finally(function () { return clearTimeout(timer); })];
                case 1:
                    _a = _e.sent(), code = _a.code, signal = _a.signal;
                    if (timedOut) {
                        hint = (_d = opts.onTimeoutHint) !== null && _d !== void 0 ? _d : "Trim it, or assign it to a higher level (quick/long/exhaustive).";
                        console.error("[budget] ".concat(cmd, " ").concat(args.join(" "), " exceeded ").concat(budgetMs, "ms \u2014 killed. ").concat(hint));
                        if (opts.throwOnFailure)
                            throw new Error("".concat(cmd, " exceeded its ").concat(budgetMs, "ms budget"));
                        process.exit(1);
                    }
                    if (signal || code !== 0) {
                        if (opts.throwOnFailure)
                            throw new Error(signal ? "".concat(cmd, " was killed by ").concat(signal) : "".concat(cmd, " exited with status ").concat(code));
                        process.exit(code !== null && code !== void 0 ? code : 1);
                    }
                    return [2 /*return*/];
            }
        });
    });
}
/** 📦️Runs a build-budgeted command while capturing metadata and replaying diagnostics only on failure. */
function capturedTestFailureDiagnostics(stdout, stderr) {
    var _a, _b, _c, _d, _e, _f, _g, _h;
    var rendered = [];
    for (var _i = 0, _j = stdout.split(/\r?\n/); _i < _j.length; _i++) {
        var line = _j[_i];
        try {
            var message = JSON.parse(line);
            if (message.reason === "compiler-message" && ((_a = message.message) === null || _a === void 0 ? void 0 : _a.level) === "error") {
                var span = (_c = (_b = message.message.spans) === null || _b === void 0 ? void 0 : _b.find(function (candidate) { return candidate.is_primary; })) !== null && _c !== void 0 ? _c : (_d = message.message.spans) === null || _d === void 0 ? void 0 : _d[0];
                var location_1 = (span === null || span === void 0 ? void 0 : span.file_name) ? "\n  --> ".concat(span.file_name, ":").concat((_e = span.line_start) !== null && _e !== void 0 ? _e : 0, ":").concat((_f = span.column_start) !== null && _f !== void 0 ? _f : 0) : "";
                rendered.push((_g = message.message.rendered) !== null && _g !== void 0 ? _g : "".concat((_h = message.message.message) !== null && _h !== void 0 ? _h : "Cargo compiler error").concat(location_1));
            }
        }
        catch (_k) { }
    }
    var stderrErrors = stderr.split(/\r?\n/).filter(function (line) { return /^(?:error(?:\[[A-Z]\d+\])?:|Caused by:)/.test(line); });
    var diagnostics = __spreadArray(__spreadArray([], rendered, true), stderrErrors, true);
    return diagnostics.length > 0 ? diagnostics.join("\n") : "".concat(stdout.slice(-16 * 1024)).concat(stderr.slice(-16 * 1024));
}
function runTestCapturedBudgeted(cmd, args, opts) {
    return __awaiter(this, void 0, void 0, function () {
        var child, stdout, stderr, timedOut, timer, _a, code, signal, diagnostics, diagnostics;
        var _b, _c, _d, _e, _f;
        return __generator(this, function (_g) {
            switch (_g.label) {
                case 0:
                    child = (0, node_child_process_1.spawn)(cmd, args, { stdio: ["inherit", "pipe", "pipe"], cwd: opts.cwd, env: (_b = opts.env) !== null && _b !== void 0 ? _b : process.env, detached: process.platform !== "win32" });
                    stdout = "";
                    stderr = "";
                    (_c = child.stdout) === null || _c === void 0 ? void 0 : _c.setEncoding("utf8");
                    (_d = child.stdout) === null || _d === void 0 ? void 0 : _d.on("data", function (chunk) {
                        stdout += chunk;
                    });
                    (_e = child.stderr) === null || _e === void 0 ? void 0 : _e.setEncoding("utf8");
                    (_f = child.stderr) === null || _f === void 0 ? void 0 : _f.on("data", function (chunk) {
                        stderr += chunk;
                    });
                    timedOut = false;
                    timer = opts.budgetMs > 0 ? setTimeout(function () {
                        timedOut = true;
                        if (child.pid)
                            killBudgetTree(child.pid);
                    }, opts.budgetMs) : undefined;
                    return [4 /*yield*/, new Promise(function (resolveExit, rejectExit) {
                            child.on("error", rejectExit);
                            child.on("exit", function (exitCode, exitSignal) { return resolveExit({ code: exitCode, signal: exitSignal }); });
                        }).finally(function () { return clearTimeout(timer); })];
                case 1:
                    _a = _g.sent(), code = _a.code, signal = _a.signal;
                    if (timedOut) {
                        diagnostics = capturedTestFailureDiagnostics(stdout, stderr);
                        if (diagnostics)
                            process.stderr.write(diagnostics);
                        console.error("[budget] ".concat(cmd, " ").concat(args.join(" "), " exceeded ").concat(opts.budgetMs, "ms \u2014 killed. ").concat((0, ____ts_6.budgetTimeoutHint)(cmd, opts.onTimeoutHint)));
                        process.exit(1);
                    }
                    if (signal || code !== 0) {
                        diagnostics = capturedTestFailureDiagnostics(stdout, stderr);
                        if (diagnostics)
                            process.stderr.write(diagnostics);
                        process.exit(code !== null && code !== void 0 ? code : 1);
                    }
                    return [2 /*return*/, stdout];
            }
        });
    });
}
var cachedCrateIndex = (0, framework_1.ephemeralBox)("framework.products.repo.modules.lib.packages.typescript.index.ts.cachedCrateIndex", null);
var CARGO_PREFIX_WORDS = new Set(["semio", "s", "framework", "os", "kernel", "plugin", "module", "tech", "app"]);
function generateCargoVariants(name) {
    var variants = new Set([name]);
    var parts = name.split("-");
    var firstNonPrefix = parts.length - 1;
    for (var i = 0; i < parts.length; i++) {
        if (!CARGO_PREFIX_WORDS.has(parts[i])) {
            firstNonPrefix = i;
            break;
        }
    }
    var coreSuffix = parts.slice(firstNonPrefix).join("-");
    variants.add(coreSuffix);
    var prefixParts = parts.slice(0, firstNonPrefix);
    for (var i = 0; i < prefixParts.length; i++) {
        variants.add(parts.slice(i).join("-"));
        variants.add(__spreadArray([prefixParts[i]], parts.slice(firstNonPrefix), true).join("-"));
    }
    return Array.from(variants);
}
/** 🦀️ Indexes admitted Cargo manifests without probing opaque nodes or following links. */
function getCargoWorkspaceIndex(repoRoot) {
    if (repoRoot === void 0) { repoRoot = (0, ____ts_5.getWorkspaceRoot)(); }
    if (cachedCrateIndex.current)
        return cachedCrateIndex.current;
    var taxonomy = (0, ____ts_4.loadTaxonomy)();
    var exactPkgNames = new Set();
    var libNameToCrates = new Map();
    var aliasToCrates = new Map();
    var addAlias = function (alias, record) {
        var _a;
        if (!alias)
            return;
        var normalized = alias.replaceAll("-", "_").toLowerCase();
        var list = (_a = aliasToCrates.get(normalized)) !== null && _a !== void 0 ? _a : [];
        if (!list.includes(record))
            list.push(record);
        aliasToCrates.set(normalized, list);
    };
    var walk = function (dir) {
        var _a;
        var relativePath = (0, node_path_1.relative)(repoRoot, dir);
        if (relativePath && (0, ____ts_4.taxonomyRelativePathIsExcluded)(relativePath, taxonomy))
            return;
        if (dir.includes("node_modules") || dir.includes("target") || dir.includes(".git") || dir.includes(".🧬semio"))
            return;
        for (var _i = 0, _b = (0, node_fs_1.readdirSync)(dir); _i < _b.length; _i++) {
            var name_2 = _b[_i];
            var full = (0, node_path_1.join)(dir, name_2);
            if ((0, ____ts_4.taxonomyRelativePathIsExcluded)((0, node_path_1.relative)(repoRoot, full), taxonomy))
                continue;
            var ent = (0, node_fs_1.lstatSync)(full);
            if (ent.isDirectory()) {
                walk(full);
            }
            else if (ent.isFile() && name_2 === "Cargo.toml" && full !== (0, node_path_1.join)(repoRoot, "Cargo.toml")) {
                try {
                    var content = (0, node_fs_1.readFileSync)(full, "utf8");
                    var pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                    var libMatch = content.match(/\[lib\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                    if (pkgMatch) {
                        var pkgName = pkgMatch[1];
                        var libName = libMatch ? libMatch[1] : pkgName.replaceAll("-", "_");
                        var record = { dir: (0, node_path_1.dirname)(full), pkgName: pkgName, libName: libName };
                        exactPkgNames.add(pkgName);
                        var libList = (_a = libNameToCrates.get(libName)) !== null && _a !== void 0 ? _a : [];
                        if (!libList.includes(record))
                            libList.push(record);
                        libNameToCrates.set(libName, libList);
                        addAlias(libName, record);
                        addAlias(pkgName, record);
                        for (var _c = 0, _d = generateCargoVariants(pkgName); _c < _d.length; _c++) {
                            var v = _d[_c];
                            addAlias(v, record);
                        }
                        if (libName) {
                            for (var _e = 0, _f = generateCargoVariants(libName.replaceAll("_", "-")); _e < _f.length; _e++) {
                                var v = _f[_e];
                                addAlias(v, record);
                            }
                        }
                    }
                }
                catch (_g) {
                    /* ignore unreadable Cargo.toml */
                }
            }
        }
    };
    walk(repoRoot);
    cachedCrateIndex.current = { exactPkgNames: exactPkgNames, libNameToCrates: libNameToCrates, aliasToCrates: aliasToCrates };
    return cachedCrateIndex.current;
}
function resolveCargoPackageName(pkg, cwd) {
    var index = getCargoWorkspaceIndex();
    if (index.exactPkgNames.has(pkg))
        return pkg;
    // 1. Check local Cargo.toml in cwd, cwd/rs, dirname(cwd)
    for (var _i = 0, _a = [cwd, (0, node_path_1.join)(cwd, "rs"), (0, node_path_1.dirname)(cwd)]; _i < _a.length; _i++) {
        var candidateDir = _a[_i];
        var cargoPath = (0, node_path_1.join)(candidateDir, "Cargo.toml");
        if ((0, node_fs_1.existsSync)(cargoPath)) {
            try {
                var content = (0, node_fs_1.readFileSync)(cargoPath, "utf8");
                var pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                var libMatch = content.match(/\[lib\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                var localPkg = pkgMatch ? pkgMatch[1] : null;
                var localLib = libMatch ? libMatch[1] : localPkg ? localPkg.replaceAll("-", "_") : null;
                if (localPkg) {
                    if (pkg === localLib || pkg === localPkg || pkg.replaceAll("-", "_") === localLib || pkg.replaceAll("_", "-") === localPkg) {
                        return localPkg;
                    }
                }
            }
            catch (_b) {
                /* ignore */
            }
        }
    }
    // 2. Direct libName lookup
    var byLib = index.libNameToCrates.get(pkg);
    if (byLib && byLib.length > 0) {
        if (byLib.length === 1)
            return byLib[0].pkgName;
        var best = byLib[0];
        var bestDist = Infinity;
        for (var _c = 0, byLib_1 = byLib; _c < byLib_1.length; _c++) {
            var item = byLib_1[_c];
            var dist = (0, node_path_1.relative)(cwd, item.dir).length;
            if (dist < bestDist) {
                bestDist = dist;
                best = item;
            }
        }
        return best.pkgName;
    }
    // 3. Alias / prefix-variant lookup
    var normPkg = pkg.replaceAll("-", "_").toLowerCase();
    var byAlias = index.aliasToCrates.get(normPkg);
    if (byAlias && byAlias.length > 0) {
        if (byAlias.length === 1)
            return byAlias[0].pkgName;
        var best = byAlias[0];
        var bestDist = Infinity;
        for (var _d = 0, byAlias_1 = byAlias; _d < byAlias_1.length; _d++) {
            var item = byAlias_1[_d];
            var dist = (0, node_path_1.relative)(cwd, item.dir).length;
            if (dist < bestDist) {
                bestDist = dist;
                best = item;
            }
        }
        return best.pkgName;
    }
    // 4. Fallback if cwd has Cargo.toml
    for (var _e = 0, _f = [cwd, (0, node_path_1.join)(cwd, "rs")]; _e < _f.length; _e++) {
        var candidateDir = _f[_e];
        var cargoPath = (0, node_path_1.join)(candidateDir, "Cargo.toml");
        if ((0, node_fs_1.existsSync)(cargoPath)) {
            try {
                var content = (0, node_fs_1.readFileSync)(cargoPath, "utf8");
                var pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                if (pkgMatch)
                    return pkgMatch[1];
            }
            catch (_g) {
                /* ignore */
            }
        }
    }
    return pkg;
}
function resolveCargoPackageNames(packages, cwd) {
    if (packages.length === 0) {
        for (var _i = 0, _a = [cwd, (0, node_path_1.join)(cwd, "rs")]; _i < _a.length; _i++) {
            var candidateDir = _a[_i];
            var cargoPath = (0, node_path_1.join)(candidateDir, "Cargo.toml");
            if ((0, node_fs_1.existsSync)(cargoPath)) {
                try {
                    var content = (0, node_fs_1.readFileSync)(cargoPath, "utf8");
                    var pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
                    if (pkgMatch)
                        return [pkgMatch[1]];
                }
                catch (_b) {
                    /* ignore */
                }
            }
        }
        return [];
    }
    return packages.map(function (pkg) { return resolveCargoPackageName(pkg, cwd); });
}
//#endregion 🦀️CargoPackageResolver
//#region 🦀️NextestExecutionFilters
/** 🧪️ Keeps runtime selection on the metadata execution command and build selection on the warm build. */
function partitionNextestExecutionFilters(args) {
    var valuedFilters = new Set(["-E", "--filter-expr", "--partition", "--run-ignored"]);
    var requiredBuildOptions = new Set([
        "-p",
        "--package",
        "--exclude",
        "--manifest-path",
        "--target",
        "--target-dir",
        "--features",
        "-F",
        "--jobs",
        "-j",
        "--build-jobs",
        "--cargo-profile",
        "--cargo-message-format",
        "--config",
        "-Z",
        "--color",
        "--profile",
        "-P",
        "--test",
        "--bin",
        "--bench",
        "--example",
        "--message-format",
        "-T",
        "--list-type",
        "--archive-file",
        "--archive-format",
        "--extract-to",
        "--cargo-metadata",
        "--workspace-remap",
        "--binaries-metadata",
        "--target-dir-remap",
        "--build-dir-remap",
        "--config-file",
        "--user-config-file",
        "--tool-config-file",
    ]);
    var optionalBuildOptions = new Set(["--timings"]);
    var joinedBuildOptions = ["-p", "-F", "-j", "-Z", "-P", "-T"];
    var buildArgs = [], executionArgs = [];
    var separator = args.indexOf("--");
    var cargoArgs = separator < 0 ? args : args.slice(0, separator);
    var libtestArgs = separator < 0 ? [] : args.slice(separator + 1);
    var _loop_1 = function (index) {
        var arg = cargoArgs[index];
        var key = arg.split("=", 1)[0];
        if (arg === "--ignore-default-filter" || arg === "--no-fail-fast" || (arg.startsWith("-E") && arg.length > 2)) {
            executionArgs.push(arg);
        }
        else if (valuedFilters.has(key)) {
            if (arg.includes("=")) {
                if (arg.endsWith("="))
                    throw new Error("Nextest filter ".concat(key, " requires a value"));
                executionArgs.push(arg);
            }
            else {
                var value = cargoArgs[index + 1];
                if (value === undefined || value.length === 0 || value.startsWith("-"))
                    throw new Error("Nextest filter ".concat(key, " requires a value"));
                executionArgs.push(arg, value);
                index += 1;
            }
        }
        else {
            if (!arg.startsWith("-"))
                executionArgs.push(arg);
            else {
                buildArgs.push(arg);
                var inlineValue = arg.includes("=") || joinedBuildOptions.some(function (option) { return arg.startsWith(option) && arg.length > option.length; });
                var requiresValue = requiredBuildOptions.has(key);
                var allowsValue = optionalBuildOptions.has(key);
                if ((requiresValue || allowsValue) && arg.endsWith("="))
                    throw new Error("Nextest build option ".concat(key, " requires a non-empty value"));
                if (!inlineValue && requiresValue) {
                    var value = cargoArgs[index + 1];
                    if (value !== undefined && value.length > 0 && !value.startsWith("-")) {
                        buildArgs.push(value);
                        index += 1;
                    }
                    else if (requiresValue)
                        throw new Error("Nextest build option ".concat(key, " requires a value"));
                }
            }
        }
        out_index_1 = index;
    };
    var out_index_1;
    for (var index = 0; index < cargoArgs.length; index += 1) {
        _loop_1(index);
        index = out_index_1;
    }
    return { buildArgs: buildArgs, executionArgs: executionArgs, libtestArgs: libtestArgs };
}
/** 📁️ Selects a caller-owned artifact root without embedding any task identity in permanent tooling. */
function nextestArtifactLocation(cwd, env) {
    var _a;
    if (env === void 0) { env = process.env; }
    var explicit = (_a = env.SEMIO_TEST_ARTIFACT_DIR) === null || _a === void 0 ? void 0 : _a.trim();
    return explicit ? { directory: (0, node_path_1.resolve)(cwd, explicit), retain: true } : { directory: (0, node_os_1.tmpdir)(), retain: false };
}
//#endregion 🦀️NextestExecutionFilters
/**
 * 🦀️Warm-builds the exact test runner invocation with the opt-in [[buildBudgetMs]], then runs
 * assertions under the active level's budget and [[nextest.toml]] profile (per-test
 * `slow-timeout`), appending cumulative `--skip <level>::` filters for every level above it (tests live in
 * `mod quick`/`mod long`/`mod exhaustive` submodules inside `mod tests`; unscoped tests are `fundamental`).
 * Splits `extraArgs` on an existing `--` so callers passing their own libtest args (e.g. `--nocapture`) still
 * compose correctly. Guarantees a 128 MiB `RUST_MIN_STACK` floor for every assertion thread unless the
 * caller sets its own, because app-fixture laws routinely exceed libtest's 2 MiB default stack.
 */
function runCargoTestBudgeted(packages_2, cwd_1) {
    return __awaiter(this, arguments, void 0, function (packages, cwd, extraArgs, env) {
        var resolvedPackages, packageArgs, dashIdx, cargoArgs, libtestArgs, level, skipArgs, profileArgs, assertionThreads, assertionThreadArgs, testBudgetMs, nextest, covArgs, buildArgs, lcovPath, _a, buildArgs, executionArgs, nextestLibtestArgs, artifactLocation, metadataDir, binariesMetadataPath, binariesMetadata;
        if (extraArgs === void 0) { extraArgs = []; }
        if (env === void 0) { env = process.env; }
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    env = env.RUST_MIN_STACK ? env : __assign(__assign({}, env), { RUST_MIN_STACK: "134217728" });
                    resolvedPackages = resolveCargoPackageNames(packages, cwd);
                    packageArgs = resolvedPackages.flatMap(function (pkg) { return ["-p", pkg]; });
                    dashIdx = extraArgs.indexOf("--");
                    cargoArgs = dashIdx === -1 ? extraArgs : extraArgs.slice(0, dashIdx);
                    libtestArgs = dashIdx === -1 ? [] : extraArgs.slice(dashIdx + 1);
                    level = isTestLevel(env.SEMIO_TEST_LEVEL) ? env.SEMIO_TEST_LEVEL : activeTestLevel();
                    skipArgs = levelsAbove(level).flatMap(function (l) { return ["--skip", "".concat(l, "::")]; });
                    profileArgs = ["--profile", level];
                    assertionThreads = Math.max(1, (0, node_os_1.availableParallelism)() - Math.max(1, Math.ceil((0, node_os_1.availableParallelism)() / 4)));
                    assertionThreadArgs = level === "fundamental" ? ["--test-threads", String(assertionThreads)] : [];
                    if (!coverageEnabled()) return [3 /*break*/, 4];
                    testBudgetMs = testLevelBudgetMs(level);
                    nextest = cargoNextestAvailable();
                    covArgs = nextest
                        ? __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(["llvm-cov", "nextest", "--release", "--no-report", "--no-tests", "warn"], profileArgs, true), packageArgs, true), cargoArgs, true), ["--"], false), libtestArgs, true), skipArgs, true)
                        : __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(["llvm-cov", "test", "--release", "--no-report"], packageArgs, true), cargoArgs, true), ["--"], false), libtestArgs, true), skipArgs, true);
                    if (!nextest) {
                        console.error("[budget] cargo-nextest not installed — coverage uses cargo llvm-cov test fallback");
                    }
                    buildArgs = nextest ? __spreadArray(["llvm-cov", "nextest", "--no-run"], covArgs.slice(2), true) : __spreadArray(__spreadArray([], covArgs, true), ["--list"], false);
                    return [4 /*yield*/, runTestBudgeted("cargo", buildArgs, {
                            cwd: cwd,
                            env: env,
                            budgetMs: (0, ____ts_6.buildBudgetMs)(),
                            onTimeoutHint: (0, ____ts_6.budgetTimeoutHint)("cargo"),
                        })];
                case 1:
                    _b.sent();
                    return [4 /*yield*/, runTestBudgeted("cargo", __spreadArray(["llvm-cov", covArgs[1], "--no-clean"], covArgs.slice(2), true), { cwd: cwd, env: env, budgetMs: testBudgetMs })];
                case 2:
                    _b.sent();
                    lcovPath = (0, node_path_1.join)(coverageDir((0, ____ts_7.findRepoRoot)(cwd), "rust"), "".concat(coverageSlug(resolvedPackages.join("_")), ".lcov"));
                    return [4 /*yield*/, runTestBudgeted("cargo", __spreadArray(__spreadArray(["llvm-cov", "report", "--release", "--lcov"], packageArgs, true), ["--output-path", lcovPath], false), {
                            cwd: cwd,
                            env: env,
                            budgetMs: (0, ____ts_6.buildBudgetMs)(),
                            onTimeoutHint: (0, ____ts_6.budgetTimeoutHint)("cargo"),
                        })];
                case 3:
                    _b.sent();
                    return [2 /*return*/];
                case 4:
                    if (!cargoNextestAvailable()) return [3 /*break*/, 10];
                    _a = partitionNextestExecutionFilters(extraArgs), buildArgs = _a.buildArgs, executionArgs = _a.executionArgs, nextestLibtestArgs = _a.libtestArgs;
                    artifactLocation = nextestArtifactLocation(cwd, env);
                    if (artifactLocation.retain)
                        (0, node_fs_1.mkdirSync)(artifactLocation.directory, { recursive: true });
                    metadataDir = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(artifactLocation.directory, "semio-nextest-"));
                    binariesMetadataPath = (0, node_path_1.join)(metadataDir, "binaries-metadata.json");
                    _b.label = 5;
                case 5:
                    _b.trys.push([5, , 8, 9]);
                    return [4 /*yield*/, runTestCapturedBudgeted("cargo", __spreadArray(__spreadArray(__spreadArray(["nextest", "list", "--list-type", "binaries-only", "--message-format", "json"], profileArgs, true), packageArgs, true), buildArgs, true), {
                            cwd: cwd,
                            env: env,
                            budgetMs: (0, ____ts_6.buildBudgetMs)(),
                            onTimeoutHint: (0, ____ts_6.budgetTimeoutHint)("cargo"),
                        })];
                case 6:
                    binariesMetadata = _b.sent();
                    (0, node_fs_1.writeFileSync)(binariesMetadataPath, binariesMetadata);
                    return [4 /*yield*/, runTestBudgeted("cargo", __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray([
                            "nextest",
                            "run",
                            "--binaries-metadata",
                            binariesMetadataPath,
                            "--no-tests",
                            "warn",
                            "--status-level",
                            "fail",
                            "--final-status-level",
                            "fail"
                        ], assertionThreadArgs, true), profileArgs, true), executionArgs, true), [
                            "--"
                        ], false), nextestLibtestArgs, true), skipArgs, true), { cwd: cwd, env: env, budgetMs: testLevelBudgetMs(level) })];
                case 7:
                    _b.sent();
                    return [3 /*break*/, 9];
                case 8:
                    if (artifactLocation.retain)
                        console.error("[DEBUG] Nextest artifacts retained at ".concat(metadataDir));
                    else
                        (0, node_fs_1.rmSync)(metadataDir, { recursive: true, force: true });
                    return [7 /*endfinally*/];
                case 9: return [2 /*return*/];
                case 10:
                    console.error("[budget] cargo-nextest not installed — falling back to cargo test (run setup or: cargo install cargo-nextest --locked)");
                    return [4 /*yield*/, runTestBudgeted("cargo", __spreadArray(["build", "--tests"], packageArgs, true), {
                            cwd: cwd,
                            env: env,
                            budgetMs: (0, ____ts_6.buildBudgetMs)(),
                            onTimeoutHint: (0, ____ts_6.budgetTimeoutHint)("cargo"),
                        })];
                case 11:
                    _b.sent();
                    return [4 /*yield*/, runTestBudgeted("cargo", __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(["test"], packageArgs, true), cargoArgs, true), ["--"], false), libtestArgs, true), skipArgs, true), { cwd: cwd, env: env })];
                case 12:
                    _b.sent();
                    return [2 /*return*/];
            }
        });
    });
}
function cargoNextestAvailable() {
    var result = (0, node_child_process_1.spawnSync)("cargo", ["nextest", "--version"], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
    return result.status === 0;
}
var CAPTURE_WRAPPER_SOURCE = "\nconst { readFileSync, writeFileSync } = await import(\"node:fs\");\nconst { spawnSync } = await import(\"node:child_process\");\nconst spec = JSON.parse(readFileSync(process.env.SEMIO_PROCESS_CAPTURE_SPEC, \"utf8\"));\nconst input = spec.input === undefined ? undefined : Buffer.from(spec.input, \"base64\");\nconst result = spawnSync(spec.cmd, spec.args, { cwd: spec.cwd, env: spec.env, timeout: spec.timeout, killSignal: spec.killSignal, stdio: [input === undefined ? \"ignore\" : \"pipe\", \"pipe\", \"pipe\"], ...(input === undefined ? {} : { input }), maxBuffer: 536870912 });\nwriteFileSync(spec.stdoutPath, result.stdout ?? Buffer.alloc(0));\nwriteFileSync(spec.stderrPath, result.stderr ?? Buffer.alloc(0));\nconst sourceError = result.error;\nconst error = sourceError ? { message: sourceError.message, code: sourceError.code, errno: sourceError.errno, syscall: sourceError.syscall, path: sourceError.path, spawnargs: sourceError.spawnargs } : undefined;\nwriteFileSync(spec.metaPath, JSON.stringify({ status: result.status, signal: result.signal, error }));\n";
/** 📥️Captures subprocess bytes through a child-owned OS-file bridge so Bun test and native process hosts agree exactly. */
function spawnCapturedSync(cmd, args, opts) {
    var _a;
    if (opts === void 0) { opts = {}; }
    var captureRoot = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)((0, node_os_1.tmpdir)(), "semio-process-capture-"));
    var stdoutPath = (0, node_path_1.join)(captureRoot, "stdout");
    var stderrPath = (0, node_path_1.join)(captureRoot, "stderr");
    var metaPath = (0, node_path_1.join)(captureRoot, "meta.json");
    var specPath = (0, node_path_1.join)(captureRoot, "spec.json");
    var env = Object.fromEntries(Object.entries((_a = opts.env) !== null && _a !== void 0 ? _a : process.env).filter(function (entry) { return entry[1] !== undefined; }));
    (0, node_fs_1.writeFileSync)(specPath, JSON.stringify({ cmd: cmd, args: args, cwd: opts.cwd, env: env, input: opts.input === undefined ? undefined : Buffer.from(opts.input).toString("base64"), timeout: opts.timeout, killSignal: opts.killSignal, stdoutPath: stdoutPath, stderrPath: stderrPath, metaPath: metaPath }));
    try {
        var launcher = (0, node_child_process_1.spawnSync)(process.execPath, ["-e", CAPTURE_WRAPPER_SOURCE], {
            env: __assign(__assign({}, process.env), { SEMIO_PROCESS_CAPTURE_SPEC: specPath }),
            timeout: opts.timeout ? opts.timeout + 5000 : undefined,
            killSignal: opts.killSignal,
            stdio: "ignore",
        });
        if (!(0, node_fs_1.existsSync)(metaPath))
            return { status: launcher.status, stdout: Buffer.alloc(0), stderr: Buffer.alloc(0), signal: launcher.signal, error: launcher.error };
        var meta = JSON.parse((0, node_fs_1.readFileSync)(metaPath, "utf8"));
        var error = meta.error ? Object.assign(new Error(meta.error.message), meta.error) : undefined;
        return { status: meta.status, stdout: (0, node_fs_1.readFileSync)(stdoutPath), stderr: (0, node_fs_1.readFileSync)(stderrPath), signal: meta.signal, error: error };
    }
    finally {
        (0, node_fs_1.rmSync)(captureRoot, { recursive: true, force: true });
    }
}
/** 🔍️Budgeted capability probe with captured stdout/stderr — for `--version` checks that must not inherit stdio. */
function runProbe(cmd, args, opts) {
    var _a, _b;
    if (opts === void 0) { opts = {}; }
    var budgetMs = (_a = opts.budgetMs) !== null && _a !== void 0 ? _a : (0, ____ts_6.defaultBudgetMs)(cmd);
    var result = spawnCapturedSync(cmd, args, {
        cwd: opts.cwd,
        env: (_b = opts.env) !== null && _b !== void 0 ? _b : process.env,
        timeout: budgetMs,
        killSignal: "SIGKILL",
    });
    if (result.error) {
        if (result.error.code === "ETIMEDOUT") {
            console.error("[budget] ".concat(cmd, " ").concat(args.join(" "), " exceeded ").concat(budgetMs, "ms \u2014 killed. ").concat((0, ____ts_6.budgetTimeoutHint)(cmd, opts.onTimeoutHint)));
        }
        throw result.error;
    }
    return {
        status: result.status,
        stdout: result.stdout.toString("utf8"),
        stderr: result.stderr.toString("utf8"),
        signal: result.signal,
    };
}
exports.EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX = ".exact-cargo-laws-active-";
exports.EXACT_CARGO_ACTIVE_LEASE_MANIFEST = "lease.json";
exports.EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS = 120000;
/** 🛡️ Recognizes only a fresh lease owned by a live exact-Cargo runner process. */
function exactCargoGeneratedOutputHasLiveLease(root) {
    var stack = [{ path: root, depth: 0 }];
    while (stack.length > 0) {
        var current = stack.pop();
        var names = void 0;
        try {
            names = (0, node_fs_1.readdirSync)(current.path);
        }
        catch (_a) {
            continue;
        }
        for (var _i = 0, names_1 = names; _i < names_1.length; _i++) {
            var name_3 = names_1[_i];
            var path = (0, node_path_1.join)(current.path, name_3);
            var state = void 0;
            try {
                state = (0, node_fs_1.lstatSync)(path);
            }
            catch (_b) {
                continue;
            }
            if (!state.isDirectory() || state.isSymbolicLink())
                continue;
            if (name_3.startsWith(exports.EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX)) {
                var manifestPath = (0, node_path_1.join)(path, exports.EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
                try {
                    var manifestState = (0, node_fs_1.lstatSync)(manifestPath);
                    if (!manifestState.isFile() || manifestState.isSymbolicLink() || manifestState.size > 128 || Date.now() - manifestState.mtimeMs > exports.EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS || manifestState.mtimeMs - Date.now() > 5000)
                        continue;
                    var manifest = JSON.parse((0, node_fs_1.readFileSync)(manifestPath, "utf8"));
                    if (manifest.version !== 1 || !Number.isSafeInteger(manifest.pid) || Number(manifest.pid) < 1)
                        continue;
                    try {
                        process.kill(Number(manifest.pid), 0);
                        return true;
                    }
                    catch (error) {
                        if (error.code === "EPERM")
                            return true;
                    }
                }
                catch (_c) {
                    continue;
                }
            }
            else if (current.depth < 4)
                stack.push({ path: path, depth: current.depth + 1 });
        }
    }
    return false;
}
/** 💓 Holds a fresh process-bound lease until the exact Cargo run reaches a terminal result. */
function beginExactCargoLease(artifactRoot) {
    var _a;
    var leaseRoot = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(artifactRoot, exports.EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
    var manifestPath = (0, node_path_1.join)(leaseRoot, exports.EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
    var heartbeat = function () {
        try {
            (0, node_fs_1.writeFileSync)(manifestPath, JSON.stringify({ version: 1, pid: process.pid }), { mode: 384 });
        }
        catch (_a) { }
    };
    heartbeat();
    var timer = setInterval(heartbeat, 10000);
    (_a = timer.unref) === null || _a === void 0 ? void 0 : _a.call(timer);
    return function () {
        clearInterval(timer);
        (0, node_fs_1.rmSync)(leaseRoot, { recursive: true, force: true });
    };
}
/** 🚫️ Preserves the precise failing stage and actual child status independently of assertion parsing. */
var ExactCargoLawError = /** @class */ (function (_super) {
    __extends(ExactCargoLawError, _super);
    function ExactCargoLawError(stage, status, signal, artifactDir, detail) {
        var _this = _super.call(this, "exact Cargo law ".concat(stage, " failed: status=").concat(status, " signal=").concat(signal !== null && signal !== void 0 ? signal : "none", " artifacts=").concat(artifactDir, "; ").concat(detail)) || this;
        _this.stage = stage;
        _this.status = status;
        _this.signal = signal;
        _this.artifactDir = artifactDir;
        return _this;
    }
    return ExactCargoLawError;
}(Error));
exports.ExactCargoLawError = ExactCargoLawError;
/** 🔬️ Fingerprints one retained executable descriptor with bounded streaming and cancellation. */
function exactExecutableFingerprint(path, control) {
    var _a;
    if (control === void 0) { control = {}; }
    var check = function () {
        var _a;
        if ((_a = control.cancelled) === null || _a === void 0 ? void 0 : _a.call(control))
            throw new Error("Executable fingerprint cancelled");
    };
    check();
    if (!(0, node_path_1.isAbsolute)(path) || (0, node_fs_1.lstatSync)(path).isSymbolicLink())
        throw new Error("Executable must be one absolute regular file");
    var canonical = (0, node_fs_1.realpathSync)(path);
    var descriptor = (0, node_fs_1.openSync)(canonical, "r");
    try {
        var before_1 = (0, node_fs_1.fstatSync)(descriptor);
        var same = function (other) { return other.isFile() && !other.isSymbolicLink() && other.dev === before_1.dev && other.ino === before_1.ino && other.size === before_1.size && other.mtimeMs === before_1.mtimeMs && other.ctimeMs === before_1.ctimeMs; };
        if (!before_1.isFile() || !same((0, node_fs_1.lstatSync)(canonical)) || before_1.size <= 0 || before_1.size > 8 * Math.pow(1024, 3) || (process.platform !== "win32" && (before_1.mode & 73) === 0))
            throw new Error("Executable size or type denied");
        var digest = (0, node_crypto_1.createHash)("sha256");
        var buffer = Buffer.alloc(64 * 1024);
        var count = 0;
        while (true) {
            check();
            var length_1 = (0, node_fs_1.readSync)(descriptor, buffer);
            if (length_1 === 0)
                break;
            digest.update(buffer.subarray(0, length_1));
            count += length_1;
            if (count > before_1.size)
                throw new Error("Executable changed while hashing");
            (_a = control.progress) === null || _a === void 0 ? void 0 : _a.call(control, count, before_1.size);
        }
        var after = (0, node_fs_1.fstatSync)(descriptor);
        if (count !== before_1.size || !same(after) || !same((0, node_fs_1.lstatSync)(canonical)))
            throw new Error("Executable changed while hashing");
        return { path: canonical, sha256: digest.digest("hex"), byteLength: count };
    }
    finally {
        (0, node_fs_1.closeSync)(descriptor);
    }
}
/** 📥️ Captures a process into caller-owned evidence files; zero disables its deadline while retaining cancellation and output limits. */
function runExactCargoLawProcess(command, args, options) {
    return __awaiter(this, void 0, void 0, function () {
        var stdout, stderr;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    stdout = (0, node_fs_1.openSync)(options.stdoutPath, "wx", 384);
                    stderr = (0, node_fs_1.openSync)(options.stderrPath, "wx", 384);
                    return [4 /*yield*/, new Promise(function (resolveResult) {
                            var _a, _b;
                            var reason = "exit";
                            var count = 0;
                            var finished = false;
                            var child = (0, node_child_process_1.spawn)(command, args, { cwd: options.cwd, env: options.env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
                            var terminate = function (cause) {
                                if (reason !== "exit")
                                    return;
                                reason = cause;
                                if (child.pid)
                                    killBudgetTree(child.pid);
                            };
                            var append = function (descriptor, bytes) {
                                var remaining = Math.max(0, options.maxOutputBytes - count);
                                if (remaining)
                                    (0, node_fs_1.writeSync)(descriptor, bytes.subarray(0, remaining));
                                count += bytes.length;
                                if (count > options.maxOutputBytes)
                                    terminate("output-limit");
                            };
                            (_a = child.stdout) === null || _a === void 0 ? void 0 : _a.on("data", function (bytes) { return append(stdout, bytes); });
                            (_b = child.stderr) === null || _b === void 0 ? void 0 : _b.on("data", function (bytes) { return append(stderr, bytes); });
                            var timer = options.budgetMs > 0 ? setTimeout(function () { return terminate("timeout"); }, options.budgetMs) : undefined;
                            var cancel = setInterval(function () {
                                if (options.cancelled())
                                    terminate("cancelled");
                            }, 100);
                            child.on("error", function (error) {
                                reason = "spawn-error";
                                append(stderr, Buffer.from(error.message));
                            });
                            child.on("close", function (status, signal) {
                                if (finished)
                                    return;
                                finished = true;
                                clearTimeout(timer);
                                clearInterval(cancel);
                                (0, node_fs_1.closeSync)(stdout);
                                (0, node_fs_1.closeSync)(stderr);
                                resolveResult({ status: status, signal: signal, reason: reason, stdout: (0, node_fs_1.readFileSync)(options.stdoutPath, "utf8"), stderr: (0, node_fs_1.readFileSync)(options.stderrPath, "utf8") });
                            });
                        })];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
/** 🧪️ Compiles each explicit target once and executes only its hash-bound, exact-listed native laws. */
function runExactCargoLaws(options_1) {
    return __awaiter(this, arguments, void 0, function (options, port) {
        var configuredEnv, artifactRoot, cargoTargetDir, targetBoundary, sourceBoundary, env, nativeEnv, groupKeys, _i, _a, group, endLease, runRoot, cancelled_1, receipts, checkedBudget_1, _loop_2, _b, _c, _d, index, group;
        var _this = this;
        var _e, _f, _g, _h, _j, _k, _l, _m, _o, _p, _q, _r, _s, _t, _u, _v;
        if (port === void 0) { port = { probe: runExactCargoLawProcess, fingerprint: exactExecutableFingerprint }; }
        return __generator(this, function (_w) {
            switch (_w.label) {
                case 0:
                    configuredEnv = (_e = options.env) !== null && _e !== void 0 ? _e : process.env;
                    artifactRoot = (_f = options.artifactDir) !== null && _f !== void 0 ? _f : configuredEnv.SEMIO_TEST_ARTIFACT_DIR;
                    if (!artifactRoot || !(0, node_path_1.isAbsolute)(artifactRoot) || !(0, ____ts_3.isGeneratedPath)(artifactRoot))
                        throw new Error("Exact Cargo laws require an absolute artifactDir or SEMIO_TEST_ARTIFACT_DIR inside a generated directory");
                    cargoTargetDir = (0, ____ts_2.cargoTargetDirectory)((0, ____ts_5.getWorkspaceRoot)(), configuredEnv);
                    targetBoundary = process.platform === "win32" ? cargoTargetDir.toLowerCase() : cargoTargetDir;
                    sourceBoundary = process.platform === "win32" ? (0, node_path_1.resolve)(options.cwd).toLowerCase() : (0, node_path_1.resolve)(options.cwd);
                    if (sourceBoundary === targetBoundary || sourceBoundary.startsWith(targetBoundary + node_path_1.sep))
                        throw new Error("Cargo target must not contain the source workspace");
                    env = __assign(__assign({}, configuredEnv), { CARGO_TARGET_DIR: cargoTargetDir });
                    nativeEnv = __assign(__assign(__assign({}, env), options.nativeEnv), { CARGO_TARGET_DIR: cargoTargetDir });
                    if (!(0, node_path_1.isAbsolute)(options.cwd) || !options.groups.length || options.groups.length > 64)
                        throw new Error("Exact Cargo laws require a bounded nonempty target list and absolute cwd");
                    groupKeys = options.groups.map(function (group) { var _a; return JSON.stringify([group.package, group.target.kind, (_a = group.target.name) !== null && _a !== void 0 ? _a : ""]); });
                    if (new Set(groupKeys).size !== groupKeys.length)
                        throw new Error("Exact Cargo groups must combine laws for the same package/target");
                    for (_i = 0, _a = options.groups; _i < _a.length; _i++) {
                        group = _a[_i];
                        if (!group.package || !group.laws.length || group.laws.length > 256 || new Set(group.laws).size !== group.laws.length || group.laws.some(function (law) { return !/^[A-Za-z_][A-Za-z0-9_:]*$/u.test(law); }))
                            throw new Error("Exact Cargo law identities must be nonempty and unique");
                    }
                    (0, node_fs_1.mkdirSync)(artifactRoot, { recursive: true });
                    endLease = beginExactCargoLease(artifactRoot);
                    _w.label = 1;
                case 1:
                    _w.trys.push([1, , 6, 7]);
                    runRoot = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(artifactRoot, "exact-cargo-laws-"));
                    cancelled_1 = (_g = options.cancelled) !== null && _g !== void 0 ? _g : (function () { return false; });
                    receipts = [];
                    checkedBudget_1 = function (value, build) {
                        if (!Number.isSafeInteger(value) || value < (build ? 0 : 1) || value > 24 * 60 * 60 * 1000)
                            throw new Error("Exact Cargo budget must be finite and positive, or zero for builds");
                        return value;
                    };
                    _loop_2 = function (index, group) {
                        var groupRoot, stage, last, fail, checkpoint, capture, target, cargoArgs, built, messages, errors, artifacts, artifact, packageId, packageName, kinds, fingerprint, initial, verify, listed, discovered, laws, _x, _y, _z, lawIndex, law, result, terminals, receipt;
                        return __generator(this, function (_0) {
                            switch (_0.label) {
                                case 0:
                                    groupRoot = (0, node_path_1.join)(runRoot, String(index).padStart(2, "0"));
                                    (0, node_fs_1.mkdirSync)(groupRoot);
                                    stage = "build";
                                    last = { status: null, signal: null, stdout: "", stderr: "" };
                                    fail = function (detail) {
                                        throw new ExactCargoLawError(stage, last.status, last.signal, groupRoot, detail);
                                    };
                                    checkpoint = function () {
                                        if (cancelled_1())
                                            fail("cancelled");
                                    };
                                    capture = function (next, command, args, budget, name) { return __awaiter(_this, void 0, void 0, function () {
                                        var stdoutPath, stderrPath;
                                        var _a, _b;
                                        return __generator(this, function (_c) {
                                            switch (_c.label) {
                                                case 0:
                                                    stage = next;
                                                    checkpoint();
                                                    (_a = options.progress) === null || _a === void 0 ? void 0 : _a.call(options, __assign(__assign({ stage: stage, package: group.package }, (next === "native" ? { law: args[0] } : {})), { artifactDir: groupRoot }));
                                                    stdoutPath = (0, node_path_1.join)(groupRoot, "".concat(name, ".stdout"));
                                                    stderrPath = (0, node_path_1.join)(groupRoot, "".concat(name, ".stderr"));
                                                    return [4 /*yield*/, port.probe(command, args, { cwd: options.cwd, env: next === "build" ? env : nativeEnv, budgetMs: checkedBudget_1(budget, next === "build"), maxOutputBytes: next === "build" ? 256 * 1024 * 1024 : 8 * 1024 * 1024, stdoutPath: stdoutPath, stderrPath: stderrPath, cancelled: cancelled_1 })];
                                                case 1:
                                                    last = _c.sent();
                                                    if (!(0, node_fs_1.existsSync)(stdoutPath))
                                                        (0, node_fs_1.writeFileSync)(stdoutPath, last.stdout, { flag: "wx", mode: 384 });
                                                    if (!(0, node_fs_1.existsSync)(stderrPath))
                                                        (0, node_fs_1.writeFileSync)(stderrPath, last.stderr, { flag: "wx", mode: 384 });
                                                    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(groupRoot, "".concat(name, ".json")), JSON.stringify({ command: command, args: args, cargoTargetDir: cargoTargetDir, status: last.status, signal: last.signal, reason: (_b = last.reason) !== null && _b !== void 0 ? _b : "exit" }), { flag: "wx", mode: 384 });
                                                    checkpoint();
                                                    return [2 /*return*/, last];
                                            }
                                        });
                                    }); };
                                    target = group.target.kind === "lib" ? ["--lib"] : ["--".concat(group.target.kind), group.target.name];
                                    cargoArgs = __spreadArray(__spreadArray([], ((_h = options.cargoArgs) !== null && _h !== void 0 ? _h : []), true), ((_j = group.cargoArgs) !== null && _j !== void 0 ? _j : []), true);
                                    if (cargoArgs.some(function (arg) {
                                        return ["--", "--test", "--bin", "--lib", "-p", "--package", "--no-run", "--message-format", "--target-dir", "--manifest-path"].includes(arg) || ["--message-format=", "--target-dir=", "--manifest-path="].some(function (prefix) { return arg.startsWith(prefix); });
                                    }))
                                        fail("Cargo target/control arguments are helper-owned");
                                    return [4 /*yield*/, capture("build", "cargo", __spreadArray(__spreadArray(__spreadArray(["test", "--manifest-path", (_k = options.manifestPath) !== null && _k !== void 0 ? _k : "Cargo.toml", "-p", group.package], target, true), cargoArgs, true), ["--no-run", "--message-format=json"], false), (_l = options.buildBudgetMs) !== null && _l !== void 0 ? _l : (0, ____ts_6.buildBudgetMs)(), "build")];
                                case 1:
                                    built = _0.sent();
                                    messages = built.stdout.split("\n").flatMap(function (line) {
                                        try {
                                            return [JSON.parse(line)];
                                        }
                                        catch (_a) {
                                            return [];
                                        }
                                    });
                                    errors = messages.filter(function (message) { var _a; return message.reason === "compiler-message" && ((_a = message.message) === null || _a === void 0 ? void 0 : _a.level) === "error"; }).map(function (message) { var _a; return (_a = message.message.rendered) !== null && _a !== void 0 ? _a : message.message.message; });
                                    if (built.status !== 0 || built.signal !== null || (built.reason && built.reason !== "exit"))
                                        fail("".concat((_m = built.reason) !== null && _m !== void 0 ? _m : "exit", "; ").concat((errors.length ? errors.slice(0, 3).join("\n") : built.stderr).slice(0, 6000)));
                                    artifacts = messages.filter(function (message) { var _a; return message.reason === "compiler-artifact" && ((_a = message.profile) === null || _a === void 0 ? void 0 : _a.test) === true && typeof message.executable === "string"; });
                                    if (artifacts.length !== 1)
                                        fail("expected one Cargo executable artifact, got ".concat(artifacts.length));
                                    artifact = artifacts[0];
                                    packageId = String(artifact.package_id);
                                    packageName = packageId.includes("#") ? packageId.slice(packageId.lastIndexOf("#") + 1).split("@")[0] : packageId.split(" ")[0];
                                    kinds = (_o = artifact.target) === null || _o === void 0 ? void 0 : _o.kind;
                                    if (packageName !== group.package ||
                                        !Array.isArray(kinds) ||
                                        !kinds.some(function (kind) { return (group.target.kind === "lib" ? ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"].includes(kind) : kind === group.target.kind); }) ||
                                        (group.target.name && ((_p = artifact.target) === null || _p === void 0 ? void 0 : _p.name) !== group.target.name) ||
                                        !(0, node_path_1.isAbsolute)(artifact.executable))
                                        fail("Cargo executable package/target/path does not match the explicit group");
                                    fingerprint = function () {
                                        try {
                                            return port.fingerprint(artifact.executable);
                                        }
                                        catch (error) {
                                            return fail(String(error));
                                        }
                                    };
                                    initial = fingerprint();
                                    verify = function () {
                                        checkpoint();
                                        var current = fingerprint();
                                        if (current.path !== initial.path || current.sha256 !== initial.sha256)
                                            fail("Cargo executable changed after its build receipt");
                                    };
                                    if (!(0, node_path_1.isAbsolute)(initial.path) || !/^[0-9a-f]{64}$/u.test(initial.sha256))
                                        fail("Cargo executable fingerprint is invalid");
                                    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(groupRoot, "executable.json"), JSON.stringify(__assign({ package: group.package, target: group.target }, initial)), { flag: "wx", mode: 384 });
                                    verify();
                                    return [4 /*yield*/, capture("list", initial.path, ["--list"], (_q = options.listBudgetMs) !== null && _q !== void 0 ? _q : 60000, "list")];
                                case 2:
                                    listed = _0.sent();
                                    verify();
                                    if (listed.status !== 0 || listed.signal !== null || (listed.reason && listed.reason !== "exit"))
                                        fail("list ".concat((_r = listed.reason) !== null && _r !== void 0 ? _r : "exit", "; ").concat(listed.stderr.slice(0, 4000)));
                                    discovered = listed.stdout
                                        .split(/\r?\n/u)
                                        .filter(function (line) { return line.endsWith(": test"); })
                                        .map(function (line) { return line.slice(0, -6); });
                                    laws = group.laws.map(function (selector) {
                                        var matches = discovered.filter(function (name) { return name === selector || name.endsWith("::".concat(selector)); });
                                        if (matches.length !== 1)
                                            fail("expected exactly one ".concat(selector, ", selected=").concat(matches.length));
                                        return matches[0];
                                    });
                                    if (new Set(laws).size !== laws.length)
                                        fail("Law selectors resolve to the same native assertion");
                                    _x = 0, _y = laws.entries();
                                    _0.label = 3;
                                case 3:
                                    if (!(_x < _y.length)) return [3 /*break*/, 6];
                                    _z = _y[_x], lawIndex = _z[0], law = _z[1];
                                    verify();
                                    return [4 /*yield*/, capture("native", initial.path, [law, "--exact", "--test-threads=1", "--show-output"], (_s = options.lawBudgetMs) !== null && _s !== void 0 ? _s : 60000, "law-".concat(lawIndex))];
                                case 4:
                                    result = _0.sent();
                                    verify();
                                    terminals = __spreadArray([], result.stdout.matchAll(/^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/gm), true);
                                    if (result.status !== 0 ||
                                        result.signal !== null ||
                                        (result.reason && result.reason !== "exit") ||
                                        terminals.length !== 1 ||
                                        ((_t = terminals[0]) === null || _t === void 0 ? void 0 : _t[1]) !== "1" ||
                                        ((_u = terminals[0]) === null || _u === void 0 ? void 0 : _u[2]) !== "0" ||
                                        ((_v = terminals[0]) === null || _v === void 0 ? void 0 : _v[3]) !== "0" ||
                                        !result.stdout.split(/\r?\n/u).includes("test ".concat(law, " ... ok")))
                                        fail("native assertion ".concat(law, " did not pass exactly once; ").concat((result.stdout + result.stderr).slice(-6000)));
                                    _0.label = 5;
                                case 5:
                                    _x++;
                                    return [3 /*break*/, 3];
                                case 6:
                                    receipt = { package: group.package, target: group.target, executable: initial.path, sha256: initial.sha256, laws: laws, assertions: laws.length, artifactDir: groupRoot, cargoTargetDir: cargoTargetDir };
                                    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(groupRoot, "receipt.json"), JSON.stringify(receipt), { flag: "wx", mode: 384 });
                                    receipts.push(receipt);
                                    return [2 /*return*/];
                            }
                        });
                    };
                    _b = 0, _c = options.groups.entries();
                    _w.label = 2;
                case 2:
                    if (!(_b < _c.length)) return [3 /*break*/, 5];
                    _d = _c[_b], index = _d[0], group = _d[1];
                    return [5 /*yield**/, _loop_2(index, group)];
                case 3:
                    _w.sent();
                    _w.label = 4;
                case 4:
                    _b++;
                    return [3 /*break*/, 2];
                case 5: return [2 /*return*/, receipts];
                case 6:
                    endLease();
                    return [7 /*endfinally*/];
                case 7: return [2 /*return*/];
            }
        });
    });
}
/** 🌙️Spawns a long-lived daemon under [[daemonBudgetMs]]; [[SpawnDaemonHandle.kill]] tears down the whole process tree. */
function spawnDaemon(cmd, args, opts) {
    var _a, _b;
    if (opts === void 0) { opts = {}; }
    var child = (0, node_child_process_1.spawn)(cmd, args, {
        cwd: opts.cwd,
        env: (_a = opts.env) !== null && _a !== void 0 ? _a : process.env,
        stdio: (_b = opts.stdio) !== null && _b !== void 0 ? _b : "inherit",
        detached: process.platform !== "win32",
    });
    var killed = false;
    var budgetMs = (0, ____ts_6.daemonBudgetMs)();
    var timer = budgetMs > 0 ? setTimeout(function () {
        killed = true;
        if (child.pid)
            killBudgetTree(child.pid);
    }, budgetMs) : undefined;
    var kill = function () {
        if (timer !== undefined)
            clearTimeout(timer);
        if (!killed && child.pid)
            killBudgetTree(child.pid);
    };
    child.on("exit", function () { if (timer !== undefined)
        clearTimeout(timer); });
    return { child: child, kill: kill };
}
/** ⏳️Polls `url` with `fetch` every 500ms until it responds `ok`, throwing once `timeoutMs` elapses —
 * the shared readiness gate every `spawnDaemon`-fronted dev/static server (Storybook's own Playwright
 * pipeline, the demonstrator's) waits on before handing off to Playwright. */
function waitForHttpUrl(url, timeoutMs) {
    return __awaiter(this, void 0, void 0, function () {
        var start, response, _a;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    start = Date.now();
                    _b.label = 1;
                case 1:
                    if (!(Date.now() - start < timeoutMs)) return [3 /*break*/, 7];
                    _b.label = 2;
                case 2:
                    _b.trys.push([2, 4, , 5]);
                    return [4 /*yield*/, fetch(url)];
                case 3:
                    response = _b.sent();
                    if (response.ok)
                        return [2 /*return*/];
                    return [3 /*break*/, 5];
                case 4:
                    _a = _b.sent();
                    return [3 /*break*/, 5];
                case 5: return [4 /*yield*/, new Promise(function (resolve) { return setTimeout(resolve, 500); })];
                case 6:
                    _b.sent();
                    return [3 /*break*/, 1];
                case 7: throw new Error("Timed out waiting for ".concat(url));
            }
        });
    });
}
//#endregion ⏱️Budget
//#region 📊️Coverage
/** 📊️Whether instrumented coverage collection is on — auto-set by [[resolveTestLevel]] at the `exhaustive` level; `SEMIO_COVERAGE=0` opts out. */
function coverageEnabled() {
    return process.env.SEMIO_COVERAGE === "1";
}
/** 📊️Per-toolchain lcov output directory under `.🧬semio/🦑️repo/📊️metrics/coverage`, created on demand. */
function coverageDir(repoRoot, kind) {
    var dir = (0, node_path_1.join)(getRepoMetaDir(repoRoot), "📊️metrics", "coverage", kind);
    (0, node_fs_1.mkdirSync)(dir, { recursive: true });
    return dir;
}
/** 📊️Filesystem-safe unique slug for a project's coverage output filename (mirrors the path-slugging idiom used for nx cache keys). */
function coverageSlug(bundleRoot) {
    return bundleRoot.replace(/[^a-zA-Z0-9_-]+/g, "_");
}
/**
 * 📊️Central, authoritative exclusion list applied at repo-wide aggregation — generated code, vendored/emitted
 * assets, and paths that are GPU-only or Electron-shell and thus unmeasurable by a headless line-coverage
 * runner. Per-tool excludes (vitest `coverage.exclude`, coverage.py `omit`, …) may mirror this for speed, but
 * this list is what decides what counts toward the repo-wide percentage. Every entry has a reason in
 * [[COVERAGE_EXCLUDE_REASONS]] so exclusions stay auditable rather than a silent denominator shrink.
 */
exports.COVERAGE_EXCLUDE_GLOBS = [
    "**/generated/**",
    "framework/module/asset/metabolism/icon/generated/**",
    "**/pkg/**",
    ".storybook/**",
    "**/*.stories.*",
    "**/*.spec.ts",
    "**/*.test.ts",
    "**/*.tex",
    "**/*.wgsl",
    "**/*.svg",
    "elements/client/lib/geometry/topologic/**",
    "framework/product/os/module/renderer/wgpu/**",
    "framework/module/ui/wgpu/**",
    "**/dist/**",
    "**/node_modules/**",
    "**/target/**",
];
/** 📊️One-line rationale per [[COVERAGE_EXCLUDE_GLOBS]] entry — printed alongside the coverage summary. */
exports.COVERAGE_EXCLUDE_REASONS = {
    "**/generated/**": "Emitted lookup tables/codegen output, not hand-authored logic.",
    "framework/module/asset/metabolism/icon/generated/**": "~22k LOC generated icon table.",
    "**/pkg/**": "wasm-bindgen build output.",
    ".storybook/**": "Covered by Playwright specs, not unit line coverage.",
    "**/*.stories.*": "Storybook fixtures, exercised visually not via line coverage.",
    "**/*.spec.ts": "Playwright specs — browser-process coverage is a separate mechanism.",
    "**/*.test.ts": "Standalone test files measure themselves, not the source under test.",
    "**/*.tex": "LaTeX templates — not executable.",
    "**/*.wgsl": "GPU shader source — not measurable by CPU line coverage.",
    "**/*.svg": "Vector assets — not executable.",
    "elements/client/lib/geometry/topologic/**": "Vendored third-party C++.",
    "framework/product/os/module/renderer/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
    "framework/module/ui/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
    "**/dist/**": "Build output.",
    "**/node_modules/**": "Third-party dependency code.",
    "**/target/**": "Cargo build output.",
};
function coverageGlobToRegExp(glob) {
    var escaped = glob
        .replace(/[.+^${}()|[\]\\]/g, "\\$&")
        .replace(/\*\*/g, "\x00")
        .replace(/\*/g, "[^/]*")
        .replace(/\x00/g, ".*");
    return new RegExp("^".concat(escaped, "$"));
}
/** 📊️Whether a repo-relative path (forward-slash, no leading `./`) matches any [[COVERAGE_EXCLUDE_GLOBS]] entry. */
function isCoverageExcluded(relPath) {
    var normalized = relPath.replace(/\\/g, "/").replace(/^\.\//, "");
    return exports.COVERAGE_EXCLUDE_GLOBS.some(function (glob) { return coverageGlobToRegExp(glob).test(normalized); });
}
/** 📊️`go test` coverage args, appended alongside `goLevelTestArgs` when [[coverageEnabled]]; the text profile is converted to LCOV via [[goProfileToLcov]] at aggregation. */
function goCoverageArgs(repoRoot, moduleLabel) {
    if (!coverageEnabled())
        return [];
    var file = (0, node_path_1.join)(coverageDir(repoRoot, "go"), "".concat(coverageSlug(moduleLabel), ".cover"));
    return ["-covermode=atomic", "-coverpkg=./...", "-coverprofile=".concat(file)];
}
/** 📊️pytest coverage args, appended alongside `pytestLevelArgs` when [[coverageEnabled]]; `pytest-cov`'s `lcov` reporter writes LCOV directly, no conversion needed. */
function pytestCoverageArgs(repoRoot, moduleLabel) {
    if (!coverageEnabled())
        return [];
    var file = (0, node_path_1.join)(coverageDir(repoRoot, "py"), "".concat(coverageSlug(moduleLabel), ".lcov"));
    return ["--cov", "--cov-report=lcov:".concat(file)];
}
/** 📊️dotnet test coverage args (coverlet via `--collect`), appended when [[coverageEnabled]]; each project gets its own results-directory subfolder so concurrent projects don't clobber one another's report file. */
function dotnetCoverageArgs(repoRoot, moduleLabel) {
    if (!coverageEnabled())
        return [];
    var dir = (0, node_path_1.join)(coverageDir(repoRoot, "dotnet"), coverageSlug(moduleLabel));
    (0, node_fs_1.mkdirSync)(dir, { recursive: true });
    return ["--collect", "XPlat Code Coverage;Format=lcov", "--results-directory", dir];
}
/** 📊️Parses one LCOV text blob into per-file line-hit maps (only `SF:`/`DA:`/`end_of_record` — the subset every toolchain here emits). */
function parseLcov(text) {
    var _a;
    var records = [];
    var current;
    for (var _i = 0, _b = text.split(/\r?\n/); _i < _b.length; _i++) {
        var rawLine = _b[_i];
        var line = rawLine.trim();
        if (line.startsWith("SF:")) {
            current = { path: line.slice(3), lines: new Map() };
            records.push(current);
        }
        else if (line.startsWith("DA:") && current) {
            var _c = line.slice(3).split(","), lineNoStr = _c[0], hitsStr = _c[1];
            var lineNo = Number(lineNoStr);
            var hits = Number(hitsStr);
            current.lines.set(lineNo, ((_a = current.lines.get(lineNo)) !== null && _a !== void 0 ? _a : 0) + hits);
        }
        else if (line === "end_of_record") {
            current = undefined;
        }
    }
    return records;
}
/** 📊️Merges LCOV records from multiple toolchain runs — line-hit counts sum, covered line numbers union. */
function mergeLcov(recordSets) {
    var _a, _b;
    var merged = new Map();
    for (var _i = 0, recordSets_1 = recordSets; _i < recordSets_1.length; _i++) {
        var records = recordSets_1[_i];
        for (var _c = 0, records_1 = records; _c < records_1.length; _c++) {
            var record = records_1[_c];
            var target = (_a = merged.get(record.path)) !== null && _a !== void 0 ? _a : new Map();
            for (var _d = 0, _e = record.lines; _d < _e.length; _d++) {
                var _f = _e[_d], lineNo = _f[0], hits = _f[1];
                target.set(lineNo, ((_b = target.get(lineNo)) !== null && _b !== void 0 ? _b : 0) + hits);
            }
            merged.set(record.path, target);
        }
    }
    return merged;
}
/** 📊️Renders a merged coverage map back to LCOV text (one `SF:`/`DA:`×N/`end_of_record` block per file, lines sorted ascending). */
function renderLcov(merged) {
    var chunks = [];
    for (var _i = 0, merged_1 = merged; _i < merged_1.length; _i++) {
        var _a = merged_1[_i], path = _a[0], lines = _a[1];
        chunks.push("SF:".concat(path));
        for (var _b = 0, _c = __spreadArray([], lines.keys(), true).sort(function (a, b) { return a - b; }); _b < _c.length; _b++) {
            var lineNo = _c[_b];
            chunks.push("DA:".concat(lineNo, ",").concat(lines.get(lineNo)));
        }
        chunks.push("end_of_record", "");
    }
    return chunks.join("\n");
}
/** 📊️Expands a `go test -coverprofile` text block (`mode: <mode>` header, then `file:startLine.startCol,endLine.endCol numStmt count` records) into per-line LCOV records — every line in a statement's range is marked hit iff `count > 0`. */
function goProfileToLcov(text) {
    var _a, _b;
    var byFile = new Map();
    for (var _i = 0, _c = text.split(/\r?\n/); _i < _c.length; _i++) {
        var rawLine = _c[_i];
        var line = rawLine.trim();
        if (!line || line.startsWith("mode:"))
            continue;
        var match = /^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$/.exec(line);
        if (!match)
            continue;
        var file = match[1], startStr = match[2], endStr = match[3], countStr = match[4];
        var start = Number(startStr);
        var end = Number(endStr);
        var hits = Number(countStr);
        var lines = (_a = byFile.get(file)) !== null && _a !== void 0 ? _a : new Map();
        for (var lineNo = start; lineNo <= end; lineNo++)
            lines.set(lineNo, ((_b = lines.get(lineNo)) !== null && _b !== void 0 ? _b : 0) + hits);
        byFile.set(file, lines);
    }
    return __spreadArray([], byFile.entries(), true).map(function (_a) {
        var path = _a[0], lines = _a[1];
        return ({ path: path, lines: lines });
    });
}
/** 📊️Reduces a merged coverage map to a repo-wide percentage plus a worst-offenders table, after dropping [[isCoverageExcluded]] paths. */
function summarizeCoverage(merged) {
    var linesFound = 0;
    var linesHit = 0;
    var perFile = [];
    for (var _i = 0, merged_2 = merged; _i < merged_2.length; _i++) {
        var _a = merged_2[_i], path = _a[0], lines = _a[1];
        if (isCoverageExcluded(path))
            continue;
        var found = lines.size;
        var hit = __spreadArray([], lines.values(), true).filter(function (count) { return count > 0; }).length;
        linesFound += found;
        linesHit += hit;
        perFile.push({ path: path, linesFound: found, linesHit: hit, pct: found === 0 ? 100 : (hit / found) * 100 });
    }
    perFile.sort(function (a, b) { return a.pct - b.pct || b.linesFound - a.linesFound; });
    return { linesFound: linesFound, linesHit: linesHit, pct: linesFound === 0 ? 0 : (linesHit / linesFound) * 100, perFile: perFile };
}
/** 📊️Prints the worst-covered files and hard-fails (`process.exit(1)`) below `thresholdPct`. */
function enforceCoverageThreshold(summary, thresholdPct, worstCount) {
    if (worstCount === void 0) { worstCount = 25; }
    console.log("[coverage] ".concat(summary.linesHit, "/").concat(summary.linesFound, " lines (").concat(summary.pct.toFixed(2), "%), threshold ").concat(thresholdPct, "%."));
    if (summary.pct < thresholdPct) {
        console.log("[coverage] worst ".concat(Math.min(worstCount, summary.perFile.length), " files:"));
        for (var _i = 0, _a = summary.perFile.slice(0, worstCount); _i < _a.length; _i++) {
            var file = _a[_i];
            console.log("  ".concat(file.pct.toFixed(1), "% (").concat(file.linesHit, "/").concat(file.linesFound, ")  ").concat(file.path));
        }
        console.error("[coverage] ".concat(summary.pct.toFixed(2), "% < ").concat(thresholdPct, "% \u2014 exhaustive gate failed."));
        process.exit(1);
    }
}
//#endregion 🗄️Lcov
//#endregion 📊️Coverage
//#region 🧹️CargoLint
/**
 * 🧹️Zero-warning gate for a crate: `cargo clippy -p <pkg> --all-targets -- -D warnings`.
 * Deny-on-warnings lives ONLY in this trailing clippy arg — never in RUSTFLAGS, which
 * would replace (not merge with) `.cargo/config.toml`'s rustflags (`-Z threads=8`, the
 * wasm32 `getrandom_backend` cfg, mold) and break every wasm build. See
 * `[workspace.lints]` in the root `Cargo.toml` for the shared lint baseline this checks.
 */
function runCargoLint(packages, cwd, extraArgs, env) {
    if (extraArgs === void 0) { extraArgs = []; }
    if (env === void 0) { env = process.env; }
    var resolvedPackages = resolveCargoPackageNames(packages, cwd);
    var packageArgs = resolvedPackages.flatMap(function (pkg) { return ["-p", pkg]; });
    (0, ____ts_6.runCmd)("cargo", __spreadArray(__spreadArray(__spreadArray(__spreadArray(["clippy"], packageArgs, true), ["--all-targets"], false), extraArgs, true), ["--", "-D", "warnings"], false), { cwd: cwd, env: env, budgetMs: (0, ____ts_6.buildBudgetMs)() });
}
//#endregion 🧹️CargoLint
//#region ⚙️ViteConfigLoader
/**
 * @emoji ⚙️ Force Vite's esbuild config loader (`--configLoader bundle`).
 * Node 24+ defaults Vite to `native` strip-only TypeScript, which rejects constructor parameter
 * properties used across monorepo configs pulled into `⚙️vite.config.ts` (e.g. via `@semio-tech/framework`).
 * @see https://vite.dev/config/#config-loader
 */
function withViteConfigLoader(args) {
    if (args.includes("--configLoader"))
        return __spreadArray([], args, true);
    var loader = typeof globalThis.Bun === "object" ? "native" : "bundle";
    var out = __spreadArray([], args, true);
    var viteIdx = out.indexOf("vite");
    if (viteIdx >= 0) {
        out.splice(viteIdx + 1, 0, "--configLoader", loader);
        return out;
    }
    return __spreadArray(["--configLoader", loader], out, true);
}
function bunArgsForVite(args) {
    return args.includes("vite") ? withViteConfigLoader(args) : __spreadArray([], args, true);
}
//#endregion ⚙️ViteConfigLoader
function bunxCmdArgs(args, cwd) {
    var formatted = bunArgsForVite(args);
    var viteIdx = formatted.indexOf("vite");
    if (viteIdx >= 0) {
        var resolved = (0, ____ts_6.resolveWorkspaceBin)("vite", cwd);
        if (resolved) {
            var copy = __spreadArray([], formatted, true);
            copy[viteIdx] = resolved;
            return copy;
        }
    }
    var binName = formatted[0];
    if (binName && binName !== "x") {
        var resolved = (0, ____ts_6.resolveWorkspaceBin)(binName, cwd);
        if (resolved) {
            return __spreadArray([resolved], formatted.slice(1), true);
        }
    }
    return __spreadArray(["x"], formatted, true);
}
/** 🥖️Runs `bun` with inherited stdio in `cwd`. */
function runBun(args, cwd, env) {
    if (env === void 0) { env = process.env; }
    (0, ____ts_6.runCmd)(process.execPath, bunArgsForVite(args), { cwd: cwd, env: env });
}
/** 🥖️Runs `bunx` synchronously in `cwd`, returning status code. */
function runBunxStatus(args, cwd, env) {
    var _a;
    if (env === void 0) { env = process.env; }
    var result = (0, node_child_process_1.spawnSync)(process.execPath, bunxCmdArgs(args, cwd), { cwd: cwd, env: env, shell: false, stdio: "inherit" });
    if (result.error) {
        console.error(result.error);
        return 1;
    }
    return (_a = result.status) !== null && _a !== void 0 ? _a : 1;
}
/** 🥖️Runs `bunx` synchronously in `cwd`. */
function runBunx(args, cwd, env) {
    if (env === void 0) { env = process.env; }
    var status = runBunxStatus(args, cwd, env);
    if (status !== 0)
        process.exit(status);
}
/** 🥖️Spawns `bunx` asynchronously; exits with child code. */
function spawnBunx(args, cwd, env) {
    if (env === void 0) { env = process.env; }
    return new Promise(function (resolve, reject) {
        var child = (0, node_child_process_1.spawn)(process.execPath, bunxCmdArgs(args, cwd), { cwd: cwd, env: env, shell: false, stdio: "inherit" });
        child.on("exit", function (code) {
            if ((code !== null && code !== void 0 ? code : 0) === 0)
                resolve();
            else
                process.exit(code !== null && code !== void 0 ? code : 0);
        });
        child.on("error", function (error) {
            console.error(error);
            reject(error);
            process.exit(1);
        });
    });
}
/** 🥖️Spawns `bun` asynchronously; exits with child code. */
function spawnBun(args, cwd, env) {
    if (env === void 0) { env = process.env; }
    var child = (0, node_child_process_1.spawn)(process.execPath, bunArgsForVite(args), { cwd: cwd, env: env, shell: true, stdio: "inherit" });
    child.on("exit", function (code) { return process.exit(code !== null && code !== void 0 ? code : 0); });
    child.on("error", function (error) {
        console.error(error);
        process.exit(1);
    });
}
/** ▶️Vite dev server with polling-friendly env defaults. */
function runViteDev(bundleRoot, segments, opts) {
    var _a, _b, _c;
    var env = playPollingEnv();
    var host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    var port = (_c = (_b = process.env[(_a = opts.portEnv) !== null && _a !== void 0 ? _a : "VITE_PORT"]) !== null && _b !== void 0 ? _b : opts.defaultPort) !== null && _c !== void 0 ? _c : "5173";
    spawnBun(__spreadArray(["run", "vite", "--config", opts.config, "--host", host, "--port", port], segments, true), bundleRoot, env);
}
/** ▶️Vite production build. */
function runViteBuild(bundleRoot, segments, config) {
    var workspace = (0, ____ts_7.findRepoRoot)(bundleRoot);
    var cli = (0, node_path_1.join)((0, node_path_1.dirname)((0, node_module_1.createRequire)((0, node_path_1.join)(workspace, "package.json")).resolve("vite/package.json")), "bin/vite.js");
    (0, ____ts_6.runCmd)(process.execPath, __spreadArray([cli, "build", "--config", config, "--configLoader", "bundle"], segments, true), { cwd: bundleRoot, env: (0, ____ts_1.devToolingEnv)() });
}
/**
 * ▶️Vitest run in bundle directory, under the [[runTestBudgeted]] wall-clock budget. Appends v8 lcov coverage
 * flags when [[coverageEnabled]]. Invokes the workspace's own `node_modules/vitest/vitest.mjs` directly rather
 * than `bun x vitest` — `bunx` resolves its own globally cached vitest version, which can silently drift from
 * the workspace's pinned version (observed: a cached 3.x core paired with a locally installed 4.x
 * `@vitest/coverage-v8` crashes the coverage provider on an undefined `reportsDirectory`). Coverage runs use
 * plain `node`, not bun, as the runtime — `@vitest/coverage-v8` drives V8 coverage via `node:inspector`'s
 * `Profiler.startPreciseCoverage`, which Bun's `node:inspector` shim doesn't implement (observed: "Coverage
 * APIs are not supported"); non-coverage runs keep using bun for its faster startup.
 */
/** 🧪️ Builds Vitest argv without overriding the owning config's no-test policy.
 *
 * 🩸️ `--config` is passed ABSOLUTE. Every call site writes the path relative to its own bundle root
 * (the shape `runVitestConfigArgumentTokens` scans for), but vitest 4 resolves a relative `--config`
 * against the root it detects — this repo's own root — not the working directory it was launched in.
 * `@semio-tech/framework:test` was the caught case: its `../../🧪️tests/🎚️config/🟦️.ts` resolved to
 * `<repo parent>/🧪️tests/🎚️config/🟦️.ts`, esbuild could not load it, and the run reported
 * "No test files found, exiting with code 1" — with the config silently absent, so its `includeSource`
 * never collected the kernel's in-source suites. Resolving here keeps the relative literals at the call
 * sites and makes the launched cwd authoritative again. */
function vitestRunArguments(bundleRoot, segments, config, collectingCoverage) {
    if (collectingCoverage === void 0) { collectingCoverage = coverageEnabled(); }
    var coverageArgs = collectingCoverage ? ["--coverage.enabled", "--coverage.provider=v8", "--coverage.reporter=lcovonly", "--coverage.reportsDirectory=".concat((0, node_path_1.join)(coverageDir((0, ____ts_7.findRepoRoot)(bundleRoot), "js"), coverageSlug(bundleRoot)))] : [];
    var vitestBin = (0, node_path_1.join)((0, ____ts_7.findRepoRoot)(bundleRoot), "node_modules", "vitest", "vitest.mjs");
    return __spreadArray(__spreadArray(__spreadArray([vitestBin, "run", "--config", (0, node_path_1.isAbsolute)(config) ? config : (0, node_path_1.resolve)(bundleRoot, config)], vitestLevelArgs(), true), coverageArgs, true), segments, true);
}
function runVitest(bundleRoot, segments, config) {
    return __awaiter(this, void 0, void 0, function () {
        var collectingCoverage, runtime;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    collectingCoverage = coverageEnabled();
                    runtime = collectingCoverage ? "node" : process.execPath;
                    return [4 /*yield*/, runTestBudgeted(runtime, vitestRunArguments(bundleRoot, segments, config, collectingCoverage), { cwd: bundleRoot, env: (0, ____ts_1.devToolingEnv)() })];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
var ____ts_8 = require("./\uD83C\uDFAE\uFE0Fplayground/\uD83D\uDD12\uFE0Fpreferences/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "SEMIO_LOCKED_LOCALE_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_LOCKED_LOCALE_ENV; } });
Object.defineProperty(exports, "SEMIO_LOCKED_TERMINOLOGY_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_LOCKED_TERMINOLOGY_ENV; } });
Object.defineProperty(exports, "SEMIO_LOCKED_THEME_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_LOCKED_THEME_ENV; } });
Object.defineProperty(exports, "SEMIO_LOCKED_APPEARANCE_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_LOCKED_APPEARANCE_ENV; } });
Object.defineProperty(exports, "SEMIO_BRAND_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_BRAND_ENV; } });
Object.defineProperty(exports, "SEMIO_DEFAULT_EXAMPLE_ENV", { enumerable: true, get: function () { return ____ts_8.SEMIO_DEFAULT_EXAMPLE_ENV; } });
Object.defineProperty(exports, "frameworkOsLockedPrefsEnv", { enumerable: true, get: function () { return ____ts_8.frameworkOsLockedPrefsEnv; } });
//#region 🖥️FrameworkOsPlaygroundDev
/** @emoji 🔌️ Local dev-time asset server port for wgpu Trunk/native playgrounds (Trunk forwards
 * route-scoped requests — e.g. `/osm`, `/vt`, `/dem` — here; driven by each playground's declared
 * `[[package.metadata.semio.assets]]` rows, not any one app's routes). */
exports.SEMIO_ASSET_SERVER_PORT = 6141;
/** @emoji 🔌️ Process env for the absolute asset server URL base (native-bin wgpu route-relative fetches). */
exports.SEMIO_ASSET_BASE_URL_ENV = "SEMIO_ASSET_BASE_URL";
/** @emoji 🔌️ Resolves the default dev port for a given catalog variant and renderer. */
function frameworkOsPlaygroundDefaultPort(catalog, variant, renderer) {
    var row = catalog.find(function (r) { return r.variant === variant; });
    if (!row)
        return 6066;
    return renderer === "wgpu" ? row.ports.wgpu : row.ports.react;
}
/** @emoji 🎯️ Resolves `bun ./📜️script.ts dev …` segments to a framework OS plugin filter via the catalog. */
function resolveFrameworkOsPlaygroundPlugin(catalog, segments) {
    if (segments.length === 0)
        return null;
    var _loop_3 = function (len) {
        var alias = segments.slice(0, len).join(" ");
        var row = catalog.find(function (r) { return r.variant === alias || r.aliases.includes(alias); });
        if (row) {
            return { value: { plugin: row.variant, rest: segments.slice(len) } };
        }
    };
    for (var len = segments.length; len >= 1; len--) {
        var state_1 = _loop_3(len);
        if (typeof state_1 === "object")
            return state_1.value;
    }
    return null;
}
/** @emoji 🧊️ Env for `@semio-tech/framework-os-dev:dev` with a plugin filter and the renderer an
 * explicit `SEMIO_RENDERER` (launch row, `extra`) selects — wgpu only as the unset default. The value
 * picks the `dev-<variant>-<renderer>-<profile>` target in `resolveNxInvocation`, so a react launch
 * row reaches Vite and never the wgpu browser server. */
function frameworkOsPlaygroundDevEnv(catalog, plugin, extra, env) {
    var _a, _b;
    if (extra === void 0) { extra = {}; }
    if (env === void 0) { env = process.env; }
    var renderer = (_b = (_a = extra.SEMIO_RENDERER) !== null && _a !== void 0 ? _a : env.SEMIO_RENDERER) !== null && _b !== void 0 ? _b : "wgpu";
    var defaultPort = frameworkOsPlaygroundDefaultPort(catalog, plugin, renderer);
    var portVal = extra.S_OS_PORT || env.S_OS_PORT || String(defaultPort);
    return (0, ____ts_1.devToolingEnv)(__assign(__assign({ SEMIO_PLUGIN: plugin }, extra), { SEMIO_RENDERER: renderer, S_OS_PORT: portVal }));
}
//#endregion 🖥️FrameworkOsPlaygroundDev
/** 🧰️Play/vite dev env with optional file-watcher polling defaults. Polling only makes sense where
 * native filesystem events don't reach the watcher (bind-mounted devcontainers) — on native macOS/
 * Windows/Linux it just burns CPU/RSS across every spawned dev server for no benefit. An explicit
 * `WATCHPACK_POLLING` always wins as a manual override in either direction. */
function playPollingEnv(extra) {
    if (extra === void 0) { extra = {}; }
    var pollingDefault = process.env.WATCHPACK_POLLING !== undefined ? {} : process.env.DEVCONTAINER === "true" ? { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" } : {};
    return (0, ____ts_1.devToolingEnv)(__assign(__assign({}, pollingDefault), extra));
}
/** @emoji 🔒️ Parses optional `example <id>` argv prefix for playground play scripts. */
function consumePlaygroundExampleArgv(segments, resolveExampleId) {
    if (segments[0] !== "example" || !segments[1]) {
        return { segments: segments, exampleEnv: {} };
    }
    var exampleId = resolveExampleId(segments[1]);
    if (!exampleId) {
        console.error("[play] unknown example ".concat(JSON.stringify(segments[1])));
        process.exit(1);
    }
    return {
        segments: segments.slice(2),
        exampleEnv: { PLAYGROUND_LOCKED_EXAMPLE_ID: exampleId },
    };
}
/** ▶️Playwright test run in bundle directory; browsers land in the shared cache root. */
function runPlaywright(bundleRoot, config, segments) {
    if (segments === void 0) { segments = []; }
    runBunx(__spreadArray(["playwright", "test", "--config", config], segments, true), bundleRoot, (0, ____ts_1.repoToolCacheEnv)((0, ____ts_7.findRepoRoot)(bundleRoot), playPollingEnv()));
}
/** @emoji 🔌️ True when host:port already accepts TCP (existing dev server). */
function isDevPortInUse(host, port) {
    var probe = "\nimport { createConnection } from \"node:net\";\nconst socket = createConnection({ host: ".concat(JSON.stringify(host), ", port: ").concat(port, " });\nsocket.setTimeout(300);\nsocket.once(\"connect\", () => process.exit(0));\nsocket.once(\"timeout\", () => process.exit(1));\nsocket.once(\"error\", () => process.exit(1));\n");
    var result = (0, node_child_process_1.spawnSync)(process.execPath, ["--input-type=module", "-e", probe], { timeout: 500 });
    return result.status === 0;
}
/** @emoji 🌐️ Loopback URL for a dev server bound to `host`/`port`. */
function devServerUrl(host, port) {
    var probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
    return "http://".concat(probeHost, ":").concat(port, "/");
}
/** @emoji 🧊️ Legacy trunk entry paths still seen on long-running dev servers. */
exports.WGPU_DEV_LEGACY_ENTRY_PATH = "/renderer-modules/wgpu/";
/** @emoji 🧊️ Play URL for a wgpu trunk entry path and plugin filter. */
function wgpuDevPlayUrl(host, port, plugin, entryPath) {
    if (entryPath === void 0) { entryPath = "/"; }
    var probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
    var base = entryPath.endsWith("/") ? entryPath : "".concat(entryPath, "/");
    return "http://".concat(probeHost, ":").concat(port).concat(base, "?plugin=").concat(encodeURIComponent(plugin));
}
/** @emoji 🧊️ Probes which wgpu trunk entry path responds on `port`, if any. */
function probeWgpuDevPort(host, port) {
    var probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
    for (var _i = 0, _a = ["/", exports.WGPU_DEV_LEGACY_ENTRY_PATH]; _i < _a.length; _i++) {
        var entryPath = _a[_i];
        var url = "http://".concat(probeHost, ":").concat(port).concat(entryPath);
        var probe = "const res = await fetch(".concat(JSON.stringify(url), ", { signal: AbortSignal.timeout(2000) });\nprocess.exit(res.ok ? 0 : 1);");
        var result = (0, node_child_process_1.spawnSync)(process.execPath, ["--input-type=module", "-e", probe], { timeout: 3000 });
        if (result.status === 0)
            return { entryPath: entryPath };
    }
    return null;
}
/** @emoji 🛑️ Stops a trunk listener on `port` when it is the sole occupant. */
function stopTrunkDevPort(port) {
    var _a;
    var occupant = describeDevPortOccupant(port);
    if (!(occupant === null || occupant === void 0 ? void 0 : occupant.startsWith("trunk")))
        return false;
    var pid = Number((_a = occupant.match(/PID (\d+)/)) === null || _a === void 0 ? void 0 : _a[1]);
    if (!Number.isFinite(pid))
        return false;
    try {
        process.kill(pid, "SIGTERM");
        return true;
    }
    catch (_b) {
        return false;
    }
}
/** @emoji 🎯️ Reads `import.meta.env.PLAYGROUND_APP_KIND` baked into a running playground dev server. */
function devServerPlayEntry(host, port) {
    var _a;
    var url = "".concat(devServerUrl(host, port), "index.ts");
    var probe = "const res = await fetch(".concat(JSON.stringify(url), ", { signal: AbortSignal.timeout(2000) });\nif (!res.ok) process.exit(1);\nconst text = await res.text();\nconst match = text.match(/PLAYGROUND_APP_KIND\\\":\\s*\\\"([^\\\"]+)\\\"/);\nprocess.stdout.write(match?.[1] ?? \"\");\n");
    var result = (0, node_child_process_1.spawnSync)(process.execPath, ["--input-type=module", "-e", probe], { encoding: "utf8", timeout: 3000 });
    var entry = (_a = result.stdout) === null || _a === void 0 ? void 0 : _a.trim();
    return entry || undefined;
}
/** @emoji 🔎️ Best-effort description of the process listening on `port` (Unix/macOS/Linux). */
function describeDevPortOccupant(port) {
    var _a;
    if (process.platform === "win32")
        return undefined;
    var result = (0, node_child_process_1.spawnSync)("lsof", ["-nP", "-iTCP:".concat(port), "-sTCP:LISTEN"], { encoding: "utf8" });
    var line = (_a = result.stdout) === null || _a === void 0 ? void 0 : _a.trim().split("\n").find(function (row, index) { return index > 0 && row.trim().length > 0; });
    if (!line)
        return undefined;
    var parts = line.trim().split(/\s+/);
    return parts.length >= 2 ? "".concat(parts[0], " (PID ").concat(parts[1], ")") : line.trim();
}
/** @emoji ♻️ True when an HTTP server on `port` already responds successfully (reuse existing dev). */
function canReuseDevPort(host, port, expectedPlayEntry) {
    var url = devServerUrl(host, port);
    var probe = "const res = await fetch(".concat(JSON.stringify(url), ", { signal: AbortSignal.timeout(2000) });\nprocess.exit(res.ok ? 0 : 1);");
    var result = (0, node_child_process_1.spawnSync)(process.execPath, ["--input-type=module", "-e", probe], { timeout: 3000 });
    if (result.status !== 0)
        return false;
    if (!expectedPlayEntry)
        return true;
    return devServerPlayEntry(host, port) === expectedPlayEntry;
}
/** @emoji 🔌️ First free TCP port at or after `preferredPort` (up to `maxAttempts`), skipping `skipPorts`. */
function resolveDevPort(host, preferredPort, maxAttempts, skipPorts) {
    if (maxAttempts === void 0) { maxAttempts = 20; }
    if (skipPorts === void 0) { skipPorts = new Set(); }
    for (var offset = 0; offset < maxAttempts; offset++) {
        var port = preferredPort + offset;
        if (skipPorts.has(port))
            continue;
        if (!isDevPortInUse(host, port))
            return port;
    }
    console.error("[dev] No free port found in range ".concat(preferredPort, "-").concat(preferredPort + maxAttempts - 1, "."));
    process.exit(1);
}
/** ▶️Vite dev via `bunx` with an explicitly owned configuration source. */
function runViteBunxDev(bundleRoot, segments, opts) {
    var _a, _b, _c, _d, _e, _f;
    var host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    var preferredPort = Number((_c = (_b = process.env[(_a = opts.portEnv) !== null && _a !== void 0 ? _a : "VITE_PORT"]) !== null && _b !== void 0 ? _b : opts.defaultPort) !== null && _c !== void 0 ? _c : "5173");
    var spawnEnv = playPollingEnv(opts.env);
    if (opts.fixedPort && isDevPortInUse(host, preferredPort)) {
        var url = devServerUrl(host, preferredPort);
        if (canReuseDevPort(host, preferredPort, opts.expectedPlayEntry)) {
            console.log("[dev] Port ".concat(preferredPort, " is already in use \u2014 dev server appears to be running at ").concat(url));
            return Promise.resolve();
        }
        var occupant = describeDevPortOccupant(preferredPort);
        var servedEntry = devServerPlayEntry(host, preferredPort);
        if (servedEntry && opts.expectedPlayEntry && servedEntry !== opts.expectedPlayEntry) {
            console.error("[dev] Port ".concat(preferredPort, " is serving play entry \"").concat(servedEntry, "\" but \"").concat(opts.expectedPlayEntry, "\" was requested. Stop that process or set ").concat((_d = opts.portEnv) !== null && _d !== void 0 ? _d : "VITE_PORT", "."));
            process.exit(1);
        }
        console.error("[dev] Port ".concat(preferredPort, " is already in use").concat(occupant ? " by ".concat(occupant) : "", ". Stop that process or set ").concat((_e = opts.portEnv) !== null && _e !== void 0 ? _e : "VITE_PORT", "."));
        process.exit(1);
    }
    var port = (function () {
        var _a;
        if (opts.fixedPort) {
            return preferredPort;
        }
        var skip = (_a = opts.reservedPorts) !== null && _a !== void 0 ? _a : new Set();
        var resolved = resolveDevPort(host, preferredPort, 20, skip);
        if (resolved !== preferredPort) {
            console.warn("[dev] Port ".concat(preferredPort, " is already in use \u2014 starting on ").concat(resolved, " instead."));
            if (opts.portEnv)
                process.env[opts.portEnv] = String(resolved);
        }
        return resolved;
    })();
    if (opts.clearViteCache) {
        var viteCache = (0, node_path_1.join)(bundleRoot, "node_modules", ".vite");
        if ((0, node_fs_1.existsSync)(viteCache))
            (0, node_fs_1.rmSync)(viteCache, { recursive: true, force: true });
    }
    var wantStrictPort = (_f = opts.strictPort) !== null && _f !== void 0 ? _f : true;
    var viteArgs = ["vite", "--config", opts.config, "--host", host, "--port", String(port)];
    if (wantStrictPort && !segments.includes("--strictPort") && !segments.includes("--no-strictPort")) {
        viteArgs.push("--strictPort");
    }
    return spawnBunx(__spreadArray(__spreadArray([], viteArgs, true), segments, true), bundleRoot, spawnEnv);
}
/** ▶️Vite dev via `bunx` without a fixed config path (extra args only). */
function runViteBunxDevPlain(bundleRoot, segments) {
    var host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    spawnBunx(__spreadArray(["vite", "--host", host], segments, true), bundleRoot, playPollingEnv());
}
/** 🦀️Runs `cargo` with inherited stdio. */
function runCargo(args, cwd, env) {
    if (env === void 0) { env = process.env; }
    (0, ____ts_6.runCmd)("cargo", args, { cwd: cwd, env: env });
}
/** 📦️ Resolves an explicitly provisioned binding generator on every supported host. */
function resolveWasmBindgenBin(repoRoot, env) {
    if (repoRoot === void 0) { repoRoot = (0, ____ts_5.getWorkspaceRoot)(); }
    if (env === void 0) { env = process.env; }
    var version = (0, ____ts_2.wasmBindgenVersion)((0, node_fs_1.readFileSync)((0, node_path_1.join)(repoRoot, "Cargo.lock"), "utf8"));
    if (env.WASM_BINDGEN_VERSION && env.WASM_BINDGEN_VERSION !== version)
        throw new Error("WASM_BINDGEN_VERSION disagrees with Cargo.lock");
    var runtime = globalThis.Bun;
    var which = typeof runtime === "object" && runtime !== null ? runtime.which : undefined;
    if (!env.SEMIO_WASM_BINDGEN_BIN && typeof which !== "function")
        throw new Error("wasm-bindgen resolution requires Bun.which.");
    var command = env.SEMIO_WASM_BINDGEN_BIN
        ? (0, node_path_1.resolve)(repoRoot, env.SEMIO_WASM_BINDGEN_BIN)
        : which("wasm-bindgen", { PATH: env.PATH });
    if (!command)
        throw new Error("Run bun nx run workspace:deps-wasm to provision wasm-bindgen");
    var probe = runProbe(command, ["--version"], { env: env });
    if (probe.status !== 0 || probe.stdout.trim() !== "wasm-bindgen ".concat(version))
        throw new Error("wasm-bindgen ".concat(version, " is required; run bun nx run workspace:deps-wasm"));
    return command;
}
/** 📦️Collect wasm-bindgen snippet paths produced by threaded builds. */
function wasmPackSnippetFiles(pkgDir) {
    var snippetsDir = (0, node_path_1.join)(pkgDir, "snippets");
    if (!(0, node_fs_1.existsSync)(snippetsDir))
        return [];
    var out = [];
    var walk = function (dir, prefix) {
        for (var _i = 0, _a = (0, node_fs_1.readdirSync)(dir); _i < _a.length; _i++) {
            var entry = _a[_i];
            var rel = prefix ? "".concat(prefix, "/").concat(entry) : entry;
            var abs = (0, node_path_1.join)(dir, entry);
            if ((0, node_fs_1.statSync)(abs).isDirectory()) {
                walk(abs, rel);
            }
            else {
                out.push("snippets/".concat(rel));
            }
        }
    };
    walk(snippetsDir, "");
    return out;
}
/** 🏗️ Shares the browser compiler environment across crates; Cargo's own config governs where it writes, no private default. */
function wasmBuildEnvironment(repoRoot, env) {
    if (env === void 0) { env = process.env; }
    return __assign({}, env);
}
/** 🧭️ Makes wasm-pack resolve the selected optimizer while preserving the pinned binding generator. */
function wasmPackEnvironment(repoRoot, bindgen, env) {
    var optimizer = env.SEMIO_WASM_OPT_BIN ? (0, node_path_1.resolve)(repoRoot, env.SEMIO_WASM_OPT_BIN) : (0, ___script_ts_1.preparedBinaryen)(repoRoot);
    if (optimizer && (0, node_path_1.basename)(optimizer) !== (process.platform === "win32" ? "wasm-opt.exe" : "wasm-opt"))
        throw new Error("SEMIO_WASM_OPT_BIN must name a wasm-opt executable");
    var environment = __assign(__assign({}, env), { PATH: __spreadArray(__spreadArray([(0, node_path_1.dirname)(bindgen)], (optimizer ? [(0, node_path_1.dirname)(optimizer)] : []), true), [env.PATH], false).filter(Boolean).join(process.platform === "win32" ? ";" : ":") });
    if (optimizer) {
        var selected = Bun.which("wasm-opt", { PATH: environment.PATH });
        if (!selected || (0, node_fs_1.realpathSync)(selected) !== (0, node_fs_1.realpathSync)(optimizer))
            throw new Error("Selected wasm-opt is missing or shadowed by another tool");
    }
    return environment;
}
/** 📂️ Validates one portable compiler output owner without executing a compiler. */
function wasmOutputDirectory(rsDir, outputDirectory) {
    if (!outputDirectory || /[/\\:*?"<>|\u0000]|[. ]$/u.test(outputDirectory))
        throw new Error("WASM outputDirectory must be one portable literal directory name");
    return (0, node_path_1.join)(rsDir, outputDirectory);
}
/** 🎚️ Keeps wasm-pack profile flags separate from Cargo's explicit profile selection. */
function wasmBuildArguments(profile) {
    return { pack: profile === "release" ? ["--release"] : profile === "dev" ? ["--dev"] : ["--profile", profile], cargo: ["--profile", profile] };
}
/** 📦️`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
function runWasmPackWebBuild(opts) {
    var _a, _b, _c;
    var rsDir = opts.rsDir, logPrefix = opts.logPrefix, pkg = opts.pkg, wasmBaseName = opts.wasmBaseName, _d = opts.outputDirectory, outputDirectory = _d === void 0 ? "pkg" : _d, _e = opts.threads, threads = _e === void 0 ? false : _e, _f = opts.cargoFeatures, cargoFeatures = _f === void 0 ? [] : _f, _g = opts.noDefaultFeatures, noDefaultFeatures = _g === void 0 ? false : _g, _h = opts.shipProfile, shipProfile = _h === void 0 ? "release" : _h;
    var captureRoot = process.env.SEMIO_TEST_ARTIFACT_DIR ? (0, node_path_1.resolve)(process.env.SEMIO_TEST_ARTIFACT_DIR) : (0, node_path_1.join)(rsDir, "dist");
    (0, node_fs_1.mkdirSync)(captureRoot, { recursive: true });
    var cargoOutput = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)(captureRoot, "wasm-cargo-"));
    try {
        var pkgDir = wasmOutputDirectory(rsDir, outputDirectory);
        var profile = (0, ____ts_6.semioBuildMode)() === "ship" ? shipProfile : "dev";
        var wasmPath = (0, node_path_1.join)(pkgDir, "".concat(wasmBaseName, "_bg.wasm"));
        var _j = wasmBuildArguments(profile), packProfileArgs = _j.pack, cargoProfileArgs = _j.cargo;
        var profileOutDir = (0, ____ts_6.cargoProfileDir)(profile);
        var compilerEnv = __assign(__assign({}, wasmBuildEnvironment((0, ____ts_5.getWorkspaceRoot)())), { CARGO_TARGET_DIR: cargoOutput });
        var bindgen = resolveWasmBindgenBin((0, ____ts_5.getWorkspaceRoot)(), compilerEnv);
        var buildEnv = wasmPackEnvironment((0, ____ts_5.getWorkspaceRoot)(), bindgen, compilerEnv);
        var buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
        console.log("[".concat(logPrefix, "] ").concat(buildLabel, " ").concat(packProfileArgs.join(" "), " --target web --out-dir ").concat(outputDirectory, " --out-name ").concat(wasmBaseName, " --no-pack"));
        var t0 = Date.now();
        var featureArgs = __spreadArray(__spreadArray([], (noDefaultFeatures ? ["--no-default-features"] : []), true), cargoFeatures.flatMap(function (feature) { return ["--features", feature]; }), true);
        var status_1;
        if (threads) {
            var repoRoot = (0, ____ts_5.getWorkspaceRoot)();
            var crateName = (_a = (0, node_fs_1.readFileSync)((0, node_path_1.join)(rsDir, "Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)) === null || _a === void 0 ? void 0 : _a[1];
            if (!crateName) {
                throw new Error("[".concat(logPrefix, "] missing package name in Cargo.toml"));
            }
            var cargoWasm = (0, node_path_1.join)((0, ____ts_2.cargoTargetDirectory)(repoRoot, buildEnv), "wasm32-unknown-unknown/".concat(profileOutDir), "".concat(crateName.replace(/-/g, "_"), ".wasm"));
            var threadedCargoArgs = __spreadArray(__spreadArray(__spreadArray(["build", "--locked"], cargoProfileArgs, true), ["--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort"], false), featureArgs, true);
            status_1 = (0, ____ts_6.runCmdStatus)("cargo", threadedCargoArgs, { cwd: rsDir, env: buildEnv, budgetMs: (0, ____ts_6.buildBudgetMs)() });
            if (status_1 !== 0) {
                throw new Error("[".concat(logPrefix, "] cargo threaded build failed (").concat(status_1, ")"));
            }
            if (!(0, node_fs_1.existsSync)(pkgDir))
                (0, node_fs_1.mkdirSync)(pkgDir, { recursive: true });
            status_1 = (0, ____ts_6.runCmdStatus)(bindgen, [cargoWasm, "--out-dir", outputDirectory, "--typescript", "--target", "web", "--out-name", wasmBaseName], { cwd: rsDir, env: buildEnv, budgetMs: (0, ____ts_6.buildBudgetMs)() });
        }
        else {
            var localWasmPack = (0, ____ts_6.resolveWorkspaceBin)("wasm-pack", rsDir);
            var buildArgs = __spreadArray(__spreadArray(__spreadArray(["build", "--mode", "no-install"], packProfileArgs, true), ["--target", "web", "--out-dir", outputDirectory, "--out-name", wasmBaseName, "--no-pack", "--", "--locked"], false), featureArgs, true);
            if (localWasmPack) {
                status_1 = (0, ____ts_6.runCmdStatus)(process.execPath, __spreadArray([localWasmPack], buildArgs, true), { cwd: rsDir, env: buildEnv, budgetMs: (0, ____ts_6.buildBudgetMs)() });
            }
            else {
                status_1 = (0, ____ts_6.runCmdStatus)("wasm-pack", buildArgs, { cwd: rsDir, env: buildEnv, budgetMs: (0, ____ts_6.buildBudgetMs)() });
            }
        }
        if (status_1 !== 0) {
            throw new Error("[".concat(logPrefix, "] wasm build failed (").concat(status_1, ")"));
        }
        console.log("[".concat(logPrefix, "] wasm build done in ").concat(((Date.now() - t0) / 1000).toFixed(1), "s"));
        if (!(0, node_fs_1.existsSync)(pkgDir))
            (0, node_fs_1.mkdirSync)(pkgDir, { recursive: true });
        var snippetFiles = wasmPackSnippetFiles(pkgDir);
        var pkgJson = __assign(__assign({ type: "module", version: (_b = pkg.version) !== null && _b !== void 0 ? _b : "0.1.0", sideEffects: (_c = pkg.sideEffects) !== null && _c !== void 0 ? _c : ["./snippets/*"] }, pkg), { files: __spreadArray([], new Set(__spreadArray(__spreadArray([], pkg.files, true), snippetFiles, true)), true) });
        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(pkgDir, "package.json"), "".concat(JSON.stringify(pkgJson, null, 2), "\n"), "utf8");
        if ((0, node_fs_1.existsSync)(wasmPath)) {
            var sz = ((0, node_fs_1.statSync)(wasmPath).size / (1024 * 1024)).toFixed(2);
            console.log("[".concat(logPrefix, "] pkg/").concat(wasmBaseName, "_bg.wasm ready (").concat(sz, " MiB) + pkg/package.json restored"));
        }
        else {
            throw new Error("[".concat(logPrefix, "] expected wasm output missing: ").concat(wasmPath));
        }
    }
    finally {
        (0, node_fs_1.rmSync)(cargoOutput, { recursive: true, force: true });
    }
}
var EXTENSION_COMPONENT_WASM_TARGET = "wasm32-wasip2";
/** 🛡️ Explicit WASI-only link profiles; native dev and publication identity stay separate. */
function selectComponentWasmProfile(mode, override) {
    if (mode !== "dev" && mode !== "ship")
        throw new Error("invalid component build mode");
    var profile = override !== null && override !== void 0 ? override : (mode === "ship" ? "wasm-release" : "wasm-dev");
    if (profile !== "wasm-dev" && profile !== "wasm-release")
        throw new Error("SEMIO_PLUGIN_PROFILE must be wasm-dev or wasm-release");
    return profile;
}
function parseCargoTomlStringArray(block, key) {
    var match = block.match(new RegExp("^".concat(key, "\\s*=\\s*\\[([^\\]]*)\\]"), "m"));
    if (!match)
        return [];
    return __spreadArray([], match[1].matchAll(/"([^"]+)"/g), true).map(function (row) { return row[1]; });
}
/** 🧩️ Reads one extension crate's Cargo identity — the crate name every extension route (`package`'s
 * `.sxt` and `describe`'s owner descriptor pair alike) builds from, its component package id, and its
 * declared `extends` host. */
function parseExtensionCargoManifest(manifestPath, repoRoot) {
    var _a, _b, _c, _d, _e, _f, _g, _h;
    var text = (0, node_fs_1.readFileSync)(manifestPath, "utf8");
    var packageName = (_a = text.match(/^name\s*=\s*"([^"]+)"/m)) === null || _a === void 0 ? void 0 : _a[1];
    if (!packageName)
        throw new Error("missing package name in ".concat(manifestPath));
    var version = (_b = text.match(/^version\s*=\s*"([^"]+)"/m)) === null || _b === void 0 ? void 0 : _b[1];
    if (!version && /^\s*version\.workspace\s*=\s*true/m.test(text)) {
        var rootToml = (0, node_fs_1.readFileSync)((0, node_path_1.join)(repoRoot, "Cargo.toml"), "utf8");
        version = (_c = rootToml.match(/\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"/m)) === null || _c === void 0 ? void 0 : _c[1];
    }
    version = version !== null && version !== void 0 ? version : "0.1.0";
    var description = (_e = (_d = text.match(/^description\s*=\s*"([^"]+)"/m)) === null || _d === void 0 ? void 0 : _d[1]) !== null && _e !== void 0 ? _e : packageName;
    var componentPackageId = (_f = text.match(/\[package\.metadata\.component\][\s\S]*?^package\s*=\s*"semio:([^"]+)"/m)) === null || _f === void 0 ? void 0 : _f[1];
    if (!componentPackageId)
        throw new Error("missing [package.metadata.component].package in ".concat(manifestPath));
    var semioLines = [];
    var inSemio = false;
    for (var _i = 0, _j = text.split("\n"); _i < _j.length; _i++) {
        var line = _j[_i];
        if (line.trim() === "[package.metadata.semio]") {
            inSemio = true;
            continue;
        }
        if (inSemio && line.startsWith("[") && line.trim() !== "[package.metadata.semio]")
            break;
        if (inSemio)
            semioLines.push(line);
    }
    var semioBlock = semioLines.join("\n");
    var extendsHost = (_h = (_g = semioBlock.match(/^extends\s*=\s*"([^"]+)"/m)) === null || _g === void 0 ? void 0 : _g[1]) !== null && _h !== void 0 ? _h : "";
    var contributes = parseCargoTomlStringArray(semioBlock, "contributes");
    var directoryName = (0, node_path_1.basename)((0, node_path_1.resolve)((0, node_path_1.dirname)(manifestPath), "../.."));
    return { packageName: packageName, directoryName: directoryName, version: version, description: description, componentPackageId: componentPackageId, extends: extendsHost, contributes: contributes };
}
/** @emoji 📦 Builds a wasip2 component and writes a runtime-installable `.sxt` beside the crate (`dist/<id>.sxt` by default). */
function runExtensionComponentPackage(opts) {
    return __awaiter(this, void 0, void 0, function () {
        var repoRoot, rsDir, manifestPath, parsed, profile, logPrefix, wasmArtifact, componentWasm, packExtensionPackage, manifest, packed, outPath;
        var _a, _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0:
                    repoRoot = (_a = opts.repoRoot) !== null && _a !== void 0 ? _a : (0, ____ts_5.getWorkspaceRoot)();
                    rsDir = (0, node_path_1.resolve)(opts.rsDir);
                    manifestPath = (0, node_path_1.join)(rsDir, "Cargo.toml");
                    parsed = parseExtensionCargoManifest(manifestPath, repoRoot);
                    profile = selectComponentWasmProfile((0, ____ts_6.semioBuildMode)(), process.env.SEMIO_PLUGIN_PROFILE);
                    logPrefix = (_b = opts.logPrefix) !== null && _b !== void 0 ? _b : parsed.componentPackageId;
                    if ((0, ____ts_6.runCmdStatus)("cargo", ["build", "-p", parsed.packageName, "--target", EXTENSION_COMPONENT_WASM_TARGET, "--profile", profile], { cwd: repoRoot, budgetMs: (0, ____ts_6.buildBudgetMs)() }) !== 0) {
                        throw new Error("extension component build failed: ".concat(parsed.packageName));
                    }
                    wasmArtifact = (0, node_path_1.join)((0, ____ts_2.cargoTargetDirectory)(repoRoot), EXTENSION_COMPONENT_WASM_TARGET, (0, ____ts_6.cargoProfileDir)(profile), "".concat(parsed.packageName.replace(/-/g, "_"), ".wasm"));
                    if (!(0, node_fs_1.existsSync)(wasmArtifact))
                        throw new Error("missing wasm artifact ".concat(wasmArtifact));
                    componentWasm = new Uint8Array((0, node_fs_1.readFileSync)(wasmArtifact));
                    return [4 /*yield*/, Promise.resolve().then(function () { return require("../../../💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts"); })];
                case 1:
                    packExtensionPackage = (_c.sent()).packExtensionPackage;
                    manifest = {
                        extensionId: parsed.componentPackageId,
                        directoryName: parsed.directoryName,
                        label: parsed.description,
                        version: parsed.version,
                        extends: parsed.extends,
                        capabilities: __spreadArray([], parsed.contributes, true),
                        contributions: parsed.contributes,
                        packageFormat: 1,
                    };
                    packed = packExtensionPackage({ manifest: manifest, componentWasm: componentWasm });
                    outPath = (0, node_path_1.join)(rsDir, "dist", "".concat(parsed.componentPackageId, ".sxt"));
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(outPath), { recursive: true });
                    (0, node_fs_1.writeFileSync)(outPath, packed);
                    console.log("[DEBUG] ".concat(logPrefix, " packaged ").concat(outPath, " (").concat(packed.length, " bytes)"));
                    return [2 /*return*/, outPath];
            }
        });
    });
}
/** 🔗️Resolves `import.meta.url` of the bundle `script.ts`. */
function scriptPathFromUrl(scriptUrl) {
    return (0, node_url_1.fileURLToPath)(scriptUrl);
}
//#endregion Types
//#region Constants
var METRICS_LOCK_FILES = new Set(["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock", "bun.lockb", "cargo.lock"]);
var METRICS_LICENSE_TEMPLATE_BASENAMES = new Set(["LICENSE", "LICENSE.md", "LICENSE.txt", "COPYING", "COPYING.md", "NOTICE", "NOTICE.md", "UNLICENSE", "UNLICENSE.md"]);
var LANG_EMOJI = {
    TypeScript: "🟦️",
    JavaScript: "🟨️",
    Go: "🔵️",
    "C#": "🟣️",
    Python: "🐍️",
    Rust: "🦀️",
    Shell: "🐚️",
    Dockerfile: "🐳️",
    Makefile: "🔧️",
    CSS: "🎨️",
    SQL: "🛢️",
    HTML: "🌐️",
    Markdown: "📝️",
    TeX: "📐️",
    JSON: "🧾️",
    YAML: "📋️",
    TOML: "⚙️",
    XML: "📄️",
    CSV: "📑️",
    "Bourne Shell": "🐚️",
    "Bourne Again Shell": "🐚️",
    PowerShell: "💠️",
    Docker: "🐳️",
};
var ULOC_EXCLUDE_DIRS = [".🧬semio", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage"];
var MAX_METRICS_FILE_BYTES = 8 * 1024 * 1024;
//#endregion Constants
//#region Path rules
function normalizeRepoPath(path) {
    return path.replace(/\\/g, "/").replace(/^\.\//, "");
}
function metricsPathBasename(rel) {
    return rel.slice(rel.lastIndexOf("/") + 1);
}
function hasHiddenDotPathSegment(rel) {
    for (var _i = 0, _a = rel.split("/"); _i < _a.length; _i++) {
        var seg = _a[_i];
        if (!seg || seg === "." || seg === "..")
            continue;
        if (seg.startsWith("."))
            return true;
    }
    return false;
}
function isMetricsLockOrGenerated(rel) {
    var base = metricsPathBasename(rel);
    if (METRICS_LOCK_FILES.has(base))
        return true;
    if (base.endsWith(".generated.go") || base.endsWith(".pb.go"))
        return true;
    return false;
}
function isMetricsLicenseTemplateFile(rel) {
    var base = metricsPathBasename(rel);
    if (METRICS_LICENSE_TEMPLATE_BASENAMES.has(base))
        return true;
    if (base.startsWith("LICENSE."))
        return true;
    return false;
}
/** 🗂️Whether paths must be excluded from uloc/metrics (dot paths, license templates, vendor, lockfiles). */
function shouldSkipPathForUloc(root, relPath) {
    var rel = normalizeRepoPath(relPath);
    if (!rel || rel === ".🧬semio" || rel.startsWith(".🧬semio/"))
        return true;
    if (hasHiddenDotPathSegment(rel))
        return true;
    if (isMetricsLicenseTemplateFile(rel))
        return true;
    if (isMetricsLockOrGenerated(rel))
        return true;
    for (var _i = 0, ULOC_EXCLUDE_DIRS_1 = ULOC_EXCLUDE_DIRS; _i < ULOC_EXCLUDE_DIRS_1.length; _i++) {
        var dir = ULOC_EXCLUDE_DIRS_1[_i];
        if (rel === dir || rel.startsWith("".concat(dir, "/")))
            return true;
    }
    return false;
}
/** 🏷️Maps a repo path to a metrics language bucket (code langs + JSON/YAML/… formats). */
function classifyPathForMetrics(path) {
    var rel = normalizeRepoPath(path);
    var base = rel.slice(rel.lastIndexOf("/") + 1).toLowerCase();
    if (base === "dockerfile" || base.startsWith("dockerfile."))
        return "Dockerfile";
    if (base === "makefile" || base === "justfile")
        return "Makefile";
    var ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
    switch (ext) {
        case ".ts":
        case ".tsx":
        case ".cts":
        case ".mts":
        case ".mtsx":
            return "TypeScript";
        case ".js":
        case ".mjs":
        case ".cjs":
            return "JavaScript";
        case ".go":
            return "Go";
        case ".cs":
            return "C#";
        case ".py":
            return "Python";
        case ".rs":
            return "Rust";
        case ".sh":
        case ".bash":
        case ".zsh":
            return "Shell";
        case ".ps1":
            return "PowerShell";
        case ".css":
        case ".scss":
        case ".sass":
            return "CSS";
        case ".sql":
            return "SQL";
        case ".html":
        case ".htm":
        case ".xhtml":
            return "HTML";
        case ".md":
        case ".markdown":
        case ".mdown":
        case ".mkd":
        case ".mdx":
        case ".mdc":
        case ".svx":
            return "Markdown";
        case ".tex":
        case ".sty":
        case ".cls":
        case ".ltx":
        case ".bib":
            return "TeX";
        case ".json":
        case ".jsonc":
            return "JSON";
        case ".yaml":
        case ".yml":
            return "YAML";
        case ".toml":
            return "TOML";
        case ".csv":
            return "CSV";
        case ".xml":
            return "XML";
        default:
            return "";
    }
}
/** 🎨️Emoji for a metrics language row. */
function langMetricsEmoji(lang) {
    var _a;
    return (_a = LANG_EMOJI[lang]) !== null && _a !== void 0 ? _a : "📎️";
}
/** 🌳️Resolves the git worktree root (never a subdirectory of the monorepo). */
function gitRepoRoot(start) {
    var r = spawnCapturedSync("git", ["rev-parse", "--show-toplevel"], { cwd: start, env: gitSpawnEnv() });
    if (r.status === 0) {
        var top_1 = r.stdout.toString("utf8").trim();
        if (top_1)
            return top_1;
    }
    return start;
}
function gitTrackedPaths(root) {
    var repoRoot = gitRepoRoot(root);
    var r = spawnCapturedSync("git", ["ls-files", "-z"], { cwd: repoRoot, env: gitSpawnEnv() });
    if (r.status !== 0)
        return [];
    return r.stdout.toString("utf8").split("\0").filter(Boolean);
}
//#endregion Path rules
//#region Uloc counting
function countJsonKeysValue(v) {
    if (v === null || typeof v !== "object")
        return 0;
    if (Array.isArray(v)) {
        var n_1 = 0;
        for (var _i = 0, v_1 = v; _i < v_1.length; _i++) {
            var item = v_1[_i];
            n_1 += countJsonKeysValue(item);
        }
        return n_1;
    }
    var o = v;
    var n = Object.keys(o).length;
    for (var _a = 0, _b = Object.values(o); _a < _b.length; _a++) {
        var vv = _b[_a];
        n += countJsonKeysValue(vv);
    }
    return n;
}
/** 🧮️Counts JSON object keys recursively (aligned with repo `loc` Data JSON rules). */
function countJsonKeys(text) {
    var t = text.trim();
    if (!t)
        return 0;
    try {
        return countJsonKeysValue(JSON.parse(t));
    }
    catch (_a) {
        return 0;
    }
}
function jsonUlocUnitFingerprint(value) {
    if (value === null || typeof value !== "object")
        return JSON.stringify(value);
    if (Array.isArray(value)) {
        return "array:".concat(value.map(function (item) { return (item !== null && typeof item === "object" ? (Array.isArray(item) ? "[]" : "{}") : JSON.stringify(item)); }).join("\\u0000"));
    }
    return "object:".concat(Object.keys(value)
        .map(function (key) { return JSON.stringify(key); })
        .join("\\u0000"));
}
function collectJsonUlocUnits(value, path, units) {
    if (Array.isArray(value)) {
        for (var _i = 0, _a = value.entries(); _i < _a.length; _i++) {
            var _b = _a[_i], index = _b[0], item = _b[1];
            collectJsonUlocUnits(item, "".concat(path, "/").concat(index), units);
        }
        return;
    }
    if (value === null || typeof value !== "object")
        return;
    for (var _c = 0, _d = Object.entries(value); _c < _d.length; _c++) {
        var _e = _d[_c], key = _e[0], item = _e[1];
        var nextPath = "".concat(path, "/").concat(JSON.stringify(key));
        units.set(nextPath, jsonUlocUnitFingerprint(item));
        collectJsonUlocUnits(item, nextPath, units);
    }
}
function jsonUlocUnits(text) {
    try {
        var value = JSON.parse(text);
        var units = new Map();
        collectJsonUlocUnits(value, "", units);
        return units.size > 0 ? units : null;
    }
    catch (_a) {
        return null;
    }
}
/** 🧮️Counts JSON additions, semantic edits, and removals in the same key-based ULOC unit as totals. */
function diffJsonUloc(before, after) {
    var beforeUnits = before === null ? new Map() : jsonUlocUnits(before);
    var afterUnits = after === null ? new Map() : jsonUlocUnits(after);
    if (beforeUnits === null || afterUnits === null)
        return null;
    var added = 0;
    var removed = 0;
    var edited = 0;
    for (var _i = 0, afterUnits_1 = afterUnits; _i < afterUnits_1.length; _i++) {
        var _a = afterUnits_1[_i], path = _a[0], fingerprint = _a[1];
        var previous = beforeUnits.get(path);
        if (previous === undefined)
            added++;
        else if (previous !== fingerprint)
            edited++;
    }
    for (var _b = 0, _c = beforeUnits.keys(); _b < _c.length; _b++) {
        var path = _c[_b];
        if (!afterUnits.has(path))
            removed++;
    }
    return { added: added, removed: removed, edited: edited };
}
function physicalLineCount(text) {
    if (!text.length)
        return 0;
    return text.split(/\r?\n/).length;
}
/** 📏️LOC for one tracked file body (JSON key count when applicable). */
function countUnifiedLocForFile(rel, data) {
    var ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
    if (ext === ".json" || ext === ".jsonc") {
        var keys = countJsonKeys(data);
        if (keys > 0)
            return keys;
        if (!data.trim())
            return 0;
        return physicalLineCount(data);
    }
    return physicalLineCount(data);
}
function gitDir(root) {
    var repoRoot = gitRepoRoot(root);
    var r = spawnCapturedSync("git", ["rev-parse", "--git-dir"], { cwd: repoRoot, env: gitSpawnEnv() });
    if (r.status !== 0)
        return (0, node_path_1.join)(repoRoot, ".git");
    var dir = r.stdout.toString("utf8").trim();
    return dir.startsWith("/") ? dir : (0, node_path_1.join)(repoRoot, dir);
}
var METRICS_CACHE_VERSION = 5;
function metricsCachePath(root) {
    return (0, node_path_1.join)(gitDir(root), "compose-metrics-cache.json");
}
function metricsMapStats(counts) {
    var total = 0;
    var langCount = 0;
    for (var _i = 0, _a = Object.values(counts); _i < _a.length; _i++) {
        var n = _a[_i];
        if (n > 0) {
            total += n;
            langCount++;
        }
    }
    return { total: total, langCount: langCount };
}
/** 🧪️Whether cached uloc looks like a complete repo scan (rejects partial/stale caches). */
function isUlocCachePlausible(root, counts) {
    var _a = metricsMapStats(counts), totalLoc = _a.total, langCount = _a.langCount;
    if (totalLoc <= 0 || langCount === 0)
        return false;
    var tracked = gitTrackedPaths(root).length;
    if (tracked === 0) {
        var head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
        if (head.status === 0 && head.stdout.toString("utf8").trim())
            return false;
        return true;
    }
    if (tracked < 100)
        return totalLoc > 0 && (langCount >= 2 || totalLoc >= 50);
    if (langCount < 6 && tracked >= 300)
        return false;
    if (totalLoc < tracked * 3)
        return false;
    return true;
}
function isSizeCachePlausible(root, counts) {
    var _a = metricsMapStats(counts), totalBytes = _a.total, langCount = _a.langCount;
    if (totalBytes <= 0 || langCount === 0)
        return false;
    var tracked = gitTrackedPaths(root).length;
    if (tracked === 0) {
        var head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
        if (head.status === 0 && head.stdout.toString("utf8").trim())
            return false;
        return true;
    }
    if (tracked < 100)
        return totalBytes > 0 && langCount >= 1;
    if (langCount < 3 && tracked >= 300)
        return false;
    return true;
}
function readMetricsCache(root) {
    var _a, _b;
    var head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: root, env: gitSpawnEnv() });
    if (head.status !== 0)
        return null;
    var h = head.stdout.toString("utf8").trim();
    if (!h)
        return null;
    try {
        var raw = (0, node_fs_1.readFileSync)(metricsCachePath(root), "utf8");
        var parsed = JSON.parse(raw);
        if (parsed.version !== METRICS_CACHE_VERSION)
            return null;
        if (parsed.head !== h || !((_a = parsed.uloc) === null || _a === void 0 ? void 0 : _a.counts) || !((_b = parsed.size) === null || _b === void 0 ? void 0 : _b.counts))
            return null;
        var tracked = gitTrackedPaths(root).length;
        if (typeof parsed.trackedFiles === "number" && Math.abs(tracked - parsed.trackedFiles) > 50)
            return null;
        if (!isUlocCachePlausible(root, parsed.uloc.counts))
            return null;
        if (!isSizeCachePlausible(root, parsed.size.counts))
            return null;
        var ulocStats = metricsMapStats(parsed.uloc.counts);
        if (parsed.uloc.totalLoc !== ulocStats.total || parsed.uloc.langCount !== ulocStats.langCount)
            return null;
        var sizeStats = metricsMapStats(parsed.size.counts);
        if (parsed.size.totalBytes !== sizeStats.total || parsed.size.langCount !== sizeStats.langCount)
            return null;
        return { uloc: parsed.uloc.counts, size: parsed.size.counts };
    }
    catch (_c) {
        return null;
    }
}
function writeMetricsCache(root, uloc, size) {
    var head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: root, env: gitSpawnEnv() });
    if (head.status !== 0)
        return;
    var h = head.stdout.toString("utf8").trim();
    if (!h)
        return;
    var ulocStats = metricsMapStats(uloc);
    var sizeStats = metricsMapStats(size);
    var payload = {
        version: METRICS_CACHE_VERSION,
        head: h,
        trackedFiles: gitTrackedPaths(root).length,
        uloc: { totalLoc: ulocStats.total, langCount: ulocStats.langCount, counts: uloc },
        size: { totalBytes: sizeStats.total, langCount: sizeStats.langCount, counts: size },
    };
    (0, node_fs_1.writeFileSync)(metricsCachePath(root), JSON.stringify(payload));
}
/** 📊️Scans all git-tracked paths and sums unified LOC per language bucket. */
function scanRepoUnifiedLocUncached(root) {
    var _a;
    var repoRoot = gitRepoRoot(root);
    var out = {};
    for (var _i = 0, _b = gitTrackedPaths(root); _i < _b.length; _i++) {
        var rel = _b[_i];
        if (shouldSkipPathForUloc(repoRoot, rel))
            continue;
        var lang = classifyPathForMetrics(rel);
        if (!lang)
            continue;
        var fp = (0, node_path_1.join)(repoRoot, rel);
        if (!(0, node_fs_1.existsSync)(fp))
            continue;
        try {
            var st = (0, node_fs_1.statSync)(fp);
            if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES)
                continue;
        }
        catch (_c) {
            continue;
        }
        var data = void 0;
        try {
            var buf = (0, node_fs_1.readFileSync)(fp);
            if (buf.includes(0))
                continue;
            data = buf.toString("utf8");
        }
        catch (_d) {
            continue;
        }
        var loc = countUnifiedLocForFile(rel, data);
        if (loc <= 0)
            continue;
        out[lang] = ((_a = out[lang]) !== null && _a !== void 0 ? _a : 0) + loc;
    }
    return out;
}
/** 📊️Scans tracked paths and sums on-disk byte size per language (includes binaries and large files). */
function scanRepoSizeByLanguageUncached(root) {
    var _a;
    var repoRoot = gitRepoRoot(root);
    var out = {};
    for (var _i = 0, _b = gitTrackedPaths(root); _i < _b.length; _i++) {
        var rel = _b[_i];
        if (shouldSkipPathForUloc(repoRoot, rel))
            continue;
        var lang = classifyPathForMetrics(rel);
        if (!lang)
            continue;
        var fp = (0, node_path_1.join)(repoRoot, rel);
        if (!(0, node_fs_1.existsSync)(fp))
            continue;
        try {
            var st = (0, node_fs_1.statSync)(fp);
            if (!st.isFile() || st.size <= 0)
                continue;
            out[lang] = ((_a = out[lang]) !== null && _a !== void 0 ? _a : 0) + st.size;
        }
        catch (_c) {
            continue;
        }
    }
    return out;
}
/** 📊️Repo uloc with per-HEAD cache under `.git/compose-metrics-cache.json`. */
function scanRepoUnifiedLoc(root) {
    var cached = readMetricsCache(root);
    if (cached)
        return cached.uloc;
    var uloc = scanRepoUnifiedLocUncached(root);
    var size = scanRepoSizeByLanguageUncached(root);
    writeMetricsCache(root, uloc, size);
    return uloc;
}
/** 📊️Repo byte size per language with per-HEAD cache. */
function scanRepoSizeByLanguage(root) {
    var cached = readMetricsCache(root);
    if (cached)
        return cached.size;
    var uloc = scanRepoUnifiedLocUncached(root);
    var size = scanRepoSizeByLanguageUncached(root);
    writeMetricsCache(root, uloc, size);
    return size;
}
/** 📊️Default uloc runner (tracked-file unified scan). */
function createDefaultUlocRunner() {
    return { countRepoByLanguage: scanRepoUnifiedLoc };
}
/** 📊️Default metrics runner (uloc + size scans). */
function createDefaultMetricsRunner() {
    return {
        countRepoUlocByLanguage: scanRepoUnifiedLoc,
        countRepoSizeByLanguage: scanRepoSizeByLanguage,
    };
}
/** 📊️Adapts legacy uloc-only runners; size bloc still comes from repo scan. */
function metricsRunnerFromUloc(ulocRunner) {
    var uloc = ulocRunner !== null && ulocRunner !== void 0 ? ulocRunner : createDefaultUlocRunner();
    return {
        countRepoUlocByLanguage: uloc.countRepoByLanguage,
        countRepoSizeByLanguage: scanRepoSizeByLanguage,
    };
}
//#endregion Uloc counting
//#region Git deltas
/** ✂️Splits git numstat into replaced (edited), net added, and net removed lines. */
function splitGitNumstatDelta(added, removed) {
    var a = Math.max(0, added);
    var r = Math.max(0, removed);
    var edited = Math.min(a, r);
    return { edited: edited, added: a - edited, removed: r - edited };
}
function parseGitNumstatZ(stdout) {
    var raw = typeof stdout === "string" ? stdout : stdout.toString("utf8");
    if (!raw)
        return [];
    var out = [];
    var entries = raw.split("\0");
    for (var index = 0; index < entries.length; index++) {
        var entry = entries[index];
        if (!entry)
            continue;
        var parts = entry.split("\t");
        if (parts.length < 3)
            continue;
        var added = parts[0] === "-" ? 0 : Number(parts[0]) || 0;
        var removed = parts[1] === "-" ? 0 : Number(parts[1]) || 0;
        var path = parts.slice(2).join("\t");
        if (!path && entries[index + 1] && entries[index + 2]) {
            path = "".concat(entries[++index], "\t").concat(entries[++index]);
        }
        if (path)
            out.push({ path: path, added: added, removed: removed });
    }
    return out;
}
function gitCachedNumstat(root) {
    var r = spawnCapturedSync("git", ["diff", "--cached", "--numstat", "-z"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
    if (r.status !== 0)
        return [];
    return parseGitNumstatZ(r.stdout);
}
function isJsonMetricsPath(path) {
    if (!path)
        return false;
    var ext = path.slice(path.lastIndexOf(".")).toLowerCase();
    return ext === ".json" || ext === ".jsonc";
}
/** 📂️Whether a repo-relative path lies under any normalized prefix. */
function pathUnderPrefixes(rel, prefixes) {
    if (prefixes.length === 0)
        return true;
    var n = normalizeRepoPath(rel);
    for (var _i = 0, prefixes_1 = prefixes; _i < prefixes_1.length; _i++) {
        var raw = prefixes_1[_i];
        var p = normalizeRepoPath(raw).replace(/\/$/, "");
        if (!p)
            continue;
        if (n === p || n.startsWith("".concat(p, "/")))
            return true;
    }
    return false;
}
/** 📈️Git numstat between two revisions. */
function gitRangeNumstat(root, base, head) {
    var repoRoot = gitRepoRoot(root);
    var r = spawnCapturedSync("git", ["diff", "--numstat", "-z", "".concat(base, "..").concat(head)], {
        cwd: repoRoot,
        env: gitSpawnEnv(),
    });
    if (r.status !== 0)
        return [];
    return parseGitNumstatZ(r.stdout);
}
/** ➕️Accumulates per-language git deltas from numstat rows (optional path prefixes). */
function accumulateGitDeltasFromNumstat(root, rows, pathPrefixes) {
    var _a;
    var m = new Map();
    for (var _i = 0, rows_1 = rows; _i < rows_1.length; _i++) {
        var _b = rows_1[_i], path = _b.path, added = _b.added, removed = _b.removed;
        if (shouldSkipPathForUloc(root, path))
            continue;
        if (pathPrefixes && !pathUnderPrefixes(path, pathPrefixes))
            continue;
        var lang = classifyPathForMetrics(path);
        if (!lang)
            continue;
        var d = splitGitNumstatDelta(added, removed);
        var row = (_a = m.get(lang)) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
        row.edited += d.edited;
        row.added += d.added;
        row.removed += d.removed;
        m.set(lang, row);
    }
    return m;
}
function accumulateGitDeltas(root) {
    return accumulateGitDeltasFromNumstat(root, gitCachedNumstat(root));
}
function gitObjectSizesAtSpecs(root, specs) {
    var _a;
    var unique = __spreadArray([], new Set(specs), true);
    if (unique.length === 0)
        return new Map();
    var result = spawnCapturedSync("git", ["cat-file", "--batch-check=%(objectname)\t%(objecttype)\t%(objectsize)\t%(rest)"], {
        cwd: gitRepoRoot(root),
        env: gitSpawnEnv(),
        input: "".concat(unique.join("\n"), "\n"),
    });
    if (result.status !== 0)
        return new Map();
    var sizes = new Map();
    var lines = result.stdout.toString("utf8").split("\n");
    for (var index = 0; index < unique.length; index++) {
        var line = (_a = lines[index]) !== null && _a !== void 0 ? _a : "";
        var first = line.indexOf("\t");
        var second = first < 0 ? -1 : line.indexOf("\t", first + 1);
        var third = second < 0 ? -1 : line.indexOf("\t", second + 1);
        if (third < 0)
            continue;
        var type = line.slice(first + 1, second);
        var size = Number(line.slice(second + 1, third));
        if (type === "blob" && Number.isSafeInteger(size) && size >= 0)
            sizes.set(unique[index], size);
    }
    return sizes;
}
var METRICS_BLOB_BATCH_BYTES = 16 * 1024 * 1024;
function gitBlobTextsAtSpecs(root, specs) {
    var unique = __spreadArray([], new Set(specs), true);
    var sizes = gitObjectSizesAtSpecs(root, unique);
    var texts = new Map();
    var batch = [];
    var batchBytes = 0;
    var flush = function () {
        if (batch.length === 0)
            return;
        var result = spawnCapturedSync("git", ["cat-file", "--batch"], {
            cwd: gitRepoRoot(root),
            env: gitSpawnEnv(),
            input: "".concat(batch.join("\n"), "\n"),
        });
        if (result.status !== 0) {
            for (var _i = 0, batch_1 = batch; _i < batch_1.length; _i++) {
                var spec = batch_1[_i];
                texts.set(spec, null);
            }
            batch = [];
            batchBytes = 0;
            return;
        }
        var output = result.stdout;
        var offset = 0;
        for (var _a = 0, batch_2 = batch; _a < batch_2.length; _a++) {
            var spec = batch_2[_a];
            var lineEnd = output.indexOf(0x0a, offset);
            if (lineEnd < 0) {
                texts.set(spec, null);
                continue;
            }
            var header = output.subarray(offset, lineEnd).toString("utf8");
            offset = lineEnd + 1;
            var match = /^[0-9a-f]+ blob (\d+)$/u.exec(header);
            if (!match) {
                texts.set(spec, null);
                continue;
            }
            var size = Number(match[1]);
            if (!Number.isSafeInteger(size) || size < 0 || offset + size > output.length) {
                texts.set(spec, null);
                continue;
            }
            var bytes = output.subarray(offset, offset + size);
            offset += size;
            if (output[offset] === 0x0a)
                offset++;
            texts.set(spec, bytes.includes(0) ? null : bytes.toString("utf8"));
        }
        batch = [];
        batchBytes = 0;
    };
    for (var _i = 0, unique_1 = unique; _i < unique_1.length; _i++) {
        var spec = unique_1[_i];
        var size = sizes.get(spec);
        if (size === undefined || size > MAX_METRICS_FILE_BYTES) {
            texts.set(spec, null);
            continue;
        }
        if (batch.length > 0 && batchBytes + size > METRICS_BLOB_BATCH_BYTES)
            flush();
        batch.push(spec);
        batchBytes += size;
    }
    flush();
    return texts;
}
function metricsPathPair(row) {
    var paths = pathsFromNumstatRow(row.path);
    if (paths.length === 0)
        return null;
    if (paths.length === 1)
        return { oldPath: paths[0], newPath: paths[0] };
    return { oldPath: paths[0], newPath: paths[1] };
}
function addLanguageDelta(deltas, language, delta) {
    var _a;
    var current = (_a = deltas.get(language)) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
    current.added += delta.added;
    current.removed += delta.removed;
    current.edited += delta.edited;
    deltas.set(language, current);
}
function ulocDeltaEntries(root, rows, pathPrefixes) {
    return rows
        .map(function (row, index) { return ({ index: index, row: row, pair: metricsPathPair(row) }); })
        .filter(function (entry) { return entry.pair !== null; })
        .filter(function (_a) {
        var pair = _a.pair;
        var paths = [pair.oldPath, pair.newPath].filter(function (path) { return path !== null; });
        return paths.some(function (path) { return !shouldSkipPathForUloc(root, path) && (!pathPrefixes || pathUnderPrefixes(path, pathPrefixes)); });
    });
}
function ulocBlobSpecs(entries, oldRev, newRev) {
    return entries.flatMap(function (_a) {
        var pair = _a.pair;
        return __spreadArray(__spreadArray([], (isJsonMetricsPath(pair.oldPath) ? ["".concat(oldRev, ":").concat(pair.oldPath)] : []), true), (isJsonMetricsPath(pair.newPath) ? ["".concat(newRev, ":").concat(pair.newPath)] : []), true);
    });
}
function ulocDeltaForEntry(root, entry, oldRev, newRev, blobs) {
    var _a, _b;
    var row = entry.row, pair = entry.pair;
    var deltas = new Map();
    var oldLanguage = pair.oldPath && !shouldSkipPathForUloc(root, pair.oldPath) ? classifyPathForMetrics(pair.oldPath) : "";
    var newLanguage = pair.newPath && !shouldSkipPathForUloc(root, pair.newPath) ? classifyPathForMetrics(pair.newPath) : "";
    var oldJson = isJsonMetricsPath(pair.oldPath) ? ((_a = blobs.get("".concat(oldRev, ":").concat(pair.oldPath))) !== null && _a !== void 0 ? _a : null) : null;
    var newJson = isJsonMetricsPath(pair.newPath) ? ((_b = blobs.get("".concat(newRev, ":").concat(pair.newPath))) !== null && _b !== void 0 ? _b : null) : null;
    if (oldLanguage === "JSON" && newLanguage === "JSON") {
        var delta = diffJsonUloc(oldJson, newJson);
        if (delta !== null) {
            addLanguageDelta(deltas, "JSON", delta);
            return deltas;
        }
    }
    if (oldLanguage && oldLanguage === newLanguage && oldLanguage !== "JSON") {
        addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(row.added, row.removed));
        return deltas;
    }
    if (oldLanguage === "JSON") {
        var delta = diffJsonUloc(oldJson, null);
        if (delta !== null)
            addLanguageDelta(deltas, oldLanguage, delta);
        else if (oldLanguage === newLanguage)
            addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(row.added, row.removed));
    }
    else if (oldLanguage) {
        addLanguageDelta(deltas, oldLanguage, splitGitNumstatDelta(0, row.removed));
    }
    if (newLanguage === "JSON") {
        var delta = diffJsonUloc(null, newJson);
        if (delta !== null)
            addLanguageDelta(deltas, newLanguage, delta);
        else if (oldLanguage !== newLanguage)
            addLanguageDelta(deltas, newLanguage, splitGitNumstatDelta(row.added, 0));
    }
    else if (newLanguage) {
        addLanguageDelta(deltas, newLanguage, splitGitNumstatDelta(row.added, 0));
    }
    return deltas;
}
function accumulateUlocDeltasByRow(root, rows, oldRev, newRev, pathPrefixes) {
    var deltas = rows.map(function () { return new Map(); });
    var entries = ulocDeltaEntries(root, rows, pathPrefixes);
    var blobs = gitBlobTextsAtSpecs(root, ulocBlobSpecs(entries, oldRev, newRev));
    for (var _i = 0, entries_4 = entries; _i < entries_4.length; _i++) {
        var entry = entries_4[_i];
        deltas[entry.index] = ulocDeltaForEntry(root, entry, oldRev, newRev, blobs);
    }
    return deltas;
}
/** 🧩️Accumulates per-language ULOC deltas from revision blobs, retaining JSON's key-based unit. */
function accumulateUlocDeltasFromPaths(root, rows, oldRev, newRev, pathPrefixes) {
    var deltas = new Map();
    for (var _i = 0, _a = accumulateUlocDeltasByRow(root, rows, oldRev, newRev, pathPrefixes); _i < _a.length; _i++) {
        var rowDeltas = _a[_i];
        for (var _b = 0, rowDeltas_1 = rowDeltas; _b < rowDeltas_1.length; _b++) {
            var _c = rowDeltas_1[_b], language = _c[0], delta = _c[1];
            addLanguageDelta(deltas, language, delta);
        }
    }
    return deltas;
}
/** ➕️Accumulates per-language byte deltas from path size changes between two git revisions. */
function accumulateSizeDeltasFromPaths(root, rows, oldRev, newRev, pathPrefixes) {
    var _a, _b, _c;
    var m = new Map();
    var paths = rows.flatMap(function (row) { return pathsFromNumstatRow(row.path); }).filter(function (path) { return !shouldSkipPathForUloc(root, path) && (!pathPrefixes || pathUnderPrefixes(path, pathPrefixes)); });
    var specs = paths.flatMap(function (path) { return ["".concat(oldRev, ":").concat(path), "".concat(newRev === ":0" ? ":0" : newRev, ":").concat(path)]; });
    var sizes = gitObjectSizesAtSpecs(root, specs);
    for (var _i = 0, paths_1 = paths; _i < paths_1.length; _i++) {
        var path = paths_1[_i];
        var lang = classifyPathForMetrics(path);
        if (!lang)
            continue;
        var oldBytes = (_a = sizes.get("".concat(oldRev, ":").concat(path))) !== null && _a !== void 0 ? _a : 0;
        var newBytes = (_b = sizes.get("".concat(newRev === ":0" ? ":0" : newRev, ":").concat(path))) !== null && _b !== void 0 ? _b : 0;
        var d = splitGitNumstatDelta(newBytes, oldBytes);
        var rowDelta = (_c = m.get(lang)) !== null && _c !== void 0 ? _c : { added: 0, removed: 0, edited: 0 };
        rowDelta.edited += d.edited;
        rowDelta.added += d.added;
        rowDelta.removed += d.removed;
        m.set(lang, rowDelta);
    }
    return m;
}
function accumulateStagedSizeDeltas(root) {
    return accumulateSizeDeltasFromPaths(root, gitCachedNumstat(root), "HEAD", ":0");
}
function accumulateStagedUlocDeltas(root) {
    return accumulateUlocDeltasFromPaths(root, gitCachedNumstat(root), "HEAD", ":0");
}
function accumulateRangeSizeDeltas(root, base, head, pathPrefixes) {
    return accumulateSizeDeltasFromPaths(root, gitRangeNumstat(root, base, head), base, head, pathPrefixes);
}
function accumulateRangeUlocDeltas(root, base, head, pathPrefixes) {
    return accumulateUlocDeltasFromPaths(root, gitRangeNumstat(root, base, head), base, head, pathPrefixes);
}
/** ➕️Sums git delta counters across all languages. */
function sumGitLangDeltas(deltas) {
    var added = 0;
    var removed = 0;
    var edited = 0;
    for (var _i = 0, _a = deltas.values(); _i < _a.length; _i++) {
        var d = _a[_i];
        added += d.added;
        removed += d.removed;
        edited += d.edited;
    }
    return { added: added, removed: removed, edited: edited };
}
/** 🟰️Sum of net added, edited (replaced), and removed line counts from git numstat. */
function gitDeltaLineTotal(d) {
    return d.added + d.edited + d.removed;
}
/** 📊️Appends `➕️` `✏️` `➖️` and total `🟰️` (sum of the three) when non-zero. */
function appendGitDeltaSuffix(line, d) {
    if (d.added > 0)
        line += "\u2795\uFE0F".concat(formatMetricLocCount(d.added));
    if (d.edited > 0)
        line += "\u270F\uFE0F".concat(formatMetricLocCount(d.edited));
    if (d.removed > 0)
        line += "\u2796\uFE0F".concat(formatMetricLocCount(d.removed));
    var sum = gitDeltaLineTotal(d);
    if (sum > 0)
        line += "\uD83D\uDFF0\uFE0F".concat(formatMetricLocCount(sum));
    return line;
}
/** 📊️Full inline `📊️metric…` suffixes (uloc + size) from bloc and git deltas. */
function formatBundleMetricSuffixes(ulocDelta, sizeDelta, ulocBloc, sizeBloc, langEmoji, langSlug) {
    return formatMetricBody(__assign(__assign({ kind: "uloc", code: ulocBloc }, ulocDelta), { langEmoji: langEmoji, langSlug: langSlug })) + formatMetricBody(__assign(__assign({ kind: "size", code: sizeBloc }, sizeDelta), { langEmoji: langEmoji, langSlug: langSlug }));
}
/** 📊️Full `📊️uloc💯️…` metrics line from bloc and git deltas (bundle header, footer, languages). */
function formatBundleUlocSuffix(d, code, langEmoji, langSlug) {
    return formatMetricBody(__assign(__assign({ kind: "uloc", code: code }, d), { langEmoji: langEmoji, langSlug: langSlug }));
}
//#endregion Git deltas
//#region Format
/** 🔢️Rounds to `digits` significant figures (preserves sign; 0 stays 0). */
function roundSignificant(n, digits) {
    if (digits === void 0) { digits = 3; }
    if (!Number.isFinite(n) || n === 0)
        return 0;
    var sign = n < 0 ? -1 : 1;
    var abs = Math.abs(n);
    var exp = Math.floor(Math.log10(abs));
    var factor = Math.pow(10, (digits - 1 - exp));
    return (sign * Math.round(abs * factor)) / factor;
}
/** 🔤️Renders a coefficient already near 3 significant figures (no scientific notation). */
function formatSignificantCoeff(n) {
    if (!Number.isFinite(n) || n === 0)
        return "0";
    var rounded = roundSignificant(n, 3);
    if (rounded === 0)
        return formatTinyPositive(Math.abs(n));
    if (Number.isInteger(rounded) || Math.abs(rounded - Math.round(rounded)) < 1e-12)
        return String(Math.round(rounded));
    var s = rounded.toPrecision(3);
    if (/e/i.test(s))
        s = expandScientificDecimal(Number(s));
    return s.replace(/(\.\d*?)0+$/, "$1").replace(/\.$/, "");
}
/** 🔤️Expands a tiny positive value without scientific notation (1 significant digit). */
function formatTinyPositive(n) {
    var one = roundSignificant(n, 1);
    if (one === 0) {
        var place = Math.pow(10, Math.floor(Math.log10(n)));
        return expandScientificDecimal(place);
    }
    return expandScientificDecimal(one);
}
/** 🔤️Decimal string for a finite number without `e` notation. */
function expandScientificDecimal(n) {
    if (!Number.isFinite(n) || n === 0)
        return "0";
    var abs = Math.abs(n);
    if (abs >= 1e-6 && abs < 1e21) {
        var decimals = Math.max(0, Math.min(16, -Math.floor(Math.log10(abs)) + 2));
        return ((n < 0 ? "-" : "") +
            abs
                .toFixed(decimals)
                .replace(/(\.\d*?)0+$/, "$1")
                .replace(/\.$/, ""));
    }
    var s = abs
        .toFixed(20)
        .replace(/(\.\d*?)0+$/, "$1")
        .replace(/\.$/, "");
    return n < 0 ? "-".concat(s) : s;
}
/** 🔢️Formats metric counts with 3 significant digits and k/M (e.g. 422377 → 422k, 1300000 → 1.3M). */
function formatMetricLocCount(n) {
    var v = Math.abs(n);
    if (!Number.isFinite(v) || v <= 0)
        return "0";
    var rounded = roundSignificant(v, 3);
    if (rounded === 0)
        rounded = roundSignificant(v, 1);
    if (rounded === 0)
        return formatTinyPositive(v);
    if (rounded >= 1000000)
        return "".concat(formatSignificantCoeff(rounded / 1000000), "M");
    if (rounded >= 1000)
        return "".concat(formatSignificantCoeff(rounded / 1000), "k");
    return formatSignificantCoeff(rounded);
}
/** ➗️Formats a ratio with 3 significant digits as a short decimal; never formats a positive value as 0. */
/** ➗️Formats net/previous as a percentage (*100) with 3 significant digits; never formats a positive value as 0. */
function formatMetricRatio(n) {
    if (!Number.isFinite(n) || n <= 0)
        return "0";
    // Caller passes a fraction (net/previous); display as percent.
    var pct = n * 100;
    var v = roundSignificant(pct, 3);
    if (v === 0)
        v = roundSignificant(pct, 1);
    if (v === 0)
        return formatTinyPositive(pct);
    if (v >= 1)
        return formatSignificantCoeff(v);
    // Short decimals by magnitude for sub-1% values.
    var places;
    if (v >= 0.1)
        places = 3;
    else if (v >= 0.01)
        places = 2;
    else if (v >= 0.001)
        places = 3;
    else
        places = 4;
    var factor = Math.pow(10, places);
    var capped = Math.round(v * factor) / factor;
    if (capped === 0)
        return formatSignificantCoeff(1 / factor);
    return formatSignificantCoeff(capped);
}
/** 🏷️Lowercase metrics slug for a language bucket (`TypeScript` → `typescript`). */
function langMetricsSlug(lang) {
    var fixed = {
        TypeScript: "typescript",
        JavaScript: "javascript",
        "C#": "csharp",
        JSON: "json",
        YAML: "yaml",
        TOML: "toml",
        Markdown: "markdown",
        Dockerfile: "dockerfile",
        Makefile: "makefile",
        "Bourne Shell": "shell",
        "Bourne Again Shell": "shell",
        PowerShell: "powershell",
        TeX: "tex",
        HTML: "html",
        CSS: "css",
        SQL: "sql",
        CSV: "csv",
        XML: "xml",
        Go: "go",
        Python: "python",
        Rust: "rust",
        Shell: "shell",
        Docker: "docker",
    };
    if (fixed[lang])
        return fixed[lang];
    return lang
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "")
        .slice(0, 24);
}
/** 🔢️Formats byte counts with 3 significant digits and B/KB/MB/GB. */
function formatMetricSizeCount(n) {
    var v = Math.abs(n);
    if (!Number.isFinite(v) || v <= 0)
        return "0";
    var rounded = roundSignificant(v, 3);
    if (rounded === 0)
        rounded = roundSignificant(v, 1);
    if (rounded === 0)
        return formatTinyPositive(v);
    if (rounded >= 1000000000)
        return "".concat(formatSignificantCoeff(rounded / 1000000000), "GB");
    if (rounded >= 1000000)
        return "".concat(formatSignificantCoeff(rounded / 1000000), "MB");
    if (rounded >= 1000)
        return "".concat(formatSignificantCoeff(rounded / 1000), "KB");
    return "".concat(formatSignificantCoeff(rounded), "B");
}
exports.METRIC_KIND_ULOC = { id: "uloc", token: "📃uloc", formatCount: formatMetricLocCount };
exports.METRIC_KIND_SIZE = { id: "size", token: "💾size", formatCount: formatMetricSizeCount };
exports.METRIC_KINDS = { uloc: exports.METRIC_KIND_ULOC, size: exports.METRIC_KIND_SIZE };
/** 📊️Commit metrics block header. */
exports.COMMIT_METRIC_HEADER = "📊️metric";
/** 🔢️Emoji for the aggregate total row (internal sums only). */
exports.COMMIT_METRIC_TOTAL_EMOJI = "🔢️";
/** 📊️Renders one explicit `📊️metric…` line; omits empty segments. */
function formatMetricBody(input) {
    var kind = input.kind, code = input.code, added = input.added, edited = input.edited, removed = input.removed, langEmoji = input.langEmoji, langSlug = input.langSlug;
    var metricKind = exports.METRIC_KINDS[kind];
    var line = exports.COMMIT_METRIC_HEADER;
    if (langEmoji && langSlug)
        line += "".concat(langEmoji).concat(langSlug);
    line += metricKind.token;
    var bloc = Math.max(0, Math.round(code));
    if (bloc > 0)
        line += "\uD83D\uDCAF\uFE0F".concat(metricKind.formatCount(bloc));
    var net = added - removed;
    if (net !== 0) {
        var absNet = Math.abs(net);
        line += net > 0 ? "\uD83D\uDCC8\uFE0F".concat(metricKind.formatCount(absNet)) : "\uD83D\uDCC9\uFE0F".concat(metricKind.formatCount(absNet));
        var previous = bloc - net;
        if (previous > 0)
            line += "\u2797\uFE0F".concat(formatMetricRatio(absNet / previous));
    }
    if (added > 0)
        line += "\u2795\uFE0F".concat(metricKind.formatCount(added));
    if (edited > 0)
        line += "\u270F\uFE0F".concat(metricKind.formatCount(edited));
    if (removed > 0)
        line += "\u2796\uFE0F".concat(metricKind.formatCount(removed));
    var sum = gitDeltaLineTotal({ added: added, edited: edited, removed: removed });
    if (sum > 0)
        line += "\uD83D\uDFF0\uFE0F".concat(metricKind.formatCount(sum));
    return line;
}
/** 📊️Renders one explicit `📊️metric📃uloc…` line (uloc alias). */
function formatUlocMetricsBody(input) {
    var _a;
    return formatMetricBody({ kind: (_a = input.kind) !== null && _a !== void 0 ? _a : "uloc", code: input.code, added: input.added, edited: input.edited, removed: input.removed, langEmoji: input.langEmoji, langSlug: input.langSlug });
}
/** 📂️Sums unified LOC for tracked paths under prefixes (bundle-scoped 💯️). */
function countUnifiedLocUnderPathPrefixes(root, prefixes) {
    var repoRoot = gitRepoRoot(root);
    var total = 0;
    for (var _i = 0, _a = gitTrackedPaths(root); _i < _a.length; _i++) {
        var rel = _a[_i];
        if (prefixes.length > 0 && !pathUnderPrefixes(rel, prefixes))
            continue;
        if (shouldSkipPathForUloc(repoRoot, rel))
            continue;
        var lang = classifyPathForMetrics(rel);
        if (!lang)
            continue;
        var fp = (0, node_path_1.join)(repoRoot, rel);
        if (!(0, node_fs_1.existsSync)(fp))
            continue;
        try {
            var st = (0, node_fs_1.statSync)(fp);
            if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES)
                continue;
        }
        catch (_b) {
            continue;
        }
        var data = void 0;
        try {
            var buf = (0, node_fs_1.readFileSync)(fp);
            if (buf.includes(0))
                continue;
            data = buf.toString("utf8");
        }
        catch (_c) {
            continue;
        }
        var loc = countUnifiedLocForFile(rel, data);
        if (loc > 0)
            total += loc;
    }
    return total;
}
/** 📂️Sums on-disk bytes for tracked paths under prefixes (bundle-scoped size 💯️). */
function countBytesUnderPathPrefixes(root, prefixes) {
    var repoRoot = gitRepoRoot(root);
    var total = 0;
    for (var _i = 0, _a = gitTrackedPaths(root); _i < _a.length; _i++) {
        var rel = _a[_i];
        if (prefixes.length > 0 && !pathUnderPrefixes(rel, prefixes))
            continue;
        if (shouldSkipPathForUloc(repoRoot, rel))
            continue;
        var lang = classifyPathForMetrics(rel);
        if (!lang)
            continue;
        var fp = (0, node_path_1.join)(repoRoot, rel);
        if (!(0, node_fs_1.existsSync)(fp))
            continue;
        try {
            var st = (0, node_fs_1.statSync)(fp);
            if (!st.isFile() || st.size <= 0)
                continue;
            total += st.size;
        }
        catch (_b) {
            continue;
        }
    }
    return total;
}
function sortMetricLanguages(codeByLang, deltas) {
    var langs = new Set();
    for (var _i = 0, _a = Object.entries(codeByLang); _i < _a.length; _i++) {
        var _b = _a[_i], lang = _b[0], n = _b[1];
        if (n > 0)
            langs.add(lang);
    }
    for (var _c = 0, _d = deltas.keys(); _c < _d.length; _c++) {
        var lang = _d[_c];
        langs.add(lang);
    }
    return __spreadArray([], langs, true).sort(function (a, b) { var _a, _b; return ((_a = codeByLang[b]) !== null && _a !== void 0 ? _a : 0) - ((_b = codeByLang[a]) !== null && _b !== void 0 ? _b : 0) || a.localeCompare(b); });
}
function buildLangMetricsFromDeltas(codeByLang, deltas) {
    var _a, _b;
    var rows = [];
    for (var _i = 0, _c = sortMetricLanguages(codeByLang, deltas); _i < _c.length; _i++) {
        var lang = _c[_i];
        var d = (_a = deltas.get(lang)) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
        var code = (_b = codeByLang[lang]) !== null && _b !== void 0 ? _b : 0;
        if (code === 0 && d.edited === 0 && d.added === 0 && d.removed === 0)
            continue;
        rows.push({
            lang: lang,
            emoji: langMetricsEmoji(lang),
            code: code,
            edited: d.edited,
            added: d.added,
            removed: d.removed,
        });
    }
    return rows;
}
function buildMicroCommitMetricsFromDeltas(codeByLang, deltas) {
    return buildLangMetricsFromDeltas(codeByLang, deltas);
}
/** 📊️Builds uloc + size metrics for staged changes. */
function buildCommitMetrics(root, runner) {
    if (runner === void 0) { runner = createDefaultMetricsRunner(); }
    var repoRoot = gitRepoRoot(root);
    var ulocDeltas = accumulateStagedUlocDeltas(repoRoot);
    var sizeDeltas = accumulateStagedSizeDeltas(repoRoot);
    var ulocByLang = runner.countRepoUlocByLanguage(repoRoot);
    var sizeByLang = runner.countRepoSizeByLanguage(repoRoot);
    return {
        uloc: buildLangMetricsFromDeltas(ulocByLang, ulocDeltas),
        size: buildLangMetricsFromDeltas(sizeByLang, sizeDeltas),
    };
}
/** 📊️Builds uloc + size metrics for a git revision range (optional path prefixes). */
function buildCommitMetricsForRange(root, base, head, pathPrefixes, runner) {
    if (head === void 0) { head = "HEAD"; }
    if (runner === void 0) { runner = createDefaultMetricsRunner(); }
    var repoRoot = gitRepoRoot(root);
    var numstat = gitRangeNumstat(repoRoot, base, head);
    var ulocDeltas = accumulateUlocDeltasFromPaths(repoRoot, numstat, base, head, pathPrefixes);
    var sizeDeltas = accumulateRangeSizeDeltas(repoRoot, base, head, pathPrefixes);
    var ulocByLang = runner.countRepoUlocByLanguage(repoRoot);
    var sizeByLang = runner.countRepoSizeByLanguage(repoRoot);
    return {
        uloc: buildLangMetricsFromDeltas(ulocByLang, ulocDeltas),
        size: buildLangMetricsFromDeltas(sizeByLang, sizeDeltas),
    };
}
/** 📊️Builds micro-commit uloc metrics: repo uloc per language + staged git deltas. */
function buildMicroCommitMetrics(root, ulocRunner) {
    if (ulocRunner === void 0) { ulocRunner = createDefaultUlocRunner(); }
    var repoRoot = gitRepoRoot(root);
    var deltas = accumulateStagedUlocDeltas(repoRoot);
    var codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
    return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}
/** 📊️Builds uloc metrics for a git revision range (optional path prefixes). */
function buildMicroCommitMetricsForRange(root, base, head, pathPrefixes, ulocRunner) {
    if (head === void 0) { head = "HEAD"; }
    if (ulocRunner === void 0) { ulocRunner = createDefaultUlocRunner(); }
    var repoRoot = gitRepoRoot(root);
    var deltas = accumulateRangeUlocDeltas(repoRoot, base, head, pathPrefixes);
    var codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
    return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}
/** 📊️Micro-commit metrics block header (legacy alias). */
exports.MICRO_COMMIT_ULOC_HEADER = exports.COMMIT_METRIC_HEADER;
/** 🔢️Emoji for the aggregate total row (legacy alias). */
exports.MICRO_COMMIT_ULOC_TOTAL_EMOJI = exports.COMMIT_METRIC_TOTAL_EMOJI;
/** 📊️Sums per-language metric rows into one total. */
function sumMicroCommitLangMetrics(metrics) {
    var total = {
        lang: "Total",
        emoji: exports.COMMIT_METRIC_TOTAL_EMOJI,
        code: 0,
        edited: 0,
        added: 0,
        removed: 0,
    };
    for (var _i = 0, metrics_1 = metrics; _i < metrics_1.length; _i++) {
        var m = metrics_1[_i];
        total.code += m.code;
        total.edited += m.edited;
        total.added += m.added;
        total.removed += m.removed;
    }
    return total;
}
/** 📊️Formats one per-language metric row for a kind. */
function formatCommitMetricLine(kind, m) {
    return formatMetricBody({
        kind: kind,
        code: m.code,
        added: m.added,
        edited: m.edited,
        removed: m.removed,
        langEmoji: m.emoji,
        langSlug: langMetricsSlug(m.lang),
    });
}
/** 📊️Formats one per-language `📊️metric🦀️rust📃uloc…` row. */
function formatMicroCommitMetricLine(m) {
    return formatCommitMetricLine("uloc", m);
}
function commitMetricRowHasActivity(m) {
    return m.code > 0 || m.edited > 0 || m.added > 0 || m.removed > 0;
}
/** 📊️Renders commit metrics: repo totals per kind, then per-language uloc+size pairs. */
function formatCommitMetricsLines(bundle) {
    var uloc = bundle.uloc, size = bundle.size;
    if (uloc.length === 0 && size.length === 0)
        return [];
    var lines = [];
    var ulocTotal = sumMicroCommitLangMetrics(uloc);
    var sizeTotal = sumMicroCommitLangMetrics(size);
    if (commitMetricRowHasActivity(ulocTotal)) {
        lines.push(formatMetricBody({ kind: "uloc", code: ulocTotal.code, added: ulocTotal.added, edited: ulocTotal.edited, removed: ulocTotal.removed }));
    }
    if (commitMetricRowHasActivity(sizeTotal)) {
        lines.push(formatMetricBody({ kind: "size", code: sizeTotal.code, added: sizeTotal.added, edited: sizeTotal.edited, removed: sizeTotal.removed }));
    }
    var sizeByLang = new Map(size.map(function (m) { return [m.lang, m]; }));
    var langs = sortMetricLanguages(Object.fromEntries(uloc.map(function (m) { return [m.lang, m.code]; })), new Map(uloc.map(function (m) { return [m.lang, { edited: m.edited }]; })));
    var _loop_4 = function (lang) {
        var u = uloc.find(function (m) { return m.lang === lang; });
        var s = sizeByLang.get(lang);
        if (u && commitMetricRowHasActivity(u))
            lines.push(formatCommitMetricLine("uloc", u));
        if (s && commitMetricRowHasActivity(s))
            lines.push(formatCommitMetricLine("size", s));
    };
    for (var _i = 0, langs_1 = langs; _i < langs_1.length; _i++) {
        var lang = langs_1[_i];
        _loop_4(lang);
    }
    for (var _a = 0, size_1 = size; _a < size_1.length; _a++) {
        var s = size_1[_a];
        if (langs.includes(s.lang))
            continue;
        if (commitMetricRowHasActivity(s))
            lines.push(formatCommitMetricLine("size", s));
    }
    return lines;
}
/** 📊️Renders the unified LOC block (legacy: uloc-only footer). */
function formatMicroCommitMetricsLines(metrics) {
    return formatCommitMetricsLines({ uloc: metrics, size: [] });
}
/** ✅️Whether two git delta sums match on ➕️ ✏️ ➖️ (🟰️ follows). */
function gitDeltaSumsEqual(a, b) {
    return a.added === b.added && a.edited === b.edited && a.removed === b.removed;
}
/** 🚫️Per-language ➕️✏️➖️ must sum to the footer total line for each metric kind. */
function validateMicroCommitLangMetricsDeltaSum(metrics, kind) {
    if (kind === void 0) { kind = "uloc"; }
    if (metrics.length === 0)
        return;
    var total = sumMicroCommitLangMetrics(metrics);
    var added = 0;
    var edited = 0;
    var removed = 0;
    for (var _i = 0, metrics_2 = metrics; _i < metrics_2.length; _i++) {
        var m = metrics_2[_i];
        added += m.added;
        edited += m.edited;
        removed += m.removed;
    }
    if (!gitDeltaSumsEqual({ added: added, edited: edited, removed: removed }, total)) {
        var kindToken = exports.METRIC_KINDS[kind].token;
        throw new Error("commit: per-language ".concat(kindToken, " deltas do not sum to the footer total \u2014 languages \u2795\uFE0F").concat(added, "\u270F\uFE0F").concat(edited, "\u2796\uFE0F").concat(removed, "\uD83D\uDFF0\uFE0F").concat(gitDeltaLineTotal({ added: added, edited: edited, removed: removed }), " vs footer \u2795\uFE0F").concat(total.added, "\u270F\uFE0F").concat(total.edited, "\u2796\uFE0F").concat(total.removed, "\uD83D\uDFF0\uFE0F").concat(gitDeltaLineTotal(total)));
    }
}
/** 🚫️Per-language delta sums for uloc and size footers. */
function validateCommitMetricsDeltaSum(bundle) {
    validateMicroCommitLangMetricsDeltaSum(bundle.uloc, "uloc");
    validateMicroCommitLangMetricsDeltaSum(bundle.size, "size");
}
var EMOJI_PRESENTATION_SELECTOR_RE = /[\uFE0E\uFE0F]/g;
var COUNTER_RE = /^(.+🎆️[\uFE0E\uFE0F]?\d{2}🌙️[\uFE0E\uFE0F]?\d{2}☀️[\uFE0E\uFE0F]?\d{2})🚩️[\uFE0E\uFE0F]?(\d+)$/u;
var BUNDLE_TAG_RE = /^(.+🎆️[\uFE0E\uFE0F]?\d{2}🌙️[\uFE0E\uFE0F]?\d{2}☀️[\uFE0E\uFE0F]?\d{2})🚩️[\uFE0E\uFE0F]?$/u;
var NUMERIC_COUNTER_RE = /^(\d+)$/;
var TICKET_JSON_RE = /^\.🧬semio\/🦑️repo\/🎫️tickets\/.+\/🎫️ticket\.json$/;
function digestMicroCommitMessage(message) {
    return (0, node_crypto_1.createHash)("sha256").update(message.replace(/\r\n/g, "\n").trimEnd()).digest("hex");
}
function preparedDigestPath(root) {
    return (0, node_path_1.join)(gitDir(root), "compose-micro-commit-digest");
}
function preparedActivePath(root) {
    return (0, node_path_1.join)(gitDir(root), "compose-micro-commit-active");
}
function markPrepareActive(root) {
    (0, node_fs_1.writeFileSync)(preparedActivePath(root), "1\n");
}
function isPrepareActive(root) {
    return (0, node_fs_1.existsSync)(preparedActivePath(root));
}
var GK_TEMPLATE_BASENAME = "gkcommittemplate";
var GK_COMMIT_TEMPLATE_FILE = "".concat(GK_TEMPLATE_BASENAME, ".txt");
var MICRO_COMMIT_POST_WIPE_HOOKS = ["post-commit", "post-checkout", "post-merge", "post-rewrite"];
function safeGitEnv(extraEnv) {
    var _a;
    var env = {};
    for (var _i = 0, _b = Object.entries(process.env); _i < _b.length; _i++) {
        var _c = _b[_i], key = _c[0], value = _c[1];
        if (value !== undefined)
            env[key] = value;
    }
    Object.assign(env, extraEnv);
    var configuredGlobal = (_a = env.GIT_CONFIG_GLOBAL) === null || _a === void 0 ? void 0 : _a.trim();
    var globalConfig = configuredGlobal || (0, node_path_1.join)((0, node_os_1.homedir)(), ".gitconfig");
    try {
        if (configuredGlobal && !(0, node_fs_1.existsSync)(globalConfig))
            throw new Error("missing configured global Git config");
        if ((0, node_fs_1.existsSync)(globalConfig))
            (0, node_fs_1.readFileSync)(globalConfig, "utf8");
    }
    catch (_d) {
        env.GIT_CONFIG_GLOBAL = node_os_1.devNull;
    }
    return env;
}
/** 🌳️Git subprocess env with explicit `cwd` — ignores inherited `GIT_DIR` / `GIT_WORK_TREE`. */
function gitSpawnEnv() {
    var env = safeGitEnv();
    delete env.GIT_DIR;
    delete env.GIT_WORK_TREE;
    return env;
}
var GIT_INDEX_LOCK_RE = /Unable to create '.*index\.lock'/;
var GIT_INDEX_LOCK_MAX_ATTEMPTS = 20;
var GIT_INDEX_LOCK_RETRY_DELAY_MS = 300;
/** ⏳️Retries on a concurrent `index.lock` (other agent/IDE git process mid-write) instead of failing the whole commit. */
function git(root, args) {
    for (var attempt = 1; attempt <= GIT_INDEX_LOCK_MAX_ATTEMPTS; attempt++) {
        var r = spawnCapturedSync("git", args, { cwd: root, env: gitSpawnEnv() });
        if (r.status === 0)
            return { ok: true, out: r.stdout.toString("utf8").trim() };
        var out = (r.stderr.length > 0 ? r.stderr : r.stdout).toString("utf8").trim();
        if (attempt < GIT_INDEX_LOCK_MAX_ATTEMPTS && GIT_INDEX_LOCK_RE.test(out)) {
            Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, GIT_INDEX_LOCK_RETRY_DELAY_MS);
            continue;
        }
        return { ok: false, out: out };
    }
    return { ok: false, out: "git: index.lock retry budget exhausted" };
}
function gitCachedNames(root, extra) {
    if (extra === void 0) { extra = []; }
    var r = spawnCapturedSync("git", __spreadArray(["diff", "--cached", "--name-only", "-z"], extra, true), { cwd: root, env: gitSpawnEnv() });
    if (r.status !== 0)
        return [];
    var raw = r.stdout.toString("utf8");
    if (!raw)
        return [];
    return raw.split("\0").filter(Boolean);
}
function currentBranch(root) {
    var result = git(root, ["branch", "--show-current"]);
    if (!result.ok)
        return { ok: false, error: result.out || "git branch --show-current failed" };
    return { ok: true, name: result.out };
}
function branchValidationError(command, branch) {
    if (!branch.ok)
        return "".concat(command, ": cannot read current branch: ").concat(branch.error);
    if (!branch.name)
        return "".concat(command, ": detached HEAD; switch to a branch containing \u26F3\uFE0Fwip or \uD83C\uDFD7\uFE0Fdev");
    var normalizedBranch = normalizeEmojiPresentation(branch.name);
    var wipMarker = normalizeEmojiPresentation("⛳️wip");
    var devMarker = normalizeEmojiPresentation("🏗️dev");
    if (!normalizedBranch.includes(wipMarker) && !normalizedBranch.includes(devMarker)) {
        return "".concat(command, ": current branch \"").concat(branch.name, "\" must contain \u26F3\uFE0Fwip or \uD83C\uDFD7\uFE0Fdev");
    }
    return null;
}
function branchAllowed(root) {
    return branchValidationError("micro-commit", currentBranch(root)) === null;
}
function gitEmail(root) {
    return git(root, ["config", "user.email"]).out;
}
function findContributor(root) {
    var _a;
    var email = gitEmail(root).toLowerCase();
    var dir = (0, node_path_1.join)(getRepoMetaDir(root), "🧑️‍💻️devs");
    if (!(0, node_fs_1.existsSync)(dir))
        return null;
    if (email) {
        for (var _i = 0, _b = (0, node_fs_1.readdirSync)(dir, { withFileTypes: true }); _i < _b.length; _i++) {
            var name_4 = _b[_i];
            if (!name_4.isDirectory())
                continue;
            var path = (0, node_path_1.join)(dir, name_4.name, "🧑️‍💻️contributor.json");
            if (!(0, node_fs_1.existsSync)(path))
                continue;
            var c = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
            var emails = __spreadArray([c.email], ((_a = c.emails) !== null && _a !== void 0 ? _a : []), true).filter(function (e) { return typeof e === "string" && e.length > 0; }).map(function (e) { return e.toLowerCase(); });
            if (emails.includes(email))
                return c;
        }
    }
    var branchName = git(root, ["branch", "--show-current"]).out;
    if (branchName) {
        for (var _c = 0, _d = (0, node_fs_1.readdirSync)(dir, { withFileTypes: true }); _c < _d.length; _c++) {
            var name_5 = _d[_c];
            if (!name_5.isDirectory())
                continue;
            var path = (0, node_path_1.join)(dir, name_5.name, "🧑️‍💻️contributor.json");
            if (!(0, node_fs_1.existsSync)(path))
                continue;
            var c = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
            if (branchName.includes(c.alias) || (c.emoji && branchName.includes(c.emoji))) {
                if (!email && c.email) {
                    git(root, ["config", "--local", "user.email", c.email]);
                    if (c.name)
                        git(root, ["config", "--local", "user.name", c.name]);
                }
                return c;
            }
        }
    }
    return null;
}
function loadLevel(root, contributor, segments) {
    var token = segments.join(" ").toLowerCase();
    if (/\b(gp|gpush|push!|\+push)\b/.test(token))
        return "prepare-and-commit-and-push";
    if (/\b(gc|commit!|\+commit)\b/.test(token))
        return "prepare-and-commit";
    if (/\b(g\.|gprepare|prepare!|\+prepare)\b/.test(token))
        return "prepare-only";
    var path = (0, node_path_1.join)(getRepoMetaDir(root), "🧑️‍💻️devs", contributor.alias, "micro-commit.json");
    if ((0, node_fs_1.existsSync)(path)) {
        var j = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
        if (j.level === "prepare-and-commit" || j.level === "prepare-and-commit-and-push" || j.level === "prepare-only") {
            return j.level;
        }
    }
    return "prepare-only";
}
function pad2(n) {
    return String(n).padStart(2, "0");
}
function pad3(n) {
    return String(n).padStart(3, "0");
}
var COUNTER_LOG_DEPTH = 40;
/** 🎨️Normalizes emoji text and emoji presentation variants for stable identity comparisons. */
function normalizeEmojiPresentation(value) {
    return value.replace(EMOJI_PRESENTATION_SELECTOR_RE, "");
}
/** 🧭️Renders a historical WIP epoch with the contributor's current canonical emoji presentation. */
function canonicalWipLine1Base(line1Base, contributor) {
    var match = /🎆️(\d{2})🌙️(\d{2})☀️(\d{2})$/u.exec(normalizeEmojiPresentation(line1Base));
    if (!match)
        return line1Base;
    return "".concat(contributor.emoji).concat(contributor.alias, "\uD83C\uDF86\uFE0F").concat(match[1], "\uD83C\uDF19\uFE0F").concat(match[2], "\u2600\uFE0F").concat(match[3]);
}
/** 🔢️Reads micro-commit counter from subject line `…🚩️NNN`. */
function extractCounterFromSubject(subject) {
    var s = subject.trim();
    var formatted = COUNTER_RE.exec(s);
    if (!formatted)
        return null;
    return { nnn: Number.parseInt(formatted[2], 10), line1Base: formatted[1] };
}
/** 🔢️Reads GitKraken numeric-only subjects (`152`, `299`, …). */
function extractNumericCounterFromSubject(subject) {
    var m = NUMERIC_COUNTER_RE.exec(subject.trim());
    if (!m)
        return null;
    var n = Number.parseInt(m[1], 10);
    return Number.isFinite(n) && n > 0 ? n : null;
}
/** 🏷️Reads WIP epoch base from a bundle squash tag (`…🎆️YY🌙️MM☀️DD🚩️`). */
function line1BaseFromBundleTag(tag) {
    var m = BUNDLE_TAG_RE.exec(tag.trim());
    return m ? m[1] : null;
}
/** 🎆️Resolves the persisted WIP epoch for line 1 from formatted history or the latest bundle tag. */
function contributorWipLine1Base(root, contributor) {
    var prefix = "".concat(contributor.emoji).concat(contributor.alias);
    var normalizedPrefix = normalizeEmojiPresentation(prefix);
    var log = git(root, ["log", "--format=%s", "-1000"]).out;
    for (var _i = 0, _a = log ? log.split("\n") : []; _i < _a.length; _i++) {
        var subject = _a[_i];
        var hit = extractCounterFromSubject(subject);
        if (hit && normalizeEmojiPresentation(hit.line1Base).startsWith(normalizedPrefix))
            return hit.line1Base;
    }
    var tags = git(root, ["tag", "-l", "--sort=-creatordate"]).out;
    for (var _b = 0, _c = tags ? tags.split("\n") : []; _b < _c.length; _b++) {
        var tag = _c[_b];
        var base = line1BaseFromBundleTag(tag);
        if (base && normalizeEmojiPresentation(base).startsWith(normalizedPrefix))
            return base;
    }
    return null;
}
/** 🎆️Bumps counter from recent `…🚩️NNN` or numeric GitKraken subjects (newest first). */
function bumpCounterFromHistory(subjectsNewestFirst, contributor, now, wipLine1Base) {
    var _a;
    if (now === void 0) { now = new Date(); }
    if (wipLine1Base === void 0) { wipLine1Base = null; }
    var yy = pad2(now.getFullYear() % 100);
    var mm = pad2(now.getMonth() + 1);
    var dd = pad2(now.getDate());
    var fresh = "".concat(contributor.emoji).concat(contributor.alias, "\uD83C\uDF86\uFE0F").concat(yy, "\uD83C\uDF19\uFE0F").concat(mm, "\u2600\uFE0F").concat(dd);
    var contributorPrefix = normalizeEmojiPresentation("".concat(contributor.emoji).concat(contributor.alias));
    var max = 0;
    var line1Base = null;
    for (var _i = 0, subjectsNewestFirst_1 = subjectsNewestFirst; _i < subjectsNewestFirst_1.length; _i++) {
        var subject = subjectsNewestFirst_1[_i];
        var hit = extractCounterFromSubject(subject);
        if (hit) {
            if (!normalizeEmojiPresentation(hit.line1Base).startsWith(contributorPrefix))
                continue;
            max = Math.max(max, hit.nnn);
            if (!line1Base)
                line1Base = hit.line1Base;
            continue;
        }
        var numeric = extractNumericCounterFromSubject(subject);
        if (numeric !== null)
            max = Math.max(max, numeric);
    }
    var epoch = canonicalWipLine1Base((_a = line1Base !== null && line1Base !== void 0 ? line1Base : wipLine1Base) !== null && _a !== void 0 ? _a : fresh, contributor);
    if (max > 0)
        return { line1Base: epoch, nnn: pad3(max + 1) };
    var unparsedPrior = subjectsNewestFirst.find(function (subject) {
        var normalized = normalizeEmojiPresentation(subject.trim());
        var epochPrefix = normalizeEmojiPresentation("".concat(contributor.emoji).concat(contributor.alias, "\uD83C\uDF86\uFE0F"));
        return normalized.startsWith(epochPrefix) && /🚩\d+$/u.test(normalized) && !extractCounterFromSubject(subject);
    });
    if (unparsedPrior)
        throw new Error("micro-commit: refusing to reset counter to 001 because prior subject could not be parsed: ".concat(unparsedPrior));
    return { line1Base: epoch, nnn: "001" };
}
function bumpCounterFromSubject(subject, contributor, now) {
    if (now === void 0) { now = new Date(); }
    return bumpCounterFromHistory([subject], contributor, now);
}
function nextCounter(root, contributor) {
    var log = git(root, ["log", "--format=%s", "-".concat(COUNTER_LOG_DEPTH)]).out;
    var subjects = log ? log.split("\n").filter(Boolean) : [];
    return bumpCounterFromHistory(subjects, contributor, new Date(), contributorWipLine1Base(root, contributor));
}
function formatSecond(now) {
    var yy = pad2(now.getFullYear() % 100);
    var mm = pad2(now.getMonth() + 1);
    var dd = pad2(now.getDate());
    var hh = pad2(now.getHours());
    var min = pad2(now.getMinutes());
    var ss = pad2(now.getSeconds());
    return "\uD83C\uDF86\uFE0F".concat(yy, "\uD83C\uDF19\uFE0F").concat(mm, "\u2600\uFE0F").concat(dd, "\u23F0\uFE0F").concat(hh, "\u231A\uFE0F").concat(min, "\u23F1\uFE0F").concat(ss);
}
function preparedBulletsPath(root) {
    return (0, node_path_1.join)(gitDir(root), "compose-micro-commit-bullets");
}
var EMOJI_LEAD_RE = /^((?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+)/u;
var MICRO_COMMIT_BULLET_RE = /^(?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+\S/u;
var TIMESTAMP_LINE_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}/u;
var RESERVED_BULLET_LEAD_EMOJIS = new Set(["🎆️", "📊️", "🔢️", "🚩️"]);
/** 🏷️Leading emoji grapheme on a bullet line, or "" if none. */
function bulletLeadEmoji(line) {
    var _a;
    var m = EMOJI_LEAD_RE.exec(line.trim());
    return (_a = m === null || m === void 0 ? void 0 : m[1]) !== null && _a !== void 0 ? _a : "";
}
/** 📝️Formats one bullet as `{emoji}{description}` (line starts with emoji, no leading `-`). */
function formatMicroCommitBulletLine(line) {
    var body = line.trim().replace(/^-+\s*/, "");
    return body.replace(new RegExp("^".concat(EMOJI_LEAD_RE.source, "\\s+"), "u"), "$1");
}
var MICRO_COMMIT_METRIC_ROW_RE = /^📊️metric/u;
function isCommitMetricLine(line) {
    return MICRO_COMMIT_METRIC_ROW_RE.test(line.trim());
}
function isMicroCommitUlocLine(line) {
    return isCommitMetricLine(line);
}
/** 📝️Normalizes LLM-authored bullet lines to `{emoji}{description}`. */
function normalizeBulletLines(text) {
    return text
        .split("\n")
        .map(function (l) { return l.trim(); })
        .filter(function (l) { return l.length > 0 && !l.startsWith("#") && !isMicroCommitUlocLine(l); })
        .map(formatMicroCommitBulletLine)
        .filter(function (l) { return l.length > 1; })
        .slice(0, 8);
}
function validateBulletSpacing(bullets) {
    for (var _i = 0, bullets_1 = bullets; _i < bullets_1.length; _i++) {
        var b = bullets_1[_i];
        if (MICRO_COMMIT_BULLET_RE.test(b))
            continue;
        console.error("micro-commit: bullet must start with {emoji} then description (no '-' prefix, no space after emoji): ".concat(b));
        process.exit(1);
    }
}
/** 🚫️Returns an error when a bullet uses reserved or timestamp emojis. */
function bulletEmojiValidationError(bullets) {
    for (var _i = 0, bullets_2 = bullets; _i < bullets_2.length; _i++) {
        var b = bullets_2[_i];
        var lead = bulletLeadEmoji(b);
        if (RESERVED_BULLET_LEAD_EMOJIS.has(lead)) {
            return "micro-commit: ".concat(lead, " is reserved for subject/timestamp/uloc \u2014 start each bullet with the emoji that best matches that line's description");
        }
        if (TIMESTAMP_LINE_RE.test(b.trim())) {
            return "micro-commit: bullet must not copy the 🎆️YY🌙️MM☀️DD timestamp pattern — use one leading emoji that fits the change, not the calendar line";
        }
    }
    return null;
}
function validateBulletEmojis(bullets) {
    var err = bulletEmojiValidationError(bullets);
    if (err) {
        console.error(err);
        process.exit(1);
    }
}
function writePreparedBullets(root, bullets) {
    (0, node_fs_1.writeFileSync)(preparedBulletsPath(root), "".concat(bullets.join("\n"), "\n"));
}
function readPreparedBullets(root) {
    var path = preparedBulletsPath(root);
    if (!(0, node_fs_1.existsSync)(path))
        return [];
    return normalizeBulletLines((0, node_fs_1.readFileSync)(path, "utf8"));
}
var GIT_COMMIT_DRAFT_FILES = ["COMMIT_EDITMSG", "MERGE_MSG", "SQUASH_MSG"];
var STAGED_CHANGE_AREAS = [
    { id: ".cursor/plans", match: function (p) { return p.includes(".cursor/plans/"); }, keywords: ["plan"] },
    { id: ".agents", match: function (p) { return p.includes(".agents/") && !p.endsWith("SKILL.md"); }, keywords: ["skill", "agent"] },
    { id: "repo", match: function (p) { return p.includes("repo/"); }, keywords: ["hook", "micro-commit"] },
    { id: ".devcontainer", match: function (p) { return p.includes(".devcontainer/"); }, keywords: ["devcontainer"] },
    {
        id: "product",
        match: function (p) { return /(^|\/)(framework|puzzle|compose|cad|ui|mathematical|infinite|elements|coda|reuse|s)\//.test(p.replace(/^[^\w./-]+/, "")); },
        keywords: [],
    },
];
function isInsignificantStagedPath(path) {
    return /\/micro-commit\.ts$/.test(path) || /\/index\.test\.ts$/.test(path) || path.endsWith("SKILL.md");
}
/** 🔤️Path tokens used to check whether bullets mention a staged file. */
function pathTokensForBulletCoverage(filePath) {
    return __spreadArray([], new Set(filePath
        .toLowerCase()
        .split(/[/._-]+/)
        .filter(function (s) { return s.length >= 4; })), true);
}
function bulletsMentionPathTokens(text, paths) {
    var tokens = paths.flatMap(pathTokensForBulletCoverage);
    return tokens.some(function (t) { return text.includes(t); });
}
function bulletsCoverArea(text, paths, keywords) {
    if (keywords.some(function (k) { return text.includes(k); }))
        return true;
    return bulletsMentionPathTokens(text, paths);
}
/** 🧪️Returns staged area ids not reflected in bullets (empty = ok). */
function uncoveredStagedAreas(bullets, staged) {
    var significant = staged.filter(function (p) { return !isInsignificantStagedPath(p); });
    if (significant.length === 0)
        return [];
    var text = bullets.join("\n").toLowerCase();
    var missed = [];
    var matched = new Set();
    var _loop_5 = function (area) {
        var files = significant.filter(function (p) {
            if (!area.match(p))
                return false;
            matched.add(p);
            return true;
        });
        if (files.length === 0)
            return "continue";
        if (!bulletsCoverArea(text, files, area.keywords))
            missed.push(area.id);
    };
    for (var _i = 0, STAGED_CHANGE_AREAS_1 = STAGED_CHANGE_AREAS; _i < STAGED_CHANGE_AREAS_1.length; _i++) {
        var area = STAGED_CHANGE_AREAS_1[_i];
        _loop_5(area);
    }
    var other = significant.filter(function (p) { return !matched.has(p); });
    if (other.length > 0 && !bulletsMentionPathTokens(text, other))
        missed.push("other staged paths");
    return missed;
}
function validateBulletsAgainstStaged(bullets, staged) {
    var missed = uncoveredStagedAreas(bullets, staged);
    if (missed.length === 0)
        return;
    console.error("micro-commit: bullets must cover every staged area \u2014 missing: ".concat(missed.join(", ")));
    console.error("micro-commit: read `micro-commit diff` again (include .cursor/plans, product code, repo, …)");
    for (var _i = 0, staged_1 = staged; _i < staged_1.length; _i++) {
        var p = staged_1[_i];
        console.error("  ".concat(p));
    }
    process.exit(1);
}
function readDiffBulletsInput(root, bulletsFile) {
    if (bulletsFile) {
        var path = bulletsFile.startsWith("/") ? bulletsFile : (0, node_path_1.join)(root, bulletsFile);
        return normalizeBulletLines((0, node_fs_1.readFileSync)(path, "utf8"));
    }
    if (!process.stdin.isTTY) {
        try {
            return normalizeBulletLines((0, node_fs_1.readFileSync)(0, "utf8"));
        }
        catch (_a) {
            // ignore errors and fall through to empty
        }
    }
    return [];
}
function listCachedPaths(root) {
    return gitCachedNames(root);
}
function listAddedTicketPaths(root) {
    return gitCachedNames(root, ["--diff-filter=A"]).filter(function (p) { return TICKET_JSON_RE.test(p); });
}
function ticketBullets(root) {
    var bullets = [];
    for (var _i = 0, _a = listAddedTicketPaths(root); _i < _a.length; _i++) {
        var rel = _a[_i];
        var path = (0, node_path_1.join)(root, rel);
        try {
            var t = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
            if (!t.emoji || !t.title)
                continue;
            bullets.push("".concat(t.emoji).concat(t.title));
        }
        catch (_b) {
            /* ignore invalid JSON ticket files */
        }
    }
    return bullets;
}
function buildMicroCommitMessage(root, contributor, diffBullets, metricsRunner) {
    if (diffBullets === void 0) { diffBullets = []; }
    root = gitRepoRoot(root);
    var _a = nextCounter(root, contributor), line1Base = _a.line1Base, nnn = _a.nnn;
    var now = new Date();
    var authored = diffBullets.length > 0 ? normalizeBulletLines(diffBullets.join("\n")) : readPreparedBullets(root);
    var tickets = ticketBullets(root);
    var authoredLower = new Set(authored.map(function (b) { return b.toLowerCase(); }));
    var bullets = __spreadArray(__spreadArray([], authored, true), tickets.filter(function (t) { return !authoredLower.has(t.toLowerCase()); }), true).slice(0, 8);
    if (bullets.length === 0) {
        throw new Error("micro-commit: at least one description bullet is required");
    }
    var runner = metricsRunner && "countRepoUlocByLanguage" in metricsRunner ? metricsRunner : metricsRunnerFromUloc(metricsRunner);
    var metricBundle = buildCommitMetrics(root, runner);
    if (metricBundle.uloc.length === 0 && metricBundle.size.length === 0) {
        throw new Error("micro-commit: required 📊️metric footer could not be built because no language metrics were collected");
    }
    validateCommitMetricsDeltaSum(metricBundle);
    var metrics = formatCommitMetricsLines(metricBundle);
    var lines = __spreadArray(["".concat(line1Base, "\uD83D\uDEA9\uFE0F").concat(nnn), formatSecond(now)], bullets, true);
    lines.push.apply(lines, __spreadArray([""], metrics, false));
    lines.push("", "Signed-off-by: ".concat(contributor.name, " <").concat(contributor.email, ">"));
    return "".concat(lines.join("\n"), "\n");
}
function writeMicroCommitTemplates(root, message) {
    var dir = gitDir(root);
    var gkCommitTemplate = (0, node_path_1.join)(dir, GK_COMMIT_TEMPLATE_FILE);
    removeGitKrakenTemplateFiles(root);
    (0, node_fs_1.writeFileSync)(gkCommitTemplate, message);
    for (var _i = 0, GIT_COMMIT_DRAFT_FILES_1 = GIT_COMMIT_DRAFT_FILES; _i < GIT_COMMIT_DRAFT_FILES_1.length; _i++) {
        var name_6 = GIT_COMMIT_DRAFT_FILES_1[_i];
        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(dir, name_6), message);
    }
    git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
    (0, node_fs_1.writeFileSync)(preparedDigestPath(root), "".concat(digestMicroCommitMessage(message), "\n"));
    markPrepareActive(root);
}
function shouldRefreshPreparedCommitMessage(current, preparedDigest) {
    var trimmed = current.trim();
    if (!trimmed)
        return true;
    if (!preparedDigest)
        return false;
    return digestMicroCommitMessage(current) === preparedDigest.trim();
}
function removeGitDirPrefixed(root, prefix) {
    var dir = gitDir(root);
    for (var _i = 0, _a = (0, node_fs_1.readdirSync)(dir); _i < _a.length; _i++) {
        var name_7 = _a[_i];
        if (name_7.startsWith(prefix)) {
            try {
                (0, node_fs_1.rmSync)((0, node_path_1.join)(dir, name_7), { force: true });
            }
            catch (_b) {
                /* ignore */
            }
        }
    }
}
function removeGitKrakenTemplateFiles(root) {
    var dir = gitDir(root);
    for (var _i = 0, _a = (0, node_fs_1.readdirSync)(dir); _i < _a.length; _i++) {
        var name_8 = _a[_i];
        if (!name_8.startsWith(GK_TEMPLATE_BASENAME))
            continue;
        try {
            (0, node_fs_1.rmSync)((0, node_path_1.join)(dir, name_8), { force: true });
        }
        catch (_b) {
            /* ignore */
        }
    }
}
function resetGitCommitTemplateState(root) {
    var dir = gitDir(root);
    removeGitKrakenTemplateFiles(root);
    var gkCommitTemplate = (0, node_path_1.join)(dir, GK_COMMIT_TEMPLATE_FILE);
    (0, node_fs_1.writeFileSync)(gkCommitTemplate, "");
    git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
}
/** 🧹️Clears GitKraken templates, git draft messages, and micro-commit prepare state. */
function clearGitCommitDraftState(root) {
    var dir = gitDir(root);
    resetGitCommitTemplateState(root);
    for (var _i = 0, GIT_COMMIT_DRAFT_FILES_2 = GIT_COMMIT_DRAFT_FILES; _i < GIT_COMMIT_DRAFT_FILES_2.length; _i++) {
        var name_9 = GIT_COMMIT_DRAFT_FILES_2[_i];
        try {
            (0, node_fs_1.writeFileSync)((0, node_path_1.join)(dir, name_9), "");
        }
        catch (_a) {
            /* ignore */
        }
    }
    removeGitDirPrefixed(root, "compose-micro-commit");
}
/** 🧹️Resets GK/git commit-template state without wiping the active draft message file (COMMIT_EDITMSG/MERGE_MSG/SQUASH_MSG). */
function clearMicroCommitTemplatesOnly(root) {
    resetGitCommitTemplateState(root);
    removeGitDirPrefixed(root, "compose-micro-commit");
}
function clearStaleTemplatesBeforePrepare(root) {
    if (!isPrepareActive(root))
        clearGitCommitDraftState(root);
}
/** 🧹️Removes prepare state and resets GK/git templates to empty after a commit. */
function wipeAfterCommit(root) {
    clearGitCommitDraftState(root);
}
function handlePrepareCommitMsg(root, msgFile, source) {
    if (!isPrepareActive(root)) {
        clearMicroCommitTemplatesOnly(root);
        return;
    }
    if (!branchAllowed(root))
        return;
    var contributor = findContributor(root);
    if (!contributor)
        return;
    if (source === "merge" || source === "squash")
        return;
    var preparedBullets = readPreparedBullets(root);
    var newTickets = listAddedTicketPaths(root);
    if (preparedBullets.length === 0 && newTickets.length === 0) {
        var current_1 = (0, node_fs_1.existsSync)(msgFile) ? (0, node_fs_1.readFileSync)(msgFile, "utf8") : "";
        if (current_1.trim())
            return;
        return;
    }
    var digestPath = preparedDigestPath(root);
    var preparedDigest = (0, node_fs_1.existsSync)(digestPath) ? (0, node_fs_1.readFileSync)(digestPath, "utf8") : null;
    var current = (0, node_fs_1.existsSync)(msgFile) ? (0, node_fs_1.readFileSync)(msgFile, "utf8") : "";
    if (!shouldRefreshPreparedCommitMessage(current, preparedDigest))
        return;
    var message = buildMicroCommitMessage(root, contributor, preparedBullets);
    (0, node_fs_1.writeFileSync)(msgFile, message);
    writeMicroCommitTemplates(root, message);
}
var MICRO_COMMIT_BUN_PIN = "compose-micro-commit-bun";
/** 🥖️Resolves the Bun executable for git hooks (GUI git often has a minimal PATH). */
function resolveMicroCommitBunBin(root) {
    var _a, _b, _c, _d, _e, _f;
    var fromEnv = (_a = process.env.COMPOSE_BUN) === null || _a === void 0 ? void 0 : _a.trim();
    if (fromEnv)
        return fromEnv;
    var argv0 = (_b = process.argv[0]) !== null && _b !== void 0 ? _b : "";
    if (/bun(\.exe)?$/i.test(argv0))
        return argv0;
    var win = process.platform === "win32";
    var home = (_d = (_c = process.env.HOME) !== null && _c !== void 0 ? _c : process.env.USERPROFILE) !== null && _d !== void 0 ? _d : "";
    var bunInstall = (_e = process.env.BUN_INSTALL) !== null && _e !== void 0 ? _e : (0, node_path_1.join)(home, ".bun");
    var candidates = [(0, node_path_1.join)(root, "node_modules", ".bin", win ? "bun.cmd" : "bun"), (0, node_path_1.join)(root, "node_modules", ".bin", "bun.exe"), (0, node_path_1.join)(bunInstall, "bin", win ? "bun.exe" : "bun"), (0, node_path_1.join)(bunInstall, "bin", "bun")];
    for (var _i = 0, candidates_1 = candidates; _i < candidates_1.length; _i++) {
        var c = candidates_1[_i];
        if (c && (0, node_fs_1.existsSync)(c))
            return c;
    }
    var which = (0, node_child_process_1.spawnSync)(win ? "where" : "which", ["bun"], { encoding: "utf8", shell: win });
    if (which.status === 0) {
        var first = ((_f = which.stdout) !== null && _f !== void 0 ? _f : "")
            .split(/\r?\n/)
            .map(function (l) { return l.trim(); })
            .find(Boolean);
        if (first && (0, node_fs_1.existsSync)(first))
            return first;
    }
    return win ? "bun.exe" : "bun";
}
var MICRO_COMMIT_SEED_EMPTY_GK_SH = "compose_micro_commit_seed_empty_gk() {\n  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0\n  GK_TEMPLATE=\"$GIT_DIR/".concat(GK_COMMIT_TEMPLATE_FILE, "\"\n  if [ -d \"$GIT_DIR\" ]; then\n    for f in \"$GIT_DIR\"/gkcommittemplate*; do\n      [ -e \"$f\" ] || continue\n      rm -f \"$f\" 2>/dev/null || true\n    done\n  fi\n  : >\"$GK_TEMPLATE\" 2>/dev/null || true\n  git config --local commit.template \"$GK_TEMPLATE\" 2>/dev/null || true\n}");
var MICRO_COMMIT_DISABLE_PREPARE_SH = "".concat(MICRO_COMMIT_SEED_EMPTY_GK_SH, "\ncompose_micro_commit_disable_prepare() {\n  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0\n  for f in \"$GIT_DIR\"/compose-micro-commit-*; do\n    [ -e \"$f\" ] || continue\n    rm -f \"$f\" 2>/dev/null || true\n  done\n  compose_micro_commit_seed_empty_gk\n}");
var MICRO_COMMIT_WIPE_FULL_SH = "".concat(MICRO_COMMIT_SEED_EMPTY_GK_SH, "\ncompose_micro_commit_wipe() {\n  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0\n  compose_micro_commit_seed_empty_gk\n  for msg in COMMIT_EDITMSG MERGE_MSG SQUASH_MSG; do\n    if [ -f \"$GIT_DIR/$msg\" ]; then\n      : >\"$GIT_DIR/$msg\" 2>/dev/null || true\n    fi\n  done\n  if [ -d \"$GIT_DIR\" ]; then\n    for f in \"$GIT_DIR\"/compose-micro-commit-*; do\n      [ -e \"$f\" ] || continue\n      rm -f \"$f\" 2>/dev/null || true\n    done\n  fi\n}");
var MICRO_COMMIT_RESOLVE_REPO_CLI_SH = "compose_resolve_repo_cli() {\n  ROOT=\"$1\"\n  if [ -n \"$REPO_CLI_BIN\" ] && [ -x \"$REPO_CLI_BIN\" ]; then\n    echo \"$REPO_CLI_BIN\"\n    return\n  fi\n  if [ -x \"$ROOT/\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCBB\uFE0Fclient/client\" ]; then\n    echo \"$ROOT/\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCBB\uFE0Fclient/client\"\n    return\n  fi\n  if [ -x \"$ROOT/\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCBB\uFE0Fclient/client.exe\" ]; then\n    echo \"$ROOT/\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCBB\uFE0Fclient/client.exe\"\n    return\n  fi\n}";
var MICRO_COMMIT_RESOLVE_BUN_SH = "compose_resolve_bun() {\n  ROOT=\"$1\"\n  if [ -n \"$COMPOSE_BUN\" ] && [ -x \"$COMPOSE_BUN\" ]; then\n    echo \"$COMPOSE_BUN\"\n    return\n  fi\n  BUN_PIN=\"$ROOT/.\uD83E\uDDECsemio/\uD83E\uDD91\uFE0Frepo/".concat(MICRO_COMMIT_BUN_PIN, "\"\n  if [ -f \"$BUN_PIN\" ]; then\n    BUN=$(sed -n '1p' \"$BUN_PIN\" 2>/dev/null)\n    if [ -n \"$BUN\" ] && [ -x \"$BUN\" ]; then\n      echo \"$BUN\"\n      return\n    fi\n  fi\n  command -v bun 2>/dev/null || true\n}");
/** 🪝️Renders a portable `sh` git hook (LF, inline wipe; repo client binary for micro-commit). */
function renderMicroCommitGitHook(name) {
    var isPostWipe = MICRO_COMMIT_POST_WIPE_HOOKS.includes(name);
    var lines = [
        "#!/usr/bin/env sh",
        isPostWipe ? MICRO_COMMIT_WIPE_FULL_SH : MICRO_COMMIT_DISABLE_PREPARE_SH,
        "ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0",
        'cd "$ROOT" || exit 0',
        "GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || exit 0",
        MICRO_COMMIT_RESOLVE_REPO_CLI_SH,
        MICRO_COMMIT_RESOLVE_BUN_SH,
    ];
    if (isPostWipe) {
        lines.push('CLI=$(compose_resolve_repo_cli "$ROOT")', '[ -n "$CLI" ] && "$CLI" micro-commit reset 2>/dev/null || true', "compose_micro_commit_wipe", "exit 0");
    }
    else {
        lines.push('[ ! -f "$GIT_DIR/compose-micro-commit-active" ] && {', "  compose_micro_commit_seed_empty_gk", "  exit 0", "}", 'BUN=$(compose_resolve_bun "$ROOT")', '[ -z "$BUN" ] && { compose_micro_commit_disable_prepare; exit 0; }', '"$BUN" "$ROOT/📜️script.ts" micro-commit prepare-commit-msg "$1" "$2" 2>/dev/null || compose_micro_commit_disable_prepare', "exit 0");
    }
    return "".concat(lines.join("\n"), "\n");
}
function writeMicroCommitHookFile(path, body) {
    (0, node_fs_1.writeFileSync)(path, body.replace(/\r\n/g, "\n"), "utf8");
    try {
        (0, node_fs_1.chmodSync)(path, 493);
    }
    catch (_a) {
        /* windows */
    }
}
function installMicroCommitGitHooks(root) {
    var bunBin = resolveMicroCommitBunBin(root).replace(/\r/g, "");
    (0, node_fs_1.mkdirSync)(getRepoMetaDir(root), { recursive: true });
    (0, node_fs_1.writeFileSync)((0, node_path_1.join)(getRepoMetaDir(root), MICRO_COMMIT_BUN_PIN), "".concat(bunBin, "\n"), "utf8");
    var hooksDir = (0, node_path_1.join)(root, ".git", "hooks");
    var repoHooksDir = (0, node_path_1.join)(root, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks");
    (0, node_fs_1.mkdirSync)(hooksDir, { recursive: true });
    (0, node_fs_1.mkdirSync)(repoHooksDir, { recursive: true });
    for (var _i = 0, _a = __spreadArray(__spreadArray([], MICRO_COMMIT_POST_WIPE_HOOKS, true), ["prepare-commit-msg"], false); _i < _a.length; _i++) {
        var name_10 = _a[_i];
        var body = renderMicroCommitGitHook(name_10);
        writeMicroCommitHookFile((0, node_path_1.join)(repoHooksDir, name_10), body);
        writeMicroCommitHookFile((0, node_path_1.join)(hooksDir, name_10), body);
    }
    var stalePreCommit = (0, node_path_1.join)(hooksDir, "pre-commit");
    if ((0, node_fs_1.existsSync)(stalePreCommit))
        (0, node_fs_1.rmSync)(stalePreCommit, { force: true });
    var repoPreCommit = (0, node_path_1.join)(repoHooksDir, "pre-commit");
    if ((0, node_fs_1.existsSync)(repoPreCommit))
        (0, node_fs_1.rmSync)(repoPreCommit, { force: true });
}
function resetMicroCommitTemplates(root) {
    wipeAfterCommit(root);
}
function emitPrepareStdout(message) {
    (0, node_fs_1.writeSync)(1, Buffer.from(message.endsWith("\n") ? message : "".concat(message, "\n")));
}
function stageMicroCommitChanges(root) {
    var excludedPaths = new Set();
    for (;;) {
        var args = ["add", "-A"];
        if (excludedPaths.size > 0)
            args.push.apply(args, __spreadArray(["--", "."], __spreadArray([], excludedPaths, true).map(function (path) { return ":(exclude)".concat(path); }), false));
        var staged = git(root, args);
        if (staged.ok)
            return staged;
        var rejectedPaths = __spreadArray([], staged.out.matchAll(/^error: '(.+)' does not have a commit checked out$/gm), true).map(function (match) { return match[1]; })
            .filter(function (path) { return !(0, node_path_1.isAbsolute)(path) && path !== "." && !path.startsWith("..".concat(node_path_1.sep)); });
        if (rejectedPaths.length === 0 || rejectedPaths.every(function (path) { return excludedPaths.has(path); }))
            return staged;
        for (var _i = 0, rejectedPaths_1 = rejectedPaths; _i < rejectedPaths_1.length; _i++) {
            var path = rejectedPaths_1[_i];
            excludedPaths.add(path);
        }
    }
}
function runMicroCommit(root, segments) {
    var _a, _b, _c, _d, _e, _f, _g, _h, _j;
    root = gitRepoRoot(root);
    var cmd = (_a = segments[0]) !== null && _a !== void 0 ? _a : "prepare";
    if (cmd === "reset") {
        resetMicroCommitTemplates(root);
        process.exit(0);
    }
    if (cmd === "install-hooks") {
        installMicroCommitGitHooks(root);
        process.exit(0);
    }
    if (cmd === "prepare-commit-msg") {
        var msgFile = segments[1];
        if (!msgFile)
            process.exit(1);
        handlePrepareCommitMsg(root, msgFile, (_b = segments[2]) !== null && _b !== void 0 ? _b : "");
        process.exit(0);
    }
    var branchError = branchValidationError("micro-commit", currentBranch(root));
    if (branchError) {
        console.error(branchError);
        process.exit(1);
    }
    var contributor = findContributor(root);
    if (!contributor) {
        console.error("micro-commit: no contributor for git user.email ".concat(gitEmail(root) || "(unset)"));
        process.exit(1);
    }
    if (cmd === "stage") {
        clearStaleTemplatesBeforePrepare(root);
        var staged_2 = stageMicroCommitChanges(root);
        if (!staged_2.ok) {
            console.error(staged_2.out || "git add -A failed");
            process.exit(1);
        }
        process.exit(0);
    }
    if (cmd === "diff") {
        var patch = git(root, ["diff", "--cached"]);
        if (!patch.ok) {
            console.error(patch.out || "git diff --cached failed");
            process.exit(1);
        }
        process.stdout.write(patch.out ? "".concat(patch.out, "\n") : "");
        process.exit(0);
    }
    if (cmd !== "prepare") {
        console.error("[micro-commit] usage: bun ./📜️script.ts micro-commit <stage|diff|prepare> [level tokens…] [-- bullets.txt]");
        process.exit(1);
    }
    var dash = segments.indexOf("--");
    var levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
    var bulletsFile = dash >= 0 ? ((_c = segments[dash + 1]) !== null && _c !== void 0 ? _c : null) : null;
    if (!bulletsFile) {
        var last = segments[segments.length - 1];
        if (last && (last.endsWith(".txt") || last.includes("/") || last.includes("\\"))) {
            bulletsFile = last;
        }
    }
    var level = loadLevel(root, contributor, levelSegments);
    clearStaleTemplatesBeforePrepare(root);
    var staged = stageMicroCommitChanges(root);
    if (!staged.ok) {
        console.error(staged.out || "git add -A failed");
        process.exit(1);
    }
    var stagedPaths = listCachedPaths(root);
    var diffBullets = readDiffBulletsInput(root, bulletsFile);
    if (diffBullets.length === 0) {
        for (var _i = 0, stagedPaths_1 = stagedPaths; _i < stagedPaths_1.length; _i++) {
            var p = stagedPaths_1[_i];
            console.error(p);
        }
        console.error("");
        var patch = git(root, ["diff", "--cached"]);
        if (patch.out)
            console.error(patch.out);
        console.error("\nmicro-commit: analyze the staged paths and diff above; pass 1–8 bullets on stdin (`{emoji}{description}` — pick the emoji that best matches each line, no leading `-`, no space after emoji; never 🎆️ 📊️ 🔢️ 🚩️)");
        process.exit(1);
    }
    validateBulletSpacing(diffBullets);
    validateBulletEmojis(diffBullets);
    validateBulletsAgainstStaged(diffBullets, stagedPaths);
    writePreparedBullets(root, diffBullets);
    var message;
    try {
        message = buildMicroCommitMessage(root, contributor, diffBullets);
    }
    catch (e) {
        console.error(e instanceof Error ? e.message : String(e));
        process.exit(1);
    }
    writeMicroCommitTemplates(root, message);
    emitPrepareStdout(message);
    if (level === "prepare-only")
        process.exit(0);
    var dir = gitDir(root);
    var commit = (0, node_child_process_1.spawnSync)("git", ["commit", "-S", "-F", (0, node_path_1.join)(dir, "COMMIT_EDITMSG")], { cwd: root, encoding: "utf8", env: gitSpawnEnv() });
    if (commit.status !== 0) {
        console.error(((_e = (_d = commit.stderr) !== null && _d !== void 0 ? _d : commit.stdout) !== null && _e !== void 0 ? _e : "git commit failed").trim());
        process.exit((_f = commit.status) !== null && _f !== void 0 ? _f : 1);
    }
    wipeAfterCommit(root);
    if (level === "prepare-and-commit")
        process.exit(0);
    var push = (0, node_child_process_1.spawnSync)("git", ["push"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() });
    if (push.status !== 0) {
        console.error(((_h = (_g = push.stderr) !== null && _g !== void 0 ? _g : push.stdout) !== null && _h !== void 0 ? _h : "git push failed").trim());
        process.exit((_j = push.status) !== null && _j !== void 0 ? _j : 1);
    }
    process.exit(0);
}
exports.BUNDLE_WIP_SUBJECT_RE = /^(.+🎆️\d{2}🌙️\d{2}☀️\d{2})🔀️$/u;
exports.BUNDLE_DATE_SECTION_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}$/u;
var EMOJI_CLUSTER_RE = /^(\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)/u;
var BUNDLE_SCOPE_RESERVED_RE = /🔀️|🚩️|📊️metric|🔢️/u;
var LABEL_TOKEN_BLOCKLIST = new Set(["uloc", "repo", "the", "and"]);
/** 🧭️Parses explicit `ct` / `cs` / `cp` step flags from argv segments. */
function parseCommitSteps(segments) {
    var token = segments.join(" ").toLowerCase();
    return {
        tag: /\b(ct|ctag|tag!|\+tag)\b/.test(token),
        squash: /\b(cs|csquash|squash!|\+squash)\b/.test(token),
        push: /\b(cp|cpush|push!|\+push)\b/.test(token),
    };
}
function commitStepsFromLevel(level) {
    switch (level) {
        case "prepare-and-tag":
            return { tag: true, squash: false, push: false };
        case "prepare-and-tag-and-squash":
            return { tag: true, squash: true, push: false };
        case "prepare-and-tag-and-squash-and-push":
            return { tag: true, squash: true, push: true };
        default:
            return { tag: false, squash: false, push: false };
    }
}
function loadCommitSteps(root, contributor, segments) {
    var explicit = parseCommitSteps(segments);
    if (explicit.tag || explicit.squash || explicit.push)
        return explicit;
    var path = (0, node_path_1.join)(getRepoMetaDir(root), "🧑️‍💻️devs", contributor.alias, "commit.json");
    if ((0, node_fs_1.existsSync)(path)) {
        var j = JSON.parse((0, node_fs_1.readFileSync)(path, "utf8"));
        var allowed = ["prepare-only", "prepare-and-tag", "prepare-and-tag-and-squash", "prepare-and-tag-and-squash-and-push"];
        if (allowed.includes(j.level))
            return commitStepsFromLevel(j.level);
    }
    return { tag: false, squash: false, push: false };
}
function isCommitPrepareOnly(segments) {
    var token = segments.join(" ").toLowerCase();
    if (/\b(c\.|cprepare|prepare!|\+prepare)\b/.test(token))
        return true;
    var steps = parseCommitSteps(segments);
    return !steps.tag && !steps.squash && !steps.push;
}
function shSingleQuote(s) {
    return "'".concat(s.replace(/'/g, "'\"'\"'"), "'");
}
function formatCommitPrepareCommandBlock(command) {
    return "```\n".concat(command, "\n```");
}
/** 🏷️GPG-signed annotated tag; tag object name and `-m` message are the same (`…🚩️`). */
function formatGitSignedTagCommand(tagName, head) {
    if (head === void 0) { head = "HEAD"; }
    var q = shSingleQuote(tagName);
    return "git tag -s -m ".concat(q, " ").concat(q, " ").concat(head);
}
/** 📋️Four copy-paste ``` blocks: signed tag, squash, push, all-in-one. */
function formatCommitPrepareCommands(opts) {
    var _a;
    var msg = (_a = opts.messageFile) !== null && _a !== void 0 ? _a : ".git/compose-commit-message";
    var tag = formatGitSignedTagCommand(opts.tagName);
    var squash = "git reset --soft ".concat(opts.wipSha, " && git commit -S -F ").concat(shSingleQuote(msg));
    var push = "git push --follow-tags";
    var all = "".concat(tag, " && git reset --soft ").concat(opts.wipSha, " && git commit -S -F ").concat(shSingleQuote(msg), " && git push --follow-tags");
    return "".concat([tag, squash, push, all].map(formatCommitPrepareCommandBlock).join("\n\n"), "\n");
}
/** 📋️Prepare-only agent reply: four `git` blocks, then tag name, then full commit message. */
function formatCommitPrepareAgentReply(opts) {
    var commands = formatCommitPrepareCommands({
        tagName: opts.tagName,
        wipSha: opts.wipSha,
        messageFile: opts.messageFile,
    });
    var tagNameBlock = formatCommitPrepareCommandBlock(opts.tagName.trim());
    var messageBlock = formatCommitPrepareCommandBlock(opts.commitMessage.trimEnd());
    return "".concat(commands).concat(tagNameBlock, "\n\n").concat(messageBlock, "\n");
}
/** 🔀️Finds the newest commit whose subject is a bundle/WIP marker (`…🔀️`). */
function findLastBundleWipCommit(root) {
    root = gitRepoRoot(root);
    var r = spawnCapturedSync("git", ["log", "--format=%H%x00%s%x00", "-n", "500"], {
        cwd: root,
        env: gitSpawnEnv(),
    });
    if (r.status !== 0)
        return null;
    var parts = r.stdout.toString("utf8").split("\0").filter(Boolean);
    for (var i = 0; i + 1 < parts.length; i += 2) {
        var sha = parts[i].trim();
        var subject = parts[i + 1].trim();
        if (exports.BUNDLE_WIP_SUBJECT_RE.test(subject))
            return { sha: sha, subject: subject };
    }
    return null;
}
/** 🏷️Signed tag name for the micro-commit tip before squash (`…🚩️`). */
function formatBundleTagName(contributor, now) {
    if (now === void 0) { now = new Date(); }
    var yy = pad2(now.getFullYear() % 100);
    var mm = pad2(now.getMonth() + 1);
    var dd = pad2(now.getDate());
    return "".concat(contributor.emoji).concat(contributor.alias, "\uD83C\uDF86\uFE0F").concat(yy, "\uD83C\uDF19\uFE0F").concat(mm, "\u2600\uFE0F").concat(dd, "\uD83D\uDEA9\uFE0F");
}
/** 🔀️Bundle squash commit subject (`…🔀️`). */
function formatBundleSubject(contributor, now) {
    if (now === void 0) { now = new Date(); }
    var yy = pad2(now.getFullYear() % 100);
    var mm = pad2(now.getMonth() + 1);
    var dd = pad2(now.getDate());
    return "".concat(contributor.emoji).concat(contributor.alias, "\uD83C\uDF86\uFE0F").concat(yy, "\uD83C\uDF19\uFE0F").concat(mm, "\u2600\uFE0F").concat(dd, "\uD83D\uDD00\uFE0F");
}
/** 🔢️Leading emoji clusters on a line. */
function leadingEmojiClusterCount(line) {
    var n = 0;
    var rest = line.trim();
    while (true) {
        var m = EMOJI_CLUSTER_RE.exec(rest);
        if (!m)
            break;
        n++;
        rest = rest.slice(m[0].length);
    }
    return n;
}
/** 🔢️Emoji clusters anywhere on a line (bundle labels interleave emoji + text). */
function emojiClusterCountInLine(line) {
    var re = /\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/gu;
    return __spreadArray([], line.matchAll(re), true).length;
}
/** 🧹️Strips hand-written uloc and reserved bundle/tag emojis from a scope line. */
function normalizeBundleScopeLabel(line) {
    return line
        .trim()
        .replace(/📊️metric.*$/u, "")
        .replace(/🔀️|🚩️/gu, "")
        .trim();
}
/** 🏷️ASCII tokens from a bundle emoji label (for matching git paths internally). */
function labelPathTokens(label) {
    var text = normalizeBundleScopeLabel(label)
        .replace(/\p{Extended_Pictographic}/gu, " ")
        .trim()
        .toLowerCase();
    return text.split(/[^a-z0-9]+/).filter(function (t) { return t.length >= 2 && !LABEL_TOKEN_BLOCKLIST.has(t); });
}
/** 🚫️Validates bundle scope label before parse. */
function bundleScopeLabelError(line) {
    var raw = line.trim();
    if (raw.includes("|") || raw.includes("/")) {
        return "commit: bundle scope must be emoji + area name only — no paths or `|`";
    }
    if (BUNDLE_SCOPE_RESERVED_RE.test(raw)) {
        return "commit: bundle scope must not include 🔀️ 🚩️ 📊️metric or 🔢️ — script adds subject and metrics";
    }
    var norm = normalizeBundleScopeLabel(raw);
    if (!norm)
        return "commit: empty bundle scope after removing reserved emojis";
    var tokens = labelPathTokens(norm);
    if (tokens.length === 0) {
        return "commit: bundle scope needs an area name after emojis (e.g. 🏘️compose✍️sketchpad, 🥅️framework, 🖱️ui⚛️react)";
    }
    if (!isBundleScopeLine(norm)) {
        return "commit: invalid bundle scope: ".concat(raw);
    }
    return null;
}
function changedPathsInRange(root, base, head) {
    var r = git(root, ["diff", "--name-only", "".concat(base, "..").concat(head)]);
    if (!r.ok || !r.out)
        return [];
    return r.out.split("\n").filter(Boolean);
}
function longestCommonPathPrefix(paths) {
    var _a;
    if (paths.length === 0)
        return "";
    var split = paths.map(function (p) { return p.split("/"); });
    var first = split[0];
    var len = 0;
    var _loop_6 = function (i) {
        var seg = first[i];
        if (split.every(function (parts) { return parts[i] === seg; }))
            len = i + 1;
        else
            return "break";
    };
    for (var i = 0; i < first.length; i++) {
        var state_2 = _loop_6(i);
        if (state_2 === "break")
            break;
    }
    if (len === 0)
        return (_a = paths[0].split("/")[0]) !== null && _a !== void 0 ? _a : "";
    return first.slice(0, len).join("/");
}
/** 📂️Infers repo path prefixes for a bundle label from the revision range (not shown in messages). */
function inferPathPrefixesForBundleLabel(root, base, head, label, assignedPaths) {
    var tokens = labelPathTokens(label);
    if (tokens.length === 0)
        return [];
    var pool = assignedPaths !== null && assignedPaths !== void 0 ? assignedPaths : changedPathsInRange(root, base, head);
    var matched = pool.filter(function (p) {
        var pl = p.toLowerCase();
        return tokens.every(function (t) { return pl.includes(t); });
    });
    if (matched.length === 0)
        return [];
    var prefix = longestCommonPathPrefix(matched);
    return prefix ? [prefix] : [];
}
function assignPathToBundleIndex(bundles, path) {
    var pl = path.toLowerCase();
    var best = -1;
    var bestScore = 0;
    for (var i = 0; i < bundles.length; i++) {
        var tokens = labelPathTokens(bundles[i].label);
        if (tokens.length === 0)
            continue;
        if (!tokens.every(function (t) { return pl.includes(t); }))
            continue;
        var score = tokens.length;
        if (score > bestScore) {
            bestScore = score;
            best = i;
        }
    }
    return best;
}
function assignChangedPathsToBundles(root, base, head, bundles) {
    var paths = changedPathsInRange(root, base, head);
    var assigned = bundles.map(function () { return []; });
    for (var _i = 0, paths_2 = paths; _i < paths_2.length; _i++) {
        var p = paths_2[_i];
        var bi = assignPathToBundleIndex(bundles, p);
        if (bi >= 0)
            assigned[bi].push(p);
    }
    return assigned;
}
function addGitDeltaSums(a, b) {
    return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}
var BUNDLE_COMMIT_TIMESTAMP_LINE_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}⏰️/u;
/** 🎆️Calendar day from a micro-commit subject (`🎆️YY🌙️MM☀️DD`). */
function extractBundleDateLineFromSubject(subject) {
    var _a;
    var m = /🎆️\d{2}🌙️\d{2}☀️\d{2}/u.exec(subject.trim());
    return (_a = m === null || m === void 0 ? void 0 : m[0]) !== null && _a !== void 0 ? _a : null;
}
/** 🎆️Calendar day from the micro-commit body timestamp line (`🎆️YY🌙️MM☀️DD⏰️…`). */
function extractBundleDateLineFromCommitBody(body) {
    var _a;
    for (var _i = 0, _b = body.split("\n"); _i < _b.length; _i++) {
        var raw = _b[_i];
        var line = raw.trim();
        if (!BUNDLE_COMMIT_TIMESTAMP_LINE_RE.test(line))
            continue;
        var m = /^🎆️\d{2}🌙️\d{2}☀️\d{2}/u.exec(line);
        return (_a = m === null || m === void 0 ? void 0 : m[0]) !== null && _a !== void 0 ? _a : null;
    }
    return null;
}
/** 🎆️Calendar day for bundle per-day uloc (body timestamp, else subject). */
function extractBundleDateLineFromCommit(subject, body) {
    var _a;
    return (_a = extractBundleDateLineFromCommitBody(body)) !== null && _a !== void 0 ? _a : extractBundleDateLineFromSubject(subject);
}
/** 📂️Paths from one numstat row (rename rows may join old/new with tabs or `=>`). */
function pathsFromNumstatRow(pathField) {
    var parts = pathField
        .split("\t")
        .map(function (p) { return p.trim(); })
        .filter(Boolean);
    if (parts.length === 0)
        return [];
    var out = new Set();
    for (var _i = 0, parts_1 = parts; _i < parts_1.length; _i++) {
        var part = parts_1[_i];
        var brace = /^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$/u.exec(part);
        if (brace) {
            var a = brace[2].trim();
            var b = brace[3].trim();
            if (a)
                out.add("".concat(brace[1]).concat(a).concat(brace[4]));
            if (b)
                out.add("".concat(brace[1]).concat(b).concat(brace[4]));
            continue;
        }
        var arrow = /\s*=>\s*/u.exec(part);
        if (arrow) {
            var _a = part.split(/\s*=>\s*/u), from = _a[0], to = _a[1];
            if (from === null || from === void 0 ? void 0 : from.trim())
                out.add(from.trim());
            if (to === null || to === void 0 ? void 0 : to.trim())
                out.add(to.trim());
            continue;
        }
        out.add(part);
    }
    return __spreadArray([], out, true);
}
/** 📂️Prefix set for a bundle: all range paths, inferred roots, and token-matching parents (renames). */
function buildBundlePathPrefixSets(root, base, head, bundles) {
    var assignments = assignChangedPathsToBundles(root, base, head, bundles);
    return bundles.map(function (bundle, i) {
        var _a;
        var assigned = (_a = assignments[i]) !== null && _a !== void 0 ? _a : [];
        var prefixes = new Set();
        var tokens = labelPathTokens(bundle.label);
        for (var _i = 0, assigned_1 = assigned; _i < assigned_1.length; _i++) {
            var p = assigned_1[_i];
            prefixes.add(p);
            if (tokens.length === 0)
                continue;
            var segments = p.split("/");
            var acc = "";
            var _loop_7 = function (seg) {
                acc = acc ? "".concat(acc, "/").concat(seg) : seg;
                var pl = acc.toLowerCase();
                if (tokens.every(function (t) { return pl.includes(t); }))
                    prefixes.add(acc);
            };
            for (var _b = 0, segments_1 = segments; _b < segments_1.length; _b++) {
                var seg = segments_1[_b];
                _loop_7(seg);
            }
        }
        return __spreadArray([], prefixes, true);
    });
}
/** 📂️Whether a path belongs to a bundle (assigned path prefixes, else label token scoring). */
function pathMatchesBundleIndex(path, bundleIndex, prefixSets, bundles) {
    var _a;
    var prefixes = (_a = prefixSets[bundleIndex]) !== null && _a !== void 0 ? _a : [];
    if (prefixes.length > 0)
        return pathUnderPrefixes(path, prefixes);
    return assignPathToBundleIndex(bundles, path) === bundleIndex;
}
/** 🧹️Strips hand-written per-day uloc from a date section line. */
function normalizeBundleDateLine(line) {
    return line
        .trim()
        .replace(/📊️metric.*$/u, "")
        .trim();
}
/** 🎆️Bundle squash only: `🎆️YY🌙️MM☀️DD` + full per-day `📊️metric…` suffixes. */
function formatBundleDateLine(dateLine, ulocDelta, sizeDelta, ulocBloc, sizeBloc) {
    return "".concat(normalizeBundleDateLine(dateLine)).concat(formatBundleMetricSuffixes(ulocDelta, sizeDelta, ulocBloc, sizeBloc));
}
function gitCommitShasInRange(root, base, head) {
    var r = git(root, ["rev-list", "--reverse", "".concat(base, "..").concat(head)]);
    if (!r.ok || !r.out)
        return [];
    return r.out.split("\n").filter(Boolean);
}
function addUlocDeltaToBundleDateMap(map, chunk, dateLine, bi) {
    var _a;
    if (gitDeltaLineTotal(chunk) === 0)
        return;
    var bundleMap = map.get(bi);
    var prev = (_a = bundleMap.get(dateLine)) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
    bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}
/** 📊️Per-bundle per-day git deltas: sum each micro-commit parent..sha row on its body 🎆️ day (sums to range partition). */
function buildBundleDateDeltasMap(root, base, head, bundles) {
    root = gitRepoRoot(root);
    var map = new Map();
    for (var i = 0; i < bundles.length; i++)
        map.set(i, new Map());
    var prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
    for (var _i = 0, _a = gitCommitShasInRange(root, base, head); _i < _a.length; _i++) {
        var sha = _a[_i];
        var parent_2 = "".concat(sha, "^");
        var subject = git(root, ["log", "-1", "--format=%s", sha]).out;
        var body = git(root, ["log", "-1", "--format=%B", sha]).out;
        var dateLine = extractBundleDateLineFromCommit(subject, body);
        if (!dateLine)
            continue;
        var rows = gitRangeNumstat(root, parent_2, sha);
        var rowDeltas = accumulateUlocDeltasByRow(root, rows, parent_2, sha);
        for (var _b = 0, _c = rows.entries(); _b < _c.length; _b++) {
            var _d = _c[_b], index = _d[0], row = _d[1];
            var rowPaths = pathsFromNumstatRow(row.path);
            if (rowPaths.length === 0 || rowPaths.every(function (p) { return shouldSkipPathForUloc(root, p); }))
                continue;
            var owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
            if (owners.length === 0) {
                throw new Error("commit: changed path is not attributed to any bundle \u2014 ".concat(row.path, "; add a bundle scope or fix labels"));
            }
            if (owners.length > 1) {
                var names = owners.map(function (i) { return bundles[i].label; }).join(", ");
                throw new Error("commit: changed path matches multiple bundles (".concat(names, ") \u2014 ").concat(row.path));
            }
            addUlocDeltaToBundleDateMap(map, sumGitLangDeltas(rowDeltas[index]), dateLine, owners[0]);
        }
    }
    return map;
}
function addSizeRowToBundleDateMap(map, row, dateLine, bi, root, oldRev, newRev) {
    var _a;
    var chunk = sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, [{ path: row.path }], oldRev, newRev));
    if (gitDeltaLineTotal(chunk) === 0)
        return;
    var bundleMap = map.get(bi);
    var prev = (_a = bundleMap.get(dateLine)) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
    bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}
/** 📊️Per-bundle per-day byte deltas from micro-commit parent..sha rows. */
function buildBundleDateSizeDeltasMap(root, base, head, bundles) {
    root = gitRepoRoot(root);
    var map = new Map();
    for (var i = 0; i < bundles.length; i++)
        map.set(i, new Map());
    var prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
    for (var _i = 0, _a = gitCommitShasInRange(root, base, head); _i < _a.length; _i++) {
        var sha = _a[_i];
        var parent_3 = "".concat(sha, "^");
        var subject = git(root, ["log", "-1", "--format=%s", sha]).out;
        var body = git(root, ["log", "-1", "--format=%B", sha]).out;
        var dateLine = extractBundleDateLineFromCommit(subject, body);
        if (!dateLine)
            continue;
        for (var _b = 0, _c = gitRangeNumstat(root, parent_3, sha); _b < _c.length; _b++) {
            var row = _c[_b];
            var rowPaths = pathsFromNumstatRow(row.path);
            if (rowPaths.length === 0 || rowPaths.every(function (p) { return shouldSkipPathForUloc(root, p); }))
                continue;
            var owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
            if (owners.length === 0) {
                throw new Error("commit: changed path is not attributed to any bundle \u2014 ".concat(row.path, "; add a bundle scope or fix labels"));
            }
            if (owners.length > 1) {
                var names = owners.map(function (i) { return bundles[i].label; }).join(", ");
                throw new Error("commit: changed path matches multiple bundles (".concat(names, ") \u2014 ").concat(row.path));
            }
            addSizeRowToBundleDateMap(map, row, dateLine, owners[0], root, parent_3, sha);
        }
    }
    return map;
}
function bundleGitDeltasForPaths(root, base, head, pathPrefixes) {
    return sumGitLangDeltas(accumulateRangeUlocDeltas(root, base, head, pathPrefixes));
}
function bundleSizeDeltasForPaths(root, base, head, pathPrefixes) {
    var rows = gitRangeNumstat(root, base, head);
    var assigned = pathPrefixes.length > 0 ? rows.filter(function (r) { return pathsFromNumstatRow(r.path).some(function (p) { return pathUnderPrefixes(p, pathPrefixes); }); }) : [];
    return sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, assigned, base, head));
}
function formatBundleHeaderLine(label, ulocTotal, sizeTotal, ulocBloc, sizeBloc) {
    return "".concat(normalizeBundleScopeLabel(label)).concat(formatBundleMetricSuffixes(ulocTotal, sizeTotal, ulocBloc, sizeBloc));
}
/** 📊️Orders bundles by descending 🟰️ (➕️+✏️+➖️) from assigned path diffs. */
function sortCommitBundlesByEditTotal(root, base, head, bundles, pathAssignments) {
    var prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
    var ranked = bundles.map(function (bundle, i) {
        var _a, _b, _c;
        return ({
            bundle: bundle,
            paths: (_a = pathAssignments[i]) !== null && _a !== void 0 ? _a : [],
            prefixes: (_b = prefixSets[i]) !== null && _b !== void 0 ? _b : [],
            total: gitDeltaLineTotal(bundleGitDeltasForPaths(root, base, head, (_c = prefixSets[i]) !== null && _c !== void 0 ? _c : [])),
        });
    });
    ranked.sort(function (a, b) { return b.total - a.total; });
    return {
        bundles: ranked.map(function (r) { return r.bundle; }),
        pathAssignments: ranked.map(function (r) { return r.paths; }),
        pathPrefixSets: ranked.map(function (r) { return r.prefixes; }),
    };
}
/** 🏷️Bundle scope line: two+ emojis, or one emoji + lowercase technology slug (`🥅️framework`). */
function isBundleScopeLine(line) {
    var t = normalizeBundleScopeLabel(line);
    if (!t || exports.BUNDLE_DATE_SECTION_RE.test(t))
        return false;
    if (t.includes("/") || t.includes("|"))
        return false;
    if (BUNDLE_SCOPE_RESERVED_RE.test(t))
        return false;
    if (emojiClusterCountInLine(t) >= 2)
        return true;
    if (emojiClusterCountInLine(t) !== 1)
        return false;
    var after = t.replace(/^\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/u, "");
    return /^[a-z][a-z0-9]{2,24}$/u.test(after);
}
/** 🚫️Validates stdin is bundle body only, not a full commit message. */
function commitBundleBodyError(text) {
    var _a, _b;
    var first = (_b = (_a = text
        .trim()
        .split("\n")
        .find(function (l) { return l.trim().length > 0; })) === null || _a === void 0 ? void 0 : _a.trim()) !== null && _b !== void 0 ? _b : "";
    if (exports.BUNDLE_WIP_SUBJECT_RE.test(first)) {
        return "commit: stdin must not include the bundle subject (…🔀️) — script adds it";
    }
    if (/^🐙️|^🧑️/.test(first) && first.includes("🚩️")) {
        return "commit: stdin must not include micro-commit subject lines";
    }
    if (/^📊️metric/m.test(text)) {
        return "commit: stdin must not include the 📊️metric footer — script adds it";
    }
    if (/^🎆️\d{2}🌙️\d{2}☀️\d{2}📊️metric/m.test(text)) {
        return "commit: stdin must not include per-day 📊️metric — script adds it to each 🎆️ line";
    }
    return null;
}
function validateBulletLine(b) {
    if (!MICRO_COMMIT_BULLET_RE.test(b)) {
        throw new Error("commit: bullet must start with {emoji} then description (no space after emoji): ".concat(b));
    }
    var err = bulletEmojiValidationError([b]);
    if (err)
        throw new Error(err.replace(/^micro-commit:/, "commit:"));
    if (exports.BUNDLE_DATE_SECTION_RE.test(b.trim())) {
        throw new Error("commit: use a date section line `🎆️YY🌙️MM☀️DD` on its own, not as a bullet");
    }
}
/** 📦️Parses LLM bundle body (emoji-only scope lines, dates, bullets). */
function parseCommitBundleBody(text) {
    var bundles = [];
    var current = null;
    var dateSection = null;
    for (var _i = 0, _a = text.split("\n"); _i < _a.length; _i++) {
        var raw = _a[_i];
        var line = raw.trim();
        if (!line || line.startsWith("#"))
            continue;
        if (line.startsWith("📊️metric") || line.startsWith("Signed-off-by:"))
            continue;
        var dateCandidate = normalizeBundleDateLine(line);
        if (exports.BUNDLE_DATE_SECTION_RE.test(dateCandidate)) {
            if (!current)
                throw new Error("commit: date section before bundle scope: ".concat(line));
            if (dateSection)
                current.dates.push(dateSection);
            dateSection = { dateLine: dateCandidate, bullets: [] };
            continue;
        }
        if (dateSection) {
            if (isBundleScopeLine(line)) {
                var scopeErr = bundleScopeLabelError(line);
                if (scopeErr)
                    throw new Error(scopeErr);
                if (current) {
                    current.dates.push(dateSection);
                    bundles.push(current);
                }
                current = { label: normalizeBundleScopeLabel(line), dates: [] };
                dateSection = null;
                continue;
            }
            validateBulletLine(line);
            dateSection.bullets.push(line);
            continue;
        }
        if (isBundleScopeLine(line)) {
            var scopeErr = bundleScopeLabelError(line);
            if (scopeErr)
                throw new Error(scopeErr);
            if (current)
                bundles.push(current);
            current = { label: normalizeBundleScopeLabel(line), dates: [] };
            continue;
        }
        if (!current)
            throw new Error("commit: expected emoji bundle scope (two+ emojis, no paths), got: ".concat(line));
        throw new Error("commit: expected \uD83C\uDF86\uFE0FYY\uD83C\uDF19\uFE0FMM\u2600\uFE0FDD date section before bullet: ".concat(line));
    }
    if (current) {
        if (dateSection)
            current.dates.push(dateSection);
        bundles.push(current);
    }
    if (bundles.length === 0)
        throw new Error("commit: at least one emoji bundle scope is required");
    for (var _b = 0, bundles_1 = bundles; _b < bundles_1.length; _b++) {
        var b = bundles_1[_b];
        if (b.dates.length === 0)
            throw new Error("commit: bundle ".concat(b.label, " needs at least one date section"));
        for (var _c = 0, _d = b.dates; _c < _d.length; _c++) {
            var d = _d[_c];
            if (d.bullets.length === 0)
                throw new Error("commit: ".concat(d.dateLine, " in ").concat(b.label, " needs at least one bullet"));
        }
    }
    return bundles;
}
function formatGitDeltaSumBrief(d) {
    return "\u2795\uFE0F".concat(d.added, "\u270F\uFE0F").concat(d.edited, "\u2796\uFE0F").concat(d.removed, "\uD83D\uDFF0\uFE0F").concat(gitDeltaLineTotal(d));
}
function assertGitDeltaSumsEqual(a, b, message) {
    if (gitDeltaSumsEqual(a, b))
        return;
    throw new Error("".concat(message, " \u2014 ").concat(formatGitDeltaSumBrief(a), " vs ").concat(formatGitDeltaSumBrief(b)));
}
/** 📂️Bundle indices that own a numstat row (0 or 1 after validation). */
function resolveBundleIndicesForNumstatRow(pathField, prefixSets, bundles) {
    var matched = new Set();
    for (var _i = 0, _a = pathsFromNumstatRow(pathField); _i < _a.length; _i++) {
        var path = _a[_i];
        for (var bi = 0; bi < bundles.length; bi++) {
            if (pathMatchesBundleIndex(path, bi, prefixSets, bundles))
                matched.add(bi);
        }
    }
    return __spreadArray([], matched, true);
}
function rangeGitDeltaTotal(root, base, head) {
    return sumGitLangDeltas(accumulateRangeUlocDeltas(root, base, head));
}
/** 📊️Partition WIP-range numstat across bundles (one bundle per row). */
function partitionRangeDeltasByBundle(root, base, head, bundles, prefixSets) {
    var bundleTotals = bundles.map(function () { return ({ added: 0, removed: 0, edited: 0 }); });
    var rangeTotal = { added: 0, removed: 0, edited: 0 };
    var rows = gitRangeNumstat(root, base, head);
    var rowDeltas = accumulateUlocDeltasByRow(root, rows, base, head);
    for (var _i = 0, _a = rows.entries(); _i < _a.length; _i++) {
        var _b = _a[_i], index = _b[0], row = _b[1];
        var rowPaths = pathsFromNumstatRow(row.path);
        if (rowPaths.length === 0 || rowPaths.every(function (p) { return shouldSkipPathForUloc(root, p); }))
            continue;
        var chunk = sumGitLangDeltas(rowDeltas[index]);
        if (gitDeltaLineTotal(chunk) === 0)
            continue;
        rangeTotal = addGitDeltaSums(rangeTotal, chunk);
        var owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
        if (owners.length === 0) {
            throw new Error("commit: changed path is not attributed to any bundle \u2014 ".concat(row.path, "; add a bundle scope or fix labels"));
        }
        if (owners.length > 1) {
            var names = owners.map(function (i) { return bundles[i].label; }).join(", ");
            throw new Error("commit: changed path matches multiple bundles (".concat(names, ") \u2014 ").concat(row.path));
        }
        var bi = owners[0];
        bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi], chunk);
    }
    return { bundleTotals: bundleTotals, rangeTotal: rangeTotal };
}
/** 📊️Partition WIP-range byte deltas across bundles (one bundle per row). */
function partitionRangeSizeDeltasByBundle(root, base, head, bundles, prefixSets) {
    var bundleTotals = bundles.map(function () { return ({ added: 0, removed: 0, edited: 0 }); });
    var rangeTotal = { added: 0, removed: 0, edited: 0 };
    for (var _i = 0, _a = gitRangeNumstat(root, base, head); _i < _a.length; _i++) {
        var row = _a[_i];
        var rowPaths = pathsFromNumstatRow(row.path);
        if (rowPaths.length === 0 || rowPaths.every(function (p) { return shouldSkipPathForUloc(root, p); }))
            continue;
        var chunk = sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, [{ path: row.path }], base, head));
        if (gitDeltaLineTotal(chunk) === 0)
            continue;
        rangeTotal = addGitDeltaSums(rangeTotal, chunk);
        var owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
        if (owners.length === 0) {
            throw new Error("commit: changed path is not attributed to any bundle \u2014 ".concat(row.path, "; add a bundle scope or fix labels"));
        }
        if (owners.length > 1) {
            var names = owners.map(function (i) { return bundles[i].label; }).join(", ");
            throw new Error("commit: changed path matches multiple bundles (".concat(names, ") \u2014 ").concat(row.path));
        }
        var bi = owners[0];
        bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi], chunk);
    }
    return { bundleTotals: bundleTotals, rangeTotal: rangeTotal };
}
function validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, bundleTotals, kindToken, additive) {
    var _a, _b, _c;
    if (kindToken === void 0) { kindToken = "📃uloc"; }
    if (additive === void 0) { additive = true; }
    for (var bi = 0; bi < bundles.length; bi++) {
        var bundle = bundles[bi];
        var total = (_a = bundleTotals[bi]) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 };
        var listedDates = new Set(bundle.dates.map(function (s) { return s.dateLine; }));
        var daySum = { added: 0, removed: 0, edited: 0 };
        for (var _i = 0, listedDates_1 = listedDates; _i < listedDates_1.length; _i++) {
            var dateLine = listedDates_1[_i];
            var d = (_c = (_b = dateDeltas.get(bi)) === null || _b === void 0 ? void 0 : _b.get(dateLine)) !== null && _c !== void 0 ? _c : { added: 0, removed: 0, edited: 0 };
            daySum = addGitDeltaSums(daySum, d);
        }
        var perDay = dateDeltas.get(bi);
        if (perDay) {
            for (var _d = 0, perDay_1 = perDay; _d < perDay_1.length; _d++) {
                var _e = perDay_1[_d], dateLine = _e[0], d = _e[1];
                if (listedDates.has(dateLine))
                    continue;
                if (gitDeltaLineTotal(d) === 0)
                    continue;
                throw new Error("commit: ".concat(bundle.label, " has micro-commit changes on ").concat(dateLine, " (").concat(formatGitDeltaSumBrief(d), ") but that day is missing from your bundle body \u2014 add a \uD83C\uDF86\uFE0F section or fix attribution"));
            }
        }
        if (additive && (daySum.added !== total.added || daySum.edited !== total.edited || daySum.removed !== total.removed)) {
            throw new Error("commit: per-day \uD83D\uDCCA\uFE0Fmetric".concat(kindToken, " for ").concat(bundle.label, " does not add up to the bundle total \u2014 days ").concat(formatGitDeltaSumBrief(daySum), " vs bundle ").concat(formatGitDeltaSumBrief(total), "; re-read log + diff and fix bundle/date attribution"));
        }
    }
}
/** 🚫️All bundle-commit metrics constraints (days→bundle, bundles→range, languages→range). */
function validateBundleCommitAttribution(root, base, head, bundles, metricsRunner) {
    var _a;
    root = gitRepoRoot(root);
    var runner = metricsRunner && "countRepoUlocByLanguage" in metricsRunner ? metricsRunner : metricsRunnerFromUloc(metricsRunner);
    var prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
    var _b = partitionRangeDeltasByBundle(root, base, head, bundles, prefixSets), ulocPartitioned = _b.bundleTotals, ulocRangeTotal = _b.rangeTotal;
    var _c = partitionRangeSizeDeltasByBundle(root, base, head, bundles, prefixSets), sizePartitioned = _c.bundleTotals, sizeRangeTotal = _c.rangeTotal;
    var dateDeltas = buildBundleDateDeltasMap(root, base, head, bundles);
    var dateSizeDeltas = buildBundleDateSizeDeltasMap(root, base, head, bundles);
    validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, ulocPartitioned, exports.METRIC_KIND_ULOC.token);
    validateBundleDayDeltasAttribution(bundles, prefixSets, dateSizeDeltas, sizePartitioned, exports.METRIC_KIND_SIZE.token, false);
    for (var bi = 0; bi < bundles.length; bi++) {
        var allDays = { added: 0, removed: 0, edited: 0 };
        var perDay = dateDeltas.get(bi);
        if (perDay) {
            for (var _i = 0, _d = perDay.values(); _i < _d.length; _i++) {
                var d = _d[_i];
                allDays = addGitDeltaSums(allDays, d);
            }
        }
        assertGitDeltaSumsEqual(allDays, (_a = ulocPartitioned[bi]) !== null && _a !== void 0 ? _a : { added: 0, removed: 0, edited: 0 }, "commit: all micro-commit days for ".concat(bundles[bi].label, " do not add up to the bundle uloc total"));
    }
    var bundleSum = { added: 0, removed: 0, edited: 0 };
    for (var _e = 0, ulocPartitioned_1 = ulocPartitioned; _e < ulocPartitioned_1.length; _e++) {
        var t = ulocPartitioned_1[_e];
        bundleSum = addGitDeltaSums(bundleSum, t);
    }
    assertGitDeltaSumsEqual(bundleSum, ulocRangeTotal, "commit: all bundle header uloc totals do not add up to the WIP range — fix bundle attribution");
    var bundleSizeSum = { added: 0, removed: 0, edited: 0 };
    for (var _f = 0, sizePartitioned_1 = sizePartitioned; _f < sizePartitioned_1.length; _f++) {
        var t = sizePartitioned_1[_f];
        bundleSizeSum = addGitDeltaSums(bundleSizeSum, t);
    }
    assertGitDeltaSumsEqual(bundleSizeSum, sizeRangeTotal, "commit: all bundle header size totals do not add up to the WIP range — fix bundle attribution");
    var metrics = buildCommitMetricsForRange(root, base, head, undefined, runner);
    validateCommitMetricsDeltaSum(metrics);
    var ulocLangTotal = sumMicroCommitLangMetrics(metrics.uloc);
    assertGitDeltaSumsEqual({ added: ulocLangTotal.added, edited: ulocLangTotal.edited, removed: ulocLangTotal.removed }, ulocRangeTotal, "commit: footer per-language 📃uloc does not add up to the WIP range total");
    var sizeLangTotal = sumMicroCommitLangMetrics(metrics.size);
    assertGitDeltaSumsEqual({ added: sizeLangTotal.added, edited: sizeLangTotal.edited, removed: sizeLangTotal.removed }, sizeRangeTotal, "commit: footer per-language 💾size does not add up to the WIP range total");
}
function buildCommitMessage(root, contributor, bundles, wipSha, head, metricsRunner, now) {
    var _a, _b, _c, _d, _e, _f;
    if (head === void 0) { head = "HEAD"; }
    if (now === void 0) { now = new Date(); }
    root = gitRepoRoot(root);
    var pathAssignments = assignChangedPathsToBundles(root, wipSha, head, bundles);
    var sorted = sortCommitBundlesByEditTotal(root, wipSha, head, bundles, pathAssignments);
    bundles = sorted.bundles;
    var runner = metricsRunner && "countRepoUlocByLanguage" in metricsRunner ? metricsRunner : metricsRunnerFromUloc(metricsRunner);
    validateBundleCommitAttribution(root, wipSha, head, bundles, runner);
    var prefixSets = buildBundlePathPrefixSets(root, wipSha, head, bundles);
    var ulocBundleTotals = partitionRangeDeltasByBundle(root, wipSha, head, bundles, prefixSets).bundleTotals;
    var sizeBundleTotals = partitionRangeSizeDeltasByBundle(root, wipSha, head, bundles, prefixSets).bundleTotals;
    var dateDeltas = buildBundleDateDeltasMap(root, wipSha, head, bundles);
    var dateSizeDeltas = buildBundleDateSizeDeltasMap(root, wipSha, head, bundles);
    var ulocBundleBlocs = prefixSets.map(function (prefixes) { return countUnifiedLocUnderPathPrefixes(root, prefixes); });
    var sizeBundleBlocs = prefixSets.map(function (prefixes) { return countBytesUnderPathPrefixes(root, prefixes); });
    var lines = [formatBundleSubject(contributor, now), ""];
    for (var bi = 0; bi < bundles.length; bi++) {
        var bundle = bundles[bi];
        var ulocBloc = (_a = ulocBundleBlocs[bi]) !== null && _a !== void 0 ? _a : 0;
        var sizeBloc = (_b = sizeBundleBlocs[bi]) !== null && _b !== void 0 ? _b : 0;
        lines.push(formatBundleHeaderLine(bundle.label, (_c = ulocBundleTotals[bi]) !== null && _c !== void 0 ? _c : { added: 0, removed: 0, edited: 0 }, (_d = sizeBundleTotals[bi]) !== null && _d !== void 0 ? _d : { added: 0, removed: 0, edited: 0 }, ulocBloc, sizeBloc));
        var perDay = dateDeltas.get(bi);
        var perSizeDay = dateSizeDeltas.get(bi);
        for (var _i = 0, _g = bundle.dates; _i < _g.length; _i++) {
            var section = _g[_i];
            var dayUloc = (_e = perDay === null || perDay === void 0 ? void 0 : perDay.get(section.dateLine)) !== null && _e !== void 0 ? _e : { added: 0, removed: 0, edited: 0 };
            var daySize = (_f = perSizeDay === null || perSizeDay === void 0 ? void 0 : perSizeDay.get(section.dateLine)) !== null && _f !== void 0 ? _f : { added: 0, removed: 0, edited: 0 };
            lines.push(formatBundleDateLine(section.dateLine, dayUloc, daySize, ulocBloc, sizeBloc));
            lines.push.apply(lines, section.bullets);
        }
        if (bi < bundles.length - 1)
            lines.push("");
    }
    var metrics = formatCommitMetricsLines(buildCommitMetricsForRange(root, wipSha, head, undefined, runner));
    if (metrics.length > 0)
        lines.push.apply(lines, __spreadArray(__spreadArray([], metrics, false), [""], false));
    lines.push("Signed-off-by: ".concat(contributor.name, " <").concat(contributor.email, ">"));
    return "".concat(lines.join("\n"), "\n");
}
function readBodyInput(root, file) {
    if (file) {
        var path = file.startsWith("/") ? file : (0, node_path_1.join)(root, file);
        return (0, node_fs_1.readFileSync)(path, "utf8");
    }
    if (process.stdin.isTTY)
        return "";
    return (0, node_fs_1.readFileSync)(0, "utf8");
}
function assertCleanWorktree(root) {
    var st = git(root, ["status", "--porcelain"]);
    if (!st.ok)
        return;
    if (st.out.trim()) {
        console.error("commit: working tree must be clean before tag/squash/push");
        process.exit(1);
    }
}
function emitStdout(message) {
    process.stdout.write(message.endsWith("\n") ? message : "".concat(message, "\n"));
}
var COMMIT_DIFF_MAX_BYTES = 1500000;
/** 🔀️Resolves last bundle WIP and revision range for analysis. */
function resolveCommitBundleRange(root) {
    root = gitRepoRoot(root);
    var wip = findLastBundleWipCommit(root);
    if (!wip)
        return null;
    return { wip: wip, range: "".concat(wip.sha, "..HEAD") };
}
function normalizeCompareLine(s) {
    return s.trim().replace(/\s+/g, " ").toLowerCase();
}
/** 📜️Collects comparable lines from commit bodies in range (for copy detection). */
function commitHistoryCompareLines(root, base, head) {
    var r = spawnCapturedSync("git", ["log", "--format=%B%x00", "".concat(base, "..").concat(head)], {
        cwd: gitRepoRoot(root),
        env: gitSpawnEnv(),
    });
    var lines = new Set();
    if (r.status !== 0)
        return lines;
    for (var _i = 0, _a = r.stdout.toString("utf8").split("\0"); _i < _a.length; _i++) {
        var body = _a[_i];
        if (!body.trim())
            continue;
        for (var _b = 0, _c = body.split("\n"); _b < _c.length; _b++) {
            var raw = _c[_b];
            var line = raw.trim();
            if (line.length < 12)
                continue;
            if (exports.BUNDLE_WIP_SUBJECT_RE.test(line))
                continue;
            if (exports.BUNDLE_DATE_SECTION_RE.test(normalizeBundleDateLine(line)))
                continue;
            if (line.startsWith("📊️metric") || line.startsWith("Signed-off-by:"))
                continue;
            if (line.startsWith("Signed-off-by:"))
                continue;
            if (/^🎆️\d{2}🌙️\d{2}☀️\d{2}⏰️/u.test(line))
                continue;
            lines.add(normalizeCompareLine(line));
        }
    }
    return lines;
}
/** 🚫️Whether a bullet verbatim-matches a line from a prior commit body in the range. */
function bulletMatchesCommitHistory(bullet, history) {
    return history.has(normalizeCompareLine(bullet));
}
/** 🚫️Ensures bullets are newly written from diff analysis, not pasted from prior commits. */
function validateBundleBulletsFresh(root, base, head, bundles) {
    var history = commitHistoryCompareLines(root, base, head);
    for (var _i = 0, bundles_2 = bundles; _i < bundles_2.length; _i++) {
        var bundle = bundles_2[_i];
        for (var _a = 0, _b = bundle.dates; _a < _b.length; _a++) {
            var section = _b[_a];
            for (var _c = 0, _d = section.bullets; _c < _d.length; _c++) {
                var bullet = _d[_c];
                if (bulletMatchesCommitHistory(bullet, history)) {
                    throw new Error("commit: bullet copies a prior commit message line \u2014 rewrite from git diff only: ".concat(bullet));
                }
            }
        }
    }
}
function emitCommitBundleAttributionNote() {
    console.error("commit: bundles and file→bundle mapping are NOT automatic — folder layout and bundle boundaries change between WIPs");
    console.error("commit: you must (1) read log for last bundle/WIP state, (2) read diff --stat + full diff for every path, (3) decide scopes/dates/bullets, then prepare stdin");
    console.error("commit: script only adds subject, uloc suffixes, sort order, footer, Signed-off-by — never invents bundles or bullets");
    console.error("commit: prepare/check fail unless days→bundle, bundles→range, and languages→range (all ➕️✏️➖️🟰️); run: bun ./📜️script.ts commit check");
}
function emitCommitAnalysisHint() {
    console.error("commit: write NEW bundle scopes, dates, and bullets on prepare stdin after log + diff attribution");
}
/** 📜️Prior commit messages since last WIP (context for dates only — not for copying bullets). */
function emitCommitLog(root) {
    root = gitRepoRoot(root);
    var resolved = resolveCommitBundleRange(root);
    if (!resolved) {
        console.error("commit: no prior bundle WIP commit (subject …🔀️) found in recent history");
        process.exit(1);
    }
    var wip = resolved.wip, range = resolved.range;
    console.error("Last bundle WIP: ".concat(wip.subject, " (").concat(wip.sha.slice(0, 7), ")"));
    console.error("Range: ".concat(range, "\n"));
    emitCommitBundleAttributionNote();
    console.error("\n=== prior commit messages (context only — do not copy bullets; old format may be wrong) ===\n");
    var log = git(root, ["log", "--format=commit %h%n%s%n%b%n---", range]);
    if (log.ok && log.out)
        console.error(log.out);
    emitCommitAnalysisHint();
}
/** 📊️Git diff since last WIP (primary source for new bundle summaries). */
function emitCommitDiff(root) {
    root = gitRepoRoot(root);
    var resolved = resolveCommitBundleRange(root);
    if (!resolved) {
        console.error("commit: no prior bundle WIP commit (subject …🔀️) found in recent history");
        process.exit(1);
    }
    var wip = resolved.wip, range = resolved.range;
    console.error("Last bundle WIP: ".concat(wip.subject, " (").concat(wip.sha.slice(0, 7), ")"));
    console.error("Range: ".concat(range, "\n"));
    var stat = git(root, ["diff", "--stat", range]);
    emitCommitBundleAttributionNote();
    console.error("\n=== git diff --stat (overview) ===\n");
    if (stat.ok && stat.out)
        console.error("".concat(stat.out, "\n"));
    var patch = git(root, ["diff", range]);
    console.error("=== git diff (write bullets from this — not from prior commit text) ===\n");
    if (!patch.ok || !patch.out) {
        console.error(patch.out || "(empty diff)\n");
        emitCommitAnalysisHint();
        return;
    }
    var bytes = Buffer.byteLength(patch.out, "utf8");
    if (bytes > COMMIT_DIFF_MAX_BYTES) {
        console.error(patch.out.slice(0, COMMIT_DIFF_MAX_BYTES));
        console.error("\n[commit diff truncated at ".concat(COMMIT_DIFF_MAX_BYTES, " bytes of ").concat(bytes, " \u2014 inspect locally: git diff ").concat(range, "]\n"));
    }
    else {
        console.error("".concat(patch.out, "\n"));
    }
    emitCommitAnalysisHint();
}
function emitCommitAnalysis(root) {
    emitCommitLog(root);
    console.error("\n");
    emitCommitDiff(root);
}
function runCommit(root, segments) {
    var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k, _l, _m, _o, _p, _q;
    root = gitRepoRoot(root);
    var cmd = (_a = segments[0]) !== null && _a !== void 0 ? _a : "prepare";
    var branchError = branchValidationError("commit", currentBranch(root));
    if (branchError) {
        console.error(branchError);
        process.exit(1);
    }
    var contributor = findContributor(root);
    if (!contributor) {
        console.error("commit: no contributor for git user.email ".concat(gitEmail(root) || "(unset)"));
        process.exit(1);
    }
    if (cmd === "log") {
        emitCommitLog(root);
        process.exit(0);
    }
    if (cmd === "diff") {
        emitCommitDiff(root);
        process.exit(0);
    }
    if (cmd === "analyze") {
        emitCommitAnalysis(root);
        process.exit(0);
    }
    if (cmd === "check") {
        var dash_1 = segments.indexOf("--");
        var bodyFile_1 = dash_1 >= 0 ? ((_b = segments[dash_1 + 1]) !== null && _b !== void 0 ? _b : null) : null;
        var body_1 = readBodyInput(root, bodyFile_1);
        if (!body_1.trim()) {
            console.error("commit check: pass bundle body on stdin or after -- body.txt");
            process.exit(1);
        }
        var bodyErr = commitBundleBodyError(body_1);
        if (bodyErr) {
            console.error(bodyErr);
            process.exit(1);
        }
        var wip_1 = findLastBundleWipCommit(root);
        if (!wip_1) {
            console.error("commit: no prior bundle WIP commit (subject …🔀️) found");
            process.exit(1);
        }
        var ahead_1 = git(root, ["rev-list", "--count", "".concat(wip_1.sha, "..HEAD")]);
        if (!ahead_1.ok || Number(ahead_1.out) === 0) {
            console.error("commit: no commits after last bundle WIP — nothing to check");
            process.exit(1);
        }
        try {
            var bundles = parseCommitBundleBody(body_1);
            validateBundleBulletsFresh(root, wip_1.sha, "HEAD", bundles);
            validateBundleCommitAttribution(root, wip_1.sha, "HEAD", bundles);
        }
        catch (e) {
            console.error(e instanceof Error ? e.message : String(e));
            process.exit(1);
        }
        var range = rangeGitDeltaTotal(root, wip_1.sha, "HEAD");
        console.error("commit check: OK \u2014 ".concat(formatGitDeltaSumBrief(range)));
        console.error("commit check: per-bundle days → bundle headers → WIP range total; per-language footer → same total");
        process.exit(0);
    }
    if (cmd !== "prepare") {
        console.error("[commit] usage: bun ./📜️script.ts commit <log|diff|analyze|check|prepare> [ct|cs|cp|…] [-- body.txt]");
        process.exit(1);
    }
    var dash = segments.indexOf("--");
    var levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
    var bodyFile = dash >= 0 ? ((_c = segments[dash + 1]) !== null && _c !== void 0 ? _c : null) : null;
    var steps = loadCommitSteps(root, contributor, levelSegments);
    var prepareOnly = isCommitPrepareOnly(levelSegments);
    var messagePath = (0, node_path_1.join)(gitDir(root), "compose-commit-message");
    var wip = findLastBundleWipCommit(root);
    if (!wip) {
        console.error("commit: no prior bundle WIP commit (subject …🔀️) found");
        process.exit(1);
    }
    var ahead = git(root, ["rev-list", "--count", "".concat(wip.sha, "..HEAD")]);
    if (!ahead.ok || Number(ahead.out) === 0) {
        console.error("commit: no commits after last bundle WIP — nothing to bundle");
        process.exit(1);
    }
    var body = readBodyInput(root, bodyFile);
    var message;
    if (body.trim()) {
        var bodyErr = commitBundleBodyError(body);
        if (bodyErr) {
            console.error(bodyErr);
            process.exit(1);
        }
        var bundles = void 0;
        try {
            bundles = parseCommitBundleBody(body);
            validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
        }
        catch (e) {
            console.error(e instanceof Error ? e.message : String(e));
            process.exit(1);
        }
        try {
            message = buildCommitMessage(root, contributor, bundles, wip.sha);
        }
        catch (e) {
            console.error(e instanceof Error ? e.message : String(e));
            process.exit(1);
        }
        (0, node_fs_1.writeFileSync)(messagePath, message);
    }
    else if ((0, node_fs_1.existsSync)(messagePath)) {
        message = (0, node_fs_1.readFileSync)(messagePath, "utf8");
        if (!message.trim()) {
            console.error("commit: compose-commit-message is empty — run prepare with bundle body first");
            process.exit(1);
        }
    }
    else {
        emitCommitAnalysis(root);
        process.exit(1);
    }
    if (prepareOnly) {
        if (!body.trim()) {
            emitCommitAnalysis(root);
            process.exit(1);
        }
        var tagName = formatBundleTagName(contributor);
        emitStdout(formatCommitPrepareAgentReply({
            tagName: tagName,
            wipSha: wip.sha,
            messageFile: ".git/compose-commit-message",
            commitMessage: message,
        }));
        process.exit(0);
    }
    if (steps.tag || steps.squash || steps.push)
        assertCleanWorktree(root);
    if (steps.tag) {
        var tagName = formatBundleTagName(contributor);
        var tag = (0, node_child_process_1.spawnSync)("git", ["tag", "-s", "-m", tagName, tagName, "HEAD"], { cwd: root, encoding: "utf8" });
        if (tag.status !== 0) {
            console.error(((_e = (_d = tag.stderr) !== null && _d !== void 0 ? _d : tag.stdout) !== null && _e !== void 0 ? _e : "git tag failed").trim());
            process.exit((_f = tag.status) !== null && _f !== void 0 ? _f : 1);
        }
    }
    if (steps.squash) {
        var reset = (0, node_child_process_1.spawnSync)("git", ["reset", "--soft", wip.sha], { cwd: root, encoding: "utf8" });
        if (reset.status !== 0) {
            console.error(((_h = (_g = reset.stderr) !== null && _g !== void 0 ? _g : reset.stdout) !== null && _h !== void 0 ? _h : "git reset --soft failed").trim());
            process.exit((_j = reset.status) !== null && _j !== void 0 ? _j : 1);
        }
        (0, node_fs_1.writeFileSync)((0, node_path_1.join)(gitDir(root), "COMMIT_EDITMSG"), message);
        var commit = (0, node_child_process_1.spawnSync)("git", ["commit", "-S", "-F", (0, node_path_1.join)(gitDir(root), "COMMIT_EDITMSG")], {
            cwd: root,
            encoding: "utf8",
        });
        if (commit.status !== 0) {
            console.error(((_l = (_k = commit.stderr) !== null && _k !== void 0 ? _k : commit.stdout) !== null && _l !== void 0 ? _l : "git commit failed").trim());
            process.exit((_m = commit.status) !== null && _m !== void 0 ? _m : 1);
        }
    }
    if (steps.push) {
        var push = (0, node_child_process_1.spawnSync)("git", ["push", "--follow-tags"], { cwd: root, encoding: "utf8" });
        if (push.status !== 0) {
            console.error(((_p = (_o = push.stderr) !== null && _o !== void 0 ? _o : push.stdout) !== null && _p !== void 0 ? _p : "git push failed").trim());
            process.exit((_q = push.status) !== null && _q !== void 0 ? _q : 1);
        }
    }
    emitStdout(message);
    process.exit(0);
}
//#endregion 🔖️commit
//#region 📻️SVG Export
/** 📻️ Encodes an SVG with bounded frame writes and publishes only a completed MP4. */
function exportAnimatedSvgToMp4(inputSvgPath_1, outputMp4Path_1) {
    return __awaiter(this, arguments, void 0, function (inputSvgPath, outputMp4Path, options) {
        var fps, duration, total, controller, output, chromium, browser, encoder, encoded, staging, force, terminate, cancel, abort, page, size, width, height, artifact, _loop_8, frame, error, error_1;
        var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k, _l, _m, _o;
        if (options === void 0) { options = {}; }
        return __generator(this, function (_p) {
            switch (_p.label) {
                case 0:
                    fps = (_a = options.fps) !== null && _a !== void 0 ? _a : 60;
                    duration = (_b = options.durationSeconds) !== null && _b !== void 0 ? _b : Number((_d = (_c = (0, node_fs_1.readFileSync)(inputSvgPath, "utf8").match(/dur="([\d.]+)s"/)) === null || _c === void 0 ? void 0 : _c[1]) !== null && _d !== void 0 ? _d : 10);
                    if (![fps, duration, (_e = options.width) !== null && _e !== void 0 ? _e : 1, (_f = options.height) !== null && _f !== void 0 ? _f : 1].every(function (value) { return Number.isFinite(value) && value > 0; }))
                        throw new Error("SVG export dimensions, frame rate and duration must be positive finite numbers");
                    total = Math.ceil(fps * duration), controller = new AbortController();
                    output = (0, node_path_1.resolve)(outputMp4Path);
                    return [4 /*yield*/, Promise.resolve("".concat(PLAYWRIGHT_MODULE_SPECIFIER)).then(function (s) { return require(s); })];
                case 1:
                    chromium = (_p.sent()).chromium;
                    terminate = function () {
                        if (!encoder || encoder.exitCode !== null || encoder.signalCode !== null)
                            return;
                        encoder.kill("SIGTERM");
                        force !== null && force !== void 0 ? force : (force = setTimeout(function () { return encoder === null || encoder === void 0 ? void 0 : encoder.kill("SIGKILL"); }, 5000));
                        force.unref();
                    };
                    cancel = function () { var _a, _b; return controller.abort((_b = (_a = options.signal) === null || _a === void 0 ? void 0 : _a.reason) !== null && _b !== void 0 ? _b : new Error("SVG export cancelled")); };
                    abort = function () { terminate(); void (browser === null || browser === void 0 ? void 0 : browser.close().catch(function () { })); };
                    controller.signal.addEventListener("abort", abort, { once: true });
                    (_g = options.signal) === null || _g === void 0 ? void 0 : _g.addEventListener("abort", cancel, { once: true });
                    process.once("SIGINT", cancel);
                    process.once("SIGTERM", cancel);
                    _p.label = 2;
                case 2:
                    _p.trys.push([2, 15, 16, 20]);
                    if ((_h = options.signal) === null || _h === void 0 ? void 0 : _h.aborted)
                        cancel();
                    controller.signal.throwIfAborted();
                    return [4 /*yield*/, chromium.launch({ headless: true })];
                case 3:
                    browser = _p.sent();
                    controller.signal.throwIfAborted();
                    return [4 /*yield*/, browser.newPage()];
                case 4:
                    page = _p.sent();
                    return [4 /*yield*/, page.goto((0, node_url_1.pathToFileURL)((0, node_path_1.resolve)(inputSvgPath)).href)];
                case 5:
                    _p.sent();
                    return [4 /*yield*/, page.waitForSelector("svg")];
                case 6:
                    _p.sent();
                    return [4 /*yield*/, page.evaluate(function () {
                            var svg = document.querySelector("svg");
                            return { width: svg.viewBox.baseVal.width || svg.width.baseVal.value || 1920, height: svg.viewBox.baseVal.height || svg.height.baseVal.value || 1080 };
                        })];
                case 7:
                    size = _p.sent();
                    width = Math.ceil(((_j = options.width) !== null && _j !== void 0 ? _j : size.width) / 2) * 2, height = Math.ceil(((_k = options.height) !== null && _k !== void 0 ? _k : size.height) / 2) * 2;
                    return [4 /*yield*/, page.setViewportSize({ width: width, height: height })];
                case 8:
                    _p.sent();
                    return [4 /*yield*/, page.evaluate(function () { return document.querySelector("svg").pauseAnimations(); })];
                case 9:
                    _p.sent();
                    (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(output), { recursive: true });
                    staging = (0, node_fs_1.mkdtempSync)((0, node_path_1.join)((0, node_path_1.dirname)(output), ".".concat((0, node_path_1.basename)(output), "-stage-")));
                    artifact = (0, node_path_1.join)(staging, "animation.mp4");
                    encoder = (0, node_child_process_1.spawn)("ffmpeg", ["-nostdin", "-loglevel", "error", "-y", "-f", "image2pipe", "-vcodec", "png", "-r", String(fps), "-i", "-", "-c:v", "libx264", "-pix_fmt", "yuv420p", artifact], { stdio: ["pipe", "ignore", "inherit"] });
                    encoded = new Promise(function (accept) { encoder.once("error", accept); encoder.once("close", function (code) { return accept(code === 0 ? undefined : new Error("ffmpeg exited with code ".concat(code))); }); });
                    encoder.stdin.on("error", function () { });
                    (_l = options.progress) === null || _l === void 0 ? void 0 : _l.call(options, { completed: 0, total: total });
                    _loop_8 = function (frame) {
                        var buffer;
                        return __generator(this, function (_q) {
                            switch (_q.label) {
                                case 0:
                                    controller.signal.throwIfAborted();
                                    return [4 /*yield*/, page.evaluate(function (time) { return document.querySelector("svg").setCurrentTime(time); }, frame / fps)];
                                case 1:
                                    _q.sent();
                                    return [4 /*yield*/, page.screenshot({ omitBackground: true })];
                                case 2:
                                    buffer = _q.sent();
                                    return [4 /*yield*/, new Promise(function (accept, reject) { return encoder.stdin.write(buffer, function (error) { return error ? reject(error) : accept(); }); })];
                                case 3:
                                    _q.sent();
                                    (_m = options.progress) === null || _m === void 0 ? void 0 : _m.call(options, { completed: frame + 1, total: total });
                                    return [2 /*return*/];
                            }
                        });
                    };
                    frame = 0;
                    _p.label = 10;
                case 10:
                    if (!(frame < total)) return [3 /*break*/, 13];
                    return [5 /*yield**/, _loop_8(frame)];
                case 11:
                    _p.sent();
                    _p.label = 12;
                case 12:
                    frame++;
                    return [3 /*break*/, 10];
                case 13:
                    encoder.stdin.end();
                    return [4 /*yield*/, encoded];
                case 14:
                    error = _p.sent();
                    controller.signal.throwIfAborted();
                    if (error)
                        throw error;
                    (0, node_fs_1.renameSync)(artifact, output);
                    return [3 /*break*/, 20];
                case 15:
                    error_1 = _p.sent();
                    controller.signal.throwIfAborted();
                    throw error_1;
                case 16:
                    terminate();
                    if (!encoded) return [3 /*break*/, 18];
                    return [4 /*yield*/, encoded];
                case 17:
                    _p.sent();
                    _p.label = 18;
                case 18:
                    if (force)
                        clearTimeout(force);
                    return [4 /*yield*/, (browser === null || browser === void 0 ? void 0 : browser.close())];
                case 19:
                    _p.sent();
                    if (staging)
                        (0, node_fs_1.rmSync)(staging, { recursive: true, force: true });
                    (_o = options.signal) === null || _o === void 0 ? void 0 : _o.removeEventListener("abort", cancel);
                    controller.signal.removeEventListener("abort", abort);
                    process.removeListener("SIGINT", cancel);
                    process.removeListener("SIGTERM", cancel);
                    return [7 /*endfinally*/];
                case 20: return [2 /*return*/];
            }
        });
    });
}
//#endregion 📻️SVG Export
//#region 🔣️TaxonomyDiscovery
/** 🔣️ Shared taxonomy vocabulary + repo-wide package discovery contract — see
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`. */
__exportStar(require("./\uD83D\uDD0D\uFE0Fdiscovery/\uD83D\uDFE6\uFE0F.ts"), exports);
var ____ts_9 = require("./\uD83D\uDD0D\uFE0Fdiscovery/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "cargoProviderTomlParser", { enumerable: true, get: function () { return ____ts_9.cargoProviderTomlParser; } });
Object.defineProperty(exports, "inspectMutationMetadataSource", { enumerable: true, get: function () { return ____ts_9.inspectMutationMetadataSource; } });
Object.defineProperty(exports, "projectCargoProviderManifest", { enumerable: true, get: function () { return ____ts_9.projectCargoProviderManifest; } });
Object.defineProperty(exports, "resolveCargoProviderBinding", { enumerable: true, get: function () { return ____ts_9.resolveCargoProviderBinding; } });
//#endregion 🔣️TaxonomyDiscovery
var ____ts_10 = require("./\uD83C\uDFD7\uFE0Fbuilder/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "authorArtifactScaffold", { enumerable: true, get: function () { return ____ts_10.authorArtifactScaffold; } });
Object.defineProperty(exports, "ArtifactScaffoldError", { enumerable: true, get: function () { return ____ts_10.ArtifactScaffoldError; } });
//#region 🗂️Workspaces
/** 🗂️ Generated root `package.json` `workspaces` array from a real on-disk package scan — see
 * `26/08/06/GENERATED-BUN-WORKSPACES-FROM-PACKAGE-CATALOG`. */
__exportStar(require("./\uD83D\uDDC2\uFE0Fworkspaces/\uD83D\uDFE6\uFE0F.ts"), exports);
//#endregion 🗂️Workspaces
//#region 🏃️Process
/** 🏃️ Budgeted command execution, workspace bin resolution and the build-mode switch — owned by
 * `📚️library/🏃️process/🟦️.ts` so tool spawners need no taxonomy walk. */
__exportStar(require("./\uD83C\uDFC3\uFE0Fprocess/\uD83D\uDFE6\uFE0F.ts"), exports);
//#endregion 🏃️Process
//#region 🎮️Playground
/** 🎮️ Generated playground catalog, dev/test port table and locked-example define — owned by
 * `📚️library/🎮️playground/🟦️.ts` so port consumers need no taxonomy walk. */
__exportStar(require("./\uD83C\uDFAE\uFE0Fplayground/\uD83D\uDFE6\uFE0F.ts"), exports);
var ____ts_11 = require("./\uD83C\uDFAE\uFE0Fplayground/\uD83E\uDDED\uFE0Fselection/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "loadFrameworkOsPlaygroundSelections", { enumerable: true, get: function () { return ____ts_11.loadFrameworkOsPlaygroundSelections; } });
var ____ts_12 = require("./\uD83C\uDFAE\uFE0Fplayground/\uD83C\uDF10\uFE0Fsite/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "playgroundVariantsFromCrateManifest", { enumerable: true, get: function () { return ____ts_12.playgroundVariantsFromCrateManifest; } });
Object.defineProperty(exports, "registerPlaygroundSiteBuildCommands", { enumerable: true, get: function () { return ____ts_12.registerPlaygroundSiteBuildCommands; } });
//#endregion 🎮️Playground
