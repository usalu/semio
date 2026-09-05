"""🐍️ Independent Python implementation of the `stdio.semio.drawing` carrier and its seventeen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, and the earlier survey of the vector-graphics libraries still
stands on its merits: `usvg`/`resvg` model SVG, which has no counterpart for this subset's anonymous
recursive `DrawNode` tree addressed by a structural `NodePath`, nor for its four hierarchy verbs;
`lyon`/`kurbo` model path geometry alone and could adjudicate at most `replace-path`. The second
producer THE STANDARD requires is therefore a second IMPLEMENTATION, written in another language
from the format's own committed specification:

* the envelope — `semio <schema>.dsl v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line canvas-line styles-line layers-line`, the four
  single-letter `node` tags with `G`'s genuinely recursive `children`, the six `segment` tags, and
  the three `option-` productions;
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio` and its
  Kaitai mirror, which declare `format` and the varint-length-prefixed `schema` and then stop at one
  opaque `payload` chain by their own admission. That layer was DERIVED from the committed
  `✳️base/📚️examples/🖍️sketch` bytes against their readable DSL twin — which between them exhibit
  every node tag and every segment tag, including the arc — and the derivation is pinned by
  re-encoding that committed file byte for byte, which a misreading could not do;
* the seventeen verbs, their argument lists and their JSON wire form are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, the committed proto mirror and the
  committed per-kind `(before, mutation, after)` specification vectors, which pin each verb's
  semantics — including what the four hierarchy verbs do to a nested tree.

Nothing here imports, links, wraps or transliterates the Rust subject. Where the two disagree the
disagreement is a finding, not something to tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import struct

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
DSL_PREAMBLE = "semio stdio.semio.drawing.dsl v1"
PACK_TOKEN = "stdio.semio.drawing.pack v1"
PACK_FORMAT = 1

#: 🖍️ `node = "P"… | "T"… | "G"… | "I"…`, in the enum order the pack tag byte indexes — the committed
#: sketch's group root carries `0x02` and its image child `0x03`.
NODE_ORDER = ("path", "text", "group-nodes", "image")
NODE_LETTER = {"path": "P", "text": "T", "group-nodes": "G", "image": "I"}
LETTER_NODE = {letter: kind for kind, letter in NODE_LETTER.items()}

#: ✏️ `segment = "M"… | "L"… | "C"… | "Q"… | "A"… | "Z"`, likewise — the sketch's one path exhibits
#: all six in that order and its pack tags them `0x00`…`0x05`.
SEGMENT_ORDER = ("moveTo", "lineTo", "cubicTo", "quadTo", "arcTo", "close")
SEGMENT_LETTER = {"moveTo": "M", "lineTo": "L", "cubicTo": "C", "quadTo": "Q", "arcTo": "A", "close": "Z"}
LETTER_SEGMENT = {letter: kind for kind, letter in SEGMENT_LETTER.items()}

KINDS = (
    "create-layer",
    "delete-layer",
    "create-node",
    "delete-node",
    "move-node",
    "drag-nodes",
    "rotate-node",
    "scale-node",
    "reorder-nodes",
    "group-nodes",
    "ungroup-node",
    "flatten-node",
    "unflatten-node",
    "replace-path",
    "replace-fill",
    "change-stroke-color",
    "change-stroke-width",
)

ARTIFACT_DSL = "local://🗣️.dsl.semio"
ARTIFACT_PACK = "local://🎒️.pack.semio"


def hex_of_text(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction, for a string leaf."""
    return text.encode("utf-8").hex()


