"""📋 Declarative morph specification: frozen dataclasses, schema-driven validation, load/dump of transition documents.

Pure Python (no Manim, no NumPy, no third-party runtime). The JSON Schema in `🧬️schema/🔣️.schema.json` is the single source of truth; this module interprets the keyword subset the schema uses (`SUPPORTED_KEYWORDS`) and reports problems with a JSON path, a stable code and how to fix them. Reference: https://json-schema.org/draft/2020-12/json-schema-core
"""

from __future__ import annotations

import difflib
import json
import math
import re
from collections.abc import Mapping
from dataclasses import dataclass
from pathlib import Path

SCHEMA_PATH = Path(__file__).resolve().parents[1] / "🧬️schema" / "🔣️.schema.json"
ANNOTATIONS = frozenset({"$schema", "$id", "$defs", "title", "description", "default", "x-code", "x-fix"})
SUPPORTED_KEYWORDS = ANNOTATIONS | {"type", "enum", "const", "minimum", "maximum", "exclusiveMinimum", "pattern", "minLength", "minItems", "maxItems", "required", "properties", "additionalProperties", "items", "allOf", "if", "then", "$ref"}
PLACEMENT_PREFIX = "entity:"
WHOLE_RECIPES = frozenset({"move", "replace", "fade", "retext"})


@dataclass(frozen=True)
class Problem:
    """🚩 One validation problem: JSON path, stable machine code, and a message that says what is wrong and how to fix it."""

    path: str
    code: str
    message: str

    def __str__(self):
        return f"{self.path} [{self.code}] {self.message}"


class SpecError(ValueError):
    """⛔ Raised when a transition document is invalid; `problems` lists every finding."""

    def __init__(self, problems):
        self.problems = tuple(problems)
        super().__init__("invalid morph transition document:\n" + "\n".join(f"  {p}" for p in self.problems))


@dataclass(frozen=True)
class Endpoint:
    """🎯 One side of a transition: entity reference plus revision, sampling policy and view/scale/pose/scenario selectors."""

    entity: str
    revision: int = 1
    sampling: str = "snapshot"
    view: str | None = None
    scale: str | None = None
    pose: str | None = None
    scenario: str | None = None


@dataclass(frozen=True)
class Correspondence:
    """🔗 How source parts map to target parts: identity, key, order, explicit pairs or a named recipe correspondence."""

    kind: str
    pairs: tuple[tuple[str, str], ...] | None = None
    name: str | None = None


@dataclass(frozen=True)
class Timing:
    """⏱ Canonical seconds: duration, easing, stagger lag and stagger limit (None = unbounded)."""

    duration: float
    easing: str = "smooth"
    lag: float = 0.0
    limit: float | None = None


@dataclass(frozen=True)
class Placement:
    """📍 Where the endpoint lands: coordinate space, logical anchor, fit policy, uniform scaling."""

    space: str = "scene"
    anchor: str = "center"
    fit: str = "contain"
    uniform: bool = True


@dataclass(frozen=True)
class Final:
    """🎞 Full-morph tuning only used by the final profile: path arc, dip and ghost."""

    path_arc: float = 0.0
    dip: float = 0.0
    ghost: float = 0.0


@dataclass(frozen=True)
class Transition:
    """🔀 One declarative transition with a stable id; a null `source` (appear) or `target` (vanish) declares absence; `copy` keeps the source (Manim TransformFromCopy)."""

    id: str
    intent: str
    source: Endpoint | None
    target: Endpoint | None
    relation: str
    recipe: str
    timing: Timing
    correspondence: Correspondence | None = None
    copy: bool = False
    placement: Placement = Placement()
    final: Final = Final()


@dataclass(frozen=True)
class Group:
    """🧺 Named ordered set of transitions that play in sequence or in parallel."""

    id: str
    transitions: tuple[str, ...]
    mode: str = "sequence"


@dataclass(frozen=True)
class Document:
    """📄 A clip's transition document."""

    transitions: tuple[Transition, ...]
    groups: tuple[Group, ...] = ()
    version: int = 1

    def transition(self, transition_id):
        """🔎 Return the transition with `transition_id` or raise KeyError listing the known ids."""
        for item in self.transitions:
            if item.id == transition_id:
                return item
        raise KeyError(f"unknown transition {transition_id!r}; declared ids: {[t.id for t in self.transitions]}")


def _schema():
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


SCHEMA = _schema()


def _equal(a, b):
    numbers = all(isinstance(x, (int, float)) and not isinstance(x, bool) for x in (a, b))
    return (numbers or type(a) is type(b)) and a == b


