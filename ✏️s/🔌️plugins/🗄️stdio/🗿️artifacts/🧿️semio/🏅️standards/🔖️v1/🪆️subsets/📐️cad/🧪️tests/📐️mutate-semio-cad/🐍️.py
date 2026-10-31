"""🐍️ Independent Python implementation of the `s.stdio.semio.cad` carrier and its sixteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio` is a semio-native carrier that no
third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is a
second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the mandatory `semio <envelope-id>.dsl v<version>` preamble line — is specified in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line layers-line blocks-line entities-line`,
  `layer = "[" hex "," i32 "," hex "," bool "]"`, `entity-record = "[" hex "," hex "," entity "]"`,
  `block = "[" hex "," point2 "," entity-record-list "]"` and the nine single-letter-tagged
  `entity` variants `L`/`A`/`C`/`E`/`P`/`T`/`I`/`S`/`D` with the field lists that grammar gives;
* the sixteen verbs, their named arguments and the three `option-…` spellings are the committed
  grammar `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is
  this case's committed per-kind specification vectors under `🧫️fixtures/`.

Two leaves the grammar states by reference rather than in full, and how each was settled:

* `hex` is declared to be the framework's built-in `hex` MACRO, so every `handle`, `name`,
  `content`, `text`, `blockName` and `schema` leaf is the lowercase hex of that string's UTF-8
  bytes. Reading the committed artifact confirms it — `434f4e54494e554f5553` is `CONTINUOUS`.
* `number = INT | FLOAT` and the grammar records that every `f64` prints through plain Rust `{v}`
  Display, which drops the fractional part of an integral value. `print_number` below reproduces
  that rule, and the reading is PINNED by `identity-round-trip`, which re-prints the committed
  drawing byte for byte — `[2,2]`, `0.5` and `6.283` in it exercise both halves.

The one place the grammars are silent is where an `add-…` verb PLACES its addition in a name-keyed
collection. This implementation appends, which is the reading the committed specification vectors
also carry, and the consequence is stated rather than hidden: the inverse of a removal is the
matching addition, so a removal that was not final restores the value but not the position. The
feature exercises the forward direction at a MIDDLE entity and the inverse at the final one, and
says so.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import importlib.util
import json
import os
import sys

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every variant of this subset's mutation vocabulary, in the order the mutations grammar's `op`
#: production lists them, kebab-cased as the catalog spells them, paired with the camel-case tag the
#: committed vectors and the feature use on the wire.
KINDS = (
    "add-layer",
    "remove-layer",
    "set-layer",
    "add-block",
    "remove-block",
    "set-block-base-point",
    "add-entity",
    "remove-entity",
    "set-entity-layer",
    "set-entity-geometry",
    "add-block-entity",
    "remove-block-entity",
    "set-block-entity-layer",
    "set-block-entity-geometry",
)


def camel(kind: str) -> str:
    """🐫 The wire tag of one verb: its kebab-case name in camel case."""
    head, *rest = kind.split("-")
    return head + "".join(word.capitalize() for word in rest)


TAG_OF_KIND = {kind: camel(kind) for kind in KINDS}
KIND_OF_TAG = {tag: kind for kind, tag in TAG_OF_KIND.items()}

#: 📐️ `entity = line | arc | circle | ellipse | polyline | text-entity | insert | solid | dimension`
#: — the grammar's single-letter tags and the field lists it declares for each.
ENTITY_FIELDS = {
    "L": ("line", (("a", "point2"), ("b", "point2"))),
    "A": ("arc", (("center", "point2"), ("radius", "number"), ("startAngle", "number"), ("endAngle", "number"))),
    "C": ("circle", (("center", "point2"), ("radius", "number"))),
    "E": ("ellipse", (("center", "point2"), ("majorAxisEnd", "point2"), ("ratio", "number"), ("startParam", "number"), ("endParam", "number"))),
    "P": ("polyline", (("vertices", "point2-list"), ("closed", "bool"))),
    "T": ("text", (("position", "point2"), ("height", "number"), ("rotation", "number"), ("content", "hex"))),
    "I": ("insert", (("blockName", "hex"), ("insertionPoint", "point2"), ("scale", "point2"), ("rotation", "number"))),
    "S": ("solid", (("p1", "point2"), ("p2", "point2"), ("p3", "point2"), ("p4", "point2"))),
    "D": ("dimension", (("defPoint", "point2"), ("textPosition", "point2"), ("measurement", "number"), ("text", "hex"))),
}
TAG_OF_ENTITY = {name: tag for tag, (name, _) in ENTITY_FIELDS.items()}

DOCUMENT_SCHEMA = "stdio.semio.cad"
DSL_PREAMBLE = "semio stdio.semio.cad.dsl v1"

DRAWING_DSL = "asset://📐️drawing/🗣️.dsl.semio"
DRAWING_PACK = "asset://📐️drawing/🎒️.pack.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🪄️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def read_number(text: str) -> float:
    """🔢️ `number = INT | FLOAT` — one `f64` leaf, integral values spelled without a point."""
    try:
        return float(text)
    except ValueError:
        raise AssertionError("expected a number, got %r" % text)


def print_number(value: float) -> str:
    """🖨️ The writing direction of `number`: Rust's `{v}` Display for `f64`, which prints an
    integral value with no fractional part and otherwise the shortest round-tripping form."""
    if value == int(value):
        return "%d" % int(value)
    return repr(float(value))


def read_bool(text: str) -> bool:
    """🔘️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("expected a bool spelled 0 or 1, got %r" % text)
    return text == "1"


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


