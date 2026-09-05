"""🐍️ Python oracle adapter for the RFC 7493 I-JSON mutation vocabulary.

The reference is ``simplejson``, registered by this subset's own 🧪️oracle contribution
(``../../🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🔣️oracle.json``) and put on this
host's import path by the ``python`` entry in the plugin's ``oracleHostPackages``. Nothing in this
file knows where the interpreter came from — the coordinator provisions it.

Why the reference runs here and not in Rust: RFC 7493 restricts the JSON VALUE SPACE, so the
reference has to surface three facts a normalizing codec throws away — every object's ORDERED member
list including duplicate names (``object_pairs_hook``), the exact number LEXEME
(``parse_int``/``parse_float`` receive the raw digits, and ``Decimal`` + ``use_decimal`` carry them
back out unrounded), and the decoded string. It both parses and re-serializes, so every one of the
ten kinds is genuinely differential.

The ONE thing this file must never do is re-implement the subject's own mutation code. It does not:
the mutation semantics here are written against ``simplejson``'s own decoded model
(``dict``/``list``/``Decimal``/``str``), the RFC 7493 clause checks are written from the RFC's text,
and the inverse of every kind is recomputed by reading the ORIGINAL document rather than by asking
the subject for its inverse.
"""

from __future__ import annotations

# region 🔖️Imports
import decimal

import simplejson

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Clauses
DOCUMENT = "shared://🔣️.json"

#: 🔢️ RFC 7493 §2.2 — the largest integer magnitude an IEEE-754 double represents exactly.
MAX_SAFE_INTEGER_MAGNITUDE = 9007199254740991

#: 📇️ The kinds this case covers, mirroring the `json-rfc8259-i-json` catalog. Duplicated rather
#: than imported: the catalog lives in JSON the framework owns the parsing of, and the enum lives in
#: a Rust crate this Python host never links. The contract phase fails with
#: `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list drifts from either.
KINDS = [
    "no-mutation",
    "set-snapshot",
    "set-top-level",
    "upsert-member",
    "remove-member",
    "rename-member",
    "set-safe-number",
    "set-string",
    "insert-array-element",
    "remove-array-element",
]


def is_noncharacter(character: str) -> bool:
    """🚫️ §2.4 — the last two code points of every plane, plus the reserved BMP range U+FDD0..U+FDEF."""
    code_point = ord(character)
    return (code_point & 0xFFFE) == 0xFFFE or 0xFDD0 <= code_point <= 0xFDEF


def is_integer_lexeme(lexeme: str) -> bool:
    """🔢️ RFC 8259 puts `.`/`e`/`E` only in the fraction and exponent parts, so their absence is integrality."""
    return "." not in lexeme and "e" not in lexeme and "E" not in lexeme


def is_safe_number_lexeme(lexeme: str) -> bool:
    """🔢️ §2.2 on the DIGITS, never through a lossy double. A non-integer lexeme is outside the clause."""
    if not is_integer_lexeme(lexeme):
        return True
    try:
        return abs(int(lexeme)) <= MAX_SAFE_INTEGER_MAGNITUDE
    except ValueError:
        return False


# endregion 🔖️Clauses


# region 🔖️Codec
def _pairs_hook(pairs):
    """🧬️ §2.3 observed rather than assumed: the reference reports the member list in document order
    WITH duplicates, and this hook refuses to hand back an object that carries one. A dict-producing
    parser would have collapsed them silently, which is exactly why the hook is used at all."""
    seen = set()
    for name, _ in pairs:
        if name in seen:
            raise AssertionError("RFC 7493 §2.3: object member name %r appears more than once" % name)
        seen.add(name)
    return dict(pairs)


def parse(text: str):
    """📥️ Decode through the reference, keeping number lexemes exact and member order intact."""
    return simplejson.loads(text, object_pairs_hook=_pairs_hook, parse_int=decimal.Decimal, parse_float=decimal.Decimal)


def serialize(value) -> bytes:
    """📤️ Re-encode through the reference. `use_decimal` emits each number's own retained lexeme."""
    return simplejson.dumps(value, use_decimal=True, ensure_ascii=False, sort_keys=False).encode("utf-8")


def plain(value):
    """🎯️ The compared projection: the reference's own decoded model with `Decimal` narrowed to the
    float the `semantic-i-json-v1` profile compares with a 1e-12 tolerance, because the host writes
    the projection with the standard library's own JSON writer."""
    if isinstance(value, decimal.Decimal):
        return float(value)
    if isinstance(value, dict):
        return {name: plain(item) for name, item in value.items()}
    if isinstance(value, list):
        return [plain(item) for item in value]
    return value


