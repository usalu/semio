"""🔌️ Independent closed Jack parent/query oracle preserves every literal child-address field."""
import json
from semio_repo_test import Adapter, Outcome

REQUIRED = {"schema", "name", "manifest", "camera", "content", "query"}
OPTIONAL = {"manifestId", "rootNodeId"}
KINDS = ("set-query",)


def uri_in(ctx, needle):
    """🧫️ Selects one actual declared scenario input."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("declared parent fixture is absent: " + needle)


def fixture(ctx, needle):
    """📦️ Reads an original complete declared JSON source."""
    return json.loads(ctx.input_bytes(uri_in(ctx, needle)).decode("utf-8"))


def parent(value):
    """🪪️ A closed parent retains its complete literal content address; no scene is reconstructed."""
    if not isinstance(value, dict) or not REQUIRED <= value.keys() or not value.keys() <= REQUIRED | OPTIONAL:
        raise AssertionError("closed Jack parent fields required")
    if any(not isinstance(value[key], str) for key in ("schema", "name", "query")):
        raise AssertionError("declared parent text required")
    content = value["content"]
    if set(content) != {"childId", "target"} or set(content["target"]) != {"artifactId", "dialect"} or set(content["target"]["dialect"]) != {"artifactKind", "standard", "subset"}:
        raise AssertionError("complete literal child address required")
    return value


def apply(source, mutation):
    """📝️ The parent event changes only its query; every other original reference remains unchanged."""
    if set(mutation) != {"mutation", "value"} or mutation["mutation"] != "setQuery" or not isinstance(mutation["value"], str):
        raise AssertionError("closed set-query event required")
    return {**source, "query": mutation["value"]}


def outcome(value):
    """📤️ Canonical comparison includes every parent field and the complete child address."""
    return Outcome(value, raw=json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode("utf-8"))


def mutate_handler(_kind):
    """🎯️ Applies the authored event to the complete neutral parent."""
    def handler(ctx):
        source = parent(fixture(ctx, "asset://"))
        changed = apply(source, json.loads(ctx.doc_string()))
        if changed["query"] == source["query"]:
            raise AssertionError("authored event was an unexpected no-op")
        return outcome(changed)
    return handler


def inverse_handler(_kind):
    """↩️ The original query creates the inverse and restores the complete parent."""
    def handler(ctx):
        source = parent(fixture(ctx, "asset://"))
        changed = apply(source, json.loads(ctx.doc_string()))
        restored = apply(changed, {"mutation": "setQuery", "value": source["query"]})
        if restored != source:
            raise AssertionError("inverse lost original parent fields")
        return outcome({"mutated": changed, "restored": restored})
    return handler


def spec_handler(_kind):
    """📐️ Replays the same complete closed parent/event/after vector as the subject."""
    def handler(ctx):
        source = parent(fixture(ctx, "⬅️before"))
        expected = parent(fixture(ctx, "➡️after"))
        changed = apply(source, fixture(ctx, "🦠️mutation"))
        if json.loads(ctx.doc_string())["verdict"] != "applied" or changed["query"] == source["query"] or changed != expected:
            raise AssertionError("complete parent differs from its committed applied vector")
        return outcome(changed)
    return handler


def identity_handler(ctx):
    """🔁️ The declared parent carrier preserves every literal field across JSON re-encoding."""
    source = parent(fixture(ctx, "asset://"))
    restored = parent(json.loads(json.dumps(source, ensure_ascii=False)))
    if restored != source:
        raise AssertionError("parent carrier lost source identity")
    return outcome(restored)


def adapter():
    """🧭️ Only oracle roles register here; the native subject remains independently owned."""
    result = Adapter("python")
    for kind in KINDS:
        result = result.oracle("mutate-" + kind, mutate_handler(kind))
        result = result.oracle("inverse-" + kind, inverse_handler(kind))
        result = result.oracle("spec-vector-" + kind, spec_handler(kind))
    return result.oracle("identity-round-trip", identity_handler)
