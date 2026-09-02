"""🐍️ Independent Python implementation of the `stdio.semio.model` carrier and its eleven-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks — IfcOpenShell and ruststep read IFC and STEP,
not a semio envelope, and neither can express `set-snapshot` at all — so the second producer THE
STANDARD requires is a second IMPLEMENTATION, written in another language from the format's own
committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope region;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line spatial-line elements-line relations-line`, and the
  `spatial-kind`, `element-class`, `geometry-ref`, `pset-value`, `relation-kind`, `opt-str` and
  `transform` productions it declares, in the order it declares them);
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio`
  (`format u8`, then the varint-length-prefixed UTF-8 `schema`), whose description then stops at the
  repeated `spatial`/`elements`/`relations` records by its own admission and names only their shape —
  "varint counts, per-field length-prefixes, real `f64` LE coordinates, u8 enum tags". That prose was
  turned into the reader and writer below by taking the field ORDER from the DSL grammar and every
  enum ORDINAL from the order that same grammar declares its alternatives in — `S|B|T|P`,
  `WA|SL|CO|BE|DO|WI|RO|ST|FU|OT`, `N|B|M`, `T|N|B`, `AG|CI|CN|FV|VE|OT` — the same rule that turned
  out to be right for `✳️table`'s `Z B I F S Y L M R` value tags. The committed
  `🎒️.pack.semio` pins the ordinals it happens to carry (`S`, `T`, `WA`, `B`-brep, `T`/`N`/`B`
  pset values, `CI`) and `pack_bytes` re-encodes that file byte for byte; the ordinals it does NOT
  carry — `OT`, the `M` geometry reference, an absent `spatialId` — are derived from the grammar's
  declared order alone, are exercised by the real artifact this case mutates, and a disagreement
  about them would surface as a red `identity-round-trip` rather than as a silent one;
* the eleven verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and the committed JSON schema, and what
  each verb MEANS is the committed `(before, mutation, after)` specification vector per kind in this
  case's own `🧫️fixtures/` — including the facts a name alone does not settle: `insert-*` carries no
  index and APPENDS, `remove-*` does NOT cascade into the collections that reference the removed
  member, and every `set-*` slot is tri-state, where a `null` or absent key means "leave untouched".

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two implementations disagree the disagreement is a finding,
not something to tune away.

🧫️ **Provenance of the complex artifact.** `local://🏗️nakagin-capsule-tower.dsl.semio` and its binary
twin were derived ONCE, by `🐍️derive-model-fixture.py` in this ticket's folder, from the real
committed IFC 4 model `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`
— Kisho Kurokawa's Nakagin Capsule Tower, 2.5 MB and 24 792 entities — read with **IfcOpenShell
0.8.4**, a genuine third-party IFC implementation. Its `IfcSite`/`IfcBuilding`/`IfcBuildingStorey`
became the spatial nodes, its `IfcElementAssembly` and 180 `IfcBuildingElementProxy` capsules the
elements, their `IfcPropertySingleValue` properties the property sets, their `IfcLocalPlacement`
axes the placements (real translations and real orientation quaternions), and its `IfcRelAggregates`,
`IfcRelContainedInSpatialStructure` and `IfcRelConnectsElements` the relations.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import struct

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


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
    return {"kind": "brep", "brep_id": value} if letter == "B" else {"kind": "mesh", "mesh_id": value}


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
    return "B[%s]" % hex_of(value["brep_id"]) if value["kind"] == "brep" else "M[%s]" % hex_of(value["mesh_id"])


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
            geometry = {"kind": "brep", "brep_id": reference} if geometry_tag == 1 else {"kind": "mesh", "mesh_id": reference}
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
            body += write_string(geometry["brep_id"])
        elif geometry["kind"] == "mesh":
            body += write_string(geometry["mesh_id"])
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


# region 🔖️Mutations
KINDS = (
    "no-mutation",
    "set-snapshot",
    "insert-spatial-node",
    "remove-spatial-node",
    "set-spatial-node",
    "insert-element",
    "remove-element",
    "set-element",
    "insert-relation",
    "remove-relation",
    "set-relation",
)

TAG_OF_KIND = {kind: kind.split("-")[0] + "".join(word.capitalize() for word in kind.split("-")[1:]) for kind in KINDS}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict):
    tag = mutation.get("mutation")
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag


def index_of(members: list, member_id: str, verb: str, what: str) -> int:
    for at, member in enumerate(members):
        if member["id"] == member_id:
            return at
    raise AssertionError("%s addresses the %s %r, which this model does not carry" % (verb, what, member_id))


def touched(mutation: dict, key: str) -> bool:
    """🎚️ A `set-*` slot is tri-state: an absent key and an explicit `null` both mean untouched, and
    only a present non-null value is a write. The committed vectors spell the untouched state as
    `null` (`set-element`'s `class`/`geometry`, `set-relation`'s `from`/`to`)."""
    return mutation.get(key) is not None


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable member is a refusal, never a
    silent no-op — a quietly skipped mutation would report as a pass.

    Two semantics the verb names do not settle, both taken from the committed vectors: `insert-*`
    carries no index and APPENDS, and `remove-*` does NOT cascade into the collections that reference
    the removed member."""
    result = clone(document)
    tag = tagged(mutation)
    if tag == "noMutation":
        return result
    if tag == "setSnapshot":
        return clone(mutation["snapshot"])
    if tag == "insertSpatialNode":
        node = clone(mutation["node"])
        if any(existing["id"] == node["id"] for existing in result["spatial"]):
            raise AssertionError("insertSpatialNode would duplicate the existing node %r" % node["id"])
        result["spatial"].append(node)
        return result
    if tag == "removeSpatialNode":
        del result["spatial"][index_of(result["spatial"], mutation["id"], tag, "spatial node")]
        return result
    if tag == "setSpatialNode":
        node = result["spatial"][index_of(result["spatial"], mutation["id"], tag, "spatial node")]
        for key in ("kind", "name", "placement"):
            if touched(mutation, key):
                node[key] = clone(mutation[key])
        if "parent_id" in mutation:
            node["parentId"] = mutation["parent_id"]
        return result
    if tag == "insertElement":
        element = clone(mutation["element"])
        if any(existing["id"] == element["id"] for existing in result["elements"]):
            raise AssertionError("insertElement would duplicate the existing element %r" % element["id"])
        result["elements"].append(element)
        return result
    if tag == "removeElement":
        del result["elements"][index_of(result["elements"], mutation["id"], tag, "element")]
        return result
    if tag == "setElement":
        element = result["elements"][index_of(result["elements"], mutation["id"], tag, "element")]
        for key in ("class", "placement", "geometry", "psets"):
            if touched(mutation, key):
                element[key] = clone(mutation[key])
        if "spatial_id" in mutation:
            element["spatialId"] = mutation["spatial_id"]
        return result
    if tag == "insertRelation":
        relation = clone(mutation["relation"])
        if any(existing["id"] == relation["id"] for existing in result["relations"]):
            raise AssertionError("insertRelation would duplicate the existing relation %r" % relation["id"])
        result["relations"].append(relation)
        return result
    if tag == "removeRelation":
        del result["relations"][index_of(result["relations"], mutation["id"], tag, "relation")]
        return result
    relation = result["relations"][index_of(result["relations"], mutation["id"], tag, "relation")]
    for key in ("kind", "from", "to"):
        if touched(mutation, key):
            relation[key] = clone(mutation[key])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to, as a SEQUENCE.

    ⚖️ `insert-*` carries no index, so the undo of a removal can only put the record back at the END
    of its collection. Removing a member that is not the last one of its collection is therefore not
    invertible within this vocabulary — a real property of the vocabulary, not of either
    implementation, and the reason the committed vectors and this case's `remove-*` parameters
    address the last member of their collection."""
    tag = tagged(mutation)
    if tag == "noMutation":
        return []
    if tag == "setSnapshot":
        return [{"mutation": "setSnapshot", "snapshot": clone(document)}]
    if tag == "insertSpatialNode":
        return [{"mutation": "removeSpatialNode", "id": mutation["node"]["id"]}]
    if tag == "removeSpatialNode":
        return [{"mutation": "insertSpatialNode", "node": clone(document["spatial"][index_of(document["spatial"], mutation["id"], tag, "spatial node")])}]
    if tag == "setSpatialNode":
        node = document["spatial"][index_of(document["spatial"], mutation["id"], tag, "spatial node")]
        undo = {"mutation": "setSpatialNode", "id": mutation["id"]}
        for key in ("kind", "name", "placement"):
            if touched(mutation, key):
                undo[key] = clone(node[key])
        if "parent_id" in mutation:
            undo["parent_id"] = node["parentId"]
        return [undo]
    if tag == "insertElement":
        return [{"mutation": "removeElement", "id": mutation["element"]["id"]}]
    if tag == "removeElement":
        return [{"mutation": "insertElement", "element": clone(document["elements"][index_of(document["elements"], mutation["id"], tag, "element")])}]
    if tag == "setElement":
        element = document["elements"][index_of(document["elements"], mutation["id"], tag, "element")]
        undo = {"mutation": "setElement", "id": mutation["id"]}
        for key in ("class", "placement", "geometry", "psets"):
            if touched(mutation, key):
                undo[key] = clone(element[key])
        if "spatial_id" in mutation:
            undo["spatial_id"] = element["spatialId"]
        return [undo]
    if tag == "insertRelation":
        return [{"mutation": "removeRelation", "id": mutation["relation"]["id"]}]
    if tag == "removeRelation":
        return [{"mutation": "insertRelation", "relation": clone(document["relations"][index_of(document["relations"], mutation["id"], tag, "relation")])}]
    relation = document["relations"][index_of(document["relations"], mutation["id"], tag, "relation")]
    undo = {"mutation": "setRelation", "id": mutation["id"]}
    for key in ("kind", "from", "to"):
        if touched(mutation, key):
            undo[key] = clone(relation[key])
    return [undo]


# endregion 🔖️Mutations


# region 🔖️Scenario input
BUILDING_DSL = "asset://📚️examples/🏢️building/🖼️assets/🗣️.dsl.semio"
BUILDING_PACK = "asset://📚️examples/🏢️building/🖼️assets/🎒️.pack.semio"
TOWER_DSL = "local://🏗️nakagin-capsule-tower.dsl.semio"
TOWER_PACK = "local://🏗️nakagin-capsule-tower.pack.semio"


def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own committed parameters — the feature owns them, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def step_fixtures(ctx: Context, scheme: str) -> list:
    """🧫️ Every `<scheme>://` URI the scenario's steps name, in step order — including the ones a
    data table carries, which is how the specification vectors are declared."""
    found = []
    for step in ctx.scenario["steps"]:
        haystacks = [step.get("text", "")] + [cell for row in (step.get("dataTable") or []) for cell in row]
        for text in haystacks:
            at = text.find(scheme + "://")
            while at != -1:
                end = at
                while end < len(text) and not text[end].isspace():
                    end += 1
                found.append(text[at:end])
                at = text.find(scheme + "://", end)
    return found


def tower(ctx: Context) -> dict:
    """🏗️ The real 181-element capsule tower, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(TOWER_DSL).decode("utf-8"))


