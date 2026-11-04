"""🐍️ Independent Python implementation of the `stdio.semio.presentation` carrier and its
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
* the pack body is the derived record document of the shared record-document codec
  (`🧿️semio/🚪️io/💾️binary/🎒️record-document/🐍️.py`), driven by this subset's committed record spec
  `…/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`: the SPK container, the fixed-Huffman deflate, the BLAKE3
  content hash and the schema-hashed record body are specified by `🧰️framework/🔨️modules/🎒️pack` and
  re-implemented there, not here. The committed `🎒️.pack.semio` files were written by the Rust record
  pack, so `parse_pack`/`pack_bytes` reproducing them is a cross-language byte agreement;
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
import importlib.util
import json
import os
import struct
import sys

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
DSL_PREAMBLE = "semio s.stdio.semio.presentation.dsl v1"

#: 🧩️ `shape = "X"… | "P"… | "T"… | "H"…`.
SHAPE_LETTER = {"textBox": "X", "picture": "P", "table": "T", "placeholder": "H"}
LETTER_SHAPE = {letter: kind for kind, letter in SHAPE_LETTER.items()}

#: 🏷️ `placeholder-kind = "T" | "S" | "B" | "F" | "N" | "D" | "O" "[" hex "]"`.
PLACEHOLDER_LETTER = {"title": "T", "subtitle": "S", "body": "B", "footer": "F", "slideNumber": "N", "dateTime": "D", "other": "O"}
LETTER_PLACEHOLDER = {letter: kind for kind, letter in PLACEHOLDER_LETTER.items()}

#: 📄️ document's own `DocBlock` union, letter for letter as the block grammar declares it.
BLOCK_LETTER = {"paragraph": "P", "heading": "H", "list": "L", "table": "T", "code": "C", "quote": "Q", "image": "I", "pageBreak": "B"}
LETTER_BLOCK = {letter: kind for kind, letter in BLOCK_LETTER.items()}

TALK_DSL = "shared://📽️mutate-semio-presentation/🎙️talk/🗣️.dsl.semio"
TALK_PACK = "shared://📽️mutate-semio-presentation/🎒️.pack.semio"
DECK_DSL = "asset://📽️deck/🗣️.dsl.semio"
DECK_PACK = "asset://📽️deck/🎒️.pack.semio"


def hex_of_text(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction, for a string leaf."""
    return text.encode("utf-8").hex()


def text_of_hex(hexed: str) -> str:
    """🪄️ The grammar's `hex` macro in the reading direction, for a string leaf."""
    return bytes.fromhex(hexed).decode("utf-8")


def bits_of(value: float) -> str:
    """🔢️ `enc_f64` — the decimal of `f64::to_bits()`, never a float literal."""
    return str(struct.unpack("<Q", struct.pack("<d", float(value)))[0])


def float_of(bits: str) -> float:
    """🌀️ `enc_f64` in the reading direction."""
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
    """🚪️ Unwraps one `[...]` group, refusing anything the grammar does not admit there."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("%s must be a bracketed group, found %r" % (what, text[:60]))
    return text[1:-1]


def items_of(text: str, what: str) -> list:
    """🧑️ The members of a bracketed list, or nothing at all for `[]`."""
    return split_top_level(strip_brackets(text, what))


def tagged_group(text: str, what: str) -> tuple:
    """🧍️ A `<letter>[...]` alternative — its tag letter and the body inside its brackets."""
    if len(text) < 1:
        raise AssertionError("%s must carry a tag letter, found %r" % (what, text[:60]))
    return text[0], text[1:]


def parse_option_hex(text: str, what: str):
    """🔘️ `option-hex = "[" "0" "]" | "[" "1" "," hex "]"`, read as an optional string."""
    parts = items_of(text, what)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return text_of_hex(parts[1])
    raise AssertionError("%s is not a well-formed option-hex: %r" % (what, text[:60]))


def print_option_hex(value) -> str:
    """🌁️ `option-hex` in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % hex_of_text(value)


def parse_option_num(text: str, what: str):
    """🔟️ `option-num = "[" "0" "]" | "[" "1" "," INT "]"`, read as an optional `f64`."""
    parts = items_of(text, what)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return float_of(parts[1])
    raise AssertionError("%s is not a well-formed option-num: %r" % (what, text[:60]))


def print_option_num(value) -> str:
    """🌂️ `option-num` in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % bits_of(value)


def parse_bool(text: str, what: str) -> bool:
    """🔀️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("%s is not a well-formed bool: %r" % (what, text[:20]))
    return text == "1"


def print_bool(value: bool) -> str:
    """🌃️ `bool` in the writing direction."""
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
    """🏃️ `run-style` in the writing direction."""
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
    """🌄️ `run = "[" hex "," run-style "]"`."""
    parts = items_of(text, "a run")
    if len(parts) != 2:
        raise AssertionError("a run carries a text and a style, found %d leaves" % len(parts))
    return {"text": text_of_hex(parts[0]), "style": parse_run_style(parts[1])}


