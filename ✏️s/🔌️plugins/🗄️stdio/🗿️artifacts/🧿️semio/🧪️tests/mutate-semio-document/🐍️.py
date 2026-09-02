"""🐍️ Independent Python implementation of the `s.stdio.semio.document` carriers and their
eighteen-verb mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is
a second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the `semio <envelope-id>.dsl v<version>` preamble for text and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line styles-line images-line blocks-line`, the eight
  single-letter-tagged `block` variants `P`/`H`/`L`/`T`/`C`/`Q`/`I`/`B` with the field lists it
  gives, `run`, `run-style`, `option-hex` and `option-f64`. That grammar also states the one thing a
  reader would otherwise get wrong: `enc_f64` prints `f64::to_bits()` as PLAIN DECIMAL DIGITS, not a
  float literal, so every `f64` leaf is a bare `INT` token;
* the pack head is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio`
  (`format u8`, then varint-length-prefixed UTF-8 `schema`), whose description declares
  `styles`/`images`/`blocks` one opaque `payload` chain by its own admission — the repeated record
  layout below was therefore DERIVED from the committed `🎒️.pack.semio` bytes, with field
  order taken from the DSL grammar and the per-variant tag byte from the grammar's own `block`
  variant order, and `pack_bytes` re-encodes that committed file byte for byte, which is what proves
  the derivation right;
* the eighteen verbs and their named arguments are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, their JSON wire form is this case's
  committed per-kind specification vectors under `🧫️fixtures/`, and the three `DocBlockPath` segment
  tags are declared by the committed schema mirror `…/🧬️mutations/🟦️.ts`
  (`quote` / `listItem` / `tableCell`) with the snake-case member spelling the committed vectors use.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import struct

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every variant of this subset's mutation vocabulary, in the order the mutations grammar's `op`
#: production lists them, kebab-cased as the catalog spells them.
KINDS = (
    "no-mutation",
    "set-snapshot",
    "insert-block",
    "remove-block",
    "set-block-content",
    "set-paragraph-style",
    "set-heading-level",
    "set-list-ordered",
    "set-run-text",
    "set-run-style",
    "set-image-block",
    "insert-style",
    "remove-style",
    "set-style-name",
    "set-style-based-on",
    "insert-image",
    "remove-image",
    "set-image-bytes",
)


def camel(kind: str) -> str:
    """🐫 The wire tag of one verb: its kebab-case name in camel case."""
    head, *rest = kind.split("-")
    return head + "".join(word.capitalize() for word in rest)


TAG_OF_KIND = {kind: camel(kind) for kind in KINDS}
KIND_OF_TAG = {tag: kind for kind, tag in TAG_OF_KIND.items()}

#: 📄️ The eight `block` variants in the grammar's own production order, which is also the ordinal
#: the pack frame writes — the committed memo confirms it: its heading carries `0x01` and its
#: trailing page break `0x07`.
BLOCK_TAGS = ("P", "H", "L", "T", "C", "Q", "I", "B")
BLOCK_KINDS = ("paragraph", "heading", "list", "table", "code", "quote", "image", "pageBreak")
KIND_OF_BLOCK_TAG = dict(zip(BLOCK_TAGS, BLOCK_KINDS))
TAG_OF_BLOCK_KIND = dict(zip(BLOCK_KINDS, BLOCK_TAGS))

DOCUMENT_SCHEMA = "s.stdio.semio.document"
DSL_PREAMBLE = "semio s.stdio.semio.document.dsl v1"
PACK_TOKEN = "s.stdio.semio.document.pack v1"
PACK_FORMAT = 1
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"

MEMO_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️.dsl.semio"
MEMO_PACK = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🎒️.pack.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    return raw_hex(hexed).decode("utf-8")


def raw_hex(hexed: str) -> bytes:
    """🔡️ The same macro over an opaque byte run — an image payload is not text."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    if any(char not in "0123456789abcdef" for char in hexed):
        raise AssertionError("hex run %r carries a digit outside the macro's alphabet" % hexed)
    return bytes.fromhex(hexed)