def fixture_json(ctx: Context, uri: str) -> dict:
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real capsule tower model by this implementation alone."""
    document = tower(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored model must be the
    capsule tower again — asserted here, and compared against the subject's restored model by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document = tower(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in inverse_mutation(document, mutation):
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("undoing %s did not restore the capsule tower model" % ctx.scenario["id"])
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed `(before, mutation, after)` vector, whose before-snapshot is
    the real committed building artifact decoded — a THIRD statement of what the verb means."""
    before_uri, mutation_uri, after_uri = step_fixtures(ctx, "local")[:3]
    before = fixture_json(ctx, before_uri)
    after = fixture_json(ctx, after_uri)
    applied = apply_mutation(before, fixture_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied model does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    restored = applied
    for step in inverse_mutation(before, fixture_json(ctx, mutation_uri)):
        restored = apply_mutation(restored, step)
    if restored != before:
        raise AssertionError("%s: undoing the committed mutation did not restore its before-snapshot" % ctx.scenario["id"])
    return Outcome({"applied": applied, "restored": restored})


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the demo building, plus the real capsule tower, each re-emitted
    from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps it from being vacuous is that each side is measured against bytes the OTHER one
    emitted: the demo building's two files were written by the Rust codec and are reproduced here
    from the grammar alone, while the capsule tower's two files were written by this implementation
    and the Rust codec has to reproduce THOSE — including the `OT` element class, the `M` geometry
    reference and the absent `spatialId`, three tags no committed pack had exercised before."""
    building_dsl = ctx.fixture_bytes(BUILDING_DSL)
    building = parse_dsl(building_dsl.decode("utf-8"))
    printed = print_dsl(building).encode("utf-8")
    if printed != building_dsl:
        raise AssertionError("re-printing the demo building did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(building_dsl)))
    building_pack = ctx.fixture_bytes(BUILDING_PACK)
    if parse_pack(building_pack) != building:
        raise AssertionError("the demo building's binary twin decodes to a different model than its text")
    repacked = pack_bytes(building)
    if repacked != building_pack:
        raise AssertionError("re-encoding the demo building did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(building_pack)))
    tower_dsl = ctx.fixture_bytes(TOWER_DSL)
    document = parse_dsl(tower_dsl.decode("utf-8"))
    tower_printed = print_dsl(document).encode("utf-8")
    if tower_printed != tower_dsl:
        raise AssertionError("re-printing the capsule tower did not reproduce its committed DSL bytes (%d vs %d bytes)" % (len(tower_printed), len(tower_dsl)))
    committed_tower_pack = ctx.fixture_bytes(TOWER_PACK)
    if parse_pack(committed_tower_pack) != document:
        raise AssertionError("the capsule tower's binary twin decodes to a different model than its text")
    tower_repacked = pack_bytes(document)
    if tower_repacked != committed_tower_pack:
        raise AssertionError("re-encoding the capsule tower did not reproduce its committed pack bytes (%d vs %d bytes)" % (len(tower_repacked), len(committed_tower_pack)))
    return Outcome(
        {
            "building": building,
            "buildingDslDigest": digest(printed),
            "buildingPackDigest": digest(repacked),
            "towerDslDigest": digest(tower_printed),
            "towerPackDigest": digest(tower_repacked),
            "towerSpatial": len(document["spatial"]),
            "towerElements": len(document["elements"]),
            "towerRelations": len(document["relations"]),
            "towerDslLength": len(tower_printed),
            "towerPackLength": len(tower_repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