def _is(kind, value):
    number = isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)
    checks = {
        "null": value is None,
        "boolean": isinstance(value, bool),
        "string": isinstance(value, str),
        "array": isinstance(value, list),
        "object": isinstance(value, Mapping),
        "number": number,
        "integer": number and float(value).is_integer(),
    }
    return checks[kind]


def _describe(value):
    return "null" if value is None else repr(value) if isinstance(value, (str, int, float, bool)) else type(value).__name__


def _resolve(node):
    ref = node.get("$ref") if isinstance(node, dict) else None
    if ref is None:
        return node
    target = SCHEMA
    for part in ref.removeprefix("#/").split("/"):
        target = target[part]
    return {**target, **{k: v for k, v in node.items() if k != "$ref"}}


def _problem(path, code, what, fix):
    return Problem(path, code, f"{what}. Fix: {fix}" if fix else what)


def _check(node, value, path):
    if node is True:
        return []
    if node is False:
        return [_problem(path, "forbidden", "this property is not allowed here", "remove it")]
    node = _resolve(node)
    found = _apply(node, value, path)
    code = node.get("x-code")
    if code and found:
        return [_problem(path, code, found[0].message.split(". Fix: ")[0], node.get("x-fix"))]
    return found


def _apply(node, value, path):
    kinds = node.get("type")
    if kinds is not None:
        kinds = [kinds] if isinstance(kinds, str) else kinds
        if not any(_is(k, value) for k in kinds):
            return [_problem(path, "type", f"expected {' or '.join(kinds)}, got {_describe(value)}", f"provide a value of type {' or '.join(kinds)}" + (" (finite numbers only)" if "number" in kinds or "integer" in kinds else ""))]
    out = []
    if "const" in node and not _equal(value, node["const"]):
        out.append(_problem(path, "const", f"expected {node['const']!r}, got {_describe(value)}", f"use {node['const']!r}"))
    if "enum" in node and not any(_equal(value, option) for option in node["enum"]):
        out.append(_problem(path, "enum", f"got {_describe(value)}, expected one of {node['enum']}", f"use one of {node['enum']}"))
    if isinstance(value, str):
        out += _apply_string(node, value, path)
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        out += _apply_number(node, value, path)
    if isinstance(value, list):
        out += _apply_array(node, value, path)
    if isinstance(value, Mapping):
        out += _apply_object(node, value, path)
    for branch in node.get("allOf", ()):
        out += _check(branch, value, path)
    if "if" in node and "then" in node and not _check(node["if"], value, path):
        out += _check(node["then"], value, path)
    return out


def _apply_string(node, value, path):
    out = []
    if "pattern" in node and not re.search(node["pattern"], value):
        out.append(_problem(path, "pattern", f"{value!r} does not match {node['pattern']}", f"match the pattern {node['pattern']}"))
    if len(value) < node.get("minLength", 0):
        out.append(_problem(path, "min-length", "string is too short", f"use at least {node['minLength']} characters"))
    return out


def _apply_number(node, value, path):
    out = []
    for key, bad, text in (("minimum", lambda: value < node["minimum"], ">="), ("maximum", lambda: value > node["maximum"], "<="), ("exclusiveMinimum", lambda: value <= node["exclusiveMinimum"], ">")):
        if key in node and bad():
            out.append(_problem(path, "range", f"{value} is out of range", f"use a number {text} {node[key]}"))
    return out


def _apply_array(node, value, path):
    out = []
    if len(value) < node.get("minItems", 0):
        out.append(_problem(path, "min-items", f"has {len(value)} items", f"provide at least {node['minItems']} items"))
    if "maxItems" in node and len(value) > node["maxItems"]:
        out.append(_problem(path, "max-items", f"has {len(value)} items", f"provide at most {node['maxItems']} items"))
    for index, item in enumerate(value):
        if "items" in node:
            out += _check(node["items"], item, f"{path}[{index}]")
    return out


def _apply_object(node, value, path):
    out = []
    properties = node.get("properties", {})
    for name in node.get("required", ()):
        if name not in value:
            out.append(_problem(path, "required", f"missing required property {name!r}", f"add \"{name}\""))
    for name, item in value.items():
        if name in properties:
            out += _check(properties[name], item, f"{path}.{name}")
        elif node.get("additionalProperties") is False:
            close = difflib.get_close_matches(name, properties, n=1)
            hint = f"did you mean {close[0]!r}? " if close else ""
            out.append(_problem(f"{path}.{name}", "unknown-property", f"unknown property {name!r}", f"{hint}allowed properties: {sorted(properties)}"))
    return out