def real_of_bits(text: str) -> float:
    """🔢️ `enc_f64` prints `f64::to_bits()` as plain decimal digits — the grammar says so."""
    if not text.isdigit():
        raise AssertionError("an f64 leaf is the decimal of its bit pattern, got %r" % text)
    return struct.unpack("<d", struct.pack("<Q", int(text)))[0]


def bits_of_real(value: float) -> str:
    return "%d" % struct.unpack("<Q", struct.pack("<d", float(value)))[0]


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


def split_top_level(text: str) -> list:
    """🔎️ Splits a bracket-nested list body on its top-level commas; `""` means the empty list."""
    if text == "":
        return []
    out, depth, start = [], 0, 0
    for at, char in enumerate(text):
        if char == "[":
            depth += 1
        elif char == "]":
            depth -= 1
            if depth < 0:
                raise AssertionError("unbalanced ']' at offset %d of %r" % (at, text))
        elif char == "," and depth == 0:
            out.append(text[start:at])
            start = at + 1
    if depth != 0:
        raise AssertionError("unbalanced '[' in %r" % text)
    out.append(text[start:])
    return out


def bracketed(text: str) -> str:
    """🔎️ The body of a `"[" … "]"` group, refusing anything that is not one."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("expected a bracketed group, got %r" % text)
    return text[1:-1]


def field(lines: list, name: str) -> str:
    """🔎️ The right-hand side of one `name "=" value` body line; a missing line is an error."""
    prefix = name + "="
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix) :]
    raise AssertionError("the document has no %r line — the grammar declares it mandatory" % name)


def read_option(text: str, decode):
    """❓️ `option-hex` / `option-f64` — `[0]` for the absent value, `[1,<leaf>]` for the present one."""
    parts = split_top_level(bracketed(text))
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return decode(parts[1])
    raise AssertionError("expected an option group, got %r" % text)


def write_option(value, encode) -> str:
    return "[0]" if value is None else "[1,%s]" % encode(value)


def read_bool(text: str) -> bool:
    """🔘️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("expected a bool spelled 0 or 1, got %r" % text)
    return text == "1"


# endregion 🔖️Carrier


# region 🔖️Dsl
def read_run_style(text: str) -> dict:
    """🖊️ `run-style = "[" bool "," bool "," bool "," option-f64 "," option-hex ×3 "]"`."""
    bold, italic, underline, size, font, color, link = split_top_level(bracketed(text))
    return {
        "bold": read_bool(bold),
        "italic": read_bool(italic),
        "underline": read_bool(underline),
        "size": read_option(size, real_of_bits),
        "font": read_option(font, text_of),
        "color": read_option(color, text_of),
        "link": read_option(link, text_of),
    }


def write_run_style(style: dict) -> str:
    return "[%s,%s,%s,%s,%s,%s,%s]" % (
        "1" if style["bold"] else "0",
        "1" if style["italic"] else "0",
        "1" if style["underline"] else "0",
        write_option(style["size"], bits_of_real),
        write_option(style["font"], hex_of),
        write_option(style["color"], hex_of),
        write_option(style["link"], hex_of),
    )


def read_run(text: str) -> dict:
    content, style = split_top_level(bracketed(text))
    return {"text": text_of(content), "style": read_run_style(style)}


def write_run(run: dict) -> str:
    return "[%s,%s]" % (hex_of(run["text"]), write_run_style(run["style"]))


def read_blocks(text: str) -> list:
    return [read_block(each) for each in split_top_level(bracketed(text))]


def write_blocks(blocks: list) -> str:
    return "[%s]" % ",".join(write_block(block) for block in blocks)


