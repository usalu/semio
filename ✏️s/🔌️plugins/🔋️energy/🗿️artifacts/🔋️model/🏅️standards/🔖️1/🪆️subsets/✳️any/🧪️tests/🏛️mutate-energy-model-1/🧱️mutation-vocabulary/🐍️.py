"""🧱️ Shared vocabulary of the mutation kinds: the committed wire form, outcomes, artifact links and the helpers more than one group uses."""

from __future__ import annotations

import copy
import json


# region 🔖️Wire
def unwrap(wire):
    """📨 Splits the committed mutation document into its kind tag and its argument object. The wire
    tag is the lowerCamel spelling of the Rust variant (`renameModel`), not the kebab catalog id."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


def wire_tag(kind: str) -> str:
    """🔤 The catalog id (kebab) a wire tag maps to, read off the committed manifest's
    `productionDispatch`, never transliterated from a Rust variant name."""
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)
# endregion 🔖️Wire


# region 🔖️Outcomes
def applied(*messages):
    """🎯️ An applied outcome and its ordered diagnostics."""
    return {"status": "applied", "messages": [{"level": level, "code": code} for level, code in messages]}


def no_op():
    """🟰️ A no-op outcome: nothing changes, and the one diagnostic says so."""
    return {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}


def rejected(code, path, invariant=None):
    """⛔️ A refusal: one fault code, the offending address, and — for a payload-intrinsic rule the leaf schema declares in
    `x-semio-invariant` — the id of that rule."""
    return {"status": "rejected", "code": code, "path": list(path), **({"invariant": invariant} if invariant is not None else {})}


def unchanged(before):
    """🪞 A refused or no-op step leaves the document exactly where it was."""
    return copy.deepcopy(before)
# endregion 🔖️Outcomes


# region 🔖️Links
def parse_uri(uri):
    """🔗️ `<artifactId>!<artifactKind>@<standard>/<subset>` — the flattened `ArtifactRef` form the
    snapshot schema's link slots carry. Anything else is not a reference."""
    if not isinstance(uri, str) or "!" not in uri:
        return None
    artifact_id, rest = uri.split("!", 1)
    if "@" not in rest or "/" not in rest:
        return None
    artifact_kind, rest = rest.split("@", 1)
    standard, subset = rest.split("/", 1)
    if not artifact_id or not artifact_kind or not standard or not subset or "/" in subset:
        return None
    return {"artifactId": artifact_id, "dialect": {"artifactKind": artifact_kind, "standard": standard, "subset": subset}}


def head_link(target, role):
    """🔗️ A head-pinned link filling one named slot."""
    return {"target": target, "pin": {"kind": "head"}, "role": role}
# endregion 🔖️Links