def print_run(run: dict) -> str:
    """🌅️ `run` in the writing direction."""
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
    """📚️ `doc-row = "[" "[" doc-cell-items? "]" "]"` — one wrapper more than this subset's own row."""
    return {"cells": [{"blocks": [parse_block(block) for block in items_of(strip_brackets(cell, "a doc cell"), "doc cell blocks")]} for cell in items_of(strip_brackets(text, "a doc row"), "doc cells")]}


def print_doc_row(row: dict) -> str:
    """🧾️ `doc-row` in the writing direction."""
    return "[[%s]]" % ",".join("[[%s]]" % ",".join(print_block(block) for block in cell["blocks"]) for cell in row["cells"])


def print_block(block: dict) -> str:
    """🧱️ document's own `block` union in the writing direction."""
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
    """🌆️ `frame` in the writing direction."""
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
    """🌇️ `placeholder-kind` in the writing direction."""
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
    """🌈️ `row` in the writing direction."""
    return "[%s]" % ",".join("[%s]" % ",".join(print_block(block) for block in cell["blocks"]) for cell in row["cells"])


def print_shape(shape: dict) -> str:
    """🔷️ `shape` in the writing direction."""
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
    """✂️ Splits the mandatory text envelope preamble off and checks it names this artifact."""
    line, _, body = text.partition("\n")
    if line != DSL_PREAMBLE:
        raise AssertionError("the text envelope preamble is %r, expected %r" % (line, DSL_PREAMBLE))
    return body


def read_field(body: str, name: str) -> tuple:
    """📛️ Reads one `name=value` line off the front of the body, in the grammar's fixed order."""
    line, _, rest = body.partition("\n")
    prefix = name + "="
    if not line.startswith(prefix):
        raise AssertionError("expected a %r line, found %r" % (name, line[:60]))
    return line[len(prefix) :], rest


def parse_dsl(text: str) -> dict:
    """🌉️ `document = artifact-mark schema-line masters-line layouts-line slides-line`."""
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
    """🌊️ `layout = "[" hex "," hex "," "[" shape-items? "]" "]"`."""
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
    """🌋️ The committed DSL grammar in the writing direction, line for line in its declared order."""
    masters = ",".join("[%s,[%s]]" % (hex_of_text(master["id"]), ",".join(print_shape(shape) for shape in master["shapes"])) for master in document["masters"])
    layouts = ",".join("[%s,%s,[%s]]" % (hex_of_text(layout["id"]), hex_of_text(layout["masterId"]), ",".join(print_shape(shape) for shape in layout["shapes"])) for layout in document["layouts"])
    slides = ",".join(
        "[%s,%s,[%s],[%s]]" % (hex_of_text(slide["id"]), print_option_hex(slide["layoutId"]), ",".join(print_shape(shape) for shape in slide["shapes"]), ",".join(print_block(block) for block in slide["notes"])) for slide in document["slides"]
    )
    return "\n".join([DSL_PREAMBLE, "schema=%s" % hex_of_text(document["schema"]), "masters=[%s]" % masters, "layouts=[%s]" % layouts, "slides=[%s]" % slides])


# endregion 🔖️Dsl


# region 🔖️Pack
_HERE = os.path.dirname(os.path.abspath(__file__))


def _load_shared(alias, *parts):
    """🧬️ Loads a shared module from the `stdio.semio` artifact root, six folders above this case, and registers it under an importable name."""
    if alias in sys.modules:
        return sys.modules[alias]
    path = os.path.join(_HERE, *[os.pardir] * 6, *parts)
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


record_document = _load_shared("record_document", "🚪️io", "💾️binary", "🎒️record-document", "🐍️.py")
RECORD_SPEC = record_document.load_record_spec(os.path.join(_HERE, os.pardir, os.pardir, "🧬️schema", "📸️snapshot", "🗺️record-spec", "🔣️.json"))


def parse_pack(data: bytes) -> dict:
    """📦️ A `.pack.semio` file as the JSON snapshot its record document carries."""
    return record_document.decode_pack(RECORD_SPEC, data)


def pack_bytes(document: dict) -> bytes:
    """🔍️ The writing direction of `parse_pack`, envelope included."""
    return record_document.encode_pack(RECORD_SPEC, document)


# endregion 🔖️Pack


