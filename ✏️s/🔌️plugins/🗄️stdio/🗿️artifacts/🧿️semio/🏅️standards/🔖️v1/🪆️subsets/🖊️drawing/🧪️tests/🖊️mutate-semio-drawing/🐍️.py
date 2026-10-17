"""🐍️ Independent Python implementation of the `stdio.semio.drawing` carrier and its seventeen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, and the earlier survey of the vector-graphics libraries still
stands on its merits: `usvg`/`resvg` model SVG, which has no counterpart for this subset's anonymous
recursive `DrawNode` tree addressed by a structural `NodePath`, nor for its four hierarchy verbs;
`lyon`/`kurbo` model path geometry alone and could adjudicate at most `replace-path`. The second
producer THE STANDARD requires is therefore a second IMPLEMENTATION, written in another language
from the format's own committed specification:

* the envelope — `semio <schema>.dsl v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope section;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  — `document = artifact-mark schema-line canvas-line styles-line layers-line`, the four
  single-letter `node` tags with `G`'s genuinely recursive `children`, the six `segment` tags, and
  the three `option-` productions;
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio` and its
  Kaitai mirror, which declare `format` and the varint-length-prefixed `schema` and then stop at one
  opaque `payload` chain by their own admission. That layer was DERIVED from the committed
  `✉️base/📚️examples/🖍️sketch` bytes against their readable DSL twin — which between them exhibit
  every node tag and every segment tag, including the arc — and the derivation is pinned by
  re-encoding that committed file byte for byte, which a misreading could not do;
* the seventeen verbs, their argument lists and their JSON wire form are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, the committed proto mirror and the
  committed per-kind `(before, mutation, after)` specification vectors, which pin each verb's
  semantics — including what the four hierarchy verbs do to a nested tree.

Nothing here imports, links, wraps or transliterates the Rust subject. Where the two disagree the
disagreement is a finding, not something to tune away.
"""

from __future__ import annotations

# region 🔖️Imports
import importlib.util
import json
import os
import sys

from semio_repo_test import Adapter, Context, Outcome, digest


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


_load("mutate_semio_drawing_text", "📝️text-carrier")
_load("mutate_semio_drawing_pack", "📦️pack-carrier")
_load("mutate_semio_drawing_verbs", "🧬️mutation-verbs")

from mutate_semio_drawing_text import ARTIFACT_DSL, ARTIFACT_PACK, print_f32, parse_dsl, print_dsl
from mutate_semio_drawing_pack import parse_pack, pack_bytes
from mutate_semio_drawing_verbs import clone, apply_mutation, inverse_mutation
# endregion 🔖️Imports


# region 🔖️Scenario input
def step_uris(ctx: Context, prefix: str) -> list:
    """🧫️ The fixture URIs under one prefix the scenario's steps name, in step order (the host's one fixture-URI grammar)."""
    return [uri for uri in ctx.step_input_uris() if uri.startswith(prefix)]


def snapshot_json(ctx: Context, uri: str):
    """🧫️ A declared fixture read as JSON."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def artifact(ctx: Context) -> dict:
    """🖍️ The real derived drawing, parsed through this implementation's own DSL reader."""
    return parse_dsl(ctx.input_bytes(ARTIFACT_DSL).decode("utf-8"))


def projection_of(document: dict) -> dict:
    """🎯️ The projection every scenario compares under `ordered-json-v1` — the snapshot's own
    structural JSON shape.

    `SemioRgba`'s four channels and a style's `opacity` are SINGLE precision, and the reference's JSON
    wire form spells such a leaf with the shortest decimal that round-trips as an `f32` (`0.35`), not
    with the widened double a Python float would print (`0.3499999940395355`). Every single-precision
    leaf therefore goes out through the same shortest-`f32` printer the DSL writer uses and back, so
    the two languages compare the same number rather than the same bit pattern printed two ways.
    """
    result = clone(document)
    narrow = lambda colour: {key: float(print_f32(colour[key])) for key in ("r", "g", "b", "a")}
    if "background" in result["canvas"]:
        result["canvas"]["background"] = narrow(result["canvas"]["background"])
    for style in result["styles"]:
        for key in ("fill", "stroke"):
            if key in style:
                style[key] = narrow(style[key])
        if "opacity" in style:
            style["opacity"] = float(print_f32(style["opacity"]))
    return result