def structural(document):
    """🧱 Validate `document` (a dict) against the JSON Schema only; return the problems."""
    return _check(SCHEMA, document, "$")


def _fill(node, value):
    node = _resolve(node)
    if isinstance(value, Mapping):
        out = dict(value)
        for name, child in node.get("properties", {}).items():
            child = _resolve(child)
            if name in out:
                out[name] = _fill(child, out[name])
            elif "default" in child:
                out[name] = child["default"]
            elif child.get("type") == "object" and "required" not in child:
                out[name] = _fill(child, {})
            elif "null" in (child.get("type") if isinstance(child.get("type"), list) else []):
                out[name] = None
        return out
    if isinstance(value, list) and "items" in node:
        return [_fill(node["items"], item) for item in value]
    return value


def _build(doc):
    def endpoint(raw):
        return None if raw is None else Endpoint(raw["entity"], int(raw["revision"]), raw["sampling"], raw["view"], raw["scale"], raw["pose"], raw["scenario"])

    def correspondence(raw):
        if raw is None:
            return None
        pairs = raw.get("pairs")
        return Correspondence(raw["kind"], None if pairs is None else tuple((a, b) for a, b in pairs), raw.get("name"))

    def transition(raw):
        t, p, f = raw["timing"], raw["placement"], raw["final"]
        timing = Timing(float(t["duration"]), t["easing"], float(t["lag"]), None if t["limit"] is None else float(t["limit"]))
        return Transition(raw["id"], raw["intent"], endpoint(raw["source"]), endpoint(raw["target"]), raw["relation"], raw["recipe"], timing, correspondence(raw.get("correspondence")), raw["copy"], Placement(p["space"], p["anchor"], p["fit"], p["uniform"]), Final(float(f["pathArc"]), float(f["dip"]), float(f["ghost"])))

    groups = tuple(Group(g["id"], tuple(g["transitions"]), g["mode"]) for g in doc.get("groups", ()))
    return Document(tuple(transition(t) for t in doc["transitions"]), groups, doc["version"])


def dump(document):
    """💾 Serialize a `Document` into its canonical JSON-ready dict (all fields explicit, camelCase); inverse of `parse`."""

    def endpoint(e):
        return None if e is None else {"entity": e.entity, "revision": e.revision, "sampling": e.sampling, "view": e.view, "scale": e.scale, "pose": e.pose, "scenario": e.scenario}

    def correspondence(c):
        out = {"kind": c.kind}
        if c.pairs is not None:
            out["pairs"] = [list(pair) for pair in c.pairs]
        if c.name is not None:
            out["name"] = c.name
        return out

    def transition(t):
        out = {
            "id": t.id,
            "intent": t.intent,
            "source": endpoint(t.source),
            "target": endpoint(t.target),
            "relation": t.relation,
            "recipe": t.recipe,
            "copy": t.copy,
            "timing": {"duration": t.timing.duration, "easing": t.timing.easing, "lag": t.timing.lag, "limit": t.timing.limit},
            "placement": {"space": t.placement.space, "anchor": t.placement.anchor, "fit": t.placement.fit, "uniform": t.placement.uniform},
            "final": {"pathArc": t.final.path_arc, "dip": t.final.dip, "ghost": t.final.ghost},
        }
        if t.correspondence is not None:
            out["correspondence"] = correspondence(t.correspondence)
        return out

    out = {"version": document.version, "transitions": [transition(t) for t in document.transitions]}
    if document.groups:
        out["groups"] = [{"id": g.id, "mode": g.mode, "transitions": list(g.transitions)} for g in document.groups]
    return out


def dumps(document):
    """🧬 Canonical file text of a `Document` or valid raw dict: defaults explicit, `$schema` of a raw dict kept first, 2-space JSON, UTF-8 characters kept, trailing newline. Persist, reload and render are byte-stable."""
    raw = document if isinstance(document, Mapping) else None
    out = dump(parse(raw) if raw is not None else document)
    if raw is not None and isinstance(raw.get("$schema"), str):
        out = {"$schema": raw["$schema"], **out}
    return json.dumps(out, indent=2, ensure_ascii=False) + "\n"


