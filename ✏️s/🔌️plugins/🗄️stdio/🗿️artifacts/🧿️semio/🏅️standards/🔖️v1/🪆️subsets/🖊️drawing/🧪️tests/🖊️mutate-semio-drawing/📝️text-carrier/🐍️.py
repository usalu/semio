"""📝️ `stdio.semio.drawing` text carrier: the envelope constants, the grammar vocabularies with their ordinals and the DSL reader and printer, written from the committed grammar alone."""

from __future__ import annotations

# region 🔖️Imports
import struct
# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
DSL_PREAMBLE = "semio stdio.semio.drawing.dsl v1"
PACK_TOKEN = "stdio.semio.drawing.pack v1"
PACK_FORMAT = 1

#: 🖍️ `node = "P"… | "T"… | "G"… | "I"…`, in the enum order the pack tag byte indexes — the committed
#: sketch's group root carries `0x02` and its image child `0x03`.
NODE_ORDER = ("path", "text", "group", "image")
NODE_LETTER = {"path": "P", "text": "T", "group": "G", "image": "I"}
LETTER_NODE = {letter: kind for kind, letter in NODE_LETTER.items()}

#: ✏️ `segment = "M"… | "L"… | "C"… | "Q"… | "A"… | "Z"`, likewise — the sketch's one path exhibits
#: all six in that order and its pack tags them `0x00`…`0x05`.
SEGMENT_ORDER = ("moveTo", "lineTo", "cubicTo", "quadTo", "arcTo", "close")
SEGMENT_LETTER = {"moveTo": "M", "lineTo": "L", "cubicTo": "C", "quadTo": "Q", "arcTo": "A", "close": "Z"}
LETTER_SEGMENT = {letter: kind for kind, letter in SEGMENT_LETTER.items()}

ARTIFACT_DSL = "shared://🖊️mutate-semio-drawing/🗣️.dsl.semio"
ARTIFACT_PACK = "shared://🖊️mutate-semio-drawing/🎒️.pack.semio"


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
    return {"translation": parse_point3(parts[0]), "rotation": parse_quaternion(parts[1]), "scale": parse_point3(parts[2])}


def print_transform(transform: dict) -> str:
    """📐️ `transform` in the writing direction."""
    return "[%s,%s,%s]" % (print_point3(transform["translation"]), print_quaternion(transform["rotation"]), print_point3(transform["scale"]))


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
        "xRotation": float(parts[2]),
        "largeArc": parse_bool(parts[3], "largeArc"),
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
            print_number(segment["xRotation"]),
            print_bool(segment["largeArc"]),
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
    if kind == "group":
        return {"kind": "group", "transform": parse_transform(parts[0]), "children": [parse_node(entry) for entry in items_of(parts[1], "children")]}
    return {"kind": "image", "at": parse_point2(parts[0]), "width": float(parts[1]), "height": float(parts[2]), "mime": text_of_hex(parts[3]), "bytes": list(bytes.fromhex(parts[4]))}


def print_node(node: dict) -> str:
    """🖍️ One `node` in the writing direction."""
    kind = node["kind"]
    if kind == "path":
        return "P[[%s],%s]" % (",".join(print_segment(segment) for segment in node["segments"]), print_option(node.get("style"), hex_of_text))
    if kind == "text":
        return "T[%s,%s,%s]" % (hex_of_text(node["value"]), print_point2(node["at"]), print_option(node.get("style"), hex_of_text))
    if kind == "group":
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
