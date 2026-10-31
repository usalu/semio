"""🎒️ Independent Python implementation of the `stdio.semio` record-document pack: the semio binary envelope around a pack container (`SPK` header, CRC-32C framed segments, fixed-Huffman deflate, manifest, footer with the BLAKE3 content hash) whose document segment is a derived record.

The codec is schema-first. It knows no subset: it is driven by the exported record spec of a subset
(`<subset>/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`, the language-neutral `PackSchemaGraph` the Rust derive
exports and a Rust law keeps equal to the derive). Every byte is derived from the committed wire contract
(`🧰️framework/🔨️modules/🎒️pack`), never from the Rust code it is measured against, so a disagreement with the
Rust record pack is a finding.

Two views of one document meet here:

* the **record** — `dict[field id -> FieldValue]` with `FieldValue = (kind, payload)`, the exact tree the wire carries;
* the **document** — the camelCase JSON snapshot every mutation oracle already works on.

`encode_pack(spec, document)` and `decode_pack(spec, data)` cross both, `encode_record`/`decode_record` stay on the wire view.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import struct
import zlib
from typing import Any, Dict, List, Optional, Tuple

# endregion 🔖️Imports


# region 🔖️Constants
BINARY_MAGIC = bytes([0x89, 0x53, 0x45, 0x4D, 0x0D, 0x0A, 0x1A, 0x0A])
PACK_MAGIC = bytes([0x89, 0x53, 0x50, 0x4B, 0x0D, 0x0A, 0x1A, 0x0A])
FOOTER_MAGIC = b"SPKFOOT1"
HEADER_SIZE = 32
FOOTER_SIZE = 84
REQUIRED_COMPRESSED = 1
REQUIRED_CHUNKED = 2
OPTIONAL_CANONICAL = 1
KIND_END = 0
KIND_MANIFEST = 1
KIND_SYMBOLS = 3
KIND_DOCUMENT = 4
KIND_CHUNK = 5
KIND_CHUNK_TABLE = 6
CODEC_DEFLATE = 1
CHUNK_THRESHOLD = 256 * 1024
CHUNK_SIZE = 1024 * 1024
FRAME_SIZE = 1024 * 1024
MAX_DEPTH = 64
MAX_SEGMENT_RAW = 1 << 26
U64 = (1 << 64) - 1

TAG_ABSENT, TAG_FALSE, TAG_TRUE, TAG_INT, TAG_UINT, TAG_F64, TAG_STR, TAG_STR_INLINE = 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07
TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_TUPLE, TAG_LIST, TAG_RECORD, TAG_BLOCK, TAG_STATEMENTS = 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
TAG_MAP, TAG_VALUE, TAG_NULL, TAG_WIRE, TAG_TABLE_SOA, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_EXPR = 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
ELEM_FALLBACK, ELEM_BOOL, ELEM_INT, ELEM_UINT, ELEM_F64, ELEM_STR, ELEM_ENUM = 0, 1, 2, 3, 4, 5, 6

SHAPE_TAGS = {
    "bool": 1, "int": 2, "uint": 3, "float": 4, "text": 5, "bytes64": 6, "enum": 7, "tuple": 8, "list": 9, "record": 10,
    "block": 11, "statements": 12, "map": 13, "value": 14, "table": 15, "wire": 16, "quantity": 17, "angle": 18, "ref": 19,
    "coord": 20, "dir": 21, "dim": 22, "range": 23, "count": 24, "expr": 25, "embed": 26, "embedFrom": 27,
}
TUPLE_SHAPES = ("tuple", "coord", "dir", "dim", "range")
FLOAT_SHAPES = ("float", "quantity", "angle")
TEXT_SHAPES = ("text", "ref")
UINT_SHAPES = ("uint", "count")

# endregion 🔖️Constants


# region 🔖️Varint
def write_varint(value: int) -> bytes:
    """🔢️ Unsigned LEB128, the `varint` every length, count, id and ordinal of the pack uses."""
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value == 0:
            out.append(byte)
            return bytes(out)
        out.append(byte | 0x80)


def write_zigzag(value: int) -> bytes:
    """➡️ Signed varint: the zig-zag mapped magnitude as an unsigned LEB128."""
    return write_varint(((value << 1) ^ (value >> 63)) & U64)


class Reader:
    """📖️ Bounds-checked cursor over a byte string; every refusal names the offset it happened at."""

    def __init__(self, data: bytes, at: int = 0) -> None:
        self.data = data
        self.at = at

    def take(self, count: int) -> bytes:
        if count < 0 or self.at + count > len(self.data):
            raise ValueError("the pack ends %d byte(s) short at offset %d" % (self.at + count - len(self.data), self.at))
        chunk = self.data[self.at : self.at + count]
        self.at += count
        return chunk

    def byte(self) -> int:
        return self.take(1)[0]

    def varint(self) -> int:
        value = 0
        shift = 0
        for index in range(10):
            byte = self.byte()
            if index == 9 and byte > 1:
                raise ValueError("varint exceeds 64 bits at offset %d" % self.at)
            value |= (byte & 0x7F) << shift
            if byte & 0x80 == 0:
                return value
            shift += 7
        raise ValueError("overlong varint ending at offset %d" % self.at)

    def zigzag(self) -> int:
        raw = self.varint()
        return (raw >> 1) ^ -(raw & 1)

    def float64(self) -> float:
        return struct.unpack("<d", self.take(8))[0]

    def remaining(self) -> int:
        return len(self.data) - self.at


# endregion 🔖️Varint


# region 🔖️Crc32c
def _crc32c_table() -> List[int]:
    table = []
    for index in range(256):
        crc = index
        for _ in range(8):
            crc = (crc >> 1) ^ 0x82F63B78 if crc & 1 else crc >> 1
        table.append(crc)
    return table


_CRC32C = _crc32c_table()


def crc32c(data: bytes) -> int:
    """🧮️ CRC-32C (Castagnoli, reflected) as every header, footer and segment frame stores it."""
    crc = 0xFFFFFFFF
    for byte in data:
        crc = (crc >> 8) ^ _CRC32C[(crc ^ byte) & 0xFF]
    return crc ^ 0xFFFFFFFF


# endregion 🔖️Crc32c


# region 🔖️Blake3
_B3_IV = (0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19)
_B3_PERMUTATION = (2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8)
_B3_CHUNK_START, _B3_CHUNK_END, _B3_PARENT, _B3_ROOT = 1, 2, 4, 8
_M32 = 0xFFFFFFFF


def _b3_compress(chaining: Tuple[int, ...], words: List[int], counter: int, length: int, flags: int) -> List[int]:
    state = list(chaining[:8]) + list(_B3_IV[:4]) + [counter & _M32, (counter >> 32) & _M32, length, flags]

    def mix(a: int, b: int, c: int, d: int, mx: int, my: int) -> None:
        state[a] = (state[a] + state[b] + mx) & _M32
        state[d] = (((state[d] ^ state[a]) >> 16) | ((state[d] ^ state[a]) << 16)) & _M32
        state[c] = (state[c] + state[d]) & _M32
        state[b] = (((state[b] ^ state[c]) >> 12) | ((state[b] ^ state[c]) << 20)) & _M32
        state[a] = (state[a] + state[b] + my) & _M32
        state[d] = (((state[d] ^ state[a]) >> 8) | ((state[d] ^ state[a]) << 24)) & _M32
        state[c] = (state[c] + state[d]) & _M32
        state[b] = (((state[b] ^ state[c]) >> 7) | ((state[b] ^ state[c]) << 25)) & _M32

    block = list(words)
    for round_index in range(7):
        mix(0, 4, 8, 12, block[0], block[1])
        mix(1, 5, 9, 13, block[2], block[3])
        mix(2, 6, 10, 14, block[4], block[5])
        mix(3, 7, 11, 15, block[6], block[7])
        mix(0, 5, 10, 15, block[8], block[9])
        mix(1, 6, 11, 12, block[10], block[11])
        mix(2, 7, 8, 13, block[12], block[13])
        mix(3, 4, 9, 14, block[14], block[15])
        if round_index < 6:
            block = [block[index] for index in _B3_PERMUTATION]
    for index in range(8):
        state[index] ^= state[index + 8]
        state[index + 8] ^= chaining[index]
    return state


def _b3_words(block: bytes) -> List[int]:
    padded = block + bytes(64 - len(block))
    return list(struct.unpack("<16I", padded))


def _b3_chunk_output(chunk: bytes, counter: int) -> Tuple[Tuple[int, ...], List[int], int, int, int]:
    chaining: Tuple[int, ...] = _B3_IV
    blocks = [chunk[at : at + 64] for at in range(0, len(chunk), 64)] or [b""]
    for index, block in enumerate(blocks[:-1]):
        flags = _B3_CHUNK_START if index == 0 else 0
        chaining = tuple(_b3_compress(chaining, _b3_words(block), counter, 64, flags)[:8])
    flags = (_B3_CHUNK_START if len(blocks) == 1 else 0) | _B3_CHUNK_END
    return chaining, _b3_words(blocks[-1]), counter, len(blocks[-1]), flags


def _b3_chaining_value(output: Tuple[Tuple[int, ...], List[int], int, int, int]) -> Tuple[int, ...]:
    return tuple(_b3_compress(*output)[:8])


def _b3_parent_output(left: Tuple[int, ...], right: Tuple[int, ...]) -> Tuple[Tuple[int, ...], List[int], int, int, int]:
    return _B3_IV, list(left) + list(right), 0, 64, _B3_PARENT


def blake3(data: bytes) -> bytes:
    """#️⃣ BLAKE3 with the default 32-byte output, the digest of the document content and of the schema graph."""
    chunks = [data[at : at + 1024] for at in range(0, len(data), 1024)] or [b""]
    stack: List[Tuple[int, ...]] = []
    for index, chunk in enumerate(chunks[:-1]):
        value = _b3_chaining_value(_b3_chunk_output(chunk, index))
        completed = index + 1
        while completed & 1 == 0:
            value = _b3_chaining_value(_b3_parent_output(stack.pop(), value))
            completed >>= 1
        stack.append(value)
    output = _b3_chunk_output(chunks[-1], len(chunks) - 1)
    while stack:
        output = _b3_parent_output(stack.pop(), _b3_chaining_value(output))
    chaining, words, counter, length, flags = output
    return struct.pack("<8I", *_b3_compress(chaining, words, counter, length, flags | _B3_ROOT)[:8])


# endregion 🔖️Blake3


# region 🔖️Deflate
_LENGTH_BASE = (3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258)
_LENGTH_EXTRA = (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0)
_DIST_BASE = (1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577)
_DIST_EXTRA = (0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13)
_WINDOW, _MIN_MATCH, _MAX_MATCH, _MAX_CHAIN, _HASH_BITS = 32768, 3, 258, 128, 15


def _canonical_codes(lengths: List[int]) -> List[int]:
    counts = [0] * (max(lengths) + 1)
    for length in lengths:
        if length:
            counts[length] += 1
    code = 0
    next_code = [0] * (len(counts) + 1)
    for bits in range(1, len(counts)):
        code = (code + counts[bits - 1]) << 1
        next_code[bits] = code
    codes = []
    for length in lengths:
        if length:
            codes.append(next_code[length])
            next_code[length] += 1
        else:
            codes.append(0)
    return codes


_FIXED_LITERAL_LENGTHS = [8] * 144 + [9] * 112 + [7] * 24 + [8] * 8
_FIXED_LITERAL_CODES = _canonical_codes(_FIXED_LITERAL_LENGTHS)
_FIXED_DISTANCE_CODES = _canonical_codes([5] * 30)


def _table_index(bases: Tuple[int, ...], value: int) -> int:
    for index in range(len(bases) - 1, -1, -1):
        if value >= bases[index]:
            return index
    return 0


class _BitWriter:
    def __init__(self) -> None:
        self.out = bytearray()
        self.buffer = 0
        self.bits = 0

    def write(self, value: int, count: int) -> None:
        self.buffer |= (value & ((1 << count) - 1)) << self.bits
        self.bits += count
        while self.bits >= 8:
            self.out.append(self.buffer & 0xFF)
            self.buffer >>= 8
            self.bits -= 8

    def huffman(self, code: int, length: int) -> None:
        for shift in range(length - 1, -1, -1):
            self.write((code >> shift) & 1, 1)

    def finish(self) -> bytes:
        if self.bits:
            self.out.append(self.buffer & 0xFF)
        return bytes(self.out)


def _hash3(a: int, b: int, c: int) -> int:
    return (((a | (b << 8) | (c << 16)) * 2654435761) & _M32) >> (32 - _HASH_BITS)


def deflate(raw: bytes) -> bytes:
    """🗜️ The pack's one deflate: a single fixed-Huffman block over a greedy hash-chain LZ77 (3..258 byte matches, 32 KiB window, 128 probes). Pinned by its own specification so equal input is equal output in every language."""
    writer = _BitWriter()
    writer.write(1, 1)
    writer.write(1, 2)
    head = [-1] * (1 << _HASH_BITS)
    prev = [-1] * len(raw)
    size = len(raw)

    def insert(position: int) -> None:
        if position + _MIN_MATCH <= size:
            slot = _hash3(raw[position], raw[position + 1], raw[position + 2])
            prev[position] = head[slot]
            head[slot] = position

    i = 0
    while i < size:
        best_length = 0
        best_distance = 0
        if i + _MIN_MATCH <= size:
            candidate = head[_hash3(raw[i], raw[i + 1], raw[i + 2])]
            chain = 0
            while candidate != -1 and chain < _MAX_CHAIN:
                if i - candidate > _WINDOW:
                    break
                limit = min(size - i, _MAX_MATCH)
                length = 0
                while length < limit and raw[candidate + length] == raw[i + length]:
                    length += 1
                if length > best_length:
                    best_length = length
                    best_distance = i - candidate
                candidate = prev[candidate]
                chain += 1
        if best_length >= _MIN_MATCH:
            index = _table_index(_LENGTH_BASE, best_length)
            writer.huffman(_FIXED_LITERAL_CODES[257 + index], _FIXED_LITERAL_LENGTHS[257 + index])
            if _LENGTH_EXTRA[index]:
                writer.write(best_length - _LENGTH_BASE[index], _LENGTH_EXTRA[index])
            index = _table_index(_DIST_BASE, best_distance)
            writer.huffman(_FIXED_DISTANCE_CODES[index], 5)
            if _DIST_EXTRA[index]:
                writer.write(best_distance - _DIST_BASE[index], _DIST_EXTRA[index])
            for position in range(i, i + best_length):
                insert(position)
            i += best_length
        else:
            writer.huffman(_FIXED_LITERAL_CODES[raw[i]], _FIXED_LITERAL_LENGTHS[raw[i]])
            insert(i)
            i += 1
    writer.huffman(_FIXED_LITERAL_CODES[256], _FIXED_LITERAL_LENGTHS[256])
    return writer.finish()


