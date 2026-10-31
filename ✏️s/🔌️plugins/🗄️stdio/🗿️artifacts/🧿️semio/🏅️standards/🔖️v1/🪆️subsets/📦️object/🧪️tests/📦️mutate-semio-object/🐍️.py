"""🐍️ Independent Python implementation of the `s.stdio.semio.object` carrier and its nine-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the child target retains four independently framed artifact/dialect strings;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line transform-line brep-line mesh-line properties-line`,
  `child = "[" "]" | "[" hex "," reference "]"`, `number = INT | FLOAT`);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`;
* the nine verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body is the derived record document of the shared record-document codec
  (`🧿️semio/🚪️io/💾️binary/🎒️record-document/🐍️.py`), driven by this subset's committed record spec
  `…/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`: the SPK container, the fixed-Huffman deflate, the BLAKE3
  content hash and the schema-hashed record body are specified by `🧰️framework/🔨️modules/🎒️pack` and
  re-implemented there, not here. The committed `🎒️.pack.semio` files were written by the Rust record
  pack, so `parse_pack`/`pack_bytes` reproducing them is a cross-language byte agreement.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import importlib.util
import json
import os
import struct
import sys

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
DSL_PREAMBLE = "semio stdio.semio.object.dsl v1"
DOCUMENT_SCHEMA = "stdio.semio.object"

#: 🎰️ The three optional owned child slots, in the order the grammar's `document` lists them —
#: which is also the order the pack frame writes them.
SLOTS = ("brep", "mesh", "properties")

CRATE_DSL = "asset://📦️crate/🗣️.dsl.semio"
CRATE_PACK = "asset://📦️crate/🎒️.pack.semio"


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🪄️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def number_of(lexeme: str) -> float:
    """🔢️ Parses finite scalars, infinities and exact binary64 NaN words."""
    if lexeme.startswith("nan64_"):
        word = lexeme[6:]
        if len(word) != 16 or any(digit not in "0123456789abcdefABCDEF" for digit in word):
            raise AssertionError("invalid binary64 NaN word")
        bits = int(word, 16)
        if bits & 0x7ff0000000000000 != 0x7ff0000000000000 or bits & 0xfffffffffffff == 0:
            raise AssertionError("binary64 NaN word has non-NaN class")
        return struct.unpack("<d", bits.to_bytes(8, "little"))[0]
    return float(lexeme)


def print_number(value: float) -> str:
    """🖨️ Preserves every binary64 scalar word in the native textual form."""
    bits = int.from_bytes(struct.pack("<d", value), "little")
    if bits & 0x7ff0000000000000 == 0x7ff0000000000000:
        if bits & 0xfffffffffffff:
            return "nan64_%016x" % bits
        return "-inf" if bits >> 63 else "inf"
    if bits == 0x8000000000000000:
        return "-0"
    if value == int(value) and abs(value) < 1e16:
        return str(int(value))
    from decimal import Decimal
    return format(Decimal(repr(value)), "f")


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


def reference_fields(target: dict) -> tuple:
    """🔗️ Returns the four independent persisted reference strings."""
    dialect = target["dialect"]
    return (target["artifactId"], dialect["artifactKind"], dialect["standard"], dialect["subset"])


def reference_from_fields(fields: list) -> dict:
    """🪪️ Reconstructs independent artifact and dialect fields without URI lowering."""
    if len(fields) != 4:
        raise AssertionError("a reference requires four strings")
    return {"artifactId": fields[0], "dialect": {"artifactKind": fields[1], "standard": fields[2], "subset": fields[3]}}


def read_reference(reader) -> dict:
    """📖️ Reads the exact bracketed four-string reference production."""
    reader.take("[")
    fields = [reader.hex()]
    for _ in range(3):
        reader.take(",")
        fields.append(reader.hex())
    reader.take("]")
    return reference_from_fields(fields)


def print_reference(target: dict) -> str:
    """🔖️ Prints the literal reference production."""
    return "[" + ",".join(hex_of(field) for field in reference_fields(target)) + "]"


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

    def hex(self) -> str:
        start = self.at
        while self.peek() in "0123456789abcdef" and self.peek() != "":
            self.at += 1
        return text_of(self.text[start : self.at])

    def number(self) -> float:
        start = self.at
        while self.peek() != "" and self.peek() not in ",]":
            self.at += 1
        return number_of(self.text[start : self.at])

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text: %r" % self.text[self.at :])


def read_child(reader: Reader) -> dict:
    """🧒️ `child = "[" "]" | "[" hex "," reference "]"` — an absent slot is the empty pair of brackets."""
    reader.take("[")
    if reader.peek() == "]":
        reader.take("]")
        return {}
    child_id = reader.hex()
    reader.take(",")
    target = read_reference(reader)
    reader.take("]")
    return {"childId": child_id, "target": target}


def print_child(child: dict) -> str:
    """👶️ The writing direction of `child`."""
    if not child:
        return "[]"
    return "[%s,%s]" % (hex_of(child["childId"]), print_reference(child["target"]))


def parse_dsl(text: str) -> dict:
    """📄️ `document = artifact-mark schema-line transform-line brep-line mesh-line
    properties-line`, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    if len(body) != 5:
        raise AssertionError("an object document is exactly five body lines, found %d" % len(body))
    keys = ["schema", "transform", "brep", "mesh", "properties"]
    values = []
    for key, line in zip(keys, body):
        if not line.startswith(key + "="):
            raise AssertionError("expected the %r line, found %r" % (key, line))
        values.append(line[len(key) + 1 :])
    schema = text_of(values[0])
    if schema != DOCUMENT_SCHEMA:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, DOCUMENT_SCHEMA))
    reader = Reader(values[1])
    reader.take("[")
    numbers = [reader.number()]
    while reader.peek() == ",":
        reader.take(",")
        numbers.append(reader.number())
    reader.take("]")
    reader.done()
    if len(numbers) != 10:
        raise AssertionError("the transform line carries %d numbers, the grammar declares 10" % len(numbers))
    document = {"schema": schema, "transform": transform_of(numbers)}
    for slot, raw in zip(SLOTS, values[2:]):
        reader = Reader(raw)
        child = read_child(reader)
        reader.done()
        if child:
            document[slot] = child
    return document


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    lines = [
        DSL_PREAMBLE,
        "schema=%s" % hex_of(document["schema"]),
        "transform=[%s]" % ",".join(print_number(value) for value in transform_numbers(document["transform"])),
    ]
    lines.extend("%s=%s" % (slot, print_child(document.get(slot, {}))) for slot in SLOTS)
    return "\n".join(lines)


