#!/usr/bin/env python3
"""♻️ An INDEPENDENT second implementation of the `s.trinity.rewrite` graph-rewrite rule and all
seven of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A rewrite rule here is five members:
three whole JSON DOCUMENTS carried as strings — the before-fixture graph, the left-hand pattern and
the right-hand side — plus two string-keyed maps, one of parameter bindings and one of layout
points. Graph-rewriting systems (GrGen, AGG, `networkx`'s isomorphism module) implement rewriting;
none of them models THIS rule document, none reads `.dsl.semio`, and none has an opinion on whether
`change-parameter-binding` on a key that is absent should insert or refuse. What a reference can
genuinely adjudicate is the document algebra — three whole-value setters and a
set/remove pair over each of two maps — and that is what this file implements, from the
specification, in another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — `RewriteSnapshot` is
  exactly `beforeFixtureJson`, `lhsJson`, `rhsJson` (all three `contentMediaType:
  application/json`), `parameterBindings` (a map of open `PropertyValue`s) and `ruleLayout` (a map of
  `{x, y}` `LayoutPoint`s), `additionalProperties: false`.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`` — the seven verbs and their positional
  argument lists: three `edit-<member> text`, `change-parameter-binding key value`,
  `remove-parameter-binding key`, `change-rule-layout-point key point-block`,
  `remove-rule-layout-point key`.
* the seven committed `(before, mutation, after, outcome)` specification vectors, which give the
  INTERNALLY tagged wire form of each verb. All seven are ACCEPTING, so unlike this artifact's
  `🔌️jack` sibling the accepting direction here already had committed evidence.

**What this implementation deliberately does not do, and why.** It does not read `.rewrite.dsl.semio`.
That carrier has no prose document and mixes three different value encodings in one file — a
backslash-escaped quoted string, a braced block, and a fenced ```json block — with nothing stating
which member gets which. A reference that guessed the rule and then claimed byte-exact reproduction
would be asserting a specification that does not exist. The real-document scenarios therefore read a
snapshot fixture derived once from that committed file (provenance in the feature description), and
the carrier's own laws stay asserted in role on the Rust side.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("beforeFixtureJson", "lhsJson", "rhsJson", "parameterBindings", "ruleLayout")
"""🗂️ The five members `RewriteSnapshot` declares — and the cross-language projection."""

KINDS = ("edit-before-fixture", "edit-lhs", "edit-rhs", "change-parameter-binding", "remove-parameter-binding", "change-rule-layout-point", "remove-rule-layout-point")
"""🏷️ Every kind the catalog declares."""

TAGS = {
    "edit-before-fixture": "editBeforeFixture",
    "edit-lhs": "editLhs",
    "edit-rhs": "editRhs",
    "change-parameter-binding": "changeParameterBinding",
    "remove-parameter-binding": "removeParameterBinding",
    "change-rule-layout-point": "changeRuleLayoutPoint",
    "remove-rule-layout-point": "removeRuleLayoutPoint",
}
"""🔤️ The internally tagged `mutation` discriminator of each kind, as the committed vectors spell it."""

DOCUMENTS = {"edit-before-fixture": ("beforeFixtureJson", "newBeforeFixtureJson"), "edit-lhs": ("lhsJson", "newLhsJson"), "edit-rhs": ("rhsJson", "newRhsJson")}
"""📄️ The three whole-document setters: which member each writes, and what its argument is called."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the document to the committed JSON Schema — including that the three string members
    really are JSON documents, which is what `contentMediaType: application/json` declares and what a
    setter that wrote a truncated payload would break."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a rewrite rule must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    for name in DOCUMENTS.values():
        member = name[0]
        if not isinstance(document[member], str):
            raise AssertionError("%s must be a string, found %r" % (member, document[member]))
        try:
            json.loads(document[member])
        except ValueError as error:
            raise AssertionError("%s declares contentMediaType application/json but does not parse: %s" % (member, error))
    if not isinstance(document["parameterBindings"], dict):
        raise AssertionError("parameterBindings must be a map, found %r" % document["parameterBindings"])
    for key, point in document["ruleLayout"].items():
        if not isinstance(point, dict) or set(point) != {"x", "y"}:
            raise AssertionError("ruleLayout[%r] must be exactly {x, y}, found %r" % (key, point))


