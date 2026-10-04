#!/usr/bin/env python3
"""🤝️ Oracle of the wire version: the version the contract states, pinned to the fingerprint of the contract without its prose.

A proctor and a client agree on the contract by the wire version alone (``$defs/WireVersion``): every quiz envelope
carries it and a proctor declares it for every quiz kind at ``GET /instance``. That only holds while the version is
raised with every change of the contract that is not prose. The fingerprint makes that checkable: the normative
schema without its prose — every ``description``, ``title`` and ``$comment`` whose value is a string; a property named
``description`` is a schema, not prose, and stays — written as JSON with sorted keys, no whitespace and no escaping
beyond JSON's own (Python's ``json.dumps`` with ``sort_keys``, ``separators`` and ``ensure_ascii=False``), hashed
with FNV-1a 64 over its UTF-8 bytes. The reference fails when the version and fingerprint of the schema are not the
committed pair: the contract changed without a raise, or the pair was raised without being committed.

Run directly, this file prints the pair to commit to ``shared://🤝️wire-version/🔣️.json`` once the version is raised.

@see ../../🧬️schema/🔣️.json — ``$defs/WireVersion``
@see ../../🧫️fixtures/🤝️wire-version/🔣️.json
@see http://www.isthe.com/chongo/tech/comp/fnv/ — FNV-1a
"""

# region 🔖️Imports
import json
import os

# endregion 🔖️Imports


# region 🔖️Reference
SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧬️schema", "🔣️.json")
VECTORS = "shared://🤝️wire-version/🔣️.json"
PROSE = ("description", "title", "$comment")


def schema():
    """📜️ The normative contract."""
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        return json.load(handle)


def without_prose(node):
    """✂️ ``node`` without its prose: the string values of ``description``, ``title`` and ``$comment``."""
    if isinstance(node, dict):
        return {key: without_prose(value) for key, value in node.items() if not (key in PROSE and isinstance(value, str))}
    if isinstance(node, list):
        return [without_prose(value) for value in node]
    return node


def fnv1a64(data):
    """🔢️ FNV-1a 64 of ``data`` as 16 lowercase hex digits."""
    hashed = 0xCBF29CE484222325
    for byte in data:
        hashed = ((hashed ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return "%016x" % hashed


def reference():
    """🤝️ The wire version the schema states and the fingerprint of the schema without its prose."""
    contract = schema()
    canonical = json.dumps(without_prose(contract), sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return {"wireVersion": contract["$defs"]["WireVersion"]["const"], "fingerprint": fnv1a64(canonical.encode("utf-8"))}


# endregion 🔖️Reference


# region 🔖️Handlers
def agreement(ctx):
    """🤝️ The pair of the schema, which must be the committed one."""
    from semio_repo_test import Outcome

    produced = reference()
    committed = {key: value for key, value in json.loads(ctx.fixture_bytes(VECTORS)).items() if key != "$comment"}
    if produced != committed:
        raise AssertionError(
            "the contract and its wire version disagree with the committed pair: the schema states %r, %s commits %r. "
            "A change beyond prose raises $defs/WireVersion in 🧬️schema/🔣️.json and WIRE_VERSION in its 🟦️.ts and 🦀️.rs twins, "
            "and commits the pair this file prints when run directly." % (produced, VECTORS, committed)
        )
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: Python's own JSON writer and an FNV-1a written from its definition are the reference, the cores' constants and canonical writers the subjects."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("agreement", agreement)


# endregion 🔖️Registration


if __name__ == "__main__":
    print(json.dumps({"$comment": "Printed by 🧪️tests/🤝️wire-version/🐍️.py run directly — never edit by hand.", **reference()}, indent=2, ensure_ascii=False))
