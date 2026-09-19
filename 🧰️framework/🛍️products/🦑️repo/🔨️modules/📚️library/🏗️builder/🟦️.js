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
exports.ArtifactScaffoldError = void 0;
exports.authorArtifactScaffold = authorArtifactScaffold;
var node_crypto_1 = require("node:crypto");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../\uD83D\uDD0D\uFE0Fdiscovery/\uD83D\uDFE6\uFE0F.ts");
/** 🧾️ Retains exact observed publication evidence without claiming all-or-nothing rollback. */
var ArtifactScaffoldError = /** @class */ (function (_super) {
    __extends(ArtifactScaffoldError, _super);
    function ArtifactScaffoldError(cause, partial) {
        var _this = _super.call(this, "Artifact scaffold authoring failed: ".concat(cause instanceof Error ? cause.message : String(cause)), { cause: cause }) || this;
        _this.name = "ArtifactScaffoldError";
        _this.partial = Object.freeze(__assign(__assign({}, partial), { created: Object.freeze(partial.created.map(function (entry) { return Object.freeze(__assign({}, entry)); })), skipped: Object.freeze(__spreadArray([], partial.skipped, true)), directories: Object.freeze(partial.directories.map(function (entry) { return Object.freeze(__assign({}, entry)); })) }));
        return _this;
    }
    return ArtifactScaffoldError;
}(Error));
exports.ArtifactScaffoldError = ArtifactScaffoldError;
function scaffoldStat(path) {
    try {
        return (0, node_fs_1.lstatSync)(path);
    }
    catch (error) {
        if (error.code === "ENOENT")
            return null;
        throw error;
    }
}
function scaffoldCoordinate(path, taxonomy) {
    var parts = path.split("/");
    if (!path || path !== path.normalize(taxonomy.unicodeNormalization.form) || parts.some(function (part) { return !part || part === "." || part === ".." || /[\\\0:]/u.test(part); }) || (0, ____ts_1.taxonomyRelativePathIsExcluded)(path, taxonomy))
        throw new Error("Invalid authoring coordinate: ".concat(JSON.stringify(path)));
    return parts;
}
function scaffoldKind(name, parentKindId, taxonomy) {
    var kind = (0, ____ts_1.semanticDirectoryKindId)(name, taxonomy, { parentKindId: parentKindId });
    if (!kind)
        throw new Error("Unregistered authoring directory: ".concat(JSON.stringify(name)));
    var spec = taxonomy.semanticDirectoryKinds[kind], members = taxonomy.semanticDirectoryMemberKinds[kind];
    if (spec && !name.startsWith(spec.emoji) || !spec && !(members === null || members === void 0 ? void 0 : members.memberNames.includes(name)))
        throw new Error("Unregistered authoring directory: ".concat(JSON.stringify(name)));
    return kind;
}
function scaffoldOwner(owner, taxonomy) {
    var _a, _b;
    var contract = taxonomy.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
    if ((contract === null || contract === void 0 ? void 0 : contract.contractKind) !== "semantic-facet-primary-file" || contract.sourceDisposition !== "authored" || contract.authoringCommand.writeDisposition !== "create-if-absent")
        throw new Error("The authored empty-facet authority is required");
    var root = scaffoldCoordinate(contract.sourceRoot, taxonomy), parts = scaffoldCoordinate(owner.subsetPath, taxonomy);
    if (root.some(function (part, index) { return parts[index] !== part; }) || parts.length !== root.length + 7)
        throw new Error("Authoring owner must be an exact artifact standard/subset path");
    var captures = ["plugin", null, "artifact", null, "standard", null, "subset"], names = [null, taxonomy.artifactsDirName, null, taxonomy.standardsDirName, null, taxonomy.subsetsDirName, null];
    var kind = "plugins";
    for (var _i = 0, _c = parts.slice(root.length).entries(); _i < _c.length; _i++) {
        var _d = _c[_i], index = _d[0], name_1 = _d[1];
        kind = scaffoldKind(name_1, kind, taxonomy);
        var capture = captures[index], rule = capture ? contract.directoryCaptures[capture] : null;
        if (capture ? !(rule === null || rule === void 0 ? void 0 : rule.kindIds.includes(kind)) || rule.names && !rule.names.includes(name_1) : name_1 !== names[index])
            throw new Error("Wrong structural authoring owner at ".concat(JSON.stringify(name_1)));
    }
    if (owner.kind === "subset")
        return { path: owner.subsetPath, kind: kind, required: parts.slice(0, -2).join("/") };
    if (owner.kind !== "surface" || !taxonomy.surfaceRoles.includes(owner.role))
        throw new Error("Unknown artifact surface role");
    var surface = taxonomy.surfaceDirNames[owner.role];
    if (!surface || !((_b = (_a = contract.directoryCaptures.surface) === null || _a === void 0 ? void 0 : _a.names) === null || _b === void 0 ? void 0 : _b.includes(surface)))
        throw new Error("Surface role lacks exact schema ownership");
    var schemaDirs = taxonomy.subsetComponentDirs.filter(function (name) { return (0, ____ts_1.semanticDirectoryKindId)(name, taxonomy, { parentKindId: kind }) === "schema"; });
    if (schemaDirs.length !== 1)
        throw new Error("Surface owner requires one registered schema facet");
    return { path: "".concat(owner.subsetPath, "/").concat(surface), kind: scaffoldKind(surface, kind, taxonomy), required: "".concat(owner.subsetPath, "/").concat(schemaDirs[0]) };
}
function scaffoldLeafOwner(path, authority, owner, taxonomy) {
    var contract = taxonomy.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
    if ((contract === null || contract === void 0 ? void 0 : contract.contractKind) !== "semantic-facet-primary-file")
        throw new Error("The authored empty-facet authority is required");
    var state = owner.kind, kind = authority.kind;
    for (var _i = 0, _a = path.slice(authority.path.length + 1).split("/").slice(0, -1); _i < _a.length; _i++) {
        var name_2 = _a[_i];
        var childKind = scaffoldKind(name_2, kind, taxonomy);
        var capture = state === "modes" ? contract.directoryCaptures.mode : state === "windows" ? contract.directoryCaptures.window : null;
        var allowed = state === "subset" ? taxonomy.subsetChildDirs : state === "surface" ? taxonomy.surfaceRequiredChildDirs : state === "mode" ? taxonomy.modeRequiredChildDirs : state === "window" ? taxonomy.windowRequiredChildDirs : state === "io" ? taxonomy.ioSemanticCollectionDirNames : state === "collection" ? taxonomy.representationDirs : [];
        if (capture ? !capture.kindIds.includes(childKind) || capture.names && !capture.names.includes(name_2) : !allowed.includes(name_2))
            throw new Error("Unpermitted authoring child: ".concat(name_2));
        if (state === "subset")
            state = taxonomy.subsetSurfaceDirs.includes(name_2) ? "surface" : taxonomy.subsetComponentDirs.includes(name_2) ? childKind === "io" ? "io" : "component" : "facet";
        else if (state === "surface")
            state = name_2 === taxonomy.modesDirName ? "modes" : "facet";
        else if (state === "modes")
            state = "mode";
        else if (state === "mode")
            state = name_2 === taxonomy.windowsDirName ? "windows" : "facet";
        else if (state === "windows")
            state = "window";
        else if (state === "window")
            state = "facet";
        else if (state === "io")
            state = "collection";
        else if (state === "collection")
            state = "component";
        kind = childKind;
    }
    var markdown = (0, node_path_1.basename)(path) === (0, ____ts_1.canonicalPrimaryFilenameForKind)(taxonomy.windowEmptyFacetFileKindId, taxonomy);
    if (markdown ? !["facet", "collection"].includes(state) : !["subset", "surface", "mode", "window", "io", "component"].includes(state))
        throw new Error("Unowned authored leaf: ".concat(path));
}
function scaffoldIdentity(path, stat) {
    return { path: path, device: Number(stat.dev), inode: Number(stat.ino), mode: Number(stat.mode) & 4095 };
}
function scaffoldSameVersion(left, right) {
    return ["dev", "ino", "mode", "size", "mtimeNs", "ctimeNs"].every(function (key) { return left[key] === right[key]; });
}
function scaffoldFile(fd, path, progress) {
    var stat = (0, node_fs_1.fstatSync)(fd, { bigint: true }), size = Number(stat.size);
    if (!stat.isFile())
        throw new Error("Authoring node is not a regular file: ".concat(path));
    if (!Number.isSafeInteger(size))
        throw new Error("Authoring file is too large to inspect exactly: ".concat(path));
    var digest = (0, node_crypto_1.createHash)("sha256"), buffer = Buffer.alloc(65536);
    for (var position = 0; position < size;) {
        var count = (0, node_fs_1.readSync)(fd, buffer, 0, Math.min(buffer.length, size - position), position);
        if (!count)
            throw new Error("Authoring file changed while reading: ".concat(path));
        digest.update(buffer.subarray(0, count));
        position += count;
        progress(position);
    }
    var after = (0, node_fs_1.fstatSync)(fd, { bigint: true });
    if (!scaffoldSameVersion(stat, after))
        throw new Error("Authoring file changed while reading: ".concat(path));
    return __assign(__assign({}, scaffoldIdentity(path, stat)), { bytes: size, sha256: digest.digest("hex") });
}
function scaffoldExistingFile(absolute, path, progress) {
    var _a;
    var before = (0, node_fs_1.lstatSync)(absolute, { bigint: true });
    if (!before.isFile() || before.isSymbolicLink())
        throw new Error("Authoring target is not a regular file: ".concat(path));
    var fd = (0, node_fs_1.openSync)(absolute, node_fs_1.constants.O_RDONLY | ((_a = node_fs_1.constants.O_NOFOLLOW) !== null && _a !== void 0 ? _a : 0));
    try {
        if (!scaffoldSameVersion(before, (0, node_fs_1.fstatSync)(fd, { bigint: true })))
            throw new Error("Authoring target identity changed: ".concat(path));
        var evidence = scaffoldFile(fd, path, progress), current = (0, node_fs_1.lstatSync)(absolute, { bigint: true });
        if (!current.isFile() || current.isSymbolicLink() || !scaffoldSameVersion(before, current))
            throw new Error("Authoring target identity changed: ".concat(path));
        return evidence;
    }
    finally {
        (0, node_fs_1.closeSync)(fd);
    }
}
/** 🏗️ Preflights all targets and publishes exclusively under cooperative identity rechecks, retaining partial output on failure. */
function authorArtifactScaffold(repoRoot, owner, leaves, taxonomy, options) {
    var _a;
    if (options === void 0) { options = {}; }
    var result = { created: [], skipped: [] }, created = [], directories = [];
    var known = new Map(), existing = new Map();
    var failedPath = null;
    var cancel = function () { var _a; if ((_a = options.cancelled) === null || _a === void 0 ? void 0 : _a.call(options))
        throw new Error("Authoring cancelled"); };
    var emit = function (phase, path, bytesRead) { var _a; (_a = options.progress) === null || _a === void 0 ? void 0 : _a.call(options, __assign(__assign({ phase: phase, path: path }, (bytesRead === undefined ? {} : { bytesRead: bytesRead })), { current: result.created.length + result.skipped.length, total: leaves.length })); cancel(); };
    var directory = function (absolute, required) {
        var stat = scaffoldStat(absolute);
        if (!stat) {
            if (required)
                throw new Error("Missing governing authoring directory: ".concat(absolute));
            return;
        }
        if (!stat.isDirectory() || stat.isSymbolicLink())
            throw new Error("Authoring ancestor is not a no-follow directory: ".concat(absolute));
        var identity = scaffoldIdentity(absolute, stat), previous = known.get(absolute);
        if (previous && (previous.device !== identity.device || previous.inode !== identity.inode || previous.mode !== identity.mode))
            throw new Error("Authoring ancestor identity changed: ".concat(absolute));
        known.set(absolute, identity);
    };
    var ancestry = function (absolute, required) {
        var cursor = (0, node_path_1.parse)(absolute).root;
        directory(cursor, true);
        for (var _i = 0, _a = (0, node_path_1.relative)(cursor, absolute).split(node_path_1.sep); _i < _a.length; _i++) {
            var part = _a[_i];
            cursor = (0, node_path_1.join)(cursor, part);
            directory(cursor, required);
        }
    };
    var recheck = function () { for (var _i = 0, _a = known.keys(); _i < _a.length; _i++) {
        var path = _a[_i];
        directory(path, true);
    } };
    try {
        cancel();
        if (!(0, node_path_1.isAbsolute)(repoRoot) || (0, node_path_1.resolve)(repoRoot) !== repoRoot)
            throw new Error("Authoring repository root must be an exact absolute directory");
        var authority = scaffoldOwner(owner, taxonomy), proposed = leaves.map(function (leaf) { return ({ path: leaf.path, content: leaf.content }); });
        var allowed = new Set(__spreadArray([taxonomy.windowEmptyFacetFileKindId], Object.values(taxonomy.componentFileKinds), true).map(function (kind) { return (0, ____ts_1.canonicalPrimaryFilenameForKind)(kind, taxonomy); }));
        var targets = new Set();
        ancestry(repoRoot, true);
        ancestry((0, node_path_1.join)(repoRoot, authority.required), true);
        if (!proposed.length)
            throw new Error("Authoring request must contain leaves");
        var _loop_1 = function (leaf) {
            failedPath = leaf.path;
            scaffoldCoordinate(leaf.path, taxonomy);
            if (typeof leaf.content !== "string" || !leaf.path.startsWith("".concat(authority.path, "/")) || !allowed.has((0, node_path_1.basename)(leaf.path)) || targets.has(leaf.path) || (0, ____ts_1.generatorContractIdsForOutputPath)(leaf.path, taxonomy).length)
                throw new Error("Invalid authored leaf request: ".concat(leaf.path));
            targets.add(leaf.path);
            scaffoldLeafOwner(leaf.path, authority, owner, taxonomy);
            var absolute = (0, node_path_1.join)(repoRoot, leaf.path);
            ancestry((0, node_path_1.dirname)(absolute), false);
            if (scaffoldStat(absolute))
                existing.set(leaf.path, scaffoldExistingFile(absolute, leaf.path, function (bytes) { return emit("reading", leaf.path, bytes); }));
            cancel();
        };
        for (var _i = 0, proposed_1 = proposed; _i < proposed_1.length; _i++) {
            var leaf = proposed_1[_i];
            _loop_1(leaf);
        }
        failedPath = null;
        recheck();
        emit("preflight");
        recheck();
        var _loop_2 = function (leaf) {
            failedPath = leaf.path;
            cancel();
            var absolute = (0, node_path_1.join)(repoRoot, leaf.path), previous = existing.get(leaf.path);
            recheck();
            if (previous) {
                if (JSON.stringify(scaffoldExistingFile(absolute, leaf.path, function (bytes) { return emit("reading", leaf.path, bytes); })) !== JSON.stringify(previous))
                    throw new Error("Existing authored leaf changed: ".concat(leaf.path));
                result.skipped.push(leaf.path);
                emit("skipped", leaf.path);
                return "continue";
            }
            if (options.dryRun) {
                result.created.push(leaf.path);
                return "continue";
            }
            var cursor = repoRoot;
            for (var _c = 0, _d = (0, node_path_1.relative)(repoRoot, (0, node_path_1.dirname)(absolute)).split(node_path_1.sep); _c < _d.length; _c++) {
                var name_3 = _d[_c];
                cursor = (0, node_path_1.join)(cursor, name_3);
                recheck();
                if (!scaffoldStat(cursor)) {
                    var owned = false;
                    try {
                        (0, node_fs_1.mkdirSync)(cursor);
                        owned = true;
                    }
                    catch (error) {
                        if (error.code !== "EEXIST")
                            throw error;
                    }
                    directory(cursor, true);
                    if (owned)
                        directories.push(__assign(__assign({}, known.get(cursor)), { path: (0, node_path_1.relative)(repoRoot, cursor).replaceAll("\\", "/") }));
                }
                else
                    directory(cursor, true);
            }
            emit("before-create", leaf.path);
            recheck();
            var fd = void 0;
            try {
                fd = (0, node_fs_1.openSync)(absolute, node_fs_1.constants.O_RDWR | node_fs_1.constants.O_CREAT | node_fs_1.constants.O_EXCL | ((_a = node_fs_1.constants.O_NOFOLLOW) !== null && _a !== void 0 ? _a : 0), 438);
            }
            catch (error) {
                if (error.code !== "EEXIST")
                    throw error;
                scaffoldExistingFile(absolute, leaf.path, function (bytes) { return emit("reading", leaf.path, bytes); });
                result.skipped.push(leaf.path);
                emit("skipped", leaf.path);
                return "continue";
            }
            var index = created.push(__assign(__assign({}, scaffoldIdentity(leaf.path, (0, node_fs_1.fstatSync)(fd))), { bytes: 0, sha256: (0, node_crypto_1.createHash)("sha256").digest("hex") })) - 1;
            try {
                (0, node_fs_1.writeFileSync)(fd, leaf.content, "utf8");
                (0, node_fs_1.fsyncSync)(fd);
            }
            finally {
                try {
                    created[index] = scaffoldFile(fd, leaf.path, function (bytes) { return emit("reading", leaf.path, bytes); });
                }
                catch (error) {
                    var stat = (0, node_fs_1.fstatSync)(fd);
                    created[index] = __assign(__assign({}, scaffoldIdentity(leaf.path, stat)), { bytes: stat.size, sha256: null });
                    throw error;
                }
                finally {
                    (0, node_fs_1.closeSync)(fd);
                }
            }
            var current = scaffoldStat(absolute), evidence = created[index];
            if (!(current === null || current === void 0 ? void 0 : current.isFile()) || current.isSymbolicLink() || current.dev !== evidence.device || current.ino !== evidence.inode)
                throw new Error("Published authored leaf identity changed: ".concat(leaf.path));
            result.created.push(leaf.path);
            emit("created", leaf.path);
        };
        for (var _b = 0, proposed_2 = proposed; _b < proposed_2.length; _b++) {
            var leaf = proposed_2[_b];
            _loop_2(leaf);
        }
        failedPath = null;
        recheck();
        emit("complete");
        recheck();
        return result;
    }
    catch (error) {
        throw new ArtifactScaffoldError(error, { created: created, skipped: result.skipped, directories: directories, failedPath: failedPath });
    }
}
