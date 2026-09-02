"""🐍️ IfcOpenShell differential ORACLE for stdio.ifc 4/✳️any — a real SECOND PRODUCER.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. The sibling case `../mutate-ifc-4` registers
`ruststep` 0.4, which parses the ISO 10303-21 grammar and has no writer at all, so every scenario
there is honestly typed `@mode-property`/`@mode-round-trip` — a second READER, never a second
producer. This case closes that gap for the seven mutation kinds a schema-bound implementation can
genuinely perform: **IfcOpenShell 0.8.4.post1 applies each mutation to the real 2 496 437-byte,
24 792-entity Nakagin Capsule Tower export and re-serializes the whole exchange structure itself**
(`ifcopenshell.file.to_string`, its own C++ Part-21 writer), and the result is read back by the
from-scratch ISO 10303-21 reader below before `semantic-ifc-v1` compares it against this
repository's own subject.

🚫️ **Nothing here reaches this repository's implementation.** The only imports are `json`,
`ifcopenshell` and the host's own `semio_repo_test` facade. No `subprocess`, no `ctypes`, no
`importlib`, no cargo, no wasm, no semio module. The reader in `🔖️Part21Reader` was written from
ISO 10303-21 clause 6 (`§6.4.2` control directives, `§6.2` doubled apostrophe) and clause 8
(`§8.2.2`/`§8.2.3` header attribute order), not from
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🦀️oracle.rs` and not from
the production `step::engine::part21` codec it is evidence about.

🧫️ **The artifact is real and complex, and it is the one already committed.** `shared://🏗️nakagin-
capsule-tower.ifc` — a real IfcOpenShell 0.8.4.post1 export of Kisho Kurokawa's Nakagin Capsule
Tower, `FILE_SCHEMA(('IFC4'))`, 2 496 437 bytes, 24 792 entity instances. Every scenario copies it
into the case work directory first; the committed asset is never written to.

📏️ **What IfcOpenShell can and cannot produce, measured rather than assumed.** This subset's
`IfcMutation` vocabulary is Part-21 RECORD-level; IfcOpenShell is EXPRESS-SCHEMA-bound. Four of the
eleven kinds therefore have no differential scenario here, each for a reason confirmed against this
exact fixture:

* `set-entity-name` — retyping `#16976` to `RENAMED_PROXY` is refused on creation
  (`Entity with name 'RENAMED_PROXY' not found in schema 'IFC4'`), and reading such a file back with
  `ifcopenshell.file.from_string` returns **16 975 of 24 792** entities with no error raised at all.
* `insert-entity-arg` — a tenth positional argument on the nine-attribute `IfcBuildingElementProxy`
  raises `IndexError` on assignment, and a hand-written file carrying one reads back with the extra
  argument silently dropped.
* `remove-entity-arg` — arity cannot be reduced; assigning `None` writes `$` and keeps nine
  arguments, which is a different mutation.
* `remove-entity` — `ifcopenshell.file.remove` documents and performs reference repair ("in the case
  of a list or set of references, the reference to the deleted will be removed from the aggregate"),
  confirmed here: `#16976` disappears from `#16991`'s member aggregate. `IfcMutation::RemoveEntity`
  deliberately does NOT cascade. Two implementations of two different verbs are not a differential.

Those four keep their `ruststep`-backed scenarios in `../mutate-ifc-4` unchanged. The removal
primitive IS used here, but only as the inverse of `insert-entity`, and only behind an explicit
`get_total_inverses(...) == 0` guard, so the cascading path is never silently taken.

@see ../mutate-ifc-4/component.feature — the exhaustive eleven-kind case this one does not replace.
@see ../../🏅️standards/🔖️4/🪆️subsets/✳️any/🔣️oracle.json — this oracle's registration.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import os
import re

import ifcopenshell

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Input
INPUT = "shared://🧪️nakagin-capsule-tower/🏗️.ifc"

#: 🧬️ The seven kinds IfcOpenShell can genuinely PRODUCE, in this subset's own catalog order.
KINDS = ["no-mutation", "set-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "set-entity-arg"]


def mutable_input(ctx: Context) -> str:
    """🧫️ The work-directory copy of the committed fixture — never the committed file itself."""
    return ctx.copy_fixture(INPUT, "input.ifc")


# endregion 🔖️Input


# region 🔖️Part21Lexer
def decode_string_literal(lexeme: str) -> str:
    """🔤️ ISO 10303-21 §6.4.2 — one string LEXEME turned into the VALUE it denotes.

    A malformed directive is an ERROR, never a passed-through lexeme: a producer that emitted a
    broken escape must fail the comparison rather than sneak through it.
    """
    chars = list(lexeme)
    out = []
    index = 0
    alphabet = "A"

    def hex_group(at: int, width: int) -> str:
        group = "".join(chars[at:at + width])
        if len(group) != width or not all(c in "0123456789abcdefABCDEF" for c in group):
            raise ValueError("bad hex group %r in %r" % (group, lexeme))
        return chr(int(group, 16))

    while index < len(chars):
        if chars[index] != "\\":
            out.append(chars[index])
            index += 1
            continue
        directive = chars[index + 1] if index + 1 < len(chars) else None
        if directive == "\\":
            out.append("\\")
            index += 2
        elif directive == "P":
            page = chars[index + 2] if index + 2 < len(chars) else None
            if page is None or not ("A" <= page <= "I") or index + 3 >= len(chars) or chars[index + 3] != "\\":
                raise ValueError("malformed \\P directive in %r" % lexeme)
            alphabet = page
            index += 4
        elif directive == "S":
            if index + 2 >= len(chars) or chars[index + 2] != "\\":
                raise ValueError("malformed \\S directive in %r" % lexeme)
            if alphabet != "A":
                raise ValueError("\\S\\ on ISO 8859 page %s needs a mapping table this projection does not carry" % alphabet)
            if index + 3 >= len(chars):
                raise ValueError("truncated \\S directive in %r" % lexeme)
            out.append(chr(ord(chars[index + 3]) + 128))
            index += 4
        elif directive == "X":
            width = chars[index + 2] if index + 2 < len(chars) else None
            if width in ("2", "4"):
                group = 4 if width == "2" else 8
                if index + 3 >= len(chars) or chars[index + 3] != "\\":
                    raise ValueError("malformed \\X%s directive in %r" % (width, lexeme))
                index += 4
                while True:
                    out.append(hex_group(index, group))
                    index += group
                    if chars[index:index + 4] == ["\\", "X", "0", "\\"]:
                        index += 4
                        break
                    if index >= len(chars):
                        raise ValueError("unterminated \\X%s run in %r" % (width, lexeme))
            elif width == "\\":
                out.append(hex_group(index + 3, 2))
                index += 5
            else:
                raise ValueError("malformed \\X directive %r in %r" % (width, lexeme))
        else:
            raise ValueError("unsupported control directive %r in %r" % (directive, lexeme))
    return "".join(out)


# endregion 🔖️Part21Lexer


# region 🔖️Part21Reader
WHITESPACE = " \t\r\n"


class Reader:
    """📥️ A from-scratch ISO 10303-21 clear-text reader, independent of both producers."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.size = len(text)
        self.at = 0

    def skip(self) -> None:
        while self.at < self.size:
            char = self.text[self.at]
            if char in WHITESPACE:
                self.at += 1
            elif char == "/" and self.text.startswith("/*", self.at):
                end = self.text.find("*/", self.at + 2)
                self.at = self.size if end < 0 else end + 2
            else:
                return

    def keyword(self) -> str:
        start = self.at
        while self.at < self.size and (self.text[self.at].isalnum() or self.text[self.at] == "_"):
            self.at += 1
        return self.text[start:self.at]

    def string_lexeme(self) -> str:
        assert self.text[self.at] == "'"
        at = self.at + 1
        parts = []
        while True:
            char = self.text[at]
            if char == "'":
                if at + 1 < self.size and self.text[at + 1] == "'":
                    parts.append("'")
                    at += 2
                    continue
                self.at = at + 1
                return "".join(parts)
            parts.append(char)
            at += 1

    def number(self) -> dict:
        start = self.at
        if self.text[self.at] in "+-":
            self.at += 1
        real = False
        while self.at < self.size:
            char = self.text[self.at]
            if char.isdigit():
                self.at += 1
            elif char == ".":
                real = True
                self.at += 1
            elif char in "eE":
                real = True
                self.at += 1
            elif char in "+-" and self.text[self.at - 1] in "eE":
                self.at += 1
            else:
                break
        lexeme = self.text[start:self.at]
        return {"t": "real", "v": float(lexeme)} if real else {"t": "integer", "v": int(lexeme)}

    def value(self) -> dict:
        self.skip()
        char = self.text[self.at]
        if char == "$":
            self.at += 1
            return {"t": "unset"}
        if char == "*":
            self.at += 1
            return {"t": "derived"}
        if char == "'":
            return {"t": "string", "v": decode_string_literal(self.string_lexeme())}
        if char == "#":
            self.at += 1
            start = self.at
            while self.at < self.size and self.text[self.at].isdigit():
                self.at += 1
            return {"t": "reference", "v": int(self.text[start:self.at])}
        if char == ".":
            end = self.text.index(".", self.at + 1)
            name = self.text[self.at + 1:end]
            self.at = end + 1
            return {"t": "enum", "v": name}
        if char == '"':
            end = self.text.index('"', self.at + 1)
            lexeme = self.text[self.at + 1:end]
            self.at = end + 1
            return {"t": "binary", "v": lexeme}
        if char == "(":
            return {"t": "aggregate", "v": self.parameter_list()}
        if char in "+-" or char.isdigit():
            return self.number()
        if char.isalpha() or char == "_":
            name = self.keyword()
            self.skip()
            if self.at >= self.size or self.text[self.at] != "(":
                raise ValueError("bare keyword %r at %d" % (name, self.at))
            inner = self.parameter_list()
            if len(inner) != 1:
                raise ValueError("typed parameter %s carries %d values, not one" % (name, len(inner)))
            return {"t": "typed", "name": name, "v": inner[0]}
        raise ValueError("unexpected character %r at %d" % (char, self.at))

    def parameter_list(self) -> list:
        assert self.text[self.at] == "("
        self.at += 1
        items: list = []
        self.skip()
        if self.text[self.at] == ")":
            self.at += 1
            return items
        while True:
            items.append(self.value())
            self.skip()
            if self.text[self.at] == ",":
                self.at += 1
                continue
            if self.text[self.at] == ")":
                self.at += 1
                return items
            raise ValueError("expected , or ) at %d" % self.at)


