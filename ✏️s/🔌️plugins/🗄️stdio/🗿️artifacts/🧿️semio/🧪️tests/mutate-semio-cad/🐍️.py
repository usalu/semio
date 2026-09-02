"""🐍️ Independent Python implementation of the `s.stdio.semio.cad` carrier and its sixteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio` is a semio-native carrier that no
third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is a
second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the mandatory `semio <envelope-id>.dsl v<version>` preamble line — is specified in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line layers-line blocks-line entities-line`,
  `layer = "[" hex "," i32 "," hex "," bool "]"`, `entity-record = "[" hex "," hex "," entity "]"`,
  `block = "[" hex "," point2 "," entity-record-list "]"` and the nine single-letter-tagged
  `entity` variants `L`/`A`/`C`/`E`/`P`/`T`/`I`/`S`/`D` with the field lists that grammar gives;
* the sixteen verbs, their named arguments and the three `option-…` spellings are the committed
  grammar `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is
  this case's committed per-kind specification vectors under `🧫️fixtures/`.

Two leaves the grammar states by reference rather than in full, and how each was settled:

* `hex` is declared to be the framework's built-in `hex` MACRO, so every `handle`, `name`,
  `content`, `text`, `block_name` and `schema` leaf is the lowercase hex of that string's UTF-8
  bytes. Reading the committed artifact confirms it — `434f4e54494e554f5553` is `CONTINUOUS`.
* `number = INT | FLOAT` and the grammar records that every `f64` prints through plain Rust `{v}`
  Display, which drops the fractional part of an integral value. `print_number` below reproduces
  that rule, and the reading is PINNED by `identity-round-trip`, which re-prints the committed
  drawing byte for byte — `[2,2]`, `0.5` and `6.283` in it exercise both halves.

The one place the grammars are silent is where an `add-…` verb PLACES its addition in a name-keyed
collection. This implementation appends, which is the reading the committed specification vectors
also carry, and the consequence is stated rather than hidden: the inverse of a removal is the
matching addition, so a removal that was not final restores the value but not the position. The
feature exercises the forward direction at a MIDDLE entity and the inverse at the final one, and
says so.

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
#: production lists them, kebab-cased as the catalog spells them, paired with the camel-case tag the
#: committed vectors and the feature use on the wire.
KINDS = (
    "no-mutation",
    "set-snapshot",
    "add-layer",
    "remove-layer",
    "set-layer",
    "add-block",
    "remove-block",
    "set-block-base-point",
    "add-entity",
    "remove-entity",
    "set-entity-layer",
    "set-entity-geometry",
    "add-block-entity",
    "remove-block-entity",
    "set-block-entity-layer",
    "set-block-entity-geometry",
)


def camel(kind: str) -> str:
    """🐫 The wire tag of one verb: its kebab-case name in camel case."""
    head, *rest = kind.split("-")
    return head + "".join(word.capitalize() for word in rest)


TAG_OF_KIND = {kind: camel(kind) for kind in KINDS}
KIND_OF_TAG = {tag: kind for kind, tag in TAG_OF_KIND.items()}

#: 📐️ `entity = line | arc | circle | ellipse | polyline | text-entity | insert | solid | dimension`
#: — the grammar's single-letter tags and the field lists it declares for each.
ENTITY_FIELDS = {
    "L": ("line", (("a", "point2"), ("b", "point2"))),
    "A": ("arc", (("center", "point2"), ("radius", "number"), ("start_angle", "number"), ("end_angle", "number"))),
    "C": ("circle", (("center", "point2"), ("radius", "number"))),
    "E": ("ellipse", (("center", "point2"), ("major_axis_end", "point2"), ("ratio", "number"), ("start_param", "number"), ("end_param", "number"))),
    "P": ("polyline", (("vertices", "point2-list"), ("closed", "bool"))),
    "T": ("text", (("position", "point2"), ("height", "number"), ("rotation", "number"), ("content", "hex"))),
    "I": ("insert", (("block_name", "hex"), ("insertion_point", "point2"), ("scale", "point2"), ("rotation", "number"))),
    "S": ("solid", (("p1", "point2"), ("p2", "point2"), ("p3", "point2"), ("p4", "point2"))),
    "D": ("dimension", (("def_point", "point2"), ("text_position", "point2"), ("measurement", "number"), ("text", "hex"))),
}
TAG_OF_ENTITY = {name: tag for tag, (name, _) in ENTITY_FIELDS.items()}

DOCUMENT_SCHEMA = "stdio.semio.cad"
DSL_PREAMBLE = "semio stdio.semio.cad.dsl v1"

DRAWING_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio"
DRAWING_PACK = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📐️drawing/🖼️assets/🎒️.pack.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def read_number(text: str) -> float:
    """🔢️ `number = INT | FLOAT` — one `f64` leaf, integral values spelled without a point."""
    try:
        return float(text)
    except ValueError:
        raise AssertionError("expected a number, got %r" % text)


def print_number(value: float) -> str:
    """🔢️ The writing direction of `number`: Rust's `{v}` Display for `f64`, which prints an
    integral value with no fractional part and otherwise the shortest round-tripping form."""
    if value == int(value):
        return "%d" % int(value)
    return repr(float(value))


def read_bool(text: str) -> bool:
    """🔘️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("expected a bool spelled 0 or 1, got %r" % text)
    return text == "1"


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


