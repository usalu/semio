"""🐍️ Independent Python implementation of the `stdio.semio.flow` carrier and its thirteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is
a second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`'s envelope region;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
  (`document = artifact-mark schema-line nodes-line edges-line`, `node = "[" hex "," hex "," hex ","
  "[" param-list? "]" "," "[" number "," number "]" "]"`, `edge = "[" hex "," port "," port "," hex
  "]"`, `port = "[" hex "," hex "]"`, and `number = INT | FLOAT` printed by Rust's own `{}` Display,
  which is why `number_text` below drops a trailing `.0` and refuses exponent notation);
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️component.protocol.semio`
  (`format u8`, then the varint-length-prefixed UTF-8 `schema`), whose description then stops at the
  repeated `nodes`/`edges` records by its own admission and names only their shape — "varint counts,
  per-field length-prefixes, real `f64` LE coordinates". That prose was turned into the reader and
  writer below by taking the field ORDER from the DSL grammar, and the derivation is PINNED against
  the committed `🎒️example.pack.semio`: `pack_bytes` re-encodes that file byte for byte, which a
  misreading could not do;
* the thirteen verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, and what each verb MEANS is the
  committed `(before, mutation, after)` specification vector per kind in this case's own
  `🧫️fixtures/` — including the two facts a name alone does not settle: `insert-node`/`insert-edge`
  carry no index and APPEND, and `remove-node` does NOT cascade into the edges that name it.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two implementations disagree the disagreement is a finding,
not something to tune away.

🧫️ **Provenance of the complex artifact.** `local://🏗️nakagin-capsule-tower.dsl.semio` and its binary
twin were derived ONCE, by `🐍️derive-flow-fixture.py` in this ticket's folder, from the real
committed IFC 4 model `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`
— Kisho Kurokawa's Nakagin Capsule Tower, 2.5 MB and 24 792 entities — read with **IfcOpenShell
0.8.4**, a genuine third-party IFC implementation. Its 180 `IfcBuildingElementProxy` capsules became
the nodes, their `IfcPropertySingleValue` properties the params, their `IfcLocalPlacement`
coordinates the positions, and the 179 `IfcRelConnectsPorts` relations between their 364
`IfcDistributionPort`s became the edges: a real 180-node, 179-edge connection network, not a
synthetic graph.
"""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
ENVELOPE_ID = "stdio.semio.flow"
DSL_PREAMBLE = "semio %s.dsl v1" % ENVELOPE_ID
PACK_TOKEN = "%s.pack v1" % ENVELOPE_ID
PACK_FORMAT = 1


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
    that round-trips, with no trailing `.0` and never in exponent notation. Python's `repr` is the
    same shortest-round-trip algorithm, so only those two spellings have to be corrected."""
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
    """🔎️ A one-character-lookahead cursor. This grammar needs no more: every record is
    bracket-delimited and every leaf is either a `hex` run or a `number`."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.at = 0

    def peek(self) -> str:
        return self.text[self.at] if self.at < len(self.text) else ""

    def take(self, char: str) -> None:
        if self.peek() != char:
            raise AssertionError("expected %r at offset %d, found %r" % (char, self.at, self.peek()))
        self.at += 1

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
    """🧱️ `"[" item-list? "]"` with `,`-separated items — the shape every repeating record takes."""
    reader.take("[")
    found = []
    while reader.peek() != "]":
        found.append(item(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return found


def read_param(reader: Reader) -> dict:
    """🎚️ `param = "[" hex "," hex "]"`."""
    reader.take("[")
    key = reader.hex()
    reader.take(",")
    value = reader.hex()
    reader.take("]")
    return {"key": key, "value": value}


def read_point(reader: Reader) -> dict:
    """📍️ `"[" number "," number "]"` — `SemioPoint2`'s `x` and `y`."""
    reader.take("[")
    x = reader.number()
    reader.take(",")
    y = reader.number()
    reader.take("]")
    return {"x": x, "y": y}


def read_node(reader: Reader) -> dict:
    """🔵️ `node = "[" hex "," hex "," hex "," "[" param-list? "]" "," "[" number "," number "]" "]"`."""
    reader.take("[")
    node = {"id": reader.hex()}
    reader.take(",")
    node["kind"] = reader.hex()
    reader.take(",")
    node["label"] = reader.hex()
    reader.take(",")
    node["params"] = read_sequence(reader, read_param)
    reader.take(",")
    node["position"] = read_point(reader)
    reader.take("]")
    return node


def read_port(reader: Reader) -> dict:
    """🔌️ `port = "[" hex "," hex "]"` — the node it belongs to and the port on it."""
    reader.take("[")
    node = reader.hex()
    reader.take(",")
    port = reader.hex()
    reader.take("]")
    return {"node": node, "port": port}


def read_edge(reader: Reader) -> dict:
    """➡️ `edge = "[" hex "," port "," port "," hex "]"`."""
    reader.take("[")
    edge = {"id": reader.hex()}
    reader.take(",")
    edge["from"] = read_port(reader)
    reader.take(",")
    edge["to"] = read_port(reader)
    reader.take(",")
    edge["kind"] = reader.hex()
    reader.take("]")
    return edge


def read_line(line: str, prefix: str, item) -> list:
    if not line.startswith(prefix + "="):
        raise AssertionError("expected a %r line, found %r" % (prefix, line[:40]))
    reader = Reader(line[len(prefix) + 1 :])
    found = read_sequence(reader, item)
    reader.done()
    return found


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line nodes-line edges-line`, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    if len(body) != 3:
        raise AssertionError("a flow document is exactly a schema, a nodes and an edges line, found %d line(s)" % len(body))
    if not body[0].startswith("schema="):
        raise AssertionError("the first body line must be the schema line, found %r" % body[0])
    schema = text_of(body[0][len("schema=") :])
    if schema != ENVELOPE_ID:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, ENVELOPE_ID))
    return {"schema": schema, "nodes": read_line(body[1], "nodes", read_node), "edges": read_line(body[2], "edges", read_edge)}


def write_param(param: dict) -> str:
    return "[%s,%s]" % (hex_of(param["key"]), hex_of(param["value"]))


def write_node(node: dict) -> str:
    params = ",".join(write_param(param) for param in node["params"])
    return "[%s,%s,%s,[%s],[%s,%s]]" % (hex_of(node["id"]), hex_of(node["kind"]), hex_of(node["label"]), params, number_text(node["position"]["x"]), number_text(node["position"]["y"]))


def write_port(port: dict) -> str:
    return "[%s,%s]" % (hex_of(port["node"]), hex_of(port["port"]))


def write_edge(edge: dict) -> str:
    return "[%s,%s,%s,%s]" % (hex_of(edge["id"]), write_port(edge["from"]), write_port(edge["to"]), hex_of(edge["kind"]))


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    nodes = ",".join(write_node(node) for node in document["nodes"])
    edges = ",".join(write_edge(edge) for edge in document["edges"])
    return "%s\nschema=%s\nnodes=[%s]\nedges=[%s]" % (DSL_PREAMBLE, hex_of(document["schema"]), nodes, edges)


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
    """🧵️ A varint-length-prefixed UTF-8 string, the protocol's only scalar past the header."""
    length, at = read_varint(data, at)
    if at + length > len(data):
        raise AssertionError("the pack frame ends inside a length-prefixed string")
    return data[at : at + length].decode("utf-8"), at + length


def write_string(text: str) -> bytes:
    raw = text.encode("utf-8")
    return write_varint(len(raw)) + raw


def read_double(data: bytes, at: int):
    """📐️ One little-endian IEEE-754 `f64` coordinate."""
    import struct

    if at + 8 > len(data):
        raise AssertionError("the pack frame ends inside a coordinate")
    return struct.unpack_from("<d", data, at)[0], at + 8


def write_double(value: float) -> bytes:
    import struct

    return struct.pack("<d", float(value))


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, the schema, the node records and the edge records."""
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
    nodes = []
    for _ in range(count):
        node = {}
        node["id"], at = read_string(data, at)
        node["kind"], at = read_string(data, at)
        node["label"], at = read_string(data, at)
        param_count, at = read_varint(data, at)
        params = []
        for _ in range(param_count):
            key, at = read_string(data, at)
            value, at = read_string(data, at)
            params.append({"key": key, "value": value})
        node["params"] = params
        x, at = read_double(data, at)
        y, at = read_double(data, at)
        node["position"] = {"x": x, "y": y}
        nodes.append(node)
    count, at = read_varint(data, at)
    edges = []
    for _ in range(count):
        edge = {}
        edge["id"], at = read_string(data, at)
        from_node, at = read_string(data, at)
        from_port, at = read_string(data, at)
        to_node, at = read_string(data, at)
        to_port, at = read_string(data, at)
        edge["from"] = {"node": from_node, "port": from_port}
        edge["to"] = {"node": to_node, "port": to_port}
        edge["kind"], at = read_string(data, at)
        edges.append(edge)
    if at != len(data):
        raise AssertionError("%d trailing byte(s) after the last edge record" % (len(data) - at))
    return {"schema": schema, "nodes": nodes, "edges": edges}


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["nodes"]))
    for node in document["nodes"]:
        body += write_string(node["id"]) + write_string(node["kind"]) + write_string(node["label"])
        body += write_varint(len(node["params"]))
        for param in node["params"]:
            body += write_string(param["key"]) + write_string(param["value"])
        body += write_double(node["position"]["x"]) + write_double(node["position"]["y"])
    body += write_varint(len(document["edges"]))
    for edge in document["edges"]:
        body += write_string(edge["id"])
        body += write_string(edge["from"]["node"]) + write_string(edge["from"]["port"])
        body += write_string(edge["to"]["node"]) + write_string(edge["to"]["port"])
        body += write_string(edge["kind"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = (
    "no-mutation",
    "set-snapshot",
    "insert-node",
    "remove-node",
    "set-node-kind",
    "set-node-label",
    "set-node-position",
    "set-node-param",
    "remove-node-param",
    "insert-edge",
    "remove-edge",
    "set-edge-endpoints",
    "set-edge-kind",
)

#: 🏷️ The internally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors spell it in their `{"mutation": "<tag>", …}` payloads.
TAG_OF_KIND = {kind: kind.split("-")[0] + "".join(word.capitalize() for word in kind.split("-")[1:]) for kind in KINDS}


def tagged(mutation: dict):
    """🔎️ Splits `{"mutation": "setNodeParam", "id": …}` into its verb and the payload itself."""
    tag = mutation.get("mutation")
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation


def clone(value):
    return json.loads(json.dumps(value))


def node_at(document: dict, node_id: str, verb: str) -> int:
    for at, node in enumerate(document["nodes"]):
        if node["id"] == node_id:
            return at
    raise AssertionError("%s addresses the node %r, which this flow does not carry" % (verb, node_id))


def edge_at(document: dict, edge_id: str, verb: str) -> int:
    for at, edge in enumerate(document["edges"]):
        if edge["id"] == edge_id:
            return at
    raise AssertionError("%s addresses the edge %r, which this flow does not carry" % (verb, edge_id))


def param_at(node: dict, key: str):
    for at, param in enumerate(node["params"]):
        if param["key"] == key:
            return at
    return None


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable node, edge or param key is a
    refusal, never a silent no-op — a quietly skipped mutation would report as a pass.

    Two semantics the verb names do not settle, both taken from the committed vectors:
    `insert-node`/`insert-edge` carry no index and APPEND, and `remove-node` does NOT cascade into
    the edges that name it. `set-node-param` overwrites an existing key IN PLACE and appends an
    absent one."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag == "noMutation":
        return result
    if tag == "setSnapshot":
        return clone(args["snapshot"])
    if tag == "insertNode":
        node = clone(args["node"])
        if any(existing["id"] == node["id"] for existing in result["nodes"]):
            raise AssertionError("insertNode would duplicate the existing node %r" % node["id"])
        result["nodes"].append(node)
        return result
    if tag == "removeNode":
        del result["nodes"][node_at(result, args["id"], tag)]
        return result
    if tag == "setNodeKind":
        result["nodes"][node_at(result, args["id"], tag)]["kind"] = args["kind"]
        return result
    if tag == "setNodeLabel":
        result["nodes"][node_at(result, args["id"], tag)]["label"] = args["label"]
        return result
    if tag == "setNodePosition":
        position = args["position"]
        result["nodes"][node_at(result, args["id"], tag)]["position"] = {"x": float(position["x"]), "y": float(position["y"])}
        return result
    if tag == "setNodeParam":
        node = result["nodes"][node_at(result, args["id"], tag)]
        at = param_at(node, args["key"])
        if at is None:
            node["params"].append({"key": args["key"], "value": args["value"]})
        else:
            node["params"][at]["value"] = args["value"]
        return result
    if tag == "removeNodeParam":
        node = result["nodes"][node_at(result, args["id"], tag)]
        at = param_at(node, args["key"])
        if at is None:
            raise AssertionError("removeNodeParam addresses the key %r, which node %r does not carry" % (args["key"], args["id"]))
        del node["params"][at]
        return result
    if tag == "insertEdge":
        edge = clone(args["edge"])
        if any(existing["id"] == edge["id"] for existing in result["edges"]):
            raise AssertionError("insertEdge would duplicate the existing edge %r" % edge["id"])
        result["edges"].append(edge)
        return result
    if tag == "removeEdge":
        del result["edges"][edge_at(result, args["id"], tag)]
        return result
    if tag == "setEdgeEndpoints":
        edge = result["edges"][edge_at(result, args["id"], tag)]
        edge["from"] = clone(args["from"])
        edge["to"] = clone(args["to"])
        return result
    result["edges"][edge_at(result, args["id"], tag)]["kind"] = args["kind"]
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to, as a SEQUENCE.

    ⚖️ `insert-node` and `insert-edge` carry no index, so the undo of a removal can only put the
    record back at the END of its collection. Removing a node or an edge that is not the last one is
    therefore not invertible within this vocabulary — a real property of the vocabulary, not of
    either implementation, and the reason both the committed vectors and this case's `remove-node` /
    `remove-edge` / `remove-node-param` parameters address the last record of their collection."""
    tag, args = tagged(mutation)
    if tag == "noMutation":
        return []
    if tag == "setSnapshot":
        return [{"mutation": "setSnapshot", "snapshot": clone(document)}]
    if tag == "insertNode":
        return [{"mutation": "removeNode", "id": args["node"]["id"]}]
    if tag == "removeNode":
        return [{"mutation": "insertNode", "node": clone(document["nodes"][node_at(document, args["id"], tag)])}]
    if tag == "setNodeKind":
        return [{"mutation": "setNodeKind", "id": args["id"], "kind": document["nodes"][node_at(document, args["id"], tag)]["kind"]}]
    if tag == "setNodeLabel":
        return [{"mutation": "setNodeLabel", "id": args["id"], "label": document["nodes"][node_at(document, args["id"], tag)]["label"]}]
    if tag == "setNodePosition":
        return [{"mutation": "setNodePosition", "id": args["id"], "position": clone(document["nodes"][node_at(document, args["id"], tag)]["position"])}]
    if tag == "setNodeParam":
        node = document["nodes"][node_at(document, args["id"], tag)]
        at = param_at(node, args["key"])
        if at is None:
            return [{"mutation": "removeNodeParam", "id": args["id"], "key": args["key"]}]
        return [{"mutation": "setNodeParam", "id": args["id"], "key": args["key"], "value": node["params"][at]["value"]}]
    if tag == "removeNodeParam":
        node = document["nodes"][node_at(document, args["id"], tag)]
        at = param_at(node, args["key"])
        if at is None:
            raise AssertionError("removeNodeParam addresses the key %r, which node %r does not carry" % (args["key"], args["id"]))
        return [{"mutation": "setNodeParam", "id": args["id"], "key": args["key"], "value": node["params"][at]["value"]}]
    if tag == "insertEdge":
        return [{"mutation": "removeEdge", "id": args["edge"]["id"]}]
    if tag == "removeEdge":
        return [{"mutation": "insertEdge", "edge": clone(document["edges"][edge_at(document, args["id"], tag)])}]
    if tag == "setEdgeEndpoints":
        edge = document["edges"][edge_at(document, args["id"], tag)]
        return [{"mutation": "setEdgeEndpoints", "id": args["id"], "from": clone(edge["from"]), "to": clone(edge["to"])}]
    return [{"mutation": "setEdgeKind", "id": args["id"], "kind": document["edges"][edge_at(document, args["id"], tag)]["kind"]}]


# endregion 🔖️Mutations


# region 🔖️Scenario input
PIPELINE_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio"
PIPELINE_PACK = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🎒️example.pack.semio"
TOWER_DSL = "local://🏗️nakagin-capsule-tower.dsl.semio"
TOWER_PACK = "local://🏗️nakagin-capsule-tower.pack.semio"


def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own committed parameters — the feature owns them, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def step_fixture(ctx: Context, scheme: str) -> str:
    """🧫️ The first `<scheme>://` URI the scenario's steps name. The feature is the single place a
    fixture path is written down; both adapters read it from there."""
    for step in ctx.scenario["steps"]:
        text = step.get("text", "")
        at = text.find(scheme + "://")
        if at != -1:
            end = at
            while end < len(text) and not text[end].isspace():
                end += 1
            return text[at:end]
    raise AssertionError("scenario %s names no %s:// fixture" % (ctx.scenario["id"], scheme))


def tower(ctx: Context) -> dict:
    """🏗️ The real 180-node capsule network, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(TOWER_DSL).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real capsule network by this implementation alone."""
    document = tower(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored flow must be the
    capsule network again — asserted here, and compared against the subject's restored flow by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document = tower(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in inverse_mutation(document, mutation):
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("undoing %s did not restore the capsule network" % ctx.scenario["id"])
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations."""
    vector = json.loads(ctx.fixture_bytes(step_fixture(ctx, "local")).decode("utf-8"))
    applied = apply_mutation(vector["before"], vector["mutation"])
    if applied != vector["after"]:
        raise AssertionError("%s: the applied flow does not match the vector's after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(vector["after"])))
    restored = applied
    for step in inverse_mutation(vector["before"], vector["mutation"]):
        restored = apply_mutation(restored, step)
    if restored != vector["before"]:
        raise AssertionError("%s: undoing the vector's mutation did not restore its before-snapshot" % ctx.scenario["id"])
    return Outcome({"applied": applied, "restored": restored})


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the demo pipeline, plus the real capsule network, each
    re-emitted from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps it from being vacuous is that each side is measured against bytes the OTHER one
    emitted: the pipeline's two files were written by the Rust codec and are reproduced here from the
    grammar alone, while the capsule network's two files were written by this implementation and the
    Rust codec has to reproduce THOSE."""
    pipeline_dsl = ctx.fixture_bytes(PIPELINE_DSL)
    pipeline = parse_dsl(pipeline_dsl.decode("utf-8"))
    printed = print_dsl(pipeline).encode("utf-8")
    if printed != pipeline_dsl:
        raise AssertionError("re-printing the demo pipeline did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(pipeline_dsl)))
    pipeline_pack = ctx.fixture_bytes(PIPELINE_PACK)
    if parse_pack(pipeline_pack) != pipeline:
        raise AssertionError("the demo pipeline's binary twin decodes to a different flow than its text")
    repacked = pack_bytes(pipeline)
    if repacked != pipeline_pack:
        raise AssertionError("re-encoding the demo pipeline did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(pipeline_pack)))
    tower_dsl = ctx.fixture_bytes(TOWER_DSL)
    document = parse_dsl(tower_dsl.decode("utf-8"))
    tower_printed = print_dsl(document).encode("utf-8")
    if tower_printed != tower_dsl:
        raise AssertionError("re-printing the capsule network did not reproduce its committed DSL bytes (%d vs %d bytes)" % (len(tower_printed), len(tower_dsl)))
    committed_tower_pack = ctx.fixture_bytes(TOWER_PACK)
    if parse_pack(committed_tower_pack) != document:
        raise AssertionError("the capsule network's binary twin decodes to a different flow than its text")
    tower_repacked = pack_bytes(document)
    if tower_repacked != committed_tower_pack:
        raise AssertionError("re-encoding the capsule network did not reproduce its committed pack bytes (%d vs %d bytes)" % (len(tower_repacked), len(committed_tower_pack)))
    return Outcome(
        {
            "pipeline": pipeline,
            "pipelineDslDigest": digest(printed),
            "pipelinePackDigest": digest(repacked),
            "towerDslDigest": digest(tower_printed),
            "towerPackDigest": digest(tower_repacked),
            "towerNodes": len(document["nodes"]),
            "towerEdges": len(document["edges"]),
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