def _semantic(document):
    out, seen = [], {}
    for index, t in enumerate(document.transitions):
        base = f"$.transitions[{index}]"
        if t.id in seen:
            out.append(_problem(f"{base}.id", "duplicate-id", f"transition id {t.id!r} is already used at $.transitions[{seen[t.id]}]", "give every transition a unique stable id"))
        seen.setdefault(t.id, index)
        if t.correspondence and t.correspondence.kind == "identity" and t.source.entity != t.target.entity:
            out.append(_problem(f"{base}.correspondence.kind", "identity-entities", f"identity keeps one object but source {t.source.entity!r} differs from target {t.target.entity!r}", "use the same entity on both sides or choose key, order, pairs or recipe"))
        if t.timing.limit is not None and t.timing.limit < t.timing.duration:
            out.append(_problem(f"{base}.timing.limit", "limit-below-duration", f"limit {t.timing.limit} is below duration {t.timing.duration}", "raise limit to at least the duration or use null"))
        if t.timing.lag > 0 and t.relation == "one-to-one":
            out.append(_problem(f"{base}.timing.lag", "lag-not-applicable", "lag staggers several elements but relation is one-to-one", "set lag to 0 or use a one-to-many, many-to-one or many-to-many relation"))
        if t.timing.lag > 0 and t.timing.duration == 0:
            out.append(_problem(f"{base}.timing.lag", "lag-not-applicable", "lag has no effect on a zero-duration (atomic) transition", "set lag to 0 or give the transition a positive duration"))
        out += _placement(document, t, base)
    out += _cycles(document)
    out += _groups(document, seen)
    return out


def _endpoints(t):
    return [e for e in (t.source, t.target) if e is not None]


def _entities(document):
    return {e.entity for t in document.transitions for e in _endpoints(t)}


def _placement(document, t, base):
    space = t.placement.space
    if space == "scene":
        return []
    entity = space.removeprefix(PLACEMENT_PREFIX)
    path = f"{base}.placement.space"
    if entity not in _entities(document):
        return [_problem(path, "placement-unknown-entity", f"placement space {space!r} names entity {entity!r} which no transition declares", "use \"scene\" or an entity id used as a source or target in this document")]
    if entity in {e.entity for e in _endpoints(t)}:
        return [_problem(path, "placement-cycle", f"entity {entity!r} cannot be placed in its own coordinate space", "choose a different parent entity or \"scene\"")]
    return []


def _cycles(document):
    parents = {}
    for t in document.transitions:
        if t.placement.space != "scene" and t.target is not None:
            parents.setdefault(t.target.entity, set()).add(t.placement.space.removeprefix(PLACEMENT_PREFIX))
    state, out = {}, []

    def visit(entity, trail):
        state[entity] = 1
        for parent in sorted(parents.get(entity, ())):
            if state.get(parent) == 1:
                cycle = " -> ".join([*trail[trail.index(parent):], parent]) if parent in trail else f"{entity} -> {parent}"
                out.append(_problem("$.transitions", "placement-cycle", f"nested placement spaces form a cycle: {cycle}", "break the cycle so every entity nests inside an acyclic chain ending in \"scene\""))
            elif parent not in state:
                visit(parent, [*trail, parent])
        state[entity] = 2

    for entity in sorted(parents):
        if entity not in state:
            visit(entity, [entity])
    return out


def _groups(document, seen):
    out, groups = [], {}
    for index, g in enumerate(document.groups):
        base = f"$.groups[{index}]"
        if g.id in groups:
            out.append(_problem(f"{base}.id", "duplicate-group-id", f"group id {g.id!r} is already used at $.groups[{groups[g.id]}]", "give every group a unique id"))
        groups.setdefault(g.id, index)
        members = set()
        for position, member in enumerate(g.transitions):
            path = f"{base}.transitions[{position}]"
            if member not in seen:
                out.append(_problem(path, "unknown-transition", f"group {g.id!r} references unknown transition {member!r}", f"use one of the declared ids: {sorted(seen)}"))
            if member in members:
                out.append(_problem(path, "group-duplicate-member", f"transition {member!r} appears twice in group {g.id!r}", "list every transition once per group"))
            members.add(member)
    return out


def _bound_checks(document, stage, only):
    out = []
    for index, t in enumerate(document.transitions):
        if only is not None and t.id != only:
            continue
        base = f"$.transitions[{index}]"
        bound = {}
        for side, ep in (("source", t.source), ("target", t.target)):
            if ep is None:
                bound[side] = None
                continue
            item = stage.bound(ep.entity)
            bound[side] = item
            path = f"{base}.{side}"
            if item is None:
                out.append(_problem(f"{path}.entity", "unbound-entity", f"entity {ep.entity!r} is not bound on the stage", f"call stage.bind({ep.entity!r}, ...) before stage.transition({t.id!r})"))
                continue
            if item.revision != ep.revision:
                out.append(_problem(f"{path}.revision", "revision-mismatch", f"document asks revision {ep.revision} but the bound entity {ep.entity!r} is revision {item.revision}", "bind the matching revision or update the document"))
            for field in ("view", "scale", "pose", "scenario"):
                wanted, offered = getattr(ep, field), item.provides.get(field)
                if wanted is not None and offered is not None and wanted not in offered:
                    out.append(_problem(f"{path}.{field}", f"unsupported-{field}", f"entity {ep.entity!r} does not provide {field} {wanted!r}; it provides {sorted(offered)}", f"use a provided {field} or extend the entity's provides[{field!r}]"))
        out += _correspondence_checks(t, base, bound, stage)
    return out


