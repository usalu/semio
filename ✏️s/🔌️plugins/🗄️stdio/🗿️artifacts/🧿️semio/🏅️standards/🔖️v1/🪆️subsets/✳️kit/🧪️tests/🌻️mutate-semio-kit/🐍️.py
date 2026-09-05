"""🐍️ Independent Python implementation of the `s.stdio.semio.kit` carrier and its fifteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the child handle's and the representation link's `target` string is the ONE dialect-coordinate
  codec in the repository, `<artifact_id>!<kind>@<standard>/<subset>`, specified by
  `ArtifactRef::to_uri`/`parse_uri` in `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`, and the
  three pin shapes are `LinkPin` (`Head`, `Checkpoint { id }`, `Snapshot { blob: BlobRef }`, and
  `BlobRef { hash, size, media_type }`) in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/
  🦀️.rs`;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line types-line designs-line objects-line models-line
  properties-line representations-line`, `pin = "[" "h" "]" | "[" "c" "," hex "]" | "[" "s" "," hex
  "," INT "," hex "]"`, `transform` = ten positional numbers);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`;
* the fifteen verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body's `format u8` + varint-length-prefixed `schema` is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio`, whose prose then names — but declines to
  frame — "types/designs/objects/models/properties/representations (all variable-length
  repeated/optional records)". That named-but-unframed layout was written out here in the order the
  grammar's `document` lists those collections and is PINNED by `pack_bytes` re-encoding the
  committed `🎒️.pack.semio` byte for byte, which a misreading could not do. The committed
  artifact carries only a `head` pin, so the `checkpoint` and `snapshot` arms of the BINARY pin
  codec below are the natural extension of the one that is pinned rather than themselves pinned —
  no scenario in this case round-trips either through the pack.

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
DSL_PREAMBLE = "semio stdio.semio.kit.dsl v1"
PACK_TOKEN = "stdio.semio.kit.pack v1"
DOCUMENT_SCHEMA = "stdio.semio.kit"
PACK_FORMAT = 1

#: 📌️ `pin = "[" "h" "]" | "[" "c" "," hex "]" | "[" "s" "," hex "," INT "," hex "]"` in the
#: grammar's own order, against `LinkPin`'s `Head | Checkpoint | Snapshot` in the same order — which
#: is also the pack ordinal, as the committed example's `head → 0x00` shows.
PIN_ORDER = ("head", "checkpoint", "snapshot")
PIN_LETTER = {"head": "h", "checkpoint": "c", "snapshot": "s"}
LETTER_PIN = {letter: kind for kind, letter in PIN_LETTER.items()}

#: 🏗️ The kit every mutation row runs on: the real Nakagin Capsule Tower as a kit of parts — 12 real
#: element types, one design carrying 180 real capsule pieces with their real placement transforms and
#: 179 real port-to-port connections, and one representation link per type — derived ONCE from the
#: real committed IFC 4 file with IfcOpenShell 0.8.4 by `🐍️derive-kit-fixture.py` in the ticket folder.
TOWER_DSL = "local://🗣️nakagin-capsule-tower.dsl.semio"
TOWER_PACK = "local://🎒️.pack.semio"
#: 🪑️ The tiny committed furniture kit, kept for the BYTE half of the identity law: its two files were
#: written by the RUST codec, so this implementation reproducing them is a cross-language byte
#: agreement the tower pair — written by this implementation — cannot restate.
FURNITURE_DSL = "asset://📚️examples/🪑️furniture/🖼️assets/🗣️.dsl.semio"
FURNITURE_PACK = "asset://📚️examples/🪑️furniture/🖼️assets/🎒️.pack.semio"


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
    fractional part, which is what the committed artifact's `[0,0,0,0,0,0,1,1,1,1]` piece transform
    shows the lexeme convention to be."""
    if value != value or value in (float("inf"), float("-inf")):
        raise AssertionError("the grammar's `number` has no lexeme for %r" % value)
    if value == int(value) and abs(value) < 1e16:
        return str(int(value))
    lexeme = repr(float(value))
    if "e" in lexeme or "E" in lexeme:
        raise AssertionError("%r has no plain-decimal lexeme" % value)
    return lexeme


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