# endregion 🔖️Carrier


# region 🔖️Dsl
def read_point2(text: str) -> dict:
    """📍️ `point2 = "[" number "," number "]"`."""
    x, y = split_top_level(bracketed(text))
    return {"x": read_number(x), "y": read_number(y)}


def write_point2(point: dict) -> str:
    return "[%s,%s]" % (print_number(point["x"]), print_number(point["y"]))


def read_leaf(text: str, shape: str):
    if shape == "point2":
        return read_point2(text)
    if shape == "point2-list":
        return [read_point2(one) for one in split_top_level(bracketed(text))]
    if shape == "number":
        return read_number(text)
    if shape == "bool":
        return read_bool(text)
    return text_of(text)


def write_leaf(value, shape: str) -> str:
    if shape == "point2":
        return write_point2(value)
    if shape == "point2-list":
        return "[%s]" % ",".join(write_point2(one) for one in value)
    if shape == "number":
        return print_number(value)
    if shape == "bool":
        return "1" if value else "0"
    return hex_of(value)


def read_entity(text: str) -> dict:
    """📐️ One tagged `entity` value: a single-letter tag then its bracketed field list."""
    tag, rest = text[:1], text[1:]
    if tag not in ENTITY_FIELDS:
        raise AssertionError("unknown entity tag %r — the grammar declares %s" % (tag, ", ".join(sorted(ENTITY_FIELDS))))
    name, fields = ENTITY_FIELDS[tag]
    parts = split_top_level(bracketed(rest))
    if len(parts) != len(fields):
        raise AssertionError("entity %s takes %d field(s), got %d in %r" % (name, len(fields), len(parts), text))
    entity = {"kind": name}
    for (key, shape), part in zip(fields, parts):
        entity[key] = read_leaf(part, shape)
    return entity


def write_entity(entity: dict) -> str:
    if entity["kind"] not in TAG_OF_ENTITY:
        raise AssertionError("unknown entity kind %r" % entity["kind"])
    tag = TAG_OF_ENTITY[entity["kind"]]
    _, fields = ENTITY_FIELDS[tag]
    return "%s[%s]" % (tag, ",".join(write_leaf(entity[key], shape) for key, shape in fields))


def read_layer(text: str) -> dict:
    """🗂️ `layer = "[" hex "," i32 "," hex "," bool "]"`."""
    name, color, line_type, visible = split_top_level(bracketed(text))
    return {"name": text_of(name), "colorIndex": int(color), "lineType": text_of(line_type), "visible": read_bool(visible)}


def write_layer(layer: dict) -> str:
    return "[%s,%d,%s,%s]" % (hex_of(layer["name"]), int(layer["colorIndex"]), hex_of(layer["lineType"]), "1" if layer["visible"] else "0")


