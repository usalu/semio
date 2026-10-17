"""🐍️ Independent Python implementation of the `stdio.semio.model` carrier and its fourteen-verb
mutation vocabulary — the differential ORACLE this case is measured against.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. `.dsl.semio`/`.pack.semio` is a semio-native carrier
that no third-party library in any ecosystem speaks — IfcOpenShell and ruststep read IFC and STEP,
not a semio envelope, and neither can express `set-snapshot` at all — so the second producer THE
STANDARD requires is a second IMPLEMENTATION, written in another language from the format's own
committed specification:

* the envelope — `semio <plugin>.<artifact>.<component> v<version>` preamble for text, and the
  `0x89 'S' 'E' 'M' 0D 0A 1A 0A` magic + little-endian u32 token length + token for binary — is
  specified in `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`'s envelope region;
* the DSL body is the committed grammar
  `../../🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`
  (`document = artifact-mark schema-line spatial-line elements-line relations-line`, and the
  `spatial-kind`, `element-class`, `geometry-ref`, `pset-value`, `relation-kind`, `opt-str` and
  `transform` productions it declares, in the order it declares them);
* the pack body is the committed protocol `…/📸️snapshot/💾️binary/📡️.protocol.semio`
  (`format u8`, then the varint-length-prefixed UTF-8 `schema`), whose description then stops at the
  repeated `spatial`/`elements`/`relations` records by its own admission and names only their shape —
  "varint counts, per-field length-prefixes, real `f64` LE coordinates, u8 enum tags". That prose was
  turned into the reader and writer below by taking the field ORDER from the DSL grammar and every
  enum ORDINAL from the order that same grammar declares its alternatives in — `S|B|T|P`,
  `WA|SL|CO|BE|DO|WI|RO|ST|FU|OT`, `N|B|M`, `T|N|B`, `AG|CI|CN|FV|VE|OT` — the same rule that turned
  out to be right for `📊️table`'s `Z B I F S Y L M R` value tags. The committed
  `🎒️.pack.semio` pins the ordinals it happens to carry (`S`, `T`, `WA`, `B`-brep, `T`/`N`/`B`
  pset values, `CI`) and `pack_bytes` re-encodes that file byte for byte; the ordinals it does NOT
  carry — `OT`, the `M` geometry reference, an absent `spatialId` — are derived from the grammar's
  declared order alone, are exercised by the real artifact this case mutates, and a disagreement
  about them would surface as a red `identity-round-trip` rather than as a silent one;
* the eleven verbs and their argument lists are the committed grammar
  `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and the committed JSON schema, and what
  each verb MEANS is the committed `(before, mutation, after)` specification vector per kind in this
  case's own `🧫️fixtures/` — including the facts a name alone does not settle: `insert-*` carries no
  index and APPENDS, `remove-*` does NOT cascade into the collections that reference the removed
  member, and every `set-*` slot is tri-state, where a `null` or absent key means "leave untouched".

Nothing here imports, links, wraps or transliterates the Rust subject. Every function was written
against the documents above; where the two implementations disagree the disagreement is a finding,
not something to tune away.

🧫️ **Provenance of the complex artifact.** `shared://🏛️mutate-semio-model/📝️nakagin-capsule-tower.dsl.semio` and its binary
twin were derived ONCE, by `🐍️derive-model-fixture.py` in this ticket's folder, from the real
committed IFC 4 model `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`
— Kisho Kurokawa's Nakagin Capsule Tower, 2.5 MB and 24 792 entities — read with **IfcOpenShell
0.8.4**, a genuine third-party IFC implementation. Its `IfcSite`/`IfcBuilding`/`IfcBuildingStorey`
became the spatial nodes, its `IfcElementAssembly` and 180 `IfcBuildingElementProxy` capsules the
elements, their `IfcPropertySingleValue` properties the property sets, their `IfcLocalPlacement`
axes the placements (real translations and real orientation quaternions), and its `IfcRelAggregates`,
`IfcRelContainedInSpatialStructure` and `IfcRelConnectsElements` the relations.
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


_load("mutate_semio_model_text", "📝️text-carrier")
_load("mutate_semio_model_pack", "📦️pack-carrier")
_load("mutate_semio_model_verbs", "🧬️mutation-verbs")

from mutate_semio_model_text import parse_dsl, print_dsl
from mutate_semio_model_pack import parse_pack, pack_bytes
from mutate_semio_model_verbs import apply_mutation, inverse_mutation
# endregion 🔖️Imports


# region 🔖️Scenario input
BUILDING_DSL = "asset://🏢️building/🗣️.dsl.semio"
BUILDING_PACK = "asset://🏢️building/🎒️.pack.semio"
TOWER_DSL = "shared://🏛️mutate-semio-model/📝️nakagin-capsule-tower.dsl.semio"
TOWER_PACK = "shared://🏛️mutate-semio-model/📦️nakagin-capsule-tower.pack.semio"


def doc_string(ctx: Context) -> str:
    """📜️ The scenario's own committed parameters — the feature owns them, not the adapter."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def tower(ctx: Context) -> dict:
    """🏗️ The real 181-element capsule tower, read through this implementation's own DSL parser."""
    return parse_dsl(ctx.input_bytes(TOWER_DSL).decode("utf-8"))


def snapshot_json(ctx: Context, uri: str) -> dict:
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


# endregion 🔖️Scenario input


# region 🔖️Handlers
def mutate(ctx: Context) -> Outcome:
    """🎯️ One verb applied to the real capsule tower model by this implementation alone."""
    document = tower(ctx)
    result = apply_mutation(document, json.loads(doc_string(ctx)))
    return Outcome(result, raw=print_dsl(result).encode("utf-8"))


def inverse(ctx: Context) -> Outcome:
    """↩️ The same verb, then this implementation's own undo of it. The restored model must be the
    capsule tower again — asserted here, and compared against the subject's restored model by the
    runner, so a wrong undo that happens to be self-consistent still shows up."""
    document = tower(ctx)
    mutation = json.loads(doc_string(ctx))
    mutated = apply_mutation(document, mutation)
    restored = mutated
    for step in inverse_mutation(document, mutation):
        restored = apply_mutation(restored, step)
    if restored != document:
        raise AssertionError("undoing %s did not restore the capsule tower model" % ctx.scenario["id"])
    return Outcome({"mutated": mutated, "restored": restored})


def spec_vector(ctx: Context) -> Outcome:
    """🧫️ The same verb on its committed `(before, mutation, after)` vector, whose before-snapshot is
    the real committed building artifact decoded — a THIRD statement of what the verb means."""
    before_uri, mutation_uri, after_uri = ctx.step_input_uris()[:3]
    before = snapshot_json(ctx, before_uri)
    after = snapshot_json(ctx, after_uri)
    applied = apply_mutation(before, snapshot_json(ctx, mutation_uri))
    if applied != after:
        raise AssertionError("%s: the applied model does not match the committed after-snapshot\n     got: %s\nexpected: %s" % (ctx.scenario["id"], json.dumps(applied), json.dumps(after)))
    restored = applied
    for step in inverse_mutation(before, snapshot_json(ctx, mutation_uri)):
        restored = apply_mutation(restored, step)
    if restored != before:
        raise AssertionError("%s: undoing the committed mutation did not restore its before-snapshot" % ctx.scenario["id"])
    return Outcome({"applied": applied, "restored": restored})


def identity_round_trip(ctx: Context) -> Outcome:
    """🔁️ Both committed encodings of the demo building, plus the real capsule tower, each re-emitted
    from the parsed document.

    `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact
    re-emission is the CORRECT answer here and the wave's must-differ tripwire would be backwards.
    What keeps it from being vacuous is that each side is measured against bytes the OTHER one
    emitted: the demo building's two files were written by the Rust codec and are reproduced here
    from the grammar alone, while the capsule tower's two files were written by this implementation
    and the Rust codec has to reproduce THOSE — including the `OT` element class, the `M` geometry
    reference and the absent `spatialId`, three tags no committed pack had exercised before."""
    building_dsl = ctx.input_bytes(BUILDING_DSL)
    building = parse_dsl(building_dsl.decode("utf-8"))
    printed = print_dsl(building).encode("utf-8")
    if printed != building_dsl:
        raise AssertionError("re-printing the demo building did not reproduce the committed DSL bytes (%d vs %d bytes)" % (len(printed), len(building_dsl)))
    building_pack = ctx.input_bytes(BUILDING_PACK)
    if parse_pack(building_pack) != building:
        raise AssertionError("the demo building's binary twin decodes to a different model than its text")
    repacked = pack_bytes(building)
    if repacked != building_pack:
        raise AssertionError("re-encoding the demo building did not reproduce the committed pack bytes (%d vs %d bytes)" % (len(repacked), len(building_pack)))
    tower_dsl = ctx.input_bytes(TOWER_DSL)
    document = parse_dsl(tower_dsl.decode("utf-8"))
    tower_printed = print_dsl(document).encode("utf-8")
    if tower_printed != tower_dsl:
        raise AssertionError("re-printing the capsule tower did not reproduce its committed DSL bytes (%d vs %d bytes)" % (len(tower_printed), len(tower_dsl)))
    committed_tower_pack = ctx.input_bytes(TOWER_PACK)
    if parse_pack(committed_tower_pack) != document:
        raise AssertionError("the capsule tower's binary twin decodes to a different model than its text")
    tower_repacked = pack_bytes(document)
    if tower_repacked != committed_tower_pack:
        raise AssertionError("re-encoding the capsule tower did not reproduce its committed pack bytes (%d vs %d bytes)" % (len(tower_repacked), len(committed_tower_pack)))
    return Outcome(
        {
            "building": building,
            "buildingDslDigest": digest(printed),
            "buildingPackDigest": digest(repacked),
            "towerDslDigest": digest(tower_printed),
            "towerPackDigest": digest(tower_repacked),
            "towerSpatial": len(document["spatial"]),
            "towerElements": len(document["elements"]),
            "towerRelations": len(document["relations"]),
            "towerDslLength": len(tower_printed),
            "towerPackLength": len(tower_repacked),
        }
    )


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. Handlers are registered under the Scenario Outline base
    ids, which the host resolves for every Examples row, and plain scenarios under their own ids."""
    return Adapter("python").oracle("mutate", mutate).oracle("inverse", inverse).oracle("spec-vector", spec_vector).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