def split_top_level(text: str) -> list:
    """🔎️ Splits a bracket-nested list body on its top-level commas; `""` means the empty list."""
    if text == "":
        return []
    out, depth, start = [], 0, 0
    for at, char in enumerate(text):
        if char == "[":
            depth += 1
        elif char == "]":
            depth -= 1
            if depth < 0:
                raise AssertionError("unbalanced ']' at offset %d of %r" % (at, text))
        elif char == "," and depth == 0:
            out.append(text[start:at])
            start = at + 1
    if depth != 0:
        raise AssertionError("unbalanced '[' in %r" % text)
    out.append(text[start:])
    return out


def bracketed(text: str) -> str:
    """🧍️ The body of a `"[" … "]"` group, refusing anything that is not one."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("expected a bracketed group, got %r" % text)
    return text[1:-1]


def field(lines: list, name: str) -> str:
    """📛️ The right-hand side of one `name "=" value` body line; a missing line is an error."""
    prefix = name + "="
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix) :]
    raise AssertionError("the document has no %r line — the grammar declares it mandatory" % name)


# endregion 🔖️Carrier


# region 🔖️Dsl
def read_point2(text: str) -> dict:
    """📍️ `point2 = "[" number "," number "]"`."""
    x, y = split_top_level(bracketed(text))
    return {"x": read_number(x), "y": read_number(y)}


def write_point2(point: dict) -> str:
    return "[%s,%s]" % (print_number(point["x"]), print_number(point["y"]))


def read_leaf(text: str, shape: str):
    if shape == "point2":
        return read_point2(text)
    if shape == "point2-list":
        return [read_point2(one) for one in split_top_level(bracketed(text))]
    if shape == "number":
        return read_number(text)
    if shape == "bool":
        return read_bool(text)
    return text_of(text)


def write_leaf(value, shape: str) -> str:
    if shape == "point2":
        return write_point2(value)
    if shape == "point2-list":
        return "[%s]" % ",".join(write_point2(one) for one in value)
    if shape == "number":
        return print_number(value)
    if shape == "bool":
        return "1" if value else "0"
    return hex_of(value)


def read_entity(text: str) -> dict:
    """📐️ One tagged `entity` value: a single-letter tag then its bracketed field list."""
    tag, rest = text[:1], text[1:]
    if tag not in ENTITY_FIELDS:
        raise AssertionError("unknown entity tag %r — the grammar declares %s" % (tag, ", ".join(sorted(ENTITY_FIELDS))))
    name, fields = ENTITY_FIELDS[tag]
    parts = split_top_level(bracketed(rest))
    if len(parts) != len(fields):
        raise AssertionError("entity %s takes %d field(s), got %d in %r" % (name, len(fields), len(parts), text))
    entity = {"kind": name}
    for (key, shape), part in zip(fields, parts):
        entity[key] = read_leaf(part, shape)
    return entity


def write_entity(entity: dict) -> str:
    if entity["kind"] not in TAG_OF_ENTITY:
        raise AssertionError("unknown entity kind %r" % entity["kind"])
    tag = TAG_OF_ENTITY[entity["kind"]]
    _, fields = ENTITY_FIELDS[tag]
    return "%s[%s]" % (tag, ",".join(write_leaf(entity[key], shape) for key, shape in fields))


def read_layer(text: str) -> dict:
    """🗂️ `layer = "[" hex "," i32 "," hex "," bool "]"`."""
    name, color, line_type, visible = split_top_level(bracketed(text))
    return {"name": text_of(name), "colorIndex": int(color), "lineType": text_of(line_type), "visible": read_bool(visible)}


def write_layer(layer: dict) -> str:
    return "[%s,%d,%s,%s]" % (hex_of(layer["name"]), int(layer["colorIndex"]), hex_of(layer["lineType"]), "1" if layer["visible"] else "0")


def read_record(text: str) -> dict:
    """🧾️ `entity-record = "[" hex "," hex "," entity "]"` — handle, layer, geometry."""
    handle, layer, entity = split_top_level(bracketed(text))
    return {"handle": text_of(handle), "layer": text_of(layer), "entity": read_entity(entity)}


def write_record(record: dict) -> str:
    return "[%s,%s,%s]" % (hex_of(record["handle"]), hex_of(record["layer"]), write_entity(record["entity"]))


def read_block(text: str) -> dict:
    """🧱️ `block = "[" hex "," point2 "," entity-record-list "]"`."""
    name, base, records = split_top_level(bracketed(text))
    return {"name": text_of(name), "basePoint": read_point2(base), "entities": [read_record(each) for each in split_top_level(bracketed(records))]}


def write_block(block: dict) -> str:
    return "[%s,%s,[%s]]" % (hex_of(block["name"]), write_point2(block["basePoint"]), ",".join(write_record(record) for record in block["entities"]))


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line layers-line blocks-line entities-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    return {
        "schema": text_of(field(lines, "schema")),
        "layers": [read_layer(each) for each in split_top_level(bracketed(field(lines, "layers")))],
        "blocks": [read_block(each) for each in split_top_level(bracketed(field(lines, "blocks")))],
        "entities": [read_record(each) for each in split_top_level(bracketed(field(lines, "entities")))],
    }


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    body = [
        "schema=%s" % hex_of(snapshot["schema"]),
        "layers=[%s]" % ",".join(write_layer(layer) for layer in snapshot["layers"]),
        "blocks=[%s]" % ",".join(write_block(block) for block in snapshot["blocks"]),
        "entities=[%s]" % ",".join(write_record(record) for record in snapshot["entities"]),
    ]
    return "\n".join([DSL_PREAMBLE] + body)


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
def clone(value):
    """🏛️ A structural copy, so applying a verb never writes through into the parsed document."""
    return json.loads(json.dumps(value))


def parts(mutation: dict) -> tuple:
    """✂️ Splits `{"mutation": "<camelTag>", …}` into its verb and its named arguments."""
    tag = mutation.get("mutation")
    if tag not in KIND_OF_TAG:
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(KIND_OF_TAG))))
    return KIND_OF_TAG[tag], mutation


def named(items: list, key: str, value: str, verb: str) -> int:
    """🗝️ The position of the entry a name-keyed verb addresses. An unaddressable name is a
    refusal, never a silent no-op — a quietly skipped mutation would report as a pass."""
    for at, item in enumerate(items):
        if item[key] == value:
            return at
    raise AssertionError("%s addresses %s %r, which the document does not carry" % (verb, key, value))


def block_named(snapshot: dict, args: dict, verb: str) -> dict:
    return snapshot["blocks"][named(snapshot["blocks"], "name", args["blockName"], verb)]


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """📱️ Applies one verb, returning a NEW snapshot. `add-…` appends to its name-keyed collection,
    which is what the committed specification vectors record and what the grammars leave open."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "add-layer":
        result["layers"].insert(min(args["at"], len(result["layers"])) if "at" in args else len(result["layers"]), clone(args["layer"]))
        return result
    if kind == "remove-layer":
        del result["layers"][named(result["layers"], "name", args["name"], kind)]
        return result
    if kind == "set-layer":
        layer = result["layers"][named(result["layers"], "name", args["name"], kind)]
        for key in ("colorIndex", "lineType", "visible"):
            if args.get(key) is not None:
                layer[key] = args[key]
        return result
    if kind == "add-block":
        result["blocks"].insert(min(args["at"], len(result["blocks"])) if "at" in args else len(result["blocks"]), clone(args["block"]))
        return result
    if kind == "remove-block":
        del result["blocks"][named(result["blocks"], "name", args["name"], kind)]
        return result
    if kind == "set-block-base-point":
        result["blocks"][named(result["blocks"], "name", args["name"], kind)]["basePoint"] = clone(args["basePoint"])
        return result
    if kind == "add-entity":
        result["entities"].insert(min(args["at"], len(result["entities"])) if "at" in args else len(result["entities"]), clone(args["entity"]))
        return result
    if kind == "remove-entity":
        del result["entities"][named(result["entities"], "handle", args["handle"], kind)]
        return result
    if kind == "set-entity-layer":
        result["entities"][named(result["entities"], "handle", args["handle"], kind)]["layer"] = args["layer"]
        return result
    if kind == "set-entity-geometry":
        result["entities"][named(result["entities"], "handle", args["handle"], kind)]["entity"] = clone(args["entity"])
        return result
    block = block_named(result, args, kind)
    if kind == "add-block-entity":
        block["entities"].insert(min(args["at"], len(block["entities"])) if "at" in args else len(block["entities"]), clone(args["entity"]))
        return result
    at = named(block["entities"], "handle", args["handle"], kind)
    if kind == "remove-block-entity":
        del block["entities"][at]
        return result
    if kind == "set-block-entity-layer":
        block["entities"][at]["layer"] = args["layer"]
        return result
    block["entities"][at]["entity"] = clone(args["entity"])
    return result


