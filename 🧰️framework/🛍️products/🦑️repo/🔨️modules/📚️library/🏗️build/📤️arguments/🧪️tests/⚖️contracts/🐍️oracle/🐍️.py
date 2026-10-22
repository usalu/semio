"""⚖️ Independent Python oracle of the JSON Schema subset the build owner contracts use (standard library only): const, enum, null, boolean, integer, string, array and object rules, anyOf, with unsupported rules admitting nothing. Requests arrive as JSON on stdin: {"cases": [{"contract", "value"}]}."""
import json, re, sys


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def code_points(text):
    return len(text.encode("utf-16-le", "surrogatepass")) // 2 - sum(1 for ch in text if ord(ch) > 0xFFFF)


def search(pattern, text):
    if pattern.endswith("$") and not pattern.endswith("\\$"):
        pattern = pattern[:-1] + "\\Z"
    return re.search(pattern, text) is not None


def admits(rule, value):
    if "anyOf" in rule:
        return any(admits(option, value) for option in rule["anyOf"])
    kind = rule.get("type")
    if kind == "null":
        return value is None
    if "const" in rule and canon(value) != canon(rule["const"]):
        return False
    if "enum" in rule:
        return canon(value) in {canon(item) for item in rule["enum"]}
    if kind == "object":
        if not isinstance(value, dict):
            return False
        props = rule.get("properties", {})
        if any(name not in value for name in rule.get("required", [])):
            return False
        if rule.get("additionalProperties") is False and any(name not in props for name in value):
            return False
        return all(admits(sub, value[name]) for name, sub in props.items() if name in value)
    if kind == "array":
        if not isinstance(value, list):
            return False
        if "minItems" in rule and len(value) < rule["minItems"]:
            return False
        if "maxItems" in rule and len(value) > rule["maxItems"]:
            return False
        if "items" in rule and not all(admits(rule["items"], item) for item in value):
            return False
        return rule.get("uniqueItems") is not True or len({canon(item) for item in value}) == len(value)
    if kind == "integer":
        if isinstance(value, bool) or not isinstance(value, int) or abs(value) > 2 ** 53 - 1:
            return False
        return ("minimum" not in rule or value >= rule["minimum"]) and ("maximum" not in rule or value <= rule["maximum"])
    if kind == "boolean":
        return isinstance(value, bool)
    if kind == "string":
        if not isinstance(value, str):
            return False
        size = code_points(value)
        return ("minLength" not in rule or size >= rule["minLength"]) and ("maxLength" not in rule or size <= rule["maxLength"]) and (not rule.get("pattern") or search(rule["pattern"], value))
    return "const" in rule


print(json.dumps([admits(case["contract"], case["value"]) for case in json.load(sys.stdin)["cases"]]))