def read_block(text: str) -> dict:
    """📄️ One tagged `block` value: a single-letter tag then its bracketed field list."""
    tag, rest = text[:1], text[1:]
    if tag not in KIND_OF_BLOCK_TAG:
        raise AssertionError("unknown block tag %r — the grammar declares %s" % (tag, ", ".join(BLOCK_TAGS)))
    kind = KIND_OF_BLOCK_TAG[tag]
    parts = split_top_level(bracketed(rest))
    if kind == "paragraph":
        style_id, runs = parts
        return {"kind": kind, "style_id": read_option(style_id, text_of), "runs": [read_run(each) for each in split_top_level(bracketed(runs))]}
    if kind == "heading":
        level, style_id, runs = parts
        return {"kind": kind, "level": int(level), "style_id": read_option(style_id, text_of), "runs": [read_run(each) for each in split_top_level(bracketed(runs))]}
    if kind == "list":
        ordered, items = parts
        return {"kind": kind, "ordered": read_bool(ordered), "items": [{"blocks": read_blocks(bracketed(each))} for each in split_top_level(bracketed(items))]}
    if kind == "table":
        (rows,) = parts
        return {"kind": kind, "rows": [{"cells": [{"blocks": read_blocks(bracketed(cell))} for cell in split_top_level(bracketed(bracketed(row)))]} for row in split_top_level(bracketed(rows))]}
    if kind == "code":
        language, body = parts
        return {"kind": kind, "language": read_option(language, text_of), "text": text_of(body)}
    if kind == "quote":
        (blocks,) = parts
        return {"kind": kind, "blocks": read_blocks(blocks)}
    if kind == "image":
        image_id, alt, width, height = parts
        return {"kind": kind, "image_id": text_of(image_id), "alt": text_of(alt), "width": read_option(width, real_of_bits), "height": read_option(height, real_of_bits)}
    if parts != []:
        raise AssertionError("a page break carries no fields, got %r" % text)
    return {"kind": kind}


def write_block(block: dict) -> str:
    kind = block["kind"]
    if kind not in TAG_OF_BLOCK_KIND:
        raise AssertionError("unknown block kind %r" % kind)
    tag = TAG_OF_BLOCK_KIND[kind]
    if kind == "paragraph":
        return "P[%s,[%s]]" % (write_option(block["style_id"], hex_of), ",".join(write_run(run) for run in block["runs"]))
    if kind == "heading":
        return "H[%d,%s,[%s]]" % (int(block["level"]), write_option(block["style_id"], hex_of), ",".join(write_run(run) for run in block["runs"]))
    if kind == "list":
        return "L[%s,[%s]]" % ("1" if block["ordered"] else "0", ",".join("[%s]" % write_blocks(item["blocks"]) for item in block["items"]))
    if kind == "table":
        return "T[[%s]]" % ",".join("[[%s]]" % ",".join("[%s]" % write_blocks(cell["blocks"]) for cell in row["cells"]) for row in block["rows"])
    if kind == "code":
        return "C[%s,%s]" % (write_option(block["language"], hex_of), hex_of(block["text"]))
    if kind == "quote":
        return "Q[%s]" % write_blocks(block["blocks"])
    if kind == "image":
        return "I[%s,%s,%s,%s]" % (hex_of(block["image_id"]), hex_of(block["alt"]), write_option(block["width"], bits_of_real), write_option(block["height"], bits_of_real))
    return "%s[]" % tag


def read_style(text: str) -> dict:
    """🎨️ `style = "[" hex "," hex "," option-hex "]"`."""
    ident, name, based_on = split_top_level(bracketed(text))
    return {"id": text_of(ident), "name": text_of(name), "basedOn": read_option(based_on, text_of)}


def write_style(style: dict) -> str:
    return "[%s,%s,%s]" % (hex_of(style["id"]), hex_of(style["name"]), write_option(style["basedOn"], hex_of))


def read_image(text: str) -> dict:
    """🖼️ `image = "[" hex "," hex "," hex "]"` — id, mime, opaque payload."""
    ident, mime, payload = split_top_level(bracketed(text))
    return {"id": text_of(ident), "mime": text_of(mime), "bytes": list(raw_hex(payload))}


