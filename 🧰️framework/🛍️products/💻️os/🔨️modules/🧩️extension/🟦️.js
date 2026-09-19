"use strict";
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
exports.installationDirectoryEmoji = installationDirectoryEmoji;
exports.installationDirectoryCollision = installationDirectoryCollision;
var ____json_1 = require("./\uD83E\uDDEC\uFE0Fschema/\uD83D\uDD23\uFE0F.json");
var schema = ____json_1.default.$defs.InstallationDirectoryV1;
var pattern = new RegExp(schema.pattern, "u");
var segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });
/** 🪪️Validates an authored installation basename and returns its folded sibling emoji identity. */
function installationDirectoryEmoji(name) {
    if (typeof name !== "string" || __spreadArray([], name, true).length > schema.maxLength || name !== name.normalize("NFC") || !pattern.test(name))
        throw new Error("Installation directory requires one explicit non-generic emoji and a portable slug");
    return __spreadArray([], segmenter.segment(name), true)[0].segment.replaceAll("\uFE0F", "");
}
/** ⚠️Finds a file or directory sibling occupying the declared emoji identity. */
function installationDirectoryCollision(name, siblings) {
    var emoji = installationDirectoryEmoji(name);
    return siblings.find(function (sibling) { var _a; return ((_a = __spreadArray([], segmenter.segment(sibling.normalize("NFC")), true)[0]) === null || _a === void 0 ? void 0 : _a.segment.replaceAll("\uFE0F", "").replaceAll("\uFE0E", "")) === emoji; });
}
