"""🐍️ `s.remodel.remodeling`'s second, independent implementation of all 36 of its mutation kinds.

`s.remodel.remodeling` is a semio-NATIVE reconstruction project document — streams, calibrations,
ground control points, the eight `ReconstructionParams` sub-records a pipeline runs under, the
durable content leaves a run publishes, and the engine-owned results — not a point cloud or a mesh
file. A reader of COLMAP, LAS or PLY output would
be judging a different artifact, and nothing reads `.dsl.semio`. This subset's own no-oracle decision
(`remodeling-mutation-semantics`) records that survey. The reference is therefore a second
IMPLEMENTATION, written from this subset's own committed
`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape, each kind's own committed
`(before, mutation, after)` leaf fixture, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
verb table (rule 5 in particular: an inverse is computed from BASE and must restore a member's
POSITION, not merely its presence). It imports nothing from the Rust it judges and transliterates
none of it.

🚦 Two answers, not one. A vector whose scenario doc string carries a `code` member declares that
this payload is REFUSED (or warned as a no-op) and that the document therefore does not move; the
reference then states, from the verb's own meaning, WHICH diagnostic that payload earns and requires
the committed after-document to be the before-document unchanged. Every other vector is applied and
compared field by field. The refusal statements below are derived from what each verb MEANS — a
stream may not be created twice, a frame index must address a frame that exists, a commit binds only
content the document already stores complete — not from production's control flow, and they carry none of
production's diagnostic prose.

📍️ WHERE A VECTOR LIVES IS THE FEATURE'S ANSWER, not this module's. Every scenario carries a doc
string naming its `(before, mutation, after)` URIs, resolved through the plan at RUN time. Nothing
here transcribes a fixture path: the 2026-09-05 repo-wide path-shortening pass renamed every case
directory under this subset, and a hard-coded table of those names is exactly the drift that broke
this case before.

🔑 `create-asset` — the ONE genuine content-address hazard in this vocabulary. Production mints a
NEW `assets.<key>.childId` via `image_asset_child_handle`, which hashes the raw `ImageAsset` bytes
through `std::collections::hash_map::DefaultHasher` — an algorithm the Rust standard library
EXPLICITLY documents as unspecified and not portable, even across compiler versions. This reference
therefore compares every field EXCEPT that one opaque digest exactly, and only checks the digest's
SHAPE (`remodeling-asset-` followed by hex) — an honest, narrow scope limit, not a silent pass. It
does state the DURABLE leaf that digest keys independently and in full: the bounded 4 KiB chunking of
the payload's own base64 content, its mime and its declared raster size are all recomputed here, and
only the key they are filed under is adopted. `delete-asset`'s inverse sidesteps the hazard entirely:
`assets[key]` in the committed BEFORE-document is already a fully-formed captured handle.

⛓️ OWNERSHIP AND ORDER, the two rules every verb here is read against. A record may be removed with
what it OWNS (a stream carries its frames, a GCP carries its observations) but never with what merely
NAMES it: a stream a GCP observation addresses, a camera a stream or a rig entry binds, an asset a
frame, texture or geo product references, all earn `mutation.target-referenced` instead. And every keyed
collection is held in ascending key order, so a `create`/`add` puts a member back exactly where a
`delete`/`remove` took it from. Together those make every kind's inverse a single step of this same
vocabulary that restores the committed BEFORE-document exactly — this module needs no synthetic
"move" step to state it, and it asserts that for all 136 committed vectors.

📦️ DURABLE CONTENT is ordinary document state. A reconstruction run publishes every sparse cloud, mesh
and raster as bounded 4 KiB leaves through `append-content` (removed again by `remove-content`), and
`commit-reconstruction` then names that content by id: a `remodeling-content:<id>|<leaves>` point
buffer, a `remodeling-mesh-content:<leaves>` mesh child, or an asset binding. Completeness of named
content is therefore decidable from the BASE document alone, so the commit is modelled in full here —
its refusals, its no-op, its forward application and its inverse.

🧩️ Layout: this file registers the adapter; the vocabulary tables and addressing live in `📖️vocabulary`, the forward appliers in
`⚙️forward-appliers`, the refusal rules and the single-step inverse in `⛔️refusals-and-inverses`, and the oracle with the scenario
handlers in `🔮️oracle-scenarios`.
"""

from __future__ import annotations

# region 🔖️Imports
import importlib.util
import os
import sys

from semio_repo_test import Adapter


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


_load("mutate_remodeling_1_vocabulary", "📖️vocabulary")
_load("mutate_remodeling_1_appliers", "⚙️forward-appliers")
_load("mutate_remodeling_1_rules", "⛔️refusals-and-inverses")
_load("mutate_remodeling_1_scenarios", "🔮️oracle-scenarios")

from mutate_remodeling_1_scenarios import (
    mutate,
    inverse,
    identity_handler,
    MUTATE_SCENARIOS,
    INVERSE_SCENARIOS,
)
# endregion 🔖️Imports


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables, in
    the ORACLE role only — registering these handlers as subjects too would make the reference its own
    subject and manufacture a green self-comparison. Every planned scenario is answered: the runner
    plans a role over ALL of them and errors on a gap, so a kind left out would report as an
    unregistered scenario rather than as the honest scope limit it was meant to be."""
    built = Adapter("python")
    for scenario in MUTATE_SCENARIOS:
        built = built.oracle(f"mutate-{scenario}", mutate)
    for scenario in INVERSE_SCENARIOS:
        built = built.oracle(f"inverse-{scenario}", inverse)
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration
