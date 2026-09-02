"""🐍️ Independent Python implementation of the `stdio.semio.value` carrier and its nine-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is
a second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope region;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`:
  `document = artifact-mark "[" hex "," value "," "[" value-node* "]" "]"`,
  `value-node = hex ":" value ","?`, and the tag-prefixed `SemioValue` production
  `Z | B[bit] | I[hex] | F[hex] | S[hex] | Y[hex] | L[…] | M[hexkey:value,…] | R[hex]`;
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio`, which
  for THIS subset declares no separate binary layout at all — the pack payload is the same DSL body
  wrapped in the binary envelope. `pack_bytes` re-encodes the committed `🎒️.pack.semio` byte
  for byte, which is what pins that reading;
* the nine verbs, their `SemioValuePath` addressing (`{"kind":"key","key":…}` /
  `{"kind":"index","index":…}` segments) and their JSON wire form are the committed
  `(before, mutation, after)` specification vectors — this case's own `🧫️fixtures/` for eight of
  them and `…/🧬️mutations/📄set-snapshot/🧪️tests/…/` for `set-snapshot` — plus the committed JSON
  schema `…/🧬️schema/🧬️mutations/🔣️.json`. They settle the two facts a name alone does not:
  `set-map-entry` and `set-node` overwrite an existing key or id IN PLACE and APPEND an absent one,
  and `remove-map-entry`/`remove-list-item`/`remove-node` do not renumber anything else.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two implementations disagree the disagreement is a finding,
not something to tune away.

🧫️ **Provenance of the complex artifact.** `local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio` and
its binary twin were derived ONCE, by `🐍️derive-value-fixture.py` in this ticket's folder, from the
real committed model
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧫️fixtures/🔣️.json`
— 424 KB of real `spatial.modelspace` building geometry — read with Python's own `json` module, an
independent RFC 8259 implementation, with `parse_int`/`parse_float` hooks so every numeric SOURCE
LEXEME survives verbatim into `Int`/`Float`, which is the one property of `SemioValue` a JSON DOM
would otherwise destroy. Each of its four sub-models' `objects` arrays was lifted into a graph NODE
keyed by `<model id>#objects`, with a `Ref` left where the array stood, so the subset's `Ref`/`nodes`
layer carries real content rather than a placeholder while the deep
`models → model → geometry → vertices → position` tree stays inline where a `SemioValuePath` can
address it. `payload-fidelity` re-derives the document from the source on every run, so the fixture
can never silently drift away from the real data it claims to carry.
"""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
ENVELOPE_ID = "stdio.semio.value"
DSL_PREAMBLE = "semio %s.dsl v1" % ENVELOPE_ID
PACK_TOKEN = "%s.pack v1" % ENVELOPE_ID

VALUE_LETTER = {"null": "Z", "bool": "B", "int": "I", "float": "F", "str": "S", "bytes": "Y", "list": "L", "map": "M", "ref": "R"}
LETTER_VALUE = {letter: kind for kind, letter in VALUE_LETTER.items()}


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    return bytes_of(hexed).decode("utf-8")


