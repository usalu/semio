"""🐍️ Independent Python implementation of the `s.stdio.semio.video` carrier and its nine-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio` is a semio-native carrier that no
third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is a
second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the mandatory `semio <envelope-id>.dsl v<version>` preamble line — is specified in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line streams-line`,
  `stream = "[" kind "," hex "," INT "," INT "," rational "," "[" sample-list? "]" "]"`,
  `sample = "[" INT "," bool "," hex "]"`, `rational = "[" INT "," INT "]"`,
  `kind = "V" | "A" | "S"` and `bool = "0" | "1"`;
* the nine verbs and their named argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is this
  case's committed per-kind specification vectors under `🧫️fixtures/`.

`hex` is declared by both grammars to be the framework's built-in `hex` MACRO, and their own notes
distinguish its two uses: a `schema`/`codec` leaf is the lowercase hex of that string's UTF-8 bytes,
while a sample `data` leaf is an uninterpreted byte run. The committed artifact confirms both —
`68323634` is `h264`, and the first sample's `00010203` is four opaque bytes. The projection keeps
`data` in its hex spelling for exactly that reason: the payload is opaque to this subset and
inventing a decoding for it would be inventing a fact the format does not carry.

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two disagree the disagreement is a finding, not something to
tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every variant of this subset's mutation vocabulary, in the order the mutations grammar's `op`
#: production lists them, kebab-cased as the catalog spells them.
KINDS = (
    "no-mutation",
    "set-snapshot",
    "insert-stream",
    "remove-stream",
    "set-stream-meta",
    "insert-sample",
    "remove-sample",
    "set-sample-data",
    "set-sample-flags",
)

#: 🎬️ `kind = "V" | "A" | "S"`, verbatim from the grammar.
STREAM_KINDS = ("V", "A", "S")

DOCUMENT_SCHEMA = "stdio.semio.video"
DSL_PREAMBLE = "semio stdio.semio.video.dsl v1"

#: 🎬️ The document every mutation row runs on: two real streams of the real "Bauen mit Bestand"
#: recording — eight real MJPEG frames of the committed AVI and twenty-four real MPEG-1 Layer III
#: frames of the committed mp3 — derived ONCE by `🐍️derive-video-fixture.py` in the ticket folder.
RECORDING_DSL = "local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio"
#: 🎥️ The tiny committed clip, kept for the BYTE half of the identity law and for the tie to the
#: committed specification vectors: its file was written by the RUST codec, so this implementation
#: reproducing it is a cross-language byte agreement the recording cannot restate.
CLIP_DSL = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️.dsl.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    return raw_hex(hexed).decode("utf-8")


def raw_hex(hexed: str) -> bytes:
    """🔡️ The same macro over an opaque byte run — a sample `data` leaf is not text."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    if any(char not in "0123456789abcdef" for char in hexed):
        raise AssertionError("hex run %r carries a digit outside the macro's alphabet" % hexed)
    return bytes.fromhex(hexed)


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
    """🔎️ The body of a `"[" … "]"` group, refusing anything that is not one."""
    if len(text) < 2 or text[0] != "[" or text[-1] != "]":
        raise AssertionError("expected a bracketed group, got %r" % text)
    return text[1:-1]


def integer(text: str, what: str) -> int:
    """🔢️ One `INT` leaf."""
    if not text.isdigit():
        raise AssertionError("%s is an INT, got %r" % (what, text))
    return int(text)


def boolean(text: str, what: str) -> bool:
    """🔘️ `bool = "0" | "1"`."""
    if text not in ("0", "1"):
        raise AssertionError("%s is a bool spelled 0 or 1, got %r" % (what, text))
    return text == "1"


def field(lines: list, name: str) -> str:
    """🔎️ The right-hand side of one `name "=" value` body line; a missing line is an error."""
    prefix = name + "="
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix) :]
    raise AssertionError("the document has no %r line — the grammar declares it mandatory" % name)


# endregion 🔖️Carrier


# region 🔖️Dsl
def read_rational(text: str) -> dict:
    """🔢️ `rational = "[" INT "," INT "]"`."""
    num, den = split_top_level(bracketed(text))
    return {"num": integer(num, "a rational numerator"), "den": integer(den, "a rational denominator")}


def read_sample(text: str) -> dict:
    """🎞️ `sample = "[" INT "," bool "," hex "]"` — presentation stamp, key flag, opaque payload."""
    pts, key, data = split_top_level(bracketed(text))
    raw_hex(data)
    return {"pts": integer(pts, "a sample pts"), "key": boolean(key, "a sample key flag"), "data": data}


