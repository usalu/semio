#!/usr/bin/env python3
"""🗒️ An INDEPENDENT second implementation of the `s.note.note` document and its thirty-three typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `note` document is a NESTED,
POSITIONAL canvas: blocks are id-keyed but their ORDER is the z-order the canvas paints in, a `group`
block contains other blocks, and six block kinds — text, stroke, table, math, image, group — carry
different members and are reached by different verbs. No canvas or whiteboard format models a
z-ordered tree whose leaves are typed like that, and none of them reads `.dsl.semio`. That this
algebra IS adjudicable was settled in this same wave by `mutate-semio-drawing`, `mutate-semio-mesh`
and the fifteen `📕️norm` references, which took Python second implementations over this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the document's members
  and the six block kinds.
* rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  document-level scalars, the id-keyed asset table, the ORDERED block tree, rule 5's
  `move-to-<container>` for re-parenting, and absolute `move`/`resize` against relative `drag`.
* the thirty-three committed `(before, mutation, after, outcome)` vectors, which are the only
  statement of five things: that `drag-blocks` translates a named block AND ITS WHOLE SUBTREE; that
  `delete-asset` removes the `assets` MEMBER outright when it empties the table rather than leaving an
  empty map; that `insert-table-column` names the appended column by its spreadsheet letter; that
  `insert-table-row`/`insert-table-column` append BLANK cells, which is what makes their inverses
  exact against the committed trailing-blank vectors; and how `duplicate-blocks` places its copies.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.

**TWO FINDINGS the reference made while it was being written, reported rather than absorbed.**

1. `duplicate-blocks` computes each copy's insertion index against the PRE-MUTATION list and does not
   re-base it as earlier copies land. Its committed vector duplicates `blk-ink` (root index 1) and
   `blk-table` (root index 2) in one mutation, and the committed after-snapshot orders the root
   `blk-text, blk-ink, blk-ink-copy, blk-table-COPY, blk-table, …` — the second copy lands BEFORE its
   own source. This implementation reproduces the committed order and names it here rather than
   quietly re-basing; `duplicate-block`, the singular verb, does place its copy after the source.
2. `edit-block-text` is REFUSED — see `UNSTATED_REASON`.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
REQUIRED = ("schema", "id", "title", "blocks", "gridVisible", "gridSpacing", "gridSubdivisions", "gridOpacity", "snapEnabled", "snapGridSpacing", "pencilWidth", "eraserRadius")
"""🗂️ The members every committed note snapshot carries. `assets` is optional: `delete-asset` removes
the member outright when it empties the table."""

MEMBERS = REQUIRED + ("assets",)

DOCUMENT_FIELDS = {"rename-note": ("title", "newTitle"), "change-grid-visible": ("gridVisible", "newVisible"), "change-grid-spacing": ("gridSpacing", "newSpacing"), "change-grid-subdivisions": ("gridSubdivisions", "newSubdivisions"), "change-grid-opacity": ("gridOpacity", "newOpacity"), "change-snap-enabled": ("snapEnabled", "newEnabled"), "change-snap-grid-spacing": ("snapGridSpacing", "newSpacing"), "change-pencil-width": ("pencilWidth", "newWidth"), "change-eraser-radius": ("eraserRadius", "newRadius")}
"""✏️ The nine document-level scalar setters of rule 1."""

BLOCK_FIELDS = {"rename-block": ("name", "newName"), "change-block-visible": ("visible", "newVisible"), "change-block-locked": ("locked", "newLocked"), "change-block-font-size": ("fontSize", "newFontSize"), "edit-block-math": ("tex", "newTex"), "change-block-ink-width": ("strokeWidth", "newStrokeWidth")}
"""🧱️ The six single-field block setters, all addressed by `id` anywhere in the tree."""

UNSTATED = {"edit-block-text"}
"""🚧️ The one kind this implementation refuses to state — see `UNSTATED_REASON`."""

UNSTATED_REASON = (
    "this implementation refuses this kind rather than guessing it. A text block does not hold its paragraphs: it holds a COMPOSED CHILD HANDLE "
    "`{childId, target}` into an `s.stdio.semio@v1/text` document. The committed vector's whole observable effect is that handle's `childId` moving "
    "from `note-text-eea42a3b80b1052b` to `note-text-938222b3522927c6` — a content address of the child AFTER the new paragraphs are written — and no "
    "document in this repository states the addressing function or the child text document's canonical encoding. Every other verb over that same "
    "block IS implemented: `change-block-font-size`, `rename-block`, `move-block` and the rest all move members the note document itself holds. "
    "`mutate-program-1` reports the identical blocker over `knowledge`/`benchmarks`, `mutate-block-3d-1` over `catalog`, and `mutate-en1990-1`'s two "
    "red scenarios are the same finding again: publishing the child-addressing rule closes all of them, and no comparison profile moves."
)

KINDS = (
    "rename-note",
    "change-grid-visible",
    "change-grid-spacing",
    "change-grid-subdivisions",
    "change-grid-opacity",
    "change-snap-enabled",
    "change-snap-grid-spacing",
    "change-pencil-width",
    "change-eraser-radius",
    "create-asset",
    "replace-asset-payload",
    "delete-asset",
    "create-block",
    "delete-block",
    "delete-blocks",
    "duplicate-block",
    "duplicate-blocks",
    "move-block-to-container",
    "drag-blocks",
    "rename-block",
    "change-block-visible",
    "change-block-locked",
    "move-block",
    "resize-block",
    "change-block-font-size",
    "edit-block-text",
    "edit-block-math",
    "change-block-ink-width",
    "edit-block-ink-stroke",
    "insert-table-row",
    "remove-table-row",
    "insert-table-column",
    "remove-table-column",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def column_name(index):
    """🔤️ The spreadsheet letter of a column by position — `A`, `B`, … `Z`, `AA`. The committed
    `insert-table-column` vector appends `C` to a two-column table, which is the only statement of the
    rule and the only position it exercises."""
    name = ""
    while True:
        name = chr(ord("A") + index % 26) + name
        index = index // 26 - 1
        if index < 0:
            return name


def walk(blocks):
    """🌲 Every block in the tree, parents before children."""
    for block in blocks:
        yield block
        for nested in block.get("children", []):
            yield nested
        for nested in block.get("children", []):
            for deeper in walk(nested.get("children", [])):
                yield deeper


def siblings_of(blocks, identity, parent=None):
    """🌲 The list one block sits in, its index in it, and the id of its container (`None` at root)."""
    for at, block in enumerate(blocks):
        if block["id"] == identity:
            return blocks, at, parent
        found = siblings_of(block.get("children", []), identity, block["id"])
        if found is not None:
            return found
    return None


def located(document, identity, kind, where):
    """🔎️ The list, index and container of the block this kind addresses; an absent id is an error."""
    found = siblings_of(document["blocks"], identity)
    if found is None:
        raise AssertionError("%s-%s: the committed vector addresses block %r, which the before-snapshot's tree does not hold" % (where, kind, identity))
    return found


def container_list(document, parent_id, kind, where):
    """🌲 The child list a re-parenting verb targets — the root list when the parent is `null`."""
    if parent_id is None:
        return document["blocks"]
    rows, at, _parent = located(document, parent_id, kind, where)
    return rows[at].setdefault("children", [])


def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: the twelve always-present
    members, `assets` only beyond them, unique block ids ANYWHERE in the tree, and a table block whose
    every row is exactly as wide as its column list."""
    if not set(REQUIRED) <= set(document):
        raise AssertionError("%s: a note document must carry %r, found %r" % (where, sorted(REQUIRED), sorted(document)))
    if not set(document) <= set(MEMBERS):
        raise AssertionError("%s: a note document may carry only %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    identifiers = [block["id"] for block in walk(document["blocks"])]
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: the block tree carries a duplicate id in %r" % (where, identifiers))
    for block in walk(document["blocks"]):
        if block["kind"] == "table":
            for row in block["rows"]:
                if len(row) != len(block["columns"]):
                    raise AssertionError("%s: table block %r has a %d-cell row against %d columns" % (where, block["id"], len(row), len(block["columns"])))
        if block["kind"] == "group" and "children" not in block:
            raise AssertionError("%s: group block %r carries no children list" % (where, block["id"]))


def translated(block, dx, dy):
    """📍 One block and its WHOLE SUBTREE moved by a relative offset — rule 7's `drag`."""
    block["x"] += dx
    block["y"] += dy
    for nested in block.get("children", []):
        translated(nested, dx, dy)
    return block
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    if kind in UNSTATED:
        raise AssertionError("mutate-%s: %s" % (kind, UNSTATED_REASON))
    document = copy.deepcopy(document)
    if kind in DOCUMENT_FIELDS:
        member, argument = DOCUMENT_FIELDS[kind]
        document[member] = payload[argument]
    elif kind == "create-asset":
        document.setdefault("assets", {})[payload["key"]] = copy.deepcopy(payload["asset"])
    elif kind == "replace-asset-payload":
        if payload["key"] not in document.get("assets", {}):
            raise AssertionError("mutate-%s: the asset table holds no %r" % (kind, payload["key"]))
        document["assets"][payload["key"]] = copy.deepcopy(payload["newAsset"])
    elif kind == "delete-asset":
        if payload["key"] not in document.get("assets", {}):
            raise AssertionError("mutate-%s: the asset table holds no %r" % (kind, payload["key"]))
        document["assets"].pop(payload["key"])
        if not document["assets"]:
            document.pop("assets")
    elif kind == "create-block":
        rows = container_list(document, payload.get("parentId"), kind, "mutate")
        index = payload.get("index")
        rows.insert(len(rows) if index is None else index, copy.deepcopy(payload["block"]))
    elif kind == "delete-block":
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        rows.pop(at)
    elif kind == "delete-blocks":
        for identity in payload["ids"]:
            rows, at, _parent = located(document, identity, kind, "mutate")
            rows.pop(at)
    elif kind == "duplicate-block":
        rows, at, _parent = located(document, payload["sourceId"], kind, "mutate")
        rows.insert(at + 1, copy.deepcopy(payload["block"]))
    elif kind == "duplicate-blocks":
        places = [(located(document, identity, kind, "mutate")[2], located(document, identity, kind, "mutate")[1]) for identity in payload["sourceIds"]]
        for (parent_id, at), block in zip(places, payload["blocks"]):
            container_list(document, parent_id, kind, "mutate").insert(at + 1, copy.deepcopy(block))
    elif kind == "move-block-to-container":
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        block = rows.pop(at)
        target = container_list(document, payload["newParentId"], kind, "mutate")
        index = payload.get("index")
        target.insert(len(target) if index is None else index, block)
    elif kind == "drag-blocks":
        for identity in payload["ids"]:
            rows, at, _parent = located(document, identity, kind, "mutate")
            translated(rows[at], payload["dx"], payload["dy"])
    elif kind in BLOCK_FIELDS:
        member, argument = BLOCK_FIELDS[kind]
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        rows[at][member] = payload[argument]
    elif kind == "move-block":
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        rows[at]["x"], rows[at]["y"] = payload["newX"], payload["newY"]
    elif kind == "resize-block":
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        rows[at]["width"], rows[at]["height"] = payload["newWidth"], payload["newHeight"]
    elif kind == "edit-block-ink-stroke":
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        block = rows[at]
        block["points"] = copy.deepcopy(payload["newPoints"])
        block["x"], block["y"] = payload["newX"], payload["newY"]
        block["width"], block["height"] = payload["newWidth"], payload["newHeight"]
    elif kind in ("insert-table-row", "remove-table-row", "insert-table-column", "remove-table-column"):
        rows, at, _parent = located(document, payload["id"], kind, "mutate")
        table = rows[at]
        if table["kind"] != "table":
            raise AssertionError("mutate-%s: block %r is a %s, not a table" % (kind, payload["id"], table["kind"]))
        if kind == "insert-table-row":
            table["rows"].append([{"content": ""} for _ in table["columns"]])
        elif kind == "remove-table-row":
            if not table["rows"]:
                raise AssertionError("mutate-%s: table %r holds no row to remove" % (kind, payload["id"]))
            table["rows"].pop()
        elif kind == "insert-table-column":
            table["columns"].append(column_name(len(table["columns"])))
            for row in table["rows"]:
                row.append({"content": ""})
        else:
            if not table["columns"]:
                raise AssertionError("mutate-%s: table %r holds no column to remove" % (kind, payload["id"]))
            table["columns"].pop()
            for row in table["rows"]:
                row.pop()
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. The tree verbs invert by POSITION, not by membership: a deleted block is put
    back into its own container at its own index, and `delete-blocks` puts its blocks back in ASCENDING
    index order so each lands where it was. The table verbs invert exactly only because the committed
    vectors act on a TRAILING row or column whose cells are blank, which the committed after-snapshots
    are the only statement of."""
    if kind in UNSTATED:
        raise AssertionError("inverse-%s: %s" % (kind, UNSTATED_REASON))
    if kind in DOCUMENT_FIELDS:
        member, argument = DOCUMENT_FIELDS[kind]
        return [(kind, {argument: document[member]})]
    if kind == "create-asset":
        return [("delete-asset", {"key": payload["key"]})]
    if kind == "replace-asset-payload":
        return [(kind, {"key": payload["key"], "newAsset": copy.deepcopy(document["assets"][payload["key"]])})]
    if kind == "delete-asset":
        return [("create-asset", {"key": payload["key"], "asset": copy.deepcopy(document["assets"][payload["key"]])})]
    if kind == "create-block":
        return [("delete-block", {"id": payload["block"]["id"]})]
    if kind == "delete-block":
        rows, at, parent = located(document, payload["id"], kind, "inverse")
        return [("create-block", {"block": copy.deepcopy(rows[at]), "parentId": parent, "index": at})]
    if kind == "delete-blocks":
        places = sorted((located(document, identity, kind, "inverse")[1], identity) for identity in payload["ids"])
        steps = []
        for at, identity in places:
            rows, index, parent = located(document, identity, kind, "inverse")
            steps.append(("create-block", {"block": copy.deepcopy(rows[index]), "parentId": parent, "index": index}))
        return steps
    if kind == "duplicate-block":
        return [("delete-block", {"id": payload["block"]["id"]})]
    if kind == "duplicate-blocks":
        return [("delete-blocks", {"ids": [block["id"] for block in payload["blocks"]]})]
    if kind == "move-block-to-container":
        rows, at, parent = located(document, payload["id"], kind, "inverse")
        return [(kind, {"id": payload["id"], "newParentId": parent, "index": at})]
    if kind == "drag-blocks":
        return [(kind, {"ids": list(payload["ids"]), "dx": -payload["dx"], "dy": -payload["dy"]})]
    if kind in BLOCK_FIELDS:
        member, argument = BLOCK_FIELDS[kind]
        rows, at, _parent = located(document, payload["id"], kind, "inverse")
        return [(kind, {"id": payload["id"], argument: rows[at][member]})]
    if kind == "move-block":
        rows, at, _parent = located(document, payload["id"], kind, "inverse")
        return [(kind, {"id": payload["id"], "newX": rows[at]["x"], "newY": rows[at]["y"]})]
    if kind == "resize-block":
        rows, at, _parent = located(document, payload["id"], kind, "inverse")
        return [(kind, {"id": payload["id"], "newWidth": rows[at]["width"], "newHeight": rows[at]["height"]})]
    if kind == "edit-block-ink-stroke":
        rows, at, _parent = located(document, payload["id"], kind, "inverse")
        block = rows[at]
        return [(kind, {"id": payload["id"], "newPoints": copy.deepcopy(block["points"]), "newX": block["x"], "newY": block["y"], "newWidth": block["width"], "newHeight": block["height"]})]
    if kind == "insert-table-row":
        return [("remove-table-row", {"id": payload["id"]})]
    if kind == "insert-table-column":
        return [("remove-table-column", {"id": payload["id"]})]
    if kind in ("remove-table-row", "remove-table-column"):
        rows, at, _parent = located(document, payload["id"], kind, "inverse")
        table = rows[at]
        if kind == "remove-table-row":
            if any(cell["content"] != "" for cell in table["rows"][-1]):
                raise AssertionError("inverse-%s: the removed row was not BLANK, and `insert-table-row` can only append a blank one, so this vocabulary cannot restore its content" % kind)
            return [("insert-table-row", {"id": payload["id"]})]
        if any(row[-1]["content"] != "" for row in table["rows"]):
            raise AssertionError("inverse-%s: the removed column was not BLANK, and `insert-table-column` can only append a blank one, so this vocabulary cannot restore its content" % kind)
        return [("insert-table-column", {"id": payload["id"]})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if produced.get(member, "⌀") != committed.get(member, "⌀"):
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:400], json.dumps(committed.get(member), sort_keys=True)[:400]))


def observable(kind, before, after):
    """👁️ All thirty-three committed vectors of this subset declare `status: applied` and all
    thirty-three move the document, which is why there is no exemption list here and none is needed."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member, index for index and depth for depth — which for a
    positional tree means an inverse that restored membership but not POSITION fails here."""
    for member in sorted(set(restored) | set(original)):
        if restored.get(member, "⌀") != original.get(member, "⌀"):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:400], json.dumps(original.get(member), sort_keys=True)[:400]))
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
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
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
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status and observability."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        before = json_fixture(ctx, "⬅️before")
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        if outcome.get("status") != "applied":
            raise AssertionError("mutate-%s: the committed outcome declares %r; this feature replays applied vectors only" % (kind, outcome.get("status")))
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(ctx, kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back, position for position."""

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
    """🚧️ `identity-round-trip` reads this subset's own `.note.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence.

    This subset DOES commit a real grammar — `🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`, not
    the repository-wide `payload = OCTET+` placeholder its `✒️writer`, `🌿️vcs` and `🔌️wires` siblings
    carry — and that is exactly what makes the gap citable rather than vague. The committed grammar and
    the committed artifact disagree on three points:

    1. `block = text-block | image-block | shape-block`. The vocabulary declares SIX block kinds —
       text, stroke, table, math, image, group — and the mutation catalog carries verbs for all six
       (`edit-block-math`, `change-block-ink-width`, `insert-table-row`, `move-block-to-container`).
       Three of the six have no production at all, so a reader written from the grammar cannot parse a
       document that carries them, and no committed statement says how they are spelled.
    2. `block-field` lists `paragraphs` and `asset-id`; the committed artifact writes neither and
       writes `content=child_id=… target="…"`, a FLATTENED nested record the grammar does not mention.
       Nothing bounds it: only a field table can say that `target` belongs to `content` and the next
       key does not.
    3. `artifact-mark = "note.note"`, while the committed artifact's first line is
       `semio note.note.dsl v1`.

    The thirty-two adjudicated mutation scenarios of this case rest on committed JSON vectors and are
    unaffected. What is missing here is a grammar that covers the six block kinds this vocabulary
    actually has.
    """
    committed = ctx.fixture_bytes(uri_in(ctx, "🗣️.dsl.semio"))
    raise AssertionError(
        "identity-round-trip: this subset's `.dsl.semio` carrier cannot be read by a second implementation. Unlike its `✒️writer`, `🌿️vcs` and "
        "`🔌️wires` siblings this subset commits a REAL grammar rather than the `payload = OCTET+` placeholder, and the grammar and the artifact "
        "disagree on three points: the grammar's `block = text-block | image-block | shape-block` covers three of the SIX block kinds this "
        "vocabulary declares, leaving stroke, table, math and group with no production at all; its `block-field` list names `paragraphs` and "
        "`asset-id` while the committed artifact writes neither and writes `content=child_id=… target=\"…\"`, a flattened nested record nothing "
        "bounds; and its `artifact-mark = \"note.note\"` is contradicted by the artifact's own first line `semio note.note.dsl v1`. Read %d bytes of "
        "the committed artifact and refused to guess their meaning. Extending the grammar to the six block kinds closes it." % len(committed)
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
