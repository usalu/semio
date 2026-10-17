"""📝️ `stdio.semio.graph` text carrier: the envelope constants, the grammar vocabularies with their ordinals and the DSL reader and printer, written from the committed grammar alone."""

from __future__ import annotations

# region 🔖️Imports
import struct
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
