"""🐍️ Independent Python implementation of the `s.stdio.semio.presentation` carrier and its
fifteen-verb mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, and `python-pptx` was surveyed and rejected for this vocabulary
(it cannot create masters or layouts at all, and reaching a `SemioPresentationSnapshot` from pptx
bytes through OUR importer would compare this repository with itself). The second producer THE
STANDARD requires is therefore a second IMPLEMENTATION, written in another language from the
format's own committed specification:

* the envelope — `semio <schema>.dsl v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line masters-line layouts-line slides-line`, the four
  single-letter `shape` tags, the seven `placeholder-kind` letters, and document's own `block`
  family verbatim, with every numeric leaf printed as the decimal of `f64::to_bits()`;
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio` and
  its Kaitai mirror, which state the `format` byte and the varint-length-prefixed `schema` and then
  declare the collections an opaque `payload` chain — naming, though, exactly what is inside it:
  *"a data-carrying tagged `SlideShape` union whose `TextBox`/`Table` variants further embed
  document's own recursive `DocBlock` union, real-tag-byte-encoded, with every `DocBlock` leaf
  reusing document's real `enc_block`/`dec_block` TEXT codec embedded as a length-prefixed UTF-8
  blob"*. The record layout below was DERIVED from that description together with the committed
  `✉️base/📚️examples/📽️deck/🖼️assets/🎒️.pack.semio` bytes, whose DSL twin pins every field
  against a readable spelling; the derivation is then pinned by re-encoding that committed file byte
  for byte, which a misreading could not do;
* the fifteen verbs, their argument lists and their JSON wire form are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, the committed proto/JSON schema
  mirrors and the committed `(before, mutation, after)` specification vectors in this case's own
  `🧫️fixtures/`, which pin each verb's semantics.

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


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
DSL_PREAMBLE = "semio s.stdio.semio.presentation.dsl v1"
PACK_TOKEN = "s.stdio.semio.presentation.pack v1"
PACK_FORMAT = 1

#: 🧩️ `shape = "X"… | "P"… | "T"… | "H"…` in the grammar's own order, which is also the tag ordinal
#: the pack frame writes — the committed deck's text box, picture, table and placeholder carry
#: `0x00`, `0x01`, `0x02` and `0x03` in exactly that order.
SHAPE_ORDER = ("textBox", "picture", "table", "placeholder")
SHAPE_LETTER = {"textBox": "X", "picture": "P", "table": "T", "placeholder": "H"}
LETTER_SHAPE = {letter: kind for kind, letter in SHAPE_LETTER.items()}

#: 🏷️ `placeholder-kind = "T" | "S" | "B" | "F" | "N" | "D" | "O" "[" hex "]"`, and the same order
#: as the pack ordinal — the committed deck's `T` master placeholder is `0x00`, its `S` layout
#: placeholder `0x01` and its `O[custom]` slide placeholder `0x06`.
PLACEHOLDER_ORDER = ("title", "subtitle", "body", "footer", "slideNumber", "dateTime", "other")
PLACEHOLDER_LETTER = {"title": "T", "subtitle": "S", "body": "B", "footer": "F", "slideNumber": "N", "dateTime": "D", "other": "O"}
LETTER_PLACEHOLDER = {letter: kind for kind, letter in PLACEHOLDER_LETTER.items()}

#: 📄️ document's own `DocBlock` union, letter for letter as the block grammar declares it.
BLOCK_LETTER = {"paragraph": "P", "heading": "H", "list": "L", "table": "T", "code": "C", "quote": "Q", "image": "I", "pageBreak": "B"}
LETTER_BLOCK = {letter: kind for kind, letter in BLOCK_LETTER.items()}

KINDS = (
    "no-mutation",
    "set-snapshot",
    "insert-slide",
    "remove-slide",
    "set-slide-layout",
    "set-slide-notes",
    "insert-shape",
    "remove-shape",
    "set-shape-frame",
    "set-text-box-blocks",
    "insert-master",
    "remove-master",
    "insert-layout",
    "remove-layout",
    "set-layout-master",
)

TALK_DSL = "local://🎙️talk/🗣️.dsl.semio"
TALK_PACK = "local://🎒️.pack.semio"
DECK_DSL = "asset://📚️examples/📽️deck/🖼️assets/🗣️.dsl.semio"
DECK_PACK = "asset://📚️examples/📽️deck/🖼️assets/🎒️.pack.semio"


def hex_of_text(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction, for a string leaf."""
    return text.encode("utf-8").hex()


