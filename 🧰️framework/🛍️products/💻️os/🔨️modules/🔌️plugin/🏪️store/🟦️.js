"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.encodeOwnedZip = encodeOwnedZip;
exports.decodeOwnedZip = decodeOwnedZip;
//#region 🔌️Adapters
var node_zlib_1 = require("node:zlib");
var MAX_ZIP_BYTES = 256 * 1024 * 1024;
var MAX_ENTRY_COUNT = 4096;
var MAX_ENTRY_NAME_BYTES = 4096;
var MAX_ENTRY_BYTES = 256 * 1024 * 1024;
var MAX_TOTAL_BYTES = 512 * 1024 * 1024;
var LOCAL_HEADER = 0x04034b50;
var CENTRAL_HEADER = 0x02014b50;
var END_HEADER = 0x06054b50;
var UTF8_FLAG = 1 << 11;
var DEFLATE_METHOD = 8;
var DOS_DATE_1980_01_01 = 1 << 21;
//#endregion 🔖️OwnedZipContract
//#region 🧮️Primitives
var crcTable = Uint32Array.from({ length: 256 }, function (_, value) {
    var crc = value;
    for (var bit = 0; bit < 8; bit++)
        crc = crc & 1 ? 0xedb88320 ^ (crc >>> 1) : crc >>> 1;
    return crc >>> 0;
});
function crc32(bytes) {
    var crc = 0xffffffff;
    for (var _i = 0, bytes_1 = bytes; _i < bytes_1.length; _i++) {
        var byte = bytes_1[_i];
        crc = crcTable[(crc ^ byte) & 0xff] ^ (crc >>> 8);
    }
    return (crc ^ 0xffffffff) >>> 0;
}
function view(bytes) {
    return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}
function assertRange(bytes, offset, length, context) {
    if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(length) || offset < 0 || length < 0 || offset + length > bytes.length)
        throw new Error("invalid extension zip ".concat(context));
}
function u16(bytes, offset) {
    assertRange(bytes, offset, 2, "u16 range");
    return view(bytes).getUint16(offset, true);
}
function u32(bytes, offset) {
    assertRange(bytes, offset, 4, "u32 range");
    return view(bytes).getUint32(offset, true);
}
function set16(bytes, offset, value) {
    view(bytes).setUint16(offset, value, true);
}
function set32(bytes, offset, value) {
    view(bytes).setUint32(offset, value >>> 0, true);
}
function decodeName(bytes) {
    if (bytes.length === 0 || bytes.length > MAX_ENTRY_NAME_BYTES)
        throw new Error("invalid extension zip entry name length");
    var name;
    try {
        name = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    }
    catch (_a) {
        throw new Error("invalid extension zip UTF-8 entry name");
    }
    validateName(name);
    return name;
}
function validateName(name) {
    var segments = name.split("/");
    if (!name || name.startsWith("/") || name.includes("\\") || name.includes("\0") || segments.some(function (segment) { return !segment || segment === "." || segment === ".."; })) {
        throw new Error("invalid extension zip entry name ".concat(JSON.stringify(name)));
    }
}
function findEndHeader(bytes) {
    var minimum = Math.max(0, bytes.length - 22 - 0xffff);
    for (var offset = bytes.length - 22; offset >= minimum; offset--) {
        if (u32(bytes, offset) === END_HEADER && offset + 22 + u16(bytes, offset + 20) === bytes.length)
            return offset;
    }
    throw new Error("invalid extension zip end record");
}
//#endregion 🧮️Primitives
//#region 📦️Codec
/** @emoji 📦️ Encodes deterministic UTF-8 ZIP/DEFLATE bytes for extension packages. */
function encodeOwnedZip(files) {
    if (files.size === 0 || files.size > MAX_ENTRY_COUNT)
        throw new Error("invalid extension zip entry count");
    var rows = [];
    var localLength = 0;
    var totalBytes = 0;
    for (var _i = 0, files_1 = files; _i < files_1.length; _i++) {
        var _a = files_1[_i], name_1 = _a[0], payload = _a[1];
        validateName(name_1);
        var nameBytes = new TextEncoder().encode(name_1);
        if (nameBytes.length > MAX_ENTRY_NAME_BYTES)
            throw new Error("invalid extension zip entry name length");
        if (payload.length > MAX_ENTRY_BYTES || totalBytes + payload.length > MAX_TOTAL_BYTES)
            throw new Error("extension zip decoded size limit exceeded");
        var compressed = new Uint8Array((0, node_zlib_1.deflateRawSync)(payload, { level: 6 }));
        var row = { name: nameBytes, payload: payload, compressed: compressed, crc: crc32(payload), localOffset: localLength };
        rows.push(row);
        localLength += 30 + nameBytes.length + compressed.length;
        totalBytes += payload.length;
    }
    var centralLength = rows.reduce(function (sum, row) { return sum + 46 + row.name.length; }, 0);
    var outputLength = localLength + centralLength + 22;
    if (outputLength > MAX_ZIP_BYTES)
        throw new Error("extension zip encoded size limit exceeded");
    var output = new Uint8Array(outputLength);
    var localOffset = 0;
    for (var _b = 0, rows_1 = rows; _b < rows_1.length; _b++) {
        var row = rows_1[_b];
        set32(output, localOffset, LOCAL_HEADER);
        set16(output, localOffset + 4, 20);
        set16(output, localOffset + 6, UTF8_FLAG);
        set16(output, localOffset + 8, DEFLATE_METHOD);
        set32(output, localOffset + 10, DOS_DATE_1980_01_01);
        set32(output, localOffset + 14, row.crc);
        set32(output, localOffset + 18, row.compressed.length);
        set32(output, localOffset + 22, row.payload.length);
        set16(output, localOffset + 26, row.name.length);
        output.set(row.name, localOffset + 30);
        output.set(row.compressed, localOffset + 30 + row.name.length);
        localOffset += 30 + row.name.length + row.compressed.length;
    }
    var centralOffset = localLength;
    for (var _c = 0, rows_2 = rows; _c < rows_2.length; _c++) {
        var row = rows_2[_c];
        set32(output, centralOffset, CENTRAL_HEADER);
        set16(output, centralOffset + 4, 20);
        set16(output, centralOffset + 6, 20);
        set16(output, centralOffset + 8, UTF8_FLAG);
        set16(output, centralOffset + 10, DEFLATE_METHOD);
        set32(output, centralOffset + 12, DOS_DATE_1980_01_01);
        set32(output, centralOffset + 16, row.crc);
        set32(output, centralOffset + 20, row.compressed.length);
        set32(output, centralOffset + 24, row.payload.length);
        set16(output, centralOffset + 28, row.name.length);
        set32(output, centralOffset + 42, row.localOffset);
        output.set(row.name, centralOffset + 46);
        centralOffset += 46 + row.name.length;
    }
    set32(output, centralOffset, END_HEADER);
    set16(output, centralOffset + 8, rows.length);
    set16(output, centralOffset + 10, rows.length);
    set32(output, centralOffset + 12, centralLength);
    set32(output, centralOffset + 16, localLength);
    return output;
}
/** @emoji 🔓️ Decodes bounded UTF-8 ZIP entries using stored or raw-DEFLATE payloads. */
function decodeOwnedZip(bytes) {
    if (bytes.length < 22 || bytes.length > MAX_ZIP_BYTES)
        throw new Error("invalid extension zip encoded size");
    var endOffset = findEndHeader(bytes);
    if (u16(bytes, endOffset + 4) !== 0 || u16(bytes, endOffset + 6) !== 0)
        throw new Error("multi-disk extension zip is unsupported");
    var entryCount = u16(bytes, endOffset + 10);
    if (entryCount === 0 || entryCount !== u16(bytes, endOffset + 8) || entryCount > MAX_ENTRY_COUNT)
        throw new Error("invalid extension zip entry count");
    var centralLength = u32(bytes, endOffset + 12);
    var centralStart = u32(bytes, endOffset + 16);
    assertRange(bytes, centralStart, centralLength, "central directory range");
    if (centralStart + centralLength !== endOffset)
        throw new Error("invalid extension zip central directory boundary");
    var files = new Map();
    var centralOffset = centralStart;
    var totalBytes = 0;
    for (var index = 0; index < entryCount; index++) {
        if (u32(bytes, centralOffset) !== CENTRAL_HEADER)
            throw new Error("invalid extension zip central header");
        var flags = u16(bytes, centralOffset + 8);
        var method = u16(bytes, centralOffset + 10);
        if (flags & 1)
            throw new Error("encrypted extension zip entries are unsupported");
        if (method !== 0 && method !== DEFLATE_METHOD)
            throw new Error("unsupported extension zip compression method ".concat(method));
        var expectedCrc = u32(bytes, centralOffset + 16);
        var compressedLength = u32(bytes, centralOffset + 20);
        var payloadLength = u32(bytes, centralOffset + 24);
        var nameLength = u16(bytes, centralOffset + 28);
        var extraLength = u16(bytes, centralOffset + 30);
        var commentLength = u16(bytes, centralOffset + 32);
        var disk = u16(bytes, centralOffset + 34);
        var localOffset = u32(bytes, centralOffset + 42);
        if (disk !== 0 || compressedLength === 0xffffffff || payloadLength === 0xffffffff || localOffset === 0xffffffff)
            throw new Error("ZIP64 extension packages are unsupported");
        assertRange(bytes, centralOffset + 46, nameLength + extraLength + commentLength, "central entry range");
        var name_2 = decodeName(bytes.subarray(centralOffset + 46, centralOffset + 46 + nameLength));
        if (files.has(name_2))
            throw new Error("duplicate extension zip entry ".concat(name_2));
        if (payloadLength > MAX_ENTRY_BYTES || totalBytes + payloadLength > MAX_TOTAL_BYTES)
            throw new Error("extension zip decoded size limit exceeded");
        if (u32(bytes, localOffset) !== LOCAL_HEADER)
            throw new Error("invalid extension zip local header");
        var localFlags = u16(bytes, localOffset + 6);
        var localMethod = u16(bytes, localOffset + 8);
        if (localFlags & 1 || localMethod !== method)
            throw new Error("extension zip local header mismatch");
        var localNameLength = u16(bytes, localOffset + 26);
        var localExtraLength = u16(bytes, localOffset + 28);
        assertRange(bytes, localOffset + 30, localNameLength + localExtraLength, "local entry range");
        var localName = decodeName(bytes.subarray(localOffset + 30, localOffset + 30 + localNameLength));
        if (localName !== name_2)
            throw new Error("extension zip local header mismatch");
        var payloadOffset = localOffset + 30 + localNameLength + localExtraLength;
        assertRange(bytes, payloadOffset, compressedLength, "compressed payload range");
        if (payloadOffset + compressedLength > centralStart)
            throw new Error("invalid extension zip compressed payload boundary");
        var compressed = bytes.subarray(payloadOffset, payloadOffset + compressedLength);
        var payload = void 0;
        try {
            payload = method === 0 ? new Uint8Array(compressed) : new Uint8Array((0, node_zlib_1.inflateRawSync)(compressed, { maxOutputLength: Math.min(payloadLength + 1, MAX_ENTRY_BYTES + 1) }));
        }
        catch (_a) {
            throw new Error("invalid extension zip compressed payload ".concat(name_2));
        }
        if (payload.length !== payloadLength || crc32(payload) !== expectedCrc)
            throw new Error("invalid extension zip checksum ".concat(name_2));
        files.set(name_2, payload);
        totalBytes += payload.length;
        centralOffset += 46 + nameLength + extraLength + commentLength;
    }
    if (centralOffset !== endOffset)
        throw new Error("invalid extension zip central entry count");
    return files;
}
//#endregion 📦️Codec
