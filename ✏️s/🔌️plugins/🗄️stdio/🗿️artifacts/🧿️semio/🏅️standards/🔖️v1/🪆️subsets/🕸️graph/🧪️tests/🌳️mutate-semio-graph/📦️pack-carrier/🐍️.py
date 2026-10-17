"""🧾️ `stdio.semio.graph` pack carrier: the LEB128 frame and the pack reader and writer, with every enum ordinal taken from the order the DSL grammar declares its alternatives in."""

from __future__ import annotations

# region 🔖️Imports
import struct

from mutate_semio_graph_text import BINARY_MAGIC, PACK_TOKEN, PACK_FORMAT, PORT_ORDER, VALUE_ORDER, number
# endregion 🔖️Imports


# region 🔖️Pack
def read_varint(data: bytes, at: int) -> tuple:
    """🔢️ Unsigned LEB128 — the `varint` the protocol description names for every length."""
    value = 0
    shift = 0
    while True:
        if at >= len(data):
            raise AssertionError("the pack frame ends inside a varint")
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if byte & 0x80 == 0:
            return value, at
        shift += 7


def write_varint(value: int) -> bytes:
    """🔢️ The writing direction of the same encoding."""
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(out)


def read_string(data: bytes, at: int) -> tuple:
    """🧵️ A varint-length-prefixed UTF-8 string."""
    length, at = read_varint(data, at)
    if at + length > len(data):
        raise AssertionError("the pack frame ends inside a length-prefixed string")
    return data[at : at + length].decode("utf-8"), at + length


def write_string(text: str) -> bytes:
    """🧵️ The writing direction of the same scalar."""
    raw = text.encode("utf-8")
    return write_varint(len(raw)) + raw


def read_pack_value(data: bytes, at: int) -> tuple:
    """🏷️ One tagged `SemioValue`, mirroring the text grammar's own tag order."""
    ordinal = data[at]
    at += 1
    if ordinal >= len(VALUE_ORDER):
        raise AssertionError("unknown value ordinal %d" % ordinal)
    kind = VALUE_ORDER[ordinal]
    if kind == "null":
        return {"kind": "null"}, at
    if kind == "bool":
        flag = data[at]
        return {"kind": "bool", "value": flag == 1}, at + 1
    if kind in ("int", "float"):
        lexeme, at = read_string(data, at)
        return {"kind": kind, "lexeme": lexeme}, at
    if kind == "str":
        raw, at = read_string(data, at)
        return {"kind": "str", "value": raw}, at
    if kind == "bytes":
        length, at = read_varint(data, at)
        return {"kind": "bytes", "value": data[at : at + length].hex()}, at + length
    if kind == "ref":
        raw, at = read_string(data, at)
        return {"kind": "ref", "id": {"value": raw}}, at
    count, at = read_varint(data, at)
    if kind == "list":
        items = []
        for _ in range(count):
            item, at = read_pack_value(data, at)
            items.append(item)
        return {"kind": "list", "items": items}, at
    entries = []
    for _ in range(count):
        key, at = read_string(data, at)
        item, at = read_pack_value(data, at)
        entries.append({"key": key, "value": item})
    return {"kind": "map", "entries": entries}, at


def write_pack_value(value: dict) -> bytes:
    """🏷️ The writing direction of `read_pack_value`."""
    kind = value["kind"]
    out = bytearray([VALUE_ORDER.index(kind)])
    if kind == "null":
        return bytes(out)
    if kind == "bool":
        out.append(1 if value["value"] else 0)
        return bytes(out)
    if kind in ("int", "float"):
        return bytes(out + write_string(value["lexeme"]))
    if kind == "str":
        return bytes(out + write_string(value["value"]))
    if kind == "bytes":
        raw = bytes.fromhex(value["value"])
        return bytes(out + write_varint(len(raw)) + raw)
    if kind == "ref":
        return bytes(out + write_string(value["id"]["value"]))
    if kind == "list":
        out += write_varint(len(value["items"]))
        for item in value["items"]:
            out += write_pack_value(item)
        return bytes(out)
    out += write_varint(len(value["entries"]))
    for entry in value["entries"]:
        out += write_string(entry["key"])
        out += write_pack_value(entry["value"])
    return bytes(out)


