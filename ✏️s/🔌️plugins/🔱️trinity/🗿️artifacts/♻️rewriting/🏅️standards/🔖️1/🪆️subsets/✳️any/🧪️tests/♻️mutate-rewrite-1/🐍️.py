"""🧪️ Independent typed Rewriting document and full declared Semio child oracle."""
import copy
import json
import re
import struct
from semio_repo_test import Adapter, Outcome

MEMBERS = ("workingGraph", "lhs", "rhs", "parameterBindings", "ruleLayout")
KINDS = ("edit-before-fixture", "edit-lhs", "edit-rhs", "change-parameter-binding", "remove-parameter-binding", "change-rule-layout-point", "remove-rule-layout-point", "drag-rule-nodes", "set-rule-layout-points")
TAGS = dict(zip(KINDS, ("editBeforeFixture", "editLhs", "editRhs", "changeParameterBinding", "removeParameterBinding", "changeRuleLayoutPoint", "removeRuleLayoutPoint", "dragRuleNodes", "setRuleLayoutPoints")))
DOCUMENTS = {"edit-before-fixture": ("workingGraph", "newWorkingGraph"), "edit-lhs": ("lhs", "newLhs"), "edit-rhs": ("rhs", "newRhs")}
RULE_ROWS = ("create", "merge", "set", "delete", "parameters")

def exact(value, required, optional=()):
    if not isinstance(value, dict) or not set(required).issubset(value) or set(value) - set(required) - set(optional):
        raise AssertionError("typed fields required %r optional %r, found %r" % (required, optional, value))

def word(value):
    if isinstance(value, dict):
        exact(value, ("bits",))
        if not isinstance(value["bits"], str) or re.fullmatch("[0-9a-f]{16}", value["bits"]) is None:
            raise AssertionError("binary64 requires the exact lowercase word")
        return copy.deepcopy(value)
    if type(value) not in (int, float):
        raise AssertionError("declared numeric input or binary64 word required")
    return {"bits": struct.pack(">d", float(value)).hex()}

def decimal(value):
    return struct.unpack(">d", bytes.fromhex(word(value)["bits"]))[0]

def intrinsic(value):
    pending = [value]
    while pending:
        held = pending.pop()
        kind = held.get("kind") if isinstance(held, dict) else None
        if kind == "null":
            exact(held, ("kind",))
        elif kind == "bool":
            exact(held, ("kind", "value"))
            if type(held["value"]) is not bool:
                raise AssertionError("intrinsic bool required")
        elif kind in ("int", "float"):
            exact(held, ("kind", "lexeme"))
            if not isinstance(held["lexeme"], str):
                raise AssertionError("intrinsic numeric lexeme required")
        elif kind == "str":
            exact(held, ("kind", "value"))
            if not isinstance(held["value"], str):
                raise AssertionError("intrinsic literal string required")
        elif kind == "bytes":
            exact(held, ("kind", "value"))
            if not isinstance(held["value"], list) or any(type(x) is not int or not 0 <= x <= 255 for x in held["value"]):
                raise AssertionError("intrinsic octets required")
        elif kind == "ref":
            exact(held, ("kind", "id")); exact(held["id"], ("value",))
            if not isinstance(held["id"]["value"], str):
                raise AssertionError("literal intrinsic reference required")
        elif kind == "list":
            exact(held, ("kind", "items"))
            if not isinstance(held["items"], list):
                raise AssertionError("ordered intrinsic list required")
            pending.extend(held["items"])
        elif kind == "map":
            exact(held, ("kind", "entries"))
            if not isinstance(held["entries"], list):
                raise AssertionError("ordered intrinsic map occurrences required")
            for entry in held["entries"]:
                exact(entry, ("key", "value"))
                if not isinstance(entry["key"], str):
                    raise AssertionError("literal member key required")
                pending.append(entry["value"])
        else:
            raise AssertionError("unknown intrinsic variant %r" % kind)

def properties(entries):
    if not isinstance(entries, list):
        raise AssertionError("ordered property occurrences required")
    for entry in entries:
        exact(entry, ("key", "value"))
        if not isinstance(entry["key"], str):
            raise AssertionError("literal property key required")
        intrinsic(entry["value"])