def text_of_hex(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction, for a string leaf."""
    return bytes.fromhex(hexed).decode("utf-8")


def to_f32(value: float) -> float:
    """🔢️ The nearest single-precision value. `SemioRgba`'s four channels and a style's `opacity` are
    `f32`, so a document that held the widened double would not survive its own pack frame."""
    return struct.unpack("<f", struct.pack("<f", float(value)))[0]


def print_f32(value: float) -> str:
    """🔢️ One single-precision leaf, printed the way the reference printer prints it: the shortest
    decimal that reads back as the same `f32`, which is NOT the shortest decimal that reads back as
    the same double — `0.35` against `0.3499999940395355`."""
    for precision in range(1, 10):
        candidate = float("%.*g" % (precision, value))
        if to_f32(candidate) == value:
            return print_number(candidate)
    return print_number(value)


def print_number(value: float) -> str:
    """🔢️ One numeric leaf, printed the way the reference printer prints it: plain `{}` `Display`,
    which is the shortest decimal that reads back as the same value, with no exponent and no
    trailing `.0`."""
    if value == int(value) and abs(value) < 1e16:
        return str(int(value))
    text = repr(float(value))
    if "e" not in text and "E" not in text:
        return text
    return format(float(value), ".17f").rstrip("0").rstrip(".")


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


def parse_bool(text: str, what: str) -> bool:
    """📜️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("%s is not a well-formed bool: %r" % (what, text[:20]))
    return text == "1"


def print_bool(value: bool) -> str:
    """📜️ `bool` in the writing direction."""
    return "1" if value else "0"


def parse_point2(text: str) -> dict:
    """📐️ `point2 = "[" number "," number "]"`."""
    parts = items_of(text, "a point2")
    return {"x": float(parts[0]), "y": float(parts[1])}


def print_point2(point: dict) -> str:
    """📐️ `point2` in the writing direction."""
    return "[%s,%s]" % (print_number(point["x"]), print_number(point["y"]))


def parse_point3(text: str) -> dict:
    """📐️ `point3 = "[" number "," number "," number "]"`."""
    parts = items_of(text, "a point3")
    return {"x": float(parts[0]), "y": float(parts[1]), "z": float(parts[2])}


def print_point3(point: dict) -> str:
    """📐️ `point3` in the writing direction."""
    return "[%s,%s,%s]" % (print_number(point["x"]), print_number(point["y"]), print_number(point["z"]))


def parse_quaternion(text: str) -> dict:
    """📐️ `quaternion = "[" number "," number "," number "," number "]"`."""
    parts = items_of(text, "a quaternion")
    return {"x": float(parts[0]), "y": float(parts[1]), "z": float(parts[2]), "w": float(parts[3])}


def print_quaternion(value: dict) -> str:
    """📐️ `quaternion` in the writing direction."""
    return "[%s,%s,%s,%s]" % (print_number(value["x"]), print_number(value["y"]), print_number(value["z"]), print_number(value["w"]))


def parse_rgba(text: str) -> dict:
    """🎨️ `rgba = "[" number "," number "," number "," number "]"`, every channel single precision."""
    parts = items_of(text, "an rgba")
    return {"r": to_f32(parts[0]), "g": to_f32(parts[1]), "b": to_f32(parts[2]), "a": to_f32(parts[3])}


def print_rgba(colour: dict) -> str:
    """🎨️ `rgba` in the writing direction."""
    return "[%s,%s,%s,%s]" % (print_f32(colour["r"]), print_f32(colour["g"]), print_f32(colour["b"]), print_f32(colour["a"]))


def parse_option(text: str, what: str, read):
    """📜️ The three `option-` productions share one shape: `[0]`, or `[1,<payload>]`."""
    parts = items_of(text, what)
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return read(parts[1])
    raise AssertionError("%s is not a well-formed option: %r" % (what, text[:60]))


def print_option(value, write) -> str:
    """📜️ An `option-` production in the writing direction."""
    return "[0]" if value is None else "[1,%s]" % write(value)


def parse_transform(text: str) -> dict:
    """📐️ `transform = "[" point3 "," quaternion "," point3 "]"`."""
    parts = items_of(text, "a transform")
    return {"translation": parse_point3(parts[0]), "rotation": parse_quaternion(parts[1]), "scale-node": parse_point3(parts[2])}


def print_transform(transform: dict) -> str:
    """📐️ `transform` in the writing direction."""
    return "[%s,%s,%s]" % (print_point3(transform["translation"]), print_quaternion(transform["rotation"]), print_point3(transform["scale-node"]))


# endregion 🔖️Text primitives


# region 🔖️Nodes
def parse_segment(text: str) -> dict:
    """✏️ One `segment` alternative, tag letter first."""
    letter = text[0]
    if letter not in LETTER_SEGMENT:
        raise AssertionError("%r is not one of the six segment tags" % letter)
    kind = LETTER_SEGMENT[letter]
    if kind == "close":
        if len(text) != 1:
            raise AssertionError("a close segment carries no payload, found %r" % text[:40])
        return {"kind": "close"}
    parts = items_of(text[1:], "a %s segment" % kind)
    if kind in ("moveTo", "lineTo"):
        return {"kind": kind, "to": parse_point2(parts[0])}
    if kind == "cubicTo":
        return {"kind": "cubicTo", "c1": parse_point2(parts[0]), "c2": parse_point2(parts[1]), "to": parse_point2(parts[2])}
    if kind == "quadTo":
        return {"kind": "quadTo", "c": parse_point2(parts[0]), "to": parse_point2(parts[1])}
    return {
        "kind": "arcTo",
        "rx": float(parts[0]),
        "ry": float(parts[1]),
        "x_rotation": float(parts[2]),
        "large_arc": parse_bool(parts[3], "largeArc"),
        "sweep": parse_bool(parts[4], "sweep"),
        "to": parse_point2(parts[5]),
    }


def print_segment(segment: dict) -> str:
    """✏️ One `segment` in the writing direction."""
    kind = segment["kind"]
    if kind == "close":
        return "Z"
    if kind in ("moveTo", "lineTo"):
        return "%s[%s]" % (SEGMENT_LETTER[kind], print_point2(segment["to"]))
    if kind == "cubicTo":
        return "C[%s,%s,%s]" % (print_point2(segment["c1"]), print_point2(segment["c2"]), print_point2(segment["to"]))
    if kind == "quadTo":
        return "Q[%s,%s]" % (print_point2(segment["c"]), print_point2(segment["to"]))
    if kind == "arcTo":
        return "A[%s,%s,%s,%s,%s,%s]" % (
            print_number(segment["rx"]),
            print_number(segment["ry"]),
            print_number(segment["x_rotation"]),
            print_bool(segment["large_arc"]),
            print_bool(segment["sweep"]),
            print_point2(segment["to"]),
        )
    raise AssertionError("%r is not one of the six segment kinds" % kind)


def parse_node(text: str) -> dict:
    """🖍️ One `node` alternative, tag letter first — `G`'s `children` is genuinely recursive."""
    letter = text[0]
    if letter not in LETTER_NODE:
        raise AssertionError("%r is not one of the four node tags" % letter)
    kind = LETTER_NODE[letter]
    parts = items_of(text[1:], "a %s node" % kind)
    if kind == "path":
        node = {"kind": "path", "segments": [parse_segment(entry) for entry in items_of(parts[0], "segments")]}
        style = parse_option(parts[1], "style", text_of_hex)
        if style is not None:
            node["style"] = style
        return node
    if kind == "text":
        node = {"kind": "text", "value": text_of_hex(parts[0]), "at": parse_point2(parts[1])}
        style = parse_option(parts[2], "style", text_of_hex)
        if style is not None:
            node["style"] = style
        return node
    if kind == "group-nodes":
        return {"kind": "group-nodes", "transform": parse_transform(parts[0]), "children": [parse_node(entry) for entry in items_of(parts[1], "children")]}
    return {"kind": "image", "at": parse_point2(parts[0]), "width": float(parts[1]), "height": float(parts[2]), "mime": text_of_hex(parts[3]), "bytes": list(bytes.fromhex(parts[4]))}


def print_node(node: dict) -> str:
    """🖍️ One `node` in the writing direction."""
    kind = node["kind"]
    if kind == "path":
        return "P[[%s],%s]" % (",".join(print_segment(segment) for segment in node["segments"]), print_option(node.get("style"), hex_of_text))
    if kind == "text":
        return "T[%s,%s,%s]" % (hex_of_text(node["value"]), print_point2(node["at"]), print_option(node.get("style"), hex_of_text))
    if kind == "group-nodes":
        return "G[%s,[%s]]" % (print_transform(node["transform"]), ",".join(print_node(child) for child in node.get("children", [])))
    if kind == "image":
        return "I[%s,%s,%s,%s,%s]" % (print_point2(node["at"]), print_number(node["width"]), print_number(node["height"]), hex_of_text(node["mime"]), bytes(node["bytes"]).hex())
    raise AssertionError("%r is not one of the four node kinds" % kind)


# endregion 🔖️Nodes


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


def parse_style(text: str) -> dict:
    """🎨️ `style = "[" hex "," option-rgba "," option-rgba "," option-number "," option-number "]"`."""
    parts = items_of(text, "a style")
    style = {"name": text_of_hex(parts[0])}
    fill = parse_option(parts[1], "fill", parse_rgba)
    stroke = parse_option(parts[2], "stroke", parse_rgba)
    width = parse_option(parts[3], "strokeWidth", float)
    opacity = parse_option(parts[4], "opacity", to_f32)
    if fill is not None:
        style["fill"] = fill
    if stroke is not None:
        style["stroke"] = stroke
    if width is not None:
        style["strokeWidth"] = width
    if opacity is not None:
        style["opacity"] = opacity
    return style


def print_style(style: dict) -> str:
    """🎨️ `style` in the writing direction."""
    return "[%s,%s,%s,%s,%s]" % (
        hex_of_text(style["name"]),
        print_option(style.get("fill"), print_rgba),
        print_option(style.get("stroke"), print_rgba),
        print_option(style.get("strokeWidth"), print_number),
        print_option(style.get("opacity"), print_f32),
    )


def parse_layer(text: str) -> dict:
    """🗂️ `layer = "[" hex "," hex "," bool "," node "]"`."""
    parts = items_of(text, "a layer")
    return {"id": text_of_hex(parts[0]), "name": text_of_hex(parts[1]), "visible": parse_bool(parts[2], "visible"), "root": parse_node(parts[3])}


def print_layer(layer: dict) -> str:
    """🗂️ `layer` in the writing direction."""
    return "[%s,%s,%s,%s]" % (hex_of_text(layer["id"]), hex_of_text(layer["name"]), print_bool(layer["visible"]), print_node(layer["root"]))


def parse_dsl(text: str) -> dict:
    """📜️ `document = artifact-mark schema-line canvas-line styles-line layers-line`."""
    body = strip_preamble(text)
    schema_hex, body = read_field(body, "schema")
    canvas, body = read_field(body, "canvas")
    styles, body = read_field(body, "styles")
    layers, body = read_field(body, "layers")
    if body != "":
        raise AssertionError("the document carries trailing content after its layers line: %r" % body[:60])
    canvas_parts = items_of(canvas, "the canvas")
    document = {"schema": text_of_hex(schema_hex), "canvas": {"width": float(canvas_parts[0]), "height": float(canvas_parts[1])}}
    background = parse_option(canvas_parts[2], "background", parse_rgba)
    if background is not None:
        document["canvas"]["background"] = background
    document["styles"] = [parse_style(entry) for entry in items_of(styles, "styles")]
    document["layers"] = [parse_layer(entry) for entry in items_of(layers, "layers")]
    return document


def print_dsl(document: dict) -> str:
    """📜️ The committed DSL grammar in the writing direction, line for line in its declared order."""
    canvas = document["canvas"]
    return "\n".join(
        [
            DSL_PREAMBLE,
            "schema=%s" % hex_of_text(document["schema"]),
            "canvas=[%s,%s,%s]" % (print_number(canvas["width"]), print_number(canvas["height"]), print_option(canvas.get("background"), print_rgba)),
            "styles=[%s]" % ",".join(print_style(style) for style in document["styles"]),
            "layers=[%s]" % ",".join(print_layer(layer) for layer in document["layers"]),
        ]
    )


# endregion 🔖️Dsl


# region 🔖️Pack
class Reader:
    """🎒️ A forward cursor over the pack frame."""

    def __init__(self, data: bytes, at: int) -> None:
        self.data = data
        self.at = at

    def byte(self) -> int:
        value = self.data[self.at]
        self.at += 1
        return value

    def varint(self) -> int:
        value = 0
        shift = 0
        while True:
            byte = self.byte()
            value |= (byte & 0x7F) << shift
            if byte & 0x80 == 0:
                return value
            shift += 7

    def take(self, length: int) -> bytes:
        payload = self.data[self.at : self.at + length]
        if len(payload) != length:
            raise AssertionError("the pack frame ends inside a length-prefixed run")
        self.at += length
        return payload

    def blob(self) -> bytes:
        return self.take(self.varint())

    def text(self) -> str:
        return self.blob().decode("utf-8")

    def f64(self) -> float:
        return struct.unpack("<d", self.take(8))[0]

    def f32(self) -> float:
        return struct.unpack("<f", self.take(4))[0]

    def flag(self) -> bool:
        value = self.byte()
        if value not in (0, 1):
            raise AssertionError("a presence or boolean byte is %d, expected 0 or 1" % value)
        return value == 1

    def point2(self) -> dict:
        return {"x": self.f64(), "y": self.f64()}

    def point3(self) -> dict:
        return {"x": self.f64(), "y": self.f64(), "z": self.f64()}

    def quaternion(self) -> dict:
        return {"x": self.f64(), "y": self.f64(), "z": self.f64(), "w": self.f64()}

    def rgba(self) -> dict:
        return {"r": self.f32(), "g": self.f32(), "b": self.f32(), "a": self.f32()}

    def transform(self) -> dict:
        return {"translation": self.point3(), "rotation": self.quaternion(), "scale-node": self.point3()}

    def option(self, read):
        return read() if self.flag() else None


class Writer:
    """🎒️ A growing pack frame."""

    def __init__(self) -> None:
        self.out = bytearray()

    def byte(self, value: int) -> None:
        self.out.append(value & 0xFF)

    def varint(self, value: int) -> None:
        while True:
            piece = value & 0x7F
            value >>= 7
            if value:
                self.byte(piece | 0x80)
                continue
            self.byte(piece)
            return

    def raw(self, payload: bytes) -> None:
        self.out += payload

    def blob(self, payload: bytes) -> None:
        self.varint(len(payload))
        self.raw(payload)

    def text(self, value: str) -> None:
        self.blob(value.encode("utf-8"))

    def f64(self, value: float) -> None:
        self.raw(struct.pack("<d", float(value)))

    def f32(self, value: float) -> None:
        self.raw(struct.pack("<f", float(value)))

    def point2(self, point: dict) -> None:
        self.f64(point["x"])
        self.f64(point["y"])

    def point3(self, point: dict) -> None:
        self.f64(point["x"])
        self.f64(point["y"])
        self.f64(point["z"])

    def quaternion(self, value: dict) -> None:
        self.f64(value["x"])
        self.f64(value["y"])
        self.f64(value["z"])
        self.f64(value["w"])

    def rgba(self, colour: dict) -> None:
        self.f32(colour["r"])
        self.f32(colour["g"])
        self.f32(colour["b"])
        self.f32(colour["a"])

    def transform(self, transform: dict) -> None:
        self.point3(transform["translation"])
        self.quaternion(transform["rotation"])
        self.point3(transform["scale-node"])

    def option(self, value, write) -> None:
        if value is None:
            self.byte(0)
            return
        self.byte(1)
        write(value)


def read_segment(reader: Reader) -> dict:
    """✏️ One tag byte, then the segment variant's own payload."""
    tag = reader.byte()
    if tag >= len(SEGMENT_ORDER):
        raise AssertionError("the pack segment tag %d is outside the declared union" % tag)
    kind = SEGMENT_ORDER[tag]
    if kind in ("moveTo", "lineTo"):
        return {"kind": kind, "to": reader.point2()}
    if kind == "cubicTo":
        return {"kind": "cubicTo", "c1": reader.point2(), "c2": reader.point2(), "to": reader.point2()}
    if kind == "quadTo":
        return {"kind": "quadTo", "c": reader.point2(), "to": reader.point2()}
    if kind == "arcTo":
        return {"kind": "arcTo", "rx": reader.f64(), "ry": reader.f64(), "x_rotation": reader.f64(), "large_arc": reader.flag(), "sweep": reader.flag(), "to": reader.point2()}
    return {"kind": "close"}


def write_segment(writer: Writer, segment: dict) -> None:
    """✏️ One segment in the writing direction."""
    kind = segment["kind"]
    writer.byte(SEGMENT_ORDER.index(kind))
    if kind in ("moveTo", "lineTo"):
        writer.point2(segment["to"])
        return
    if kind == "cubicTo":
        writer.point2(segment["c1"])
        writer.point2(segment["c2"])
        writer.point2(segment["to"])
        return
    if kind == "quadTo":
        writer.point2(segment["c"])
        writer.point2(segment["to"])
        return
    if kind == "arcTo":
        writer.f64(segment["rx"])
        writer.f64(segment["ry"])
        writer.f64(segment["x_rotation"])
        writer.byte(1 if segment["large_arc"] else 0)
        writer.byte(1 if segment["sweep"] else 0)
        writer.point2(segment["to"])


def read_node(reader: Reader) -> dict:
    """🖍️ One tag byte, then the node variant's own payload — `group` recurses."""
    tag = reader.byte()
    if tag >= len(NODE_ORDER):
        raise AssertionError("the pack node tag %d is outside the declared union" % tag)
    kind = NODE_ORDER[tag]
    if kind == "path":
        segments = [read_segment(reader) for _ in range(reader.varint())]
        node = {"kind": "path", "segments": segments}
        style = reader.option(reader.text)
        if style is not None:
            node["style"] = style
        return node
    if kind == "text":
        node = {"kind": "text", "value": reader.text(), "at": reader.point2()}
        style = reader.option(reader.text)
        if style is not None:
            node["style"] = style
        return node
    if kind == "group-nodes":
        transform = reader.transform()
        return {"kind": "group-nodes", "transform": transform, "children": [read_node(reader) for _ in range(reader.varint())]}
    at = reader.point2()
    return {"kind": "image", "at": at, "width": reader.f64(), "height": reader.f64(), "mime": reader.text(), "bytes": list(reader.blob())}


def write_node(writer: Writer, node: dict) -> None:
    """🖍️ One node in the writing direction."""
    kind = node["kind"]
    writer.byte(NODE_ORDER.index(kind))
    if kind == "path":
        writer.varint(len(node["segments"]))
        for segment in node["segments"]:
            write_segment(writer, segment)
        writer.option(node.get("style"), writer.text)
        return
    if kind == "text":
        writer.text(node["value"])
        writer.point2(node["at"])
        writer.option(node.get("style"), writer.text)
        return
    if kind == "group-nodes":
        writer.transform(node["transform"])
        children = node.get("children", [])
        writer.varint(len(children))
        for child in children:
            write_node(writer, child)
        return
    writer.point2(node["at"])
    writer.f64(node["width"])
    writer.f64(node["height"])
    writer.text(node["mime"])
    writer.blob(bytes(node["bytes"]))


def parse_pack(data: bytes) -> dict:
    """🎒️ The committed binary envelope and the pack frame it wraps."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the binary envelope magic is %r, expected %r" % (data[:8], BINARY_MAGIC))
    token_length = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_length].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("the binary envelope token is %r, expected %r" % (token, PACK_TOKEN))
    reader = Reader(data, 12 + token_length)
    if reader.byte() != PACK_FORMAT:
        raise AssertionError("the pack format byte is not %d" % PACK_FORMAT)
    document = {"schema": reader.text(), "canvas": {"width": reader.f64(), "height": reader.f64()}}
    background = reader.option(reader.rgba)
    if background is not None:
        document["canvas"]["background"] = background
    styles = []
    for _ in range(reader.varint()):
        style = {"name": reader.text()}
        fill = reader.option(reader.rgba)
        stroke = reader.option(reader.rgba)
        width = reader.option(reader.f64)
        opacity = reader.option(reader.f32)
        if fill is not None:
            style["fill"] = fill
        if stroke is not None:
            style["stroke"] = stroke
        if width is not None:
            style["strokeWidth"] = width
        if opacity is not None:
            style["opacity"] = opacity
        styles.append(style)
    document["styles"] = styles
    document["layers"] = [{"id": reader.text(), "name": reader.text(), "visible": reader.flag(), "root": read_node(reader)} for _ in range(reader.varint())]
    if reader.at != len(data):
        raise AssertionError("the pack frame ends %d bytes before its envelope does" % (len(data) - reader.at))
    return document


def pack_bytes(document: dict) -> bytes:
    """🎒️ The pack frame in the writing direction, inside the shared binary envelope."""
    writer = Writer()
    writer.byte(PACK_FORMAT)
    writer.text(document["schema"])
    canvas = document["canvas"]
    writer.f64(canvas["width"])
    writer.f64(canvas["height"])
    writer.option(canvas.get("background"), writer.rgba)
    writer.varint(len(document["styles"]))
    for style in document["styles"]:
        writer.text(style["name"])
        writer.option(style.get("fill"), writer.rgba)
        writer.option(style.get("stroke"), writer.rgba)
        writer.option(style.get("strokeWidth"), writer.f64)
        writer.option(style.get("opacity"), writer.f32)
    writer.varint(len(document["layers"]))
    for layer in document["layers"]:
        writer.text(layer["id"])
        writer.text(layer["name"])
        writer.byte(1 if layer["visible"] else 0)
        write_node(writer, layer["root"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(writer.out)


# endregion 🔖️Pack


# region 🔖️Mutations
VERBS = {
    "CreateLayer": "create-layer",
    "DeleteLayer": "delete-layer",
    "CreateNode": "create-node",
    "DeleteNode": "delete-node",
    "MoveNode": "move-node",
    "DragNodes": "drag-nodes",
    "Rotate": "rotate-node",
    "Scale": "scale-node",
    "ReorderNodes": "reorder-nodes",
    "Group": "group-nodes",
    "Ungroup": "ungroup-node",
    "Flatten": "flatten-node",
    "Unflatten": "unflatten-node",
    "ReplacePath": "replace-path",
    "ReplaceFill": "replace-fill",
    "ChangeStrokeColor": "change-stroke-color",
    "ChangeStrokeWidth": "change-stroke-width",
}


def clone(value):
    """🧬️ A deep copy, so no arm ever aliases the document it was handed."""
    return json.loads(json.dumps(value))


def single(colour):
    """🎨️ A wire colour narrowed to the single precision the model actually holds."""
    return None if colour is None else {key: to_f32(colour[key]) for key in ("r", "g", "b", "a")}


def verb_of(mutation: dict) -> tuple:
    """🏷️ The externally tagged wire form: one key, the variant's own `PascalCase` name, whose value
    carries the verb's `snake_case` arguments."""
    keys = list(mutation.keys())
    if len(keys) != 1 or keys[0] not in VERBS:
        raise AssertionError("%r is not one of this subset's seventeen declared verbs" % (keys,))
    return keys[0], mutation[keys[0]]


def parent_of(document: dict, at: dict, verb: str) -> list:
    """🌳️ The child list a `NodePath` addresses INTO — `layer` selects the layer, `path` is a chain of
    child indices from its root, and the last index is the position inside the list this returns."""
    layer = at["layer"]
    if layer < 0 or layer >= len(document["layers"]):
        raise AssertionError("%s addresses layer %d of a %d-layer drawing" % (verb, layer, len(document["layers"])))
    node = document["layers"][layer]["root"]
    for index in at["path"]:
        if node["kind"] != "group-nodes":
            raise AssertionError("%s walks into a %s node, which carries no children" % (verb, node["kind"]))
        children = node.get("children", [])
        if index < 0 or index >= len(children):
            raise AssertionError("%s addresses child %d of a %d-child group" % (verb, index, len(children)))
        node = children[index]
    return node


def node_at(document: dict, at: dict, verb: str) -> dict:
    """🌳️ The node a `NodePath` addresses. An empty `path` is the layer's own root."""
    return parent_of(document, at, verb)


def container_of(document: dict, at: dict, verb: str) -> tuple:
    """🌳️ The child list holding the addressed node, and its index inside it."""
    if not at["path"]:
        raise AssertionError("%s addresses a layer root, which has no containing list" % verb)
    owner = node_at(document, {"layer": at["layer"], "path": at["path"][:-1]}, verb)
    if owner["kind"] != "group-nodes":
        raise AssertionError("%s addresses a child of a %s node, which carries no children" % (verb, owner["kind"]))
    return owner.setdefault("children", []), at["path"][-1]


IDENTITY = {"translation": {"x": 0.0, "y": 0.0, "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale-node": {"x": 1.0, "y": 1.0, "z": 1.0}}


def leaves_of(node: dict) -> list:
    """🫓 Every non-group descendant of a node, in depth-first order — what `flatten` keeps."""
    if node["kind"] != "group-nodes":
        return [clone(node)]
    found = []
    for child in node.get("children", []):
        found.extend(leaves_of(child))
    return found


def transformed_descendant_group(node: dict) -> bool:
    """🫓 Whether any DESCENDANT group carries a transform other than the identity. Dissolving such a
    group would silently drop the transform its children are drawn under, so `flatten` refuses the
    whole mutation when one is present — the specification vector's own slug says as much
    (`flattens-an-IDENTITY-nested-group-into-its-leaves`), and the addressed node's own transform is
    not at issue because `flatten` keeps the node itself."""
    for child in node.get("children", []):
        if child["kind"] != "group-nodes":
            continue
        if child["transform"] != IDENTITY or transformed_descendant_group(child):
            return True
    return False


def style_of(document: dict, name: str, verb: str) -> dict:
    """🎨️ The named style a style verb addresses."""
    for style in document["styles"]:
        if style["name"] == name:
            return style
    raise AssertionError("%s addresses the style %r, which the drawing does not carry" % (verb, name))


def set_optional(holder: dict, key: str, value) -> None:
    """🎨️ Writes an optional leaf the way the wire form spells it: absent rather than null."""
    if value is None:
        holder.pop(key, None)
        return
    holder[key] = value


def apply_mutation(document: dict, mutation: dict) -> dict:
    """▶️ One verb applied to a drawing, returning the resulting drawing.

    Each arm is the behaviour its committed `(before, mutation, after)` specification vector states:
    layers are an ordered list with an id-keyed removal, nodes are addressed by a structural
    `NodePath`, and the four hierarchy verbs are the ones the vectors pin most sharply — `group`
    replaces the addressed children with one new group AT THE FIRST INDEX, `ungroup` splices a
    group's children back into its parent in place, `flatten` replaces a group's children with all
    its LEAF descendants in depth-first order, and `unflatten` puts a captured node back.
    """
    verb, argument = verb_of(mutation)
    kind = VERBS[verb]
    result = clone(document)
    if kind == "create-layer":
        index = argument["index"]
        if index < 0 or index > len(result["layers"]):
            raise AssertionError("create-layer addresses position %d of a %d-layer drawing" % (index, len(result["layers"])))
        result["layers"].insert(index, clone(argument["layer"]))
        return result
    if kind == "delete-layer":
        if not any(layer["id"] == argument["id"] for layer in result["layers"]):
            raise AssertionError("delete-layer addresses %r, which the drawing does not carry" % argument["id"])
        result["layers"] = [layer for layer in result["layers"] if layer["id"] != argument["id"]]
        return result
    if kind == "create-node":
        parent = node_at(result, argument["parent"], "create-node")
        if parent["kind"] != "group-nodes":
            raise AssertionError("create-node addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        index = argument["index"]
        if index < 0 or index > len(children):
            raise AssertionError("create-node addresses position %d of a %d-child group" % (index, len(children)))
        children.insert(index, clone(argument["node"]))
        return result
    if kind == "delete-node":
        children, index = container_of(result, argument["at"], "delete-node")
        children.pop(index)
        return result
    if kind == "move-node":
        node = node_at(result, argument["at"], "move-node")
        origin = argument["new_origin"]
        if node["kind"] in ("text", "image"):
            node["at"] = clone(origin)
            return result
        if node["kind"] == "group-nodes":
            node["transform"]["translation"]["x"] = origin["x"]
            node["transform"]["translation"]["y"] = origin["y"]
            return result
        raise AssertionError("move-node addresses a %s node, which carries no origin" % node["kind"])
    if kind == "drag-nodes":
        offset = argument["offset"]
        for at in argument["ats"]:
            node = node_at(result, at, "drag-nodes")
            if node["kind"] in ("text", "image"):
                node["at"]["x"] += offset["x"]
                node["at"]["y"] += offset["y"]
                continue
            if node["kind"] == "group-nodes":
                node["transform"]["translation"]["x"] += offset["x"]
                node["transform"]["translation"]["y"] += offset["y"]
                continue
            raise AssertionError("drag-nodes addresses a %s node, which carries no origin" % node["kind"])
        return result
    if kind == "rotate-node":
        node = node_at(result, argument["at"], "rotate-node")
        if node["kind"] != "group-nodes":
            raise AssertionError("rotate addresses a %s node, which carries no transform" % node["kind"])
        node["transform"]["rotation"] = clone(argument["new_rotation"])
        return result
    if kind == "scale-node":
        node = node_at(result, argument["at"], "scale-node")
        if node["kind"] != "group-nodes":
            raise AssertionError("scale addresses a %s node, which carries no transform" % node["kind"])
        node["transform"]["scale-node"] = clone(argument["new_scale"])
        return result
    if kind == "reorder-nodes":
        parent = node_at(result, argument["parent"], "reorder-nodes")
        if parent["kind"] != "group-nodes":
            raise AssertionError("reorder-nodes addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        source = argument["from"]
        target = argument["to"]
        for index in (source, target):
            if index < 0 or index >= len(children):
                raise AssertionError("reorder-nodes addresses child %d of a %d-child group" % (index, len(children)))
        children.insert(target, children.pop(source))
        return result
    if kind == "group-nodes":
        parent = node_at(result, argument["parent"], "group-nodes")
        if parent["kind"] != "group-nodes":
            raise AssertionError("group addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        indices = sorted(argument["indices"])
        for index in indices:
            if index < 0 or index >= len(children):
                raise AssertionError("group addresses child %d of a %d-child group" % (index, len(children)))
        taken = [clone(children[index]) for index in argument["indices"]]
        for index in reversed(indices):
            children.pop(index)
        children.insert(indices[0], {"kind": "group-nodes", "transform": clone(argument["transform"]), "children": taken})
        return result
    if kind == "ungroup-node":
        children, index = container_of(result, argument["at"], "ungroup-node")
        node = children[index]
        if node["kind"] != "group-nodes":
            raise AssertionError("ungroup addresses a %s node, which is not a group" % node["kind"])
        children[index : index + 1] = clone(node.get("children", []))
        return result
    if kind == "flatten-node":
        node = node_at(result, argument["at"], "flatten-node")
        if node["kind"] != "group-nodes":
            raise AssertionError("flatten addresses a %s node, which is not a group" % node["kind"])
        if transformed_descendant_group(node):
            return result
        node["children"] = leaves_of(node)
        return result
    if kind == "unflatten-node":
        at = argument["at"]
        if not at["path"]:
            result["layers"][at["layer"]]["root"] = clone(argument["original"])
            return result
        children, index = container_of(result, at, "unflatten-node")
        children[index] = clone(argument["original"])
        return result
    if kind == "replace-path":
        node = node_at(result, argument["at"], "replace-path")
        if node["kind"] != "path":
            raise AssertionError("replace-path addresses a %s node, which carries no segments" % node["kind"])
        node["segments"] = clone(argument["new_segments"])
        return result
    if kind == "replace-fill":
        set_optional(style_of(result, argument["style_name"], "replace-fill"), "fill", single(argument["new_fill"]))
        return result
    if kind == "change-stroke-color":
        set_optional(style_of(result, argument["style_name"], "change-stroke-color"), "stroke", single(argument["new_color"]))
        return result
    set_optional(style_of(result, argument["style_name"], "change-stroke-width"), "strokeWidth", argument["new_width"])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The verb's own inverse against the drawing it is about to be applied to, as the ordered
    sequence of verbs that restores it. Every hierarchy verb's inverse captures the node it is about
    to disturb, so the restoration is exact rather than merely structural."""
    verb, argument = verb_of(mutation)
    kind = VERBS[verb]
    if kind == "create-layer":
        return [{"DeleteLayer": {"id": argument["layer"]["id"]}}]
    if kind == "delete-layer":
        index = next(at for at, layer in enumerate(document["layers"]) if layer["id"] == argument["id"])
        return [{"CreateLayer": {"index": index, "layer": clone(document["layers"][index])}}]
    if kind == "create-node":
        return [{"DeleteNode": {"at": {"layer": argument["parent"]["layer"], "path": list(argument["parent"]["path"]) + [argument["index"]]}}}]
    if kind == "delete-node":
        at = argument["at"]
        node = node_at(document, at, "delete-node")
        return [{"CreateNode": {"parent": {"layer": at["layer"], "path": list(at["path"])[:-1]}, "index": at["path"][-1], "node": clone(node)}}]
    if kind == "move-node":
        node = node_at(document, argument["at"], "move-node")
        origin = node["at"] if node["kind"] in ("text", "image") else {"x": node["transform"]["translation"]["x"], "y": node["transform"]["translation"]["y"]}
        return [{"MoveNode": {"at": clone(argument["at"]), "new_origin": clone(origin)}}]
    if kind == "drag-nodes":
        offset = argument["offset"]
        return [{"DragNodes": {"ats": clone(argument["ats"]), "offset": {"x": -offset["x"], "y": -offset["y"]}}}]
    if kind == "rotate-node":
        return [{"Rotate": {"at": clone(argument["at"]), "new_rotation": clone(node_at(document, argument["at"], "rotate-node")["transform"]["rotation"])}}]
    if kind == "scale-node":
        return [{"Scale": {"at": clone(argument["at"]), "new_scale": clone(node_at(document, argument["at"], "scale-node")["transform"]["scale-node"])}}]
    if kind == "reorder-nodes":
        return [{"ReorderNodes": {"parent": clone(argument["parent"]), "from": argument["to"], "to": argument["from"]}}]
    if kind == "group-nodes":
        indices = sorted(argument["indices"])
        return [{"Ungroup": {"at": {"layer": argument["parent"]["layer"], "path": list(argument["parent"]["path"]) + [indices[0]]}}}]
    if kind == "ungroup-node":
        at = argument["at"]
        node = node_at(document, at, "ungroup-node")
        count = len(node.get("children", []))
        first = at["path"][-1]
        return [
            {
                "Group": {
                    "parent": {"layer": at["layer"], "path": list(at["path"])[:-1]},
                    "indices": list(range(first, first + count)),
                    "transform": clone(node["transform"]),
                }
            }
        ]
    if kind in ("flatten-node", "unflatten-node"):
        return [{"Unflatten": {"at": clone(argument["at"]), "original": clone(node_at(document, argument["at"], kind))}}]
    if kind == "replace-path":
        return [{"ReplacePath": {"at": clone(argument["at"]), "new_segments": clone(node_at(document, argument["at"], "replace-path")["segments"])}}]
    if kind == "replace-fill":
        return [{"ReplaceFill": {"style_name": argument["style_name"], "new_fill": clone(style_of(document, argument["style_name"], "replace-fill").get("fill"))}}]
    if kind == "change-stroke-color":
        return [{"ChangeStrokeColor": {"style_name": argument["style_name"], "new_color": clone(style_of(document, argument["style_name"], "change-stroke-color").get("stroke"))}}]
    return [{"ChangeStrokeWidth": {"style_name": argument["style_name"], "new_width": style_of(document, argument["style_name"], "change-stroke-width").get("strokeWidth")}}]


# endregion 🔖️Mutations


# region 🔖️Scenario input
def step_uris(ctx: Context, scheme: str) -> list:
    """🧫️ Every fixture URI of one scheme the scenario's steps name, in step order."""
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


def artifact(ctx: Context) -> dict:
    """🖍️ The real derived drawing, parsed through this implementation's own DSL reader."""
    return parse_dsl(ctx.fixture_bytes(ARTIFACT_DSL).decode("utf-8"))


def projection_of(document: dict) -> dict:
    """🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own
    structural JSON shape.

    `SemioRgba`'s four channels and a style's `opacity` are SINGLE precision, and the reference's JSON
    wire form spells such a leaf with the shortest decimal that round-trips as an `f32` (`0.35`), not
    with the widened double a Python float would print (`0.3499999940395355`). Every single-precision
    leaf therefore goes out through the same shortest-`f32` printer the DSL writer uses and back, so
    the two languages compare the same number rather than the same bit pattern printed two ways.
    """
    result = clone(document)
    narrow = lambda colour: {key: float(print_f32(colour[key])) for key in ("r", "g", "b", "a")}
    if "background" in result["canvas"]:
        result["canvas"]["background"] = narrow(result["canvas"]["background"])
    for style in result["styles"]:
        for key in ("fill", "stroke"):
            if key in style:
                style[key] = narrow(style[key])
        if "opacity" in style:
            style["opacity"] = float(print_f32(style["opacity"]))
    return result


def shape_report(document: dict) -> dict:
    """🌳️ A structural census of the scene graph, so a mutation that lands in the wrong branch shows
    up as a shape difference and not only as a deep value difference: per layer, the node kind
    histogram, the maximum depth and the total segment count."""
    census = []
    for layer in document["layers"]:
        counts = {"path": 0, "text": 0, "group-nodes": 0, "image": 0}
        segments = [0]
        depth = [0]

        def walk(node: dict, level: int) -> None:
            counts[node["kind"]] += 1
            depth[0] = max(depth[0], level)
            if node["kind"] == "path":
                segments[0] += len(node["segments"])
            for child in node.get("children", []):
                walk(child, level + 1)

        walk(layer["root"], 0)
        census.append({"id": layer["id"], "visible": layer["visible"], "nodes": counts, "depth": depth[0], "segments": segments[0]})
    return {"layers": census, "styles": [style["name"] for style in document["styles"]]}


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real derived drawing by this implementation alone."""
    document = artifact(ctx)
    mutation = fixture_json(ctx, step_uris(ctx, "local://🦠️")[0])
    applied = apply_mutation(document, mutation)
    return Outcome({"document": projection_of(applied), "shape": shape_report(applied)})


def inverse(ctx: Context) -> Outcome:
    """↩️ The metamorphic inverse law on the real drawing: the verb followed by its OWN computed
    inverse must restore it exactly, scene-graph ORDER and nesting included."""
    document = artifact(ctx)
    mutation = fixture_json(ctx, step_uris(ctx, "local://🦠️")[0])
    undo = inverse_mutation(document, mutation)
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in undo:
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("%s: undoing the mutation did not restore the drawing" % ctx.scenario["id"])
    return Outcome({"mutated": projection_of(mutated), "restored": projection_of(restored)})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    statement of what the verb means, independent of both implementations."""
    uris = step_uris(ctx, "asset://")
    before = fixture_json(ctx, uris[0])
    mutation = fixture_json(ctx, uris[1])
    expected = fixture_json(ctx, uris[2])
    applied = apply_mutation(before, mutation)
    if applied != expected:
        raise AssertionError("%s: the applied drawing is not the committed after-snapshot" % ctx.scenario["id"])
    return Outcome(projection_of(applied))


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real derived drawing, each re-emitted from the parsed
    document.

    🔒️ The BYTE half of the identity law. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` its binary twin, so reproducing both files byte for byte is the CORRECT answer here
    and a must-differ tripwire would be exactly backwards. What stops that from being a codec
    agreeing with itself is that the Rust subject reproduces the same two files from its own reading
    of the same grammar, and the digests of what each side emitted are what the runner compares.
    """
    dsl_bytes = ctx.fixture_bytes(ARTIFACT_DSL)
    parsed = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(parsed).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("identity-round-trip: re-printing the parsed drawing did not reproduce the committed DSL file")
    pack = ctx.fixture_bytes(ARTIFACT_PACK)
    unpacked = parse_pack(pack)
    if unpacked != parsed:
        raise AssertionError("identity-round-trip: the committed binary twin decodes to a different drawing than the committed text artifact")
    repacked = pack_bytes(parsed)
    if repacked != pack:
        raise AssertionError("identity-round-trip: re-encoding the parsed drawing did not reproduce the committed pack file")
    return Outcome(
        {
            "document": projection_of(parsed),
            "shape": shape_report(parsed),
            "dslDigest": digest(printed),
            "packDigest": digest(repacked),
            "dslLength": len(printed),
            "packLength": len(repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the Python host calls, keyed by FULL expanded scenario id."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
