"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SEMIO_ASSET_ROUTE = exports.SEMIO_ASSET_DIRECTORY = void 0;
exports.parseAssetDeliveryAuthority = parseAssetDeliveryAuthority;
exports.assetPathFromRequest = assetPathFromRequest;
exports.assetTransportUrl = assetTransportUrl;
var ___delivery_json_1 = require("../\uD83D\uDE9A\uFE0Fdelivery.json");
/** 🚚️ Admits the single asset-owned publication directory without aliases. */
function parseAssetDeliveryAuthority(value) {
    if (!value || typeof value !== "object" || Array.isArray(value))
        throw new Error("Invalid asset delivery authority");
    var record = value;
    if (Object.keys(record).length !== 3 || record.$schema !== "./🧬️schema/🔣️.json" || record.version !== 1 || record.directoryName !== "🖼️assets")
        throw new Error("Unknown asset delivery authority");
    return value;
}
exports.SEMIO_ASSET_DIRECTORY = parseAssetDeliveryAuthority(___delivery_json_1.default).directoryName;
exports.SEMIO_ASSET_ROUTE = "/".concat(exports.SEMIO_ASSET_DIRECTORY);
function assetRelativePath(path) {
    return !/[\\%?#\u0000-\u001f\u007f]/u.test(path) && path.split("/").every(function (segment) { return segment.length > 0 && segment !== "." && segment !== ".."; });
}
/** 🔎️ Resolves raw or once-encoded request paths inside the exact asset namespace. */
function assetPathFromRequest(target) {
    var encoded = target.split(/[?#]/u, 1)[0];
    if (/%(?:2f|5c)/iu.test(encoded))
        return null;
    var path;
    try {
        path = decodeURIComponent(encoded);
    }
    catch (_a) {
        return null;
    }
    if (!path.startsWith("".concat(exports.SEMIO_ASSET_ROUTE, "/")))
        return null;
    var relative = path.slice(exports.SEMIO_ASSET_ROUTE.length + 1);
    return assetRelativePath(relative) ? relative : null;
}
/** 🔗️ Encodes an admitted source-relative asset path without changing its identity. */
function assetTransportUrl(path) {
    if (!assetRelativePath(path))
        throw new Error("Invalid relative asset path");
    return "/".concat(encodeURIComponent(exports.SEMIO_ASSET_DIRECTORY), "/").concat(path.split("/").map(encodeURIComponent).join("/"));
}
