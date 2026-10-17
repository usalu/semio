"""🧾️ `stdio.semio.brep` pack carrier: the LEB128 frame and the pack reader and writer, with every enum ordinal taken from the order the DSL grammar declares its alternatives in."""

from __future__ import annotations

# region 🔖️Imports
import struct

from mutate_semio_brep_text import (
    BINARY_MAGIC,
    PACK_TOKEN,
    PACK_FORMAT,
    CURVE_ORDER,
    SURFACE_ORDER,
    CURVE2_ORDER,
    CURVE_FIELDS,
    CURVE2_FIELDS,
    SURFACE_FIELDS,
)
# endregion 🔖️Imports


# region 🔖️Pack
def read_varint(data: bytes, at: int) -> tuple:
    """🔢️ Unsigned LEB128 — the `varint` the protocol description names for every count."""
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


def read_point_bytes(data: bytes, at: int) -> tuple:
    """📍️ Three little-endian f64 — the protocol's `real f64 LE coordinates`."""
    x, y, z = struct.unpack_from("<3d", data, at)
    return {"x": x, "y": y, "z": z}, at + 24


def write_point_bytes(point: dict) -> bytes:
    return struct.pack("<3d", point["x"], point["y"], point["z"])


def read_point2_bytes(data: bytes, at: int) -> tuple:
    """📌️ Two little-endian f64 — a p-curve's parameter-plane point."""
    x, y = struct.unpack_from("<2d", data, at)
    return {"x": x, "y": y}, at + 16


def write_point2_bytes(point: dict) -> bytes:
    return struct.pack("<2d", point["x"], point["y"])


def read_f64(data: bytes, at: int) -> tuple:
    """🔢️ One little-endian f64 — every `tol` and every prange bound."""
    return struct.unpack_from("<d", data, at)[0], at + 8


def read_pack_geometry(data: bytes, at: int, order: tuple, fields: dict, what: str) -> tuple:
    """🧩️ One tagged curve/surface record, read against its arm's own field list."""
    ordinal = data[at]
    at += 1
    if ordinal >= len(order):
        raise AssertionError("unknown %s ordinal %d" % (what, ordinal))
    kind = order[ordinal]
    value = {"kind": kind}
    for name, shape in fields[kind]:
        if shape == "p":
            value[name], at = read_point_bytes(data, at)
        elif shape == "q":
            value[name], at = read_point2_bytes(data, at)
        elif shape == "Q":
            count, at = read_varint(data, at)
            points = []
            for _ in range(count):
                point, at = read_point2_bytes(data, at)
                points.append(point)
            value[name] = points
        elif shape == "n":
            value[name] = struct.unpack_from("<d", data, at)[0]
            at += 8
        elif shape == "i":
            value[name], at = read_varint(data, at)
        elif shape == "P":
            count, at = read_varint(data, at)
            points = []
            for _ in range(count):
                point, at = read_point_bytes(data, at)
                points.append(point)
            value[name] = points
        else:
            count, at = read_varint(data, at)
            value[name] = list(struct.unpack_from("<%dd" % count, data, at)) if count else []
            at += 8 * count
    return value, at


def write_pack_geometry(value: dict, order: tuple, fields: dict) -> bytes:
    """🧩️ The writing direction of `read_pack_geometry`."""
    out = bytearray([order.index(value["kind"])])
    for name, shape in fields[value["kind"]]:
        member = value[name]
        if shape == "p":
            out += write_point_bytes(member)
        elif shape == "q":
            out += write_point2_bytes(member)
        elif shape == "Q":
            out += write_varint(len(member))
            for point in member:
                out += write_point2_bytes(point)
        elif shape == "n":
            out += struct.pack("<d", member)
        elif shape == "i":
            out += write_varint(int(member))
        elif shape == "P":
            out += write_varint(len(member))
            for point in member:
                out += write_point_bytes(point)
        else:
            out += write_varint(len(member))
            for number in member:
                out += struct.pack("<d", number)
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


def read_flagged(data: bytes, at: int, field: tuple) -> tuple:
    """🚩️ A length-prefixed id plus a one-byte flag — loop edge, shell face and solid shell alike."""
    name, at = read_string(data, at)
    flag = data[at]
    if flag not in (0, 1):
        raise AssertionError("the %s flag byte is %d, not 0 or 1" % (field[1], flag))
    return {field[0]: name, field[1]: flag == 1}, at + 1


def write_flagged(item: dict, field: tuple) -> bytes:
    return write_string(item[field[0]]) + bytes([1 if item[field[1]] else 0])


