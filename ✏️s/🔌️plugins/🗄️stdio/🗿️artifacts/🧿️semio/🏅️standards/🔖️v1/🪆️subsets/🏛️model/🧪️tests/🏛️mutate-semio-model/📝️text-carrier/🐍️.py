"""📝️ `stdio.semio.model` text carrier: the envelope constants, the grammar vocabularies with their ordinals and the DSL reader and printer, written from the committed grammar alone."""

from __future__ import annotations


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
ENVELOPE_ID = "stdio.semio.model"
DSL_PREAMBLE = "semio %s.dsl v1" % ENVELOPE_ID
PACK_TOKEN = "%s.pack v1" % ENVELOPE_ID
PACK_FORMAT = 1

#: 🏛️ `spatial-kind = "S" | "B" | "T" | "P"`, in the grammar's own order — which is also the pack's
#: ordinal (`S` → 0 and `T` → 2 are pinned by the committed example).
SPATIAL_ORDER = ("site", "building", "storey", "space")
SPATIAL_LETTER = {"site": "S", "building": "B", "storey": "T", "space": "P"}

#: 🧱️ `element-class = "WA" | "SL" | "CO" | "BE" | "DO" | "WI" | "RO" | "ST" | "FU" | "OT" "[" hex "]"`.
ELEMENT_ORDER = ("wall", "slab", "column", "beam", "door", "window", "roof", "stair", "furniture", "other")
ELEMENT_LETTERS = {"wall": "WA", "slab": "SL", "column": "CO", "beam": "BE", "door": "DO", "window": "WI", "roof": "RO", "stair": "ST", "furniture": "FU", "other": "OT"}

#: 🔗️ `relation-kind = "AG" | "CI" | "CN" | "FV" | "VE" | "OT" "[" hex "]"`.
RELATION_ORDER = ("aggregates", "containedIn", "connectsTo", "fillsVoid", "voidsElement", "other")
RELATION_LETTERS = {"aggregates": "AG", "containedIn": "CI", "connectsTo": "CN", "fillsVoid": "FV", "voidsElement": "VE", "other": "OT"}

#: 📐️ `geometry-ref = "N" | "B" "[" hex "]" | "M" "[" hex "]"` and `pset-value = "T" | "N" | "B"`.
GEOMETRY_ORDER = ("none", "brep", "mesh")
PSET_ORDER = ("text", "number", "boolean")


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def number_text(value) -> str:
    """🔢️ `number = INT | FLOAT`, as Rust's `{}` Display for `f64` writes it: the shortest decimal
    that round-trips, with no trailing `.0` and never in exponent notation."""
    text = repr(float(value))
    if "e" in text or "E" in text or text in ("inf", "-inf", "nan"):
        raise AssertionError("%r has no `number` spelling in this grammar" % value)
    return text[:-2] if text.endswith(".0") else text


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


# endregion 🔖️Carrier