def shape_report(document: dict) -> dict:
    """🌳️ A structural census of the scene graph, so a mutation that lands in the wrong branch shows
    up as a shape difference and not only as a deep value difference: per layer, the node kind
    histogram, the maximum depth and the total segment count."""
    census = []
    for layer in document["layers"]:
        counts = {"path": 0, "text": 0, "group": 0, "image": 0}
        segments = [0]
        depth = [0]

        def walk(node: dict, level: int) -> None:
            counts[node["kind"]] += 1
            depth[0] = max(depth[0], level)
            if node["kind"] == "path":
                segments[0] += len(node["segments"])
            for child in node.get("children", []):
                walk(child, level + 1)

        walk(layer["root"], 0)
        census.append({"id": layer["id"], "visible": layer["visible"], "nodes": counts, "depth": depth[0], "segments": segments[0]})
    return {"layers": census, "styles": [style["name"] for style in document["styles"]]}


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real derived drawing by this implementation alone."""
    document = artifact(ctx)
    mutation = snapshot_json(ctx, next(uri for uri in step_uris(ctx, "shared://🖊️mutate-semio-drawing/") if uri.endswith("/🦠️mutation/🔣️.json")))
    applied = apply_mutation(document, mutation)
    return Outcome({"document": projection_of(applied), "shape": shape_report(applied)})


def inverse(ctx: Context) -> Outcome:
    """↩️ The metamorphic inverse law on the real drawing: the verb followed by its OWN computed
    inverse must restore it exactly, scene-graph ORDER and nesting included."""
    document = artifact(ctx)
    mutation = snapshot_json(ctx, next(uri for uri in step_uris(ctx, "shared://🖊️mutate-semio-drawing/") if uri.endswith("/🦠️mutation/🔣️.json")))
    undo = inverse_mutation(document, mutation)
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in undo:
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("%s: undoing the mutation did not restore the drawing" % ctx.scenario["id"])
    return Outcome({"mutated": projection_of(mutated), "restored": projection_of(restored)})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    statement of what the verb means, independent of both implementations."""
    uris = ctx.step_input_uris()
    before = snapshot_json(ctx, uris[0])
    mutation = snapshot_json(ctx, uris[1])
    expected = snapshot_json(ctx, uris[2])
    applied = apply_mutation(before, mutation)
    if applied != expected:
        raise AssertionError("%s: the applied drawing is not the committed after-snapshot" % ctx.scenario["id"])
    return Outcome(projection_of(applied))


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the real derived drawing, each re-emitted from the parsed
    document.

    🔒️ The BYTE half of the identity law. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` its binary twin, so reproducing both files byte for byte is the CORRECT answer here
    and a must-differ tripwire would be exactly backwards. What stops that from being a codec
    agreeing with itself is that the Rust subject reproduces the same two files from its own reading
    of the same grammar, and the digests of what each side emitted are what the runner compares.
    """
    dsl_bytes = ctx.input_bytes(ARTIFACT_DSL)
    parsed = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(parsed).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("identity-round-trip: re-printing the parsed drawing did not reproduce the committed DSL file")
    pack = ctx.input_bytes(ARTIFACT_PACK)
    unpacked = parse_pack(pack)
    if unpacked != parsed:
        raise AssertionError("identity-round-trip: the committed binary twin decodes to a different drawing than the committed text artifact")
    repacked = pack_bytes(parsed)
    if repacked != pack:
        raise AssertionError("identity-round-trip: re-encoding the parsed drawing did not reproduce the committed pack file")
    return Outcome(
        {
            "document": projection_of(parsed),
            "shape": shape_report(parsed),
            "dslDigest": digest(printed),
            "packDigest": digest(repacked),
            "dslLength": len(printed),
            "packLength": len(repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the Python host calls. Handlers are registered under the Scenario
    Outline base ids, which the host resolves for every Examples row, and plain scenarios under their
    own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