def ref_to_uri(target: dict) -> str:
    """🔗️ `ArtifactRef::to_uri` — `<artifact_id>!<kind>@<standard>/<subset>`."""
    dialect = target["dialect"]
    return "%s!%s@%s/%s" % (target["artifactId"], dialect["artifactKind"], dialect["standard"], dialect["subset"])


def ref_from_uri(uri: str) -> dict:
    """🔗️ `ArtifactRef::parse_uri` — splits on the FIRST `!`, then `@`, then the LAST `/`."""
    artifact_id, separator, coordinate = uri.partition("!")
    if separator == "" or artifact_id == "":
        raise AssertionError("artifact ref uri %r has no artifact id" % uri)
    kind, separator, rest = coordinate.partition("@")
    if separator == "":
        raise AssertionError("artifact ref uri %r is missing '@'" % uri)
    standard, separator, subset = rest.rpartition("/")
    if separator == "" or kind == "" or standard == "" or subset == "":
        raise AssertionError("artifact ref uri %r has an empty dialect component" % uri)
    return {"artifactId": artifact_id, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}


def transform_of(numbers: list) -> dict:
    """📐️ The ten positional transform numbers as the committed vectors name them."""
    return {
        "translation": {"x": numbers[0], "y": numbers[1], "z": numbers[2]},
        "rotation": {"x": numbers[3], "y": numbers[4], "z": numbers[5], "w": numbers[6]},
        "scale": {"x": numbers[7], "y": numbers[8], "z": numbers[9]},
    }


def transform_numbers(transform: dict) -> list:
    """📐️ The inverse of `transform_of` — translation, rotation, scale, in the grammar's order."""
    translation, rotation, scale = transform["translation"], transform["rotation"], transform["scale"]
    return [translation["x"], translation["y"], translation["z"], rotation["x"], rotation["y"], rotation["z"], rotation["w"], scale["x"], scale["y"], scale["z"]]


# endregion 🔖️Carrier


# region 🔖️Dsl
class Reader:
    """🔎️ A one-character-lookahead cursor over one DSL line — enough for a grammar whose every
    production is bracket-delimited and comma-separated."""

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

    def integer(self) -> int:
        start = self.at
        while self.peek() != "" and self.peek() not in ",]":
            self.at += 1
        return int(self.text[start : self.at])

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text: %r" % self.text[self.at :])


def read_items(reader: Reader, reader_of) -> list:
    """📚️ A bracketed, comma-separated record list — the shape every collection line shares."""
    reader.take("[")
    items = []
    while reader.peek() != "]":
        items.append(reader_of(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return items


def read_type(reader: Reader) -> dict:
    """🏷️ `kit-type = "[" hex "," hex "," hex "]"` — id, name, category."""
    reader.take("[")
    type_id = reader.hex()
    reader.take(",")
    name = reader.hex()
    reader.take(",")
    category = reader.hex()
    reader.take("]")
    return {"id": type_id, "name": name, "category": category}


def read_transform(reader: Reader) -> dict:
    """📐️ `transform` — ten positional numbers."""
    reader.take("[")
    numbers = [reader.number()]
    for _ in range(9):
        reader.take(",")
        numbers.append(reader.number())
    reader.take("]")
    return transform_of(numbers)


def read_piece(reader: Reader) -> dict:
    """🧩️ `piece = "[" hex "," hex "," transform "]"`."""
    reader.take("[")
    piece_id = reader.hex()
    reader.take(",")
    type_id = reader.hex()
    reader.take(",")
    transform = read_transform(reader)
    reader.take("]")
    return {"id": piece_id, "typeId": type_id, "transform": transform}


def read_connection(reader: Reader) -> dict:
    """🔗️ `connection = "[" hex "," hex "," hex "," hex "," hex "]"`."""
    reader.take("[")
    fields = [reader.hex()]
    for _ in range(4):
        reader.take(",")
        fields.append(reader.hex())
    reader.take("]")
    return {"id": fields[0], "connectingPieceId": fields[1], "connectingPort": fields[2], "connectedPieceId": fields[3], "connectedPort": fields[4]}


def read_design(reader: Reader) -> dict:
    """🏛️ `design = "[" hex "," hex "," "[" piece-list? "]" "," "[" connection-list? "]" "]"`."""
    reader.take("[")
    design_id = reader.hex()
    reader.take(",")
    name = reader.hex()
    reader.take(",")
    pieces = read_items(reader, read_piece)
    reader.take(",")
    connections = read_items(reader, read_connection)
    reader.take("]")
    return {"id": design_id, "name": name, "pieces": pieces, "connections": connections}


def read_child(reader: Reader) -> dict:
    """🧒️ `child = "[" hex "," hex "]"` — the two-string owned handle."""
    reader.take("[")
    child_id = reader.hex()
    reader.take(",")
    target = ref_from_uri(reader.hex())
    reader.take("]")
    return {"childId": child_id, "target": target}


def read_pin(reader: Reader) -> dict:
    """📌️ `pin` — head, a checkpoint id, or an escrowed blob reference."""
    reader.take("[")
    letter = reader.letter()
    if letter not in LETTER_PIN:
        raise AssertionError("unknown pin tag %r — the grammar declares h, c, s" % letter)
    kind = LETTER_PIN[letter]
    if kind == "head":
        reader.take("]")
        return {"kind": "head"}
    reader.take(",")
    if kind == "checkpoint":
        checkpoint = reader.hex()
        reader.take("]")
        return {"kind": "checkpoint", "id": checkpoint}
    blob_hash = reader.hex()
    reader.take(",")
    size = reader.integer()
    reader.take(",")
    media_type = reader.hex()
    reader.take("]")
    return {"kind": "snapshot", "blob": {"hash": blob_hash, "size": size, "mediaType": media_type}}


def print_pin(pin: dict) -> str:
    """📌️ The writing direction of `pin`."""
    kind = pin["kind"]
    if kind == "head":
        return "[h]"
    if kind == "checkpoint":
        return "[c,%s]" % hex_of(pin["id"])
    blob = pin["blob"]
    return "[s,%s,%d,%s]" % (hex_of(blob["hash"]), blob["size"], hex_of(blob["mediaType"]))


def read_link(reader: Reader) -> dict:
    """🖇️ `link = "[" hex "," pin "," hex "]"` — target uri, pin, role."""
    reader.take("[")
    target = ref_from_uri(reader.hex())
    reader.take(",")
    pin = read_pin(reader)
    reader.take(",")
    role = reader.hex()
    reader.take("]")
    return {"target": target, "pin": pin, "role": role}


def parse_dsl(text: str) -> dict:
    """📖️ The seven body lines of a kit document, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    keys = ["schema", "types", "designs", "objects", "models", "properties", "representations"]
    if len(body) != len(keys):
        raise AssertionError("a kit document is exactly %d body lines, found %d" % (len(keys), len(body)))
    values = []
    for key, line in zip(keys, body):
        if not line.startswith(key + "="):
            raise AssertionError("expected the %r line, found %r" % (key, line))
        values.append(line[len(key) + 1 :])
    schema = text_of(values[0])
    if schema != DOCUMENT_SCHEMA:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, DOCUMENT_SCHEMA))
    document = {"schema": schema}
    for key, raw, reader_of in (("types", values[1], read_type), ("designs", values[2], read_design), ("objects", values[3], read_child), ("models", values[4], read_child)):
        reader = Reader(raw)
        document[key] = read_items(reader, reader_of)
        reader.done()
    reader = Reader(values[5])
    if values[5] == "[]":
        reader.take("[")
        reader.take("]")
    else:
        document["properties"] = read_child(reader)
    reader.done()
    reader = Reader(values[6])
    document["representations"] = read_items(reader, read_link)
    reader.done()
    return document


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    types = ",".join("[%s,%s,%s]" % (hex_of(entry["id"]), hex_of(entry["name"]), hex_of(entry["category"])) for entry in document["types"])
    designs = ",".join(
        "[%s,%s,[%s],[%s]]"
        % (
            hex_of(design["id"]),
            hex_of(design["name"]),
            ",".join("[%s,%s,[%s]]" % (hex_of(piece["id"]), hex_of(piece["typeId"]), ",".join(print_number(value) for value in transform_numbers(piece["transform"]))) for piece in design["pieces"]),
            ",".join(
                "[%s,%s,%s,%s,%s]" % (hex_of(link["id"]), hex_of(link["connectingPieceId"]), hex_of(link["connectingPort"]), hex_of(link["connectedPieceId"]), hex_of(link["connectedPort"]))
                for link in design["connections"]
            ),
        )
        for design in document["designs"]
    )
    children = {key: ",".join("[%s,%s]" % (hex_of(child["childId"]), hex_of(ref_to_uri(child["target"]))) for child in document[key]) for key in ("objects", "models")}
    properties = document.get("properties")
    properties_line = "[]" if not properties else "[%s,%s]" % (hex_of(properties["childId"]), hex_of(ref_to_uri(properties["target"])))
    representations = ",".join("[%s,%s,%s]" % (hex_of(ref_to_uri(link["target"])), print_pin(link["pin"]), hex_of(link["role"])) for link in document["representations"])
    return "\n".join(
        [
            DSL_PREAMBLE,
            "schema=%s" % hex_of(document["schema"]),
            "types=[%s]" % types,
            "designs=[%s]" % designs,
            "objects=[%s]" % children["objects"],
            "models=[%s]" % children["models"],
            "properties=%s" % properties_line,
            "representations=[%s]" % representations,
        ]
    )


# endregion 🔖️Dsl


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


def read_pack_pin(data: bytes, at: int) -> tuple:
    """📌️ One tagged pin, mirroring the text grammar's own `h|c|s` order."""
    ordinal = data[at]
    at += 1
    if ordinal >= len(PIN_ORDER):
        raise AssertionError("unknown pin ordinal %d" % ordinal)
    kind = PIN_ORDER[ordinal]
    if kind == "head":
        return {"kind": "head"}, at
    if kind == "checkpoint":
        checkpoint, at = read_string(data, at)
        return {"kind": "checkpoint", "id": checkpoint}, at
    blob_hash, at = read_string(data, at)
    size, at = read_varint(data, at)
    media_type, at = read_string(data, at)
    return {"kind": "snapshot", "blob": {"hash": blob_hash, "size": size, "mediaType": media_type}}, at


def write_pack_pin(pin: dict) -> bytes:
    """📌️ The writing direction of `read_pack_pin`."""
    out = bytearray([PIN_ORDER.index(pin["kind"])])
    if pin["kind"] == "head":
        return bytes(out)
    if pin["kind"] == "checkpoint":
        return bytes(out + write_string(pin["id"]))
    blob = pin["blob"]
    return bytes(out + write_string(blob["hash"]) + write_varint(blob["size"]) + write_string(blob["mediaType"]))


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


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, the schema, and the six collections in grammar order."""
    body = unwrap_binary(data)
    if body[0] != PACK_FORMAT:
        raise AssertionError("unknown pack format byte %d" % body[0])
    schema, at = read_string(body, 1)
    document = {"schema": schema}
    count, at = read_varint(body, at)
    types = []
    for _ in range(count):
        type_id, at = read_string(body, at)
        name, at = read_string(body, at)
        category, at = read_string(body, at)
        types.append({"id": type_id, "name": name, "category": category})
    document["types"] = types
    count, at = read_varint(body, at)
    designs = []
    for _ in range(count):
        design_id, at = read_string(body, at)
        name, at = read_string(body, at)
        piece_count, at = read_varint(body, at)
        pieces = []
        for _ in range(piece_count):
            piece_id, at = read_string(body, at)
            type_id, at = read_string(body, at)
            numbers = list(struct.unpack_from("<10d", body, at))
            at += 80
            pieces.append({"id": piece_id, "typeId": type_id, "transform": transform_of(numbers)})
        connection_count, at = read_varint(body, at)
        connections = []
        for _ in range(connection_count):
            fields = []
            for _ in range(5):
                field, at = read_string(body, at)
                fields.append(field)
            connections.append({"id": fields[0], "connectingPieceId": fields[1], "connectingPort": fields[2], "connectedPieceId": fields[3], "connectedPort": fields[4]})
        designs.append({"id": design_id, "name": name, "pieces": pieces, "connections": connections})
    document["designs"] = designs
    for key in ("objects", "models"):
        count, at = read_varint(body, at)
        children = []
        for _ in range(count):
            child_id, at = read_string(body, at)
            uri, at = read_string(body, at)
            children.append({"childId": child_id, "target": ref_from_uri(uri)})
        document[key] = children
    present = body[at]
    at += 1
    if present not in (0, 1):
        raise AssertionError("the properties presence byte is %d, not 0 or 1" % present)
    if present:
        child_id, at = read_string(body, at)
        uri, at = read_string(body, at)
        document["properties"] = {"childId": child_id, "target": ref_from_uri(uri)}
    count, at = read_varint(body, at)
    links = []
    for _ in range(count):
        uri, at = read_string(body, at)
        pin, at = read_pack_pin(body, at)
        role, at = read_string(body, at)
        links.append({"target": ref_from_uri(uri), "pin": pin, "role": role})
    document["representations"] = links
    if at != len(body):
        raise AssertionError("%d trailing byte(s) after the last representation" % (len(body) - at))
    return document


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["types"]))
    for entry in document["types"]:
        body += write_string(entry["id"]) + write_string(entry["name"]) + write_string(entry["category"])
    body += write_varint(len(document["designs"]))
    for design in document["designs"]:
        body += write_string(design["id"]) + write_string(design["name"])
        body += write_varint(len(design["pieces"]))
        for piece in design["pieces"]:
            body += write_string(piece["id"]) + write_string(piece["typeId"])
            body += struct.pack("<10d", *transform_numbers(piece["transform"]))
        body += write_varint(len(design["connections"]))
        for link in design["connections"]:
            for field in (link["id"], link["connectingPieceId"], link["connectingPort"], link["connectedPieceId"], link["connectedPort"]):
                body += write_string(field)
    for key in ("objects", "models"):
        body += write_varint(len(document[key]))
        for child in document[key]:
            body += write_string(child["childId"]) + write_string(ref_to_uri(child["target"]))
    properties = document.get("properties")
    if not properties:
        body.append(0)
    else:
        body.append(1)
        body += write_string(properties["childId"]) + write_string(ref_to_uri(properties["target"]))
    body += write_varint(len(document["representations"]))
    for link in document["representations"]:
        body += write_string(ref_to_uri(link["target"]))
        body += write_pack_pin(link["pin"])
        body += write_string(link["role"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = (
    "create-object",
    "delete-object",
    "create-model",
    "delete-model",
    "create-properties",
    "delete-properties",
    "bind-representation",
    "unbind-representation",
    "change-representation-pin",
    "add-type",
    "remove-type",
    "rename-type",
    "add-design",
    "remove-design",
    "edit-design",
)

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-object": "CreateObject",
    "delete-object": "DeleteObject",
    "create-model": "CreateModel",
    "delete-model": "DeleteModel",
    "create-properties": "CreateProperties",
    "delete-properties": "DeleteProperties",
    "bind-representation": "BindRepresentation",
    "unbind-representation": "UnbindRepresentation",
    "change-representation-pin": "ChangeRepresentationPin",
    "add-type": "AddType",
    "remove-type": "RemoveType",
    "rename-type": "RenameType",
    "add-design": "AddDesign",
    "remove-design": "RemoveDesign",
    "edit-design": "EditDesign",
}
CHILD_SLOT_OF_TAG = {"CreateObject": "objects", "DeleteObject": "objects", "CreateModel": "models", "DeleteModel": "models"}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"AddType": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def entry_at(items: list, key: str, value: str, verb: str, what: str) -> int:
    """🔎️ The position of the id-keyed record one verb addresses; absence is a refusal."""
    for index, entry in enumerate(items):
        if entry[key] == value:
            return index
    raise AssertionError("%s addresses %s %r, which the kit does not carry" % (verb, what, value))


def link_index(items: list, index, verb: str) -> int:
    """🔎️ A positional index into the ordered representation pool."""
    if not isinstance(index, int) or index < 0 or index >= len(items):
        raise AssertionError("%s addresses representation %r of a pool holding %d" % (verb, index, len(items)))
    return index


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable id or index is a refusal,
    never a silent no-op — a quietly skipped mutation would report as a pass."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag in CHILD_SLOT_OF_TAG:
        slot = CHILD_SLOT_OF_TAG[tag]
        if tag.startswith("Create"):
            if any(child["childId"] == args["child_id"] for child in result[slot]):
                raise AssertionError("%s uses child id %r, which the %s pool already carries" % (tag, args["child_id"], slot))
            result[slot].append({"childId": args["child_id"], "target": clone(args["target"])})
        else:
            del result[slot][entry_at(result[slot], "childId", args["child_id"], tag, "child")]
        return result
    if tag == "CreateProperties":
        if "properties" in result:
            raise AssertionError("CreateProperties attaches a child to the properties slot, which already carries %r" % result["properties"]["childId"])
        result["properties"] = {"childId": args["child_id"], "target": clone(args["target"])}
    elif tag == "DeleteProperties":
        if "properties" not in result:
            raise AssertionError("DeleteProperties detaches the properties slot, which is already empty")
        del result["properties"]
    elif tag == "BindRepresentation":
        result["representations"].append({"target": clone(args["target"]), "pin": clone(args["pin"]), "role": args["role"]})
    elif tag == "UnbindRepresentation":
        del result["representations"][link_index(result["representations"], args["index"], tag)]
    elif tag == "ChangeRepresentationPin":
        result["representations"][link_index(result["representations"], args["index"], tag)]["pin"] = clone(args["pin"])
    elif tag == "AddType":
        if any(entry["id"] == args["id"] for entry in result["types"]):
            raise AssertionError("AddType uses id %r, which the catalogue already carries" % args["id"])
        result["types"].append({"id": args["id"], "name": args["name"], "category": args["category"]})
    elif tag == "RemoveType":
        del result["types"][entry_at(result["types"], "id", args["id"], tag, "type")]
    elif tag == "RenameType":
        result["types"][entry_at(result["types"], "id", args["id"], tag, "type")]["name"] = args["new_name"]
    elif tag == "AddDesign":
        if any(design["id"] == args["id"] for design in result["designs"]):
            raise AssertionError("AddDesign uses id %r, which the kit already carries" % args["id"])
        result["designs"].append({"id": args["id"], "name": args["name"], "pieces": [], "connections": []})
    elif tag == "RemoveDesign":
        del result["designs"][entry_at(result["designs"], "id", args["id"], tag, "design")]
    else:
        design = result["designs"][entry_at(result["designs"], "id", args["id"], tag, "design")]
        design["pieces"] = clone(args["pieces"])
        design["connections"] = clone(args["connections"])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an append is undone by the matching removal, a removal by re-adding the exact record
    it took out, and a whole-design replacement by replacing it back with what it displaced. A
    removed DESIGN needs two steps, because `add-design` creates an empty one and only `edit-design`
    can put its pieces and connections back."""
    tag, args = tagged(mutation)
    if tag in CHILD_SLOT_OF_TAG:
        slot = CHILD_SLOT_OF_TAG[tag]
        if tag.startswith("Create"):
            return [{"Delete" + tag[len("Create") :]: {"child_id": args["child_id"]}}]
        child = document[slot][entry_at(document[slot], "childId", args["child_id"], tag, "child")]
        return [{"Create" + tag[len("Delete") :]: {"child_id": child["childId"], "target": clone(child["target"])}}]
    if tag == "CreateProperties":
        return [{"DeleteProperties": {}}]
    if tag == "DeleteProperties":
        if "properties" not in document:
            raise AssertionError("DeleteProperties detaches the properties slot, which is already empty")
        child = document["properties"]
        return [{"CreateProperties": {"child_id": child["childId"], "target": clone(child["target"])}}]
    if tag == "BindRepresentation":
        return [{"UnbindRepresentation": {"index": len(document["representations"])}}]
    if tag == "UnbindRepresentation":
        link = document["representations"][link_index(document["representations"], args["index"], tag)]
        return [{"BindRepresentation": {"target": clone(link["target"]), "pin": clone(link["pin"]), "role": link["role"]}}]
    if tag == "ChangeRepresentationPin":
        link = document["representations"][link_index(document["representations"], args["index"], tag)]
        return [{"ChangeRepresentationPin": {"index": args["index"], "pin": clone(link["pin"])}}]
    if tag == "AddType":
        return [{"RemoveType": {"id": args["id"]}}]
    if tag == "RemoveType":
        entry = document["types"][entry_at(document["types"], "id", args["id"], tag, "type")]
        return [{"AddType": {"id": entry["id"], "name": entry["name"], "category": entry["category"]}}]
    if tag == "RenameType":
        entry = document["types"][entry_at(document["types"], "id", args["id"], tag, "type")]
        return [{"RenameType": {"id": entry["id"], "new_name": entry["name"]}}]
    if tag == "AddDesign":
        return [{"RemoveDesign": {"id": args["id"]}}]
    if tag == "RemoveDesign":
        design = document["designs"][entry_at(document["designs"], "id", args["id"], tag, "design")]
        return [
            {"AddDesign": {"id": design["id"], "name": design["name"]}},
            {"EditDesign": {"id": design["id"], "pieces": clone(design["pieces"]), "connections": clone(design["connections"])}},
        ]
    design = document["designs"][entry_at(document["designs"], "id", args["id"], tag, "design")]
    return [{"EditDesign": {"id": design["id"], "pieces": clone(design["pieces"]), "connections": clone(design["connections"])}}]


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
    """🪑️ The real committed furniture kit, put into the state the scenario's verb is defined for by
    the doc string's own `prepare` list, plus the verb itself."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.fixture_bytes(TOWER_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def fixture_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed furniture kit by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored kit must be the
    prepared kit again — asserted here, and compared against the subject's restored kit by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the furniture kit\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
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
    """🔁️ One kit's two encodings, each re-emitted from the parsed document and required back byte
    for byte. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so
    an exact re-emission is the CORRECT answer and the must-differ tripwire would be backwards."""
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
        raise AssertionError("the binary twin of %s decodes to a different kit than its text\n     got: %s\nexpected: %s" % (what, json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding %s did not reproduce its committed pack bytes (%d vs %d bytes)" % (what, len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack of %s lost content" % what)
    return {"document": document, "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both kits, in both encodings — four files, all four reproduced byte for byte.

    The committed furniture kit's two files were written by the RUST codec, so this implementation
    reproducing them is a cross-language byte agreement, not a codec agreeing with itself. The
    capsule tower's two files were written by THIS implementation from the grammar and the protocol,
    so the Rust codec has to reproduce THOSE — 1 800 real `f64` transform components among them.
    """
    furniture = carrier_pair(ctx, FURNITURE_DSL, FURNITURE_PACK, "the committed furniture kit")
    document = furniture["document"]
    if "properties" not in document or not document["representations"] or not document["designs"][0]["connections"]:
        raise AssertionError("the committed furniture kit is the artifact this case describes, but its properties handle, its representation link or its connection decoded as absent")
    tower = carrier_pair(ctx, TOWER_DSL, TOWER_PACK, "the capsule tower kit")
    kit = tower["document"]
    shape = (len(kit["types"]), len(kit["designs"]), len(kit["designs"][0]["pieces"]), len(kit["designs"][0]["connections"]), len(kit["representations"]))
    if shape != (12, 1, 180, 179, 12):
        raise AssertionError("the capsule tower kit is the 12/1/180/179/12 document this case describes, but decoded as %r" % (shape,))
    return Outcome({"furniture": furniture, "tower": tower})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