def write_image(image: dict) -> str:
    return "[%s,%s,%s]" % (hex_of(image["id"]), hex_of(image["mime"]), bytes(image["bytes"]).hex())


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line styles-line images-line blocks-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    return {
        "schema": text_of(field(lines, "schema")),
        "styles": [read_style(each) for each in split_top_level(bracketed(field(lines, "styles")))],
        "images": [read_image(each) for each in split_top_level(bracketed(field(lines, "images")))],
        "blocks": [read_block(each) for each in split_top_level(bracketed(field(lines, "blocks")))],
    }


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    body = [
        "schema=%s" % hex_of(snapshot["schema"]),
        "styles=[%s]" % ",".join(write_style(style) for style in snapshot["styles"]),
        "images=[%s]" % ",".join(write_image(image) for image in snapshot["images"]),
        "blocks=[%s]" % ",".join(write_block(block) for block in snapshot["blocks"]),
    ]
    return "\n".join([DSL_PREAMBLE] + body)


# endregion 🔖️Dsl


# region 🔖️Pack
class Cursor:
    """🔎️ A byte cursor over a pack body — the record framing is positional, so one cursor with a
    varint reader and a fixed-width reader is the whole machine the frame needs."""

    def __init__(self, payload: bytes) -> None:
        self.payload = payload
        self.at = 0

    def take(self, count: int) -> bytes:
        if self.at + count > len(self.payload):
            raise AssertionError("the pack frame ends %d byte(s) early" % (self.at + count - len(self.payload)))
        chunk = self.payload[self.at : self.at + count]
        self.at += count
        return chunk

    def byte(self) -> int:
        return self.take(1)[0]

    def varint(self) -> int:
        """🔢️ LEB128, seven bits per byte, little end first — the protocol's own length prefix."""
        value, shift = 0, 0
        while True:
            octet = self.byte()
            value |= (octet & 0x7F) << shift
            if octet & 0x80 == 0:
                return value
            shift += 7

    def real(self) -> float:
        return struct.unpack("<d", self.take(8))[0]

    def string(self) -> str:
        return self.take(self.varint()).decode("utf-8")

    def flag(self) -> bool:
        return self.byte() == 1

    def option(self, decode):
        return decode(self) if self.flag() else None

    def done(self) -> None:
        if self.at != len(self.payload):
            raise AssertionError("%d trailing byte(s) after the pack body" % (len(self.payload) - self.at))


def put_varint(value: int) -> bytes:
    if value < 0:
        raise AssertionError("the pack frame writes a varint, which carries no sign; got %d" % value)
    out = bytearray()
    while True:
        octet = value & 0x7F
        value >>= 7
        out.append(octet | (0x80 if value else 0))
        if not value:
            return bytes(out)


def put_string(text: str) -> bytes:
    raw = text.encode("utf-8")
    return put_varint(len(raw)) + raw


def put_real(value: float) -> bytes:
    return struct.pack("<d", float(value))


def put_flag(value: bool) -> bytes:
    return bytes([1 if value else 0])


def put_option(value, encode) -> bytes:
    return b"\x00" if value is None else b"\x01" + encode(value)


def take_run(cursor: Cursor) -> dict:
    text = cursor.string()
    return {
        "text": text,
        "style": {
            "bold": cursor.flag(),
            "italic": cursor.flag(),
            "underline": cursor.flag(),
            "size": cursor.option(Cursor.real),
            "font": cursor.option(Cursor.string),
            "color": cursor.option(Cursor.string),
            "link": cursor.option(Cursor.string),
        },
    }


def put_run(run: dict) -> bytes:
    style = run["style"]
    return (
        put_string(run["text"])
        + put_flag(style["bold"])
        + put_flag(style["italic"])
        + put_flag(style["underline"])
        + put_option(style["size"], put_real)
        + put_option(style["font"], put_string)
        + put_option(style["color"], put_string)
        + put_option(style["link"], put_string)
    )


