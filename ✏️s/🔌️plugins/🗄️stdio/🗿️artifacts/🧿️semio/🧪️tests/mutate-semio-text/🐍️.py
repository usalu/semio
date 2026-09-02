"""🐍️ Independent Python implementation of the `s.stdio.semio.text` carrier and its seven-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section,
  which is the carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line runs-line`, hex-encoded scalars, `mark-kind = b|i|c|l`);
* the pack body is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio` (`format u8`, then varint-length-prefixed
  UTF-8 `schema`), whose description stops at the `runs` array by its own admission — the repeated
  record layout below was DERIVED from the committed `🎒️example.pack.semio` bytes, field order taken
  from the grammar, and `pack_bytes` re-encodes that committed file byte for byte, which is what
  proves the derivation right;
* the seven verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Carrier
BINARY_MAGIC = b"\x89SEM\x0d\x0a\x1a\x0a"
DSL_PREAMBLE = "semio s.stdio.semio.text.dsl v1"
PACK_TOKEN = "s.stdio.semio.text.pack v1"
DOCUMENT_SCHEMA = "s.stdio.semio.text"
PACK_FORMAT = 1

#: 🔤️ `mark-kind = "b" | "i" | "c" | "l"`, in the grammar's own order — which is also the ordinal
#: the pack frame writes, as the committed example's `bold` → `0x00` and `link` → `0x03` show.
MARK_ORDER = ("bold", "italic", "code", "link")
MARK_LETTER = {"bold": "b", "italic": "i", "code": "c", "link": "l"}
LETTER_MARK = {letter: kind for kind, letter in MARK_LETTER.items()}

#: 📰️ The document every mutation row runs on: 384 real runs of the real German article "Zukunft
#: Bau: Entwerfen mit Bestand", derived ONCE from this repository's own committed HTML 5 fixture by
#: `🐍️derive-text-fixture.py` in the ticket folder.
ARTICLE_DSL = "local://🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio"
ARTICLE_PACK = "local://🎒️zukunft-bau-entwerfen-mit-bestand.pack.semio"
#: 🗣️ The tiny committed note, kept for the BYTE half of the identity law: its two files were
#: written by the RUST codec, so this implementation reproducing them is a cross-language byte
#: agreement the article pair — written by this implementation — cannot restate.
NOTE_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🗣️.dsl.semio"
NOTE_PACK = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🎒️example.pack.semio"


def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def split_preamble(text: str) -> str:
    """📜️ Strips the mandatory text envelope, refusing any preamble but this subset's own."""
    first, _, rest = text.partition("\n")
    if first.strip() != DSL_PREAMBLE:
        raise AssertionError("expected the %r preamble, got %r" % (DSL_PREAMBLE, first.strip()))
    return rest.lstrip("\r\n")


# endregion 🔖️Carrier


# region 🔖️Dsl
class Reader:
    """🔎️ A one-character-lookahead cursor — enough for a grammar whose only ambiguity is that a
    `mark-kind` letter is also a hex digit, which the fixed `"[" mark-kind "," …` shape resolves."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.at = 0

    def peek(self) -> str:
        return self.text[self.at] if self.at < len(self.text) else ""

    def take(self, char: str) -> None:
        if self.peek() != char:
            raise AssertionError("expected %r at offset %d of the runs line, found %r" % (char, self.at, self.peek()))
        self.at += 1

    def letter(self) -> str:
        char = self.peek()
        if char == "":
            raise AssertionError("the runs line ends inside a mark")
        self.at += 1
        return char

    def hex(self) -> str:
        start = self.at
        while self.peek() in "0123456789abcdef" and self.peek() != "":
            self.at += 1
        return text_of(self.text[start : self.at])

    def done(self) -> None:
        if self.at != len(self.text):
            raise AssertionError("trailing text after the runs line: %r" % self.text[self.at :])


def read_mark(reader: Reader) -> dict:
    """🏷️ `mark = "[" mark-kind "," hex "]"`."""
    reader.take("[")
    letter = reader.letter()
    if letter not in LETTER_MARK:
        raise AssertionError("unknown mark-kind %r — the grammar declares b, i, c, l" % letter)
    reader.take(",")
    href = reader.hex()
    reader.take("]")
    return {"kind": LETTER_MARK[letter], "href": href}


def read_run(reader: Reader) -> dict:
    """🏃️ `run = "[" hex "," hex "," "[" mark-list? "]" "]"` — language, content, marks."""
    reader.take("[")
    language = reader.hex()
    reader.take(",")
    content = reader.hex()
    reader.take(",")
    reader.take("[")
    marks = []
    if reader.peek() != "]":
        marks.append(read_mark(reader))
        while reader.peek() == ",":
            reader.take(",")
            marks.append(read_mark(reader))
    reader.take("]")
    reader.take("]")
    return {"language": language, "content": content, "marks": marks}


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line runs-line`, under the text envelope."""
    lines = [line.rstrip("\r") for line in split_preamble(text).split("\n")]
    body = [line for line in lines if line != ""]
    if len(body) != 2:
        raise AssertionError("a text document is exactly a schema line and a runs line, found %d line(s)" % len(body))
    if not body[0].startswith("schema="):
        raise AssertionError("the first body line must be the schema line, found %r" % body[0])
    if not body[1].startswith("runs="):
        raise AssertionError("the second body line must be the runs line, found %r" % body[1])
    schema = text_of(body[0][len("schema=") :])
    if schema != DOCUMENT_SCHEMA:
        raise AssertionError("the artifact-mark is %r, not %r" % (schema, DOCUMENT_SCHEMA))
    reader = Reader(body[1][len("runs=") :])
    reader.take("[")
    runs = []
    if reader.peek() != "]":
        runs.append(read_run(reader))
        while reader.peek() == ",":
            reader.take(",")
            runs.append(read_run(reader))
    reader.take("]")
    reader.done()
    return {"schema": schema, "runs": runs}


def print_dsl(document: dict) -> str:
    """✍️ The writing direction of the same grammar, under the same envelope."""
    runs = ",".join(
        "[%s,%s,[%s]]"
        % (
            hex_of(run["language"]),
            hex_of(run["content"]),
            ",".join("[%s,%s]" % (MARK_LETTER[mark["kind"]], hex_of(mark["href"])) for mark in run["marks"]),
        )
        for run in document["runs"]
    )
    return "%s\nschema=%s\nruns=[%s]" % (DSL_PREAMBLE, hex_of(document["schema"]), runs)


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
    """🧵️ A varint-length-prefixed UTF-8 string, the protocol's only scalar past the header."""
    length, at = read_varint(data, at)
    if at + length > len(data):
        raise AssertionError("the pack frame ends inside a length-prefixed string")
    return data[at : at + length].decode("utf-8"), at + length


def write_string(text: str) -> bytes:
    """🧵️ The writing direction of the same scalar."""
    raw = text.encode("utf-8")
    return write_varint(len(raw)) + raw


def parse_pack(data: bytes) -> dict:
    """📦️ Binary envelope, then `format u8`, then the schema, then the repeated run records."""
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
    runs = []
    for _ in range(count):
        language, at = read_string(data, at)
        content, at = read_string(data, at)
        marks_count, at = read_varint(data, at)
        marks = []
        for _ in range(marks_count):
            ordinal = data[at]
            at += 1
            if ordinal >= len(MARK_ORDER):
                raise AssertionError("unknown mark ordinal %d" % ordinal)
            href, at = read_string(data, at)
            marks.append({"kind": MARK_ORDER[ordinal], "href": href})
        runs.append({"language": language, "content": content, "marks": marks})
    if at != len(data):
        raise AssertionError("%d trailing byte(s) after the last run record" % (len(data) - at))
    return {"schema": schema, "runs": runs}


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    body = bytearray([PACK_FORMAT])
    body += write_string(document["schema"])
    body += write_varint(len(document["runs"]))
    for run in document["runs"]:
        body += write_string(run["language"])
        body += write_string(run["content"])
        body += write_varint(len(run["marks"]))
        for mark in run["marks"]:
            body.append(MARK_ORDER.index(mark["kind"]))
            body += write_string(mark["href"])
    token = PACK_TOKEN.encode("utf-8")
    return BINARY_MAGIC + len(token).to_bytes(4, "little") + token + bytes(body)


# endregion 🔖️Pack


# region 🔖️Mutations
KINDS = ("insert-run", "remove-run", "edit-run", "change-run-language", "reorder-runs", "add-mark", "remove-mark")

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "insert-run": "InsertRun",
    "remove-run": "RemoveRun",
    "edit-run": "EditRun",
    "change-run-language": "ChangeRunLanguage",
    "reorder-runs": "ReorderRuns",
    "add-mark": "AddMark",
    "remove-mark": "RemoveMark",
}


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"InsertRun": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def run_at(document: dict, index: int, verb: str) -> dict:
    if not isinstance(index, int) or index < 0 or index >= len(document["runs"]):
        raise AssertionError("%s addresses run %r, but the document has %d run(s)" % (verb, index, len(document["runs"])))
    return document["runs"][index]


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable index is a refusal, never a
    silent no-op — a quietly skipped mutation would report as a pass."""
    result = json.loads(json.dumps(document))
    tag, args = tagged(mutation)
    runs = result["runs"]
    if tag == "InsertRun":
        index = args["index"]
        if not isinstance(index, int) or index < 0 or index > len(runs):
            raise AssertionError("InsertRun addresses position %r, but the document has %d run(s)" % (index, len(runs)))
        runs.insert(index, json.loads(json.dumps(args["run"])))
    elif tag == "RemoveRun":
        run_at(result, args["index"], tag)
        del runs[args["index"]]
    elif tag == "EditRun":
        run_at(result, args["index"], tag)["content"] = args["new_content"]
    elif tag == "ChangeRunLanguage":
        run_at(result, args["index"], tag)["language"] = args["new_language"]
    elif tag == "ReorderRuns":
        source, target = args["from"], args["to"]
        run_at(result, source, tag)
        run_at(result, target, tag)
        runs.insert(target, runs.pop(source))
    elif tag == "AddMark":
        marks = run_at(result, args["run_index"], tag)["marks"]
        index = args["index"]
        if not isinstance(index, int) or index < 0 or index > len(marks):
            raise AssertionError("AddMark addresses mark %r of a run carrying %d mark(s)" % (index, len(marks)))
        marks.insert(index, json.loads(json.dumps(args["mark"])))
    else:
        marks = run_at(result, args["run_index"], tag)["marks"]
        index = args["index"]
        if not isinstance(index, int) or index < 0 or index >= len(marks):
            raise AssertionError("RemoveMark addresses mark %r of a run carrying %d mark(s)" % (index, len(marks)))
        del marks[index]
    return result


def inverse_mutation(document: dict, mutation: dict) -> dict:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an insertion is undone by a removal at the position it took, an overwrite by an
    overwrite with the value it displaced, and a move by the opposite move."""
    tag, args = tagged(mutation)
    if tag == "InsertRun":
        return {"RemoveRun": {"index": args["index"]}}
    if tag == "RemoveRun":
        return {"InsertRun": {"index": args["index"], "run": run_at(document, args["index"], tag)}}
    if tag == "EditRun":
        return {"EditRun": {"index": args["index"], "new_content": run_at(document, args["index"], tag)["content"]}}
    if tag == "ChangeRunLanguage":
        return {"ChangeRunLanguage": {"index": args["index"], "new_language": run_at(document, args["index"], tag)["language"]}}
    if tag == "ReorderRuns":
        return {"ReorderRuns": {"from": args["to"], "to": args["from"]}}
    if tag == "AddMark":
        return {"RemoveMark": {"run_index": args["run_index"], "index": args["index"]}}
    marks = run_at(document, args["run_index"], tag)["marks"]
    index = args["index"]
    if index < 0 or index >= len(marks):
        raise AssertionError("RemoveMark addresses mark %r of a run carrying %d mark(s)" % (index, len(marks)))
    return {"AddMark": {"run_index": args["run_index"], "index": index, "mark": marks[index]}}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own committed vector — the feature owns the parameters, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def step_assets(ctx: Context) -> list:
    """🧫️ Every `asset://` URI the scenario's steps name, in step order. The feature is the single
    place the specification-vector paths are written down; both adapters read them from here."""
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


def note(ctx: Context) -> dict:
    """📰️ The real article, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(ARTICLE_DSL).decode("utf-8"))


def fixture_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed note by this implementation alone."""
    document = note(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored document must be
    the note again — asserted here, and compared against the subject's restored document by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document = note(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = apply_mutation(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the note\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
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
    """🔁️ One document's two encodings, each re-emitted from the parsed document and required back
    byte for byte. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    twin, so an exact re-emission is the CORRECT answer and the must-differ tripwire would be
    backwards here."""
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
        raise AssertionError("the binary twin of %s decodes to a different document than its text\n     got: %s\nexpected: %s" % (what, json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding %s did not reproduce its committed pack bytes (%d vs %d bytes)" % (what, len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack of %s lost content" % what)
    return {"document": document, "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both documents, in both encodings — four files, all four reproduced byte for byte.

    The committed note's two files were written by the RUST codec, so this implementation
    reproducing them is a cross-language byte agreement, not a codec agreeing with itself. The
    article's two files were written by THIS implementation from the grammar and the protocol, so
    the Rust codec has to reproduce THOSE — 384 runs and 344 marks among them.
    """
    note_report = carrier_pair(ctx, NOTE_DSL, NOTE_PACK, "the committed note")
    runs = note_report["document"]["runs"]
    if len(runs) != 3 or {run["language"] for run in runs} != {"en", "de"}:
        raise AssertionError("the committed note is the three-run English/German artifact this case describes, but decoded as %r" % runs)
    article_report = carrier_pair(ctx, ARTICLE_DSL, ARTICLE_PACK, "the article")
    article = article_report["document"]["runs"]
    marks = [mark for run in article for mark in run["marks"]]
    if len(article) != 384 or len(marks) != 344:
        raise AssertionError("the article is the 384-run 344-mark document this case describes, but decoded as %d runs and %d marks" % (len(article), len(marks)))
    if {mark["kind"] for mark in marks} != {"bold", "link"}:
        raise AssertionError("the article carries the page's own `<strong>` and `<a href>` inline structure, which this decoding contradicts")
    return Outcome({"note": note_report, "article": article_report})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector)
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
