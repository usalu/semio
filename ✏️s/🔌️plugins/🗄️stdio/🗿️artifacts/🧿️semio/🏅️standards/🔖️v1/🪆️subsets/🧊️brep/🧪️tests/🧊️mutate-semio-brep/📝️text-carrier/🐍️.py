"""📝️ `stdio.semio.brep` text carrier: the envelope constants, the grammar vocabularies with their ordinals and the DSL reader and printer, written from the committed grammar alone."""

from __future__ import annotations


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
#: ➰️ `curve2 = "L" … | "C" … | "E" … | "N" …` — a coedge's p-curve, the same tags one dimension down.
CURVE2_ORDER = CURVE_ORDER
LETTER_CURVE2 = LETTER_CURVE

#: 📐️ The scalar members of each tagged arm, in the order the grammar lists them. `p` is a
#: `point3`, `n` a `number`, `P` a `point3-list`, `N` a `number-list` and `i` the integral `degree`.
CURVE_FIELDS = {
    "line": (("origin", "p"), ("direction", "p")),
    "circle": (("center", "p"), ("axis", "p"), ("radius", "n")),
    "ellipse": (("center", "p"), ("axis", "p"), ("radiusMajor", "n"), ("radiusMinor", "n")),
    "nurbs": (("controlPoints", "P"), ("weights", "N"), ("degree", "i"), ("knots", "N")),
}
#: 🗺️ `curve2`'s arms, with `q` a `point2` and `Q` a `point2-list`.
CURVE2_FIELDS = {
    "line": (("origin", "q"), ("direction", "q")),
    "circle": (("center", "q"), ("radius", "n")),
    "ellipse": (("center", "q"), ("xAxis", "q"), ("radiusMajor", "n"), ("radiusMinor", "n")),
    "nurbs": (("controlPoints", "Q"), ("weights", "N"), ("degree", "i"), ("knots", "N")),
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
FOREST_DSL = "shared://🧊️mutate-semio-brep/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio"
FOREST_PACK = "shared://🧊️mutate-semio-brep/🎒️.pack.semio"
#: 🧊️ The tiny committed `✉️base` solid, kept for the BYTE half of the identity law: its two files
#: were written by the RUST codec, so this implementation reproducing them is a cross-language byte
#: agreement the forest pair — written by this implementation — cannot restate.
SOLID_DSL = "asset://🧊️solid/🗣️.dsl.semio"
SOLID_PACK = "asset://🧊️solid/🎒️.pack.semio"


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


def read_point2(reader: Reader) -> dict:
    """📌️ `point2 = "[" number "," number "]"`."""
    reader.take("[")
    x = reader.number()
    reader.take(",")
    y = reader.number()
    reader.take("]")
    return {"x": x, "y": y}


def print_point2(point: dict) -> str:
    return "[%s,%s]" % (print_number(point["x"]), print_number(point["y"]))


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
        elif shape == "q":
            value[name] = read_point2(reader)
        elif shape == "Q":
            value[name] = read_items(reader, read_point2)
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
        elif shape == "q":
            parts.append(print_point2(member))
        elif shape == "Q":
            parts.append("[%s]" % ",".join(print_point2(point) for point in member))
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
    """📍️ `vertex = "[" hex "," point3 "," number "]"` — the number is the vertex's own `tol`."""
    reader.take("[")
    vertex_id = reader.hex()
    reader.take(",")
    point = read_point(reader)
    reader.take(",")
    tol = reader.number()
    reader.take("]")
    return {"id": vertex_id, "point": point, "tol": tol}


def read_edge(reader: Reader) -> dict:
    """➰ `edge = "[" hex "," hex "," hex "," curve "," number "]"` — the number is the edge's own `tol`."""
    reader.take("[")
    edge_id = reader.hex()
    reader.take(",")
    start = reader.hex()
    reader.take(",")
    end = reader.hex()
    reader.take(",")
    curve = read_curve(reader)
    reader.take(",")
    tol = reader.number()
    reader.take("]")
    return {"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve, "tol": tol}


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
    """🔷️ `face = "[" hex "," hex "," hex-list "," surface "," bool "," number "]"` — the number is the face's own `tol`."""
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
    reader.take(",")
    tol = reader.number()
    reader.take("]")
    return {"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation, "tol": tol}


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


def read_coedge(reader: Reader) -> dict:
    """🧱️ `coedge = "[" hex "," hex "," bool "," opt-curve2 "," prange "," hex "," hex "," hex "]"` with
    `opt-curve2 = "-" | "~" curve2` and `prange = "[" number "," number "]"`."""
    reader.take("[")
    coedge_id = reader.hex()
    reader.take(",")
    edge = reader.hex()
    reader.take(",")
    forward = reader.bit()
    reader.take(",")
    if reader.peek() == "-":
        reader.take("-")
        pcurve = None
    else:
        reader.take("~")
        pcurve = read_geometry(reader, LETTER_CURVE2, CURVE2_FIELDS, "curve2")
    reader.take(",")
    reader.take("[")
    start = reader.number()
    reader.take(",")
    end = reader.number()
    reader.take("]")
    reader.take(",")
    loop_id = reader.hex()
    reader.take(",")
    following = reader.hex()
    reader.take(",")
    preceding = reader.hex()
    reader.take("]")
    return {"id": coedge_id, "edge": edge, "forward": forward, "pcurve": pcurve, "prange": [start, end], "loopId": loop_id, "next": following, "prev": preceding}


def print_coedge(coedge: dict) -> str:
    """✍️ The writing direction of `read_coedge`."""
    pcurve = "-" if coedge["pcurve"] is None else "~" + print_geometry(coedge["pcurve"], CURVE_LETTER, CURVE2_FIELDS, "curve2")
    return "[%s,%s,%s,%s,[%s,%s],%s,%s,%s]" % (hex_of(coedge["id"]), hex_of(coedge["edge"]), "1" if coedge["forward"] else "0", pcurve, print_number(coedge["prange"][0]), print_number(coedge["prange"][1]), hex_of(coedge["loopId"]), hex_of(coedge["next"]), hex_of(coedge["prev"]))


COLLECTIONS = (("vertices", read_vertex), ("edges", read_edge), ("loops", read_loop), ("faces", read_face), ("shells", read_shell), ("solids", read_solid), ("coedges", read_coedge))


def parse_dsl(text: str) -> dict:
    """📖️ The nine body lines of a brep document, under the text envelope: the schema, the seven collections and
    `nextLabel`, the native label high-water mark."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    keys = ["schema"] + [name for name, _ in COLLECTIONS] + ["nextLabel"]
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
    for (name, reader_of), raw in zip(COLLECTIONS, values[1:-1]):
        reader = Reader(raw)
        document[name] = read_items(reader, reader_of)
        reader.done()
    if not values[-1].isdigit():
        raise AssertionError("nextLabel is an unsigned integer, found %r" % values[-1])
    document["nextLabel"] = int(values[-1])
    return document


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    vertices = ",".join("[%s,%s,%s]" % (hex_of(vertex["id"]), print_point(vertex["point"]), print_number(vertex["tol"])) for vertex in document["vertices"])
    edges = ",".join("[%s,%s,%s,%s,%s]" % (hex_of(edge["id"]), hex_of(edge["startVertex"]), hex_of(edge["endVertex"]), print_curve(edge["curve"]), print_number(edge["tol"])) for edge in document["edges"])
    loops = ",".join("[%s,[%s]]" % (hex_of(loop["id"]), ",".join("[%s,%s]" % (hex_of(item["edge"]), "1" if item["orientation"] else "0") for item in loop["edges"])) for loop in document["loops"])
    faces = ",".join(
        "[%s,%s,[%s],%s,%s,%s]" % (hex_of(face["id"]), hex_of(face["outerLoop"]), ",".join(hex_of(name) for name in face["innerLoops"]), print_surface(face["surface"]), "1" if face["orientation"] else "0", print_number(face["tol"]))
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
            "coedges=[%s]" % ",".join(print_coedge(coedge) for coedge in document["coedges"]),
            "nextLabel=%d" % document["nextLabel"],
        ]
    )


# endregion 🔖️Dsl
