"""🐍️ Independent Python implementation of the `s.stdio.semio.document` carriers and their
eighteen-verb mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is
a second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the `semio <envelope-id>.dsl v<version>` preamble for text and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line styles-line images-line blocks-line`, the eight
  single-letter-tagged `block` variants `P`/`H`/`L`/`T`/`C`/`Q`/`I`/`B` with the field lists it
  gives, `run`, `run-style`, `option-hex` and `option-f64`. That grammar also states the one thing a
  reader would otherwise get wrong: `enc_f64` prints `f64::to_bits()` as PLAIN DECIMAL DIGITS, not a
  float literal, so every `f64` leaf is a bare `INT` token;
* the pack body is the derived record document of the shared record-document codec
  (`🧿️semio/🚪️io/💾️binary/🎒️record-document/🐍️.py`), driven by this subset's committed record spec
  `…/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`: the SPK container, the fixed-Huffman deflate, the BLAKE3
  content hash and the schema-hashed record body are specified by `🧰️framework/🔨️modules/🎒️pack` and
  re-implemented there, not here. The committed `🎒️.pack.semio` files were written by the Rust record
  pack, so `parse_pack`/`pack_bytes` reproducing them is a cross-language byte agreement;
* the eighteen verbs and their named arguments are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, their JSON wire form is this case's
  committed per-kind specification vectors under `🧫️fixtures/`, and the three `DocBlockPath` segment
  tags are declared by the committed schema mirror `…/🧬️mutations/🟦️.ts`
  (`quote` / `listItem` / `tableCell`) with the camelCase member spelling (`blockIndex`) the committed vectors use.

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


# region 🔖️Vocabulary
#: 🏷️ Every variant of this subset's mutation vocabulary, in the order the mutations grammar's `op`
#: production lists them, kebab-cased as the catalog spells them.
KINDS = (
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

MEMO_DSL = "asset://🗒️memo/🗣️.dsl.semio"
MEMO_PACK = "asset://🗒️memo/🎒️.pack.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🪄️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    return raw_hex(hexed).decode("utf-8")


def raw_hex(hexed: str) -> bytes:
    """🧊️ The same macro over an opaque byte run — an image payload is not text."""
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
    """🧍️ The body of a `"[" … "]"` group, refusing anything that is not one."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("expected a bracketed group, got %r" % text)
    return text[1:-1]


def field(lines: list, name: str) -> str:
    """📛️ The right-hand side of one `name "=" value` body line; a missing line is an error."""
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
def clone(value):
    """🏛️ A structural copy, so applying a verb never writes through into the parsed document."""
    return json.loads(json.dumps(value))


def parts(mutation: dict) -> tuple:
    """✂️ Splits `{"mutation": "<camelTag>", …}` into its verb and its named arguments."""
    tag = mutation.get("mutation")
    if tag not in KIND_OF_TAG:
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(KIND_OF_TAG))))
    return KIND_OF_TAG[tag], mutation


def container(snapshot: dict, path: dict, verb: str) -> list:
    """🧭️ The block list one `DocBlockPath` addresses: each segment descends into a quote, a list
    item or a table cell, and `index` then addresses inside the list this returns."""
    blocks = snapshot["blocks"]
    for segment in path.get("segments") or []:
        at = segment["blockIndex"]
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
    """📱️ Applies one verb, returning a NEW snapshot. `insert-style`/`insert-image` insert at `at`
    when present and append otherwise, which is what the committed specification vectors record."""
    result = clone(snapshot)
    kind, args = parts(mutation)
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
        result["styles"].insert(min(args["at"], len(result["styles"])) if "at" in args else len(result["styles"]), clone(args["style"]))
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
        result["images"].insert(min(args["at"], len(result["images"])) if "at" in args else len(result["images"]), clone(args["image"]))
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
    A removal from the id-keyed `styles`/`images` collections is undone by an
    insertion carrying the removed index (`at`), so the original position is restored exactly."""
    kind, args = parts(mutation)
    tag = TAG_OF_KIND
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
        return {"mutation": tag["insert-style"], "style": clone(snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)]), "at": by_id(snapshot["styles"], args["id"], kind)}
    if kind == "set-style-name":
        return {"mutation": tag[kind], "id": args["id"], "name": snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)]["name"]}
    if kind == "set-style-based-on":
        return {"mutation": tag[kind], "id": args["id"], "based_on": snapshot["styles"][by_id(snapshot["styles"], args["id"], kind)]["basedOn"]}
    if kind == "insert-image":
        return {"mutation": tag["remove-image"], "id": args["image"]["id"]}
    if kind == "remove-image":
        return {"mutation": tag["insert-image"], "image": clone(snapshot["images"][by_id(snapshot["images"], args["id"], kind)]), "at": by_id(snapshot["images"], args["id"], kind)}
    was = snapshot["images"][by_id(snapshot["images"], args["id"], kind)]
    return {"mutation": tag[kind], "id": args["id"], "mime": was["mime"], "bytes": clone(was["bytes"])}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """🔌️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def memo(ctx: Context) -> dict:
    """🌀️ The real committed memo, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.input_bytes(MEMO_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.input_bytes(next(token for step in ctx.scenario["steps"] for token in step["text"].split() if token.startswith("shared://📃️mutate-semio-document/") and token.endswith("%s/🦠️mutation/🔣️.json" % kind))).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed memo by this implementation alone."""
    result = apply_mutation(memo(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """🧲️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the memo again — asserted here, and the MUTATED snapshot travels in the projection too, so the
    eighteen rows cannot all project the same restored value and compare vacuously."""
    snapshot = memo(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the memo\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """➡️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations, kept from before this oracle existed."""
    kind = ctx.row()
    committed = vector(ctx, kind)
    applied = apply_mutation(committed["before"], committed["mutation"])
    if applied != committed["after"]:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(committed["after"])))
    return Outcome(applied)


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real memo, each re-emitted from the parsed snapshot.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps that from being vacuous is that both committed files were written by the OTHER
    implementation: this file reproducing them is a cross-language byte agreement, not a codec
    agreeing with itself. The two encodings also cross-check each other — the binary twin has to
    decode to the same memo the text does, which no single codec can arrange on its own.
    """
    committed = ctx.input_bytes(MEMO_DSL)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing the memo did not reproduce the committed DSL bytes (%d vs %d bytes)\n     got: %s\nexpected: %s" % (len(printed), len(committed), printed.decode("utf-8"), committed.decode("utf-8")))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed memo lost content")
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("the committed memo declares schema %r, expected %r" % (snapshot["schema"], DOCUMENT_SCHEMA))
    committed_pack = ctx.input_bytes(MEMO_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != snapshot:
        raise AssertionError("the committed binary twin decodes to a different memo than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked), json.dumps(snapshot)))
    repacked = pack_bytes(snapshot)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the memo did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    if parse_pack(repacked) != snapshot:
        raise AssertionError("re-decoding the encoded pack lost content")
    declared = vector(ctx, "set-run-style")["before"]
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
    """🏠️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