# region 🔖️Dsl
class Reader:
    """🔎️ A two-character-lookahead cursor — `element-class` and `relation-kind` are the only tokens
    wider than one character, and both are exactly two."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.at = 0

    def peek(self) -> str:
        return self.text[self.at] if self.at < len(self.text) else ""

    def take(self, char: str) -> None:
        if self.peek() != char:
            raise AssertionError("expected %r at offset %d, found %r" % (char, self.at, self.peek()))
        self.at += 1

    def letters(self, count: int) -> str:
        if self.at + count > len(self.text):
            raise AssertionError("the document ends where a %d-letter tag was expected" % count)
        found = self.text[self.at : self.at + count]
        self.at += count
        return found

    def hex(self) -> str:
        start = self.at
        while self.peek() in "0123456789abcdef" and self.peek() != "":
            self.at += 1
        return text_of(self.text[start : self.at])

    def number(self) -> float:
        start = self.at
        while self.peek() in "0123456789+-." and self.peek() != "":
            self.at += 1
        raw = self.text[start : self.at]
        if raw == "":
            raise AssertionError("expected a number at offset %d, found %r" % (self.at, self.peek()))
        return float(raw)

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text at offset %d: %r" % (self.at, self.text[self.at :]))


def read_sequence(reader: Reader, item) -> list:
    """🧱️ `"[" item-list? "]"` with `,`-separated items."""
    reader.take("[")
    found = []
    while reader.peek() != "]":
        found.append(item(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return found


def read_opt_str(reader: Reader):
    """🎚️ `opt-str = "[" "0" "]" | "[" "1" "," hex "]"`."""
    reader.take("[")
    tag = reader.letters(1)
    if tag == "0":
        reader.take("]")
        return None
    if tag != "1":
        raise AssertionError("an opt-str tag is 0 or 1, found %r" % tag)
    reader.take(",")
    value = reader.hex()
    reader.take("]")
    return value


def read_point3(reader: Reader) -> dict:
    reader.take("[")
    x = reader.number()
    reader.take(",")
    y = reader.number()
    reader.take(",")
    z = reader.number()
    reader.take("]")
    return {"x": x, "y": y, "z": z}


def read_quat(reader: Reader) -> dict:
    reader.take("[")
    x = reader.number()
    reader.take(",")
    y = reader.number()
    reader.take(",")
    z = reader.number()
    reader.take(",")
    w = reader.number()
    reader.take("]")
    return {"x": x, "y": y, "z": z, "w": w}


def read_transform(reader: Reader) -> dict:
    """📐️ `transform = "[" point3 "," quat "," point3 "]"` — translation, rotation, scale."""
    reader.take("[")
    translation = read_point3(reader)
    reader.take(",")
    rotation = read_quat(reader)
    reader.take(",")
    scale = read_point3(reader)
    reader.take("]")
    return {"translation": translation, "rotation": rotation, "scale": scale}


def read_spatial_node(reader: Reader) -> dict:
    """🏛️ `spatial-node = "[" hex "," spatial-kind "," hex "," opt-str "," transform "]"`."""
    reader.take("[")
    node = {"id": reader.hex()}
    reader.take(",")
    letter = reader.letters(1)
    kind = next((name for name, mark in SPATIAL_LETTER.items() if mark == letter), None)
    if kind is None:
        raise AssertionError("unknown spatial-kind %r — the grammar declares S, B, T, P" % letter)
    node["kind"] = kind
    reader.take(",")
    node["name"] = reader.hex()
    reader.take(",")
    node["parentId"] = read_opt_str(reader)
    reader.take(",")
    node["placement"] = read_transform(reader)
    reader.take("]")
    return node


def read_element_class(reader: Reader) -> dict:
    letters = reader.letters(2)
    kind = next((name for name, mark in ELEMENT_LETTERS.items() if mark == letters), None)
    if kind is None:
        raise AssertionError("unknown element-class %r" % letters)
    if kind != "other":
        return {"kind": kind}
    reader.take("[")
    name = reader.hex()
    reader.take("]")
    return {"kind": "other", "name": name}


def read_geometry_ref(reader: Reader) -> dict:
    letter = reader.letters(1)
    if letter == "N":
        return {"kind": "none"}
    if letter not in ("B", "M"):
        raise AssertionError("unknown geometry-ref %r — the grammar declares N, B, M" % letter)
    reader.take("[")
    value = reader.hex()
    reader.take("]")
    return {"kind": "brep", "brepId": value} if letter == "B" else {"kind": "mesh", "meshId": value}


def read_pset_value(reader: Reader) -> dict:
    letter = reader.letters(1)
    reader.take("[")
    if letter == "T":
        value = {"kind": "text", "value": reader.hex()}
    elif letter == "N":
        value = {"kind": "number", "value": reader.number()}
    elif letter == "B":
        bit = reader.letters(1)
        if bit not in ("0", "1"):
            raise AssertionError("`bit` is 0 or 1, found %r" % bit)
        value = {"kind": "boolean", "value": bit == "1"}
    else:
        raise AssertionError("unknown pset-value %r — the grammar declares T, N, B" % letter)
    reader.take("]")
    return value


def read_property(reader: Reader) -> dict:
    reader.take("[")
    key = reader.hex()
    reader.take(",")
    value = read_pset_value(reader)
    reader.take("]")
    return {"key": key, "value": value}


def read_property_set(reader: Reader) -> dict:
    reader.take("[")
    name = reader.hex()
    reader.take(",")
    properties = read_sequence(reader, read_property)
    reader.take("]")
    return {"name": name, "properties": properties}


def read_element(reader: Reader) -> dict:
    """🧱️ `element = "[" hex "," element-class "," transform "," geometry-ref "," opt-str "," "[" pset-list? "]" "]"`."""
    reader.take("[")
    element = {"id": reader.hex()}
    reader.take(",")
    element["class"] = read_element_class(reader)
    reader.take(",")
    element["placement"] = read_transform(reader)
    reader.take(",")
    element["geometry"] = read_geometry_ref(reader)
    reader.take(",")
    element["spatialId"] = read_opt_str(reader)
    reader.take(",")
    element["psets"] = read_sequence(reader, read_property_set)
    reader.take("]")
    return element


def read_relation(reader: Reader) -> dict:
    """🔗️ `relation = "[" hex "," relation-kind "," hex "," hex "]"`."""
    reader.take("[")
    relation = {"id": reader.hex()}
    reader.take(",")
    letters = reader.letters(2)
    kind = next((name for name, mark in RELATION_LETTERS.items() if mark == letters), None)
    if kind is None:
        raise AssertionError("unknown relation-kind %r" % letters)
    if kind == "other":
        reader.take("[")
        relation["kind"] = {"kind": "other", "label": reader.hex()}
        reader.take("]")
    else:
        relation["kind"] = {"kind": kind}
    reader.take(",")
    relation["from"] = reader.hex()
    reader.take(",")
    relation["to"] = reader.hex()
    reader.take("]")
    return relation


def read_line(line: str, prefix: str, item) -> list:
    if not line.startswith(prefix + "="):
        raise AssertionError("expected a %r line, found %r" % (prefix, line[:40]))
    reader = Reader(line[len(prefix) + 1 :])
    found = read_sequence(reader, item)
    reader.done()
    return found


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line spatial-line elements-line relations-line`."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    if len(body) != 4:
        raise AssertionError("a model document is exactly a schema, a spatial, an elements and a relations line, found %d line(s)" % len(body))
    if not body[0].startswith("schema="):
        raise AssertionError("the first body line must be the schema line, found %r" % body[0])
    schema = text_of(body[0][len("schema=") :])
    if schema != ENVELOPE_ID:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, ENVELOPE_ID))
    return {
        "schema": schema,
        "spatial": read_line(body[1], "spatial", read_spatial_node),
        "elements": read_line(body[2], "elements", read_element),
        "relations": read_line(body[3], "relations", read_relation),
    }


def write_opt_str(value) -> str:
    return "[0]" if value is None else "[1,%s]" % hex_of(value)


def write_transform(transform: dict) -> str:
    translation, rotation, scale = transform["translation"], transform["rotation"], transform["scale"]
    return "[[%s,%s,%s],[%s,%s,%s,%s],[%s,%s,%s]]" % (
        number_text(translation["x"]),
        number_text(translation["y"]),
        number_text(translation["z"]),
        number_text(rotation["x"]),
        number_text(rotation["y"]),
        number_text(rotation["z"]),
        number_text(rotation["w"]),
        number_text(scale["x"]),
        number_text(scale["y"]),
        number_text(scale["z"]),
    )


def write_spatial_node(node: dict) -> str:
    return "[%s,%s,%s,%s,%s]" % (hex_of(node["id"]), SPATIAL_LETTER[node["kind"]], hex_of(node["name"]), write_opt_str(node["parentId"]), write_transform(node["placement"]))


def write_element_class(value: dict) -> str:
    letters = ELEMENT_LETTERS[value["kind"]]
    return "%s[%s]" % (letters, hex_of(value["name"])) if value["kind"] == "other" else letters


def write_geometry_ref(value: dict) -> str:
    if value["kind"] == "none":
        return "N"
    return "B[%s]" % hex_of(value["brepId"]) if value["kind"] == "brep" else "M[%s]" % hex_of(value["meshId"])


def write_pset_value(value: dict) -> str:
    if value["kind"] == "text":
        return "T[%s]" % hex_of(value["value"])
    if value["kind"] == "number":
        return "N[%s]" % number_text(value["value"])
    return "B[%d]" % (1 if value["value"] else 0)


def write_property_set(value: dict) -> str:
    properties = ",".join("[%s,%s]" % (hex_of(prop["key"]), write_pset_value(prop["value"])) for prop in value["properties"])
    return "[%s,[%s]]" % (hex_of(value["name"]), properties)


def write_element(element: dict) -> str:
    psets = ",".join(write_property_set(pset) for pset in element["psets"])
    return "[%s,%s,%s,%s,%s,[%s]]" % (hex_of(element["id"]), write_element_class(element["class"]), write_transform(element["placement"]), write_geometry_ref(element["geometry"]), write_opt_str(element["spatialId"]), psets)


def write_relation(relation: dict) -> str:
    kind = relation["kind"]
    letters = RELATION_LETTERS[kind["kind"]]
    tag = "%s[%s]" % (letters, hex_of(kind["label"])) if kind["kind"] == "other" else letters
    return "[%s,%s,%s,%s]" % (hex_of(relation["id"]), tag, hex_of(relation["from"]), hex_of(relation["to"]))


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    return "%s\nschema=%s\nspatial=[%s]\nelements=[%s]\nrelations=[%s]" % (
        DSL_PREAMBLE,
        hex_of(document["schema"]),
        ",".join(write_spatial_node(node) for node in document["spatial"]),
        ",".join(write_element(element) for element in document["elements"]),
        ",".join(write_relation(relation) for relation in document["relations"]),
    )


# endregion 🔖️Dsl