def document(ctx: Context):
    """🧫️ The committed fixture, copied into the work directory first and decoded by the reference."""
    with open(ctx.copy_fixture(DOCUMENT, "input.json"), "rb") as handle:
        return parse(handle.read().decode("utf-8"))


def spec(ctx: Context):
    """📄️ The scenario's own `{"kind": ..., "params": ...}` doc string, decoded with the same hooks so
    a number written in the feature table is a `Decimal` exactly as one read from the document is."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return parse(step["docString"])
    raise AssertionError("scenario %s carries no mutation spec doc string" % ctx.scenario["id"])


# endregion 🔖️Codec


# region 🔖️Navigation
def resolve(root, path):
    """🔎️ Walks `path` (a member name or an array index per step) and returns the addressed node."""
    node = root
    for segment in path:
        if isinstance(segment, decimal.Decimal) or isinstance(segment, int):
            index = int(segment)
            if not isinstance(node, list) or index >= len(node):
                raise AssertionError("path segment %r does not address an array element" % segment)
            node = node[index]
        else:
            if not isinstance(node, dict) or segment not in node:
                raise AssertionError("path segment %r does not address an object member" % segment)
            node = node[segment]
    return node


def path_of(params) -> list:
    return list(params.get("path", []))


# endregion 🔖️Navigation


# region 🔖️Mutations
def apply_mutation(root, mutation):
    """▶️ The ten I-JSON kinds, performed on the REFERENCE's own decoded model. Each clause-carrying
    kind refuses rather than writes, exactly as RFC 7493 requires — a refusal is an `AssertionError`,
    which the host records as a failed scenario instead of a silent no-op."""
    kind = mutation["kind"]
    params = mutation.get("params") or {}

    if kind == "no-mutation":
        return root

    if kind == "set-snapshot":
        return params["value"]

    if kind == "set-top-level":
        if "object" in params:
            new_root = params["object"]
            if not isinstance(new_root, dict):
                raise AssertionError("RFC 7493 §2.1: set-top-level's `object` payload is not an object")
            return new_root
        if "array" in params:
            new_root = params["array"]
            if not isinstance(new_root, list):
                raise AssertionError("RFC 7493 §2.1: set-top-level's `array` payload is not an array")
            return new_root
        raise AssertionError("RFC 7493 §2.1: set-top-level carries neither an `object` nor an `array` payload — a scalar document root is unrepresentable")

    target = resolve(root, path_of(params))

    if kind == "upsert-member":
        if not isinstance(target, dict):
            raise AssertionError("upsert-member: the addressed path is not an object")
        target[params["key"]] = params["value"]
        return root

    if kind == "remove-member":
        if not isinstance(target, dict) or params["key"] not in target:
            raise AssertionError("remove-member: the object carries no member named %r" % params["key"])
        del target[params["key"]]
        return root

    if kind == "rename-member":
        source, destination = params["from"], params["to"]
        if not isinstance(target, dict) or source not in target:
            raise AssertionError("rename-member: the object carries no member named %r" % source)
        if source != destination and destination in target:
            raise AssertionError("RFC 7493 §2.3: the object already carries a member named %r, so this rename would create a duplicate" % destination)
        renamed = {(destination if name == source else name): item for name, item in target.items()}
        target.clear()
        target.update(renamed)
        return root

    if kind == "set-safe-number":
        lexeme = params["lexeme"]
        if not isinstance(target, decimal.Decimal):
            raise AssertionError("set-safe-number: the addressed path does not hold a number")
        if not is_safe_number_lexeme(lexeme):
            raise AssertionError("RFC 7493 §2.2: integer %s exceeds ±%d = ±(2^53−1) and is not exactly representable as an IEEE-754 double" % (lexeme, MAX_SAFE_INTEGER_MAGNITUDE))
        return replace_at(root, path_of(params), decimal.Decimal(lexeme))

    if kind == "set-string":
        value = params["value"]
        if not isinstance(target, str):
            raise AssertionError("set-string: the addressed path does not hold a string")
        offending = next((character for character in value if is_noncharacter(character)), None)
        if offending is not None:
            raise AssertionError("RFC 7493 §2.4: the value carries the Unicode noncharacter U+%04X" % ord(offending))
        return replace_at(root, path_of(params), value)

    if kind == "insert-array-element":
        if not isinstance(target, list):
            raise AssertionError("insert-array-element: the addressed path is not an array")
        target.insert(min(int(params["index"]), len(target)), params["value"])
        return root

    if kind == "remove-array-element":
        index = int(params["index"])
        if not isinstance(target, list) or index >= len(target):
            raise AssertionError("remove-array-element: index %d is out of range" % index)
        del target[index]
        return root

    raise AssertionError("unknown I-JSON mutation kind %r" % kind)


def replace_at(root, path, value):
    """🔁️ Replaces the whole node at `path` — the one operation `resolve` cannot do in place, because
    a `Decimal` and a `str` are immutable. An empty path replaces the document root."""
    if not path:
        return value
    parent = resolve(root, path[:-1])
    last = path[-1]
    if isinstance(last, (decimal.Decimal, int)) and not isinstance(last, str):
        parent[int(last)] = value
    else:
        parent[last] = value
    return root


def inverse_spec(original, mutation):
    """↩️ The undo, recomputed INDEPENDENTLY by reading the pre-mutation document — never by asking
    the subject for its own inverse, which would compare an implementation with itself."""
    kind = mutation["kind"]
    params = mutation.get("params") or {}
    path = path_of(params)

    if kind in ("no-mutation",):
        return {"kind": "no-mutation", "params": {}}
    if kind == "set-snapshot":
        return {"kind": "set-snapshot", "params": {"value": original}}
    if kind == "set-top-level":
        if isinstance(original, dict):
            return {"kind": "set-top-level", "params": {"object": original}}
        return {"kind": "set-top-level", "params": {"array": original}}
    if kind == "upsert-member":
        parent = resolve(original, path)
        key = params["key"]
        if key in parent:
            return {"kind": "upsert-member", "params": {"path": path, "key": key, "value": parent[key]}}
        return {"kind": "remove-member", "params": {"path": path, "key": key}}
    if kind == "remove-member":
        parent = resolve(original, path)
        key = params["key"]
        return {"kind": "upsert-member", "params": {"path": path, "key": key, "value": parent[key]}}
    if kind == "rename-member":
        return {"kind": "rename-member", "params": {"path": path, "from": params["to"], "to": params["from"]}}
    if kind == "set-safe-number":
        return {"kind": "set-safe-number", "params": {"path": path, "lexeme": str(resolve(original, path))}}
    if kind == "set-string":
        return {"kind": "set-string", "params": {"path": path, "value": resolve(original, path)}}
    if kind == "insert-array-element":
        target = resolve(original, path)
        return {"kind": "remove-array-element", "params": {"path": path, "index": min(int(params["index"]), len(target))}}
    if kind == "remove-array-element":
        index = int(params["index"])
        return {"kind": "insert-array-element", "params": {"path": path, "index": index, "value": resolve(original, path)[index]}}
    raise AssertionError("no inverse rule for kind %r" % kind)


# endregion 🔖️Mutations


# region 🔖️Scenarios
def projected(value):
    """👁️ The compared PROJECTION SHAPE, which both halves of this case must present identically or
    the profile compares two different questions: `{"format": "json", "value": <document>}`.

    That envelope is not this file's invention — it is what the independent Rust reader
    (`project_json_value`, `../../🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🦀️.rs`) emits,
    and what the ✳️any sibling case is already compared through on both sides. Returning the bare
    document here instead made every `mutate-`/`inverse-`/`identity-` row disagree on `$.format` and
    `$.value` alone, while the documents underneath were identical — a shape mismatch reported as a
    semantic one."""
    return {"format": "json", "value": plain(value)}


def mutate(ctx: Context) -> Outcome:
    """🔮️ One handler shared by every `mutate-<kind>` scenario id — the scenario's own spec selects
    which kind runs, which is why one function covers all ten.

    A mutation that silently changes nothing would still hand back a projection the subject could
    match, so every kind except `no-mutation` is required here to actually move the document. That is
    an assertion the reference can make ALONE, without waiting for the subject phase."""
    original = document(ctx)
    forward = spec(ctx)
    mutated = apply_mutation(document(ctx), forward)
    if forward["kind"] != "no-mutation" and plain(mutated) == plain(original):
        raise AssertionError("%s left the document unchanged — a mutation that applies to nothing is a silent no-op, not a pass" % forward["kind"])
    if forward["kind"] == "no-mutation" and plain(mutated) != plain(original):
        raise AssertionError("no-mutation changed the document")
    return Outcome(projected(mutated), serialize(mutated))


def inverse(ctx: Context) -> Outcome:
    """🔮️ One handler shared by every `inverse-<kind>` scenario id: apply, then undo, then project.

    The metamorphic law is asserted HERE, by the reference against its own pre-mutation document, so
    the property holds as evidence on its own rather than only as an oracle/subject agreement. Object
    member order is deliberately not part of it — RFC 8259 §4 leaves member order to the producer and
    the `semantic-i-json-v1` profile agrees, which is exactly what Python's own `dict` equality means."""
    original = document(ctx)
    forward = spec(ctx)
    undo = inverse_spec(original, forward)
    restored = apply_mutation(apply_mutation(document(ctx), forward), undo)
    if plain(restored) != plain(original):
        raise AssertionError("applying %s and then its own inverse did not restore the document" % forward["kind"])
    return Outcome(projected(restored), serialize(restored))