def read_stream(text: str) -> dict:
    """🎬️ `stream = "[" kind "," hex "," INT "," INT "," rational "," "[" sample-list? "]" "]"`."""
    kind, codec, width, height, rate, samples = split_top_level(bracketed(text))
    if kind not in STREAM_KINDS:
        raise AssertionError("unknown stream kind %r — the grammar declares %s" % (kind, ", ".join(STREAM_KINDS)))
    return {
        "kind": kind,
        "codec": text_of(codec),
        "width": integer(width, "a stream width"),
        "height": integer(height, "a stream height"),
        "rate": read_rational(rate),
        "samples": [read_sample(each) for each in split_top_level(bracketed(samples))],
    }


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line streams-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    return {
        "schema": text_of(field(lines, "schema")),
        "streams": [read_stream(each) for each in split_top_level(bracketed(field(lines, "streams")))],
    }


def write_sample(sample: dict) -> str:
    return "[%d,%s,%s]" % (sample["pts"], "1" if sample["key"] else "0", sample["data"])


def write_stream(stream: dict) -> str:
    return "[%s,%s,%d,%d,[%d,%d],[%s]]" % (
        stream["kind"],
        hex_of(stream["codec"]),
        stream["width"],
        stream["height"],
        stream["rate"]["num"],
        stream["rate"]["den"],
        ",".join(write_sample(sample) for sample in stream["samples"]),
    )


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    body = ["schema=%s" % hex_of(snapshot["schema"]), "streams=[%s]" % ",".join(write_stream(stream) for stream in snapshot["streams"])]
    return "\n".join([DSL_PREAMBLE] + body)


# endregion 🔖️Dsl


# region 🔖️Mutations
def clone(value):
    """🧬️ A structural copy, so applying a verb never writes through into the parsed document."""
    return json.loads(json.dumps(value))


def parts(mutation: dict) -> tuple:
    """🔎️ Splits `{"kind": …, "params": {…}}` into its verb and its arguments."""
    kind = mutation.get("kind")
    if kind not in KINDS:
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (kind, ", ".join(KINDS)))
    return kind, mutation.get("params") or {}


def index_at(count: int, index, verb: str, inclusive: bool) -> int:
    """🔢️ One positional index argument, checked against the collection it addresses. An
    unaddressable index is a refusal, never a silent no-op."""
    limit = count if inclusive else count - 1
    if not isinstance(index, int) or isinstance(index, bool) or index < 0 or index > limit:
        raise AssertionError("%s addresses position %r of a collection holding %d entry/entries" % (verb, index, count))
    return index


def stream_at(snapshot: dict, args: dict, verb: str) -> dict:
    return snapshot["streams"][index_at(len(snapshot["streams"]), args["streamIndex"], verb, False)]


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW snapshot."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        return clone(args["snapshot"])
    if kind == "insert-stream":
        result["streams"].insert(index_at(len(result["streams"]), args["index"], kind, True), clone(args["stream"]))
        return result
    if kind == "remove-stream":
        del result["streams"][index_at(len(result["streams"]), args["index"], kind, False)]
        return result
    if kind == "set-stream-meta":
        stream = result["streams"][index_at(len(result["streams"]), args["index"], kind, False)]
        if args["kind"] not in STREAM_KINDS:
            raise AssertionError("set-stream-meta names stream kind %r, which the grammar does not declare" % args["kind"])
        stream.update({"kind": args["kind"], "codec": args["codec"], "width": args["width"], "height": args["height"], "rate": clone(args["rate"])})
        return result
    stream = stream_at(result, args, kind)
    if kind == "insert-sample":
        stream["samples"].insert(index_at(len(stream["samples"]), args["index"], kind, True), clone(args["sample"]))
        return result
    if kind == "remove-sample":
        del stream["samples"][index_at(len(stream["samples"]), args["index"], kind, False)]
        return result
    sample = stream["samples"][index_at(len(stream["samples"]), args["index"], kind, False)]
    if kind == "set-sample-data":
        raw_hex(args["data"])
        sample["data"] = args["data"]
        return result
    sample["pts"] = args["pts"]
    sample["key"] = args["key"]
    return result


def inverse_mutation(snapshot: dict, mutation: dict) -> dict:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an insertion is undone by a removal at the position it took, and an overwrite by an
    overwrite with the value it displaced."""
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return {"kind": "no-mutation", "params": {}}
    if kind == "set-snapshot":
        return {"kind": "set-snapshot", "params": {"snapshot": clone(snapshot)}}
    if kind == "insert-stream":
        return {"kind": "remove-stream", "params": {"index": args["index"]}}
    if kind == "remove-stream":
        index = index_at(len(snapshot["streams"]), args["index"], kind, False)
        return {"kind": "insert-stream", "params": {"index": index, "stream": clone(snapshot["streams"][index])}}
    if kind == "set-stream-meta":
        was = snapshot["streams"][index_at(len(snapshot["streams"]), args["index"], kind, False)]
        return {"kind": "set-stream-meta", "params": {"index": args["index"], "kind": was["kind"], "codec": was["codec"], "width": was["width"], "height": was["height"], "rate": clone(was["rate"])}}
    stream = stream_at(snapshot, args, kind)
    if kind == "insert-sample":
        return {"kind": "remove-sample", "params": {"streamIndex": args["streamIndex"], "index": args["index"]}}
    index = index_at(len(stream["samples"]), args["index"], kind, False)
    if kind == "remove-sample":
        return {"kind": "insert-sample", "params": {"streamIndex": args["streamIndex"], "index": index, "sample": clone(stream["samples"][index])}}
    if kind == "set-sample-data":
        return {"kind": "set-sample-data", "params": {"streamIndex": args["streamIndex"], "index": index, "data": stream["samples"][index]["data"]}}
    return {"kind": "set-sample-flags", "params": {"streamIndex": args["streamIndex"], "index": index, "pts": stream["samples"][index]["pts"], "key": stream["samples"][index]["key"]}}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def clip(ctx: Context) -> dict:
    """🎬️ The real recording, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(RECORDING_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.fixture_bytes("local://🦠️%s.json" % kind).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed clip by this implementation alone."""
    result = apply_mutation(clip(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the clip again — asserted here, and the MUTATED snapshot travels in the projection too, so the
    nine rows cannot all project the same restored value and compare vacuously."""
    snapshot = clip(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the clip\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(kind: str):
    """🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    what the verb means, independent of both implementations, kept from before this oracle existed."""

    def handler(ctx: Context) -> Outcome:
        committed = vector(ctx, kind)
        applied = apply_mutation(committed["before"], {"kind": committed["kind"], "params": committed["params"]})
        if applied != committed["after"]:
            raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(committed["after"])))
        return Outcome(applied)

    return handler


def carrier_once(ctx: Context, uri: str, what: str) -> dict:
    """🔁️ One document, re-emitted from the parsed snapshot and required back byte for byte.
    `.dsl.semio` is a fixed-layout record grammar, so an exact re-emission is the CORRECT answer and
    the must-differ tripwire would be backwards. `✳️video` exports no pack bridge, so no claim is
    made about any binary twin — one carrier measured, the other named as unmeasured."""
    committed = ctx.fixture_bytes(uri)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing %s did not reproduce its committed DSL bytes (%d vs %d bytes)" % (what, len(printed), len(committed)))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed %s lost content" % what)
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("%s declares schema %r, expected %r" % (what, snapshot["schema"], DOCUMENT_SCHEMA))
    return {"document": snapshot, "dslDigest": digest(printed), "dslLength": len(printed)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both documents, re-emitted from their parsed snapshots.

    The committed clip's file was written by the RUST codec, so this implementation reproducing it is
    a cross-language byte agreement, not a codec agreeing with itself, and it is also the document
    every committed specification vector starts from — both ties are asserted here. The recording's
    file was written by THIS implementation from the grammar alone, so the Rust codec has to
    reproduce THAT, 32 real coded frames among them.
    """
    clip_report = carrier_once(ctx, CLIP_DSL, "the committed clip")
    declared = vector(ctx, "no-mutation")["before"]
    if clip_report["document"] != declared:
        raise AssertionError("the real committed clip does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(clip_report["document"]), json.dumps(declared)))
    recording = carrier_once(ctx, RECORDING_DSL, "the recording")
    streams = recording["document"]["streams"]
    shape = [(stream["kind"], stream["codec"], len(stream["samples"])) for stream in streams]
    if shape != [("V", "MJPG", 8), ("A", "mp3", 24)]:
        raise AssertionError("the recording is the MJPG/mp3 two-stream document this case describes, but decoded as %r" % (shape,))
    return Outcome({"clip": clip_report, "recording": recording})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector(kind))
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
