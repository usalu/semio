"""🐍️ Independent Python implementation of the `stdio.semio.table` carrier and its eight-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is
a second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope region
  (`wrap_text`/`split_text_preamble`/`wrap_binary`/`unwrap_binary`), the carrier's normative
  description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line columns-line rows-line`, `column = "[" hex "," cell-kind
  "]"`, `cell-kind = "n"|"b"|"i"|"f"|"s"|"y"`, `row = "[" list-item* "]"`, and the tag-prefixed
  `SemioValue` production it restates from `🔢️value`'s own grammar);
* the pack body is the derived record document of the shared record-document codec
  (`🧿️semio/🚪️io/💾️binary/🎒️record-document/🐍️.py`), driven by this subset's committed record spec
  `…/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json`: the SPK container, the fixed-Huffman deflate, the BLAKE3
  content hash and the schema-hashed record body are specified by `🧰️framework/🔨️modules/🎒️pack` and
  re-implemented there, not here. The committed `🎒️.pack.semio` files were written by the Rust record
  pack, so `parse_pack`/`pack_bytes` reproducing them is a cross-language byte agreement;
* the eight verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and the committed JSON schema
  `…/🧬️mutations/🔣️.json`, and what each verb MEANS is the committed per-kind
  `(before, mutation, after)` specification vector under `…/🧬️mutations/<kind>/🧪️tests/<slug>/` —
  a worked example per verb, which is the vocabulary's own normative statement of its semantics.

Nothing here imports, links, wraps or transliterates the Rust subject; no file under
`🧬️schema/🧬️mutations/<kind>/{🦠️mutation,↩️inverse,🔺️diff}/🦀️.rs` was read. Every function
was written against the documents above; where the two implementations disagree the disagreement is
a finding, not something to tune away.

🧫️ **Provenance of the complex artifact.** `shared://📊️mutate-semio-table/📝️reuse-marketplaces.dsl.semio` and its binary
twin were derived ONCE, by `derive_document_from_csv` below, from the real committed survey table
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv` — 50 data rows over
12 columns of real German building-material-reuse marketplace research, with commas, em dashes and
umlauts inside quoted fields. That source file is committed beside them as
`shared://📊️mutate-semio-table/📊️reuse-marketplaces.csv`, and `payload-fidelity` re-derives the document from it on every
run through Python's own `csv` module — an independent RFC 4180 implementation — so the fixture can
never silently drift away from the real data it claims to carry.
"""

from __future__ import annotations

# region 🔖️Imports
import csv
import importlib.util
import io
import json
import os
import sys

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
ENVELOPE_ID = "stdio.semio.table"
DSL_PREAMBLE = "semio %s.dsl v1" % ENVELOPE_ID

#: 🔤️ `cell-kind = "n" | "b" | "i" | "f" | "s" | "y"`, paired with the JSON tag each letter stands
#: for in the committed specification vectors.
KIND_LETTER = {"null": "n", "bool": "b", "int": "i", "float": "f", "str": "s", "bytes": "y"}
LETTER_KIND = {letter: kind for kind, letter in KIND_LETTER.items()}

VALUE_LETTER = {"null": "Z", "bool": "B", "int": "I", "float": "F", "str": "S", "bytes": "Y", "list": "L", "map": "M", "ref": "R"}
LETTER_VALUE = {letter: kind for kind, letter in VALUE_LETTER.items()}


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🪄️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    return bytes_of(hexed).decode("utf-8")


