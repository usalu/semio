"""🧾️ `stdio.semio.model` pack carrier: the LEB128 frame and the pack reader and writer, with every enum ordinal taken from the order the DSL grammar declares its alternatives in."""

from __future__ import annotations

# region 🔖️Imports
import struct

from mutate_semio_model_text import (
    BINARY_MAGIC,
    PACK_TOKEN,
    PACK_FORMAT,
    SPATIAL_ORDER,
    ELEMENT_ORDER,
    RELATION_ORDER,
    GEOMETRY_ORDER,
    PSET_ORDER,
)
# endregion 🔖️Imports


# region 🔖️Pack
def read_varint(data: bytes, at: int):
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
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(out)


def read_string(data: bytes, at: int):
    length, at = read_varint(data, at)
    if at + length > len(data):
        raise AssertionError("the pack frame ends inside a length-prefixed string")
    return data[at : at + length].decode("utf-8"), at + length


def write_string(text: str) -> bytes:
    raw = text.encode("utf-8")
    return write_varint(len(raw)) + raw


def read_opt_string(data: bytes, at: int):
    tag = data[at]
    at += 1
    if tag == 0:
        return None, at
    return read_string(data, at)


def write_opt_string(value) -> bytes:
    return b"\x00" if value is None else b"\x01" + write_string(value)


def read_pack_transform(data: bytes, at: int):
    if at + 80 > len(data):
        raise AssertionError("the pack frame ends inside a transform")
    values = struct.unpack_from("<10d", data, at)
    transform = {
        "translation": {"x": values[0], "y": values[1], "z": values[2]},
        "rotation": {"x": values[3], "y": values[4], "z": values[5], "w": values[6]},
        "scale": {"x": values[7], "y": values[8], "z": values[9]},
    }
    return transform, at + 80