def read_record(text: str) -> dict:
    """🧾️ `entity-record = "[" hex "," hex "," entity "]"` — handle, layer, geometry."""
    handle, layer, entity = split_top_level(bracketed(text))
    return {"handle": text_of(handle), "layer": text_of(layer), "entity": read_entity(entity)}


def write_record(record: dict) -> str:
    return "[%s,%s,%s]" % (hex_of(record["handle"]), hex_of(record["layer"]), write_entity(record["entity"]))


def read_block(text: str) -> dict:
    """🧱️ `block = "[" hex "," point2 "," entity-record-list "]"`."""
    name, base, records = split_top_level(bracketed(text))
    return {"name": text_of(name), "basePoint": read_point2(base), "entities": [read_record(each) for each in split_top_level(bracketed(records))]}


def write_block(block: dict) -> str:
    return "[%s,%s,[%s]]" % (hex_of(block["name"]), write_point2(block["basePoint"]), ",".join(write_record(record) for record in block["entities"]))


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line layers-line blocks-line entities-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    return {
        "schema": text_of(field(lines, "schema")),
        "layers": [read_layer(each) for each in split_top_level(bracketed(field(lines, "layers")))],
        "blocks": [read_block(each) for each in split_top_level(bracketed(field(lines, "blocks")))],
        "entities": [read_record(each) for each in split_top_level(bracketed(field(lines, "entities")))],
    }


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    body = [
        "schema=%s" % hex_of(snapshot["schema"]),
        "layers=[%s]" % ",".join(write_layer(layer) for layer in snapshot["layers"]),
        "blocks=[%s]" % ",".join(write_block(block) for block in snapshot["blocks"]),
        "entities=[%s]" % ",".join(write_record(record) for record in snapshot["entities"]),
    ]
    return "\n".join([DSL_PREAMBLE] + body)


# endregion 🔖️Dsl


# region 🔖️Pack
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
PACK_TOKEN = "stdio.semio.cad.pack v1"
PACK_FORMAT = 1


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
        """🔢️ LEB128, seven bits per byte, little end first — `write_str_lp`'s own length prefix."""
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


def put_point2(point: dict) -> bytes:
    return put_real(point["x"]) + put_real(point["y"])


def take_point2(cursor: Cursor) -> dict:
    return {"x": cursor.real(), "y": cursor.real()}


#: 📐️ The per-variant tag byte of a packed `entity`, in the grammar's own `entity` production order
#: — `line | arc | circle | ellipse | polyline | text-entity | insert | solid | dimension`. The
#: committed drawing confirms every one of the nine: its `h1` arc carries `0x01`, its `h8` dimension
#: `0x08`, and the `door` block's line `0x00`.
PACK_TAGS = ("line", "arc", "circle", "ellipse", "polyline", "text", "insert", "solid", "dimension")


def take_entity(cursor: Cursor) -> dict:
    tag = cursor.byte()
    if tag >= len(PACK_TAGS):
        raise AssertionError("unknown packed entity tag %d — the grammar declares nine variants" % tag)
    kind = PACK_TAGS[tag]
    if kind == "line":
        return {"kind": kind, "a": take_point2(cursor), "b": take_point2(cursor)}
    if kind == "arc":
        return {"kind": kind, "center": take_point2(cursor), "radius": cursor.real(), "start_angle": cursor.real(), "end_angle": cursor.real()}
    if kind == "circle":
        return {"kind": kind, "center": take_point2(cursor), "radius": cursor.real()}
    if kind == "ellipse":
        return {"kind": kind, "center": take_point2(cursor), "major_axis_end": take_point2(cursor), "ratio": cursor.real(), "start_param": cursor.real(), "end_param": cursor.real()}
    if kind == "polyline":
        return {"kind": kind, "vertices": [take_point2(cursor) for _ in range(cursor.varint())], "closed": cursor.byte() == 1}
    if kind == "text":
        return {"kind": kind, "position": take_point2(cursor), "height": cursor.real(), "rotation": cursor.real(), "content": cursor.string()}
    if kind == "insert":
        return {"kind": kind, "block_name": cursor.string(), "insertion_point": take_point2(cursor), "scale": take_point2(cursor), "rotation": cursor.real()}
    if kind == "solid":
        return {"kind": kind, "p1": take_point2(cursor), "p2": take_point2(cursor), "p3": take_point2(cursor), "p4": take_point2(cursor)}
    return {"kind": kind, "def_point": take_point2(cursor), "text_position": take_point2(cursor), "measurement": cursor.real(), "text": cursor.string()}


