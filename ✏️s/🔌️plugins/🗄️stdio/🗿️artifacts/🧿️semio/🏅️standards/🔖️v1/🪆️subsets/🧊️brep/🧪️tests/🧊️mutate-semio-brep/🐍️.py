"""🐍️ Independent Python implementation of the `s.stdio.semio.brep` carrier and its thirteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line vertices-line edges-line loops-line faces-line shells-line
  solids-line coedges-line next-label-line`, every vertex/edge/face closing on its `tol` number, the
  tagged `curve = L|C|E|N`, `curve2 = L|C|E|N` (a coedge's `~`-marked p-curve, or `-`) and
  `surface = P|C|O|S|T|N` value productions with their exact field lists, and `bool = "0" | "1"`);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`, which names every
  member of every curve and surface arm (`origin`/`direction`, `center`/`axis`/`radius`,
  `radiusMajor`/`radiusMinor`, `controlPoints`/`weights`/`degree`/`knots`, `normal`, `halfAngle`,
  `majorRadius`/`minorRadius`, `uCount`/`vCount`/`degreeU`/`degreeV`/`knotsU`/`knotsV`) and the
  topology records' `startVertex`/`endVertex`/`outerLoop`/`innerLoops`/`isVoid`;
* the thirteen verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body's `format u8` + varint-length-prefixed `schema` is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio`, whose prose then names — but declines to
  frame — "vertices/edges/loops/faces/shells/solids … varint counts, per-field length-prefixes,
  real `f64` LE coordinates, a real per-variant tag byte for `curve`/`surface`". That
  named-but-unframed layout was written out here from the protocol's own sentence, with the field
  order and the tag ordinals read off the grammar's own `L|C|E|N` and `P|C|O|S|T|N` orders, and is
  PINNED by `pack_bytes` re-encoding the committed `🎒️.pack.semio` byte for byte — a file
  that carries a line, a circle, a NURBS curve and a NURBS surface, so four of the ten tagged arms
  are pinned directly and the remaining six are the same field lists the grammar spells out.

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


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


_load("mutate_semio_brep_text", "📝️text-carrier")
_load("mutate_semio_brep_pack", "📦️pack-carrier")
_load("mutate_semio_brep_verbs", "🧬️mutation-verbs")

from mutate_semio_brep_text import FOREST_DSL, FOREST_PACK, SOLID_DSL, SOLID_PACK, parse_dsl, print_dsl
from mutate_semio_brep_pack import parse_pack, pack_bytes
from mutate_semio_brep_verbs import apply_mutation, inverse_mutation, apply_all
# endregion 🔖️Imports


# region 🔖️Scenario input
def doc_json(ctx: Context) -> dict:
    """📜️ The scenario's own committed vector — the feature owns the parameters, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def prepared(ctx: Context) -> tuple:
    """🧊️ The real committed solid, put into the state the scenario's verb is aimed at by the doc
    string's own `prepare` list, plus the verb itself."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.input_bytes(FOREST_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def snapshot_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed solid by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored solid must be the
    prepared solid again — asserted here, and compared against the subject's restored solid by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the solid\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector. The vector
    is a THIRD statement of what the verb means, independent of both implementations."""
    before_uri, mutation_uri, after_uri = ctx.step_input_uris()[:3]
    before = snapshot_json(ctx, before_uri)
    after = snapshot_json(ctx, after_uri)
    applied = apply_mutation(before, snapshot_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied snapshot does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    return Outcome(applied)


def carrier_pair(ctx: Context, dsl_uri: str, pack_uri: str, what: str) -> dict:
    """🔁️ One document's two committed encodings, each re-emitted from the parsed document and
    required to come back byte for byte. `.dsl.semio` is a fixed-layout record grammar and
    `.pack.semio` is its binary twin, so an exact re-emission is the CORRECT answer and the wave's
    must-differ tripwire would be backwards here."""
    dsl_bytes = ctx.input_bytes(dsl_uri)
    document = parse_dsl(dsl_bytes.decode("utf-8"))
    printed = print_dsl(document).encode("utf-8")
    if printed != dsl_bytes:
        raise AssertionError("re-printing %s did not reproduce its committed DSL bytes (%d vs %d bytes)" % (what, len(printed), len(dsl_bytes)))
    if parse_dsl(printed.decode("utf-8")) != document:
        raise AssertionError("re-parsing the printed %s lost content" % what)
    committed_pack = ctx.input_bytes(pack_uri)
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

    The committed `✉️base` solid's two files were written by the RUST codec, so this implementation
    reproducing them is a cross-language byte agreement, not a codec agreeing with itself. The
    concrete forest's two files were written by THIS implementation from the grammar and the
    protocol, so the Rust codec has to reproduce THOSE — 2 466 real `f64` among them, 98 of which
    have no exponent-free shortest lexeme and are written positionally.
    """
    solid = carrier_pair(ctx, SOLID_DSL, SOLID_PACK, "the committed solid")
    kinds = {edge["curve"]["kind"] for edge in solid["document"]["edges"]}
    if not {"line", "circle", "nurbs"} <= kinds or solid["document"]["faces"][0]["surface"]["kind"] != "nurbs":
        raise AssertionError("the committed solid is the line/circle/NURBS artifact this case describes, but decoded with curve kinds %r" % sorted(kinds))
    forest = carrier_pair(ctx, FOREST_DSL, FOREST_PACK, "the concrete forest")
    shape = forest["document"]
    counts = (len(shape["vertices"]), len(shape["edges"]), len(shape["loops"]), len(shape["faces"]), len(shape["shells"]), len(shape["solids"]))
    if counts != (167, 270, 127, 127, 12, 12):
        raise AssertionError("the concrete forest is the 167/270/127/127/12/12 structure this case describes, but decoded as %r" % (counts,))
    if {edge["curve"]["kind"] for edge in shape["edges"]} != {"nurbs"} or {face["surface"]["kind"] for face in shape["faces"]} != {"plane"}:
        raise AssertionError("the concrete forest carries B-spline edges on planar faces throughout, which this decoding contradicts")
    return Outcome({"solid": solid, "forest": forest})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