def i_json_conformance(ctx: Context) -> Outcome:
    """🔮️ Every clause RFC 7493 adds to RFC 8259, checked against the real document by the reference.
    This scenario is what earns the feature's claim that the fixture is an I-JSON document — the
    duplicate-name clause is enforced inside `_pairs_hook` during the decode itself."""
    with open(ctx.copy_fixture(DOCUMENT, "input.json"), "rb") as handle:
        raw = handle.read()
    root = parse(raw.decode("utf-8"))
    if not isinstance(root, (dict, list)):
        raise AssertionError("RFC 7493 §2.1: the top-level value is a bare scalar")

    integers = 0
    unsafe = []
    strings = 0
    noncharacters = []

    def walk(node, where):
        nonlocal integers, strings
        if isinstance(node, decimal.Decimal):
            lexeme = str(node)
            if is_integer_lexeme(lexeme):
                integers += 1
                if not is_safe_number_lexeme(lexeme):
                    unsafe.append("%s = %s" % (where, lexeme))
        elif isinstance(node, str):
            strings += 1
            if any(is_noncharacter(character) for character in node):
                noncharacters.append(where)
        elif isinstance(node, dict):
            for name, item in node.items():
                if any(is_noncharacter(character) for character in name):
                    noncharacters.append("%s/%s (member name)" % (where, name))
                walk(item, "%s/%s" % (where, name))
        elif isinstance(node, list):
            for index, item in enumerate(node):
                walk(item, "%s[%d]" % (where, index))

    walk(root, "")
    if unsafe:
        raise AssertionError("RFC 7493 §2.2: %d integer(s) outside ±(2^53−1): %s" % (len(unsafe), ", ".join(unsafe[:5])))
    if noncharacters:
        raise AssertionError("RFC 7493 §2.4: %d string(s) carry a Unicode noncharacter: %s" % (len(noncharacters), ", ".join(noncharacters[:5])))
    return Outcome(
        {
            "topLevel": "object" if isinstance(root, dict) else "array",
            "duplicateMemberNames": 0,
            "integers": integers,
            "unsafeIntegers": 0,
            "strings": strings,
            "noncharacterStrings": 0,
            "bytes": len(raw),
        }
    )