def put_entity(entity: dict) -> bytes:
    kind = entity["kind"]
    if kind not in PACK_TAGS:
        raise AssertionError("unknown entity kind %r" % kind)
    out = bytes([PACK_TAGS.index(kind)])
    if kind == "line":
        return out + put_point2(entity["a"]) + put_point2(entity["b"])
    if kind == "arc":
        return out + put_point2(entity["center"]) + put_real(entity["radius"]) + put_real(entity["start_angle"]) + put_real(entity["end_angle"])
    if kind == "circle":
        return out + put_point2(entity["center"]) + put_real(entity["radius"])
    if kind == "ellipse":
        return out + put_point2(entity["center"]) + put_point2(entity["major_axis_end"]) + put_real(entity["ratio"]) + put_real(entity["start_param"]) + put_real(entity["end_param"])
    if kind == "polyline":
        return out + put_varint(len(entity["vertices"])) + b"".join(put_point2(one) for one in entity["vertices"]) + bytes([1 if entity["closed"] else 0])
    if kind == "text":
        return out + put_point2(entity["position"]) + put_real(entity["height"]) + put_real(entity["rotation"]) + put_string(entity["content"])
    if kind == "insert":
        return out + put_string(entity["block_name"]) + put_point2(entity["insertion_point"]) + put_point2(entity["scale"]) + put_real(entity["rotation"])
    if kind == "solid":
        return out + put_point2(entity["p1"]) + put_point2(entity["p2"]) + put_point2(entity["p3"]) + put_point2(entity["p4"])
    return out + put_point2(entity["def_point"]) + put_point2(entity["text_position"]) + put_real(entity["measurement"]) + put_string(entity["text"])


def take_record(cursor: Cursor) -> dict:
    return {"handle": cursor.string(), "layer": cursor.string(), "entity": take_entity(cursor)}


def put_record(record: dict) -> bytes:
    return put_string(record["handle"]) + put_string(record["layer"]) + put_entity(record["entity"])


def parse_pack(payload: bytes) -> dict:
    """📦️ The binary twin of the DSL. The committed protocol
    `../../🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio`
    fully describes the envelope and the `format u8` + varint-length-prefixed `schema` head, and
    then declares the three collections one opaque trailing `payload` chain by its own admission.
    That layer was therefore DERIVED from the committed `🎒️.pack.semio` bytes: varint counts,
    `write_str_lp` length-prefixed UTF-8 strings, little-endian `f64` coordinates, a `u8` bool and a
    per-variant `u8` entity tag in the grammar's own variant order, with field order taken from the
    DSL grammar. The derivation is PINNED — `pack_bytes` re-encodes that committed file byte for
    byte, which it could not do from a misreading."""
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
    snapshot = {"schema": cursor.string(), "layers": [], "blocks": [], "entities": []}
    for _ in range(cursor.varint()):
        snapshot["layers"].append({"name": cursor.string(), "colorIndex": cursor.varint(), "lineType": cursor.string(), "visible": cursor.byte() == 1})
    for _ in range(cursor.varint()):
        name, base = cursor.string(), take_point2(cursor)
        snapshot["blocks"].append({"name": name, "basePoint": base, "entities": [take_record(cursor) for _ in range(cursor.varint())]})
    snapshot["entities"] = [take_record(cursor) for _ in range(cursor.varint())]
    cursor.done()
    return snapshot


def pack_bytes(snapshot: dict) -> bytes:
    """📦️ The same frame in the writing direction; `identity-round-trip` requires it to reproduce
    the committed binary twin byte for byte."""
    body = bytearray([PACK_FORMAT])
    body += put_string(snapshot["schema"])
    body += put_varint(len(snapshot["layers"]))
    for layer in snapshot["layers"]:
        body += put_string(layer["name"]) + put_varint(int(layer["colorIndex"])) + put_string(layer["lineType"]) + bytes([1 if layer["visible"] else 0])
    body += put_varint(len(snapshot["blocks"]))
    for block in snapshot["blocks"]:
        body += put_string(block["name"]) + put_point2(block["basePoint"]) + put_varint(len(block["entities"]))
        for record in block["entities"]:
            body += put_record(record)
    body += put_varint(len(snapshot["entities"]))
    for record in snapshot["entities"]:
        body += put_record(record)
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