def validate_child(child):
    exact(child, ("schema", "nodes", "edges"))
    if child["schema"] != "s.stdio.semio.graph" or not isinstance(child["nodes"], list) or not isinstance(child["edges"], list):
        raise AssertionError("full declared Semio child required")
    for node in child["nodes"]:
        exact(node, ("id", "kind", "label", "position", "width", "height", "ports", "properties"))
        exact(node["id"], ("value",)); exact(node["position"], ("x", "y"))
        if not all(isinstance(node[k], str) for k in ("kind", "label")) or not isinstance(node["id"]["value"], str):
            raise AssertionError("literal node fields required")
        for axis in (node["position"]["x"], node["position"]["y"], node["width"], node["height"]):
            if word(axis) != axis:
                raise AssertionError("retained child geometry uses raw words")
        properties(node["properties"])
        for port in node["ports"]:
            exact(port, ("name", "kind", "category", "properties"))
            properties(port["properties"])
    for edge in child["edges"]:
        exact(edge, ("id", "source", "target", "kind", "label", "properties"), ("sourcePort", "targetPort"))
        exact(edge["id"], ("value",)); exact(edge["source"], ("value",)); exact(edge["target"], ("value",))
        properties(edge["properties"])

def validate_owner(owner):
    parent, child = owner
    exact(parent, MEMBERS)
    graph = parent["workingGraph"]
    exact(graph, ("schema", "name", "manifest", "camera", "content", "query"), ("manifestId", "rootNodeId"))
    exact(graph["camera"], ("x", "y", "zoom"))
    for axis in graph["camera"].values():
        word(axis)
    exact(graph["content"], ("childId", "target")); exact(graph["content"]["target"], ("artifactId", "dialect"))
    if graph["content"]["target"]["dialect"] != {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "graph"}:
        raise AssertionError("workingGraph requires its exact Semio child dialect")
    exact(parent["lhs"], ("pattern",), ("whereClause",))
    exact(parent["rhs"], RULE_ROWS)
    if not isinstance(parent["parameterBindings"], dict) or not isinstance(parent["ruleLayout"], dict):
        raise AssertionError("typed bindings and layout required")
    for point in parent["ruleLayout"].values():
        exact(point, ("x", "y")); word(point["x"]); word(point["y"])
    validate_child(child)

def fixture(ctx, role):
    prefix = role + " shared://"
    hits = [step["text"][len(role) + 1:] for step in ctx.scenario["steps"] if step["text"].startswith(prefix)]
    if len(hits) != 1 or not hits[0].startswith("shared://"):
        raise AssertionError("scenario %s must declare exactly one %s input" % (ctx.scenario["id"], role))
    return json.loads(ctx.fixture_bytes(hits[0]).decode("utf-8"))

def owner_input(ctx, parent_role, child_role):
    owner = (fixture(ctx, parent_role), fixture(ctx, child_role))
    validate_owner(owner)
    return owner

def payload(ctx):
    rows = [step["docString"] for step in ctx.scenario["steps"] if "docString" in step]
    if len(rows) != 1:
        raise AssertionError("exactly one declared typed payload required")
    return json.loads(rows[0])

def kind_of(mutation):
    for kind, tag in TAGS.items():
        if mutation.get("mutation") == tag:
            return kind
    raise AssertionError("unknown typed mutation")

def rule_graph_slots(parent):
    slots = {"lhs-match": (0.0, 0.0)}
    if (parent["lhs"].get("whereClause") or "").strip():
        slots["lhs-where"] = (220.0, 80.0)
    clauses = {}
    for row, name in enumerate(RULE_ROWS):
        prefix = "rhs-parameter" if name == "parameters" else "rhs-" + name
        for index in range(len(parent["rhs"][name])):
            clauses["%s-%d" % (prefix, index)] = (index * 220.0, row * 80.0)
    slots.update(clauses or {"rhs-empty": (0.0, 0.0)})
    return slots

def rule_position(parent, key):
    slots = rule_graph_slots(parent)
    if key not in slots:
        return None
    point = parent["ruleLayout"].get(key)
    return (decimal(point["x"]), decimal(point["y"])) if point is not None else slots[key]