def transform_of(numbers: list) -> dict:
    """📐️ The ten positional transform numbers as the committed JSON schema names them."""
    return {
        "translation": {"x": numbers[0], "y": numbers[1], "z": numbers[2]},
        "rotation": {"x": numbers[3], "y": numbers[4], "z": numbers[5], "w": numbers[6]},
        "scale": {"x": numbers[7], "y": numbers[8], "z": numbers[9]},
    }


def transform_numbers(transform: dict) -> list:
    """⚖️ The inverse of `transform_of` — translation, rotation, scale, in the grammar's order."""
    translation, rotation, scale = transform["translation"], transform["rotation"], transform["scale"]
    return [
        translation["x"],
        translation["y"],
        translation["z"],
        rotation["x"],
        rotation["y"],
        rotation["z"],
        rotation["w"],
        scale["x"],
        scale["y"],
        scale["z"],
    ]


# endregion 🔖️Dsl


# region 🔖️Pack
_HERE = os.path.dirname(os.path.abspath(__file__))


def _load_shared(alias, *parts):
    """🧬️ Loads a shared module from the `stdio.semio` artifact root, six folders above this case, and registers it under an importable name."""
    if alias in sys.modules:
        return sys.modules[alias]
    path = os.path.join(_HERE, *[os.pardir] * 6, *parts)
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


record_document = _load_shared("record_document", "🚪️io", "💾️binary", "🎒️record-document", "🐍️.py")
RECORD_SPEC = record_document.load_record_spec(os.path.join(_HERE, os.pardir, os.pardir, "🧬️schema", "📸️snapshot", "🗺️record-spec", "🔣️.json"))


def parse_pack(data: bytes) -> dict:
    """📦️ A `.pack.semio` file as the JSON snapshot its record document carries."""
    return record_document.decode_pack(RECORD_SPEC, data)


def pack_bytes(document: dict) -> bytes:
    """🔍️ The writing direction of `parse_pack`, envelope included."""
    return record_document.encode_pack(RECORD_SPEC, document)


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = ("move-object", "rotate-object", "scale-object", "create-brep", "delete-brep", "create-mesh", "delete-mesh", "create-properties", "delete-properties")

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "move-object": "MoveObject",
    "rotate-object": "RotateObject",
    "scale-object": "ScaleObject",
    "create-brep": "CreateBrep",
    "delete-brep": "DeleteBrep",
    "create-mesh": "CreateMesh",
    "delete-mesh": "DeleteMesh",
    "create-properties": "CreateProperties",
    "delete-properties": "DeleteProperties",
}
SLOT_OF_TAG = {"CreateBrep": "brep", "DeleteBrep": "brep", "CreateMesh": "mesh", "DeleteMesh": "mesh", "CreateProperties": "properties", "DeleteProperties": "properties"}
FIELD_OF_TAG = {"MoveObject": ("translation", "translation"), "RotateObject": ("rotation", "rotation"), "ScaleObject": ("scale", "scale")}


def tagged(mutation: dict) -> tuple:
    """✂️ Splits `{"MoveObject": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def clone(value):
    return json.loads(json.dumps(value))


def apply_mutation(document: dict, mutation: dict) -> dict:
    """📱️ Applies one verb, returning a NEW document. A verb addressing a slot that cannot carry
    its argument is a refusal, never a silent no-op — a quietly skipped mutation would report as a
    pass."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag in FIELD_OF_TAG:
        field, argument = FIELD_OF_TAG[tag]
        result["transform"][field] = clone(args[argument])
        return result
    slot = SLOT_OF_TAG[tag]
    if tag.startswith("Create"):
        if slot in result:
            raise AssertionError("%s attaches a child to the %s slot, which already carries %r" % (tag, slot, result[slot]["childId"]))
        result[slot] = {"childId": args["child_id"], "target": clone(args["target"])}
    else:
        if slot not in result:
            raise AssertionError("%s detaches the %s slot, which is already empty" % (tag, slot))
        del result[slot]
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an overwrite is undone by an overwrite with the value it displaced, an attachment by
    the matching detachment, and a detachment by re-attaching the exact handle it removed."""
    tag, args = tagged(mutation)
    if tag in FIELD_OF_TAG:
        field, argument = FIELD_OF_TAG[tag]
        return [{tag: {argument: clone(document["transform"][field])}}]
    slot = SLOT_OF_TAG[tag]
    if tag.startswith("Create"):
        return [{"Delete" + tag[len("Create") :]: {}}]
    if slot not in document:
        raise AssertionError("%s detaches the %s slot, which is already empty" % (tag, slot))
    child = document[slot]
    return [{"Create" + tag[len("Delete") :]: {"child_id": child["childId"], "target": clone(child["target"])}}]


def apply_all(document: dict, mutations: list) -> dict:
    """🗜️ Folds a list of verbs over a document, left to right."""
    for mutation in mutations:
        document = apply_mutation(document, mutation)
    return document


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_json(ctx: Context) -> dict:
    """➡️ The scenario's own committed vector — the feature owns the parameters, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def prepared(ctx: Context) -> tuple:
    """🔟️ The real committed crate object, put into the state the scenario's verb is defined for by
    the doc string's own `prepare` list, plus the verb itself."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.input_bytes(CRATE_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def snapshot_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed crate object by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """📚️ The same verb, then this implementation's own undo of it. The restored document must be
    the prepared object again — asserted here, and compared against the subject's restored document
    by the runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the crate object\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🌀️ The same verb on its committed handcrafted `(before, mutation, after)` vector. The vector
    is a THIRD statement of what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = ctx.step_input_uris()[:3]
    before = snapshot_json(ctx, before_uri)
    after = snapshot_json(ctx, after_uri)
    applied = apply_mutation(before, snapshot_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    return Outcome(applied)


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the crate object, each re-emitted from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps that from being vacuous is that the bytes were written by the OTHER implementation:
    this file reproducing them is a cross-language byte agreement, not a codec agreeing with itself.
    """
    dsl_bytes = ctx.input_bytes(CRATE_DSL)
    document = parse_dsl(dsl_bytes.decode("utf-8"))
    if any(slot not in document for slot in SLOTS):
        raise AssertionError("the committed crate object is the all-three-children artifact this case describes, but a slot decoded as absent")
    printed = print_dsl(document).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("re-printing the crate object did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(dsl_bytes)))
    if parse_dsl(printed.decode("utf-8")) != document:
        raise AssertionError("re-parsing the printed crate object lost content")
    committed_pack = ctx.input_bytes(CRATE_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != document:
        raise AssertionError("the committed binary twin decodes to a different object than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the crate object did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack lost content")
    return Outcome(
        {
            "document": document,
            "dslDigest": digest(printed),
            "packDigest": digest(repacked),
            "dslLength": len(printed),
            "packLength": len(repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