def identity_round_trip(ctx: Context) -> Outcome:
    """🔒️ BOTH halves of the identity law, asserted here by the reference alone.

    The no-byte-pass-through half: the reference fully parses the real document and re-serializes
    from its own model alone, so an output equal to the input would be indistinguishable from a read
    that never parsed anything. The semantic half: the re-encoded bytes are read BACK and their
    projection compared against the projection of the decoded original — projecting the in-memory
    model alone (which is what this handler used to return) can never catch a writer that drops or
    reshapes something on the way out."""
    with open(ctx.copy_fixture(DOCUMENT, "input.json"), "rb") as handle:
        raw = handle.read()
    root = parse(raw.decode("utf-8"))
    output = serialize(root)
    if output == raw:
        raise AssertionError("byte pass-through: the reference's output is bit-identical to the input")
    reread = plain(parse(output.decode("utf-8")))
    if reread != plain(root):
        raise AssertionError("identity law violated: re-reading the reference's own output does not reproduce the decoded document")
    return Outcome(projected(parse(output.decode("utf-8"))), output)


# endregion 🔖️Scenarios


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls. `mutate-<kind>` / `inverse-<kind>` share ONE handler
    each across all ten kinds — the scenario id only selects which Examples row's doc string the
    shared handler reads."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate).oracle("inverse-%s" % kind, inverse)
    return built.oracle("i-json-conformance", i_json_conformance).oracle("identity-round-trip", identity_round_trip)


# endregion 🔖️Registration