def read_document(text: str) -> tuple:
    """📥️ Header records and DATA entity instances of one exchange structure."""
    header: dict = {}
    header_start = text.index("HEADER;") + len("HEADER;")
    header_end = text.index("ENDSEC;", header_start)
    reader = Reader(text[header_start:header_end])
    while True:
        reader.skip()
        if reader.at >= reader.size:
            break
        name = reader.keyword()
        if name == "":
            raise ValueError("unparsable header record at %d" % reader.at)
        reader.skip()
        header.setdefault(name.upper(), reader.parameter_list())
        reader.skip()
        if reader.at < reader.size and reader.text[reader.at] == ";":
            reader.at += 1

    data_start = text.index("DATA;", header_end) + len("DATA;")
    data_end = text.rindex("ENDSEC;")
    reader = Reader(text[data_start:data_end])
    entities = []
    while True:
        reader.skip()
        if reader.at >= reader.size or reader.text[reader.at] != "#":
            break
        reader.at += 1
        start = reader.at
        while reader.text[reader.at].isdigit():
            reader.at += 1
        identifier = int(reader.text[start:reader.at])
        reader.skip()
        if reader.text[reader.at] != "=":
            raise ValueError("expected = after #%d" % identifier)
        reader.at += 1
        reader.skip()
        name = reader.keyword()
        reader.skip()
        entities.append({"id": identifier, "name": name.upper(), "args": reader.parameter_list()})
        reader.skip()
        if reader.at < reader.size and reader.text[reader.at] == ";":
            reader.at += 1
    return header, entities