LOOP_EDGE = ("edge", "orientation")
SHELL_FACE = ("face", "orientation")
SOLID_SHELL = ("shell", "isVoid")


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, the schema, the seven collections in grammar order (each vertex, edge and
    face closing on its `tol` f64; a coedge's p-curve behind a presence byte) and the varint `nextLabel`."""
    body = unwrap_binary(data)
    if body[0] != PACK_FORMAT:
        raise AssertionError("unknown pack format byte %d" % body[0])
    schema, at = read_string(body, 1)
    document = {"schema": schema}
    count, at = read_varint(body, at)
    vertices = []
    for _ in range(count):
        vertex_id, at = read_string(body, at)
        point, at = read_point_bytes(body, at)
        tol, at = read_f64(body, at)
        vertices.append({"id": vertex_id, "point": point, "tol": tol})
    document["vertices"] = vertices
    count, at = read_varint(body, at)
    edges = []
    for _ in range(count):
        edge_id, at = read_string(body, at)
        start, at = read_string(body, at)
        end, at = read_string(body, at)
        curve, at = read_pack_geometry(body, at, CURVE_ORDER, CURVE_FIELDS, "curve")
        tol, at = read_f64(body, at)
        edges.append({"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve, "tol": tol})
    document["edges"] = edges
    count, at = read_varint(body, at)
    loops = []
    for _ in range(count):
        loop_id, at = read_string(body, at)
        inner_count, at = read_varint(body, at)
        items = []
        for _ in range(inner_count):
            item, at = read_flagged(body, at, LOOP_EDGE)
            items.append(item)
        loops.append({"id": loop_id, "edges": items})
    document["loops"] = loops
    count, at = read_varint(body, at)
    faces = []
    for _ in range(count):
        face_id, at = read_string(body, at)
        outer, at = read_string(body, at)
        inner_count, at = read_varint(body, at)
        inner = []
        for _ in range(inner_count):
            name, at = read_string(body, at)
            inner.append(name)
        surface, at = read_pack_geometry(body, at, SURFACE_ORDER, SURFACE_FIELDS, "surface")
        orientation = body[at]
        at += 1
        if orientation not in (0, 1):
            raise AssertionError("the face orientation byte is %d, not 0 or 1" % orientation)
        tol, at = read_f64(body, at)
        faces.append({"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation == 1, "tol": tol})
    document["faces"] = faces
    for key, field, member in (("shells", SHELL_FACE, "faces"), ("solids", SOLID_SHELL, "shells")):
        count, at = read_varint(body, at)
        records = []
        for _ in range(count):
            record_id, at = read_string(body, at)
            inner_count, at = read_varint(body, at)
            items = []
            for _ in range(inner_count):
                item, at = read_flagged(body, at, field)
                items.append(item)
            records.append({"id": record_id, member: items})
        document[key] = records
    count, at = read_varint(body, at)
    coedges = []
    for _ in range(count):
        coedge_id, at = read_string(body, at)
        edge, at = read_string(body, at)
        forward, at = body[at] == 1, at + 1
        present, at = body[at] == 1, at + 1
        pcurve = None
        if present:
            pcurve, at = read_pack_geometry(body, at, CURVE2_ORDER, CURVE2_FIELDS, "curve2")
        start, at = read_f64(body, at)
        end, at = read_f64(body, at)
        loop_id, at = read_string(body, at)
        following, at = read_string(body, at)
        preceding, at = read_string(body, at)
        coedges.append({"id": coedge_id, "edge": edge, "forward": forward, "pcurve": pcurve, "prange": [start, end], "loopId": loop_id, "next": following, "prev": preceding})
    document["coedges"] = coedges
    document["nextLabel"], at = read_varint(body, at)
    if at != len(body):
        raise AssertionError("%d trailing byte(s) after nextLabel" % (len(body) - at))
    return document


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["vertices"]))
    for vertex in document["vertices"]:
        body += write_string(vertex["id"]) + write_point_bytes(vertex["point"]) + struct.pack("<d", vertex["tol"])
    body += write_varint(len(document["edges"]))
    for edge in document["edges"]:
        body += write_string(edge["id"]) + write_string(edge["startVertex"]) + write_string(edge["endVertex"])
        body += write_pack_geometry(edge["curve"], CURVE_ORDER, CURVE_FIELDS) + struct.pack("<d", edge["tol"])
    body += write_varint(len(document["loops"]))
    for loop in document["loops"]:
        body += write_string(loop["id"]) + write_varint(len(loop["edges"]))
        for item in loop["edges"]:
            body += write_flagged(item, LOOP_EDGE)
    body += write_varint(len(document["faces"]))
    for face in document["faces"]:
        body += write_string(face["id"]) + write_string(face["outerLoop"]) + write_varint(len(face["innerLoops"]))
        for name in face["innerLoops"]:
            body += write_string(name)
        body += write_pack_geometry(face["surface"], SURFACE_ORDER, SURFACE_FIELDS)
        body.append(1 if face["orientation"] else 0)
        body += struct.pack("<d", face["tol"])
    for key, field, member in (("shells", SHELL_FACE, "faces"), ("solids", SOLID_SHELL, "shells")):
        body += write_varint(len(document[key]))
        for record in document[key]:
            body += write_string(record["id"]) + write_varint(len(record[member]))
            for item in record[member]:
                body += write_flagged(item, field)
    body += write_varint(len(document["coedges"]))
    for coedge in document["coedges"]:
        body += write_string(coedge["id"]) + write_string(coedge["edge"]) + bytes([1 if coedge["forward"] else 0, 0 if coedge["pcurve"] is None else 1])
        if coedge["pcurve"] is not None:
            body += write_pack_geometry(coedge["pcurve"], CURVE2_ORDER, CURVE2_FIELDS)
        body += struct.pack("<2d", coedge["prange"][0], coedge["prange"][1]) + write_string(coedge["loopId"]) + write_string(coedge["next"]) + write_string(coedge["prev"])
    body += write_varint(document["nextLabel"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack
