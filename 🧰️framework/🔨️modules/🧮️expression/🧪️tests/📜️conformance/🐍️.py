"""Third-party reproduction of the expression fixtures in `🧫️fixtures/*` with Python's own `ast`, `math`, `decimal`, `networkx` and `jsonschema`.

Every fixture case carries a Python rendering of its expression. The oracle parses that rendering with `ast.parse`, checks dimensions, evaluates it lazily
in SI base units and resolves parameter sets with `networkx` strongly connected components and topological generations; the result must equal the
committed expectation that the Rust implementation reproduces from the Semio syntax.

Run from the repo root: `.venv/Scripts/python.exe -m pytest -p no:cacheprovider "<this file>"`.
"""
import ast
import json
import math
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

import networkx as nx
import pytest
from jsonschema import Draft7Validator
from referencing import Registry, Resource

HERE = Path(__file__).resolve().parent
MODULE = HERE.parent.parent
FIXTURES = MODULE / "🧫️fixtures"
TOLERANCE = 1e-9
NUMERIC = ("number", "length", "angle", "area", "volume")
LENGTH = {"mm": (1, 1000), "cm": (1, 100), "m": (1, 1), "km": (1000, 1), "in": (254, 10000), "ft": (3048, 10000)}
AREA = {"mm2": (1, 10**6), "cm2": (1, 10**4), "m2": (1, 1)}
VOLUME = {"mm3": (1, 10**9), "cm3": (1, 10**6), "m3": (1, 1), "l": (1, 1000)}
ARITY = {"sqrt": 1, "abs": 1, "round": 1, "floor": 1, "ceil": 1, "sin": 1, "cos": 1, "tan": 1, "atan2": 2}
BINARY = {ast.Add: "add", ast.Sub: "subtract", ast.Mult: "multiply", ast.Div: "divide", ast.Pow: "power"}
COMPARE = {ast.Eq: "equal", ast.NotEq: "not-equal", ast.Lt: "less", ast.LtE: "less-equal", ast.Gt: "greater", ast.GtE: "greater-equal"}
SYMBOL = {"add": "+", "subtract": "-", "multiply": "*", "divide": "/", "power": "^", "min": "min", "max": "max", "equal": "=", "not-equal": "!=", "less": "<", "less-equal": "<=", "greater": ">", "greater-equal": ">="}
EXPONENT_KINDS = {("length", 1.0): "length", ("length", 2.0): "area", ("length", 3.0): "volume", ("area", 1.0): "area", ("area", 0.5): "length", ("volume", 1.0): "volume", ("angle", 1.0): "angle"}


class Failure(Exception):
    def __init__(self, code, path, **detail):
        super().__init__(code)
        self.code = code
        self.path = list(path)
        self.detail = detail

    def as_json(self):
        return {"code": self.code, "path": self.path}


def to_si(kind, value, unit):
    if kind == "angle":
        return value * math.pi / 180.0 if unit == "deg" else value
    num, den = {"length": LENGTH, "area": AREA, "volume": VOLUME}[kind][unit]
    return value * num / den


def finite(value, path):
    if not math.isfinite(value):
        raise Failure("overflow", path)
    return value


def approx_eq(x, y):
    return x == y or abs(x - y) <= TOLERANCE * max(1.0, abs(x), abs(y))


def round_half_away(x):
    if abs(x) >= 2.0**52:
        return x
    return float(Decimal(x).quantize(Decimal(1), rounding=ROUND_HALF_UP))


def binary_kind(op, left, right, exponent):
    if op in ("add", "subtract", "min", "max"):
        return left if left == right else None
    if op == "multiply":
        if right == "number":
            return left
        if left == "number":
            return right
        return {("length", "length"): "area", ("length", "area"): "volume", ("area", "length"): "volume"}.get((left, right))
    if op == "divide":
        if right == "number":
            return left
        if left == right:
            return "number"
        return {("area", "length"): "length", ("volume", "length"): "area", ("volume", "area"): "length"}.get((left, right))
    if left == "number" and right == "number":
        return "number"
    return None if exponent is None else EXPONENT_KINDS.get((left, exponent))


def literal_exponent(node):
    if isinstance(node, ast.Constant) and isinstance(node.value, (int, float)) and not isinstance(node.value, bool):
        return float(node.value)
    if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub) and isinstance(node.operand, ast.Constant):
        return -float(node.operand.value)
    return None


def call_kind(name, kinds, path):
    def operand(position, expected):
        return Failure("operand-kind", path, position=position, expected=expected, found=kinds[position])

    if name == "sqrt":
        if kinds[0] in ("number", "area"):
            return "length" if kinds[0] == "area" else "number"
        raise operand(0, ["number", "area"])
    if name == "abs":
        if kinds[0] in NUMERIC:
            return kinds[0]
        raise operand(0, list(NUMERIC))
    if name in ("round", "floor", "ceil"):
        if kinds[0] == "number":
            return "number"
        raise operand(0, ["number"])
    if name in ("sin", "cos", "tan"):
        if kinds[0] == "angle":
            return "number"
        raise operand(0, ["angle"])
    for position in (0, 1):
        if kinds[position] not in NUMERIC:
            raise operand(position, list(NUMERIC))
    if kinds[0] != kinds[1]:
        raise Failure("mixed-kinds", path)
    return "angle"


class Interpreter:
    """Dimension-checks and evaluates the Python rendering of one expression; children are addressed like the Semio tree (binary folds of `and`, `or`, `min`, `max`)."""

    def __init__(self, env):
        self.env = env

    def kinds(self, node, path=()):
        path = list(path)
        sub = lambda index, child: self.kinds(child, path + [index])
        if isinstance(node, ast.Constant):
            return {bool: "bool", str: "text"}.get(type(node.value), "number")
        if isinstance(node, ast.Name):
            return self.lookup(node.id, path)[0]
        if isinstance(node, ast.Call):
            return self.call_kinds(node, path)
        if isinstance(node, ast.UnaryOp):
            found = sub(0, node.operand)
            if isinstance(node.op, ast.Not):
                if found != "bool":
                    raise Failure("operand-kind", path, position=0)
                return "bool"
            if found not in NUMERIC:
                raise Failure("operand-kind", path, position=0)
            return found
        if isinstance(node, ast.BinOp):
            op = BINARY[type(node.op)]
            left, right = sub(0, node.left), sub(1, node.right)
            return self.arithmetic_kind(op, left, right, node.right, path)
        if isinstance(node, ast.Compare):
            op = COMPARE[type(node.ops[0])]
            left, right = sub(0, node.left), sub(1, node.comparators[0])
            if op not in ("equal", "not-equal"):
                for position, found in enumerate((left, right)):
                    if found not in NUMERIC:
                        raise Failure("operand-kind", path, position=position)
            if left != right:
                raise Failure("mixed-kinds", path)
            return "bool"
        if isinstance(node, ast.BoolOp):
            return self.fold(node.values, path, lambda p, child: self.kinds(child, p), lambda p, l, r, _: self.logic_kind(p, l, r))
        if isinstance(node, ast.IfExp):
            condition, then, otherwise = sub(0, node.test), sub(1, node.body), sub(2, node.orelse)
            if condition != "bool":
                raise Failure("operand-kind", path, position=0)
            if then != otherwise:
                raise Failure("branch-kinds", path)
            return then
        raise ValueError(ast.dump(node))

    def logic_kind(self, path, left, right):
        for position, found in enumerate((left, right)):
            if found != "bool":
                raise Failure("operand-kind", path, position=position)
        return "bool"

    def arithmetic_kind(self, op, left, right, right_node, path):
        for position, found in enumerate((left, right)):
            if found not in NUMERIC:
                raise Failure("operand-kind", path, position=position)
        if op == "power":
            if right != "number":
                raise Failure("operand-kind", path, position=1)
            result = binary_kind(op, left, right, literal_exponent(right_node))
            if result is None:
                raise Failure("exponent", path)
            return result
        result = binary_kind(op, left, right, None)
        if result is None:
            raise Failure("mixed-kinds", path)
        return result

    def call_kinds(self, node, path):
        name = node.func.id
        if name == "Q":
            return {**{u: "length" for u in LENGTH}, **{u: "angle" for u in ("deg", "rad")}, **{u: "area" for u in AREA}, **{u: "volume" for u in VOLUME}}[node.args[1].value]
        if name == "P":
            return self.lookup(node.args[0].value, path)[0]
        if name in ("min", "max"):
            return self.fold(node.args, path, lambda p, child: self.kinds(child, p), lambda p, l, r, rn: self.arithmetic_kind(name, l, r, rn, p))
        kinds = [self.kinds(arg, path + [i]) for i, arg in enumerate(node.args)]
        if len(kinds) != ARITY[name]:
            raise Failure("arity", path)
        return call_kind(name, kinds, path)

    @staticmethod
    def fold(items, path, leaf, combine):
        def go(upto, prefix):
            if upto == 0:
                return leaf(prefix, items[0])
            left = go(upto - 1, prefix + [0])
            right = leaf(prefix + [1], items[upto])
            return combine(prefix, left, right, items[upto])

        return go(len(items) - 1, path)

    def lookup(self, name, path):
        if name not in self.env:
            raise Failure("unknown-parameter", path, name=name)
        return self.env[name]

    def value(self, node, path=()):
        path = list(path)
        sub = lambda index, child: self.value(child, path + [index])
        if isinstance(node, ast.Constant):
            if isinstance(node.value, bool):
                return ("bool", node.value)
            if isinstance(node.value, str):
                return ("text", node.value)
            return ("number", finite(float(node.value), path))
        if isinstance(node, ast.Name):
            return self.lookup(node.id, path)
        if isinstance(node, ast.Call):
            return self.call_value(node, path)
        if isinstance(node, ast.UnaryOp):
            kind, v = sub(0, node.operand)
            if isinstance(node.op, ast.Not):
                return ("bool", not v)
            return (kind, finite(-v, path))
        if isinstance(node, ast.BinOp):
            op = BINARY[type(node.op)]
            left, right = sub(0, node.left), sub(1, node.right)
            return self.arithmetic_value(op, left, right, node.right, path)
        if isinstance(node, ast.Compare):
            op = COMPARE[type(node.ops[0])]
            (lk, x), (_, y) = sub(0, node.left), sub(1, node.comparators[0])
            if lk in ("bool", "text"):
                return ("bool", (x == y) == (op == "equal"))
            same = approx_eq(x, y)
            result = {"equal": same, "not-equal": not same, "less": x < y and not same, "less-equal": x < y or same, "greater": x > y and not same, "greater-equal": x > y or same}[op]
            return ("bool", result)
        if isinstance(node, ast.BoolOp):
            return self.fold_bool_values(node, path)
        if isinstance(node, ast.IfExp):
            return sub(1, node.body) if sub(0, node.test)[1] else sub(2, node.orelse)
        raise ValueError(ast.dump(node))

    def fold_bool_values(self, node, path):
        is_and = isinstance(node.op, ast.And)

        def go(upto, prefix):
            if upto == 0:
                return self.value(node.values[0], prefix)
            left = go(upto - 1, prefix + [0])
            if bool(left[1]) != is_and:
                return ("bool", bool(left[1]))
            return ("bool", bool(self.value(node.values[upto], prefix + [1])[1]))

        return go(len(node.values) - 1, path)

    def arithmetic_value(self, op, left, right, right_node, path):
        (lk, x), (rk, y) = left, right
        kind = binary_kind(op, lk, rk, literal_exponent(right_node))
        if op == "add":
            raw = x + y
        elif op == "subtract":
            raw = x - y
        elif op == "multiply":
            raw = x * y
        elif op == "divide":
            if y == 0.0:
                raise Failure("division-by-zero", path)
            raw = x / y
        elif op == "power":
            if x == 0.0 and y < 0.0:
                raise Failure("division-by-zero", path)
            try:
                raw = math.pow(x, y)
            except ValueError:
                raise Failure("domain", path)
            except OverflowError:
                raise Failure("overflow", path)
        else:
            raw = min(x, y) if op == "min" else max(x, y)
        return (kind, finite(raw, path))

    def call_value(self, node, path):
        name = node.func.id
        if name == "Q":
            kind = self.call_kinds(node, path)
            return (kind, finite(to_si(kind, float(node.args[0].value), node.args[1].value), path))
        if name == "P":
            return self.lookup(node.args[0].value, path)
        if name in ("min", "max"):
            return self.fold(node.args, path, lambda p, child: self.value(child, p), lambda p, l, r, rn: self.arithmetic_value(name, l, r, rn, p))
        args = [self.value(arg, path + [i]) for i, arg in enumerate(node.args)]
        kind, x = args[0]
        if name == "sqrt":
            if x < 0.0:
                raise Failure("domain", path)
            return ("length" if kind == "area" else "number", finite(math.sqrt(x), path))
        if name == "abs":
            return (kind, abs(x))
        if name == "round":
            return (kind, round_half_away(x))
        if name == "floor":
            return (kind, float(math.floor(x)))
        if name == "ceil":
            return (kind, float(math.ceil(x)))
        if name in ("sin", "cos", "tan"):
            return ("number", finite(getattr(math, name)(x), path))
        return ("angle", finite(math.atan2(x, args[1][1]), path))


def env_of(rows):
    return {name: (row["kind"], row["value"]) for name, row in rows.items()}


def parse_python(text):
    return ast.parse(text, mode="eval").body


def run(text, rows):
    """Evaluates a Python rendering against `{name: {kind, value}}` and returns `{kind, value}` or `{error: {code, path}}`."""
    interpreter = Interpreter(env_of(rows))
    try:
        tree = parse_python(text)
        interpreter.kinds(tree)
        kind, value = interpreter.value(tree)
    except Failure as failure:
        return {"error": failure.as_json()}
    return {"kind": kind, "value": value}


def kind_only(text, kinds):
    try:
        return {"kind": Interpreter({name: (kind, None) for name, kind in kinds.items()}).kinds(parse_python(text))}
    except Failure as failure:
        return {"error": failure.as_json()}


def dependencies(text):
    names = set()
    for node in ast.walk(parse_python(text)):
        if isinstance(node, ast.Name) and isinstance(node.ctx, ast.Load) and node.id not in ARITY and node.id not in ("min", "max", "Q", "P"):
            names.add(node.id)
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "P":
            names.add(node.args[0].value)
    return names


def resolve(params, overrides, declared):
    """Resolves a parameter set: `params`/`overrides` map names to Python renderings; returns order, values and errors like the Rust `evaluate_all`."""
    effective = dict(params)
    errors = {}
    for name, text in overrides.items():
        if name in effective:
            effective[name] = text
        else:
            errors[name] = {"code": "unknown-override"}
    graph = nx.DiGraph()
    graph.add_nodes_from(effective)
    for name, text in effective.items():
        for dependency in dependencies(text):
            if dependency in effective:
                graph.add_edge(dependency, name)
    cyclic = set()
    cycles = []
    for component in nx.strongly_connected_components(graph):
        member = next(iter(component))
        if len(component) > 1 or graph.has_edge(member, member):
            cyclic |= component
            cycles.append(sorted(component))
    blocked = set()
    for member in cyclic:
        blocked |= nx.descendants(graph, member)
    blocked -= cyclic
    clean = graph.subgraph(set(effective) - cyclic - blocked)
    order = [name for generation in nx.topological_generations(clean) for name in sorted(generation)]
    values = {}
    failing = cyclic | blocked
    for name in order:
        failed = sorted(d for d in dependencies(effective[name]) if d in effective and (d in errors or d in failing))
        if failed:
            errors[name] = {"code": "failed-dependency", "name": failed[0]}
            continue
        result = run(effective[name], {n: {"kind": k, "value": v} for n, (k, v) in values.items()})
        if "error" in result:
            errors[name] = result["error"]
        elif name in declared and declared[name] != result["kind"]:
            errors[name] = {"code": "declared-kind", "declared": declared[name], "found": result["kind"]}
        else:
            values[name] = (result["kind"], result["value"])
    for members in cycles:
        for member in members:
            errors[member] = {"code": "cycle", "members": members}
    for name in sorted(blocked):
        failed = sorted(d for d in dependencies(effective[name]) if d in effective and (d in errors or d in failing))
        errors[name] = {"code": "failed-dependency", "name": failed[0]}
    tail = sorted(cyclic | blocked)
    return {"order": order + tail, "values": {n: {"kind": k, "value": v} for n, (k, v) in values.items()}, "errors": errors}


