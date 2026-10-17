"""📦️ `stdio.semio.drawing` pack carrier: the LEB128 frame and the pack reader and writer, with every enum ordinal taken from the order the DSL grammar declares its alternatives in."""

from __future__ import annotations

# region 🔖️Imports
import struct

from mutate_semio_drawing_text import BINARY_MAGIC, PACK_TOKEN, PACK_FORMAT, NODE_ORDER, SEGMENT_ORDER
# endregion 🔖️Imports


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
        return {"translation": self.point3(), "rotation": self.quaternion(), "scale": self.point3()}

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
        self.point3(transform["scale"])

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
        return {"kind": "arcTo", "rx": reader.f64(), "ry": reader.f64(), "xRotation": reader.f64(), "largeArc": reader.flag(), "sweep": reader.flag(), "to": reader.point2()}
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
        writer.f64(segment["xRotation"])
        writer.byte(1 if segment["largeArc"] else 0)
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
    if kind == "group":
        transform = reader.transform()
        return {"kind": "group", "transform": transform, "children": [read_node(reader) for _ in range(reader.varint())]}
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
    if kind == "group":
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
