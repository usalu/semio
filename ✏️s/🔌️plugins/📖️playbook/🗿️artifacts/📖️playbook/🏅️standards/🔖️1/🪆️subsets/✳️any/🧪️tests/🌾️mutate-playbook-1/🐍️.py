#!/usr/bin/env python3
"""📖️ An INDEPENDENT second implementation of the `s.playbook.playbook` parent document and its one parent-lane mutation,
`change-title`, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `playbook` document is a HANDLE RECORD: the snapshot carries
`schema`, `id`, `version`, `title` and ONE composed child handle (`flow`), while the steps and blocks live in that child and are
edited only on its own lane (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). No form or checklist format
models a programme whose content is a child artifact, and none of them reads `.dsl.semio`.

**What it was written from.** ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` (the snapshot's members) and the
committed `(before, mutation, after, outcome)` vector. **No Rust was read to write this.**

The child-leaf builders the step/block verbs use are pinned separately by the language-neutral vectors
``🗿️artifacts/📖️playbook/🧫️fixtures/🧫️child-leaves/🔣️.json``, written by another independent Python implementation.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
REQUIRED = ("schema", "id", "version", "flow")
"""🗂️ The members every committed playbook snapshot carries; `title` is nullable and written by `change-title`."""

MEMBERS = REQUIRED + ("title",)

KINDS = ("change-title",)
"""🏷️ Every kind the catalog declares, in its declared order."""

TAGS = {"change-title": "changeTitle"}
"""🔤️ The internally tagged `mutation` discriminator of each kind."""

NO_OP = "mutation.no-op"
"""🚨️ The diagnostic a title change to the current title raises."""
# endregion 🔖️Vocabulary


# region 🔖️Verbs
def diagnose(kind, payload, document):
    """🚦️ The diagnostic this kind raises against this document, derived rather than read off the committed outcome."""
    if kind == "change-title":
        return (NO_OP, None) if payload.get("newTitle") == document.get("title") else (None, None)
    raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind to the snapshot: `change-title` writes the title member and nothing else."""
    document = copy.deepcopy(document)
    if kind == "change-title":
        document["title"] = payload.get("newTitle")
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse over the snapshot: a title change back to the base title."""
    if kind == "change-title":
        return [(kind, {"newTitle": document.get("title")})]
    return []
# endregion 🔖️Verbs


# region 🔖️Laws
def declared(outcome):
    """🚨️ The (status, code, path) a committed `🎯️outcome` vector declares."""
    listed = [message.get("code") for message in outcome.get("messages", []) if message.get("code")]
    code = listed[0] if listed else outcome.get("code")
    return outcome.get("status"), code, outcome.get("path")


def diagnoses_as_committed(kind, produced, outcome):
    """⚖️ The derived diagnostic against the committed one — status, code and path — asserted before anything else."""
    status, code, path = declared(outcome)
    derived_code, derived_path = produced
    derived_status = "no-op" if derived_code == NO_OP else "applied"
    if (derived_status, derived_code) != (status, code):
        raise AssertionError("mutate-%s: this implementation derives %r/%r from the document, the committed 🎯️outcome vector declares %r/%r" % (kind, derived_status, derived_code, status, code))
    if derived_path is not None and path is not None and derived_path != path:
        raise AssertionError("mutate-%s: this implementation derives the path %r, the committed vector declares %r" % (kind, derived_path, path))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if produced.get(member, "⌀") != committed.get(member, "⌀"):
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:300], json.dumps(committed.get(member), sort_keys=True)[:300]))


def restores(kind, restored, original):
    """↩️ The full inverse law, member for member."""
    for member in sorted(set(restored) | set(original)):
        if restored.get(member, "⌀") != original.get(member, "⌀"):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:300], json.dumps(original.get(member), sort_keys=True)[:300]))


def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: the four always-present members, `title` only beyond
    them, and one well-formed composed `flow` child handle."""
    if not set(REQUIRED) <= set(document):
        raise AssertionError("%s: a playbook document must carry %r, found %r" % (where, sorted(REQUIRED), sorted(document)))
    if not set(document) <= set(MEMBERS):
        raise AssertionError("%s: a playbook document may carry only %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    if set(document["flow"]) != {"childId", "target"}:
        raise AssertionError("%s: the composed flow child handle must carry exactly childId and target, found %r" % (where, sorted(document["flow"])))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def payload_of(ctx, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    payload = json_fixture(ctx, "🦠️mutation")
    if payload.get("mutation") != TAGS[kind]:
        raise AssertionError("%s: the committed vector carries a %r payload, not %r" % (ctx.scenario["id"], payload.get("mutation"), TAGS[kind]))
    return {key: value for key, value in payload.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Derives this kind's diagnostic from the document, asserts it against the committed outcome, and answers the snapshot
    the verb leaves behind."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        before = json_fixture(ctx, "⬅️before")
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        validate(before, "mutate-%s" % kind)
        payload = payload_of(ctx, kind)
        diagnoses_as_committed(kind, diagnose(kind, payload, before), outcome)
        applied = apply_mutation(before, kind, payload)
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot back, member for member."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        before = json_fixture(ctx, "⬅️before")
        payload = payload_of(ctx, kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def refuse_carrier(ctx):
    """🚧️ `identity-round-trip` reads this subset's own `.playbook.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence. The committed grammar
    `🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` describes a DIFFERENT DOCUMENT: it is the
    generic `family-scene` canvas grammar — `doc-body = schema-line layers-block`,
    `layer = shape-layer | path-layer | text-layer`, `canvas-field = "id" | "x" | "y" | "fill" |
    "stroke" | "opacity"` — and the committed artifact contains no `layers` block, no layer and no
    canvas field. What it does contain is HEX-ENCODED scalars and a `[hex,hex]` child-handle pair,
    none of which the grammar mentions. Four more subsets — `📋️forms`, `📏️layout`, `🖍️draw` and
    `🖨️raster` — carry the same canvas grammar over four equally unrelated documents, differing from
    this one only in the `grammar`, `extension` and `artifact-mark` lines."""
    committed = ctx.fixture_bytes(uri_in(ctx, "🗣️.dsl.semio"))
    raise AssertionError(
        "identity-round-trip: this subset's `.dsl.semio` carrier cannot be read by a second implementation. Its committed grammar describes a "
        "DIFFERENT document — the generic `family-scene` canvas grammar, `doc-body = schema-line layers-block` with shape/path/text layers and "
        "`id`/`x`/`y`/`fill`/`stroke`/`opacity` fields — while the committed artifact carries no `layers` block at all, and instead HEX-ENCODED "
        "scalars and a `[hex,hex]` child-handle pair the grammar never mentions. Nothing committed says the values are hex, that a pair is "
        "`(childId, target)`, or how the second element's `<artifactId>!<kind>@<standard>/<subset>` spelling is split. Four more subsets — `📋️forms`, `📏️layout`, "
        "`🖍️draw` and `🖨️raster` — carry the same canvas grammar over four equally unrelated documents, differing only in their `grammar`, "
        "`extension` and `artifact-mark` lines. Read %d bytes of the committed artifact and refused to "
        "guess their meaning." % len(committed)
    )
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
    return built.oracle("identity-round-trip", refuse_carrier)
# endregion 🔖️Registration