def take_block(cursor: Cursor) -> dict:
    tag = cursor.byte()
    if tag >= len(BLOCK_KINDS):
        raise AssertionError("unknown packed block tag %d — the grammar declares eight variants" % tag)
    kind = BLOCK_KINDS[tag]
    if kind == "paragraph":
        style_id = cursor.option(Cursor.string)
        return {"kind": kind, "style_id": style_id, "runs": [take_run(cursor) for _ in range(cursor.varint())]}
    if kind == "heading":
        level = cursor.byte()
        style_id = cursor.option(Cursor.string)
        return {"kind": kind, "level": level, "style_id": style_id, "runs": [take_run(cursor) for _ in range(cursor.varint())]}
    if kind == "list":
        ordered = cursor.flag()
        return {"kind": kind, "ordered": ordered, "items": [{"blocks": [take_block(cursor) for _ in range(cursor.varint())]} for _ in range(cursor.varint())]}
    if kind == "table":
        return {"kind": kind, "rows": [{"cells": [{"blocks": [take_block(cursor) for _ in range(cursor.varint())]} for _ in range(cursor.varint())]} for _ in range(cursor.varint())]}
    if kind == "code":
        language = cursor.option(Cursor.string)
        return {"kind": kind, "language": language, "text": cursor.string()}
    if kind == "quote":
        return {"kind": kind, "blocks": [take_block(cursor) for _ in range(cursor.varint())]}
    if kind == "image":
        image_id, alt = cursor.string(), cursor.string()
        return {"kind": kind, "image_id": image_id, "alt": alt, "width": cursor.option(Cursor.real), "height": cursor.option(Cursor.real)}
    return {"kind": kind}


def put_block(block: dict) -> bytes:
    kind = block["kind"]
    if kind not in BLOCK_KINDS:
        raise AssertionError("unknown block kind %r" % kind)
    out = bytes([BLOCK_KINDS.index(kind)])
    if kind == "paragraph":
        return out + put_option(block["style_id"], put_string) + put_varint(len(block["runs"])) + b"".join(put_run(run) for run in block["runs"])
    if kind == "heading":
        return out + bytes([int(block["level"])]) + put_option(block["style_id"], put_string) + put_varint(len(block["runs"])) + b"".join(put_run(run) for run in block["runs"])
    if kind == "list":
        return out + put_flag(block["ordered"]) + put_varint(len(block["items"])) + b"".join(put_varint(len(item["blocks"])) + b"".join(put_block(one) for one in item["blocks"]) for item in block["items"])
    if kind == "table":
        rows = b""
        for row in block["rows"]:
            rows += put_varint(len(row["cells"]))
            for cell in row["cells"]:
                rows += put_varint(len(cell["blocks"])) + b"".join(put_block(one) for one in cell["blocks"])
        return out + put_varint(len(block["rows"])) + rows
    if kind == "code":
        return out + put_option(block["language"], put_string) + put_string(block["text"])
    if kind == "quote":
        return out + put_varint(len(block["blocks"])) + b"".join(put_block(one) for one in block["blocks"])
    if kind == "image":
        return out + put_string(block["image_id"]) + put_string(block["alt"]) + put_option(block["width"], put_real) + put_option(block["height"], put_real)
    return out


def parse_pack(payload: bytes) -> dict:
    """📦️ The binary twin of the DSL. The committed protocol fully describes the envelope and the
    `format u8` + varint-length-prefixed `schema` head, and then declares the three collections one
    opaque trailing `payload` chain by its own admission. That layer was therefore DERIVED from the
    committed `🎒️.pack.semio` bytes: varint counts, length-prefixed UTF-8 strings,
    little-endian `f64` leaves, a `u8` bool, a `u8` option discriminant and a per-variant `u8` block
    tag in the grammar's own variant order, with field order taken from the DSL grammar. The
    derivation is PINNED — `pack_bytes` re-encodes that committed file byte for byte."""
    if not payload.startswith(BINARY_MAGIC):
        raise AssertionError("the pack does not start with the semio binary magic")
    at = len(BINARY_MAGIC)
    (token_len,) = struct.unpack("<I", payload[at : at + 4])
    at += 4
    token = payload[at : at + token_len].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("expected the %r envelope token, got %r" % (PACK_TOKEN, token))
    cursor = Cursor(payload[at + token_len :])
    if cursor.byte() != PACK_FORMAT:
        raise AssertionError("unexpected pack format byte")
    snapshot = {"schema": cursor.string(), "styles": [], "images": [], "blocks": []}
    for _ in range(cursor.varint()):
        snapshot["styles"].append({"id": cursor.string(), "name": cursor.string(), "basedOn": cursor.option(Cursor.string)})
    for _ in range(cursor.varint()):
        ident, mime = cursor.string(), cursor.string()
        snapshot["images"].append({"id": ident, "mime": mime, "bytes": list(cursor.take(cursor.varint()))})
    snapshot["blocks"] = [take_block(cursor) for _ in range(cursor.varint())]
    cursor.done()
    return snapshot