def write_pack_transform(transform: dict) -> bytes:
    translation, rotation, scale = transform["translation"], transform["rotation"], transform["scale"]
    return struct.pack("<10d", translation["x"], translation["y"], translation["z"], rotation["x"], rotation["y"], rotation["z"], rotation["w"], scale["x"], scale["y"], scale["z"])


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, the schema, and the three repeated record collections."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the pack file does not start with the semio binary magic")
    if len(data) < 12:
        raise AssertionError("the pack file is truncated inside its envelope")
    token_len = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_len].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("expected the %r envelope token, got %r" % (PACK_TOKEN, token))
    at = 12 + token_len
    if data[at] != PACK_FORMAT:
        raise AssertionError("unknown pack format byte %d" % data[at])
    at += 1
    schema, at = read_string(data, at)
    count, at = read_varint(data, at)
    spatial = []
    for _ in range(count):
        node_id, at = read_string(data, at)
        kind = data[at]
        at += 1
        if kind >= len(SPATIAL_ORDER):
            raise AssertionError("unknown spatial-kind ordinal %d" % kind)
        name, at = read_string(data, at)
        parent, at = read_opt_string(data, at)
        placement, at = read_pack_transform(data, at)
        spatial.append({"id": node_id, "kind": SPATIAL_ORDER[kind], "name": name, "parentId": parent, "placement": placement})
    count, at = read_varint(data, at)
    elements = []
    for _ in range(count):
        element_id, at = read_string(data, at)
        class_tag = data[at]
        at += 1
        if class_tag >= len(ELEMENT_ORDER):
            raise AssertionError("unknown element-class ordinal %d" % class_tag)
        if ELEMENT_ORDER[class_tag] == "other":
            name, at = read_string(data, at)
            element_class = {"kind": "other", "name": name}
        else:
            element_class = {"kind": ELEMENT_ORDER[class_tag]}
        placement, at = read_pack_transform(data, at)
        geometry_tag = data[at]
        at += 1
        if geometry_tag >= len(GEOMETRY_ORDER):
            raise AssertionError("unknown geometry-ref ordinal %d" % geometry_tag)
        if geometry_tag == 0:
            geometry = {"kind": "none"}
        else:
            reference, at = read_string(data, at)
            geometry = {"kind": "brep", "brepId": reference} if geometry_tag == 1 else {"kind": "mesh", "meshId": reference}
        spatial_id, at = read_opt_string(data, at)
        pset_count, at = read_varint(data, at)
        psets = []
        for _ in range(pset_count):
            pset_name, at = read_string(data, at)
            property_count, at = read_varint(data, at)
            properties = []
            for _ in range(property_count):
                key, at = read_string(data, at)
                value_tag = data[at]
                at += 1
                if value_tag == 0:
                    raw, at = read_string(data, at)
                    value = {"kind": "text", "value": raw}
                elif value_tag == 1:
                    value = {"kind": "number", "value": struct.unpack_from("<d", data, at)[0]}
                    at += 8
                elif value_tag == 2:
                    value = {"kind": "boolean", "value": data[at] == 1}
                    at += 1
                else:
                    raise AssertionError("unknown pset-value ordinal %d" % value_tag)
                properties.append({"key": key, "value": value})
            psets.append({"name": pset_name, "properties": properties})
        elements.append({"id": element_id, "class": element_class, "placement": placement, "geometry": geometry, "spatialId": spatial_id, "psets": psets})
    count, at = read_varint(data, at)
    relations = []
    for _ in range(count):
        relation_id, at = read_string(data, at)
        kind_tag = data[at]
        at += 1
        if kind_tag >= len(RELATION_ORDER):
            raise AssertionError("unknown relation-kind ordinal %d" % kind_tag)
        if RELATION_ORDER[kind_tag] == "other":
            label, at = read_string(data, at)
            kind = {"kind": "other", "label": label}
        else:
            kind = {"kind": RELATION_ORDER[kind_tag]}
        source, at = read_string(data, at)
        target, at = read_string(data, at)
        relations.append({"id": relation_id, "kind": kind, "from": source, "to": target})
    if at != len(data):
        raise AssertionError("%d trailing byte(s) after the last relation record" % (len(data) - at))
    return {"schema": schema, "spatial": spatial, "elements": elements, "relations": relations}


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["spatial"]))
    for node in document["spatial"]:
        body += write_string(node["id"])
        body.append(SPATIAL_ORDER.index(node["kind"]))
        body += write_string(node["name"])
        body += write_opt_string(node["parentId"])
        body += write_pack_transform(node["placement"])
    body += write_varint(len(document["elements"]))
    for element in document["elements"]:
        body += write_string(element["id"])
        body.append(ELEMENT_ORDER.index(element["class"]["kind"]))
        if element["class"]["kind"] == "other":
            body += write_string(element["class"]["name"])
        body += write_pack_transform(element["placement"])
        geometry = element["geometry"]
        body.append(GEOMETRY_ORDER.index(geometry["kind"]))
        if geometry["kind"] == "brep":
            body += write_string(geometry["brepId"])
        elif geometry["kind"] == "mesh":
            body += write_string(geometry["meshId"])
        body += write_opt_string(element["spatialId"])
        body += write_varint(len(element["psets"]))
        for pset in element["psets"]:
            body += write_string(pset["name"])
            body += write_varint(len(pset["properties"]))
            for prop in pset["properties"]:
                body += write_string(prop["key"])
                value = prop["value"]
                body.append(PSET_ORDER.index(value["kind"]))
                if value["kind"] == "text":
                    body += write_string(value["value"])
                elif value["kind"] == "number":
                    body += struct.pack("<d", float(value["value"]))
                else:
                    body.append(1 if value["value"] else 0)
    body += write_varint(len(document["relations"]))
    for relation in document["relations"]:
        body += write_string(relation["id"])
        kind = relation["kind"]
        body.append(RELATION_ORDER.index(kind["kind"]))
        if kind["kind"] == "other":
            body += write_string(kind["label"])
        body += write_string(relation["from"]) + write_string(relation["to"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack
