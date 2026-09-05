"""🐍️ Independent Python implementation of the `s.stdio.semio.animation` carrier and its
thirteen-verb mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio` is a semio-native carrier that no
third-party library in any ecosystem speaks, so the second producer THE STANDARD requires is a
second IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — the mandatory `semio <envelope-id>.dsl v<version>` preamble line — is specified in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line timelines-line`,
  `timeline = "[" option-name "," channel-list "]"`,
  `channel = "[" target "," interpolation "," keyframe-list "]"`,
  `target = "[" hex "," property "]"`, `property = "t" | "r" | "s" | "w" | "c" ":" hex`,
  `interpolation = "l" | "s" | "c"`, `keyframe = "[" number "," value "]"` and
  `value = "S" ":" number | "V" ":" point3 | "Q" ":" quat | "W" ":" number-list`;
* the thirteen verbs and their positional argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is this
  case's committed per-kind specification vectors under `🧫️fixtures/`.

Two leaves the grammar states by reference rather than in full, and how each was settled:

* `hex` is declared to be the framework's built-in `hex` MACRO, so every `schema`, timeline `name`,
  target `node` and custom-property `name` leaf is the lowercase hex of that string's UTF-8 bytes.
  Reading the committed artifact confirms it — `77616c6b` is `walk`, `6f706163697479` is `opacity`.
* `number = INT | FLOAT` and the grammar records that every `f64` prints through plain Rust `{v}`
  Display, which drops the fractional part of an integral value. `print_number` below reproduces
  that rule, and the reading is PINNED by `identity-round-trip`, which re-prints the committed file
  byte for byte and could not do so from a misreading — `[0,V:[0,0,0]]` and `[0.5,Q:[0,0,0,1]]` in
  the committed walk cycle exercise both halves of it.

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
    "insert-timeline",
    "remove-timeline",
    "set-timeline-name",
    "insert-channel",
    "remove-channel",
    "set-channel-target",
    "set-channel-interpolation",
    "insert-keyframe",
    "remove-keyframe",
    "set-keyframe-time",
    "set-keyframe-value",
)

#: 🎯️ `property = "t" | "r" | "s" | "w" | "c" ":" hex` — the four unit variants, in that order.
PROPERTY_LETTER = {"translation": "t", "rotation": "r", "scale": "s", "weights": "w"}
LETTER_PROPERTY = {letter: kind for kind, letter in PROPERTY_LETTER.items()}

#: 📈️ `interpolation = "l" | "s" | "c"`.
INTERPOLATION_LETTER = {"linear": "l", "step": "s", "cubicSpline": "c"}
LETTER_INTERPOLATION = {letter: kind for kind, letter in INTERPOLATION_LETTER.items()}

DOCUMENT_SCHEMA = "s.stdio.semio.animation"
DSL_PREAMBLE = "semio s.stdio.semio.animation.dsl v1"

WALK_DSL = "asset://📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio"

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


def read_number(text: str) -> float:
    """🔢️ `number = INT | FLOAT` — one `f64` leaf, integral values spelled without a point."""
    try:
        return float(text)
    except ValueError:
        raise AssertionError("expected a number, got %r" % text)


def print_number(value: float) -> str:
    """🔢️ The writing direction of `number`: Rust's `{v}` Display for `f64`, which prints an
    integral value with no fractional part and otherwise the shortest round-tripping form."""
    if value == int(value):
        return "%d" % int(value)
    return repr(float(value))


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
def read_option_name(text: str):
    """🏷️ `option-name = "[" "0" "]" | "[" "1" "," hex "]"` — the absent name is a first-class value."""
    parts = split_top_level(bracketed(text))
    if parts == ["0"]:
        return None
    if len(parts) == 2 and parts[0] == "1":
        return text_of(parts[1])
    raise AssertionError("expected an option-name group, got %r" % text)


def write_option_name(name) -> str:
    return "[0]" if name is None else "[1,%s]" % hex_of(name)


def read_property(text: str) -> dict:
    """🎯️ `property = "t" | "r" | "s" | "w" | "c" ":" hex`."""
    if text in LETTER_PROPERTY:
        return {"kind": LETTER_PROPERTY[text]}
    if text.startswith("c:"):
        return {"kind": "custom", "name": text_of(text[2:])}
    raise AssertionError("unknown property %r — the grammar declares t, r, s, w and c:<hex>" % text)


def write_property(prop: dict) -> str:
    if prop["kind"] == "custom":
        return "c:%s" % hex_of(prop["name"])
    if prop["kind"] not in PROPERTY_LETTER:
        raise AssertionError("unknown property kind %r" % prop["kind"])
    return PROPERTY_LETTER[prop["kind"]]


def read_target(text: str) -> dict:
    """🎯️ `target = "[" hex "," property "]"`."""
    node, prop = split_top_level(bracketed(text))
    return {"node": text_of(node), "property": read_property(prop)}


def write_target(target: dict) -> str:
    return "[%s,%s]" % (hex_of(target["node"]), write_property(target["property"]))


def read_value(text: str) -> dict:
    """🎞️ `value = "S" ":" number | "V" ":" point3 | "Q" ":" quat | "W" ":" number-list`."""
    tag, _, rest = text.partition(":")
    if tag == "S":
        return {"kind": "scalar", "value": read_number(rest)}
    if tag == "V":
        x, y, z = split_top_level(bracketed(rest))
        return {"kind": "vec3", "value": {"x": read_number(x), "y": read_number(y), "z": read_number(z)}}
    if tag == "Q":
        x, y, z, w = split_top_level(bracketed(rest))
        return {"kind": "quat", "value": {"x": read_number(x), "y": read_number(y), "z": read_number(z), "w": read_number(w)}}
    if tag == "W":
        return {"kind": "weights", "values": [read_number(one) for one in split_top_level(bracketed(rest))]}
    raise AssertionError("unknown value tag %r — the grammar declares S, V, Q and W" % tag)


def write_value(value: dict) -> str:
    if value["kind"] == "scalar":
        return "S:%s" % print_number(value["value"])
    if value["kind"] == "vec3":
        point = value["value"]
        return "V:[%s,%s,%s]" % (print_number(point["x"]), print_number(point["y"]), print_number(point["z"]))
    if value["kind"] == "quat":
        quat = value["value"]
        return "Q:[%s,%s,%s,%s]" % (print_number(quat["x"]), print_number(quat["y"]), print_number(quat["z"]), print_number(quat["w"]))
    if value["kind"] == "weights":
        return "W:[%s]" % ",".join(print_number(one) for one in value["values"])
    raise AssertionError("unknown value kind %r" % value["kind"])


def read_keyframe(text: str) -> dict:
    """🎞️ `keyframe = "[" number "," value "]"`."""
    at, value = split_top_level(bracketed(text))
    return {"t": read_number(at), "value": read_value(value)}


def write_keyframe(keyframe: dict) -> str:
    return "[%s,%s]" % (print_number(keyframe["t"]), write_value(keyframe["value"]))


def read_channel(text: str) -> dict:
    """📈️ `channel = "[" target "," interpolation "," keyframe-list "]"`."""
    target, interpolation, keyframes = split_top_level(bracketed(text))
    if interpolation not in LETTER_INTERPOLATION:
        raise AssertionError("unknown interpolation %r — the grammar declares l, s and c" % interpolation)
    return {
        "target": read_target(target),
        "interpolation": LETTER_INTERPOLATION[interpolation],
        "keyframes": [read_keyframe(each) for each in split_top_level(bracketed(keyframes))],
    }


def write_channel(channel: dict) -> str:
    if channel["interpolation"] not in INTERPOLATION_LETTER:
        raise AssertionError("unknown interpolation %r" % channel["interpolation"])
    return "[%s,%s,[%s]]" % (
        write_target(channel["target"]),
        INTERPOLATION_LETTER[channel["interpolation"]],
        ",".join(write_keyframe(keyframe) for keyframe in channel["keyframes"]),
    )


def read_timeline(text: str) -> dict:
    """🎬️ `timeline = "[" option-name "," channel-list "]"`."""
    name, channels = split_top_level(bracketed(text))
    return {"name": read_option_name(name), "channels": [read_channel(each) for each in split_top_level(bracketed(channels))]}


def write_timeline(timeline: dict) -> str:
    return "[%s,[%s]]" % (write_option_name(timeline["name"]), ",".join(write_channel(channel) for channel in timeline["channels"]))


def parse_dsl(text: str) -> dict:
    """📖️ `document = artifact-mark schema-line timelines-line`."""
    lines = [line.strip() for line in split_preamble(text).splitlines()]
    lines = [line for line in lines if line != ""]
    return {
        "schema": text_of(field(lines, "schema")),
        "timelines": [read_timeline(each) for each in split_top_level(bracketed(field(lines, "timelines")))],
    }


def print_dsl(snapshot: dict) -> str:
    """✍️ The same grammar in the writing direction, no trailing newline — the shape of the
    committed artifact, which `identity-round-trip` reproduces byte for byte."""
    body = ["schema=%s" % hex_of(snapshot["schema"]), "timelines=[%s]" % ",".join(write_timeline(timeline) for timeline in snapshot["timelines"])]
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
    """🔢️ One positional `index` argument, checked against the collection it addresses. An
    unaddressable index is a refusal, never a silent no-op."""
    limit = count if inclusive else count - 1
    if not isinstance(index, int) or isinstance(index, bool) or index < 0 or index > limit:
        raise AssertionError("%s addresses position %r of a collection holding %d entry/entries" % (verb, index, count))
    return index


def timeline_at(snapshot: dict, args: dict, verb: str) -> dict:
    return snapshot["timelines"][index_at(len(snapshot["timelines"]), args["timelineIndex"], verb, False)]


def channel_at(snapshot: dict, args: dict, verb: str) -> dict:
    timeline = timeline_at(snapshot, args, verb)
    return timeline["channels"][index_at(len(timeline["channels"]), args["channelIndex"], verb, False)]


def apply_mutation(snapshot: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW snapshot."""
    result = clone(snapshot)
    kind, args = parts(mutation)
    if kind == "no-mutation":
        return result
    if kind == "set-snapshot":
        return clone(args["snapshot"])
    if kind == "insert-timeline":
        result["timelines"].insert(index_at(len(result["timelines"]), args["index"], kind, True), clone(args["timeline"]))
        return result
    if kind == "remove-timeline":
        del result["timelines"][index_at(len(result["timelines"]), args["index"], kind, False)]
        return result
    if kind == "set-timeline-name":
        result["timelines"][index_at(len(result["timelines"]), args["index"], kind, False)]["name"] = args["name"]
        return result
    if kind in ("insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation"):
        timeline = timeline_at(result, args, kind)
        if kind == "insert-channel":
            timeline["channels"].insert(index_at(len(timeline["channels"]), args["index"], kind, True), clone(args["channel"]))
            return result
        index = index_at(len(timeline["channels"]), args["index"], kind, False)
        if kind == "remove-channel":
            del timeline["channels"][index]
            return result
        if kind == "set-channel-target":
            timeline["channels"][index]["target"] = clone(args["target"])
            return result
        if args["interpolation"] not in INTERPOLATION_LETTER:
            raise AssertionError("set-channel-interpolation names %r, which the grammar does not declare" % args["interpolation"])
        timeline["channels"][index]["interpolation"] = args["interpolation"]
        return result
    channel = channel_at(result, args, kind)
    if kind == "insert-keyframe":
        channel["keyframes"].insert(index_at(len(channel["keyframes"]), args["index"], kind, True), clone(args["keyframe"]))
        return result
    index = index_at(len(channel["keyframes"]), args["index"], kind, False)
    if kind == "remove-keyframe":
        del channel["keyframes"][index]
        return result
    if kind == "set-keyframe-time":
        channel["keyframes"][index]["t"] = args["t"]
        return result
    channel["keyframes"][index]["value"] = clone(args["value"])
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
    if kind == "insert-timeline":
        return {"kind": "remove-timeline", "params": {"index": args["index"]}}
    if kind == "remove-timeline":
        index = index_at(len(snapshot["timelines"]), args["index"], kind, False)
        return {"kind": "insert-timeline", "params": {"index": index, "timeline": clone(snapshot["timelines"][index])}}
    if kind == "set-timeline-name":
        index = index_at(len(snapshot["timelines"]), args["index"], kind, False)
        return {"kind": "set-timeline-name", "params": {"index": index, "name": snapshot["timelines"][index]["name"]}}
    if kind in ("insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation"):
        timeline = timeline_at(snapshot, args, kind)
        if kind == "insert-channel":
            return {"kind": "remove-channel", "params": {"timelineIndex": args["timelineIndex"], "index": args["index"]}}
        index = index_at(len(timeline["channels"]), args["index"], kind, False)
        was = timeline["channels"][index]
        if kind == "remove-channel":
            return {"kind": "insert-channel", "params": {"timelineIndex": args["timelineIndex"], "index": index, "channel": clone(was)}}
        if kind == "set-channel-target":
            return {"kind": "set-channel-target", "params": {"timelineIndex": args["timelineIndex"], "index": index, "target": clone(was["target"])}}
        return {"kind": "set-channel-interpolation", "params": {"timelineIndex": args["timelineIndex"], "index": index, "interpolation": was["interpolation"]}}
    channel = channel_at(snapshot, args, kind)
    if kind == "insert-keyframe":
        return {"kind": "remove-keyframe", "params": {"timelineIndex": args["timelineIndex"], "channelIndex": args["channelIndex"], "index": args["index"]}}
    index = index_at(len(channel["keyframes"]), args["index"], kind, False)
    was = channel["keyframes"][index]
    common = {"timelineIndex": args["timelineIndex"], "channelIndex": args["channelIndex"], "index": index}
    if kind == "remove-keyframe":
        return {"kind": "insert-keyframe", "params": dict(common, keyframe=clone(was))}
    if kind == "set-keyframe-time":
        return {"kind": "set-keyframe-time", "params": dict(common, t=was["t"])}
    return {"kind": "set-keyframe-value", "params": dict(common, value=clone(was["value"]))}


# endregion 🔖️Mutations


# region 🔖️Scenario input
def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own parameters — the feature owns them, not the adapter, so the two
    implementations cannot read two different transcriptions of the same verb."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def walk(ctx: Context) -> dict:
    """🚶️ The real committed walk cycle, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.fixture_bytes(WALK_DSL).decode("utf-8"))


def vector(ctx: Context, kind: str) -> dict:
    """🧫️ One committed `(before, mutation, after)` specification vector."""
    return json.loads(ctx.fixture_bytes("local://🦠️%s.json" % kind).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed walk cycle by this implementation alone."""
    result = apply_mutation(walk(ctx), json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored snapshot must be
    the walk cycle again — asserted here, and the MUTATED snapshot travels in the projection too, so
    the thirteen rows cannot all project the same restored value and compare vacuously."""
    snapshot = walk(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(snapshot, mutation)
    restored = apply_mutation(mutated, inverse_mutation(snapshot, mutation))
    if restored != snapshot:
        raise AssertionError("undoing %s did not restore the walk cycle\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(snapshot)))
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


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ The committed walk cycle, re-emitted from the parsed snapshot.

    `.dsl.semio` is a fixed-layout record grammar, so an exact re-emission is the CORRECT answer
    here and the wave's must-differ tripwire would be backwards. What keeps that from being vacuous
    is that the bytes were written by the OTHER implementation: this file reproducing them is a
    cross-language byte agreement, not a codec agreeing with itself. `🎞️animation` exports no pack
    bridge, so the committed `🎒️.pack.semio` twin is deliberately not read and no claim is
    made about it — one carrier measured, the other named as unmeasured.
    """
    committed = ctx.fixture_bytes(WALK_DSL)
    snapshot = parse_dsl(committed.decode("utf-8"))
    printed = print_dsl(snapshot).encode("utf-8")
    if printed != committed:
        raise AssertionError("re-printing the walk cycle did not reproduce the committed DSL bytes (%d vs %d bytes)\n     got: %s\nexpected: %s" % (len(printed), len(committed), printed.decode("utf-8"), committed.decode("utf-8")))
    if parse_dsl(printed.decode("utf-8")) != snapshot:
        raise AssertionError("re-parsing the printed walk cycle lost content")
    if snapshot["schema"] != DOCUMENT_SCHEMA:
        raise AssertionError("the committed walk cycle declares schema %r, expected %r" % (snapshot["schema"], DOCUMENT_SCHEMA))
    declared = vector(ctx, "no-mutation")["before"]
    if snapshot != declared:
        raise AssertionError("the real committed walk cycle does not decode to the before-snapshot every specification vector starts from\n     got: %s\nexpected: %s" % (json.dumps(snapshot), json.dumps(declared)))
    return Outcome({"document": snapshot, "dslDigest": digest(printed), "dslLength": len(printed)})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls — by FULL expanded scenario id, one per row."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse).oracle("spec-vector-%s" % kind, spec_vector(kind))
    return built.oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