def pack_bytes(snapshot: dict) -> bytes:
    """📦️ The same frame in the writing direction; `identity-round-trip` requires it to reproduce
    the committed binary twin byte for byte."""
    body = bytearray([PACK_FORMAT])
    body += put_string(snapshot["schema"])
    body += put_varint(len(snapshot["styles"]))
    for style in snapshot["styles"]:
        body += put_string(style["id"]) + put_string(style["name"]) + put_option(style["basedOn"], put_string)
    body += put_varint(len(snapshot["images"]))
    for image in snapshot["images"]:
        payload = bytes(image["bytes"])
        body += put_string(image["id"]) + put_string(image["mime"]) + put_varint(len(payload)) + payload
    body += put_varint(len(snapshot["blocks"]))
    for block in snapshot["blocks"]:
        body += put_block(block)
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + struct.pack("<I", len(token)) + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
def clone(value):
    """🧬️ A structural copy, so applying a verb never writes through into the parsed document."""
    return json.loads(json.dumps(value))


def parts(mutation: dict) -> tuple:
    """🔎️ Splits `{"mutation": "<camelTag>", …}` into its verb and its named arguments."""
    tag = mutation.get("mutation")
    if tag not in KIND_OF_TAG:
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(KIND_OF_TAG))))
    return KIND_OF_TAG[tag], mutation


def container(snapshot: dict, path: dict, verb: str) -> list:
    """🧭️ The block list one `DocBlockPath` addresses: each segment descends into a quote, a list
    item or a table cell, and `index` then addresses inside the list this returns."""
    blocks = snapshot["blocks"]
    for segment in path.get("segments") or []:
        at = segment["block_index"]
        if not isinstance(at, int) or at < 0 or at >= len(blocks):
            raise AssertionError("%s descends into block %r of a list holding %d" % (verb, at, len(blocks)))
        block = blocks[at]
        kind = segment["kind"]
        if kind == "quote":
            if block["kind"] != "quote":
                raise AssertionError("%s descends into a quote, but block %d is a %s" % (verb, at, block["kind"]))
            blocks = block["blocks"]
        elif kind == "listItem":
            if block["kind"] != "list":
                raise AssertionError("%s descends into a list item, but block %d is a %s" % (verb, at, block["kind"]))
            blocks = block["items"][segment["item"]]["blocks"]
        elif kind == "tableCell":
            if block["kind"] != "table":
                raise AssertionError("%s descends into a table cell, but block %d is a %s" % (verb, at, block["kind"]))
            blocks = block["rows"][segment["row"]]["cells"][segment["cell"]]["blocks"]
        else:
            raise AssertionError("unknown path segment %r — the schema declares quote, listItem and tableCell" % kind)
    return blocks


def addressed(snapshot: dict, path: dict, verb: str, inclusive: bool = False) -> tuple:
    blocks = container(snapshot, path, verb)
    index = path["index"]
    limit = len(blocks) if inclusive else len(blocks) - 1
    if not isinstance(index, int) or index < 0 or index > limit:
        raise AssertionError("%s addresses block %r of a list holding %d" % (verb, index, len(blocks)))
    return blocks, index


def by_id(items: list, ident: str, verb: str) -> int:
    for at, item in enumerate(items):
        if item["id"] == ident:
            return at
    raise AssertionError("%s addresses id %r, which the document does not carry" % (verb, ident))


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW snapshot. `insert-style`/`insert-image` append to their
    id-keyed collections, which is what the committed specification vectors record."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        return clone(args["snapshot"])
    if kind == "insert-block":
        blocks, index = addressed(result, args["path"], kind, True)
        blocks.insert(index, clone(args["block"]))
        return result
    if kind in ("remove-block", "set-block-content", "set-paragraph-style", "set-heading-level", "set-list-ordered", "set-run-text", "set-run-style", "set-image-block"):
        blocks, index = addressed(result, args["path"], kind)
        if kind == "remove-block":
            del blocks[index]
            return result
        block = blocks[index]
        if kind == "set-block-content":
            blocks[index] = clone(args["block"])
            return result
        if kind == "set-paragraph-style":
            block["style_id"] = args["style_id"]
            return result
        if kind == "set-heading-level":
            block["level"] = args["level"]
            return result
        if kind == "set-list-ordered":
            block["ordered"] = args["ordered"]
            return result
        if kind == "set-image-block":
            block.update({"image_id": args["image_id"], "alt": args["alt"], "width": args["width"], "height": args["height"]})
            return result
        runs = block["runs"]
        at = args["run_index"]
        if not isinstance(at, int) or at < 0 or at >= len(runs):
            raise AssertionError("%s addresses run %r of a block carrying %d" % (kind, at, len(runs)))
        if kind == "set-run-text":
            runs[at]["text"] = args["text"]
            return result
        runs[at]["style"] = clone(args["style"])
        return result
    if kind == "insert-style":
        result["styles"].append(clone(args["style"]))
        return result
    if kind == "remove-style":
        del result["styles"][by_id(result["styles"], args["id"], kind)]
        return result
    if kind == "set-style-name":
        result["styles"][by_id(result["styles"], args["id"], kind)]["name"] = args["name"]
        return result
    if kind == "set-style-based-on":
        result["styles"][by_id(result["styles"], args["id"], kind)]["basedOn"] = args["based_on"]
        return result
    if kind == "insert-image":
        result["images"].append(clone(args["image"]))
        return result
    if kind == "remove-image":
        del result["images"][by_id(result["images"], args["id"], kind)]
        return result
    image = result["images"][by_id(result["images"], args["id"], kind)]
    image.update({"mime": args["mime"], "bytes": clone(args["bytes"])})
    return result