def unwrap_binary(data: bytes) -> bytes:
    """📖️ Strips the semio binary envelope and refuses any token but this subset's own."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the pack file does not start with the semio binary magic")
    if len(data) < 12:
        raise AssertionError("the pack file is truncated inside its envelope")
    token_len = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_len].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("expected the %r envelope token, got %r" % (PACK_TOKEN, token))
    return data[12 + token_len :]


def read_pack_properties(data: bytes, at: int) -> tuple:
    """🏷️ A varint count, then that many `key` string + tagged value pairs."""
    count, at = read_varint(data, at)
    properties = []
    for _ in range(count):
        key, at = read_string(data, at)
        value, at = read_pack_value(data, at)
        properties.append({"key": key, "value": value})
    return properties, at


def write_pack_properties(properties: list) -> bytes:
    """🏷️ The writing direction of `read_pack_properties`."""
    out = bytearray(write_varint(len(properties)))
    for entry in properties:
        out += write_string(entry["key"])
        out += write_pack_value(entry["value"])
    return bytes(out)


def read_pack_word(data: bytes, at: int) -> tuple:
    """🔢️ One raw little-endian binary64, kept as its exact bit pattern."""
    return {"bits": "%016x" % struct.unpack_from("<Q", data, at)[0]}, at + 8


def write_pack_word(geometry: dict) -> bytes:
    """🔢️ The writing direction of `read_pack_word`."""
    return struct.pack("<d", number(geometry))


def read_pack_optional(data: bytes, at: int) -> tuple:
    """🫥️ `0` (absent) or `1` + one length-prefixed string."""
    tag = data[at]
    if tag == 0:
        return None, at + 1
    if tag != 1:
        raise AssertionError("invalid optional text tag %d" % tag)
    return read_string(data, at + 1)


def write_pack_optional(text) -> bytes:
    """🫥️ The writing direction of `read_pack_optional`."""
    return b"\x00" if text is None else b"\x01" + write_string(text)


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, the schema, the node records and the edge records."""
    body = unwrap_binary(data)
    if body[0] != PACK_FORMAT:
        raise AssertionError("unknown pack format byte %d" % body[0])
    schema, at = read_string(body, 1)
    node_count, at = read_varint(body, at)
    nodes = []
    for _ in range(node_count):
        node_id, at = read_string(body, at)
        kind, at = read_string(body, at)
        label, at = read_string(body, at)
        x, at = read_pack_word(body, at)
        y, at = read_pack_word(body, at)
        width, at = read_pack_word(body, at)
        height, at = read_pack_word(body, at)
        port_count, at = read_varint(body, at)
        ports = []
        for _ in range(port_count):
            name, at = read_string(body, at)
            ordinal = body[at]
            at += 1
            if ordinal >= len(PORT_ORDER):
                raise AssertionError("unknown port ordinal %d" % ordinal)
            category, at = read_string(body, at)
            port_properties, at = read_pack_properties(body, at)
            ports.append({"name": name, "kind": PORT_ORDER[ordinal], "category": category, "properties": port_properties})
        properties, at = read_pack_properties(body, at)
        nodes.append({"id": {"value": node_id}, "kind": kind, "label": label, "position": {"x": x, "y": y}, "width": width, "height": height, "ports": ports, "properties": properties})
    edge_count, at = read_varint(body, at)
    edges = []
    for _ in range(edge_count):
        fields = []
        for _ in range(5):
            field, at = read_string(body, at)
            fields.append(field)
        source_port, at = read_pack_optional(body, at)
        target_port, at = read_pack_optional(body, at)
        properties, at = read_pack_properties(body, at)
        edge = {"id": {"value": fields[0]}, "source": {"value": fields[1]}, "target": {"value": fields[2]}, "kind": fields[3], "label": fields[4], "properties": properties}
        for member, value in (("sourcePort", source_port), ("targetPort", target_port)):
            if value is not None:
                edge[member] = value
        edges.append(edge)
    if at != len(body):
        raise AssertionError("%d trailing byte(s) after the last edge record" % (len(body) - at))
    return {"schema": schema, "nodes": nodes, "edges": edges}


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["nodes"]))
    for node in document["nodes"]:
        body += write_string(node["id"]["value"])
        body += write_string(node["kind"])
        body += write_string(node["label"])
        for geometry in (node["position"]["x"], node["position"]["y"], node["width"], node["height"]):
            body += write_pack_word(geometry)
        body += write_varint(len(node["ports"]))
        for port in node["ports"]:
            body += write_string(port["name"])
            body.append(PORT_ORDER.index(port["kind"]))
            body += write_string(port["category"])
            body += write_pack_properties(port["properties"])
        body += write_pack_properties(node["properties"])
    body += write_varint(len(document["edges"]))
    for edge in document["edges"]:
        for field in (edge["id"]["value"], edge["source"]["value"], edge["target"]["value"], edge["kind"], edge["label"]):
            body += write_string(field)
        body += write_pack_optional(edge.get("sourcePort"))
        body += write_pack_optional(edge.get("targetPort"))
        body += write_pack_properties(edge["properties"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack
