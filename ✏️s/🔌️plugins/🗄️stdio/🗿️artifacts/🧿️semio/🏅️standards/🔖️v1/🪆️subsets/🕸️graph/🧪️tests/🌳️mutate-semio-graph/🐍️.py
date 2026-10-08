"""🐍️ Independent Python implementation of the `s.stdio.semio.graph` carrier and its eleven-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line nodes-line edges-line`, `node = "[" hex "," hex "," hex ","
  hex "," hex "," hex "," hex "," "[" port-list? "]" "," "[" property-list? "]" "]"` — id, kind, label,
  x, y, width, height —, `port = "[" hex "," port-kind "," hex "," "[" property-list? "]" "]"`,
  `edge = "[" hex "," hex "," hex "," hex "," hex "," optional-text "," optional-text "," "["
  property-list? "]" "]"`, `port-kind = "i" | "o" | "x"`, and the restated `SemioValue` production
  `Z|B|I|F|S|Y|L|M|R`);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`, whose `ports.kind`
  enum is `in|out|inOut` and whose geometry (`position.x`/`position.y`/`width`/`height`) is the
  declared binary64 WORD `{"bits": "<16 lowercase hex digits>"}` — the IEEE 754 bit pattern, so no
  float ever round-trips through a decimal lexeme in the projection — plus `🔢️value`'s own
  `…/🔢️value/🧬️schema/📸️snapshot/🔣️.json` for the `SemioValue` member names (`lexeme`, `value`,
  `items`, `entries`, `id`);
* the eleven verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body's `format u8` + varint-length-prefixed `schema` is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio`, whose prose then names — but declines to
  frame — "per-node varint-length-prefixed id/kind/label strings plus a fixed 16-byte position,
  nested ports/properties lists; per-edge four length-prefixed strings". That named-but-unframed
  layout was written out here from the protocol's own sentence, with the port-kind and value-tag
  ordinals read off the grammar's own `i|o|x` and `Z|B|I|F|S|Y|L|M|R` orders, and is PINNED by
  `pack_bytes` re-encoding the committed `🎒️.pack.semio` byte for byte, which a misreading
  could not do. The committed artifact carries `int` and `str` values only, so the `bytes`/`list`/
  `map`/`ref` arms of the BINARY value codec below are the natural extension of the four that are
  pinned rather than themselves pinned — no scenario in this case exercises them.

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
DSL_PREAMBLE = "semio s.stdio.semio.graph.dsl v1"
PACK_TOKEN = "s.stdio.semio.graph.pack v1"
DOCUMENT_SCHEMA = "s.stdio.semio.graph"
PACK_FORMAT = 1

#: 🔌️ `port-kind = "i" | "o" | "x"` in the grammar's own order, against the projection schema's
#: `in|out|inOut` enum in the same order — which is also the pack ordinal, as the committed
#: example's `out → 0x01` and `in → 0x00` show.
PORT_ORDER = ("in", "out", "inOut")
PORT_LETTER = {"in": "i", "out": "o", "inOut": "x"}
LETTER_PORT = {letter: kind for kind, letter in PORT_LETTER.items()}

#: 🏷️ `value = "Z" | "B" | "I" | "F" | "S" | "Y" | "L" | "M" | "R"` in the grammar's own order,
#: against the value schema's `null|bool|int|float|str|bytes|list|map|ref` enum in the same order —
#: which is also the pack ordinal, as the committed example's `int → 0x02` and `str → 0x04` show.
VALUE_ORDER = ("null", "bool", "int", "float", "str", "bytes", "list", "map", "ref")
VALUE_LETTER = {"null": "Z", "bool": "B", "int": "I", "float": "F", "str": "S", "bytes": "Y", "list": "L", "map": "M", "ref": "R"}
LETTER_VALUE = {letter: kind for kind, letter in VALUE_LETTER.items()}

#: 🏗️ The document every mutation row runs on: the real port-and-connection graph of Kisho
#: Kurokawa's Nakagin Capsule Tower — 181 nodes, 179 edges, 364 ports and 366 typed properties —
#: derived ONCE from the real committed IFC 4 file with IfcOpenShell 0.8.4 by
#: `🐍️derive-graph-fixture.py` in the ticket folder.
TOWER_DSL = "shared://🌳️mutate-semio-graph/🏢️nakagin-capsule-tower/🗣️.dsl.semio"
TOWER_PACK = "shared://🌳️mutate-semio-graph/🎒️.pack.semio"
#: 🕸️ The tiny committed wires graph, kept for the BYTE half of the identity law on a document that
#: carries a property on every node. Both pairs were re-emitted by THIS implementation when the
#: layout grew `width`/`height`, port `category`/`properties` and edge ports/properties (2026-10-04),
#: so the Rust codec reproducing all four files byte for byte is the cross-language agreement.
WIRES_DSL = "asset://🕸️wires/🗣️.dsl.semio"
WIRES_PACK = "asset://🕸️wires/🎒️.pack.semio"


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def print_number(value: float) -> str:
    """🔢️ The `number` lexeme in the writing direction — an integral magnitude prints without a
    fractional part, which is what the committed artifact's `x=hex("0")`/`y=hex("-30.25")` shows the
    convention to be."""
    if value != value or value in (float("inf"), float("-inf")):
        raise AssertionError("there is no lexeme for %r" % value)
    if value == int(value) and abs(value) < 1e16:
        return str(int(value))
    lexeme = repr(float(value))
    if "e" in lexeme or "E" in lexeme:
        raise AssertionError("%r has no plain-decimal lexeme" % value)
    return lexeme


def word(value: float) -> dict:
    """🔢️ A binary64 as the projection's declared geometry word: its IEEE 754 bits, big-endian, lowercase hex."""
    return {"bits": struct.pack(">d", float(value)).hex()}


def number(geometry: dict) -> float:
    """🔢️ The reading direction of `word`."""
    bits = geometry["bits"]
    if len(bits) != 16 or any(digit not in "0123456789abcdef" for digit in bits):
        raise AssertionError("a geometry word is 16 lowercase hex digits, got %r" % bits)
    return struct.unpack(">d", bytes.fromhex(bits))[0]


def point_word(point: dict) -> dict:
    """📍️ A mutation payload's plain `{x, y}` as the projection's pair of words."""
    return {"x": word(point["x"]), "y": word(point["y"])}


def point_number(point: dict) -> dict:
    """📍️ The projection's pair of words as a mutation payload's plain `{x, y}`."""
    return {"x": number(point["x"]), "y": number(point["y"])}


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
    `port-kind`/value tag letter is also a possible hex digit, which the fixed bracket shapes around
    each of them resolve."""

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

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text: %r" % self.text[self.at :])


def read_value(reader: Reader) -> dict:
    """🏷️ One `SemioValue`, tag-prefixed and genuinely recursive through `L`/`M`."""
    letter = reader.letter()
    if letter not in LETTER_VALUE:
        raise AssertionError("unknown value tag %r — the grammar declares Z, B, I, F, S, Y, L, M, R" % letter)
    kind = LETTER_VALUE[letter]
    if kind == "null":
        return {"kind": "null"}
    reader.take("[")
    if kind == "bool":
        bit = reader.letter()
        reader.take("]")
        return {"kind": "bool", "value": bit == "1"}
    if kind in ("int", "float"):
        lexeme = reader.hex()
        reader.take("]")
        return {"kind": kind, "lexeme": lexeme}
    if kind in ("str", "bytes"):
        raw = reader.hex()
        reader.take("]")
        return {"kind": kind, "value": raw}
    if kind == "ref":
        raw = reader.hex()
        reader.take("]")
        return {"kind": "ref", "id": {"value": raw}}
    if kind == "list":
        items = []
        while reader.peek() != "]":
            items.append(read_value(reader))
            if reader.peek() == ",":
                reader.take(",")
        reader.take("]")
        return {"kind": "list", "items": items}
    entries = []
    while reader.peek() != "]":
        key = reader.hex()
        reader.take(":")
        entries.append({"key": key, "value": read_value(reader)})
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return {"kind": "map", "entries": entries}


def print_value(value: dict) -> str:
    """🏷️ The writing direction of the same production."""
    kind = value["kind"]
    if kind not in VALUE_LETTER:
        raise AssertionError("unknown value kind %r" % kind)
    letter = VALUE_LETTER[kind]
    if kind == "null":
        return "Z"
    if kind == "bool":
        return "B[%s]" % ("1" if value["value"] else "0")
    if kind in ("int", "float"):
        return "%s[%s]" % (letter, hex_of(value["lexeme"]))
    if kind in ("str", "bytes"):
        return "%s[%s]" % (letter, hex_of(value["value"]))
    if kind == "ref":
        return "R[%s]" % hex_of(value["id"]["value"])
    if kind == "list":
        return "L[%s]" % ",".join(print_value(item) for item in value["items"])
    return "M[%s]" % ",".join("%s:%s" % (hex_of(entry["key"]), print_value(entry["value"])) for entry in value["entries"])


def read_properties(reader: Reader) -> list:
    """🏷️ `"[" property-list? "]"` with `property = hex ":" value`."""
    reader.take("[")
    properties = []
    while reader.peek() != "]":
        key = reader.hex()
        reader.take(":")
        properties.append({"key": key, "value": read_value(reader)})
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return properties


def print_properties(properties: list) -> str:
    """🏷️ The writing direction of `read_properties`."""
    return "[%s]" % ",".join("%s:%s" % (hex_of(entry["key"]), print_value(entry["value"])) for entry in properties)


def read_port(reader: Reader) -> dict:
    """🔌️ `port = "[" hex "," port-kind "," hex "," "[" property-list? "]" "]"` — name, kind, category, properties."""
    reader.take("[")
    name = reader.hex()
    reader.take(",")
    letter = reader.letter()
    if letter not in LETTER_PORT:
        raise AssertionError("unknown port-kind %r — the grammar declares i, o, x" % letter)
    reader.take(",")
    category = reader.hex()
    reader.take(",")
    properties = read_properties(reader)
    reader.take("]")
    return {"name": name, "kind": LETTER_PORT[letter], "category": category, "properties": properties}


def read_optional_text(reader: Reader):
    """🫥️ `optional-text = "-" | "[" hex "]"`."""
    if reader.peek() == "-":
        reader.take("-")
        return None
    reader.take("[")
    text = reader.hex()
    reader.take("]")
    return text


def print_optional_text(text) -> str:
    """🫥️ The writing direction of `read_optional_text`."""
    return "-" if text is None else "[%s]" % hex_of(text)


def read_node(reader: Reader) -> dict:
    """🔵️ One `node` record — id, kind, label, x, y, width, height, ports, properties."""
    reader.take("[")
    texts = [reader.hex()]
    for _ in range(6):
        reader.take(",")
        texts.append(reader.hex())
    node_id, kind, label, x, y, width, height = texts
    reader.take(",")
    reader.take("[")
    ports = []
    while reader.peek() != "]":
        ports.append(read_port(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    reader.take(",")
    properties = read_properties(reader)
    reader.take("]")
    return {"id": {"value": node_id}, "kind": kind, "label": label, "position": point_word({"x": float(x), "y": float(y)}), "width": word(float(width)), "height": word(float(height)), "ports": ports, "properties": properties}


def read_edge(reader: Reader) -> dict:
    """➡️ `edge = "[" hex "," hex "," hex "," hex "," hex "," optional-text "," optional-text "," "[" property-list? "]" "]"` —
    id, source, target, kind, label, source port, target port, properties. An absent port is no projection member."""
    reader.take("[")
    fields = [reader.hex()]
    for _ in range(4):
        reader.take(",")
        fields.append(reader.hex())
    reader.take(",")
    source_port = read_optional_text(reader)
    reader.take(",")
    target_port = read_optional_text(reader)
    reader.take(",")
    properties = read_properties(reader)
    reader.take("]")
    edge = {"id": {"value": fields[0]}, "source": {"value": fields[1]}, "target": {"value": fields[2]}, "kind": fields[3], "label": fields[4], "properties": properties}
    for member, value in (("sourcePort", source_port), ("targetPort", target_port)):
        if value is not None:
            edge[member] = value
    return edge


def read_list(line: str, reader_of) -> list:
    """📚️ A bracketed, comma-separated record list — the shape both body lines share."""
    reader = Reader(line)
    reader.take("[")
    items = []
    while reader.peek() != "]":
        items.append(reader_of(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    reader.done()
    return items


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line nodes-line edges-line`, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    if len(body) != 3:
        raise AssertionError("a graph document is exactly three body lines, found %d" % len(body))
    values = []
    for key, line in zip(["schema", "nodes", "edges"], body):
        if not line.startswith(key + "="):
            raise AssertionError("expected the %r line, found %r" % (key, line))
        values.append(line[len(key) + 1 :])
    schema = text_of(values[0])
    if schema != DOCUMENT_SCHEMA:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, DOCUMENT_SCHEMA))
    return {"schema": schema, "nodes": read_list(values[1], read_node), "edges": read_list(values[2], read_edge)}


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    nodes = ",".join(
        "[%s,%s,%s,%s,%s,%s,%s,[%s],%s]"
        % (
            hex_of(node["id"]["value"]),
            hex_of(node["kind"]),
            hex_of(node["label"]),
            hex_of(print_number(number(node["position"]["x"]))),
            hex_of(print_number(number(node["position"]["y"]))),
            hex_of(print_number(number(node["width"]))),
            hex_of(print_number(number(node["height"]))),
            ",".join("[%s,%s,%s,%s]" % (hex_of(port["name"]), PORT_LETTER[port["kind"]], hex_of(port["category"]), print_properties(port["properties"])) for port in node["ports"]),
            print_properties(node["properties"]),
        )
        for node in document["nodes"]
    )
    edges = ",".join(
        "[%s,%s,%s,%s,%s,%s,%s,%s]"
        % (hex_of(edge["id"]["value"]), hex_of(edge["source"]["value"]), hex_of(edge["target"]["value"]), hex_of(edge["kind"]), hex_of(edge["label"]), print_optional_text(edge.get("sourcePort")), print_optional_text(edge.get("targetPort")), print_properties(edge["properties"]))
        for edge in document["edges"]
    )
    return "%s\nschema=%s\nnodes=[%s]\nedges=[%s]" % (DSL_PREAMBLE, hex_of(document["schema"]), nodes, edges)


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


#: 🧱️ The incident edges ONE `delete-node` severs at most. The committed leaf payload schema
#: `…/🧬️mutations/🗑️delete-node/🧬️schema/🔣️.json` declares `x-semio-inverse-rows: {"bounded": 1025}`: the undo is the
#: row that restores the node plus one row per severed edge, so the 1025th incident edge makes the delete a refusal
#: (`mutation.target-referenced`) that changes nothing and has nothing to undo.
DELETE_NODE_CASCADE_EDGES_MAXIMUM = 1024

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-node": "CreateNode",
    "delete-node": "DeleteNode",
    "change-node-kind": "ChangeNodeKind",
    "change-node-label": "ChangeNodeLabel",
    "move-node": "MoveNode",
    "add-node-port": "AddNodePort",
    "remove-node-port": "RemoveNodePort",
    "add-node-property": "AddNodeProperty",
    "remove-node-property": "RemoveNodeProperty",
    "create-edge": "CreateEdge",
    "delete-edge": "DeleteEdge",
    "drag-nodes": "DragNodes",
    "set-node-property": "SetNodeProperty",
    "resize-node": "ResizeNode",
    "rename-node": "RenameNode",
    "set-edge-property": "SetEdgeProperty",
    "add-edge-property": "AddEdgeProperty",
    "remove-edge-property": "RemoveEdgeProperty",
}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"CreateNode": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def node_at(document: dict, node_id: dict, verb: str) -> dict:
    """🔎️ The node one verb addresses. An id no node carries is a refusal, never a no-op."""
    for node in document["nodes"]:
        if node["id"] == node_id:
            return node
    raise AssertionError("%s addresses node %r, which the graph does not carry" % (verb, node_id))


def slot_index(items: list, index, verb: str, what: str, inclusive: bool) -> int:
    """🔎️ A positional index into a node's own ordered `ports`/`properties` list."""
    limit = len(items) if inclusive else len(items) - 1
    if not isinstance(index, int) or index < 0 or index > limit:
        raise AssertionError("%s addresses %s %r of a node carrying %d" % (verb, what, index, len(items)))
    return index


def insert_index(items: list, at) -> int:
    """📍️ Where a created record lands: its `at` clamped to the end, or the end when `at` is absent."""
    return len(items) if at is None else min(at, len(items))


def property_of(owner: dict, key: str, verb: str) -> dict:
    """🔎️ The keyed property one verb addresses on a node or an edge. A key it does not carry is a refusal, never a no-op."""
    for entry in owner.get("properties", []):
        if entry["key"] == key:
            return entry
    raise AssertionError("%s addresses property %r, which %r does not carry" % (verb, key, owner["id"]))


def edge_at(document: dict, edge_id: dict, verb: str) -> dict:
    """🔎️ The edge one verb addresses. An id no edge carries is a refusal, never a no-op."""
    for edge in document["edges"]:
        if edge["id"] == edge_id:
            return edge
    raise AssertionError("%s addresses edge %r, which the graph does not carry" % (verb, edge_id))


def severed_edges(document: dict, node_id: dict, verb: str) -> list:
    """✂️ `(index, edge)` of every edge a `delete-node` of `node_id` severs, in document order. More than
    `DELETE_NODE_CASCADE_EDGES_MAXIMUM` of them is a refusal, never a partial cascade."""
    severed = [(at, edge) for at, edge in enumerate(document["edges"]) if edge["source"] == node_id or edge["target"] == node_id]
    if len(severed) > DELETE_NODE_CASCADE_EDGES_MAXIMUM:
        raise AssertionError("%s addresses node %r, which %d edges still reference — one delete severs at most %d" % (verb, node_id, len(severed), DELETE_NODE_CASCADE_EDGES_MAXIMUM))
    return severed


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. `delete-node` CASCADES into every edge with
    that node as source or target, which is the behaviour the committed `removes-the-sink-node-and-
    severs-the-edge-into-it` vector pins. An unaddressable id or index is a refusal, never a silent
    no-op — a quietly skipped mutation would report as a pass."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag == "CreateNode":
        if any(node["id"] == args["id"] for node in result["nodes"]):
            raise AssertionError("CreateNode uses id %r, which the graph already carries" % args["id"])
        created = {"id": clone(args["id"]), "kind": args["kind"], "label": args["label"], "position": point_word(args["position"]), "width": word(args["width"]), "height": word(args["height"]), "ports": clone(args["ports"]), "properties": clone(args["properties"])}
        result["nodes"].insert(insert_index(result["nodes"], args.get("at")), created)
    elif tag == "DeleteNode":
        node_at(result, args["id"], tag)
        severed = {at for at, _ in severed_edges(result, args["id"], tag)}
        result["nodes"] = [node for node in result["nodes"] if node["id"] != args["id"]]
        result["edges"] = [edge for at, edge in enumerate(result["edges"]) if at not in severed]
    elif tag == "ChangeNodeKind":
        node_at(result, args["id"], tag)["kind"] = args["new_kind"]
    elif tag == "ChangeNodeLabel":
        node_at(result, args["id"], tag)["label"] = args["new_label"]
    elif tag == "MoveNode":
        node_at(result, args["id"], tag)["position"] = point_word(args["new_position"])
    elif tag == "AddNodePort":
        ports = node_at(result, args["node_id"], tag)["ports"]
        ports.insert(slot_index(ports, args["index"], tag, "port", True), clone(args["port"]))
    elif tag == "RemoveNodePort":
        ports = node_at(result, args["node_id"], tag)["ports"]
        del ports[slot_index(ports, args["index"], tag, "port", False)]
    elif tag == "AddNodeProperty":
        properties = node_at(result, args["node_id"], tag)["properties"]
        properties.insert(slot_index(properties, args["index"], tag, "property", True), clone(args["property"]))
    elif tag == "RemoveNodeProperty":
        node = node_at(result, args["node_id"], tag)
        node["properties"].remove(property_of(node, args["key"], tag))
    elif tag == "DragNodes":
        present = [node for node in result["nodes"] if node["id"] in args["targets"]]
        if not present:
            raise AssertionError("DragNodes addresses none of the graph's nodes: %r" % args["targets"])
        for node in present:
            node["position"] = point_word({"x": number(node["position"]["x"]) + args["dx"], "y": number(node["position"]["y"]) + args["dy"]})
    elif tag == "SetNodeProperty":
        property_of(node_at(result, args["node_id"], tag), args["key"], tag)["value"] = clone(args["value"])
    elif tag == "ResizeNode":
        if not all(isinstance(args[side], (int, float)) and args[side] >= 0 for side in ("width", "height")):
            raise AssertionError("ResizeNode needs a finite non-negative size, got %r" % args)
        node = node_at(result, args["id"], tag)
        node["width"], node["height"] = word(args["width"]), word(args["height"])
    elif tag == "RenameNode":
        node = node_at(result, args["id"], tag)
        if not args["new_id"]["value"] or any(other["id"] == args["new_id"] for other in result["nodes"]):
            raise AssertionError("RenameNode needs a free non-empty id, got %r" % args["new_id"])
        node["id"] = clone(args["new_id"])
        for edge in result["edges"]:
            for end in ("source", "target"):
                if edge[end] == args["id"]:
                    edge[end] = clone(args["new_id"])
    elif tag == "SetEdgeProperty":
        property_of(edge_at(result, args["edge_id"], tag), args["key"], tag)["value"] = clone(args["value"])
    elif tag == "AddEdgeProperty":
        properties = edge_at(result, args["edge_id"], tag)["properties"]
        if any(entry["key"] == args["property"]["key"] for entry in properties):
            raise AssertionError("AddEdgeProperty attaches key %r, which the edge already carries" % args["property"]["key"])
        properties.insert(min(args["index"], len(properties)), clone(args["property"]))
    elif tag == "RemoveEdgeProperty":
        edge = edge_at(result, args["edge_id"], tag)
        edge["properties"].remove(property_of(edge, args["key"], tag))
    elif tag == "CreateEdge":
        if any(edge["id"] == args["id"] for edge in result["edges"]):
            raise AssertionError("CreateEdge uses id %r, which the graph already carries" % args["id"])
        created = {"id": clone(args["id"]), "source": clone(args["source"]), "target": clone(args["target"]), "kind": args["kind"], "label": args["label"], "properties": clone(args.get("properties", []))}
        for member, key in (("sourcePort", "source_port"), ("targetPort", "target_port")):
            if args.get(key) is not None:
                created[member] = args[key]
        result["edges"].insert(insert_index(result["edges"], args.get("at")), created)
    else:
        if not any(edge["id"] == args["id"] for edge in result["edges"]):
            raise AssertionError("DeleteEdge addresses edge %r, which the graph does not carry" % args["id"])
        result["edges"] = [edge for edge in result["edges"] if edge["id"] != args["id"]]
    return result