def inverse_mutation(snapshot: dict, mutation: dict) -> dict:
    """↩️ The undo of one verb against the state it was applied to. An insertion is undone by a
    removal at the position it took and an overwrite by an overwrite with the value it displaced.
    Because `insert-style`/`insert-image` append, undoing a NON-FINAL removal from those two
    id-keyed collections restores the value at the end — the feature exercises them at the final
    entry and says so."""
    kind, args = parts(mutation)
    tag = TAG_OF_KIND
    if kind == "no-mutation":
        return {"mutation": tag[kind]}
    if kind == "set-snapshot":
        return {"mutation": tag[kind], "snapshot": clone(snapshot)}
    if kind == "insert-block":
        return {"mutation": tag["remove-block"], "path": clone(args["path"])}
    if kind in ("remove-block", "set-block-content", "set-paragraph-style", "set-heading-level", "set-list-ordered", "set-run-text", "set-run-style", "set-image-block"):
        blocks, index = addressed(snapshot, args["path"], kind)
        block = blocks[index]
        if kind == "remove-block":
            return {"mutation": tag["insert-block"], "path": clone(args["path"]), "block": clone(block)}
        if kind == "set-block-content":
            return {"mutation": tag[kind], "path": clone(args["path"]), "block": clone(block)}
        if kind == "set-paragraph-style":
            return {"mutation": tag[kind], "path": clone(args["path"]), "style_id": block["style_id"]}
        if kind == "set-heading-level":
            return {"mutation": tag[kind], "path": clone(args["path"]), "level": block["level"]}
        if kind == "set-list-ordered":
            return {"mutation": tag[kind], "path": clone(args["path"]), "ordered": block["ordered"]}
        if kind == "set-image-block":
            return {"mutation": tag[kind], "path": clone(args["path"]), "image_id": block["image_id"], "alt": block["alt"], "width": block["width"], "height": block["height"]}
        run = block["runs"][args["run_index"]]
        if kind == "set-run-text":
            return {"mutation": tag[kind], "path": clone(args["path"]), "run_index": args["run_index"], "text": run["text"]}
        return {"mutation": tag[kind], "path": clone(args["path"]), "run_index": args["run_index"], "style": clone(run["style"])}
    if kind == "insert-style":
        return {"mutation": tag["remove-style"], "id": args["style"]["id"]}
    if kind == "remove-style":
        return {"mutation": tag["insert-style"], "style": clone(snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)])}
    if kind == "set-style-name":
        return {"mutation": tag[kind], "id": args["id"], "name": snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)]["name"]}
    if kind == "set-style-based-on":
        return {"mutation": tag[kind], "id": args["id"], "based_on": snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)]["basedOn"]}
    if kind == "insert-image":
        return {"mutation": tag["remove-image"], "id": args["image"]["id"]}
    if kind == "remove-image":
        return {"mutation": tag["insert-image"], "image": clone(snapshot["images"][by_id(snapshot["images"], args["id"], kind)])}
    was = snapshot["images"][by_id(snapshot["images"], args["id"], kind)]
    return {"mutation": tag[kind], "id": args["id"], "mime": was["mime"], "bytes": clone(was["bytes"])}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def memo(ctx: Context) -> dict:
    """📄️ The real committed memo, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(MEMO_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.fixture_bytes("local://🦠️%s.json" % kind).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed memo by this implementation alone."""
    result = apply_mutation(memo(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the memo again — asserted here, and the MUTATED snapshot travels in the projection too, so the
    eighteen rows cannot all project the same restored value and compare vacuously."""
    snapshot = memo(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the memo\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(kind: str):
    """🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations, kept from before this oracle existed."""

    def handler(ctx: Context) -> Outcome:
        committed = vector(ctx, kind)
        applied = apply_mutation(committed["before"], committed["mutation"])
        if applied != committed["after"]:
            raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(committed["after"])))
        return Outcome(applied)

    return handler


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real memo, each re-emitted from the parsed snapshot.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps that from being vacuous is that both committed files were written by the OTHER
    implementation: this file reproducing them is a cross-language byte agreement, not a codec
    agreeing with itself. The two encodings also cross-check each other — the binary twin has to
    decode to the same memo the text does, which no single codec can arrange on its own.
    """
    committed = ctx.fixture_bytes(MEMO_DSL)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing the memo did not reproduce the committed DSL bytes (%d vs %d bytes)\n     got: %s\nexpected: %s" % (len(printed), len(committed), printed.decode("utf-8"), committed.decode("utf-8")))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed memo lost content")
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("the committed memo declares schema %r, expected %r" % (snapshot["schema"], DOCUMENT_SCHEMA))
    committed_pack = ctx.fixture_bytes(MEMO_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != snapshot:
        raise AssertionError("the committed binary twin decodes to a different memo than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked), json.dumps(snapshot)))
    repacked = pack_bytes(snapshot)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the memo did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    if parse_pack(repacked) != snapshot:
        raise AssertionError("re-decoding the encoded pack lost content")
    declared = vector(ctx, "no-mutation")["before"]
    if snapshot != declared:
        raise AssertionError("the real committed memo does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(snapshot), json.dumps(declared)))
    return Outcome(
        {
            "document": snapshot,
            "dslDigest": digest(printed),
            "packDigest": digest(repacked),
            "dslLength": len(printed),
            "packLength": len(repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector(kind))
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
