"""🐍️ Independent Python implementation of the `s.stdio.semio.audio` carrier and its ten-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio` is a semio-native carrier that no
third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is a
second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the mandatory `semio <envelope-id>.dsl v<version>` preamble line — is specified in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section, which is the
  carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line sample-rate-line format-line channels-line tags-line`,
  `channel = "[" sample-list? "]"`, `tag = "[" hex "," hex "]"`, and
  `format = "pcm8" | "pcm16" | "pcm24" | "pcm32" | "f32" | "f64"`;
* the ten verbs and their positional argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is this
  case's committed per-kind specification vectors under `🧫️fixtures/`.

Two leaves the grammar states by reference rather than in full, and how each was settled:

* `hex` is declared to be the framework's built-in `hex` MACRO, so every `schema`, tag `key` and tag
  `value` leaf is the lowercase hex of that string's UTF-8 bytes. Reading the committed artifact
  confirms it — `7469746c65` is `title`.
* a `sample` is described by the same grammar as "every hex-encoded `f32` sample bit-pattern token".
  The committed artifact fixes the byte order: `3f000000` is `0.5` and `bf800000` is `-1.0`, i.e.
  the IEEE-754 binary32 bits written most-significant byte first in eight digits. That reading is
  PINNED by `identity-round-trip`, which re-prints the committed file byte for byte and could not do
  so from a misreading.

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


# region 🔖️Vocabulary
#: 🏷️ Every variant of this subset's mutation vocabulary, in the order the mutations grammar's `op`
#: production lists them, kebab-cased as the catalog spells them.
KINDS = (
    "no-mutation",
    "set-snapshot",
    "set-sample-rate",
    "set-format",
    "insert-channel",
    "remove-channel",
    "set-channel-samples",
    "insert-tag",
    "remove-tag",
    "set-tag-value",
)

#: 🎚️ `format = "pcm8" | "pcm16" | "pcm24" | "pcm32" | "f32" | "f64"`, verbatim from the grammar.
FORMATS = ("pcm8", "pcm16", "pcm24", "pcm32", "f32", "f64")

DOCUMENT_SCHEMA = "stdio.semio.audio"
DSL_PREAMBLE = "semio stdio.semio.audio.dsl v1"

#: 🎤️ The document every mutation row runs on: the first real second of the real committed
#: "Bauen mit Bestand" recording — 8 000 real 16-bit PCM samples at the file's own 8 000 Hz — carrying
#: the real ID3v2.3 tags of the same recording's committed mp3. Derived ONCE by
#: `🐍️derive-audio-fixture.py` in the ticket folder.
RECORDING_DSL = "local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio"
#: 🎵️ The tiny committed tone, kept for the BYTE half of the identity law and for the tie to the
#: committed specification vectors: its file was written by the RUST codec, so this implementation
#: reproducing it is a cross-language byte agreement the recording — written by this implementation —
#: cannot restate.
TONE_DSL = "asset://📚️examples/🎵️tone/🖼️assets/🗣️.dsl.semio"

# endregion 🔖️Vocabulary


# region 🔖️Carrier
def hex_of(text: str) -> str:
    """🔡️ The grammar's `hex` macro in the writing direction: lowercase hex of the UTF-8 bytes."""
    return text.encode("utf-8").hex()


def text_of(hexed: str) -> str:
    """🔡️ The grammar's `hex` macro in the reading direction; the empty string is a legal value."""
    if len(hexed) % 2 != 0:
        raise AssertionError("hex run %r has an odd digit count" % hexed)
    return bytes.fromhex(hexed).decode("utf-8")


def sample_of(hexed: str) -> float:
    """🎚️ One `f32` sample: eight hex digits of IEEE-754 binary32, most-significant byte first."""
    if len(hexed) != 8:
        raise AssertionError("a sample is eight hex digits of binary32 bits, got %r" % hexed)
    return struct.unpack(">f", bytes.fromhex(hexed))[0]


def hex_of_sample(value: float) -> str:
    """🎚️ One `f32` sample in the writing direction — the value narrowed to binary32 first."""
    return struct.pack(">f", value).hex()


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


def field(lines: list, name: str) -> str:
    """🔎️ The right-hand side of one `name "=" value` body line; a missing line is an error."""
    prefix = name + "="
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix) :]
    raise AssertionError("the document has no %r line — the grammar declares it mandatory" % name)


# endregion 🔖️Carrier


# region 🔖️Dsl
def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line sample-rate-line format-line channels-line tags-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    schema = text_of(field(lines, "schema"))
    rate = field(lines, "sampleRate")
    if not rate.isdigit():
        raise AssertionError("sampleRate is an INT, got %r" % rate)
    fmt = field(lines, "format")
    if fmt not in FORMATS:
        raise AssertionError("unknown format %r — the grammar declares %s" % (fmt, ", ".join(FORMATS)))
    channels = [{"samples": [sample_of(one) for one in split_top_level(bracketed(each))]} for each in split_top_level(bracketed(field(lines, "channels")))]
    tags = []
    for each in split_top_level(bracketed(field(lines, "tags"))):
        key, value = split_top_level(bracketed(each))
        tags.append({"key": text_of(key), "value": text_of(value)})
    return {"schema": schema, "sampleRate": int(rate), "format": fmt, "channels": channels, "tags": tags}


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    channels = ",".join("[%s]" % ",".join(hex_of_sample(sample) for sample in channel["samples"]) for channel in snapshot["channels"])
    tags = ",".join("[%s,%s]" % (hex_of(tag["key"]), hex_of(tag["value"])) for tag in snapshot["tags"])
    body = [
        "schema=%s" % hex_of(snapshot["schema"]),
        "sampleRate=%d" % snapshot["sampleRate"],
        "format=%s" % snapshot["format"],
        "channels=[%s]" % channels,
        "tags=[%s]" % tags,
    ]
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
    """🔢️ One positional `INT` argument, checked against the collection it addresses. An
    unaddressable index is a refusal, never a silent no-op — a quietly skipped mutation would
    report as a pass."""
    limit = count if inclusive else count - 1
    if not isinstance(index, int) or isinstance(index, bool) or index < 0 or index > limit:
        raise AssertionError("%s addresses position %r of a collection holding %d entry/entries" % (verb, index, count))
    return index


def checked_format(name: str, verb: str) -> str:
    if name not in FORMATS:
        raise AssertionError("%s names format %r, which the grammar does not declare" % (verb, name))
    return name


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW snapshot."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        return clone(args["snapshot"])
    if kind == "set-sample-rate":
        rate = args["sampleRate"]
        if not isinstance(rate, int) or isinstance(rate, bool) or rate < 0:
            raise AssertionError("set-sample-rate takes a non-negative INT, got %r" % rate)
        result["sampleRate"] = rate
        return result
    if kind == "set-format":
        result["format"] = checked_format(args["format"], kind)
        return result
    if kind == "insert-channel":
        result["channels"].insert(index_at(len(result["channels"]), args["index"], kind, True), clone(args["channel"]))
        return result
    if kind == "remove-channel":
        del result["channels"][index_at(len(result["channels"]), args["index"], kind, False)]
        return result
    if kind == "set-channel-samples":
        result["channels"][index_at(len(result["channels"]), args["index"], kind, False)]["samples"] = clone(args["samples"])
        return result
    if kind == "insert-tag":
        result["tags"].insert(index_at(len(result["tags"]), args["index"], kind, True), clone(args["tag"]))
        return result
    if kind == "remove-tag":
        del result["tags"][index_at(len(result["tags"]), args["index"], kind, False)]
        return result
    result["tags"][index_at(len(result["tags"]), args["index"], kind, False)]["value"] = args["value"]
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
    if kind == "set-sample-rate":
        return {"kind": "set-sample-rate", "params": {"sampleRate": snapshot["sampleRate"]}}
    if kind == "set-format":
        return {"kind": "set-format", "params": {"format": snapshot["format"]}}
    if kind == "insert-channel":
        return {"kind": "remove-channel", "params": {"index": args["index"]}}
    if kind == "remove-channel":
        index = index_at(len(snapshot["channels"]), args["index"], kind, False)
        return {"kind": "insert-channel", "params": {"index": index, "channel": clone(snapshot["channels"][index])}}
    if kind == "set-channel-samples":
        index = index_at(len(snapshot["channels"]), args["index"], kind, False)
        return {"kind": "set-channel-samples", "params": {"index": index, "samples": clone(snapshot["channels"][index]["samples"])}}
    if kind == "insert-tag":
        return {"kind": "remove-tag", "params": {"index": args["index"]}}
    if kind == "remove-tag":
        index = index_at(len(snapshot["tags"]), args["index"], kind, False)
        return {"kind": "insert-tag", "params": {"index": index, "tag": clone(snapshot["tags"][index])}}
    index = index_at(len(snapshot["tags"]), args["index"], kind, False)
    return {"kind": "set-tag-value", "params": {"index": index, "value": snapshot["tags"][index]["value"]}}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def tone(ctx: Context) -> dict:
    """🎤️ The real recording, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(RECORDING_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.fixture_bytes("local://🦠️%s.json" % kind).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed tone by this implementation alone."""
    result = apply_mutation(tone(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the tone again — asserted here, and the MUTATED snapshot travels in the projection too, so the
    seven index-addressing verbs cannot all project the same restored value and compare vacuously."""
    snapshot = tone(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the tone\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
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
    the must-differ tripwire would be backwards. `🔊️audio` exports no pack bridge, so no claim is
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

    The committed tone's file was written by the RUST codec, so this implementation reproducing it is
    a cross-language byte agreement, not a codec agreeing with itself, and it is also the document
    every committed specification vector starts from — both ties are asserted here. The recording's
    file was written by THIS implementation from the grammar alone, so the Rust codec has to
    reproduce THAT, 8 000 real binary32 samples among them.
    """
    tone_report = carrier_once(ctx, TONE_DSL, "the committed tone")
    declared = vector(ctx, "no-mutation")["before"]
    if tone_report["document"] != declared:
        raise AssertionError("the real committed tone does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(tone_report["document"]), json.dumps(declared)))
    recording = carrier_once(ctx, RECORDING_DSL, "the recording")
    snapshot = recording["document"]
    samples = [sample for channel in snapshot["channels"] for sample in channel["samples"]]
    if snapshot["sampleRate"] != 8000 or snapshot["format"] != "pcm16" or len(snapshot["channels"]) != 1 or len(samples) != 8000 or len(snapshot["tags"]) != 4:
        raise AssertionError("the recording is the 8 000 Hz pcm16 one-channel 8 000-sample four-tag document this case describes, but decoded otherwise")
    if [tag["key"] for tag in snapshot["tags"]] != ["TSSE", "TIT2", "TPE1", "TLEN"]:
        raise AssertionError("the recording carries the mp3's own four real ID3v2.3 frames in file order, which this decoding contradicts")
    return Outcome({"tone": tone_report, "recording": recording})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector(kind))
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