def bytes_of(hexed: str) -> bytes:
    """🔡️ The same macro read as raw octets — what a `Y[…]` value carries."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed)


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.strip()


# endregion 🔖️Carrier


# region 🔖️Dsl
class Reader:
    """🔎️ A one-character-lookahead cursor. Every `SemioValue` variant is fixed by its leading tag
    letter and every payload is bracket-delimited, so no more lookahead is needed."""

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
            raise AssertionError("the document ends where a tag letter was expected")
        self.at += 1
        return char

    def hex_run(self) -> str:
        start = self.at
        text = self.text
        at = self.at
        limit = len(text)
        while at < limit and text[at] in "0123456789abcdef":
            at += 1
        self.at = at
        return text[start:at]

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text at offset %d: %r" % (self.at, self.text[self.at :]))


def read_value(reader: Reader) -> dict:
    """🌳️ `value = "Z" | "B" "[" bit "]" | "I" "[" hex "]" | … | "R" "[" hex "]"`, recursive through
    `L` and `M`."""
    letter = reader.letter()
    if letter not in LETTER_VALUE:
        raise AssertionError("unknown value tag %r — the grammar declares Z B I F S Y L M R" % letter)
    kind = LETTER_VALUE[letter]
    if kind == "null":
        return {"kind": "null"}
    reader.take("[")
    if kind == "bool":
        bit = reader.letter()
        if bit not in ("0", "1"):
            raise AssertionError("`bit` is 0 or 1, found %r" % bit)
        value = {"kind": "bool", "value": bit == "1"}
    elif kind in ("int", "float"):
        value = {"kind": kind, "lexeme": text_of(reader.hex_run())}
    elif kind == "str":
        value = {"kind": "str", "value": text_of(reader.hex_run())}
    elif kind == "bytes":
        value = {"kind": "bytes", "value": list(bytes_of(reader.hex_run()))}
    elif kind == "list":
        items = []
        while reader.peek() != "]":
            items.append(read_value(reader))
            if reader.peek() == ",":
                reader.take(",")
        value = {"kind": "list", "items": items}
    elif kind == "map":
        entries = []
        while reader.peek() != "]":
            key = text_of(reader.hex_run())
            reader.take(":")
            entries.append({"key": key, "value": read_value(reader)})
            if reader.peek() == ",":
                reader.take(",")
        value = {"kind": "map", "entries": entries}
    else:
        value = {"kind": "ref", "id": {"value": text_of(reader.hex_run())}}
    reader.take("]")
    return value


def write_value(value: dict, out: list) -> None:
    """🌳️ The writing direction of `read_value`, appending into a buffer so a deep document does not
    build one string per node."""
    kind = value["kind"]
    if kind == "null":
        out.append("Z")
        return
    if kind == "bool":
        out.append("B[%d]" % (1 if value["value"] else 0))
        return
    if kind in ("int", "float"):
        out.append("%s[%s]" % (VALUE_LETTER[kind], hex_of(value["lexeme"])))
        return
    if kind == "str":
        out.append("S[%s]" % hex_of(value["value"]))
        return
    if kind == "bytes":
        out.append("Y[%s]" % bytes(value["value"]).hex())
        return
    if kind == "ref":
        out.append("R[%s]" % hex_of(value["id"]["value"]))
        return
    if kind == "list":
        out.append("L[")
        for at, item in enumerate(value["items"]):
            if at:
                out.append(",")
            write_value(item, out)
        out.append("]")
        return
    if kind == "map":
        out.append("M[")
        for at, entry in enumerate(value["entries"]):
            if at:
                out.append(",")
            out.append(hex_of(entry["key"]))
            out.append(":")
            write_value(entry["value"], out)
        out.append("]")
        return
    raise AssertionError("unknown value kind %r" % kind)


def parse_body(body: str) -> dict:
    """📖️ `"[" hex "," value "," "[" value-node* "]" "]"` — schema, root, graph nodes."""
    reader = Reader(body)
    reader.take("[")
    schema = text_of(reader.hex_run())
    if schema != ENVELOPE_ID:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, ENVELOPE_ID))
    reader.take(",")
    root = read_value(reader)
    reader.take(",")
    reader.take("[")
    nodes = []
    while reader.peek() != "]":
        node_id = text_of(reader.hex_run())
        reader.take(":")
        nodes.append({"id": {"value": node_id}, "value": read_value(reader)})
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    reader.take("]")
    reader.done()
    return {"schema": schema, "root": root, "nodes": nodes}


def print_body(document: dict) -> str:
    """✍️ The writing direction of `parse_body`."""
    out = ["[", hex_of(document["schema"]), ","]
    write_value(document["root"], out)
    out.append(",[")
    for at, node in enumerate(document["nodes"]):
        if at:
            out.append(",")
        out.append(hex_of(node["id"]["value"]))
        out.append(":")
        write_value(node["value"], out)
    out.append("]]")
    return "".join(out)


def parse_dsl(text: str) -> dict:
    return parse_body(split_preamble(text))


def print_dsl(document: dict) -> str:
    return "%s\n%s" % (DSL_PREAMBLE, print_body(document))


# endregion 🔖️Dsl


# region 🔖️Pack
def parse_pack(data: bytes) -> dict:
    """📦️ The binary envelope, then the SAME DSL body text — this subset declares no separate binary
    layout, and the committed `🎒️.pack.semio` carries the body verbatim after its token."""
    if data[:8] != BINARY_MAGIC:
        raise AssertionError("the pack file does not start with the semio binary magic")
    if len(data) < 12:
        raise AssertionError("the pack file is truncated inside its envelope")
    token_len = int.from_bytes(data[8:12], "little")
    token = data[12 : 12 + token_len].decode("utf-8")
    if token != PACK_TOKEN:
        raise AssertionError("expected the %r envelope token, got %r" % (PACK_TOKEN, token))
    return parse_body(data[12 + token_len :].decode("utf-8"))


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + print_body(document).encode("utf-8")


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = ("no-mutation", "set-snapshot", "set-value", "set-map-entry", "remove-map-entry", "insert-list-item", "remove-list-item", "set-node", "remove-node")

TAG_OF_KIND = {kind: kind.split("-")[0] + "".join(word.capitalize() for word in kind.split("-")[1:]) for kind in KINDS}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> str:
    tag = mutation.get("mutation")
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag


def descend(root: dict, path: list, verb: str) -> dict:
    """🧭️ Walks a `SemioValuePath` — `{"kind":"key"}` descends into a `map` member and
    `{"kind":"index"}` into a `list` element. An unaddressable step is a refusal."""
    current = root
    for at, segment in enumerate(path):
        if segment.get("kind") == "key":
            if current["kind"] != "map":
                raise AssertionError("%s: path step %d descends by key into a %s" % (verb, at, current["kind"]))
            found = next((entry for entry in current["entries"] if entry["key"] == segment["key"]), None)
            if found is None:
                raise AssertionError("%s: path step %d names the absent key %r" % (verb, at, segment["key"]))
            current = found["value"]
        elif segment.get("kind") == "index":
            if current["kind"] != "list":
                raise AssertionError("%s: path step %d descends by index into a %s" % (verb, at, current["kind"]))
            index = segment["index"]
            if not isinstance(index, int) or index < 0 or index >= len(current["items"]):
                raise AssertionError("%s: path step %d addresses item %r of %d" % (verb, at, index, len(current["items"])))
            current = current["items"][index]
        else:
            raise AssertionError("%s: unknown path segment %r" % (verb, segment))
    return current


def replace_at(document: dict, path: list, value: dict, verb: str) -> None:
    """🎯️ Overwrites the value a path addresses, in place — the root itself when the path is empty."""
    if not path:
        document["root"] = clone(value)
        return
    parent = descend(document["root"], path[:-1], verb)
    last = path[-1]
    if last.get("kind") == "key":
        if parent["kind"] != "map":
            raise AssertionError("%s: the final path step descends by key into a %s" % (verb, parent["kind"]))
        found = next((entry for entry in parent["entries"] if entry["key"] == last["key"]), None)
        if found is None:
            raise AssertionError("%s: the final path step names the absent key %r" % (verb, last["key"]))
        found["value"] = clone(value)
        return
    if last.get("kind") != "index":
        raise AssertionError("%s: unknown path segment %r" % (verb, last))
    if parent["kind"] != "list":
        raise AssertionError("%s: the final path step descends by index into a %s" % (verb, parent["kind"]))
    index = last["index"]
    if not isinstance(index, int) or index < 0 or index >= len(parent["items"]):
        raise AssertionError("%s: the final path step addresses item %r of %d" % (verb, index, len(parent["items"])))
    parent["items"][index] = clone(value)


def node_index(document: dict, node_id: str):
    for at, node in enumerate(document["nodes"]):
        if node["id"]["value"] == node_id:
            return at
    return None


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable path, key, index or node id is
    a refusal, never a silent no-op — a quietly skipped mutation would report as a pass.

    `set-map-entry` and `set-node` overwrite an existing member IN PLACE, keeping its position, and
    APPEND an absent one; that asymmetry is what makes `remove-map-entry`'s undo a multi-step
    sequence rather than one verb."""
    result = clone(document)
    tag = tagged(mutation)
    if tag == "noMutation":
        return result
    if tag == "setSnapshot":
        return clone(mutation["snapshot"])
    if tag == "setValue":
        replace_at(result, mutation["path"], mutation["value"], tag)
        return result
    if tag == "setMapEntry":
        target = descend(result["root"], mutation["path"], tag)
        if target["kind"] != "map":
            raise AssertionError("setMapEntry addresses a %s, not a map" % target["kind"])
        found = next((entry for entry in target["entries"] if entry["key"] == mutation["key"]), None)
        if found is None:
            target["entries"].append({"key": mutation["key"], "value": clone(mutation["value"])})
        else:
            found["value"] = clone(mutation["value"])
        return result
    if tag == "removeMapEntry":
        target = descend(result["root"], mutation["path"], tag)
        if target["kind"] != "map":
            raise AssertionError("removeMapEntry addresses a %s, not a map" % target["kind"])
        at = next((index for index, entry in enumerate(target["entries"]) if entry["key"] == mutation["key"]), None)
        if at is None:
            raise AssertionError("removeMapEntry names the absent key %r" % mutation["key"])
        del target["entries"][at]
        return result
    if tag == "insertListItem":
        target = descend(result["root"], mutation["path"], tag)
        if target["kind"] != "list":
            raise AssertionError("insertListItem addresses a %s, not a list" % target["kind"])
        index = mutation["index"]
        if not isinstance(index, int) or index < 0 or index > len(target["items"]):
            raise AssertionError("insertListItem addresses position %r of %d item(s)" % (index, len(target["items"])))
        target["items"].insert(index, clone(mutation["value"]))
        return result
    if tag == "removeListItem":
        target = descend(result["root"], mutation["path"], tag)
        if target["kind"] != "list":
            raise AssertionError("removeListItem addresses a %s, not a list" % target["kind"])
        index = mutation["index"]
        if not isinstance(index, int) or index < 0 or index >= len(target["items"]):
            raise AssertionError("removeListItem addresses item %r of %d" % (index, len(target["items"])))
        del target["items"][index]
        return result
    if tag == "setNode":
        at = node_index(result, mutation["id"]["value"])
        if at is None:
            result["nodes"].append({"id": clone(mutation["id"]), "value": clone(mutation["value"])})
        else:
            result["nodes"][at]["value"] = clone(mutation["value"])
        return result
    at = node_index(result, mutation["id"]["value"])
    if at is None:
        raise AssertionError("removeNode names the absent node %r" % mutation["id"]["value"])
    del result["nodes"][at]
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to, as a SEQUENCE.

    ⚖️ `set-map-entry` appends an absent key and `set-node` appends an absent id, so restoring a
    REMOVED member at its original position takes one `set-map-entry` per member that followed it —
    a real property of the vocabulary, which is why `remove-map-entry`'s undo is a sequence and why
    `remove-list-item` and `remove-node` address the last member of their collection here and in the
    committed vectors."""
    tag = tagged(mutation)
    if tag == "noMutation":
        return []
    if tag == "setSnapshot":
        return [{"mutation": "setSnapshot", "snapshot": clone(document)}]
    if tag == "setValue":
        return [{"mutation": "setValue", "path": clone(mutation["path"]), "value": clone(descend(document["root"], mutation["path"], tag))}]
    if tag == "setMapEntry":
        target = descend(document["root"], mutation["path"], tag)
        found = next((entry for entry in target["entries"] if entry["key"] == mutation["key"]), None)
        if found is None:
            return [{"mutation": "removeMapEntry", "path": clone(mutation["path"]), "key": mutation["key"]}]
        return [{"mutation": "setMapEntry", "path": clone(mutation["path"]), "key": mutation["key"], "value": clone(found["value"])}]
    if tag == "removeMapEntry":
        target = descend(document["root"], mutation["path"], tag)
        at = next((index for index, entry in enumerate(target["entries"]) if entry["key"] == mutation["key"]), None)
        if at is None:
            raise AssertionError("removeMapEntry names the absent key %r" % mutation["key"])
        return reposition_map(target, at, mutation["path"])
    if tag == "insertListItem":
        return [{"mutation": "removeListItem", "path": clone(mutation["path"]), "index": mutation["index"]}]
    if tag == "removeListItem":
        target = descend(document["root"], mutation["path"], tag)
        return [{"mutation": "insertListItem", "path": clone(mutation["path"]), "index": mutation["index"], "value": clone(target["items"][mutation["index"]])}]
    if tag == "setNode":
        at = node_index(document, mutation["id"]["value"])
        if at is None:
            return [{"mutation": "removeNode", "id": clone(mutation["id"])}]
        return [{"mutation": "setNode", "id": clone(mutation["id"]), "value": clone(document["nodes"][at]["value"])}]
    at = node_index(document, mutation["id"]["value"])
    if at is None:
        raise AssertionError("removeNode names the absent node %r" % mutation["id"]["value"])
    return [{"mutation": "setNode", "id": clone(mutation["id"]), "value": clone(document["nodes"][at]["value"])}]


def reposition_map(target: dict, at: int, path: list) -> list:
    """↩️ Restoring a map member at position `at` when the only writing verb appends: drop every
    member that followed it, then write it and them back in order."""
    tail = target["entries"][at:]
    undo = [{"mutation": "removeMapEntry", "path": clone(path), "key": entry["key"]} for entry in tail[1:]]
    undo += [{"mutation": "setMapEntry", "path": clone(path), "key": entry["key"], "value": clone(entry["value"])} for entry in tail]
    return undo


# endregion 🔖️Mutations


# region 🔖️Derivation
class Lexeme:
    """🔢️ A number as it was SPELLED in the source, handed over by `json`'s `parse_int`/`parse_float`
    hooks before any float conversion can round it."""

    __slots__ = ("text", "is_float")

    def __init__(self, text: str, is_float: bool) -> None:
        self.text = text
        self.is_float = is_float


def value_of_json(node) -> dict:
    """🌳️ One RFC 8259 value as a `SemioValue`. `Lexeme` carries the SOURCE spelling of every number
    verbatim, which is the property `SemioValue` exists to preserve and a plain JSON DOM destroys."""
    if node is None:
        return {"kind": "null"}
    if isinstance(node, bool):
        return {"kind": "bool", "value": node}
    if isinstance(node, Lexeme):
        return {"kind": "float" if node.is_float else "int", "lexeme": node.text}
    if isinstance(node, str):
        return {"kind": "str", "value": node}
    if isinstance(node, list):
        return {"kind": "list", "items": [value_of_json(item) for item in node]}
    if isinstance(node, dict):
        return {"kind": "map", "entries": [{"key": key, "value": value_of_json(item)} for key, item in node.items()]}
    raise AssertionError("no SemioValue for %r" % type(node))


def derive_document_from_json(raw: bytes) -> dict:
    """🌲️ The real committed building model as a `stdio.semio.value` document — the ONE derivation
    that produced `local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio`, re-run by `payload-fidelity`
    so the fixture can never drift from the source.

    It is a faithful transcription with one documented restructuring: each of the source's four
    `models[].model.objects` arrays is lifted into a graph NODE keyed by `<model id>#objects`, and a
    `Ref` to that node is left where the array stood — so the subset's `Ref`/`nodes` layer, which
    JSON has no analogue for, carries real content instead of a placeholder, while the deep
    `models → model → geometry → vertices → position` tree stays inline where a `SemioValuePath` can
    address it."""
    document = json.loads(raw.decode("utf-8"), parse_int=lambda text: Lexeme(text, False), parse_float=lambda text: Lexeme(text, True))
    if not isinstance(document, dict) or "models" not in document:
        raise AssertionError("the source model carries no `models` member")
    nodes = []
    entries = []
    for key, value in document.items():
        if key != "models":
            entries.append({"key": key, "value": value_of_json(value)})
            continue
        models = []
        for model in value:
            node_id = model["id"]
            if not isinstance(node_id, str):
                raise AssertionError("every source model needs a string id")
            lifted = "%s#objects" % node_id
            nodes.append({"id": {"value": lifted}, "value": value_of_json(model["model"]["objects"])})
            body = [{"key": name, "value": {"kind": "ref", "id": {"value": lifted}} if name == "objects" else value_of_json(member)} for name, member in model["model"].items()]
            models.append({"kind": "map", "entries": [{"key": "id", "value": {"kind": "str", "value": node_id}}, {"key": "model", "value": {"kind": "map", "entries": body}}]})
        entries.append({"key": "models", "value": {"kind": "list", "items": models}})
    return {"schema": ENVELOPE_ID, "root": {"kind": "map", "entries": entries}, "nodes": nodes}


# endregion 🔖️Derivation


# region 🔖️Scenario input
GRAPH_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🗣️.dsl.semio"
GRAPH_PACK = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🕸️graph/🖼️assets/🎒️.pack.semio"
FOREST_JSON = "local://🔣️.json"
FOREST_DSL = "local://🧪️hexagonal-cut-concrete-forest/🗣️.dsl.semio"
FOREST_PACK = "local://🧪️hexagonal-cut-concrete-forest/🎒️.pack.semio"


def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own committed parameters — the feature owns them, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def step_fixtures(ctx: Context) -> list:
    """🧫️ Every fixture URI the scenario's steps name, in step order and whatever scheme it uses. The
    feature is the single place a vector path is written down; both adapters read it from there."""
    found = []
    for step in ctx.scenario["steps"]:
        text = step.get("text", "")
        at = 0
        while at < len(text):
            marks = [(text.find(scheme, at), scheme) for scheme in ("local://", "asset://", "shared://")]
            marks = [(where, scheme) for where, scheme in marks if where != -1]
            if not marks:
                break
            where, _ = min(marks)
            end = where
            while end < len(text) and not text[end].isspace():
                end += 1
            found.append(text[where:end])
            at = end
    return found


def forest(ctx: Context) -> dict:
    """🌲️ The real building model, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(FOREST_DSL).decode("utf-8"))