def to_tree(node):
    """Converts the Python syntax tree of a rendering back into the Semio JSON tree (binary folds), the precedence check of the syntax fixtures."""
    if isinstance(node, ast.Constant):
        if isinstance(node.value, bool):
            return {"op": "bool", "value": node.value}
        if isinstance(node.value, str):
            return {"op": "text", "value": node.value}
        return {"op": "number", "value": float(node.value)}
    if isinstance(node, ast.Name):
        return {"op": "param", "name": node.id}
    if isinstance(node, ast.UnaryOp):
        return {"op": "unary", "operator": "not" if isinstance(node.op, ast.Not) else "negate", "operand": to_tree(node.operand)}
    if isinstance(node, ast.BinOp):
        return {"op": "binary", "operator": BINARY[type(node.op)], "left": to_tree(node.left), "right": to_tree(node.right)}
    if isinstance(node, ast.Compare):
        return {"op": "compare", "operator": COMPARE[type(node.ops[0])], "left": to_tree(node.left), "right": to_tree(node.comparators[0])}
    if isinstance(node, ast.BoolOp):
        result = to_tree(node.values[0])
        for value in node.values[1:]:
            result = {"op": "and" if isinstance(node.op, ast.And) else "or", "left": result, "right": to_tree(value)}
        return result
    if isinstance(node, ast.IfExp):
        return {"op": "if", "condition": to_tree(node.test), "then": to_tree(node.body), "otherwise": to_tree(node.orelse)}
    name = node.func.id
    if name == "Q":
        value, unit = node.args[0].value, node.args[1].value
        kind = "angle" if unit in ("deg", "rad") else "area" if unit in AREA else "volume" if unit in VOLUME else "length"
        return {"op": kind, "value": float(value), "unit": unit}
    if name == "P":
        return {"op": "param", "name": node.args[0].value}
    args = [to_tree(a) for a in node.args]
    if name in ("min", "max"):
        result = args[0]
        for arg in args[1:]:
            result = {"op": "binary", "operator": name, "left": result, "right": arg}
        return result
    return {"op": "call", "function": name, "arguments": args}


