"""🐍️ Independent Python implementation of the `s.stdio.semio.brep` carrier and its thirteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line vertices-line edges-line loops-line faces-line shells-line
  solids-line`, the tagged `curve = L|C|E|N` and `surface = P|C|O|S|T|N` value productions with
  their exact field lists, and `bool = "0" | "1"`);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`, which names every
  member of every curve and surface arm (`origin`/`direction`, `center`/`axis`/`radius`,
  `radiusMajor`/`radiusMinor`, `controlPoints`/`weights`/`degree`/`knots`, `normal`, `halfAngle`,
  `majorRadius`/`minorRadius`, `uCount`/`vCount`/`degreeU`/`degreeV`/`knotsU`/`knotsV`) and the
  topology records' `startVertex`/`endVertex`/`outerLoop`/`innerLoops`/`isVoid`;
* the thirteen verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body's `format u8` + varint-length-prefixed `schema` is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio`, whose prose then names — but declines to
  frame — "vertices/edges/loops/faces/shells/solids … varint counts, per-field length-prefixes,
  real `f64` LE coordinates, a real per-variant tag byte for `curve`/`surface`". That
  named-but-unframed layout was written out here from the protocol's own sentence, with the field
  order and the tag ordinals read off the grammar's own `L|C|E|N` and `P|C|O|S|T|N` orders, and is
  PINNED by `pack_bytes` re-encoding the committed `🎒️.pack.semio` byte for byte — a file
  that carries a line, a circle, a NURBS curve and a NURBS surface, so four of the ten tagged arms
  are pinned directly and the remaining six are the same field lists the grammar spells out.

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
DSL_PREAMBLE = "semio stdio.semio.brep.dsl v1"
PACK_TOKEN = "stdio.semio.brep.pack v1"
DOCUMENT_SCHEMA = "stdio.semio.brep"
PACK_FORMAT = 1

#: ➰ `curve = "L" … | "C" … | "E" … | "N" …` in the grammar's own order — which is also the pack
#: ordinal, as the committed example's `line → 0x00`, `circle → 0x01` and `nurbs → 0x03` show.
CURVE_ORDER = ("line", "circle", "ellipse", "nurbs")
CURVE_LETTER = {"line": "L", "circle": "C", "ellipse": "E", "nurbs": "N"}
LETTER_CURVE = {letter: kind for kind, letter in CURVE_LETTER.items()}
#: 🗺️ `surface = "P" … | "C" … | "O" … | "S" … | "T" … | "N" …` in the grammar's own order — which is
#: also the pack ordinal, as the committed example's `nurbs → 0x05` shows.
SURFACE_ORDER = ("plane", "cylinder", "cone", "sphere", "torus", "nurbs")
SURFACE_LETTER = {"plane": "P", "cylinder": "C", "cone": "O", "sphere": "S", "torus": "T", "nurbs": "N"}
LETTER_SURFACE = {letter: kind for kind, letter in SURFACE_LETTER.items()}

#: 📐️ The scalar members of each tagged arm, in the order the grammar lists them. `p` is a
#: `point3`, `n` a `number`, `P` a `point3-list`, `N` a `number-list` and `i` the integral `degree`.
CURVE_FIELDS = {
    "line": (("origin", "p"), ("direction", "p")),
    "circle": (("center", "p"), ("axis", "p"), ("radius", "n")),
    "ellipse": (("center", "p"), ("axis", "p"), ("radiusMajor", "n"), ("radiusMinor", "n")),
    "nurbs": (("controlPoints", "P"), ("weights", "N"), ("degree", "i"), ("knots", "N")),
}
SURFACE_FIELDS = {
    "plane": (("origin", "p"), ("normal", "p")),
    "cylinder": (("origin", "p"), ("axis", "p"), ("radius", "n")),
    "cone": (("origin", "p"), ("axis", "p"), ("radius", "n"), ("halfAngle", "n")),
    "sphere": (("center", "p"), ("radius", "n")),
    "torus": (("center", "p"), ("axis", "p"), ("majorRadius", "n"), ("minorRadius", "n")),
    "nurbs": (("controlPoints", "P"), ("weights", "N"), ("uCount", "i"), ("vCount", "i"), ("degreeU", "i"), ("degreeV", "i"), ("knotsU", "N"), ("knotsV", "N")),
}

#: 🌲️ The document every mutation row runs on: the real "hexagonal cut concrete forest" structure,
#: 167 vertices / 270 B-spline edges / 127 loops / 127 planar faces / 12 shells / 12 solids, derived
#: ONCE from the real committed Rhino BIM export by `🐍️derive-brep-fixture.py` in the ticket folder.
FOREST_DSL = "local://🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio"
FOREST_PACK = "local://🎒️.pack.semio"
#: 🧊️ The tiny committed `✉️base` solid, kept for the BYTE half of the identity law: its two files
#: were written by the RUST codec, so this implementation reproducing them is a cross-language byte
#: agreement the forest pair — written by this implementation — cannot restate.
SOLID_DSL = "asset://📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio"
SOLID_PACK = "asset://📚️examples/🧊️solid/🖼️assets/🎒️.pack.semio"


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def print_number(value: float) -> str:
    """🔢️ `number = INT | FLOAT` in the writing direction — an integral magnitude prints without a
    fractional part, which is what the committed artifact's `[4,1.5,0]` circle centre shows the
    lexeme convention to be.

    Neither `INT` nor `FLOAT` has an exponent part, so a magnitude whose shortest round-tripping
    digit string would otherwise need one is written POSITIONALLY instead: the shortest digits that
    identify the value are placed against the decimal point and padded with zeros. The real
    B-spline control points and plane normals of the derived concrete-forest solid carry 98 such
    magnitudes (real Rhino residues down to 1e-18), and no committed artifact had ever exercised
    this lexeme class before them."""
    if value != value or value in (float("inf"), float("-inf")):
        raise AssertionError("the grammar's `number` has no lexeme for %r" % value)
    if value == int(value) and abs(value) < 1e16:
        return str(int(value))
    lexeme = repr(float(value))
    if "e" not in lexeme and "E" not in lexeme:
        return lexeme
    mantissa, _, exponent = lexeme.lower().partition("e")
    sign = "-" if mantissa.startswith("-") else ""
    whole, _, fraction = mantissa.lstrip("-").partition(".")
    digits = whole + fraction
    point = len(whole) + int(exponent)
    if point <= 0:
        return "%s0.%s%s" % (sign, "0" * -point, digits)
    if point >= len(digits):
        return "%s%s%s" % (sign, digits, "0" * (point - len(digits)))
    return "%s%s.%s" % (sign, digits[:point], digits[point:])


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


# endregion 🔖️Carrier


# region 🔖️Dsl
class Reader:
    """🔎️ A one-character-lookahead cursor over one DSL line. The grammar's only ambiguity is that a
    curve/surface tag letter is also a possible hex digit, which the fixed position of each tag at
    the head of its own bracketed value resolves."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.at = 0

    def peek(self) -> str:
        return self.text[self.at] if self.at < len(self.text) else ""

    def take(self, char: str) -> None:
        if self.peek() != char:
            raise AssertionError("expected %r at offset %d, found %r" % (char, self.at, self.peek()))
        self.at += 1

    def letter(self) -> str:
        char = self.peek()
        if char == "":
            raise AssertionError("the line ends where a tag letter was expected")
        self.at += 1
        return char

    def hex(self) -> str:
        start = self.at
        while self.peek() in "0123456789abcdef" and self.peek() != "":
            self.at += 1
        return text_of(self.text[start : self.at])

    def number(self) -> float:
        start = self.at
        while self.peek() != "" and self.peek() not in ",]":
            self.at += 1
        return float(self.text[start : self.at])

    def bit(self) -> bool:
        char = self.letter()
        if char not in ("0", "1"):
            raise AssertionError("expected the grammar's `bool` (0 or 1), found %r" % char)
        return char == "1"

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text: %r" % self.text[self.at :])