def fixture_json(ctx: Context, uri: str) -> dict:
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real building model by this implementation alone."""
    document = forest(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored document must be the
    building model again — asserted here, and compared against the subject's restored document by
    the runner, so a wrong undo that happens to be self-consistent still shows up."""
    document = forest(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in inverse_mutation(document, mutation):
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("undoing %s did not restore the building model" % ctx.scenario["id"])
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = step_fixtures(ctx)[:3]
    before = fixture_json(ctx, before_uri)
    after = fixture_json(ctx, after_uri)
    mutation = fixture_json(ctx, mutation_uri)
    applied = apply_mutation(before, mutation)
    if applied != after:
        raise AssertionError("%s: the applied document does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    restored = applied
    for step in inverse_mutation(before, mutation):
        restored = apply_mutation(restored, step)
    if restored != before:
        raise AssertionError("%s: undoing the committed mutation did not restore its before-snapshot" % ctx.scenario["id"])
    return Outcome({"applied": applied, "restored": restored})


def payload_fidelity(ctx: Context) -> Outcome:
    """🌲️ The derived fixture against the real JSON it came from, re-read on every run by Python's own
    RFC 8259 parser with lexeme-preserving number hooks."""
    derived = derive_document_from_json(ctx.fixture_bytes(FOREST_JSON))
    committed = forest(ctx)
    if derived != committed:
        raise AssertionError("the committed building document no longer matches the JSON it was derived from")
    return Outcome({"document": derived, "nodes": len(derived["nodes"]), "rootEntries": len(derived["root"]["entries"])})


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the demo graph, plus the real building model, each re-emitted
    from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps it from being vacuous is that each side is measured against bytes the OTHER one
    emitted: the demo graph's two files were written by the Rust codec and are reproduced here from
    the grammar alone, while the building model's two files were written by this implementation and
    the Rust codec has to reproduce THOSE."""
    graph_dsl = ctx.fixture_bytes(GRAPH_DSL)
    graph = parse_dsl(graph_dsl.decode("utf-8"))
    printed = print_dsl(graph).encode("utf-8")
    if printed != graph_dsl:
        raise AssertionError("re-printing the demo graph did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(graph_dsl)))
    graph_pack = ctx.fixture_bytes(GRAPH_PACK)
    if parse_pack(graph_pack) != graph:
        raise AssertionError("the demo graph's binary twin decodes to a different document than its text")
    repacked = pack_bytes(graph)
    if repacked != graph_pack:
        raise AssertionError("re-encoding the demo graph did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(graph_pack)))
    forest_dsl = ctx.fixture_bytes(FOREST_DSL)
    document = parse_dsl(forest_dsl.decode("utf-8"))
    forest_printed = print_dsl(document).encode("utf-8")
    if forest_printed != forest_dsl:
        raise AssertionError("re-printing the building model did not reproduce its committed DSL bytes (%d vs %d bytes)" % (len(forest_printed), len(forest_dsl)))
    committed_forest_pack = ctx.fixture_bytes(FOREST_PACK)
    if parse_pack(committed_forest_pack) != document:
        raise AssertionError("the building model's binary twin decodes to a different document than its text")
    forest_repacked = pack_bytes(document)
    if forest_repacked != committed_forest_pack:
        raise AssertionError("re-encoding the building model did not reproduce its committed pack bytes (%d vs %d bytes)" % (len(forest_repacked), len(committed_forest_pack)))
    return Outcome(
        {
            "graph": graph,
            "graphDslDigest": digest(printed),
            "graphPackDigest": digest(repacked),
            "forestDslDigest": digest(forest_printed),
            "forestPackDigest": digest(forest_repacked),
            "forestNodes": len(document["nodes"]),
            "forestDslLength": len(forest_printed),
            "forestPackLength": len(forest_repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("payload-fidelity", payload_fidelity).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