# endregion 🔖️Part21Reader


# region 🔖️Projection
#: 📇️ The seven attributes ISO 10303-21 §8.2.3 fixes for FILE_NAME, in its own order.
FILE_NAME_ATTRIBUTES = ["name", "timestamp", "author", "organization", "preprocessorVersion", "originatingSystem", "authorization"]
#: 📇️ The two attributes ISO 10303-21 §8.2.2 fixes for FILE_DESCRIPTION.
FILE_DESCRIPTION_ATTRIBUTES = ["description", "implementationLevel"]


def header_object(values: list, attributes: list) -> dict:
    return {name: (values[index] if index < len(values) else None) for index, name in enumerate(attributes)}


def project(text: str) -> dict:
    """👁️ The `semantic-ifc-v1` surface: declared FILE_SCHEMA, both header records under the names
    the standard fixes for them, and the full id-sorted entity graph."""
    header, entities = read_document(text)
    schema: list = []
    file_schema = header.get("FILE_SCHEMA", [])
    if file_schema and file_schema[0]["t"] == "aggregate":
        schema = [item["v"] for item in file_schema[0]["v"] if item["t"] == "string"]
    entities.sort(key=lambda entity: entity["id"])
    return {
        "fileSchema": schema,
        "fileDescription": header_object(header.get("FILE_DESCRIPTION", []), FILE_DESCRIPTION_ATTRIBUTES),
        "fileName": header_object(header.get("FILE_NAME", []), FILE_NAME_ATTRIBUTES),
        "entityCount": len(entities),
        "entities": entities,
    }


# endregion 🔖️Projection


# region 🔖️ValueGrammar
def to_ifcopenshell(model, value: dict):
    """🔤️ One `{"t": …, "v": …}` wire value turned into what IfcOpenShell's own API accepts."""
    kind = value["t"]
    if kind == "unset":
        return None
    if kind in ("integer", "real"):
        return value["v"]
    if kind in ("string", "enum"):
        return value["v"]
    if kind == "reference":
        return model.by_id(int(value["v"]))
    if kind == "aggregate":
        return tuple(to_ifcopenshell(model, item) for item in value["v"])
    if kind == "typed":
        return model.create_entity(value["name"], to_ifcopenshell(model, value["v"]))
    raise ValueError("IfcOpenShell has no representation for the wire value type %r" % kind)


def literal(value: dict):
    """🔤️ The same grammar restricted to values that carry no entity reference — what an instance
    created in a scratch file may hold before it is added at a chosen id."""
    kind = value["t"]
    if kind == "reference":
        raise ValueError("insert-entity with a referencing argument cannot be placed at a chosen id: IfcOpenShell's `add` deep-copies what a new instance points at, which would insert entities the mutation does not name")
    if kind == "aggregate":
        return tuple(literal(item) for item in value["v"])
    return to_ifcopenshell(None, value) if kind in ("unset",) else value["v"] if kind in ("integer", "real", "string", "enum") else None


# endregion 🔖️ValueGrammar



# region 🔖️SilentLossGuard
#: 🔎️ A DATA-section record header at the start of its own line — the shape both this subset's
#: committed fixture and every IfcOpenShell-written document use.
RECORD_LINE = re.compile(r"^\s*#\d+\s*=", re.MULTILINE)


def declared_record_count(text: str) -> int:
    """🔢️ How many entity instances the document TEXT declares, counted without parsing it."""
    start = text.index("DATA;") + len("DATA;")
    return len(RECORD_LINE.findall(text[start:text.rindex("ENDSEC;")]))


def open_model(path: str):
    """📥️ `ifcopenshell.open` plus the guard a real measurement made necessary.

    ⚠️ CONFIRMED defect in IfcOpenShell 0.8.4.post1, reproduced standalone before being guarded
    against, not assumed: a document whose `FILE_SCHEMA` lists an identifier the schema resolver
    does not know — which ISO 10303-21 §8.2.4 explicitly permits, since `schema_identifiers` is a
    LIST — is opened WITHOUT error and yields a model whose header is intact and whose data section
    is EMPTY. Re-serializing it then writes a 332-byte document with `DATA; ENDSEC;` and no
    instances at all: 3 464 real instances silently gone, no exception, no warning. (Reached through
    `ifcopenshell.file.from_string` the same input raises `RuntimeError: No schema loaded`, so the
    two entry points disagree about whether this is an error.) An oracle that swallowed that would
    report a passing round trip over an empty document, which is the exact failure this platform
    exists to prevent. Every read in this module goes through here.
    """
    with open(path, "r", encoding="utf-8") as handle:
        text = handle.read()
    declared = declared_record_count(text)
    model = ifcopenshell.open(path)
    materialized = len(list(model))
    if materialized < declared:
        raise AssertionError("IfcOpenShell opened %s without raising but materialized only %d of the %d entity instances the document declares — refusing to treat a silently truncated model as a producer's input" % (os.path.basename(path), materialized, declared))
    return model


# endregion 🔖️SilentLossGuard

# region 🔖️Producer
def apply_mutation(path: str, spec: dict) -> bytes:
    """🦠️ IfcOpenShell reads the real exchange structure, applies one declared mutation through its
    own API, and re-serializes the whole structure with its own writer. An unrecognised kind is an
    error, never a silent no-op: a mutation quietly skipped reports as a passing test."""
    model = open_model(path)
    kind = spec["kind"]
    params = spec.get("params") or {}

    if kind == "no-mutation":
        pass
    elif kind == "set-snapshot":
        names = params["fileSchema"]
        if not names:
            raise ValueError("set-snapshot requires a non-empty fileSchema field")
        model.header.file_schema.schema_identifiers = list(names)
    elif kind == "set-file-schema":
        values = params["values"]
        if not values or values[0]["t"] != "aggregate":
            raise ValueError("set-file-schema requires one aggregate value")
        model.header.file_schema.schema_identifiers = [item["v"] for item in values[0]["v"]]
    elif kind == "set-file-description":
        values = params["values"]
        model.header.file_description.description = list(to_ifcopenshell(model, values[0]))
        if len(values) > 1:
            model.header.file_description.implementation_level = to_ifcopenshell(model, values[1])
    elif kind == "set-file-name":
        values = params["values"]
        header = model.header.file_name
        for index, attribute in enumerate(["name", "time_stamp", "author", "organization", "preprocessor_version", "originating_system", "authorization"]):
            if index >= len(values):
                break
            converted = to_ifcopenshell(model, values[index])
            setattr(header, attribute, list(converted) if isinstance(converted, tuple) else converted)
    elif kind == "insert-entity":
        entity = params["entity"]
        scratch = ifcopenshell.file(schema=model.schema)
        created = scratch.create_entity(entity["name"], *[literal(argument) for argument in entity["args"]])
        model.add(created, int(entity["id"]))
    elif kind == "remove-entity":
        target = model.by_id(int(params["id"]))
        inverses = model.get_total_inverses(target)
        if inverses != 0:
            raise AssertionError("#%s is referenced by %d other instance(s); `ifcopenshell.file.remove` repairs those references while IfcMutation::RemoveEntity deliberately does not, so this oracle refuses the removal rather than silently performing a different verb" % (params["id"], inverses))
        model.remove(target)
    elif kind == "set-entity-arg":
        target = model.by_id(int(params["id"]))
        target[int(params["index"])] = to_ifcopenshell(model, params["value"])
    else:
        raise ValueError("mutation kind %r has no IfcOpenShell producer in this case — see this module's docstring for the measured reason" % kind)

    return model.to_string().encode("utf-8")


# endregion 🔖️Producer


# region 🔖️Inverse
def inverse_spec(kind: str) -> dict:
    """↩️ The inverse of one forward `(kind, params)` pair against this fixture's own real header
    and entity values, computed here from the committed fixture rather than read from any
    implementation's `inverse()` method."""
    if kind == "set-snapshot":
        return {"kind": "set-snapshot", "params": {"fileSchema": ["IFC4"]}}
    if kind == "set-file-schema":
        return {"kind": "set-file-schema", "params": {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC4"}]}]}}
    if kind == "set-file-description":
        return {"kind": "set-file-description", "params": {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "ViewDefinition[DesignTransferView]"}]}, {"t": "string", "v": "2;1"}]}}
    if kind == "set-file-name":
        return {
            "kind": "set-file-name",
            "params": {
                "values": [
                    {"t": "string", "v": "/dev/null"},
                    {"t": "string", "v": "2026-03-20T21:51:27+00:00"},
                    {"t": "aggregate", "v": [{"t": "string", "v": ""}]},
                    {"t": "aggregate", "v": [{"t": "string", "v": ""}]},
                    {"t": "string", "v": "IfcOpenShell 0.8.4.post1"},
                    {"t": "string", "v": "IfcOpenShell 0.8.4.post1"},
                    {"t": "string", "v": "Nobody"},
                ]
            },
        }
    if kind == "insert-entity":
        return {"kind": "remove-entity", "params": {"id": 90001}}
    if kind == "set-entity-arg":
        return {"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"t": "string", "v": "b"}}}
    return {"kind": kind, "params": {}}


# endregion 🔖️Inverse


# region 🔖️Laws
def first_divergence(path: str, expected, actual):
    """🔍️ The first point at which two projections disagree, as a `path: expected != actual`
    sentence — a violated law must name the field that broke it, not dump two documents."""
    if isinstance(expected, dict) and isinstance(actual, dict):
        for key in expected:
            if key not in actual:
                return "%s.%s is absent from the second projection" % (path, key)
            found = first_divergence("%s.%s" % (path, key), expected[key], actual[key])
            if found:
                return found
        for key in actual:
            if key not in expected:
                return "%s.%s is absent from the first projection" % (path, key)
        return None
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return "%s has %d entries against %d" % (path, len(expected), len(actual))
        for index, (left, right) in enumerate(zip(expected, actual)):
            found = first_divergence("%s[%d]" % (path, index), left, right)
            if found:
                return found
        return None
    if isinstance(expected, float) or isinstance(actual, float):
        try:
            if abs(float(expected) - float(actual)) <= 1e-6:
                return None
        except (TypeError, ValueError):
            pass
    return None if expected == actual else "%s: %r against %r" % (path, expected, actual)


def observable(kind: str, baseline: dict, projection: dict) -> None:
    """👁️ Every row other than `no-mutation` MUST move the semantic projection. A row whose
    parameters make the mutation a no-op passes whenever the reference library merely declined to
    error, which is not a test."""
    if kind == "no-mutation":
        return
    if first_divergence("$", baseline, projection) is None:
        raise AssertionError("%r left IfcOpenShell's semantic projection of the IFC4 exchange structure unchanged — a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name" % kind)


# endregion 🔖️Laws


# region 🔖️Handlers
def no_mutation() -> dict:
    return {"kind": "no-mutation", "params": {}}


def spec_of(ctx: Context) -> dict:
    """📜️ The `(kind, params)` pair the scenario's own doc string carries — never a default."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def mutate(ctx: Context) -> Outcome:
    """🔮️ IfcOpenShell applies the named mutation and re-serializes; the from-scratch reader
    projects its own written bytes. The baseline runs one `no-mutation` cycle so the observability
    law isolates the mutation rather than IfcOpenShell's own normal form."""
    path = mutable_input(ctx)
    spec = spec_of(ctx)
    baseline = project(apply_mutation(path, no_mutation()).decode("utf-8"))
    produced = apply_mutation(path, spec)
    projection = project(produced.decode("utf-8"))
    observable(spec["kind"], baseline, projection)
    return Outcome(projection, produced)


def inverse(ctx: Context) -> Outcome:
    """↩️ The inverse law, checkable in role without a subject: IfcOpenShell applies the forward
    mutation and then the independently computed inverse, and the restored exchange structure MUST
    project exactly as the untouched one does. `no-mutation` is not short-circuited — it runs the
    same two cycles as every other kind, so the trivial case is evidence rather than an exemption."""
    path = mutable_input(ctx)
    spec = spec_of(ctx)
    kind = spec["kind"]
    baseline = project(apply_mutation(path, no_mutation()).decode("utf-8"))
    mutated = apply_mutation(path, spec)
    restored = apply_mutation_to_bytes(ctx, mutated, inverse_spec(kind))
    projection = project(restored.decode("utf-8"))
    found = first_divergence("$", baseline, projection)
    if found:
        raise AssertionError("inverse law violated for %r — undoing it did not restore the exchange structure: %s" % (kind, found))
    return Outcome(projection, restored)


def apply_mutation_to_bytes(ctx: Context, produced: bytes, spec: dict) -> bytes:
    """🧫️ The second cycle of an inverse pair, applied to the first cycle's own written bytes."""
    path = os.path.join(ctx.work_dir, "intermediate.ifc")
    with open(path, "wb") as handle:
        handle.write(produced)
    return apply_mutation(path, spec)


def identity_round_trip(ctx: Context) -> Outcome:
    """🔒️ IfcOpenShell decodes the whole real exchange structure into its own typed model and
    re-serializes from that model alone.

    📌️ NO byte tripwire here, and the reason is recorded rather than hidden: this fixture was itself
    exported by IfcOpenShell 0.8.4.post1, so its own writer is a fixed point of it and the output IS
    bit-identical to the input (measured: 2 496 437 bytes in, 2 496 437 identical bytes out). A
    "must not be bit-identical" assertion would fail for a correct implementation. What is asserted
    instead is that the document was genuinely parsed into a typed model: IfcOpenShell's own count
    of materialized `entity_instance` objects must equal the entity count the from-scratch text
    reader finds in the bytes it wrote — two independent counts of the same model, which a byte copy
    could not report. The sibling `../mutate-ifc-2x3` fixture, exported by a different tool, is
    where the same code path does visibly reformat (193 915 bytes in, 188 288 out)."""
    path = mutable_input(ctx)
    model = open_model(path)
    materialized = len(list(model))
    produced = model.to_string().encode("utf-8")
    projection = project(produced.decode("utf-8"))
    if materialized != projection["entityCount"]:
        raise AssertionError("IfcOpenShell materialized %d typed instances but its own written bytes carry %d entity records — the model was not fully parsed" % (materialized, projection["entityCount"]))
    before = project(open(path, "r", encoding="utf-8").read())
    found = first_divergence("$", before, projection)
    if found:
        raise AssertionError("identity round trip is not semantics-preserving: %s" % found)
    return Outcome(projection, produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by full expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("differential-%s" % kind, mutate).oracle("differential-inverse-%s" % kind, inverse)
    return built.oracle("differential-identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