# region 🔖️Mutations
TAG_TO_KIND = {
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
    """🌌️ The kebab-case kind a wire payload names, refusing anything outside the vocabulary."""
    tag = mutation.get("mutation")
    if tag not in TAG_TO_KIND:
        raise AssertionError("%r is not one of this subset's sixteen declared verbs" % (tag,))
    return TAG_TO_KIND[tag]


def clone(value):
    """📑️ A deep copy, so no arm ever aliases the document it was handed."""
    return json.loads(json.dumps(value))


def slide_at(document: dict, index: int, verb: str) -> dict:
    """🔖️ The slide a positional verb addresses, refusing an index the deck does not hold."""
    if index < 0 or index >= len(document["slides"]):
        raise AssertionError("%s addresses slide %d of a %d-slide deck" % (verb, index, len(document["slides"])))
    return document["slides"][index]


def shape_at(document: dict, slide_index: int, shape_index: int, verb: str) -> dict:
    """🌍️ The shape a positional verb addresses, refusing an index the slide does not hold."""
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
            result["masters"].insert(min(int(mutation["at"]), len(result["masters"])) if "at" in mutation else len(result["masters"]), master)
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
            result["layouts"].insert(min(int(mutation["at"]), len(result["layouts"])) if "at" in mutation else len(result["layouts"]), layout)
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
        return {"mutation": "insertMaster", "master": clone(previous), "at": document["masters"].index(previous)}
    if kind == "insert-layout":
        previous = named(document["layouts"], mutation["layout"]["id"])
        if previous is None:
            return {"mutation": "removeLayout", "id": mutation["layout"]["id"]}
        return {"mutation": "insertLayout", "layout": clone(previous)}
    if kind == "remove-layout":
        previous = named(document["layouts"], mutation["id"])
        if previous is None:
            raise AssertionError("remove-layout addresses %r, which the deck does not carry" % mutation["id"])
        return {"mutation": "insertLayout", "layout": clone(previous), "at": document["layouts"].index(previous)}
    layout = named(document["layouts"], mutation["id"])
    if layout is None:
        raise AssertionError("set-layout-master addresses %r, which the deck does not carry" % mutation["id"])
    return {"mutation": "setLayoutMaster", "id": mutation["id"], "master_id": layout["masterId"]}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """🧵️ The scenario's own doc string. The Python `Context` exposes the raw plan, not a helper."""
    for step in ctx.scenario.get("steps", []):
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("%s declares no doc string" % ctx.scenario["id"])


def step_uris(ctx: Context, prefix: str) -> list:
    """🧫️ The fixture URIs under one prefix the scenario's steps name, in step order (the host's one fixture-URI grammar)."""
    return [uri for uri in ctx.step_input_uris() if uri.startswith(prefix)]


def snapshot_json(ctx: Context, uri: str):
    """🪑️ A declared fixture read as JSON."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def payload(ctx: Context) -> dict:
    """🦠️ The scenario's own mutation payload — the committed fixture its steps name, or, for the `no-mutation`
    baselines, the sentinel in its doc string."""
    uri = next((uri for uri in step_uris(ctx, "shared://📽️mutate-semio-presentation/") if uri.endswith("/🦠️mutation/🔣️.json")), None)
    return json.loads(doc_string(ctx)) if uri is None else snapshot_json(ctx, uri)


def talk(ctx: Context) -> dict:
    """🎤️ The real derived talk deck, parsed through this implementation's own DSL reader."""
    return parse_dsl(ctx.input_bytes(TALK_DSL).decode("utf-8"))


def projection_of(document: dict) -> dict:
    """🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own
    structural JSON shape, field for field."""
    return clone(document)


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """📱️ One verb applied to the real derived talk deck by this implementation alone."""
    document = talk(ctx)
    mutation = payload(ctx)
    return Outcome(projection_of(apply_mutation(document, mutation)))


def inverse(ctx: Context) -> Outcome:
    """⚖️ The metamorphic inverse law on the real deck: the verb followed by its OWN computed inverse
    must restore the deck exactly, slide and shape ORDER included."""
    document = talk(ctx)
    mutation = payload(ctx)
    undo = inverse_mutation(document, mutation)
    mutated = apply_mutation(document, mutation)
    restored = apply_mutation(mutated, undo)
    if restored != document:
        raise AssertionError("%s: undoing the mutation did not restore the deck" % ctx.scenario["id"])
    return Outcome({"mutated": projection_of(mutated), "restored": projection_of(restored)})


def spec_vector(ctx: Context) -> Outcome:
    """➡️ The same verb on its committed handcrafted `(before, mutation, after)` vector, whose
    before-state is the committed `📽️deck` example artifact — a THIRD statement of what the verb
    means, independent of both implementations."""
    uris = step_uris(ctx, "shared://📽️mutate-semio-presentation/")
    before = snapshot_json(ctx, uris[0])
    mutation = snapshot_json(ctx, uris[1])
    expected = snapshot_json(ctx, uris[2])
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
        dsl_bytes = ctx.input_bytes(dsl_uri)
        parsed = parse_dsl(dsl_bytes.decode("utf-8"))
        printed = print_dsl(parsed).encode("utf-8")
        if printed != dsl_bytes:
            raise AssertionError("identity-round-trip: re-printing the %s did not reproduce its committed DSL file" % name)
        pack = ctx.input_bytes(pack_uri)
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
    """🧭️ Registration entry point the Python host calls. Handlers are registered under the Scenario
    Outline base ids, which the host resolves for every Examples row, and plain scenarios under their
    own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