def inflate(stored: bytes, raw_length: int) -> bytes:
    """📤️ Raw RFC 1951 inflate through the system zlib, the third-party reader the deflate above is measured against."""
    if raw_length > MAX_SEGMENT_RAW:
        raise ValueError("a deflate segment declares %d byte(s), more than the %d allowed" % (raw_length, MAX_SEGMENT_RAW))
    inflater = zlib.decompressobj(-15)
    raw = inflater.decompress(stored, raw_length + 1)
    if len(raw) != raw_length or inflater.unconsumed_tail:
        raise ValueError("a deflate segment declares %d byte(s) but inflates to %d" % (raw_length, len(raw)))
    return raw


# endregion 🔖️Deflate


# region 🔖️Container
def encode_segment(kind: int, payload: bytes, compressed: bool = True) -> bytes:
    """🖇️ One framed segment: `kind, flags, stored length, [raw length], stored bytes, CRC-32C of everything before the CRC`."""
    if compressed:
        stored = deflate(payload)
        frame = bytes([kind, 1 | (CODEC_DEFLATE << 1)]) + write_varint(len(stored)) + write_varint(len(payload)) + stored
    else:
        frame = bytes([kind, 0]) + write_varint(len(payload)) + payload
    return frame + struct.pack("<I", crc32c(frame))


def decode_segment(reader: Reader) -> Tuple[int, bytes, int]:
    """🚪️ Reads one segment, checks its CRC and returns `(kind, raw payload, wire length)`."""
    start = reader.at
    kind = reader.byte()
    flags = reader.byte()
    stored_length = reader.varint()
    compressed = flags & 1 != 0
    raw_length = reader.varint() if compressed else stored_length
    stored = reader.take(stored_length)
    expected = struct.unpack("<I", reader.take(4))[0]
    if crc32c(reader.data[start : reader.at - 4]) != expected:
        raise ValueError("the segment at offset %d fails its CRC-32C" % start)
    if compressed:
        if (flags >> 1) & 7 != CODEC_DEFLATE:
            raise ValueError("the segment at offset %d names the unsupported codec %d" % (start, (flags >> 1) & 7))
        return kind, inflate(stored, raw_length), reader.at - start
    return kind, stored, reader.at - start


def encode_manifest(schema_hash: bytes, doc: Tuple[int, int], frames: int, symbols: Tuple[int, int], table: Tuple[int, int], body_length: int, field_count: int, chunk_count: int, symbol_count: int) -> bytes:
    """🗺️ The manifest segment payload: the spans and counts that describe every other segment."""
    out = bytearray(write_varint(0))
    out += schema_hash
    for value in (doc[0], doc[1], frames, symbols[0], symbols[1], table[0], table[1], 0, 0, body_length, field_count, chunk_count, symbol_count):
        out += write_varint(value)
    return bytes(out)


def decode_manifest(payload: bytes) -> Dict[str, Any]:
    reader = Reader(payload)
    reader.varint()
    schema_hash = reader.take(32)
    names = ("docOffset", "docLength", "docFrames", "symbolsOffset", "symbolsLength", "tableOffset", "tableLength", "indexOffset", "indexLength", "bodyLength", "fieldCount", "chunkCount", "symbolCount")
    manifest: Dict[str, Any] = {name: reader.varint() for name in names}
    manifest["schemaHash"] = schema_hash
    return manifest


def encode_chunk_table(entries: List[Tuple[int, int, int, int, bytes]]) -> bytes:
    out = bytearray(write_varint(len(entries)))
    for offset, stored, raw, crc, digest in entries:
        out += write_varint(offset) + write_varint(stored) + write_varint(raw) + struct.pack("<I", crc) + digest
    return bytes(out)


def build_pack(schema_hash: bytes, symbols_payload: bytes, body: bytes, chunks: List[bytes]) -> bytes:
    """🏗️ Assembles the container in the order the writer lays it out: header, symbols, raw chunks, document frames, chunk table, manifest, end segment, footer."""
    required = REQUIRED_COMPRESSED | (REQUIRED_CHUNKED if chunks else 0)
    header = PACK_MAGIC + struct.pack("<HHII", 1, 0, required, OPTIONAL_CANONICAL)
    header += struct.pack("<I", crc32c(header)) + bytes(HEADER_SIZE - len(header) - 4)
    out = bytearray(header)
    symbols_start = len(out)
    out += encode_segment(KIND_SYMBOLS, symbols_payload)
    symbols_span = (symbols_start, len(out) - symbols_start)
    entries = []
    for chunk in chunks:
        frame_head = bytes([KIND_CHUNK, 0]) + write_varint(len(chunk))
        payload_offset = len(out) + len(frame_head)
        out += frame_head + chunk + struct.pack("<I", crc32c(frame_head + chunk))
        entries.append((payload_offset, len(chunk), len(chunk), crc32c(chunk), blake3(chunk)))
    doc_start = len(out)
    frames = 0
    for at in range(0, len(body), FRAME_SIZE):
        out += encode_segment(KIND_DOCUMENT, body[at : at + FRAME_SIZE])
        frames += 1
    doc_span = (doc_start, len(out) - doc_start)
    table_span = (0, 0)
    if entries:
        table_start = len(out)
        out += encode_segment(KIND_CHUNK_TABLE, encode_chunk_table(entries), compressed=False)
        table_span = (table_start, len(out) - table_start)
    field_count = Reader(body).varint()
    manifest_start = len(out)
    out += encode_segment(KIND_MANIFEST, encode_manifest(schema_hash, doc_span, frames, symbols_span, table_span, len(body), field_count, len(entries), Reader(symbols_payload).varint()))
    manifest_span = (manifest_start, len(out) - manifest_start)
    out += encode_segment(KIND_END, b"", compressed=False)
    footer = FOOTER_MAGIC + struct.pack("<HHIQQQ", 1, 0, required, manifest_span[0], manifest_span[1], len(out) + FOOTER_SIZE) + blake3(body) + struct.pack("<Q", 0)
    return bytes(out) + footer + struct.pack("<I", crc32c(footer))


def open_pack(inner: bytes) -> Dict[str, Any]:
    """📂️ Verifies the container end to end — header CRC, footer CRC and span, every segment CRC, the document hash — and returns its symbol table and document body."""
    if len(inner) < HEADER_SIZE + FOOTER_SIZE:
        raise ValueError("the pack is %d byte(s), shorter than its header and footer" % len(inner))
    if inner[:8] != PACK_MAGIC:
        raise ValueError("the pack does not start with the SPK magic")
    major, minor, required, optional, stored_crc = struct.unpack("<HHIII", inner[8:24])
    if crc32c(inner[:20]) != stored_crc or major != 1 or required & ~0xF:
        raise ValueError("the pack header is corrupt or of an unsupported version")
    footer = inner[-FOOTER_SIZE:]
    if footer[:8] != FOOTER_MAGIC or crc32c(footer[:80]) != struct.unpack("<I", footer[80:])[0]:
        raise ValueError("the pack footer is corrupt")
    manifest_offset, manifest_length, file_length = struct.unpack("<QQQ", footer[16:40])
    if file_length != len(inner):
        raise ValueError("the footer declares %d byte(s) for a %d byte pack" % (file_length, len(inner)))
    reader = Reader(inner, HEADER_SIZE)
    symbols_payload = b""
    frames: List[bytes] = []
    chunks: List[Tuple[int, bytes]] = []
    manifest: Optional[Dict[str, Any]] = None
    while True:
        at = reader.at
        kind, payload, _ = decode_segment(reader)
        if kind == KIND_SYMBOLS:
            symbols_payload = payload
        elif kind == KIND_DOCUMENT:
            frames.append(payload)
        elif kind == KIND_CHUNK:
            chunks.append((at, payload))
        elif kind == KIND_MANIFEST:
            if at != manifest_offset or reader.at - at != manifest_length:
                raise ValueError("the manifest is not where the footer says")
            manifest = decode_manifest(payload)
        elif kind == KIND_END:
            break
    body = b"".join(frames)
    if reader.at != len(inner) - FOOTER_SIZE or manifest is None:
        raise ValueError("the pack carries bytes or lacks a manifest between its end segment and its footer")
    if blake3(body) != footer[40:72]:
        raise ValueError("the document body does not hash to the footer's content hash")
    if manifest["bodyLength"] != len(body):
        raise ValueError("the manifest body length disagrees with the document")
    return {"manifest": manifest, "symbols": decode_symbols(symbols_payload), "body": body, "chunks": [data for _, data in chunks], "optionalFlags": optional}


def decode_symbols(payload: bytes) -> List[str]:
    reader = Reader(payload)
    symbols = [reader.take(reader.varint()).decode("utf-8") for _ in range(reader.varint())]
    if reader.remaining():
        raise ValueError("the symbol table carries trailing bytes")
    return symbols


def wrap_binary(token: str, inner: bytes) -> bytes:
    """📦️ The semio binary envelope: magic, little-endian u32 token length, the `<envelope>.pack v1` token."""
    raw = token.encode("utf-8")
    return BINARY_MAGIC + struct.pack("<I", len(raw)) + raw + inner


def unwrap_binary(data: bytes) -> Tuple[str, bytes]:
    if len(data) < 12 or data[:8] != BINARY_MAGIC:
        raise ValueError("the file does not start with the semio binary magic")
    length = struct.unpack("<I", data[8:12])[0]
    if 12 + length > len(data):
        raise ValueError("the semio binary envelope ends inside its token")
    return data[12 : 12 + length].decode("utf-8"), data[12 + length :]


# endregion 🔖️Container


# region 🔖️Spec
class RecordSpec:
    """🧭️ The exported record spec: the canonical record graph of a subset snapshot (`semio.pack.record-spec/v1`). Record `0` is the root; every `record`, `table` and `statements` shape names another record by index."""

    def __init__(self, document: Dict[str, Any]) -> None:
        if document.get("schema") != "semio.pack.record-spec/v1":
            raise ValueError("not a semio.pack.record-spec/v1 document")
        self.envelope: str = document["envelope"]
        self.expected_hash: str = document["schemaHash"]
        self.graph: List[List[Dict[str, Any]]] = document["graph"]
        self.fields: List[Dict[int, Dict[str, Any]]] = [{field["id"]: field for field in record} for record in self.graph]
        annotations = document.get("annotations", {})
        self.single = {(record, field) for record, field in annotations.get("single", [])}
        self.float32 = {(record, field) for record, field in annotations.get("float32", [])}
        self.tags = {(record, field): name for record, field, name in annotations.get("tags", [])}
        self.verbatim = {(record, field) for record, field in annotations.get("verbatim", [])}
        self.nested = {record: (name, members) for record, name, members in annotations.get("nested", [])}

    @property
    def token(self) -> str:
        return "%s.pack v1" % self.envelope

    def canonical_bytes(self) -> bytes:
        """🧾️ `varint(records)` then per record `varint(fields)` and per field `varint(id) text(key) flags shape` — the bytes the schema hash covers."""

        def text(value: str) -> bytes:
            raw = value.encode("utf-8")
            return write_varint(len(raw)) + raw

        def shape(node: Dict[str, Any]) -> bytes:
            kind = node["kind"]
            tag = bytes([SHAPE_TAGS[kind]])
            if kind == "enum":
                return tag + write_varint(len(node["variants"])) + b"".join(write_varint(ordinal) + text(name) for ordinal, name in node["variants"])
            if kind == "tuple":
                return tag + shape(node["item"]) + (b"\x00" if node["len"] is None else b"\x01" + write_varint(node["len"]))
            if kind in ("list", "block", "map"):
                return tag + shape(node["item"])
            if kind in ("record", "table"):
                return tag + write_varint(node["record"])
            if kind == "statements":
                return tag + write_varint(len(node["variants"])) + b"".join(text(keyword) + write_varint(index) for keyword, index in node["variants"])
            if kind in ("quantity", "angle"):
                return tag + text(node["unit"])
            if kind == "ref":
                return tag + text(node["entity"])
            if kind in ("coord", "dim"):
                return tag + bytes([node["dims"]])
            if kind == "embed":
                return tag + text(node["lang"])
            if kind == "embedFrom":
                return tag + text(node["key"])
            return tag

        out = bytearray(write_varint(len(self.graph)))
        for record in self.graph:
            out += write_varint(len(record))
            for field in record:
                out += write_varint(field["id"]) + text(field["key"]) + bytes([int(field["optional"]) | (int(field["flatten"]) << 1)]) + shape(field["shape"])
        return bytes(out)

    def schema_hash(self) -> bytes:
        """🔑️ BLAKE3 over the canonical bytes, recomputed here and required to equal the hash the export carries."""
        return blake3(self.canonical_bytes())


def load_record_spec(path: str) -> RecordSpec:
    """📥️ Reads one committed `🗺️record-spec/🔣️.json` and refuses it when its carried hash is not the one its graph hashes to."""
    with open(path, "r", encoding="utf-8") as handle:
        spec = RecordSpec(json.load(handle))
    if spec.schema_hash().hex() != spec.expected_hash:
        raise ValueError("the record spec graph hashes to %s, not the carried %s" % (spec.schema_hash().hex(), spec.expected_hash))
    return spec


# endregion 🔖️Spec


# region 🔖️Encode
def numeric_run(items: List[Tuple[str, Any]]) -> Optional[str]:
    """➗️ Whether a sequence is the mandatory packed form: every element a float (`f64`) or every element an int, enum or `u64 <= i64::MAX` (`varint`); empty runs stay plain."""
    if not items:
        return None
    kinds = {kind for kind, _ in items}
    if kinds == {"float"}:
        return "f64"
    if kinds in ({"int"}, {"enum"}):
        return "varint"
    if kinds == {"uint"} and all(value <= (1 << 63) - 1 for _, value in items):
        return "varint"
    return None


def elem_tag(shape: Dict[str, Any]) -> int:
    """🏷️ The column element tag of a table column from its declared shape."""
    kind = shape["kind"]
    if kind in FLOAT_SHAPES:
        return ELEM_F64
    if kind == "int":
        return ELEM_INT
    if kind in UINT_SHAPES:
        return ELEM_UINT
    if kind == "enum":
        return ELEM_ENUM
    if kind == "bool":
        return ELEM_BOOL
    if kind in TEXT_SHAPES:
        return ELEM_STR
    return ELEM_FALLBACK


def element_shape(shape: Optional[Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    return shape["item"] if shape is not None and shape["kind"] in ("tuple", "list") else None


def dsl_strings(value: Any, counts: Dict[str, int]) -> None:
    if isinstance(value, str):
        counts[value] = counts.get(value, 0) + 1
    elif isinstance(value, list):
        for item in value:
            dsl_strings(item, counts)
    elif isinstance(value, dict):
        for item in value.values():
            dsl_strings(item, counts)


class Encoder:
    """✍️ One document encode: the deterministic interning pre-pass, then the tag-prefixed, canonical record body."""

    def __init__(self, spec: RecordSpec) -> None:
        self.spec = spec
        self.symbols: List[str] = []
        self.index: Dict[str, int] = {}
        self.chunks: List[bytes] = []

    def intern(self, record: Dict[int, Any]) -> None:
        counts: Dict[str, int] = {}
        forced = set()

        def note(text: str) -> None:
            counts[text] = counts.get(text, 0) + 1

        def force(text: str) -> None:
            forced.add(text)
            note(text)

        def walk_record(spec_index: Optional[int], fields: Dict[int, Any]) -> None:
            for field_id in sorted(fields):
                shape = self.spec.fields[spec_index][field_id]["shape"] if spec_index is not None and field_id in self.spec.fields[spec_index] else None
                walk(shape, fields[field_id])

        def walk(shape: Optional[Dict[str, Any]], value: Tuple[str, Any]) -> None:
            kind, payload = value
            if kind == "text":
                note(payload)
            elif kind == "tuple" or (kind == "list" and (shape is None or shape["kind"] != "table")):
                for item in payload:
                    walk(element_shape(shape), item)
            elif kind == "list":
                columns = self.spec.graph[shape["record"]]
                for row in payload:
                    for column in columns:
                        cell = row[1].get(column["id"])
                        if cell is None:
                            continue
                        if column["shape"]["kind"] in TEXT_SHAPES:
                            if cell[0] == "text":
                                force(cell[1])
                        else:
                            walk(column["shape"], cell)
            elif kind == "record":
                walk_record(shape["record"] if shape is not None and shape["kind"] == "record" else None, payload)
            elif kind == "block":
                walk(shape["item"] if shape is not None and shape["kind"] == "block" else None, payload)
            elif kind == "statements":
                variants = dict(shape["variants"]) if shape is not None and shape["kind"] == "statements" else {}
                for keyword, fields in payload:
                    force(keyword)
                    walk_record(variants.get(keyword), fields)
            elif kind == "map":
                inner = shape["item"] if shape is not None and shape["kind"] == "map" else None
                for key, item in payload:
                    note(key)
                    walk(inner, item)
            elif kind == "value":
                dsl_strings(payload, counts)

        walk_record(0, record)
        chosen = set(forced) | {text for text, count in counts.items() if len(text.encode("utf-8")) <= 128 or count >= 2}
        self.symbols = sorted(chosen, key=lambda text: text.encode("utf-8"))
        self.index = {text: position for position, text in enumerate(self.symbols)}

    def string(self, text: str) -> bytes:
        if text in self.index:
            return bytes([TAG_STR]) + write_varint(self.index[text])
        return self.inline(text)

    @staticmethod
    def inline(text: str) -> bytes:
        raw = text.encode("utf-8")
        return bytes([TAG_STR_INLINE]) + write_varint(len(raw)) + raw

    def forced(self, text: str) -> bytes:
        return write_varint(self.index[text])

    def record_fields(self, spec_index: Optional[int], fields: Dict[int, Any], depth: int) -> bytes:
        if depth > MAX_DEPTH:
            raise ValueError("the record nests deeper than %d" % MAX_DEPTH)
        known = self.spec.fields[spec_index] if spec_index is not None else {}
        ids = sorted(field_id for field_id in fields if field_id in known or spec_index is None)
        out = bytearray(write_varint(len(ids)))
        for field_id in ids:
            out += write_varint(field_id) + self.value(known[field_id]["shape"] if field_id in known else None, fields[field_id], depth + 1)
        return bytes(out)

    def value(self, shape: Optional[Dict[str, Any]], value: Tuple[str, Any], depth: int) -> bytes:
        if depth > MAX_DEPTH:
            raise ValueError("the record nests deeper than %d" % MAX_DEPTH)
        kind, payload = value
        if kind == "bool":
            return bytes([TAG_TRUE if payload else TAG_FALSE])
        if kind == "int":
            return bytes([TAG_INT]) + write_zigzag(payload)
        if kind == "uint":
            return bytes([TAG_UINT]) + write_varint(payload)
        if kind == "float":
            return bytes([TAG_F64]) + struct.pack("<d", payload)
        if kind == "text":
            return self.string(payload)
        if kind == "bytes":
            return self.bytes_value(payload)
        if kind == "enum":
            return bytes([TAG_ENUM]) + write_varint(payload)
        if kind == "tuple":
            return self.sequence(shape, payload, True, depth)
        if kind == "list":
            if shape is not None and shape["kind"] == "table":
                return self.table(shape["record"], payload, depth)
            return self.sequence(shape, payload, False, depth)
        if kind == "record":
            nested = shape["record"] if shape is not None and shape["kind"] == "record" else None
            return bytes([TAG_RECORD]) + self.record_fields(nested, payload, depth + 1)
        if kind == "block":
            return bytes([TAG_BLOCK]) + self.value(shape["item"] if shape is not None and shape["kind"] == "block" else None, payload, depth + 1)
        if kind == "statements":
            variants = dict(shape["variants"]) if shape is not None and shape["kind"] == "statements" else {}
            out = bytearray([TAG_STATEMENTS]) + write_varint(len(payload))
            for keyword, fields in payload:
                out += self.forced(keyword) + self.record_fields(variants.get(keyword), fields, depth + 1)
            return bytes(out)
        if kind == "map":
            inner = shape["item"] if shape is not None and shape["kind"] == "map" else None
            entries = sorted(payload, key=lambda entry: entry[0].encode("utf-8"))
            out = bytearray([TAG_MAP]) + write_varint(len(entries))
            for key, item in entries:
                out += self.string(key) + self.value(inner, item, depth + 1)
            return bytes(out)
        if kind == "value":
            return bytes([TAG_VALUE]) + self.dsl_value(payload, depth + 1)
        raise ValueError("the %s field value has no wire encoding here" % kind)

    def bytes_value(self, payload: bytes) -> bytes:
        if len(payload) < CHUNK_THRESHOLD:
            return bytes([TAG_BYTES]) + write_varint(len(payload)) + payload
        ids = []
        for at in range(0, len(payload), CHUNK_SIZE):
            self.chunks.append(payload[at : at + CHUNK_SIZE])
            ids.append(len(self.chunks) - 1)
        return bytes([TAG_BYTES_CHUNKED]) + write_varint(len(ids)) + b"".join(write_varint(chunk_id) for chunk_id in ids)

    def sequence(self, shape: Optional[Dict[str, Any]], items: List[Tuple[str, Any]], is_tuple: bool, depth: int) -> bytes:
        run = numeric_run(items)
        if run == "f64":
            return bytes([TAG_PACKED_F64]) + write_varint(len(items)) + b"".join(struct.pack("<d", value) for _, value in items)
        if run == "varint":
            return bytes([TAG_PACKED_VARINT]) + write_varint(len(items)) + b"".join(write_zigzag(value) for _, value in items)
        out = bytearray([TAG_TUPLE if is_tuple else TAG_LIST]) + write_varint(len(items))
        inner = element_shape(shape)
        for item in items:
            out += self.value(inner, item, depth + 1)
        return bytes(out)

    def table(self, spec_index: int, rows: List[Tuple[str, Any]], depth: int) -> bytes:
        columns = sorted(self.spec.graph[spec_index], key=lambda column: column["id"])
        out = bytearray([TAG_TABLE_SOA]) + write_varint(len(rows)) + write_varint(len(columns))
        for column in columns:
            cells = [row[1].get(column["id"]) for row in rows]
            present = [cell is not None for cell in cells]
            out += write_varint(column["id"])
            if all(present):
                out.append(0)
            else:
                bitmap = bytearray((len(rows) + 7) // 8)
                for position, flag in enumerate(present):
                    if flag:
                        bitmap[position // 8] |= 1 << (position % 8)
                out.append(1)
                out += bitmap
            tag = elem_tag(column["shape"])
            out.append(tag)
            if tag == ELEM_BOOL:
                bits = bytearray((len(rows) + 7) // 8)
                for position, cell in enumerate(cells):
                    if cell is not None and cell[1]:
                        bits[position // 8] |= 1 << (position % 8)
                out += bits
                continue
            for cell in cells:
                if cell is None:
                    continue
                if tag == ELEM_F64:
                    out += struct.pack("<d", cell[1])
                elif tag == ELEM_INT:
                    out += write_zigzag(cell[1])
                elif tag in (ELEM_UINT, ELEM_ENUM):
                    out += write_varint(cell[1])
                elif tag == ELEM_STR:
                    out += self.forced(cell[1])
                else:
                    out += self.value(column["shape"], cell, depth + 1)
        return bytes(out)

    def dsl_value(self, value: Any, depth: int) -> bytes:
        if depth > MAX_DEPTH:
            raise ValueError("the value nests deeper than %d" % MAX_DEPTH)
        if value is None:
            return bytes([TAG_NULL])
        if isinstance(value, bool):
            return bytes([TAG_TRUE if value else TAG_FALSE])
        if isinstance(value, Float):
            return bytes([TAG_F64]) + struct.pack("<d", float(value))
        if isinstance(value, int):
            return bytes([TAG_UINT]) + write_varint(value) if value >= 0 else bytes([TAG_INT]) + write_zigzag(value)
        if isinstance(value, float):
            return bytes([TAG_F64]) + struct.pack("<d", value)
        if isinstance(value, str):
            return self.string(value)
        if isinstance(value, bytes):
            return self.bytes_value(value)
        if isinstance(value, list):
            return bytes([TAG_LIST]) + write_varint(len(value)) + b"".join(self.dsl_value(item, depth + 1) for item in value)
        if isinstance(value, dict):
            return bytes([TAG_MAP]) + write_varint(len(value)) + b"".join(self.inline(key) + self.dsl_value(item, depth + 1) for key, item in value.items())
        raise ValueError("the %s value has no dynamic encoding" % type(value).__name__)


class Float(float):
    """🪙️ A dynamic-value number that is a float even when it is whole (`1.0`), so it keeps the `f64` tag."""


def encode_record(spec: RecordSpec, record: Dict[int, Tuple[str, Any]]) -> bytes:
    """🪄️ Encodes a record tree to the container (no envelope): symbols, document frames, chunks, manifest, footer."""
    encoder = Encoder(spec)
    encoder.intern(record)
    body = encoder.record_fields(0, record, 0)
    symbols = bytearray(write_varint(len(encoder.symbols)))
    for text in encoder.symbols:
        raw = text.encode("utf-8")
        symbols += write_varint(len(raw)) + raw
    return build_pack(spec.schema_hash(), bytes(symbols), body, encoder.chunks)


# endregion 🔖️Encode


# region 🔖️Decode
class Decoder:
    """🔬️ One document decode against a record spec: tags are self-describing, the spec only settles tuple versus list, packed element types and nested record shapes."""

    def __init__(self, spec: RecordSpec, symbols: List[str], chunks: List[bytes]) -> None:
        self.spec = spec
        self.symbols = symbols
        self.chunks = chunks

    def symbol(self, index: int) -> str:
        if index >= len(self.symbols):
            raise ValueError("symbol reference %d is outside the %d symbol table" % (index, len(self.symbols)))
        return self.symbols[index]

    def text(self, reader: Reader) -> str:
        tag = reader.byte()
        if tag == TAG_STR:
            return self.symbol(reader.varint())
        if tag == TAG_STR_INLINE:
            return reader.take(reader.varint()).decode("utf-8")
        raise ValueError("expected a string tag, found 0x%02x at offset %d" % (tag, reader.at))

    def record_fields(self, reader: Reader, spec_index: Optional[int], depth: int) -> Dict[int, Tuple[str, Any]]:
        if depth > MAX_DEPTH:
            raise ValueError("the record nests deeper than %d" % MAX_DEPTH)
        fields: Dict[int, Tuple[str, Any]] = {}
        known = self.spec.fields[spec_index] if spec_index is not None else {}
        for _ in range(reader.varint()):
            field_id = reader.varint()
            value = self.value(reader, known[field_id]["shape"] if field_id in known else None, depth + 1)
            if value is not None:
                fields[field_id] = value
        return fields

    def value(self, reader: Reader, shape: Optional[Dict[str, Any]], depth: int) -> Optional[Tuple[str, Any]]:
        if depth > MAX_DEPTH:
            raise ValueError("the record nests deeper than %d" % MAX_DEPTH)
        tag = reader.byte()
        kind = shape["kind"] if shape is not None else None
        if tag == TAG_ABSENT:
            return None
        if tag in (TAG_FALSE, TAG_TRUE):
            return ("bool", tag == TAG_TRUE)
        if tag == TAG_INT:
            return ("int", reader.zigzag())
        if tag == TAG_UINT:
            return ("uint", reader.varint())
        if tag == TAG_F64:
            return ("float", reader.float64())
        if tag == TAG_STR:
            return ("text", self.symbol(reader.varint()))
        if tag == TAG_STR_INLINE:
            return ("text", reader.take(reader.varint()).decode("utf-8"))
        if tag == TAG_BYTES:
            return ("bytes", reader.take(reader.varint()))
        if tag == TAG_BYTES_CHUNKED:
            return ("bytes", b"".join(self.chunks[reader.varint()] for _ in range(reader.varint())))
        if tag == TAG_ENUM:
            return ("enum", reader.varint())
        if tag in (TAG_TUPLE, TAG_LIST):
            inner = element_shape(shape) if tag == TAG_TUPLE or kind != "table" else {"kind": "record", "record": shape["record"]}
            return ("tuple" if tag == TAG_TUPLE else "list", [self.value(reader, inner, depth + 1) for _ in range(reader.varint())])
        if tag == TAG_RECORD:
            return ("record", self.record_fields(reader, shape["record"] if kind == "record" else None, depth + 1))
        if tag == TAG_BLOCK:
            return ("block", self.value(reader, shape["item"] if kind == "block" else None, depth + 1))
        if tag == TAG_STATEMENTS:
            variants = dict(shape["variants"]) if kind == "statements" else {}
            items = []
            for _ in range(reader.varint()):
                keyword = self.symbol(reader.varint())
                items.append((keyword, self.record_fields(reader, variants.get(keyword), depth + 1)))
            return ("statements", items)
        if tag == TAG_MAP:
            inner = shape["item"] if kind == "map" else None
            entries = []
            for _ in range(reader.varint()):
                key = self.text(reader)
                entries.append((key, self.value(reader, inner, depth + 1)))
            return ("map", entries)
        if tag == TAG_VALUE:
            return ("value", self.dsl_value(reader, depth + 1))
        if tag == TAG_TABLE_SOA:
            return ("list", self.table(reader, shape["record"] if kind == "table" else None, depth))
        if tag == TAG_PACKED_F64:
            return ("tuple" if kind in TUPLE_SHAPES else "list", [("float", reader.float64()) for _ in range(reader.varint())])
        if tag == TAG_PACKED_VARINT:
            inner = element_shape(shape) if kind in ("tuple", "list") else shape
            item_kind = "uint" if inner is not None and inner["kind"] in ("uint",) else "enum" if inner is not None and inner["kind"] == "enum" else "int"
            items = []
            for _ in range(reader.varint()):
                number = reader.zigzag()
                if item_kind != "int" and number < 0:
                    raise ValueError("a packed %s run holds the negative value %d" % (item_kind, number))
                items.append((item_kind, number))
            return ("tuple" if kind in TUPLE_SHAPES else "list", items)
        raise ValueError("unrecognized value tag 0x%02x at offset %d" % (tag, reader.at))

    def table(self, reader: Reader, spec_index: Optional[int], depth: int) -> List[Tuple[str, Any]]:
        rows_count = reader.varint()
        columns_count = reader.varint()
        if rows_count > reader.remaining() * 8:
            raise ValueError("a table of %d row(s) overruns the %d byte(s) left at offset %d" % (rows_count, reader.remaining(), reader.at))
        rows: List[Dict[int, Tuple[str, Any]]] = [{} for _ in range(rows_count)]
        known = self.spec.fields[spec_index] if spec_index is not None else {}
        for _ in range(columns_count):
            field_id = reader.varint()
            dense = reader.byte() == 0
            bitmap = None if dense else reader.take((rows_count + 7) // 8)
            tag = reader.byte()

            def present(position: int) -> bool:
                return bitmap is None or bool(bitmap[position // 8] & (1 << (position % 8)))

            if tag == ELEM_BOOL:
                bits = reader.take((rows_count + 7) // 8)
                for position in range(rows_count):
                    if present(position):
                        rows[position][field_id] = ("bool", bool(bits[position // 8] & (1 << (position % 8))))
                continue
            for position in range(rows_count):
                if not present(position):
                    continue
                if tag == ELEM_F64:
                    rows[position][field_id] = ("float", reader.float64())
                elif tag == ELEM_INT:
                    rows[position][field_id] = ("int", reader.zigzag())
                elif tag == ELEM_UINT:
                    rows[position][field_id] = ("uint", reader.varint())
                elif tag == ELEM_ENUM:
                    rows[position][field_id] = ("enum", reader.varint())
                elif tag == ELEM_STR:
                    rows[position][field_id] = ("text", self.symbol(reader.varint()))
                else:
                    value = self.value(reader, known[field_id]["shape"] if field_id in known else None, depth + 1)
                    if value is not None:
                        rows[position][field_id] = value
        return [("record", row) for row in rows]

    def dsl_value(self, reader: Reader, depth: int) -> Any:
        if depth > MAX_DEPTH:
            raise ValueError("the value nests deeper than %d" % MAX_DEPTH)
        tag = reader.byte()
        if tag == TAG_NULL:
            return None
        if tag in (TAG_FALSE, TAG_TRUE):
            return tag == TAG_TRUE
        if tag == TAG_UINT:
            return reader.varint()
        if tag == TAG_INT:
            return reader.zigzag()
        if tag == TAG_F64:
            return Float(reader.float64())
        if tag == TAG_STR:
            return self.symbol(reader.varint())
        if tag == TAG_STR_INLINE:
            return reader.take(reader.varint()).decode("utf-8")
        if tag == TAG_BYTES:
            return reader.take(reader.varint())
        if tag == TAG_LIST:
            return [self.dsl_value(reader, depth + 1) for _ in range(reader.varint())]
        if tag == TAG_MAP:
            entries = {}
            for _ in range(reader.varint()):
                key = self.text(reader)
                entries[key] = self.dsl_value(reader, depth + 1)
            return entries
        raise ValueError("unexpected dynamic value tag 0x%02x at offset %d" % (tag, reader.at))


def decode_record(spec: RecordSpec, inner: bytes) -> Dict[int, Tuple[str, Any]]:
    """🔓️ Decodes a container (no envelope) to its record tree, refusing a schema hash other than the spec's and any trailing document byte."""
    pack = open_pack(inner)
    if pack["manifest"]["schemaHash"] != spec.schema_hash():
        raise ValueError("the pack was written for another record schema")
    reader = Reader(pack["body"])
    record = Decoder(spec, pack["symbols"], pack["chunks"]).record_fields(reader, 0, 0)
    if reader.remaining():
        raise ValueError("%d trailing byte(s) after the document record" % reader.remaining())
    return record


# endregion 🔖️Decode


# region 🔖️Document
def camel(name: str) -> str:
    """🐫️ `snake_case` and `kebab-case` spec keys and keywords as the camelCase JSON names the snapshot documents use."""
    head, *rest = name.replace("-", "_").split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def normal(name: str) -> str:
    """🧹️ A name with its case and separators dropped, the one comparison between a document tag and a spec tag."""
    return name.replace("-", "").replace("_", "").lower()


def shortest_f32(value: float) -> float:
    """🪶️ The double whose shortest decimal still round-trips the single-precision `value`, the number a document spells for an `f32` field (`0.1`, not `0.10000000149011612`)."""
    for digits in range(1, 10):
        candidate = float("%.*g" % (digits, value))
        if struct.unpack("<f", struct.pack("<f", candidate))[0] == value:
            return candidate
    return value


class Document:
    """📄️ The two-way mapping between a camelCase JSON snapshot and the record tree, driven by the record spec alone.

    * a record field is the JSON key `camel(field key)`; an absent optional field is JSON `null`, or no key at all when `omit_nulls` is set (and `null` or a missing key
      is absent either way);
    * the payload keys of a statements variant are camelCase like every other record, except on the fields the spec's `verbatim` annotation lists
      (serde's `rename_all_fields` is absent there, so the Rust field name, `style_id`, is the JSON key);
    * a record the spec's `nested` annotation lists keeps some of its fields in a JSON sub-object (`ArtifactRef` flattens its `dialect` on the wire);
    * a unit-enum shape is its tag string; a statements shape is an object `{"kind": <tag>, ...variant fields}` (the tag key is `kind` unless the
      spec's `tags` annotation names another), a list of them when the field holds many, and the single object when the spec's `single`
      annotation says the field holds exactly one;
    * every float shape is a JSON number; a field the spec's `float32` annotation lists is single precision, so the encoder narrows its number to `f32` first and the decoder
      returns the widened double, or with `narrow_f32` the shortest decimal that round-trips the `f32` (`0.1`, as serde writes it).
    """

    def __init__(self, spec: RecordSpec, omit_nulls: bool = False, narrow_f32: bool = False) -> None:
        self.spec = spec
        self.omit_nulls = omit_nulls
        self.narrow_f32 = narrow_f32
        self.single = spec.single
        self.float32 = spec.float32
        self.verbatim = spec.verbatim

    def record(self, spec_index: int, document: Dict[str, Any], path: str = "$", verbatim: bool = False) -> Dict[int, Tuple[str, Any]]:
        if not isinstance(document, dict):
            raise ValueError("%s: expected an object for record %d, found %s" % (path, spec_index, type(document).__name__))
        document = self.unnest(spec_index, document)
        known = {(field["key"].replace("-", "_") if verbatim else camel(field["key"])): field for field in self.spec.graph[spec_index]}
        extra = set(document) - set(known)
        if extra:
            raise ValueError("%s: the document carries %s, which record %d does not declare" % (path, sorted(extra), spec_index))
        fields: Dict[int, Tuple[str, Any]] = {}
        for name, field in known.items():
            raw = document.get(name)
            if raw is None:
                if not field["optional"] and not self.omittable(field["shape"]):
                    raise ValueError("%s.%s: the required field is missing" % (path, name))
                if (spec_index, field["id"]) in self.single:
                    fields[field["id"]] = self.empty_statements(field["shape"])
                continue
            fields[field["id"]] = self.value(field["shape"], raw, (spec_index, field["id"]), "%s.%s" % (path, name))
        return fields

    @staticmethod
    def empty_statements(shape: Dict[str, Any]) -> Tuple[str, Any]:
        """🕳️ An absent `Option<enum>` is a present field holding no statement, wrapped in its block where the shape has one."""
        return ("block", ("statements", [])) if shape["kind"] == "block" else ("statements", [])

    def unnest(self, spec_index: int, document: Dict[str, Any]) -> Dict[str, Any]:
        if spec_index not in self.spec.nested or not isinstance(document, dict):
            return document
        name, _ = self.spec.nested[spec_index]
        inner = document.get(name)
        if not isinstance(inner, dict):
            return document
        return {**{key: item for key, item in document.items() if key != name}, **inner}

    @staticmethod
    def omittable(shape: Dict[str, Any]) -> bool:
        return shape["kind"] in ("list", "table", "map", "statements", "block")

    def value(self, shape: Dict[str, Any], raw: Any, at: Tuple[int, int], path: str) -> Tuple[str, Any]:
        kind = shape["kind"]
        if kind == "bool":
            return ("bool", bool(raw))
        if kind == "int":
            return ("int", int(raw))
        if kind in UINT_SHAPES:
            return ("uint", int(raw))
        if kind in FLOAT_SHAPES:
            number = float(raw)
            return ("float", struct.unpack("<f", struct.pack("<f", number))[0] if at in self.float32 else number)
        if kind in TEXT_SHAPES:
            return ("text", str(raw))
        if kind == "bytes64":
            return ("bytes", bytes(raw))
        if kind == "enum":
            for ordinal, tag in shape["variants"]:
                if normal(tag) == normal(str(raw)):
                    return ("enum", ordinal)
            raise ValueError("%s: %r is not one of %s" % (path, raw, [tag for _, tag in shape["variants"]]))
        if kind in TUPLE_SHAPES:
            inner = shape["item"] if kind == "tuple" else {"kind": "float"}
            return ("tuple", [self.value(inner, item, at, "%s[%d]" % (path, index)) for index, item in enumerate(raw)])
        if kind == "list":
            return ("list", [self.value(shape["item"], item, at, "%s[%d]" % (path, index)) for index, item in enumerate(raw)])
        if kind == "table":
            return ("list", [("record", self.record(shape["record"], item, "%s[%d]" % (path, index))) for index, item in enumerate(raw)])
        if kind == "record":
            return ("record", self.record(shape["record"], raw, path))
        if kind == "block":
            return ("block", self.value(shape["item"], raw, at, path))
        if kind == "statements":
            items = raw if isinstance(raw, list) else [raw]
            return ("statements", [self.statement(shape, item, at, "%s[%d]" % (path, index)) for index, item in enumerate(items)])
        if kind == "map":
            return ("map", [(key, self.value(shape["item"], item, at, "%s.%s" % (path, key))) for key, item in raw.items()])
        if kind == "value":
            return ("value", raw)
        raise ValueError("%s: the %s shape has no document form" % (path, kind))

    def statement(self, shape: Dict[str, Any], raw: Dict[str, Any], at: Tuple[int, int], path: str) -> Tuple[str, Dict[int, Tuple[str, Any]]]:
        key = self.spec.tags.get(at, "kind")
        tag = raw.get(key)
        for keyword, index in shape["variants"]:
            if normal(keyword) == normal(str(tag)):
                body = {name: item for name, item in raw.items() if name != key}
                return (keyword, self.record(index, body, path, at in self.verbatim))
        raise ValueError("%s: %r is not one of %s" % (path, tag, [keyword for keyword, _ in shape["variants"]]))

    def document(self, spec_index: int, fields: Dict[int, Tuple[str, Any]], verbatim: bool = False) -> Dict[str, Any]:
        out: Dict[str, Any] = {}
        for field in self.spec.graph[spec_index]:
            value = fields.get(field["id"])
            name = field["key"].replace("-", "_") if verbatim else camel(field["key"])
            if value is None:
                if not self.omit_nulls:
                    out[name] = [] if self.omittable(field["shape"]) and not field["optional"] else None
            else:
                out[name] = self.json(field["shape"], value, (spec_index, field["id"]))
                if out[name] is None and self.omit_nulls:
                    del out[name]
        if spec_index in self.spec.nested:
            name, members = self.spec.nested[spec_index]
            out[name] = {camel(member): out.pop(camel(member)) for member in members if camel(member) in out}
        return out

    def json(self, shape: Dict[str, Any], value: Tuple[str, Any], at: Tuple[int, int]) -> Any:
        kind, payload = value
        shape_kind = shape["kind"]
        if kind in ("bool", "int", "uint", "text"):
            return payload
        if kind == "float":
            return shortest_f32(payload) if self.narrow_f32 and at in self.float32 else payload
        if kind == "bytes":
            return list(payload)
        if kind == "enum":
            return next(camel(tag) for ordinal, tag in shape["variants"] if ordinal == payload)
        if kind in ("tuple", "list"):
            if shape_kind == "table":
                return [self.document(shape["record"], item[1]) for item in payload]
            inner = shape["item"] if shape_kind in ("tuple", "list") else {"kind": "float"}
            return [self.json(inner, item, at) for item in payload]
        if kind == "record":
            return self.document(shape["record"], payload)
        if kind == "block":
            return self.json(shape["item"], payload, at)
        if kind == "statements":
            items = [self.statement_json(shape, item, at) for item in payload]
            if at in self.single:
                return items[0] if items else None
            return items
        if kind == "map":
            return {key: self.json(shape["item"], item, at) for key, item in payload}
        if kind == "value":
            return payload
        raise ValueError("the %s value has no document form" % kind)

    def statement_json(self, shape: Dict[str, Any], item: Tuple[str, Dict[int, Tuple[str, Any]]], at: Tuple[int, int]) -> Dict[str, Any]:
        keyword, fields = item
        index = dict(shape["variants"])[keyword]
        return {self.spec.tags.get(at, "kind"): camel(keyword), **self.document(index, fields, at in self.spec.verbatim)}


def encode_pack(spec: RecordSpec, document: Dict[str, Any]) -> bytes:
    """🚚️ A JSON snapshot to the `.pack.semio` bytes: the document mapped to its record tree, the record encoded, the semio envelope around it."""
    return wrap_binary(spec.token, encode_record(spec, Document(spec).record(0, document)))


def decode_pack(spec: RecordSpec, data: bytes, omit_nulls: bool = False, narrow_f32: bool = False) -> Dict[str, Any]:
    """📨️ The reading direction of `encode_pack`, refusing any envelope token other than the spec's; `omit_nulls` leaves an absent optional field out of the document instead of writing `null`, `narrow_f32` spells single-precision fields as their shortest decimal."""
    token, inner = unwrap_binary(data)
    if token != spec.token:
        raise ValueError("expected the %r envelope token, got %r" % (spec.token, token))
    return Document(spec, omit_nulls, narrow_f32).document(0, decode_record(spec, inner))


# endregion 🔖️Document