def load(domain):
    return json.loads((FIXTURES / domain / "🔣️.json").read_text(encoding="utf-8"))


def registry():
    resources = []
    for schema_file in MODULE.glob("*/🧬️schema/🔣️.json"):
        document = json.loads(schema_file.read_text(encoding="utf-8"))
        resources.append((document["$id"], Resource.from_contents(document)))
    return Registry().with_resources(resources)


def validator(domain):
    document = json.loads((MODULE / domain / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8"))
    return Draft7Validator(document, registry=registry())


def same_value(got, want):
    if got["kind"] != want["kind"]:
        return False
    if got["kind"] in ("bool", "text"):
        return got["value"] == want["value"]
    return abs(got["value"] - want["value"]) <= 1e-12 * max(1.0, abs(want["value"]))


def same_outcome(got, want):
    if "error" in want or "error" in got:
        return got.get("error") == want.get("error")
    return same_value(got, want)


DOMAINS = ["🌳️tree", "📏️kinds", "🧮️evaluation", "🕸️parameters", "🔤️syntax"]


@pytest.mark.parametrize("domain", [d for d in DOMAINS if d != "🌳️tree"])
def test_fixtures_validate_against_their_schema(domain):
    errors = sorted(validator(domain).iter_errors(load(domain)), key=lambda e: list(e.path))
    assert [e.message for e in errors[:3]] == []


def test_the_schema_rejects_malformed_trees():
    tree = validator("🌳️tree")
    assert tree.is_valid({"op": "number", "value": 2.0})
    assert not tree.is_valid({"op": "number", "value": -2.0})
    assert not tree.is_valid({"op": "length", "value": 2.0, "unit": "yd"})
    assert not tree.is_valid({"op": "binary", "operator": "add", "left": {"op": "number", "value": 1.0}})
    assert not tree.is_valid({"op": "call", "function": "frob", "arguments": []})
    assert not tree.is_valid({"op": "number", "value": 1.0, "extra": True})
    assert tree.is_valid({"op": "if", "condition": {"op": "bool", "value": True}, "then": {"op": "number", "value": 1.0}, "otherwise": {"op": "number", "value": 2.0}})


def test_syntax_fixtures_are_valid_python_with_the_same_shape():
    cases = load("🔤️syntax")["cases"]
    assert len(cases) >= 100
    for case in cases:
        assert to_tree(parse_python(case["python"])) == case["ast"], case["name"]


def test_kind_fixtures_are_reproduced():
    cases = load("📏️kinds")["cases"]
    assert len(cases) >= 150
    for case in cases:
        assert kind_only(case["python"], case["params"]) == case["expected"], case["name"]


def test_evaluation_fixtures_are_reproduced():
    cases = load("🧮️evaluation")["cases"]
    assert len(cases) >= 200
    for case in cases:
        got = run(case["python"], case["env"])
        assert same_outcome(got, case["expected"]), (case["name"], got, case["expected"])


def test_evaluation_fixtures_cover_values_and_every_error_code():
    cases = load("🧮️evaluation")["cases"]
    codes = {c["expected"]["error"]["code"] for c in cases if "error" in c["expected"]}
    kinds = {c["expected"]["kind"] for c in cases if "kind" in c["expected"]}
    assert {"unknown-parameter", "operand-kind", "mixed-kinds", "exponent", "branch-kinds", "division-by-zero", "domain", "overflow"} <= codes
    assert set(NUMERIC) | {"bool", "text"} <= kinds


def test_parameter_fixtures_are_reproduced():
    cases = load("🕸️parameters")["cases"]
    assert len(cases) >= 40
    for case in cases:
        got = resolve({n: p["python"] for n, p in case["params"].items()}, {n: p["python"] for n, p in case["overrides"].items()}, case["declared"])
        want = case["expected"]
        assert got["order"] == want["order"], case["name"]
        assert got["errors"] == want["errors"], case["name"]
        assert set(got["values"]) == set(want["values"]), case["name"]
        for name, value in got["values"].items():
            assert same_value(value, want["values"][name]), (case["name"], name)


def test_the_parameter_fixtures_contain_cycles_blocked_parameters_and_overrides():
    cases = load("🕸️parameters")["cases"]
    codes = {e["code"] for c in cases for e in c["expected"]["errors"].values()}
    assert {"cycle", "failed-dependency", "unknown-override", "declared-kind", "division-by-zero", "unknown-parameter"} <= codes
    assert any(c["overrides"] for c in cases)
