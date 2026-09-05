#!/usr/bin/env python3
"""🗂️ An INDEPENDENT second implementation of the `sourcing.curation` document and all three of its
typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A curation document is a composed
`s.stdio.semio@v1/kit` catalogue handle, a bulk-populated stock table and an ORDERED, id-keyed
curation of `(objectId, count)` pairs. No third party reads `.curation.dsl.semio`, and no bill-of-
materials or inventory library is authoritative over this vocabulary: `stock` is not in the
vocabulary at all — it is replaced wholesale through a non-history path — and a `CuratedItem` is two
scalars with no rename and no nested collection, so the entire surface is create/delete/change over
one ordered list. What a reference can adjudicate is exactly that: membership, POSITION and count.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — `CurationSnapshot` is
  `catalog`, `stockExtra` and `curated`; a `CuratedItem` is `{objectId: string, count: uint32}` with
  `additionalProperties: false`; an `ObjectKindExtra` is `{id, name, moduleId, typologyPath,
  availability, geometry}` and a `GeometryRecipe` is one of `box | frame | slab | mesh`.
* ``…/🧬️schema/🧬️mutations/🔣️.json`` — the three variants and their INTERNALLY tagged wire
  form: `{"mutation": "createCuratedItem", "item": …}`, `{"mutation": "deleteCuratedItem",
  "objectId": …}`, `{"mutation": "changeCuratedItemCount", "objectId": …, "newCount": …}`.
* ``…/🧬️schema/🧬️mutations/📖️.grammar.semio`` — the three verbs and their positional
  argument lists.
* the three committed `(before, mutation, after, diff, outcome)` specification vectors.

**What this implementation deliberately does not do, and why.** It does not read
`.curation.dsl.semio`. That carrier has no prose document, and its committed example is a structured
document — a catalogue handle, a flat stock member list whose per-entry geometry recipe starts the
line AFTER the entry it belongs to, and a declared-column `curated` table — whose encoding rules for
strings, absent members and numbers cannot be read off one example. The real-document scenarios
therefore read a snapshot fixture derived once from that committed file, with its provenance
recorded in the feature description, and the carrier's own laws stay asserted in role on the Rust
side.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("catalog", "stockExtra", "curated")
"""🗂️ The three members `CurationSnapshot` declares — and the cross-language projection."""

KINDS = ("create-curated-item", "delete-curated-item", "change-curated-item-count")
"""🏷️ Every kind the catalog declares."""

TAGS = {"create-curated-item": "createCuratedItem", "delete-curated-item": "deleteCuratedItem", "change-curated-item-count": "changeCuratedItemCount"}
"""🔤️ The internally tagged `mutation` discriminator of each kind, as the committed schema spells it."""

GEOMETRY_FIELDS = {"box": ("width", "height", "depth"), "frame": ("width", "height", "depth", "profile"), "slab": ("width", "depth", "thickness"), "mesh": ("positions", "normals", "indices")}
"""📐️ The members each `GeometryRecipe` variant declares, from the committed snapshot JSON Schema."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the document to the committed JSON Schema, including the parts no mutation edits —
    a stock table that silently lost its geometry would otherwise pass every curation check."""
    if not isinstance(document.get("catalog"), dict):
        raise AssertionError("catalog must be an object, found %r" % document.get("catalog"))
    for entry in document["stockExtra"]:
        if set(entry) != {"id", "name", "moduleId", "typologyPath", "availability", "geometry"}:
            raise AssertionError("a stock entry must be exactly the six declared members, found %r" % sorted(entry))
        if not isinstance(entry["availability"], int) or isinstance(entry["availability"], bool) or entry["availability"] < 0:
            raise AssertionError("a stock entry's availability must be a uint32, found %r" % entry["availability"])
        recipe = entry["geometry"]
        if not isinstance(recipe, dict) or recipe.get("kind") not in GEOMETRY_FIELDS:
            raise AssertionError("a stock entry's geometry must be one of %r, found %r" % (sorted(GEOMETRY_FIELDS), recipe))
        if set(recipe) != {"kind", *GEOMETRY_FIELDS[recipe["kind"]]}:
            raise AssertionError("a %s recipe must be exactly %r, found %r" % (recipe["kind"], sorted(GEOMETRY_FIELDS[recipe["kind"]]), sorted(recipe)))
    identifiers = []
    for item in document["curated"]:
        if set(item) != {"objectId", "count"}:
            raise AssertionError("a curated item must be exactly {objectId, count}, found %r" % sorted(item))
        if not isinstance(item["objectId"], str):
            raise AssertionError("a curated item's objectId must be a string, found %r" % item["objectId"])
        if not isinstance(item["count"], int) or isinstance(item["count"], bool) or item["count"] < 0:
            raise AssertionError("a curated item's count must be a uint32, found %r" % item["count"])
        identifiers.append(item["objectId"])
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("the curation carries a duplicate objectId: %r" % identifiers)


def document_of(payload):
    """📥️ Reads the three declared members out of a snapshot JSON value."""
    document = {name: payload[name] for name in MEMBERS}
    validate(document)
    return document


def index_of(items, identifier):
    """🔎️ The position of an objectId in the curation, or `None` when it is absent."""
    for position, item in enumerate(items):
        if item["objectId"] == identifier:
            return position
    return None


# endregion 🔖️Document


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting document.

    Every rejection is an error rather than a silent no-op: a quietly skipped mutation would report
    a pass having done nothing.
    """
    kind = kind_of(mutation)
    items = [dict(item) for item in document["curated"]]
    if kind == "create-curated-item":
        item = mutation["item"]
        if index_of(items, item["objectId"]) is not None:
            raise AssertionError("%s: %r is already curated" % (kind, item["objectId"]))
        items.append({"objectId": item["objectId"], "count": item["count"]})
    elif kind == "delete-curated-item":
        at = index_of(items, mutation["objectId"])
        if at is None:
            raise AssertionError("%s: %r is not curated" % (kind, mutation["objectId"]))
        items.pop(at)
    else:
        at = index_of(items, mutation["objectId"])
        if at is None:
            raise AssertionError("%s: %r is not curated" % (kind, mutation["objectId"]))
        items[at] = {"objectId": mutation["objectId"], "count": mutation["newCount"]}
    result = dict(document)
    result["curated"] = items
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to.

    A deleted item has to come BACK where it was, and this vocabulary has no insert-at-index verb,
    so the inverse of a delete is only exact for a trailing item — which is why the restoring law
    below compares position for position rather than membership.
    """
    kind = kind_of(mutation)
    items = document["curated"]
    if kind == "create-curated-item":
        return {"mutation": TAGS["delete-curated-item"], "objectId": mutation["item"]["objectId"]}
    at = index_of(items, mutation["objectId"])
    if at is None:
        raise AssertionError("inverse of %s: %r is not curated" % (kind, mutation["objectId"]))
    if kind == "delete-curated-item":
        return {"mutation": TAGS["create-curated-item"], "item": dict(items[at])}
    return {"mutation": TAGS["change-curated-item-count"], "objectId": mutation["objectId"], "newCount": items[at]["count"]}


# endregion 🔖️Mutations


# region 🔖️Laws
def entries(document):
    """🧾️ The curation as an ordered `(objectId, count)` list — what every positional claim reads."""
    return [(item["objectId"], item["count"]) for item in document["curated"]]


def effect_holds(kind, effect, before, after):
    """👁️ The positional observability law the feature's `effect` column states.

    `append` requires exactly one more entry with every member already present still at its own
    index; `detach` requires exactly one fewer with the survivors in their original relative order;
    `retune` requires the same length, the same ids at the same indices and exactly one count moved.
    All three fail an implementation that rebuilt or re-sorted the curation, which a membership
    comparison would let through.
    """
    was, now = entries(before), entries(after)
    if effect == "append":
        if len(now) != len(was) + 1:
            raise AssertionError("%s: an append must leave exactly one more curated entry, went from %d to %d" % (kind, len(was), len(now)))
        if now[: len(was)] != was:
            raise AssertionError("%s: an append must land AFTER the members already present, but the leading entries changed: %r -> %r" % (kind, was, now))
    elif effect == "detach":
        if len(now) + 1 != len(was):
            raise AssertionError("%s: a detach must leave exactly one fewer curated entry, went from %d to %d" % (kind, len(was), len(now)))
        survivors = [entry for entry in was if entry in now]
        if survivors != now:
            raise AssertionError("%s: a detach must leave the survivors in their original order, got %r from %r" % (kind, now, was))
    elif effect == "retune":
        if len(now) != len(was):
            raise AssertionError("%s: a count change must not add or drop an entry, went from %d to %d" % (kind, len(was), len(now)))
        moved = [at for at in range(len(was)) if was[at] != now[at]]
        if len(moved) != 1:
            raise AssertionError("%s: a count change must move exactly one entry, moved %d: %r -> %r" % (kind, len(moved), was, now))
        if was[moved[0]][0] != now[moved[0]][0]:
            raise AssertionError("%s: a count change must keep the entry at its own index, but %r became %r" % (kind, was[moved[0]][0], now[moved[0]][0]))
    else:
        raise AssertionError("%s: the feature declares an unknown effect %r" % (kind, effect))
    for name in ("catalog", "stockExtra"):
        if before[name] != after[name]:
            raise AssertionError("%s: the vocabulary edits `curated` only, but %s moved too" % (kind, name))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, position for position, reported by the first index that moved."""
    was, now = entries(original), entries(restored)
    if len(was) != len(now):
        raise AssertionError("inverse-%s: the curation came back with %d entr(ies), not %d" % (kind, len(now), len(was)))
    for at, (left, right) in enumerate(zip(was, now)):
        if left != right:
            raise AssertionError("inverse-%s: curated[%d] came back as %r, not %r" % (kind, at, right, left))
    for name in ("catalog", "stockExtra"):
        if restored[name] != original[name]:
            raise AssertionError("inverse-%s: %s did not come back" % (kind, name))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, over the three members the schema declares."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True), json.dumps(committed[name], sort_keys=True)))


# endregion 🔖️Laws


# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the real derived timber-kit curation."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
        payload = json.loads(doc_string(ctx))
        mutation, effect = payload["mutation"], payload["effect"]
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        effect_holds("mutate-%s" % kind, effect, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived curation and then its OWN computed inverse.

    The projection carries BOTH documents; projecting only the restored one would make all three
    rows project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
        payload = json.loads(doc_string(ctx))
        mutation, effect = payload["mutation"], payload["effect"]
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        effect_holds("inverse-%s" % kind, effect, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": applied, "restored": restored})

    return handler


def spec_vector_handler(kind):
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        effect_holds("spec-vector-%s" % kind, json.loads(doc_string(ctx))["effect"], before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real curation and answers with the three declared members.

    This implementation additionally requires, in role, that the derived document really is the
    committed kit: ten stock entries, a curation drawn from them, and no curated count above the
    stock availability it was derived from — checks a codec that dropped a member could not pass.
    The `.curation.dsl.semio` carrier's own laws are asserted in role on the Rust side, against the
    artifact's committed example; see this file's module docstring for why they are not mirrored
    here.
    """
    document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
    if len(document["stockExtra"]) != 10:
        raise AssertionError("identity-round-trip: the committed kit carries ten stock entries, read %d" % len(document["stockExtra"]))
    availability = {entry["id"]: entry["availability"] for entry in document["stockExtra"]}
    for item in document["curated"]:
        if item["objectId"] not in availability:
            raise AssertionError("identity-round-trip: %r is curated but is not in the kit" % item["objectId"])
        if item["count"] > availability[item["objectId"]]:
            raise AssertionError("identity-round-trip: %r is curated at %d against an availability of %d" % (item["objectId"], item["count"], availability[item["objectId"]]))
    reread = document_of(json.loads(json.dumps(document)))
    if reread != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return outcome_of(document)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)


# endregion 🔖️Registration