def apply_mutation(owner, mutation, replacement=None):
    kind = kind_of(mutation)
    parent, child = copy.deepcopy(owner)
    if kind in DOCUMENTS:
        field, argument = DOCUMENTS[kind]
        parent[field] = copy.deepcopy(mutation[argument])
        if kind == "edit-before-fixture":
            if replacement is None:
                raise AssertionError("edit-before-fixture requires its declared full replacement child")
            child = copy.deepcopy(replacement)
    elif kind.endswith("parameter-binding"):
        if kind == "change-parameter-binding":
            parent["parameterBindings"][mutation["key"]] = copy.deepcopy(mutation["newValue"])
        else:
            parent["parameterBindings"].pop(mutation["key"], None)
    elif kind == "drag-rule-nodes":
        for key in mutation["targets"]:
            at = rule_position(parent, key)
            if at is not None:
                parent["ruleLayout"][key] = {"x": word(at[0] + decimal(mutation["dx"])), "y": word(at[1] + decimal(mutation["dy"]))}
    elif kind == "set-rule-layout-points":
        for point in mutation["points"]:
            parent["ruleLayout"][point["key"]] = {"x": word(point["x"]), "y": word(point["y"])}
        for key in mutation["cleared"]:
            parent["ruleLayout"].pop(key, None)
    elif kind == "change-rule-layout-point":
        parent["ruleLayout"][mutation["key"]] = {"x": word(mutation["newPoint"]["x"]), "y": word(mutation["newPoint"]["y"])}
    elif kind == "remove-rule-layout-point":
        parent["ruleLayout"].pop(mutation["key"], None)
    else:
        raise AssertionError("unhandled declared mutation")
    result = (parent, child)
    validate_owner(result)
    return result

def inverse_mutation(owner, mutation):
    parent, child = owner
    kind = kind_of(mutation)
    if kind == "edit-before-fixture":
        return ({"mutation": TAGS["edit-before-fixture"], "newWorkingGraph": copy.deepcopy(parent["workingGraph"])}, copy.deepcopy(child))
    if kind in DOCUMENTS:
        field, argument = DOCUMENTS[kind]
        return ({"mutation": TAGS[kind], argument: copy.deepcopy(parent[field])}, None)
    if kind in ("drag-rule-nodes", "set-rule-layout-points"):
        keys = [key for key in mutation["targets"] if rule_position(parent, key) is not None] if kind == "drag-rule-nodes" else [point["key"] for point in mutation["points"]] + mutation["cleared"]
        layout = parent["ruleLayout"]
        return ({"mutation": TAGS["set-rule-layout-points"], "points": [{"key": key, **copy.deepcopy(layout[key])} for key in keys if key in layout], "cleared": [key for key in keys if key not in layout]}, None)
    field = "parameterBindings" if kind.endswith("parameter-binding") else "ruleLayout"
    change, remove = ("change-parameter-binding", "remove-parameter-binding") if field == "parameterBindings" else ("change-rule-layout-point", "remove-rule-layout-point")
    key = mutation["key"]
    if key not in parent[field]:
        return ({"mutation": TAGS[remove], "key": key}, None)
    return ({"mutation": TAGS[change], "key": key, "newValue" if field == "parameterBindings" else "newPoint": copy.deepcopy(parent[field][key])}, None)

def projection(owner):
    return {"snapshot": owner[0], "child": owner[1]}

def first_difference(actual, expected):
    pending = [("owner", actual, expected)]
    while pending:
        path, got, wanted = pending.pop()
        if got == wanted:
            continue
        if isinstance(got, dict) and isinstance(wanted, dict) and got.keys() == wanted.keys():
            pending.extend((path + "." + key, got[key], wanted[key]) for key in reversed(got))
        elif isinstance(got, list) and isinstance(wanted, list) and len(got) == len(wanted):
            pending.extend((path + "[%d]" % index, got[index], wanted[index]) for index in range(len(got) - 1, -1, -1))
        else:
            return "%s: actual=%r expected=%r" % (path, got, wanted)
    return "equal"

def touches_one(scenario, kind, before, after):
    expected = DOCUMENTS[kind][0] if kind in DOCUMENTS else "parameterBindings" if kind.endswith("parameter-binding") else "ruleLayout"
    moved = [field for field in MEMBERS if before[0][field] != after[0][field] or field == "workingGraph" and before[1] != after[1]]
    if moved != [expected]:
        raise AssertionError("%s writes exactly %s, found %r" % (scenario, expected, moved))

def restores(kind, restored, original):
    if projection(restored) != projection(original):
        raise AssertionError("inverse-%s must restore the complete declared parent and child" % kind)

def outcome_of(value):
    return Outcome(value, raw=json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

def mutate_handler(kind, inverse=False):
    def handler(ctx):
        before = owner_input(ctx, "the real derived rule", "the real derived child")
        mutation = payload(ctx)
        if kind_of(mutation) != kind:
            raise AssertionError("scenario payload discriminator differs from its exact declared kind")
        replacement = fixture(ctx, "the replacement working child") if kind == "edit-before-fixture" else None
        applied = apply_mutation(before, mutation, replacement)
        if projection(applied) == projection(before):
            raise AssertionError("forward operation must move its declared member")
        touches_one(ctx.scenario["id"], kind, before, applied)
        if inverse:
            back, retained = inverse_mutation(before, mutation)
            restored = apply_mutation(applied, back, retained)
            restores(kind, restored, before)
            return outcome_of({"mutated": projection(applied), "restored": projection(restored)})
        return outcome_of(projection(applied))
    return handler

def spec_vector_handler(kind):
    def handler(ctx):
        before = owner_input(ctx, "the committed before-rule", "the committed before-child")
        mutation = fixture(ctx, "the committed mutation")
        if kind_of(mutation) != kind:
            raise AssertionError("committed discriminator differs from the declared scenario")
        replacement = fixture(ctx, "the committed after-child") if kind == "edit-before-fixture" else None
        applied = apply_mutation(before, mutation, replacement)
        after = owner_input(ctx, "the committed after-rule", "the committed after-child")
        if projection(applied) != projection(after):
            raise AssertionError("spec-vector-%s must equal the complete committed parent and child: %s" % (kind, first_difference(projection(applied), projection(after))))
        if projection(applied) == projection(before):
            raise AssertionError("committed forward operation must move its declared member")
        touches_one(ctx.scenario["id"], kind, before, applied)
        back, retained = inverse_mutation(before, mutation)
        restores(kind, apply_mutation(applied, back, retained), before)
        return outcome_of(projection(applied))
    return handler

def identity_handler(ctx):
    ground = owner_input(ctx, "the two-node ground-floor rule this case used to rest on", "the two-node ground-floor child")
    tower = owner_input(ctx, "the real derived rule", "the real derived child")
    for view, expected in ((ground, (2, 1, 2)), (tower, (180, 179, 364))):
        parent, child = view
        facts = (len(child["nodes"]), len(child["edges"]), sum(len(node["ports"]) for node in child["nodes"]))
        if facts != expected:
            raise AssertionError("committed full Nakagin facts differ: %r != %r" % (facts, expected))
        if "whereClause" not in parent["lhs"]:
            raise AssertionError("the actual pattern retains its declared where clause")
        names = {item["name"] for item in parent["rhs"]["parameters"]}
        if any(name not in names for name in parent["parameterBindings"]):
            raise AssertionError("a binding must retain its declared RHS parameter")
        if json.loads(json.dumps(projection(view))) != projection(view):
            raise AssertionError("complete declared JSON serialization moves the owner")
    if ground[0]["workingGraph"]["rootNodeId"] != tower[0]["workingGraph"]["rootNodeId"]:
        raise AssertionError("both authored owners retain the same actual root piece")
    return outcome_of({"groundFloor": projection(ground), "capsuleTower": projection(tower)})

def adapter():
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-" + kind, mutate_handler(kind))
        built = built.oracle("inverse-" + kind, mutate_handler(kind, True))
        built = built.oracle("spec-vector-" + kind, spec_vector_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)