WHOLE = ("hull", "retext", "fade", "move", "replace")


def uses_parts(t):
    """🧷 Whether a transition pairs parts of its endpoints (single source of truth for the full executor and validation)."""
    return t.recipe not in WHOLE or (t.relation == "many-to-many" and (t.recipe == "fade" or (t.recipe in ("move", "replace") and t.timing.lag > 0)))


def _correspondence_checks(t, base, bound, stage):
    c = t.correspondence
    if c is None:
        return []
    out = []
    path = f"{base}.correspondence"
    names = stage.correspondences
    if c.kind == "recipe" and names is not None and c.name not in names:
        out.append(_problem(f"{path}.name", "unknown-correspondence", f"correspondence recipe {c.name!r} is not registered", f"use one of {sorted(names)}"))
    source, target = bound["source"], bound["target"]
    if source is None or target is None or not uses_parts(t):
        return out
    have, want = source.part_names(), target.part_names()
    if t.relation == "one-to-one" and t.recipe in WHOLE_RECIPES:
        have = want = None
    if c.kind == "key" and have is not None and want is not None and set(have) != set(want):
        out.append(_problem(f"{path}.kind", "key-mismatch", f"source keys {sorted(set(have) - set(want))} and target keys {sorted(set(want) - set(have))} have no counterpart", "bind the same keys on both sides or declare explicit pairs"))
    if c.kind == "order" and have is not None and want is not None and len(have) != len(want):
        out.append(_problem(f"{path}.kind", "order-mismatch", f"source has {len(have)} parts but target has {len(want)}", "use equal counts or declare explicit pairs"))
    for position, (a, b) in enumerate(c.pairs or ()):
        for part, names_, side in ((a, have, "source"), (b, want, "target")):
            if names_ is not None and part not in names_:
                out.append(_problem(f"{path}.pairs[{position}]", "pair-unknown-part", f"{side} part {part!r} is not bound; known parts: {list(names_)}", "use part names of the bound entity (mapping keys, or indexes for lists)"))
    return out


def validate(document, stage=None, *, only=None):
    """🔍 Validate a `Document` or raw dict: schema, semantics, and (with `stage`) bound entities; return a list of `Problem`.

    `only` limits the stage-bound checks to one transition id. Stage duck type: `bound(entity_id)` returning an object with `revision`, `provides`, `part_names()` or None, and `correspondences` (frozenset or None).
    """
    raw = dump(document) if isinstance(document, Document) else document
    problems = structural(raw)
    if problems:
        return problems
    parsed = _build(_fill(SCHEMA, raw))
    problems = _semantic(parsed)
    if only is not None and only not in {t.id for t in parsed.transitions}:
        problems.append(_problem("$.transitions", "unknown-transition", f"transition {only!r} is not declared", f"use one of the declared ids: {sorted(t.id for t in parsed.transitions)}"))
    return problems + (_bound_checks(parsed, stage, only) if stage is not None else [])


def parse(raw):
    """🧩 Parse a raw dict into a `Document` with every default made explicit; raise `SpecError` when invalid."""
    problems = validate(raw)
    if problems:
        raise SpecError(problems)
    return _build(_fill(SCHEMA, raw))


def _reject_constant(name):
    raise ValueError(f"{name} is not a finite number")


def load(path):
    """📂 Read and parse a transition document file (UTF-8 JSON); raise `SpecError` with a `Problem` for unreadable, malformed or invalid files."""
    file = Path(path)
    try:
        raw = json.loads(file.read_text(encoding="utf-8"), parse_constant=_reject_constant)
    except OSError as error:
        raise SpecError([_problem("$", "file", f"cannot read {file}: {error.strerror or error}", "check the path of the clip's 🔀️transitions/🔣️.json")]) from error
    except ValueError as error:
        raise SpecError([_problem("$", "json", f"{file} is not valid JSON: {error}", "fix the JSON syntax and use only finite numbers")]) from error
    return parse(raw)