def text_of_hex(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction, for a string leaf."""
    return bytes.fromhex(hexed).decode("utf-8")


def bits_of(value: float) -> str:
    """🔢️ `enc_f64` — the decimal of `f64::to_bits()`, never a float literal."""
    return str(struct.unpack("<Q", struct.pack("<d", float(value)))[0])


def float_of(bits: str) -> float:
    """🔢️ `enc_f64` in the reading direction."""
    return struct.unpack("<d", struct.pack("<Q", int(bits)))[0]


# endregion 🔖️Carrier


# region 🔖️Text primitives
def split_top_level(text: str) -> list:
    """📜️ Splits a list body on the commas that sit at bracket depth zero."""
    parts = []
    depth = 0
    current = []
    for character in text:
        if character == "[":
            depth += 1
        elif character == "]":
            depth -= 1
        if character == "," and depth == 0:
            parts.append("".join(current))
            current = []
            continue
        current.append(character)
    tail = "".join(current)
    if tail != "" or parts:
        parts.append(tail)
    return parts


def strip_brackets(text: str, what: str) -> str:
    """📜️ Unwraps one `[...]` group, refusing anything the grammar does not admit there."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("%s must be a bracketed group, found %r" % (what, text[:60]))
    return text[1:-1]


def items_of(text: str, what: str) -> list:
    """📜️ The members of a bracketed list, or nothing at all for `[]`."""
    return split_top_level(strip_brackets(text, what))


def tagged_group(text: str, what: str) -> tuple:
    """📜️ A `<letter>[...]` alternative — its tag letter and the body inside its brackets."""
    if len(text) < 1:
        raise AssertionError("%s must carry a tag letter, found %r" % (what, text[:60]))
    return text[0], text[1:]


def parse_option_hex(text: str, what: str):
    """📜️ `option-hex = "[" "0" "]" | "[" "1" "," hex "]"`, read as an optional string."""
    parts = items_of(text, what)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return text_of_hex(parts[1])
    raise AssertionError("%s is not a well-formed option-hex: %r" % (what, text[:60]))


def print_option_hex(value) -> str:
    """📜️ `option-hex` in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % hex_of_text(value)


def parse_option_num(text: str, what: str):
    """📜️ `option-num = "[" "0" "]" | "[" "1" "," INT "]"`, read as an optional `f64`."""
    parts = items_of(text, what)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return float_of(parts[1])
    raise AssertionError("%s is not a well-formed option-num: %r" % (what, text[:60]))


def print_option_num(value) -> str:
    """📜️ `option-num` in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % bits_of(value)


def parse_bool(text: str, what: str) -> bool:
    """📜️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("%s is not a well-formed bool: %r" % (what, text[:20]))
    return text == "1"


def print_bool(value: bool) -> str:
    """📜️ `bool` in the writing direction."""
    return "1" if value else "0"


# endregion 🔖️Text primitives


# region 🔖️Blocks
def parse_run_style(text: str) -> dict:
    """✍️ `run-style = "[" bool "," bool "," bool "," option-num "," option-hex ×3 "]"`."""
    parts = items_of(text, "a run style")
    if len(parts) != 7:
        raise AssertionError("a run style carries seven leaves, found %d" % len(parts))
    return {
        "bold": parse_bool(parts[0], "bold"),
        "italic": parse_bool(parts[1], "italic"),
        "underline": parse_bool(parts[2], "underline"),
        "size": parse_option_num(parts[3], "size"),
        "font": parse_option_hex(parts[4], "font"),
        "color": parse_option_hex(parts[5], "color"),
        "link": parse_option_hex(parts[6], "link"),
    }


def print_run_style(style: dict) -> str:
    """✍️ `run-style` in the writing direction."""
    return "[%s,%s,%s,%s,%s,%s,%s]" % (
        print_bool(style["bold"]),
        print_bool(style["italic"]),
        print_bool(style["underline"]),
        print_option_num(style["size"]),
        print_option_hex(style["font"]),
        print_option_hex(style["color"]),
        print_option_hex(style["link"]),
    )


def parse_run(text: str) -> dict:
    """✍️ `run = "[" hex "," run-style "]"`."""
    parts = items_of(text, "a run")
    if len(parts) != 2:
        raise AssertionError("a run carries a text and a style, found %d leaves" % len(parts))
    return {"text": text_of_hex(parts[0]), "style": parse_run_style(parts[1])}


def print_run(run: dict) -> str:
    """✍️ `run` in the writing direction."""
    return "[%s,%s]" % (hex_of_text(run["text"]), print_run_style(run["style"]))


def parse_block(text: str) -> dict:
    """📄️ document's own `block` union, copied verbatim into this subset's grammar."""
    letter, rest = tagged_group(text, "a block")
    if letter not in LETTER_BLOCK:
        raise AssertionError("%r is not one of document's eight block tags" % letter)
    kind = LETTER_BLOCK[letter]
    parts = items_of(rest, "a %s block" % kind)
    if kind == "paragraph":
        return {"kind": "paragraph", "style_id": parse_option_hex(parts[0], "styleId"), "runs": [parse_run(entry) for entry in items_of(parts[1], "runs")]}
    if kind == "heading":
        return {"kind": "heading", "level": int(parts[0]), "style_id": parse_option_hex(parts[1], "styleId"), "runs": [parse_run(entry) for entry in items_of(parts[2], "runs")]}
    if kind == "list":
        return {"kind": "list", "ordered": parse_bool(parts[0], "ordered"), "items": [{"blocks": [parse_block(block) for block in items_of(strip_brackets(entry, "a list item"), "list item blocks")]} for entry in items_of(parts[1], "list items")]}
    if kind == "table":
        return {"kind": "table", "rows": [parse_doc_row(entry) for entry in items_of(parts[0], "table rows")]}
    if kind == "code":
        return {"kind": "code", "language": parse_option_hex(parts[0], "language"), "text": text_of_hex(parts[1])}
    if kind == "quote":
        return {"kind": "quote", "blocks": [parse_block(entry) for entry in items_of(parts[0], "quoted blocks")]}
    if kind == "image":
        return {"kind": "image", "image_id": text_of_hex(parts[0]), "alt": text_of_hex(parts[1]), "width": parse_option_num(parts[2], "width"), "height": parse_option_num(parts[3], "height")}
    return {"kind": "pageBreak"}


def parse_doc_row(text: str) -> dict:
    """📄️ `doc-row = "[" "[" doc-cell-items? "]" "]"` — one wrapper more than this subset's own row."""
    return {"cells": [{"blocks": [parse_block(block) for block in items_of(strip_brackets(cell, "a doc cell"), "doc cell blocks")]} for cell in items_of(strip_brackets(text, "a doc row"), "doc cells")]}


def print_doc_row(row: dict) -> str:
    """📄️ `doc-row` in the writing direction."""
    return "[[%s]]" % ",".join("[[%s]]" % ",".join(print_block(block) for block in cell["blocks"]) for cell in row["cells"])


def print_block(block: dict) -> str:
    """📄️ document's own `block` union in the writing direction."""
    kind = block["kind"]
    if kind == "paragraph":
        return "P[%s,[%s]]" % (print_option_hex(block.get("style_id")), ",".join(print_run(run) for run in block.get("runs", [])))
    if kind == "heading":
        return "H[%d,%s,[%s]]" % (block["level"], print_option_hex(block.get("style_id")), ",".join(print_run(run) for run in block.get("runs", [])))
    if kind == "list":
        return "L[%s,[%s]]" % (print_bool(block.get("ordered", False)), ",".join("[[%s]]" % ",".join(print_block(inner) for inner in item["blocks"]) for item in block.get("items", [])))
    if kind == "table":
        return "T[[%s]]" % ",".join(print_doc_row(row) for row in block.get("rows", []))
    if kind == "code":
        return "C[%s,%s]" % (print_option_hex(block.get("language")), hex_of_text(block.get("text", "")))
    if kind == "quote":
        return "Q[[%s]]" % ",".join(print_block(inner) for inner in block.get("blocks", []))
    if kind == "image":
        return "I[%s,%s,%s,%s]" % (hex_of_text(block["image_id"]), hex_of_text(block.get("alt", "")), print_option_num(block.get("width")), print_option_num(block.get("height")))
    if kind == "pageBreak":
        return "B[]"
    raise AssertionError("%r is not one of document's eight block kinds" % kind)


# endregion 🔖️Blocks


# region 🔖️Shapes
def parse_frame(text: str) -> dict:
    """📐️ `frame = "[" point2 "," INT "," INT "]"`, every numeric leaf an `f64` bit pattern."""
    parts = items_of(text, "a frame")
    if len(parts) != 3:
        raise AssertionError("a frame carries an origin, a width and a height, found %d leaves" % len(parts))
    origin = items_of(parts[0], "a point2")
    return {"origin": {"x": float_of(origin[0]), "y": float_of(origin[1])}, "width": float_of(parts[1]), "height": float_of(parts[2])}


def print_frame(frame: dict) -> str:
    """📐️ `frame` in the writing direction."""
    return "[[%s,%s],%s,%s]" % (bits_of(frame["origin"]["x"]), bits_of(frame["origin"]["y"]), bits_of(frame["width"]), bits_of(frame["height"]))


def parse_placeholder_kind(text: str) -> dict:
    """🏷️ `placeholder-kind` — six fieldless letters plus `O[value]`."""
    letter = text[0]
    if letter not in LETTER_PLACEHOLDER:
        raise AssertionError("%r is not one of the seven placeholder tags" % letter)
    kind = LETTER_PLACEHOLDER[letter]
    if kind != "other":
        if len(text) != 1:
            raise AssertionError("the %s placeholder carries no payload, found %r" % (kind, text[:40]))
        return {"kind": kind}
    return {"kind": "other", "value": text_of_hex(strip_brackets(text[1:], "an other-placeholder value"))}


def print_placeholder_kind(kind: dict) -> str:
    """🏷️ `placeholder-kind` in the writing direction."""
    if kind["kind"] == "other":
        return "O[%s]" % hex_of_text(kind["value"])
    return PLACEHOLDER_LETTER[kind["kind"]]


def parse_shape(text: str) -> dict:
    """🧩️ `shape` — the four single-letter alternatives of this subset's own shape tree."""
    letter, rest = tagged_group(text, "a shape")
    if letter not in LETTER_SHAPE:
        raise AssertionError("%r is not one of the four shape tags" % letter)
    kind = LETTER_SHAPE[letter]
    parts = items_of(rest, "a %s shape" % kind)
    frame = parse_frame(parts[0])
    if kind == "textBox":
        return {"shapeKind": "textBox", "frame": frame, "blocks": [parse_block(entry) for entry in items_of(parts[1], "text box blocks")]}
    if kind == "picture":
        image = items_of(parts[1], "an image")
        return {"shapeKind": "picture", "frame": frame, "image": {"assetId": text_of_hex(image[0]), "mime": text_of_hex(image[1]), "bytes": list(bytes.fromhex(image[2]))}}
    if kind == "table":
        return {"shapeKind": "table", "frame": frame, "rows": [parse_table_row(entry) for entry in items_of(parts[1], "table rows")]}
    return {"shapeKind": "placeholder", "frame": frame, "kind": parse_placeholder_kind(parts[1])}


def parse_table_row(text: str) -> dict:
    """➖️ `row = "[" cell-items? "]"`, `cell = "[" block-items? "]"` — single-bracketed, unlike
    document's own doubly-wrapped `doc-row`/`doc-cell`."""
    return {"cells": [{"blocks": [parse_block(block) for block in items_of(cell, "cell blocks")]} for cell in items_of(text, "cells")]}


def print_table_row(row: dict) -> str:
    """➖️ `row` in the writing direction."""
    return "[%s]" % ",".join("[%s]" % ",".join(print_block(block) for block in cell["blocks"]) for cell in row["cells"])


def print_shape(shape: dict) -> str:
    """🧩️ `shape` in the writing direction."""
    kind = shape["shapeKind"]
    frame = print_frame(shape["frame"])
    if kind == "textBox":
        return "X[%s,[%s]]" % (frame, ",".join(print_block(block) for block in shape.get("blocks", [])))
    if kind == "picture":
        image = shape["image"]
        return "P[%s,[%s,%s,%s]]" % (frame, hex_of_text(image["assetId"]), hex_of_text(image["mime"]), bytes(image.get("bytes", [])).hex())
    if kind == "table":
        return "T[%s,[%s]]" % (frame, ",".join(print_table_row(row) for row in shape.get("rows", [])))
    if kind == "placeholder":
        return "H[%s,%s]" % (frame, print_placeholder_kind(shape["kind"]))
    raise AssertionError("%r is not one of the four shape kinds" % kind)


# endregion 🔖️Shapes


# region 🔖️Dsl
def strip_preamble(text: str) -> str:
    """📜️ Splits the mandatory text envelope preamble off and checks it names this artifact."""
    line, _, body = text.partition("\n")
    if line != DSL_PREAMBLE:
        raise AssertionError("the text envelope preamble is %r, expected %r" % (line, DSL_PREAMBLE))
    return body


def read_field(body: str, name: str) -> tuple:
    """📜️ Reads one `name=value` line off the front of the body, in the grammar's fixed order."""
    line, _, rest = body.partition("\n")
    prefix = name + "="
    if not line.startswith(prefix):
        raise AssertionError("expected a %r line, found %r" % (name, line[:60]))
    return line[len(prefix) :], rest


def parse_dsl(text: str) -> dict:
    """📜️ `document = artifact-mark schema-line masters-line layouts-line slides-line`."""
    body = strip_preamble(text)
    schema_hex, body = read_field(body, "schema")
    masters, body = read_field(body, "masters")
    layouts, body = read_field(body, "layouts")
    slides, body = read_field(body, "slides")
    if body != "":
        raise AssertionError("the document carries trailing content after its slides line: %r" % body[:60])
    return {
        "schema": text_of_hex(schema_hex),
        "masters": [parse_master(entry) for entry in items_of(masters, "masters")],
        "layouts": [parse_layout(entry) for entry in items_of(layouts, "layouts")],
        "slides": [parse_slide(entry) for entry in items_of(slides, "slides")],
    }


def parse_master(text: str) -> dict:
    """🗂️ `master = "[" hex "," "[" shape-items? "]" "]"`."""
    parts = items_of(text, "a master")
    return {"id": text_of_hex(parts[0]), "shapes": [parse_shape(entry) for entry in items_of(parts[1], "master shapes")]}


def parse_layout(text: str) -> dict:
    """📐️ `layout = "[" hex "," hex "," "[" shape-items? "]" "]"`."""
    parts = items_of(text, "a layout")
    return {"id": text_of_hex(parts[0]), "masterId": text_of_hex(parts[1]), "shapes": [parse_shape(entry) for entry in items_of(parts[2], "layout shapes")]}


def parse_slide(text: str) -> dict:
    """🎞️ `slide = "[" hex "," option-hex "," "[" shape-items? "]" "," "[" block-items? "]" "]"`."""
    parts = items_of(text, "a slide")
    return {
        "id": text_of_hex(parts[0]),
        "layoutId": parse_option_hex(parts[1], "layoutId"),
        "shapes": [parse_shape(entry) for entry in items_of(parts[2], "slide shapes")],
        "notes": [parse_block(entry) for entry in items_of(parts[3], "slide notes")],
    }


def print_dsl(document: dict) -> str:
    """📜️ The committed DSL grammar in the writing direction, line for line in its declared order."""
    masters = ",".join("[%s,[%s]]" % (hex_of_text(master["id"]), ",".join(print_shape(shape) for shape in master["shapes"])) for master in document["masters"])
    layouts = ",".join("[%s,%s,[%s]]" % (hex_of_text(layout["id"]), hex_of_text(layout["masterId"]), ",".join(print_shape(shape) for shape in layout["shapes"])) for layout in document["layouts"])
    slides = ",".join(
        "[%s,%s,[%s],[%s]]" % (hex_of_text(slide["id"]), print_option_hex(slide["layoutId"]), ",".join(print_shape(shape) for shape in slide["shapes"]), ",".join(print_block(block) for block in slide["notes"])) for slide in document["slides"]
    )
    return "\n".join([DSL_PREAMBLE, "schema=%s" % hex_of_text(document["schema"]), "masters=[%s]" % masters, "layouts=[%s]" % layouts, "slides=[%s]" % slides])


# endregion 🔖️Dsl


# region 🔖️Pack
def read_varint(data: bytes, at: int) -> tuple:
    """🔢️ LEB128, the `varint` every count and length prefix below uses."""
    value = 0
    shift = 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if byte & 0x80 == 0:
            return value, at
        shift += 7


def write_varint(value: int) -> bytes:
    """🔢️ LEB128 in the writing direction."""
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
            continue
        out.append(byte)
        return bytes(out)


def read_blob(data: bytes, at: int) -> tuple:
    """🔢️ A varint-length-prefixed byte run."""
    length, at = read_varint(data, at)
    return data[at : at + length], at + length


def write_blob(payload: bytes) -> bytes:
    """🔢️ A varint-length-prefixed byte run in the writing direction."""
    return write_varint(len(payload)) + payload


def read_f64(data: bytes, at: int) -> tuple:
    """🔢️ One little-endian `f64`."""
    return struct.unpack_from("<d", data, at)[0], at + 8


def read_frame(data: bytes, at: int) -> tuple:
    """📐️ The four `f64` leaves of a frame, origin first."""
    x, at = read_f64(data, at)
    y, at = read_f64(data, at)
    width, at = read_f64(data, at)
    height, at = read_f64(data, at)
    return {"origin": {"x": x, "y": y}, "width": width, "height": height}, at


def write_frame(frame: dict) -> bytes:
    """📐️ A frame in the writing direction."""
    return struct.pack("<dddd", frame["origin"]["x"], frame["origin"]["y"], frame["width"], frame["height"])


def read_blocks(data: bytes, at: int) -> tuple:
    """📄️ A varint-counted run of `DocBlock`s, each one an `enc_block` TEXT blob."""
    count, at = read_varint(data, at)
    blocks = []
    for _ in range(count):
        blob, at = read_blob(data, at)
        blocks.append(parse_block(blob.decode("utf-8")))
    return blocks, at


def write_blocks(blocks: list) -> bytes:
    """📄️ The same run in the writing direction."""
    out = bytearray(write_varint(len(blocks)))
    for block in blocks:
        out += write_blob(print_block(block).encode("utf-8"))
    return bytes(out)


def read_shape(data: bytes, at: int) -> tuple:
    """🧩️ One tag byte, one frame, then the variant's own payload."""
    tag = data[at]
    at += 1
    if tag >= len(SHAPE_ORDER):
        raise AssertionError("the pack shape tag %d is outside the declared union" % tag)
    kind = SHAPE_ORDER[tag]
    frame, at = read_frame(data, at)
    if kind == "textBox":
        blocks, at = read_blocks(data, at)
        return {"shapeKind": "textBox", "frame": frame, "blocks": blocks}, at
    if kind == "picture":
        asset, at = read_blob(data, at)
        mime, at = read_blob(data, at)
        payload, at = read_blob(data, at)
        return {"shapeKind": "picture", "frame": frame, "image": {"assetId": asset.decode("utf-8"), "mime": mime.decode("utf-8"), "bytes": list(payload)}}, at
    if kind == "table":
        rows, at = read_varint(data, at)
        table = []
        for _ in range(rows):
            cells, at = read_varint(data, at)
            row = []
            for _ in range(cells):
                blocks, at = read_blocks(data, at)
                row.append({"blocks": blocks})
            table.append({"cells": row})
        return {"shapeKind": "table", "frame": frame, "rows": table}, at
    ordinal = data[at]
    at += 1
    if ordinal >= len(PLACEHOLDER_ORDER):
        raise AssertionError("the pack placeholder ordinal %d is outside the declared union" % ordinal)
    if PLACEHOLDER_ORDER[ordinal] != "other":
        return {"shapeKind": "placeholder", "frame": frame, "kind": {"kind": PLACEHOLDER_ORDER[ordinal]}}, at
    value, at = read_blob(data, at)
    return {"shapeKind": "placeholder", "frame": frame, "kind": {"kind": "other", "value": value.decode("utf-8")}}, at


def write_shape(shape: dict) -> bytes:
    """🧩️ One shape in the writing direction."""
    kind = shape["shapeKind"]
    out = bytearray([SHAPE_ORDER.index(kind)])
    out += write_frame(shape["frame"])
    if kind == "textBox":
        out += write_blocks(shape.get("blocks", []))
        return bytes(out)
    if kind == "picture":
        image = shape["image"]
        out += write_blob(image["assetId"].encode("utf-8"))
        out += write_blob(image["mime"].encode("utf-8"))
        out += write_blob(bytes(image.get("bytes", [])))
        return bytes(out)
    if kind == "table":
        rows = shape.get("rows", [])
        out += write_varint(len(rows))
        for row in rows:
            out += write_varint(len(row["cells"]))
            for cell in row["cells"]:
                out += write_blocks(cell["blocks"])
        return bytes(out)
    placeholder = shape["kind"]
    out.append(PLACEHOLDER_ORDER.index(placeholder["kind"]))
    if placeholder["kind"] == "other":
        out += write_blob(placeholder["value"].encode("utf-8"))
    return bytes(out)


def read_shapes(data: bytes, at: int) -> tuple:
    """🧩️ A varint-counted shape tree."""
    count, at = read_varint(data, at)
    shapes = []
    for _ in range(count):
        shape, at = read_shape(data, at)
        shapes.append(shape)
    return shapes, at


def write_shapes(shapes: list) -> bytes:
    """🧩️ A shape tree in the writing direction."""
    out = bytearray(write_varint(len(shapes)))
    for shape in shapes:
        out += write_shape(shape)
    return bytes(out)


def parse_pack(data: bytes) -> dict:
    """🎒️ The committed binary envelope and the pack frame it wraps."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the binary envelope magic is %r, expected %r" % (data[:8], BINARY_MAGIC))
    token_length = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_length].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("the binary envelope token is %r, expected %r" % (token, PACK_TOKEN))
    at = 12 + token_length
    if data[at] != PACK_FORMAT:
        raise AssertionError("the pack format byte is %d, expected %d" % (data[at], PACK_FORMAT))
    at += 1
    schema, at = read_blob(data, at)
    count, at = read_varint(data, at)
    masters = []
    for _ in range(count):
        identifier, at = read_blob(data, at)
        shapes, at = read_shapes(data, at)
        masters.append({"id": identifier.decode("utf-8"), "shapes": shapes})
    count, at = read_varint(data, at)
    layouts = []
    for _ in range(count):
        identifier, at = read_blob(data, at)
        master, at = read_blob(data, at)
        shapes, at = read_shapes(data, at)
        layouts.append({"id": identifier.decode("utf-8"), "masterId": master.decode("utf-8"), "shapes": shapes})
    count, at = read_varint(data, at)
    slides = []
    for _ in range(count):
        identifier, at = read_blob(data, at)
        present = data[at]
        at += 1
        layout = None
        if present == 1:
            blob, at = read_blob(data, at)
            layout = blob.decode("utf-8")
        elif present != 0:
            raise AssertionError("the layout-id presence byte is %d, expected 0 or 1" % present)
        shapes, at = read_shapes(data, at)
        notes, at = read_blocks(data, at)
        slides.append({"id": identifier.decode("utf-8"), "layoutId": layout, "shapes": shapes, "notes": notes})
    if at != len(data):
        raise AssertionError("the pack frame ends %d bytes before its envelope does" % (len(data) - at))
    return {"schema": schema.decode("utf-8"), "masters": masters, "layouts": layouts, "slides": slides}


def pack_bytes(document: dict) -> bytes:
    """🎒️ The pack frame in the writing direction, inside the shared binary envelope."""
    body = bytearray([PACK_FORMAT])
    body += write_blob(document["schema"].encode("utf-8"))
    body += write_varint(len(document["masters"]))
    for master in document["masters"]:
        body += write_blob(master["id"].encode("utf-8"))
        body += write_shapes(master["shapes"])
    body += write_varint(len(document["layouts"]))
    for layout in document["layouts"]:
        body += write_blob(layout["id"].encode("utf-8"))
        body += write_blob(layout["masterId"].encode("utf-8"))
        body += write_shapes(layout["shapes"])
    body += write_varint(len(document["slides"]))
    for slide in document["slides"]:
        body += write_blob(slide["id"].encode("utf-8"))
        if slide["layoutId"] is None:
            body.append(0)
        else:
            body.append(1)
            body += write_blob(slide["layoutId"].encode("utf-8"))
        body += write_shapes(slide["shapes"])
        body += write_blocks(slide["notes"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
TAG_TO_KIND = {
    "noMutation": "no-mutation",
    "setSnapshot": "set-snapshot",
    "insertSlide": "insert-slide",
    "removeSlide": "remove-slide",
    "setSlideLayout": "set-slide-layout",
    "setSlideNotes": "set-slide-notes",
    "insertShape": "insert-shape",
    "removeShape": "remove-shape",
    "setShapeFrame": "set-shape-frame",
    "setTextBoxBlocks": "set-text-box-blocks",
    "insertMaster": "insert-master",
    "removeMaster": "remove-master",
    "insertLayout": "insert-layout",
    "removeLayout": "remove-layout",
    "setLayoutMaster": "set-layout-master",
}


def kind_of(mutation: dict) -> str:
    """🏷️ The kebab-case kind a wire payload names, refusing anything outside the vocabulary."""
    tag = mutation.get("mutation")
    if tag not in TAG_TO_KIND:
        raise AssertionError("%r is not one of this subset's fifteen declared verbs" % (tag,))
    return TAG_TO_KIND[tag]


def clone(value):
    """🧬️ A deep copy, so no arm ever aliases the document it was handed."""
    return json.loads(json.dumps(value))


def slide_at(document: dict, index: int, verb: str) -> dict:
    """🎞️ The slide a positional verb addresses, refusing an index the deck does not hold."""
    if index < 0 or index >= len(document["slides"]):
        raise AssertionError("%s addresses slide %d of a %d-slide deck" % (verb, index, len(document["slides"])))
    return document["slides"][index]


def shape_at(document: dict, slide_index: int, shape_index: int, verb: str) -> dict:
    """🧩️ The shape a positional verb addresses, refusing an index the slide does not hold."""
    slide = slide_at(document, slide_index, verb)
    if shape_index < 0 or shape_index >= len(slide["shapes"]):
        raise AssertionError("%s addresses shape %d of a %d-shape slide" % (verb, shape_index, len(slide["shapes"])))
    return slide["shapes"][shape_index]


def named(collection: list, identifier: str):
    """🔑️ The member of an id-keyed collection with this id, or `None`."""
    for entry in collection:
        if entry["id"] == identifier:
            return entry
    return None


def apply_mutation(document: dict, mutation: dict) -> dict:
    """▶️ One verb applied to a deck, returning the resulting deck.

    Each arm is the behaviour its committed `(before, mutation, after)` specification vector states:
    slides and shapes are INDEX-addressed and their verbs carry the exact position, while masters and
    layouts are ID-keyed and their insert verbs append an unknown id and replace a known one in
    place.
    """
    kind = kind_of(mutation)
    result = clone(document)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        replacement = clone(mutation["snapshot"])
        replacement["schema"] = document["schema"]
        return replacement
    if kind == "insert-slide":
        index = int(mutation["index"])
        if index < 0 or index > len(result["slides"]):
            raise AssertionError("insert-slide addresses position %d of a %d-slide deck" % (index, len(result["slides"])))
        result["slides"].insert(index, clone(mutation["slide"]))
        return result
    if kind == "remove-slide":
        index = int(mutation["index"])
        slide_at(result, index, "remove-slide")
        result["slides"].pop(index)
        return result
    if kind == "set-slide-layout":
        slide_at(result, int(mutation["index"]), "set-slide-layout")["layoutId"] = mutation["layout_id"]
        return result
    if kind == "set-slide-notes":
        slide_at(result, int(mutation["index"]), "set-slide-notes")["notes"] = clone(mutation["notes"])
        return result
    if kind == "insert-shape":
        slide = slide_at(result, int(mutation["slide_index"]), "insert-shape")
        index = int(mutation["shape_index"])
        if index < 0 or index > len(slide["shapes"]):
            raise AssertionError("insert-shape addresses position %d of a %d-shape slide" % (index, len(slide["shapes"])))
        slide["shapes"].insert(index, clone(mutation["shape"]))
        return result
    if kind == "remove-shape":
        slide = slide_at(result, int(mutation["slide_index"]), "remove-shape")
        index = int(mutation["shape_index"])
        shape_at(result, int(mutation["slide_index"]), index, "remove-shape")
        slide["shapes"].pop(index)
        return result
    if kind == "set-shape-frame":
        shape_at(result, int(mutation["slide_index"]), int(mutation["shape_index"]), "set-shape-frame")["frame"] = clone(mutation["frame"])
        return result
    if kind == "set-text-box-blocks":
        shape = shape_at(result, int(mutation["slide_index"]), int(mutation["shape_index"]), "set-text-box-blocks")
        if shape["shapeKind"] != "textBox":
            raise AssertionError("set-text-box-blocks addresses a %s shape" % shape["shapeKind"])
        shape["blocks"] = clone(mutation["blocks"])
        return result
    if kind == "insert-master":
        master = clone(mutation["master"])
        existing = named(result["masters"], master["id"])
        if existing is None:
            result["masters"].append(master)
        else:
            result["masters"][result["masters"].index(existing)] = master
        return result
    if kind == "remove-master":
        if named(result["masters"], mutation["id"]) is None:
            raise AssertionError("remove-master addresses %r, which the deck does not carry" % mutation["id"])
        result["masters"] = [entry for entry in result["masters"] if entry["id"] != mutation["id"]]
        return result
    if kind == "insert-layout":
        layout = clone(mutation["layout"])
        existing = named(result["layouts"], layout["id"])
        if existing is None:
            result["layouts"].append(layout)
        else:
            result["layouts"][result["layouts"].index(existing)] = layout
        return result
    if kind == "remove-layout":
        if named(result["layouts"], mutation["id"]) is None:
            raise AssertionError("remove-layout addresses %r, which the deck does not carry" % mutation["id"])
        result["layouts"] = [entry for entry in result["layouts"] if entry["id"] != mutation["id"]]
        return result
    layout = named(result["layouts"], mutation["id"])
    if layout is None:
        raise AssertionError("set-layout-master addresses %r, which the deck does not carry" % mutation["id"])
    layout["masterId"] = mutation["master_id"]
    return result


def inverse_mutation(document: dict, mutation: dict) -> dict:
    """↩️ The verb's own inverse against the deck it is about to be applied to."""
    kind = kind_of(mutation)
    if kind == "no-mutation":
        return {"mutation": "noMutation"}
    if kind == "set-snapshot":
        return {"mutation": "setSnapshot", "snapshot": clone(document)}
    if kind == "insert-slide":
        return {"mutation": "removeSlide", "index": int(mutation["index"])}
    if kind == "remove-slide":
        index = int(mutation["index"])
        return {"mutation": "insertSlide", "index": index, "slide": clone(slide_at(document, index, "remove-slide"))}
    if kind == "set-slide-layout":
        index = int(mutation["index"])
        return {"mutation": "setSlideLayout", "index": index, "layout_id": slide_at(document, index, "set-slide-layout")["layoutId"]}
    if kind == "set-slide-notes":
        index = int(mutation["index"])
        return {"mutation": "setSlideNotes", "index": index, "notes": clone(slide_at(document, index, "set-slide-notes")["notes"])}
    if kind == "insert-shape":
        return {"mutation": "removeShape", "slide_index": int(mutation["slide_index"]), "shape_index": int(mutation["shape_index"])}
    if kind == "remove-shape":
        slide_index = int(mutation["slide_index"])
        shape_index = int(mutation["shape_index"])
        return {"mutation": "insertShape", "slide_index": slide_index, "shape_index": shape_index, "shape": clone(shape_at(document, slide_index, shape_index, "remove-shape"))}
    if kind == "set-shape-frame":
        slide_index = int(mutation["slide_index"])
        shape_index = int(mutation["shape_index"])
        return {"mutation": "setShapeFrame", "slide_index": slide_index, "shape_index": shape_index, "frame": clone(shape_at(document, slide_index, shape_index, "set-shape-frame")["frame"])}
    if kind == "set-text-box-blocks":
        slide_index = int(mutation["slide_index"])
        shape_index = int(mutation["shape_index"])
        shape = shape_at(document, slide_index, shape_index, "set-text-box-blocks")
        if shape["shapeKind"] != "textBox":
            raise AssertionError("set-text-box-blocks addresses a %s shape" % shape["shapeKind"])
        return {"mutation": "setTextBoxBlocks", "slide_index": slide_index, "shape_index": shape_index, "blocks": clone(shape.get("blocks", []))}
    if kind == "insert-master":
        previous = named(document["masters"], mutation["master"]["id"])
        if previous is None:
            return {"mutation": "removeMaster", "id": mutation["master"]["id"]}
        return {"mutation": "insertMaster", "master": clone(previous)}
    if kind == "remove-master":
        previous = named(document["masters"], mutation["id"])
        if previous is None:
            raise AssertionError("remove-master addresses %r, which the deck does not carry" % mutation["id"])
        return {"mutation": "insertMaster", "master": clone(previous)}
    if kind == "insert-layout":
        previous = named(document["layouts"], mutation["layout"]["id"])
        if previous is None:
            return {"mutation": "removeLayout", "id": mutation["layout"]["id"]}
        return {"mutation": "insertLayout", "layout": clone(previous)}
    if kind == "remove-layout":
        previous = named(document["layouts"], mutation["id"])
        if previous is None:
            raise AssertionError("remove-layout addresses %r, which the deck does not carry" % mutation["id"])
        return {"mutation": "insertLayout", "layout": clone(previous)}
    layout = named(document["layouts"], mutation["id"])
    if layout is None:
        raise AssertionError("set-layout-master addresses %r, which the deck does not carry" % mutation["id"])
    return {"mutation": "setLayoutMaster", "id": mutation["id"], "master_id": layout["masterId"]}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own doc string. The Python `Context` exposes the raw plan, not a helper."""
    for step in ctx.scenario.get("steps", []):
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("%s declares no doc string" % ctx.scenario["id"])


def step_uris(ctx: Context, scheme: str) -> list:
    """🧫️ Every fixture URI of one scheme the scenario's steps name, in step order — including the
    cells of a step's data table, which is where the specification-vector paths live."""
    found = []
    for step in ctx.scenario.get("steps", []):
        cells = [step["text"]] + [cell for row in step.get("dataTable", []) or [] for cell in row]
        for cell in cells:
            for token in cell.split():
                if token.startswith(scheme):
                    found.append(token)
    return found


def fixture_json(ctx: Context, uri: str):
    """🧫️ A declared fixture read as JSON."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


def talk(ctx: Context) -> dict:
    """🎤️ The real derived talk deck, parsed through this implementation's own DSL reader."""
    return parse_dsl(ctx.fixture_bytes(TALK_DSL).decode("utf-8"))


def projection_of(document: dict) -> dict:
    """🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own
    structural JSON shape, field for field."""
    return clone(document)


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real derived talk deck by this implementation alone."""
    document = talk(ctx)
    mutation = fixture_json(ctx, next(uri for uri in step_uris(ctx, "local://") if uri.endswith("/🦠️mutation/🔣️.json")))
    return Outcome(projection_of(apply_mutation(document, mutation)))


def inverse(ctx: Context) -> Outcome:
    """↩️ The metamorphic inverse law on the real deck: the verb followed by its OWN computed inverse
    must restore the deck exactly, slide and shape ORDER included."""
    document = talk(ctx)
    mutation = fixture_json(ctx, next(uri for uri in step_uris(ctx, "local://") if uri.endswith("/🦠️mutation/🔣️.json")))
    undo = inverse_mutation(document, mutation)
    mutated = apply_mutation(document, mutation)
    restored = apply_mutation(mutated, undo)
    if restored != document:
        raise AssertionError("%s: undoing the mutation did not restore the deck" % ctx.scenario["id"])
    return Outcome({"mutated": projection_of(mutated), "restored": projection_of(restored)})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector, whose
    before-state is the committed `📽️deck` example artifact — a THIRD statement of what the verb
    means, independent of both implementations."""
    uris = step_uris(ctx, "local://")
    before = fixture_json(ctx, uris[0])
    mutation = fixture_json(ctx, uris[1])
    expected = fixture_json(ctx, uris[2])
    applied = apply_mutation(before, mutation)
    if applied != expected:
        raise AssertionError("%s: the applied deck is not the committed after-snapshot" % ctx.scenario["id"])
    return Outcome(projection_of(applied))


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of BOTH decks — the real derived talk deck and the committed
    `📽️deck` example — each re-emitted from the parsed document.

    🔒️ The BYTE half of the identity law. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` its binary twin, so reproducing all four files byte for byte is the CORRECT answer
    here and a must-differ tripwire would be exactly backwards. What stops that from being a codec
    agreeing with itself is that the Rust subject reproduces the same four files from its own reading
    of the same grammar, and the digests of what each side emitted are what the runner compares.
    """
    report = {}
    for name, dsl_uri, pack_uri in (("talk", TALK_DSL, TALK_PACK), ("deck", DECK_DSL, DECK_PACK)):
        dsl_bytes = ctx.fixture_bytes(dsl_uri)
        parsed = parse_dsl(dsl_bytes.decode("utf-8"))
        printed = print_dsl(parsed).encode("utf-8")
        if printed != dsl_bytes:
            raise AssertionError("identity-round-trip: re-printing the %s did not reproduce its committed DSL file" % name)
        pack = ctx.fixture_bytes(pack_uri)
        unpacked = parse_pack(pack)
        if unpacked != parsed:
            raise AssertionError("identity-round-trip: the %s's binary twin decodes to a different deck than its text artifact" % name)
        repacked = pack_bytes(parsed)
        if repacked != pack:
            raise AssertionError("identity-round-trip: re-encoding the %s did not reproduce its committed pack file" % name)
        report[name] = {"document": projection_of(parsed), "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}
    return Outcome(report)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the Python host calls, keyed by FULL expanded scenario id."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