def named(items: list, key: str, value: str, verb: str) -> int:
    """🔎️ The position of the entry a name-keyed verb addresses. An unaddressable name is a
    refusal, never a silent no-op — a quietly skipped mutation would report as a pass."""
    for at, item in enumerate(items):
        if item[key] == value:
            return at
    raise AssertionError("%s addresses %s %r, which the document does not carry" % (verb, key, value))


def block_named(snapshot: dict, args: dict, verb: str) -> dict:
    return snapshot["blocks"][named(snapshot["blocks"], "name", args["block_name"], verb)]


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW snapshot. `add-…` appends to its name-keyed collection,
    which is what the committed specification vectors record and what the grammars leave open."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        return clone(args["snapshot"])
    if kind == "add-layer":
        result["layers"].append(clone(args["layer"]))
        return result
    if kind == "remove-layer":
        del result["layers"][named(result["layers"], "name", args["name"], kind)]
        return result
    if kind == "set-layer":
        layer = result["layers"][named(result["layers"], "name", args["name"], kind)]
        for key, member in (("color_index", "colorIndex"), ("line_type", "lineType"), ("visible", "visible")):
            if args.get(key) is not None:
                layer[member] = args[key]
        return result
    if kind == "add-block":
        result["blocks"].append(clone(args["block"]))
        return result
    if kind == "remove-block":
        del result["blocks"][named(result["blocks"], "name", args["name"], kind)]
        return result
    if kind == "set-block-base-point":
        result["blocks"][named(result["blocks"], "name", args["name"], kind)]["basePoint"] = clone(args["base_point"])
        return result
    if kind == "add-entity":
        result["entities"].append(clone(args["entity"]))
        return result
    if kind == "remove-entity":
        del result["entities"][named(result["entities"], "handle", args["handle"], kind)]
        return result
    if kind == "set-entity-layer":
        result["entities"][named(result["entities"], "handle", args["handle"], kind)]["layer"] = args["layer"]
        return result
    if kind == "set-entity-geometry":
        result["entities"][named(result["entities"], "handle", args["handle"], kind)]["entity"] = clone(args["entity"])
        return result
    block = block_named(result, args, kind)
    if kind == "add-block-entity":
        block["entities"].append(clone(args["entity"]))
        return result
    at = named(block["entities"], "handle", args["handle"], kind)
    if kind == "remove-block-entity":
        del block["entities"][at]
        return result
    if kind == "set-block-entity-layer":
        block["entities"][at]["layer"] = args["layer"]
        return result
    block["entities"][at]["entity"] = clone(args["entity"])
    return result