def edge_mutation(edge: dict, at: int) -> dict:
    """➡️ The `CreateEdge` that puts one edge back exactly as it was, at the index it held."""
    return {"CreateEdge": {"id": clone(edge["id"]), "source": clone(edge["source"]), "target": clone(edge["target"]), "kind": edge["kind"], "label": edge["label"], "source_port": edge.get("sourcePort"), "target_port": edge.get("targetPort"), "properties": clone(edge["properties"]), "at": at}}


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an append is undone by the matching delete, an overwrite by an overwrite with the
    value it displaced, and a cascading delete by re-creating the node AND every edge it severed."""
    tag, args = tagged(mutation)
    if tag == "CreateNode":
        return [{"DeleteNode": {"id": clone(args["id"])}}]
    if tag == "DeleteNode":
        node = node_at(document, args["id"], tag)
        steps = [{"CreateNode": {"id": clone(node["id"]), "kind": node["kind"], "label": node["label"], "position": point_number(node["position"]), "width": number(node["width"]), "height": number(node["height"]), "ports": clone(node["ports"]), "properties": clone(node["properties"]), "at": document["nodes"].index(node)}}]
        steps.extend(edge_mutation(edge, at) for at, edge in severed_edges(document, args["id"], tag))
        return steps
    if tag == "ChangeNodeKind":
        return [{"ChangeNodeKind": {"id": clone(args["id"]), "new_kind": node_at(document, args["id"], tag)["kind"]}}]
    if tag == "ChangeNodeLabel":
        return [{"ChangeNodeLabel": {"id": clone(args["id"]), "new_label": node_at(document, args["id"], tag)["label"]}}]
    if tag == "MoveNode":
        return [{"MoveNode": {"id": clone(args["id"]), "new_position": point_number(node_at(document, args["id"], tag)["position"])}}]
    if tag == "AddNodePort":
        return [{"RemoveNodePort": {"node_id": clone(args["node_id"]), "index": args["index"]}}]
    if tag == "RemoveNodePort":
        ports = node_at(document, args["node_id"], tag)["ports"]
        index = slot_index(ports, args["index"], tag, "port", False)
        return [{"AddNodePort": {"node_id": clone(args["node_id"]), "index": index, "port": clone(ports[index])}}]
    if tag == "AddNodeProperty":
        return [{"RemoveNodeProperty": {"node_id": clone(args["node_id"]), "key": args["property"]["key"]}}]
    if tag == "RemoveNodeProperty":
        node = node_at(document, args["node_id"], tag)
        entry = property_of(node, args["key"], tag)
        return [{"AddNodeProperty": {"node_id": clone(args["node_id"]), "index": node["properties"].index(entry), "property": clone(entry)}}]
    if tag == "DragNodes":
        seen = []
        for target in args["targets"]:
            if target not in seen:
                seen.append(target)
        return [{"MoveNode": {"id": clone(node["id"]), "new_position": point_number(node["position"])}} for target in seen for node in document["nodes"] if node["id"] == target]
    if tag == "SetNodeProperty":
        return [{"SetNodeProperty": {"node_id": clone(args["node_id"]), "key": args["key"], "value": clone(property_of(node_at(document, args["node_id"], tag), args["key"], tag)["value"])}}]
    if tag == "ResizeNode":
        node = node_at(document, args["id"], tag)
        return [{"ResizeNode": {"id": clone(args["id"]), "width": number(node["width"]), "height": number(node["height"])}}]
    if tag == "RenameNode":
        return [{"RenameNode": {"id": clone(args["new_id"]), "new_id": clone(args["id"])}}]
    if tag == "SetEdgeProperty":
        return [{"SetEdgeProperty": {"edge_id": clone(args["edge_id"]), "key": args["key"], "value": clone(property_of(edge_at(document, args["edge_id"], tag), args["key"], tag)["value"])}}]
    if tag == "AddEdgeProperty":
        return [{"RemoveEdgeProperty": {"edge_id": clone(args["edge_id"]), "key": args["property"]["key"]}}]
    if tag == "RemoveEdgeProperty":
        edge = edge_at(document, args["edge_id"], tag)
        entry = property_of(edge, args["key"], tag)
        return [{"AddEdgeProperty": {"edge_id": clone(args["edge_id"]), "index": edge["properties"].index(entry), "property": clone(entry)}}]
    if tag == "CreateEdge":
        return [{"DeleteEdge": {"id": clone(args["id"])}}]
    for at, edge in enumerate(document["edges"]):
        if edge["id"] == args["id"]:
            return [edge_mutation(edge, at)]
    raise AssertionError("DeleteEdge addresses edge %r, which the graph does not carry" % args["id"])


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


def prepared(ctx: Context) -> tuple:
    """🕸️ The real committed wires graph, plus the scenario's own verb."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.input_bytes(TOWER_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def snapshot_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed wires graph by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored graph must be the
    wires graph again — asserted here, and compared against the subject's restored graph by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the wires graph\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector. The vector
    is a THIRD statement of what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = ctx.step_input_uris()[:3]
    before = snapshot_json(ctx, before_uri)
    after = snapshot_json(ctx, after_uri)
    applied = apply_mutation(before, snapshot_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    return Outcome(applied)


def carrier_pair(ctx: Context, dsl_uri: str, pack_uri: str, what: str) -> dict:
    """🔁️ One document's two encodings, each re-emitted from the parsed document and required back
    byte for byte. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    twin, so an exact re-emission is the CORRECT answer and the must-differ tripwire would be
    backwards here."""
    dsl_bytes = ctx.input_bytes(dsl_uri)
    document = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(document).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("re-printing %s did not reproduce its committed DSL bytes (%d vs %d bytes)" % (what, len(printed), len(dsl_bytes)))
    if parse_dsl(printed.decode("utf-8")) != document:
        raise AssertionError("re-parsing the printed %s lost content" % what)
    committed_pack = ctx.input_bytes(pack_uri)
    unpacked = parse_pack(committed_pack)
    if unpacked != document:
        raise AssertionError("the binary twin of %s decodes to a different graph than its text\n     got: %s\nexpected: %s" % (what, json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding %s did not reproduce its committed pack bytes (%d vs %d bytes)" % (what, len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack of %s lost content" % what)
    return {"document": document, "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both graphs, in both encodings — four files, all four reproduced byte for byte.

    All four files were written by THIS implementation from the grammar and the protocol, so the
    Rust codec has to reproduce THEM — 364 ports and 366 typed properties among the tower's — which
    is a cross-language byte agreement, not a codec agreeing with itself.
    """
    wires = carrier_pair(ctx, WIRES_DSL, WIRES_PACK, "the committed wires graph")
    if len(wires["document"]["nodes"]) != 2 or len(wires["document"]["edges"]) != 1:
        raise AssertionError("the committed wires graph is the two-node one-edge artifact this case describes, but decoded as %d node(s) and %d edge(s)" % (len(wires["document"]["nodes"]), len(wires["document"]["edges"])))
    tower = carrier_pair(ctx, TOWER_DSL, TOWER_PACK, "the capsule tower graph")
    nodes, edges = tower["document"]["nodes"], tower["document"]["edges"]
    ports = [port for node in nodes for port in node["ports"]]
    properties = [entry for node in nodes for entry in node["properties"]]
    if (len(nodes), len(edges), len(ports), len(properties)) != (181, 179, 364, 366):
        raise AssertionError("the capsule tower graph is the 181/179/364/366 document this case describes, but decoded as %d/%d/%d/%d" % (len(nodes), len(edges), len(ports), len(properties)))
    if {port["kind"] for port in ports} != {"in", "out", "inOut"}:
        raise AssertionError("the capsule tower graph carries all three port directions, which this decoding contradicts")
    return Outcome({"wires": wires, "tower": tower})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