def read_items(reader: Reader, reader_of) -> list:
    """📚️ A bracketed, comma-separated list — the shape every collection in this grammar shares."""
    reader.take("[")
    items = []
    while reader.peek() != "]":
        items.append(reader_of(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return items


def read_point(reader: Reader) -> dict:
    """📍️ `point3 = "[" number "," number "," number "]"`."""
    reader.take("[")
    x = reader.number()
    reader.take(",")
    y = reader.number()
    reader.take(",")
    z = reader.number()
    reader.take("]")
    return {"x": x, "y": y, "z": z}


def print_point(point: dict) -> str:
    return "[%s,%s,%s]" % (print_number(point["x"]), print_number(point["y"]), print_number(point["z"]))


def read_geometry(reader: Reader, letters: dict, fields: dict, what: str) -> dict:
    """🧩️ One tagged `curve`/`surface`, read against its arm's own field list."""
    letter = reader.letter()
    if letter not in letters:
        raise AssertionError("unknown %s tag %r" % (what, letter))
    kind = letters[letter]
    reader.take("[")
    value = {"kind": kind}
    for index, (name, shape) in enumerate(fields[kind]):
        if index:
            reader.take(",")
        if shape == "p":
            value[name] = read_point(reader)
        elif shape == "n":
            value[name] = reader.number()
        elif shape == "i":
            value[name] = int(reader.number())
        elif shape == "P":
            value[name] = read_items(reader, read_point)
        else:
            value[name] = read_items(reader, lambda inner: inner.number())
    reader.take("]")
    return value


def print_geometry(value: dict, letters: dict, fields: dict, what: str) -> str:
    """🧩️ The writing direction of `read_geometry`."""
    kind = value["kind"]
    if kind not in fields:
        raise AssertionError("unknown %s kind %r" % (what, kind))
    parts = []
    for name, shape in fields[kind]:
        member = value[name]
        if shape == "p":
            parts.append(print_point(member))
        elif shape in ("n", "i"):
            parts.append(print_number(member))
        elif shape == "P":
            parts.append("[%s]" % ",".join(print_point(point) for point in member))
        else:
            parts.append("[%s]" % ",".join(print_number(number) for number in member))
    return "%s[%s]" % (letters[kind], ",".join(parts))


def read_curve(reader: Reader) -> dict:
    return read_geometry(reader, LETTER_CURVE, CURVE_FIELDS, "curve")


def print_curve(value: dict) -> str:
    return print_geometry(value, CURVE_LETTER, CURVE_FIELDS, "curve")


def read_surface(reader: Reader) -> dict:
    return read_geometry(reader, LETTER_SURFACE, SURFACE_FIELDS, "surface")


def print_surface(value: dict) -> str:
    return print_geometry(value, SURFACE_LETTER, SURFACE_FIELDS, "surface")


def read_vertex(reader: Reader) -> dict:
    """📍️ `vertex = "[" hex "," point3 "]"`."""
    reader.take("[")
    vertex_id = reader.hex()
    reader.take(",")
    point = read_point(reader)
    reader.take("]")
    return {"id": vertex_id, "point": point}


def read_edge(reader: Reader) -> dict:
    """➰ `edge = "[" hex "," hex "," hex "," curve "]"`."""
    reader.take("[")
    edge_id = reader.hex()
    reader.take(",")
    start = reader.hex()
    reader.take(",")
    end = reader.hex()
    reader.take(",")
    curve = read_curve(reader)
    reader.take("]")
    return {"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve}


def flagged_reader(field: str):
    """🚩️ `"[" hex "," bool "]"` — the shape a loop edge, a shell face and a solid shell all share."""

    def read(reader: Reader) -> dict:
        reader.take("[")
        name = reader.hex()
        reader.take(",")
        flag = reader.bit()
        reader.take("]")
        return {field[0]: name, field[1]: flag}

    return read


read_loop_edge = flagged_reader(("edge", "orientation"))
read_shell_face = flagged_reader(("face", "orientation"))
read_solid_shell = flagged_reader(("shell", "isVoid"))


def read_loop(reader: Reader) -> dict:
    """🔁️ `brep-loop = "[" hex "," loop-edge-list "]"`."""
    reader.take("[")
    loop_id = reader.hex()
    reader.take(",")
    edges = read_items(reader, read_loop_edge)
    reader.take("]")
    return {"id": loop_id, "edges": edges}


def read_face(reader: Reader) -> dict:
    """🔷️ `face = "[" hex "," hex "," hex-list "," surface "," bool "]"`."""
    reader.take("[")
    face_id = reader.hex()
    reader.take(",")
    outer = reader.hex()
    reader.take(",")
    inner = read_items(reader, lambda inner_reader: inner_reader.hex())
    reader.take(",")
    surface = read_surface(reader)
    reader.take(",")
    orientation = reader.bit()
    reader.take("]")
    return {"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation}


def read_shell(reader: Reader) -> dict:
    """🐚️ `shell = "[" hex "," shell-face-list "]"`."""
    reader.take("[")
    shell_id = reader.hex()
    reader.take(",")
    faces = read_items(reader, read_shell_face)
    reader.take("]")
    return {"id": shell_id, "faces": faces}


def read_solid(reader: Reader) -> dict:
    """🧊️ `solid = "[" hex "," solid-shell-list "]"`."""
    reader.take("[")
    solid_id = reader.hex()
    reader.take(",")
    shells = read_items(reader, read_solid_shell)
    reader.take("]")
    return {"id": solid_id, "shells": shells}


COLLECTIONS = (("vertices", read_vertex), ("edges", read_edge), ("loops", read_loop), ("faces", read_face), ("shells", read_shell), ("solids", read_solid))


def parse_dsl(text: str) -> dict:
    """📖️ The seven body lines of a brep document, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    keys = ["schema"] + [name for name, _ in COLLECTIONS]
    if len(body) != len(keys):
        raise AssertionError("a brep document is exactly %d body lines, found %d" % (len(keys), len(body)))
    values = []
    for key, line in zip(keys, body):
        if not line.startswith(key + "="):
            raise AssertionError("expected the %r line, found %r" % (key, line))
        values.append(line[len(key) + 1 :])
    schema = text_of(values[0])
    if schema != DOCUMENT_SCHEMA:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, DOCUMENT_SCHEMA))
    document = {"schema": schema}
    for (name, reader_of), raw in zip(COLLECTIONS, values[1:]):
        reader = Reader(raw)
        document[name] = read_items(reader, reader_of)
        reader.done()
    return document


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    vertices = ",".join("[%s,%s]" % (hex_of(vertex["id"]), print_point(vertex["point"])) for vertex in document["vertices"])
    edges = ",".join("[%s,%s,%s,%s]" % (hex_of(edge["id"]), hex_of(edge["startVertex"]), hex_of(edge["endVertex"]), print_curve(edge["curve"])) for edge in document["edges"])
    loops = ",".join("[%s,[%s]]" % (hex_of(loop["id"]), ",".join("[%s,%s]" % (hex_of(item["edge"]), "1" if item["orientation"] else "0") for item in loop["edges"])) for loop in document["loops"])
    faces = ",".join(
        "[%s,%s,[%s],%s,%s]" % (hex_of(face["id"]), hex_of(face["outerLoop"]), ",".join(hex_of(name) for name in face["innerLoops"]), print_surface(face["surface"]), "1" if face["orientation"] else "0")
        for face in document["faces"]
    )
    shells = ",".join("[%s,[%s]]" % (hex_of(shell["id"]), ",".join("[%s,%s]" % (hex_of(item["face"]), "1" if item["orientation"] else "0") for item in shell["faces"])) for shell in document["shells"])
    solids = ",".join("[%s,[%s]]" % (hex_of(solid["id"]), ",".join("[%s,%s]" % (hex_of(item["shell"]), "1" if item["isVoid"] else "0") for item in solid["shells"])) for solid in document["solids"])
    return "\n".join(
        [
            DSL_PREAMBLE,
            "schema=%s" % hex_of(document["schema"]),
            "vertices=[%s]" % vertices,
            "edges=[%s]" % edges,
            "loops=[%s]" % loops,
            "faces=[%s]" % faces,
            "shells=[%s]" % shells,
            "solids=[%s]" % solids,
        ]
    )


# endregion 🔖️Dsl


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
    """📦️ Binary envelope, then `format u8`, the schema, and the six collections in grammar order."""
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
        vertices.append({"id": vertex_id, "point": point})
    document["vertices"] = vertices
    count, at = read_varint(body, at)
    edges = []
    for _ in range(count):
        edge_id, at = read_string(body, at)
        start, at = read_string(body, at)
        end, at = read_string(body, at)
        curve, at = read_pack_geometry(body, at, CURVE_ORDER, CURVE_FIELDS, "curve")
        edges.append({"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve})
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
        faces.append({"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation == 1})
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
    if at != len(body):
        raise AssertionError("%d trailing byte(s) after the last solid record" % (len(body) - at))
    return document


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["vertices"]))
    for vertex in document["vertices"]:
        body += write_string(vertex["id"]) + write_point_bytes(vertex["point"])
    body += write_varint(len(document["edges"]))
    for edge in document["edges"]:
        body += write_string(edge["id"]) + write_string(edge["startVertex"]) + write_string(edge["endVertex"])
        body += write_pack_geometry(edge["curve"], CURVE_ORDER, CURVE_FIELDS)
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
    for key, field, member in (("shells", SHELL_FACE, "faces"), ("solids", SOLID_SHELL, "shells")):
        body += write_varint(len(document[key]))
        for record in document[key]:
            body += write_string(record["id"]) + write_varint(len(record[member]))
            for item in record[member]:
                body += write_flagged(item, field)
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = (
    "create-vertex",
    "delete-vertex",
    "create-edge",
    "delete-edge",
    "create-face",
    "delete-face",
    "create-shell",
    "delete-shell",
    "create-solid",
    "delete-solid",
    "replace-curve",
    "replace-surface",
    "move-vertex",
)

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-vertex": "CreateVertex",
    "delete-vertex": "DeleteVertex",
    "create-edge": "CreateEdge",
    "delete-edge": "DeleteEdge",
    "create-face": "CreateFace",
    "delete-face": "DeleteFace",
    "create-shell": "CreateShell",
    "delete-shell": "DeleteShell",
    "create-solid": "CreateSolid",
    "delete-solid": "DeleteSolid",
    "replace-curve": "ReplaceCurve",
    "replace-surface": "ReplaceSurface",
    "move-vertex": "MoveVertex",
}
#: 🗂️ Which collection each simple `delete-<entity>` verb removes from.
DELETE_SLOT = {"DeleteEdge": "edges", "DeleteFace": "faces", "DeleteShell": "shells", "DeleteSolid": "solids"}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"CreateVertex": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def index_of(items: list, entity_id: str, verb: str, what: str) -> int:
    """🔎️ The position of the id-keyed record one verb addresses; absence is a refusal."""
    for index, entry in enumerate(items):
        if entry["id"] == entity_id:
            return index
    raise AssertionError("%s addresses %s %r, which the solid does not carry" % (verb, what, entity_id))


def refuse_duplicate(items: list, entity_id: str, verb: str, what: str) -> None:
    if any(entry["id"] == entity_id for entry in items):
        raise AssertionError("%s uses %s id %r, which the solid already carries" % (verb, what, entity_id))


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. `delete-vertex` CASCADES into every edge that
    starts or ends at it, which is the behaviour the committed `removes-a-corner-vertex-and-cascades-
    into-its-two-incident-edges` vector pins; no other deletion cascades, which is what the committed
    `🚫️removes-the-closing-edge-and-keeps-its-two-vertices` and `removes-the-only-face-and-leaves-its-
    loop-behind` vectors pin. An unaddressable id is a refusal, never a silent no-op."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag == "CreateVertex":
        refuse_duplicate(result["vertices"], args["id"], tag, "vertex")
        result["vertices"].append({"id": args["id"], "point": clone(args["point"])})
    elif tag == "DeleteVertex":
        del result["vertices"][index_of(result["vertices"], args["id"], tag, "vertex")]
        result["edges"] = [edge for edge in result["edges"] if edge["startVertex"] != args["id"] and edge["endVertex"] != args["id"]]
    elif tag == "CreateEdge":
        refuse_duplicate(result["edges"], args["id"], tag, "edge")
        result["edges"].append({"id": args["id"], "startVertex": args["start_vertex"], "endVertex": args["end_vertex"], "curve": clone(args["curve"])})
    elif tag == "CreateFace":
        refuse_duplicate(result["faces"], args["id"], tag, "face")
        result["faces"].append({"id": args["id"], "outerLoop": args["outer_loop"], "innerLoops": clone(args["inner_loops"]), "surface": clone(args["surface"]), "orientation": args["orientation"]})
    elif tag == "CreateShell":
        refuse_duplicate(result["shells"], args["id"], tag, "shell")
        result["shells"].append({"id": args["id"], "faces": clone(args["faces"])})
    elif tag == "CreateSolid":
        refuse_duplicate(result["solids"], args["id"], tag, "solid")
        result["solids"].append({"id": args["id"], "shells": clone(args["shells"])})
    elif tag in DELETE_SLOT:
        slot = DELETE_SLOT[tag]
        del result[slot][index_of(result[slot], args["id"], tag, slot[:-1])]
    elif tag == "ReplaceCurve":
        result["edges"][index_of(result["edges"], args["edge_id"], tag, "edge")]["curve"] = clone(args["new_curve"])
    elif tag == "ReplaceSurface":
        result["faces"][index_of(result["faces"], args["face_id"], tag, "face")]["surface"] = clone(args["new_surface"])
    else:
        result["vertices"][index_of(result["vertices"], args["vertex_id"], tag, "vertex")]["point"] = clone(args["new_point"])
    return result


def edge_mutation(edge: dict) -> dict:
    """➰ The `CreateEdge` that puts one edge back exactly as it was."""
    return {"CreateEdge": {"id": edge["id"], "start_vertex": edge["startVertex"], "end_vertex": edge["endVertex"], "curve": clone(edge["curve"])}}


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an append is undone by the matching delete, a replacement by a replacement with the
    geometry it displaced, and the cascading `delete-vertex` by re-creating the vertex AND every
    incident edge it severed."""
    tag, args = tagged(mutation)
    if tag == "CreateVertex":
        return [{"DeleteVertex": {"id": args["id"]}}]
    if tag == "DeleteVertex":
        vertex = document["vertices"][index_of(document["vertices"], args["id"], tag, "vertex")]
        steps = [{"CreateVertex": {"id": vertex["id"], "point": clone(vertex["point"])}}]
        steps.extend(edge_mutation(edge) for edge in document["edges"] if edge["startVertex"] == args["id"] or edge["endVertex"] == args["id"])
        return steps
    if tag == "CreateEdge":
        return [{"DeleteEdge": {"id": args["id"]}}]
    if tag == "DeleteEdge":
        return [edge_mutation(document["edges"][index_of(document["edges"], args["id"], tag, "edge")])]
    if tag == "CreateFace":
        return [{"DeleteFace": {"id": args["id"]}}]
    if tag == "DeleteFace":
        face = document["faces"][index_of(document["faces"], args["id"], tag, "face")]
        return [{"CreateFace": {"id": face["id"], "outer_loop": face["outerLoop"], "inner_loops": clone(face["innerLoops"]), "surface": clone(face["surface"]), "orientation": face["orientation"]}}]
    if tag == "CreateShell":
        return [{"DeleteShell": {"id": args["id"]}}]
    if tag == "DeleteShell":
        shell = document["shells"][index_of(document["shells"], args["id"], tag, "shell")]
        return [{"CreateShell": {"id": shell["id"], "faces": clone(shell["faces"])}}]
    if tag == "CreateSolid":
        return [{"DeleteSolid": {"id": args["id"]}}]
    if tag == "DeleteSolid":
        solid = document["solids"][index_of(document["solids"], args["id"], tag, "solid")]
        return [{"CreateSolid": {"id": solid["id"], "shells": clone(solid["shells"])}}]
    if tag == "ReplaceCurve":
        edge = document["edges"][index_of(document["edges"], args["edge_id"], tag, "edge")]
        return [{"ReplaceCurve": {"edge_id": edge["id"], "new_curve": clone(edge["curve"])}}]
    if tag == "ReplaceSurface":
        face = document["faces"][index_of(document["faces"], args["face_id"], tag, "face")]
        return [{"ReplaceSurface": {"face_id": face["id"], "new_surface": clone(face["surface"])}}]
    vertex = document["vertices"][index_of(document["vertices"], args["vertex_id"], tag, "vertex")]
    return [{"MoveVertex": {"vertex_id": vertex["id"], "new_point": clone(vertex["point"])}}]


def apply_all(document: dict, mutations: list) -> dict:
    """🧬️ Folds a list of verbs over a document, left to right."""
    for mutation in mutations:
        document = apply_mutation(document, mutation)
    return document


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_json(ctx: Context) -> dict:
    """📜️ The scenario's own committed vector — the feature owns the parameters, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def step_assets(ctx: Context) -> list:
    """🧫️ Every `asset://` URI the scenario's steps name, in step order."""
    found = []
    for step in ctx.scenario["steps"]:
        text = step.get("text", "")
        at = text.find("asset://")
        while at != -1:
            end = at
            while end < len(text) and not text[end].isspace():
                end += 1
            found.append(text[at:end])
            at = text.find("asset://", end)
    return found


def prepared(ctx: Context) -> tuple:
    """🧊️ The real committed solid, put into the state the scenario's verb is aimed at by the doc
    string's own `prepare` list, plus the verb itself."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.fixture_bytes(FOREST_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def fixture_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed solid by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored solid must be the
    prepared solid again — asserted here, and compared against the subject's restored solid by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the solid\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector. The vector
    is a THIRD statement of what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = step_assets(ctx)[:3]
    before = fixture_json(ctx, before_uri)
    after = fixture_json(ctx, after_uri)
    applied = apply_mutation(before, fixture_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    return Outcome(applied)


def carrier_pair(ctx: Context, dsl_uri: str, pack_uri: str, what: str) -> dict:
    """🔁️ One document's two committed encodings, each re-emitted from the parsed document and
    required to come back byte for byte. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` is its binary twin, so an exact re-emission is the CORRECT answer and the wave's
    must-differ tripwire would be backwards here."""
    dsl_bytes = ctx.fixture_bytes(dsl_uri)
    document = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(document).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("re-printing %s did not reproduce its committed DSL bytes (%d vs %d bytes)" % (what, len(printed), len(dsl_bytes)))
    if parse_dsl(printed.decode("utf-8")) != document:
        raise AssertionError("re-parsing the printed %s lost content" % what)
    committed_pack = ctx.fixture_bytes(pack_uri)
    unpacked = parse_pack(committed_pack)
    if unpacked != document:
        raise AssertionError("the binary twin of %s decodes to a different document than its text\n     got: %s\nexpected: %s" % (what, json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding %s did not reproduce its committed pack bytes (%d vs %d bytes)" % (what, len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack of %s lost content" % what)
    return {"document": document, "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both documents, in both encodings — four files, all four reproduced byte for byte.

    The committed `✉️base` solid's two files were written by the RUST codec, so this implementation
    reproducing them is a cross-language byte agreement, not a codec agreeing with itself. The
    concrete forest's two files were written by THIS implementation from the grammar and the
    protocol, so the Rust codec has to reproduce THOSE — 2 466 real `f64` among them, 98 of which
    have no exponent-free shortest lexeme and are written positionally.
    """
    solid = carrier_pair(ctx, SOLID_DSL, SOLID_PACK, "the committed solid")
    kinds = {edge["curve"]["kind"] for edge in solid["document"]["edges"]}
    if not {"line", "circle", "nurbs"} <= kinds or solid["document"]["faces"][0]["surface"]["kind"] != "nurbs":
        raise AssertionError("the committed solid is the line/circle/NURBS artifact this case describes, but decoded with curve kinds %r" % sorted(kinds))
    forest = carrier_pair(ctx, FOREST_DSL, FOREST_PACK, "the concrete forest")
    shape = forest["document"]
    counts = (len(shape["vertices"]), len(shape["edges"]), len(shape["loops"]), len(shape["faces"]), len(shape["shells"]), len(shape["solids"]))
    if counts != (167, 270, 127, 127, 12, 12):
        raise AssertionError("the concrete forest is the 167/270/127/127/12/12 structure this case describes, but decoded as %r" % (counts,))
    if {edge["curve"]["kind"] for edge in shape["edges"]} != {"nurbs"} or {face["surface"]["kind"] for face in shape["faces"]} != {"plane"}:
        raise AssertionError("the concrete forest carries B-spline edges on planar faces throughout, which this decoding contradicts")
    return Outcome({"solid": solid, "forest": forest})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
