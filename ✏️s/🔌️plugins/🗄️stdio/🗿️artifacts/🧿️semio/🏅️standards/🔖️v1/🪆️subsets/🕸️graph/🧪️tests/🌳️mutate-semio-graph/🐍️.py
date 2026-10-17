"""🐍️ Independent Python implementation of the `s.stdio.semio.graph` carrier and its eleven-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library speaks, so the second producer THE STANDARD requires is a second
IMPLEMENTATION, written in another language from the format's own committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s `🔖️Envelope`/
  `🔖️Binary`/`🔖️Text` regions, the carrier's normative description;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line nodes-line edges-line`, `node = "[" hex "," hex "," hex ","
  hex "," hex "," hex "," hex "," "[" port-list? "]" "," "[" property-list? "]" "]"` — id, kind, label,
  x, y, width, height —, `port = "[" hex "," port-kind "," hex "," "[" property-list? "]" "]"`,
  `edge = "[" hex "," hex "," hex "," hex "," hex "," optional-text "," optional-text "," "["
  property-list? "]" "]"`, `port-kind = "i" | "o" | "x"`, and the restated `SemioValue` production
  `Z|B|I|F|S|Y|L|M|R`);
* the JSON projection is the committed schema `…/📸️snapshot/🔣️.json`, whose `ports.kind`
  enum is `in|out|inOut` and whose geometry (`position.x`/`position.y`/`width`/`height`) is the
  declared binary64 WORD `{"bits": "<16 lowercase hex digits>"}` — the IEEE 754 bit pattern, so no
  float ever round-trips through a decimal lexeme in the projection — plus `🔢️value`'s own
  `…/🔢️value/🧬️schema/📸️snapshot/🔣️.json` for the `SemioValue` member names (`lexeme`, `value`,
  `items`, `entries`, `id`);
* the eleven verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`, and their JSON wire form is the
  committed per-kind specification vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/`;
* the pack body's `format u8` + varint-length-prefixed `schema` is the committed protocol
  `…/📸️snapshot/💾️binary/📡️.protocol.semio`, whose prose then names — but declines to
  frame — "per-node varint-length-prefixed id/kind/label strings plus a fixed 16-byte position,
  nested ports/properties lists; per-edge four length-prefixed strings". That named-but-unframed
  layout was written out here from the protocol's own sentence, with the port-kind and value-tag
  ordinals read off the grammar's own `i|o|x` and `Z|B|I|F|S|Y|L|M|R` orders, and is PINNED by
  `pack_bytes` re-encoding the committed `🎒️.pack.semio` byte for byte, which a misreading
  could not do. The committed artifact carries `int` and `str` values only, so the `bytes`/`list`/
  `map`/`ref` arms of the BINARY value codec below are the natural extension of the four that are
  pinned rather than themselves pinned — no scenario in this case exercises them.

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


_load("mutate_semio_graph_text", "📝️text-carrier")
_load("mutate_semio_graph_pack", "📦️pack-carrier")
_load("mutate_semio_graph_verbs", "🧬️mutation-verbs")

from mutate_semio_graph_text import TOWER_DSL, TOWER_PACK, WIRES_DSL, WIRES_PACK, parse_dsl, print_dsl
from mutate_semio_graph_pack import parse_pack, pack_bytes
from mutate_semio_graph_verbs import apply_mutation, inverse_mutation, apply_all
# endregion 🔖️Imports


# region 🔖️Scenario input
def doc_json(ctx: Context) -> dict:
    """📜️ The scenario's own committed vector — the feature owns the parameters, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def prepared(ctx: Context) -> tuple:
    """🕸️ The real committed wires graph, plus the scenario's own verb."""
    plan = doc_json(ctx)
    document = apply_all(parse_dsl(ctx.input_bytes(TOWER_DSL).decode("utf-8")), plan.get("prepare", []))
    return document, plan["mutation"]


def snapshot_json(ctx: Context, uri: str) -> dict:
    """🧫️ One committed specification-vector file, decoded from the bytes the plan pinned."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real committed wires graph by this implementation alone."""
    document, mutation = prepared(ctx)
    result = apply_mutation(document, mutation)
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored graph must be the
    wires graph again — asserted here, and compared against the subject's restored graph by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document, mutation = prepared(ctx)
    mutated = apply_mutation(document, mutation)
    restored = apply_all(mutated, inverse_mutation(document, mutation))
    if restored != document:
        raise AssertionError("undoing %s did not restore the wires graph\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(restored), json.dumps(document)))
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
    """🔁️ One document's two encodings, each re-emitted from the parsed document and required back
    byte for byte. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    twin, so an exact re-emission is the CORRECT answer and the must-differ tripwire would be
    backwards here."""
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
        raise AssertionError("the binary twin of %s decodes to a different graph than its text\n     got: %s\nexpected: %s" % (what, json.dumps(unpacked), json.dumps(document)))
    repacked = pack_bytes(document)
    if repacked != committed_pack:
        raise AssertionError("re-encoding %s did not reproduce its committed pack bytes (%d vs %d bytes)" % (what, len(repacked), len(committed_pack)))
    if parse_pack(repacked) != document:
        raise AssertionError("re-decoding the encoded pack of %s lost content" % what)
    return {"document": document, "dslDigest": digest(printed), "packDigest": digest(repacked), "dslLength": len(printed), "packLength": len(repacked)}


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both graphs, in both encodings — four files, all four reproduced byte for byte.

    All four files were written by THIS implementation from the grammar and the protocol, so the
    Rust codec has to reproduce THEM — 364 ports and 366 typed properties among the tower's — which
    is a cross-language byte agreement, not a codec agreeing with itself.
    """
    wires = carrier_pair(ctx, WIRES_DSL, WIRES_PACK, "the committed wires graph")
    if len(wires["document"]["nodes"]) != 2 or len(wires["document"]["edges"]) != 1:
        raise AssertionError("the committed wires graph is the two-node one-edge artifact this case describes, but decoded as %d node(s) and %d edge(s)" % (len(wires["document"]["nodes"]), len(wires["document"]["edges"])))
    tower = carrier_pair(ctx, TOWER_DSL, TOWER_PACK, "the capsule tower graph")
    nodes, edges = tower["document"]["nodes"], tower["document"]["edges"]
    ports = [port for node in nodes for port in node["ports"]]
    properties = [entry for node in nodes for entry in node["properties"]]
    if (len(nodes), len(edges), len(ports), len(properties)) != (181, 179, 364, 366):
        raise AssertionError("the capsule tower graph is the 181/179/364/366 document this case describes, but decoded as %d/%d/%d/%d" % (len(nodes), len(edges), len(ports), len(properties)))
    if {port["kind"] for port in ports} != {"in", "out", "inOut"}:
        raise AssertionError("the capsule tower graph carries all three port directions, which this decoding contradicts")
    return Outcome({"wires": wires, "tower": tower})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