def inverse_mutation(snapshot: dict, mutation: dict) -> dict:
    """↩️ The undo of one verb against the state it was applied to. An addition is undone by the
    matching removal and an overwrite by an overwrite with the value it displaced. Because `add-…`
    appends, undoing a NON-FINAL removal restores the value at the end of its collection rather than
    in place — the feature exercises the inverse at the final entry and states that limit."""
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return {"mutation": TAG_OF_KIND[kind]}
    if kind == "set-snapshot":
        return {"mutation": TAG_OF_KIND[kind], "snapshot": clone(snapshot)}
    if kind == "add-layer":
        return {"mutation": TAG_OF_KIND["remove-layer"], "name": args["layer"]["name"]}
    if kind == "remove-layer":
        return {"mutation": TAG_OF_KIND["add-layer"], "layer": clone(snapshot["layers"][named(snapshot["layers"], "name", args["name"], kind)])}
    if kind == "set-layer":
        was = snapshot["layers"][named(snapshot["layers"], "name", args["name"], kind)]
        undo = {"mutation": TAG_OF_KIND[kind], "name": args["name"]}
        for key, member in (("color_index", "colorIndex"), ("line_type", "lineType"), ("visible", "visible")):
            if args.get(key) is not None:
                undo[key] = was[member]
        return undo
    if kind == "add-block":
        return {"mutation": TAG_OF_KIND["remove-block"], "name": args["block"]["name"]}
    if kind == "remove-block":
        return {"mutation": TAG_OF_KIND["add-block"], "block": clone(snapshot["blocks"][named(snapshot["blocks"], "name", args["name"], kind)])}
    if kind == "set-block-base-point":
        was = snapshot["blocks"][named(snapshot["blocks"], "name", args["name"], kind)]
        return {"mutation": TAG_OF_KIND[kind], "name": args["name"], "base_point": clone(was["basePoint"])}
    if kind == "add-entity":
        return {"mutation": TAG_OF_KIND["remove-entity"], "handle": args["entity"]["handle"]}
    if kind == "remove-entity":
        return {"mutation": TAG_OF_KIND["add-entity"], "entity": clone(snapshot["entities"][named(snapshot["entities"], "handle", args["handle"], kind)])}
    if kind in ("set-entity-layer", "set-entity-geometry"):
        was = snapshot["entities"][named(snapshot["entities"], "handle", args["handle"], kind)]
        if kind == "set-entity-layer":
            return {"mutation": TAG_OF_KIND[kind], "handle": args["handle"], "layer": was["layer"]}
        return {"mutation": TAG_OF_KIND[kind], "handle": args["handle"], "entity": clone(was["entity"])}
    block = block_named(snapshot, args, kind)
    if kind == "add-block-entity":
        return {"mutation": TAG_OF_KIND["remove-block-entity"], "block_name": args["block_name"], "handle": args["entity"]["handle"]}
    was = block["entities"][named(block["entities"], "handle", args["handle"], kind)]
    if kind == "remove-block-entity":
        return {"mutation": TAG_OF_KIND["add-block-entity"], "block_name": args["block_name"], "entity": clone(was)}
    if kind == "set-block-entity-layer":
        return {"mutation": TAG_OF_KIND[kind], "block_name": args["block_name"], "handle": args["handle"], "layer": was["layer"]}
    return {"mutation": TAG_OF_KIND[kind], "block_name": args["block_name"], "handle": args["handle"], "entity": clone(was["entity"])}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def drawing(ctx: Context) -> dict:
    """📐️ The real committed drawing, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(DRAWING_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.fixture_bytes("local://🦠️%s.json" % kind).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed drawing by this implementation alone."""
    result = apply_mutation(drawing(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the drawing again — asserted here, and the MUTATED snapshot travels in the projection too, so
    the sixteen rows cannot all project the same restored value and compare vacuously."""
    snapshot = drawing(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the drawing\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
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
    """🔁️ Both committed encodings of the real drawing, each re-emitted from the parsed snapshot.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps that from being vacuous is that both committed files were written by the OTHER
    implementation: this file reproducing them is a cross-language byte agreement, not a codec
    agreeing with itself. The two encodings also cross-check each other — the binary twin has to
    decode to the same drawing the text does, which no single codec can arrange on its own.
    """
    committed = ctx.fixture_bytes(DRAWING_DSL)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing the drawing did not reproduce the committed DSL bytes (%d vs %d bytes)\n     got: %s\nexpected: %s" % (len(printed), len(committed), printed.decode("utf-8"), committed.decode("utf-8")))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed drawing lost content")
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("the committed drawing declares schema %r, expected %r" % (snapshot["schema"], DOCUMENT_SCHEMA))
    committed_pack = ctx.fixture_bytes(DRAWING_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != snapshot:
        raise AssertionError("the committed binary twin decodes to a different drawing than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked), json.dumps(snapshot)))
    repacked = pack_bytes(snapshot)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the drawing did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    if parse_pack(repacked) != snapshot:
        raise AssertionError("re-decoding the encoded pack lost content")
    declared = vector(ctx, "no-mutation")["before"]
    if snapshot != declared:
        raise AssertionError("the real committed drawing does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(snapshot), json.dumps(declared)))
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
