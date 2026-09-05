#!/usr/bin/env python3
"""🗒️ The whole-document identity oracle for `s.note.note`, relocated out of the artifact-level
`mutate-note-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`. Unlike
its eight mutation siblings (`📜️document`, `🎨️canvas`, `🖋️ink`, `🖼️asset`, `🧱️block`, `📝️text`,
`🧮️math`, `📊️table`), this case REFUSES its one scenario by clause rather than implementing it — see
`refuse_carrier` below.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
from semio_repo_test import Adapter

# endregion 🔖️Imports


# region 🔖️Plan
def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


# endregion 🔖️Plan


# region 🔖️Handlers
def refuse_carrier(ctx):
    """🚧️ `identity-round-trip` reads this subset's own `.note.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence.

    This subset DOES commit a real grammar — `../../🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`,
    not the repository-wide `payload = OCTET+` placeholder its `✒️writer`, `🌿️vcs` and `🔌️wires`
    siblings carry — and that is exactly what makes the gap citable rather than vague. The committed
    grammar and the committed artifact disagree on three points:

    1. `block = text-block | image-block | shape-block`. The vocabulary declares SIX block kinds —
       text, stroke, table, math, image, group — and the mutation catalogs across this artifact's
       eight subsets carry verbs for all six (`edit-block-math`, `change-block-ink-width`,
       `insert-table-row`, `move-block-to-container`). Three of the six have no production at all, so
       a reader written from the grammar cannot parse a document that carries them, and no committed
       statement says how they are spelled.
    2. `block-field` lists `paragraphs` and `asset-id`; the committed artifact writes neither and
       writes `content=child_id=… target="…"`, a FLATTENED nested record nothing bounds. Only a
       field table can say that `target` belongs to `content` and the next key does not.
    3. `artifact-mark = "note.note"`, while the committed artifact's first line is
       `semio note.note.dsl v1`.

    Every mutation subset's adjudicated scenarios rest on committed JSON vectors and are unaffected.
    What is missing here is a grammar that covers the six block kinds this vocabulary actually has.
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
    """🧭️ Registration in the ORACLE role only — registering this handler as a subject too would make
    the reference its own subject and manufacture a green self-comparison."""
    built = Adapter("python")
    return built.oracle("identity-round-trip", refuse_carrier)
# endregion 🔖️Registration