def bytes_of(hexed: str) -> bytes:
    """🔋️ The same macro read as raw octets — what a `Y[…]` cell carries."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed)


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


# endregion 🔖️Carrier


# region 🔖️Value
class Reader:
    """🔎️ A one-character-lookahead cursor. The `SemioValue` grammar needs no more: every variant is
    fixed by its leading tag letter, and every payload is bracket-delimited."""

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

    def hex(self) -> str:
        start = self.at
        while self.peek() in "0123456789abcdef" and self.peek() != "":
            self.at += 1
        return self.text[start : self.at]

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text at offset %d: %r" % (self.at, self.text[self.at :]))


def read_value(reader: Reader) -> dict:
    """🌳️ `value = "Z" | "B" "[" bit "]" | "I" "[" hex "]" | … | "R" "[" hex "]"`."""
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
        value = {"kind": kind, "lexeme": text_of(reader.hex())}
    elif kind == "str":
        value = {"kind": "str", "value": text_of(reader.hex())}
    elif kind == "bytes":
        value = {"kind": "bytes", "value": list(bytes_of(reader.hex()))}
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
            key = text_of(reader.hex())
            reader.take(":")
            entries.append({"key": key, "value": read_value(reader)})
            if reader.peek() == ",":
                reader.take(",")
        value = {"kind": "map", "entries": entries}
    else:
        value = {"kind": "ref", "id": {"value": text_of(reader.hex())}}
    reader.take("]")
    return value


def write_value(value: dict) -> str:
    """🌀️ The writing direction of `read_value`."""
    kind = value["kind"]
    if kind == "null":
        return "Z"
    if kind == "bool":
        return "B[%d]" % (1 if value["value"] else 0)
    if kind in ("int", "float"):
        return "%s[%s]" % (VALUE_LETTER[kind], hex_of(value["lexeme"]))
    if kind == "str":
        return "S[%s]" % hex_of(value["value"])
    if kind == "bytes":
        return "Y[%s]" % bytes(value["value"]).hex()
    if kind == "list":
        return "L[%s]" % ",".join(write_value(item) for item in value["items"])
    if kind == "map":
        return "M[%s]" % ",".join("%s:%s" % (hex_of(entry["key"]), write_value(entry["value"])) for entry in value["entries"])
    if kind == "ref":
        return "R[%s]" % hex_of(value["id"]["value"])
    raise AssertionError("unknown value kind %r" % kind)


# endregion 🔖️Value


# region 🔖️Dsl
def read_column(reader: Reader) -> dict:
    """🏛️ `column = "[" hex "," cell-kind "]"`."""
    reader.take("[")
    name = text_of(reader.hex())
    reader.take(",")
    letter = reader.letter()
    if letter not in LETTER_KIND:
        raise AssertionError("unknown cell-kind %r — the grammar declares n, b, i, f, s, y" % letter)
    reader.take("]")
    return {"name": name, "kind": LETTER_KIND[letter]}


def read_row(reader: Reader) -> dict:
    """📏️ `row = "[" list-item* "]"`, each `list-item` a `value` with an optional trailing comma."""
    reader.take("[")
    cells = []
    while reader.peek() != "]":
        cells.append(read_value(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    return {"cells": cells}


def read_bracketed(line: str, prefix: str, item):
    """🧱️ `<prefix> "=" "[" item-list? "]"` — the shape both the columns line and the rows line take."""
    if not line.startswith(prefix + "="):
        raise AssertionError("expected a %r line, found %r" % (prefix, line[:40]))
    reader = Reader(line[len(prefix) + 1 :])
    reader.take("[")
    found = []
    while reader.peek() != "]":
        found.append(item(reader))
        if reader.peek() == ",":
            reader.take(",")
    reader.take("]")
    reader.done()
    return found


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line columns-line rows-line`, under the text envelope."""
    body = [line.rstrip("\r") for line in split_preamble(text).split("\n") if line.strip() != ""]
    if len(body) != 3:
        raise AssertionError("a table document is exactly a schema, a columns and a rows line, found %d line(s)" % len(body))
    if not body[0].startswith("schema="):
        raise AssertionError("the first body line must be the schema line, found %r" % body[0])
    schema = text_of(body[0][len("schema=") :])
    if schema != ENVELOPE_ID:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, ENVELOPE_ID))
    return {"schema": schema, "columns": read_bracketed(body[1], "columns", read_column), "rows": read_bracketed(body[2], "rows", read_row)}


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    columns = ",".join("[%s,%s]" % (hex_of(column["name"]), KIND_LETTER[column["kind"]]) for column in document["columns"])
    rows = ",".join("[%s]" % ",".join(write_value(cell) for cell in row["cells"]) for row in document["rows"])
    return "%s\nschema=%s\ncolumns=[%s]\nrows=[%s]" % (DSL_PREAMBLE, hex_of(document["schema"]), columns, rows)


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
KINDS = ("create-column", "delete-column", "rename-column", "reorder-columns", "insert-row", "remove-row", "reorder-rows", "edit-cell")

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<slug>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-column": "CreateColumn",
    "delete-column": "DeleteColumn",
    "rename-column": "RenameColumn",
    "reorder-columns": "ReorderColumns",
    "insert-row": "InsertRow",
    "remove-row": "RemoveRow",
    "reorder-rows": "ReorderRows",
    "edit-cell": "EditCell",
}


def tagged(mutation: dict):
    """✂️ Splits `{"CreateColumn": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def clone(document: dict) -> dict:
    return json.loads(json.dumps(document))


def column_index(document: dict, name: str, verb: str) -> int:
    for at, column in enumerate(document["columns"]):
        if column["name"] == name:
            return at
    raise AssertionError("%s addresses the column %r, which this table does not carry (%s)" % (verb, name, ", ".join(column["name"] for column in document["columns"])))


def row_index(document: dict, index, verb: str) -> int:
    if not isinstance(index, int) or isinstance(index, bool) or index < 0 or index >= len(document["rows"]):
        raise AssertionError("%s addresses row %r, but the table has %d row(s)" % (verb, index, len(document["rows"])))
    return index


def apply_mutation(document: dict, mutation: dict) -> dict:
    """📱️ Applies one verb, returning a NEW document. An unaddressable column, row or position is a
    refusal, never a silent no-op — a quietly skipped mutation would report as a pass.

    The semantics are the committed specification vectors': `CreateColumn` inserts the column at its
    position and NULL-PADS every row there; `DeleteColumn` cascades into every row; `ReorderColumns`
    realigns every row's cells with the columns it moved; `ReorderRows` and `ReorderColumns` MOVE
    rather than swap; `EditCell` addresses its cell by row index and column NAME."""
    result = clone(document)
    tag, args = tagged(mutation)
    columns, rows = result["columns"], result["rows"]
    if tag == "CreateColumn":
        if any(column["name"] == args["name"] for column in columns):
            raise AssertionError("CreateColumn would duplicate the existing column %r" % args["name"])
        if args["kind"] not in KIND_LETTER:
            raise AssertionError("CreateColumn declares the unknown kind %r" % args["kind"])
        at = args.get("index")
        at = len(columns) if at is None else at
        if not isinstance(at, int) or at < 0 or at > len(columns):
            raise AssertionError("CreateColumn addresses position %r of %d column(s)" % (args.get("index"), len(columns)))
        columns.insert(at, {"name": args["name"], "kind": args["kind"]})
        for row in rows:
            row["cells"].insert(at, {"kind": "null"})
    elif tag == "DeleteColumn":
        at = column_index(result, args["name"], tag)
        del columns[at]
        for row in rows:
            del row["cells"][at]
    elif tag == "RenameColumn":
        at = column_index(result, args["name"], tag)
        if any(column["name"] == args["new_name"] for column in columns):
            raise AssertionError("RenameColumn would duplicate the existing column %r" % args["new_name"])
        columns[at]["name"] = args["new_name"]
    elif tag == "ReorderColumns":
        source = column_index(result, args["name"], tag)
        target = args["to_index"]
        if not isinstance(target, int) or target < 0 or target >= len(columns):
            raise AssertionError("ReorderColumns addresses position %r of %d column(s)" % (target, len(columns)))
        columns.insert(target, columns.pop(source))
        for row in rows:
            row["cells"].insert(target, row["cells"].pop(source))
    elif tag == "InsertRow":
        at = args["index"]
        if not isinstance(at, int) or at < 0 or at > len(rows):
            raise AssertionError("InsertRow addresses position %r of %d row(s)" % (at, len(rows)))
        rows.insert(at, clone(args["row"]))
    elif tag == "RemoveRow":
        del rows[row_index(result, args["index"], tag)]
    elif tag == "ReorderRows":
        source = row_index(result, args["from"], tag)
        target = row_index(result, args["to"], tag)
        rows.insert(target, rows.pop(source))
    else:
        at = row_index(result, args["row_index"], tag)
        column = column_index(result, args["column_name"], tag)
        rows[at]["cells"][column] = clone(args["new_value"])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to, as a SEQUENCE — a deleted column
    cannot be restored by one verb, because `CreateColumn` can only null-pad the rows it reopens, so
    every displaced cell has to be written back by its own `EditCell`."""
    tag, args = tagged(mutation)
    if tag == "CreateColumn":
        return [{"DeleteColumn": {"name": args["name"]}}]
    if tag == "DeleteColumn":
        at = column_index(document, args["name"], tag)
        column = document["columns"][at]
        undo = [{"CreateColumn": {"name": column["name"], "kind": column["kind"], "index": at}}]
        for index, row in enumerate(document["rows"]):
            undo.append({"EditCell": {"row_index": index, "column_name": column["name"], "new_value": row["cells"][at]}})
        return undo
    if tag == "RenameColumn":
        return [{"RenameColumn": {"name": args["new_name"], "new_name": args["name"]}}]
    if tag == "ReorderColumns":
        return [{"ReorderColumns": {"name": args["name"], "to_index": column_index(document, args["name"], tag)}}]
    if tag == "InsertRow":
        return [{"RemoveRow": {"index": args["index"]}}]
    if tag == "RemoveRow":
        return [{"InsertRow": {"index": args["index"], "row": document["rows"][row_index(document, args["index"], tag)]}}]
    if tag == "ReorderRows":
        return [{"ReorderRows": {"from": args["to"], "to": args["from"]}}]
    at = row_index(document, args["row_index"], tag)
    column = column_index(document, args["column_name"], tag)
    return [{"EditCell": {"row_index": at, "column_name": args["column_name"], "new_value": document["rows"][at]["cells"][column]}}]


# endregion 🔖️Mutations


# region 🔖️Derivation
def derive_document_from_csv(raw: bytes) -> dict:
    """🧫️ The real committed survey table as an `stdio.semio.table` document — the ONE derivation
    that produced `shared://📊️mutate-semio-table/📝️reuse-marketplaces.dsl.semio`, kept here and re-run by
    `payload-fidelity` so the fixture can never drift from the CSV it claims to carry.

    It is a faithful transcription and nothing more: the header record names the columns, every
    column is declared `str` because every field of the source is text, and every cell carries its
    field verbatim — commas, em dashes and umlauts inside quoted fields included. Python's own `csv`
    module does the RFC 4180 tokenizing, so the payload has a second reader independent of this
    repository's Rust one."""
    records = list(csv.reader(io.StringIO(raw.decode("utf-8"), newline=""), strict=True))
    if len(records) < 2:
        raise AssertionError("the source survey table carries a header and at least one data record")
    header, data = records[0], records[1:]
    for at, record in enumerate(data):
        if len(record) != len(header):
            raise AssertionError("record %d has %d field(s), the header declares %d" % (at, len(record), len(header)))
    return {
        "schema": ENVELOPE_ID,
        "columns": [{"name": name, "kind": "str"} for name in header],
        "rows": [{"cells": [{"kind": "str", "value": field} for field in record]} for record in data],
    }


# endregion 🔖️Derivation


# region 🔖️Scenario input
SHEET_DSL = "asset://📃️sheet/🗣️.dsl.semio"
SHEET_PACK = "asset://📃️sheet/🎒️.pack.semio"
SURVEY_CSV = "shared://📊️mutate-semio-table/♻️reuse-marketplaces/📊️.csv"
SURVEY_DSL = "shared://📊️mutate-semio-table/📝️reuse-marketplaces.dsl.semio"
SURVEY_PACK = "shared://📊️mutate-semio-table/📦️reuse-marketplaces.pack.semio"


def doc_string(ctx: Context) -> str:
    """🔌️ The scenario's own committed parameters — the feature owns them, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def survey(ctx: Context) -> dict:
    """📊️ The real complex table, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.input_bytes(SURVEY_DSL).decode("utf-8"))


def snapshot_json(ctx: Context, uri: str) -> dict:
    """➡️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real complex table by this implementation alone."""
    document = survey(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """🌁️ The same verb, then this implementation's own undo of it. The restored table must be the
    survey again — asserted here, and compared against the subject's restored table by the runner,
    so a wrong undo that happens to be self-consistent still shows up."""
    document = survey(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in inverse_mutation(document, mutation):
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("undoing %s did not restore the survey table\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored, ensure_ascii=False), json.dumps(document, ensure_ascii=False)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🌂️ The same verb on its committed handcrafted `(before, mutation, after)` vector. The vector
    is a THIRD statement of what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = ctx.step_input_uris()[:3]
    before = snapshot_json(ctx, before_uri)
    after = snapshot_json(ctx, after_uri)
    applied = apply_mutation(before, snapshot_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied table does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied, ensure_ascii=False), json.dumps(after, ensure_ascii=False)))
    return Outcome(applied)


def payload_fidelity(ctx: Context) -> Outcome:
    """🪑️ The derived fixture against the real CSV it was derived from, re-tokenized on every run by
    Python's own `csv` module — an RFC 4180 implementation with no connection to this repository."""
    derived = derive_document_from_csv(ctx.input_bytes(SURVEY_CSV))
    committed = survey(ctx)
    if derived != committed:
        raise AssertionError("the committed survey document no longer matches the CSV it was derived from")
    return Outcome({"document": derived, "columns": len(derived["columns"]), "rows": len(derived["rows"]), "cells": sum(len(row["cells"]) for row in derived["rows"])})


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the small demo sheet, plus the real complex survey table, each
    re-emitted from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps the demo sheet from being vacuous is that its bytes were written by the OTHER
    implementation: this file reproducing them is a cross-language byte agreement. The survey table's
    bytes were written by THIS implementation, so the agreement runs the other way — the Rust subject
    has to read and reproduce a document a Python writer produced from the grammar."""
    sheet_dsl = ctx.input_bytes(SHEET_DSL)
    sheet = parse_dsl(sheet_dsl.decode("utf-8"))
    printed = print_dsl(sheet).encode("utf-8")
    if printed != sheet_dsl:
        raise AssertionError("re-printing the demo sheet did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(sheet_dsl)))
    committed_pack = ctx.input_bytes(SHEET_PACK)
    unpacked = parse_pack(committed_pack)
    if unpacked != sheet:
        raise AssertionError("the committed binary twin decodes to a different sheet than the committed text\n     got: %s\nexpected: %s" % (json.dumps(unpacked, ensure_ascii=False), json.dumps(sheet, ensure_ascii=False)))
    repacked = pack_bytes(sheet)
    if repacked != committed_pack:
        raise AssertionError("re-encoding the demo sheet did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(committed_pack)))
    survey_dsl = ctx.input_bytes(SURVEY_DSL)
    document = parse_dsl(survey_dsl.decode("utf-8"))
    survey_printed = print_dsl(document).encode("utf-8")
    if survey_printed != survey_dsl:
        raise AssertionError("re-printing the survey table did not reproduce its committed DSL bytes (%d vs %d bytes)" % (len(survey_printed), len(survey_dsl)))
    survey_pack = ctx.input_bytes(SURVEY_PACK)
    if parse_pack(survey_pack) != document:
        raise AssertionError("the survey table's binary twin decodes to a different table than its text")
    survey_repacked = pack_bytes(document)
    if survey_repacked != survey_pack:
        raise AssertionError("re-encoding the survey table did not reproduce its committed pack bytes (%d vs %d bytes)" % (len(survey_repacked), len(survey_pack)))
    return Outcome(
        {
            "sheet": sheet,
            "sheetDslDigest": digest(printed),
            "sheetPackDigest": digest(repacked),
            "surveyDslDigest": digest(survey_printed),
            "surveyPackDigest": digest(survey_repacked),
            "surveyDslLength": len(survey_printed),
            "surveyPackLength": len(survey_repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("payload-fidelity", payload_fidelity).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