def document_of(payload):
    """📥️ Reads a rewrite rule out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


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

    `change-` on an absent key INSERTS and `remove-` on an absent key is a no-op that leaves the map
    alone: the grammar gives both verbs a bare `key` with no existence precondition, and the schema
    declares both members open maps rather than closed records.
    """
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind in DOCUMENTS:
        member, argument = DOCUMENTS[kind]
        result[member] = mutation[argument]
    elif kind == "change-parameter-binding":
        result["parameterBindings"][mutation["key"]] = copy.deepcopy(mutation["newValue"])
    elif kind == "remove-parameter-binding":
        result["parameterBindings"].pop(mutation["key"], None)
    elif kind == "change-rule-layout-point":
        point = mutation["newPoint"]
        result["ruleLayout"][mutation["key"]] = {"x": float(point["x"]), "y": float(point["y"])}
    else:
        result["ruleLayout"].pop(mutation["key"], None)
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to."""
    kind = kind_of(mutation)
    if kind in DOCUMENTS:
        member, argument = DOCUMENTS[kind]
        return {"mutation": TAGS[kind], argument: document[member]}
    map_name = "parameterBindings" if kind.endswith("parameter-binding") else "ruleLayout"
    change, remove = ("change-parameter-binding", "remove-parameter-binding") if map_name == "parameterBindings" else ("change-rule-layout-point", "remove-rule-layout-point")
    key = mutation["key"]
    if key not in document[map_name]:
        return {"mutation": TAGS[remove], "key": key}
    held = copy.deepcopy(document[map_name][key])
    return {"mutation": TAGS[change], "key": key, "newValue" if map_name == "parameterBindings" else "newPoint": held}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ Every row below writes a value the rule does not already hold, so a forward application
    must move it. A setter that quietly did nothing would otherwise pass by agreeing."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the rule untouched, so nothing was proved" % scenario)


def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the five members. An implementation that rebuilt the whole
    rule on every edit — re-serializing a JSON member, say — would satisfy an after-snapshot
    comparison and fail this."""
    written = DOCUMENTS[kind][0] if kind in DOCUMENTS else "parameterBindings" if kind.endswith("parameter-binding") else "ruleLayout"
    moved = [name for name in MEMBERS if before[name] != after[name]]
    if moved != [written]:
        raise AssertionError("%s: this verb writes %s and nothing else, but %r moved" % (scenario, written, moved))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the member that failed to come back."""
    for name in MEMBERS:
        if restored[name] != original[name]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, name, json.dumps(restored[name], sort_keys=True)[:200], json.dumps(original[name], sort_keys=True)[:200]))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True)[:200], json.dumps(committed[name], sort_keys=True)[:200]))


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
    """🎯️ Applies one kind to the real derived Nakagin ground-floor rule."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "nakagin-capsule-tower"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        touches_one("mutate-%s" % kind, kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived rule and then its OWN computed inverse.

    The projection carries BOTH rules; projecting only the restored one would make all seven rows
    project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "nakagin-capsule-tower"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
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
        observable("spec-vector-%s" % kind, before, applied)
        touches_one("spec-vector-%s" % kind, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real rule and answers with the whole document.

    This implementation additionally requires, in role, that the rule really is the committed one:
    its before-fixture is the two-node Nakagin ground floor with a real connection between the
    service core and the left capsule, its left-hand pattern carries a `whereClause`, and its
    right-hand side declares the `label` parameter its bindings bind. The `.dsl.semio` carrier's own
    laws are asserted in role on the Rust side, against the artifact's committed example.
    """
    ground_floor = document_of(json_fixture(ctx, "nakagin-ground-floor"))
    small = json.loads(ground_floor["beforeFixtureJson"])
    if small.get("name") != "Nakagin Capsule Tower — Ground Floor" or len(small["nodes"]) != 2 or len(small["edges"]) != 1:
        raise AssertionError("identity-round-trip: the committed ground-floor rule rewrites a two-node graph, found %r" % small.get("name"))
    document = document_of(json_fixture(ctx, "nakagin-capsule-tower"))
    fixture = json.loads(document["beforeFixtureJson"])
    ports = sum(len(node["ports"]) for node in fixture["nodes"])
    if fixture.get("name") != "Nakagin Capsule Tower" or len(fixture["nodes"]) != 180 or ports != 364 or len(fixture["edges"]) != 179:
        raise AssertionError("identity-round-trip: the rule rewrites the whole 180-node 364-port 179-edge Nakagin building, found %r with %d node(s), %d port(s) and %d edge(s)" % (fixture.get("name"), len(fixture["nodes"]), ports, len(fixture["edges"])))
    if fixture["rootNodeId"] != small["rootNodeId"]:
        raise AssertionError("identity-round-trip: the whole building and the ground floor name different root pieces, so they are not the same real model")
    for whole in (ground_floor, document):
        if "whereClause" not in json.loads(whole["lhsJson"]):
            raise AssertionError("identity-round-trip: the committed left-hand pattern carries a whereClause")
        declared = {parameter["name"] for parameter in json.loads(whole["rhsJson"])["parameters"]}
        unbound = [key for key in whole["parameterBindings"] if key not in declared]
        if unbound:
            raise AssertionError("identity-round-trip: %r is bound but not declared by the right-hand side" % unbound)
        if document_of(json.loads(json.dumps(whole))) != whole:
            raise AssertionError("identity-round-trip: serializing and re-reading the rule moved it")
    return outcome_of({"groundFloor": ground_floor, "capsuleTower": document})


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