def inverse_mutation(snapshot: dict, mutation: dict) -> dict:
    """↩️ The undo of one verb against the state it was applied to. An addition is undone by the
    matching removal, an overwrite by an overwrite with the value it displaced, and a removal by an
    `add-…` carrying the removed index (`at`), so the original position is restored exactly."""
    kind, args = parts(mutation)
    if kind == "add-layer":
        return {"mutation": TAG_OF_KIND["remove-layer"], "name": args["layer"]["name"]}
    if kind == "remove-layer":
        return {"mutation": TAG_OF_KIND["add-layer"], "layer": clone(snapshot["layers"][named(snapshot["layers"], "name", args["name"], kind)]), "at": named(snapshot["layers"], "name", args["name"], kind)}
    if kind == "set-layer":
        was = snapshot["layers"][named(snapshot["layers"], "name", args["name"], kind)]
        undo = {"mutation": TAG_OF_KIND[kind], "name": args["name"]}
        for key in ("colorIndex", "lineType", "visible"):
            if args.get(key) is not None:
                undo[key] = was[key]
        return undo
    if kind == "add-block":
        return {"mutation": TAG_OF_KIND["remove-block"], "name": args["block"]["name"]}
    if kind == "remove-block":
        return {"mutation": TAG_OF_KIND["add-block"], "block": clone(snapshot["blocks"][named(snapshot["blocks"], "name", args["name"], kind)]), "at": named(snapshot["blocks"], "name", args["name"], kind)}
    if kind == "set-block-base-point":
        was = snapshot["blocks"][named(snapshot["blocks"], "name", args["name"], kind)]
        return {"mutation": TAG_OF_KIND[kind], "name": args["name"], "basePoint": clone(was["basePoint"])}
    if kind == "add-entity":
        return {"mutation": TAG_OF_KIND["remove-entity"], "handle": args["entity"]["handle"]}
    if kind == "remove-entity":
        return {"mutation": TAG_OF_KIND["add-entity"], "entity": clone(snapshot["entities"][named(snapshot["entities"], "handle", args["handle"], kind)]), "at": named(snapshot["entities"], "handle", args["handle"], kind)}
    if kind in ("set-entity-layer", "set-entity-geometry"):
        was = snapshot["entities"][named(snapshot["entities"], "handle", args["handle"], kind)]
        if kind == "set-entity-layer":
            return {"mutation": TAG_OF_KIND[kind], "handle": args["handle"], "layer": was["layer"]}
        return {"mutation": TAG_OF_KIND[kind], "handle": args["handle"], "entity": clone(was["entity"])}
    block = block_named(snapshot, args, kind)
    if kind == "add-block-entity":
        return {"mutation": TAG_OF_KIND["remove-block-entity"], "blockName": args["blockName"], "handle": args["entity"]["handle"]}
    was = block["entities"][named(block["entities"], "handle", args["handle"], kind)]
    if kind == "remove-block-entity":
        return {"mutation": TAG_OF_KIND["add-block-entity"], "blockName": args["blockName"], "entity": clone(was), "at": named(block["entities"], "handle", args["handle"], kind)}
    if kind == "set-block-entity-layer":
        return {"mutation": TAG_OF_KIND[kind], "blockName": args["blockName"], "handle": args["handle"], "layer": was["layer"]}
    return {"mutation": TAG_OF_KIND[kind], "blockName": args["blockName"], "handle": args["handle"], "entity": clone(was["entity"])}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """🔌️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def drawing(ctx: Context) -> dict:
    """✏️ The real committed drawing, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.input_bytes(DRAWING_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.input_bytes(next(token for step in ctx.scenario["steps"] for token in step["text"].split() if token.startswith("shared://📐️mutate-semio-cad/") and token.endswith("%s/🦠️mutation/🔣️.json" % kind))).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed drawing by this implementation alone."""
    result = apply_mutation(drawing(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """🧲️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the drawing again — asserted here, and the MUTATED snapshot travels in the projection too, so
    the sixteen rows cannot all project the same restored value and compare vacuously."""
    snapshot = drawing(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the drawing\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """➡️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations, kept from before this oracle existed."""
    kind = ctx.row()
    committed = vector(ctx, kind)
    applied = apply_mutation(committed["before"], committed["mutation"])
    if applied != committed["after"]:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(committed["after"])))
    return Outcome(applied)


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real drawing, each re-emitted from the parsed snapshot.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps that from being vacuous is that both committed files were written by the OTHER
    implementation: this file reproducing them is a cross-language byte agreement, not a codec
    agreeing with itself. The two encodings also cross-check each other — the binary twin has to
    decode to the same drawing the text does, which no single codec can arrange on its own.
    """
    committed = ctx.input_bytes(DRAWING_DSL)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing the drawing did not reproduce the committed DSL bytes (%d vs %d bytes)\n     got: %s\nexpected: %s" % (len(printed), len(committed), printed.decode("utf-8"), committed.decode("utf-8")))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed drawing lost content")
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("the committed drawing declares schema %r, expected %r" % (snapshot["schema"], DOCUMENT_SCHEMA))
    committed_pack = ctx.input_bytes(DRAWING_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != snapshot:
        raise AssertionError("the committed binary twin decodes to a different drawing than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked), json.dumps(snapshot)))
    repacked = pack_bytes(snapshot)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the drawing did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    if parse_pack(repacked) != snapshot:
        raise AssertionError("re-decoding the encoded pack lost content")
    declared = vector(ctx, "remove-block-entity")["before"]
    if snapshot != declared:
        raise AssertionError("the real committed drawing does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(snapshot), json.dumps(declared)))
    return Outcome(
        {
            "document": snapshot,
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
