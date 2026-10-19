"""🐍️ `s.energy.model`'s second, independent implementation of its own mutation vocabulary.

No third-party library reads or writes `.dsl.semio` — the recorded survey named and DECLINED
EnergyPlus and OpenStudio, and the `energyplus` weather reader already registered under
`✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset reads a different format for a different purpose, so it is
deliberately not reused here. The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json`, each kind's own
`../../🧬️schema/🧬️mutations/<dir>/🧬️schema/🔣️.json`, and
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s `📓️taxonomy.md`
verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the Rust it judges and
transliterates none of it.

The honest boundary this file used to carry is gone with `replace-model`: a whole-document swap is
not a mutation at all (rule 6), so there is no longer a kind whose only vector is a no-op. Every kind
below is exercised by one vector that moves the document and one that is refused.

`identity-round-trip` reads the committed `.dsl.semio` carrier through this file's own reader and
printer (`🔖️TextCarrier`), written from the carrier text itself: `key=value` members, `{ }` records,
`[ ]` lists, quoted strings, bare atoms, `null`. Its one boundary: no committed asset spells a
`referenced_model`/`weather_link` link slot, so the reader answers an absent slot with `null` and
refuses a present one instead of guessing its spelling.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import importlib.util
import json
import os
import re
import sys

from semio_repo_test import Adapter, Context, Outcome


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


mutation_vocabulary = _load("mutate_energy_model_1_mutation_vocabulary", "🧱️mutation-vocabulary")
building = _load("mutate_energy_model_1_building", "🏠️building")
openings = _load("mutate_energy_model_1_openings", "🪟️openings")
fabric = _load("mutate_energy_model_1_fabric", "🪨️fabric-and-gains")
airflow = _load("mutate_energy_model_1_airflow", "💨️airflow-and-controls")
zone_systems = _load("mutate_energy_model_1_zone_systems", "🔧️zone-systems")
power = _load("mutate_energy_model_1_power", "🔌️power-and-plant")
schedules = _load("mutate_energy_model_1_schedules", "📆️schedules-and-layers")

from mutate_energy_model_1_mutation_vocabulary import unwrap, wire_tag
from mutate_energy_model_1_building import FIELD_LEAVES, _zone
# endregion 🔖️Imports


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    **building.VECTOR_ROOTS,
    **openings.VECTOR_ROOTS,
    **fabric.VECTOR_ROOTS,
    **airflow.VECTOR_ROOTS,
    **zone_systems.VECTOR_ROOTS,
    **power.VECTOR_ROOTS,
    **schedules.VECTOR_ROOTS,
}


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.input_bytes(uri))


def _vector(ctx: Context, scenario: str):
    root = VECTOR_ROOTS[scenario]
    return (
        _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json"),
        _read_json(ctx, f"{root}/🦠️mutation/🔣️.json"),
        _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json"),
        _read_json(ctx, f"{root}/🎯️outcome/🔣️.json"),
    )
# endregion 🔖️Fixtures


# region 🔖️Vocabulary

VOCABULARY = {
    **building.VOCABULARY,
    **openings.VOCABULARY,
    **fabric.VOCABULARY,
    **airflow.VOCABULARY,
    **zone_systems.VOCABULARY,
    **power.VOCABULARY,
    **schedules.VOCABULARY,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    **building.EXTRA_INVERT,
    **openings.EXTRA_INVERT,
    **fabric.EXTRA_INVERT,
    **airflow.EXTRA_INVERT,
    **zone_systems.EXTRA_INVERT,
    **power.EXTRA_INVERT,
    **schedules.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary


# region 🔖️Inverse
def invert(kind, before, payload):
    """↩️ The undo steps a kind owes, always read off BASE — never by inverting a delta. A refused or
    no-op forward step owes nothing (taxonomy.md's addressing convention)."""
    after, outcome = VOCABULARY[kind](before, payload)
    if outcome["status"] == "rejected" or after == before:
        return []
    if kind in EXTRA_INVERT:
        return EXTRA_INVERT[kind](before, payload)
    if kind == "rename-model":
        return [("rename-model", {"newName": before["model"]["name"]})]
    if kind == "change-model-version":
        return [("change-model-version", {"newVersion": before["model"]["version"]})]
    if kind in FIELD_LEAVES:
        return [(kind, FIELD_LEAVES[kind](before, payload))]
    if kind == "replace-airflow-network":
        network = before["model"]["airflow_network"]
        if network is None:
            return [("replace-airflow-network", {"present": False, "zoneIds": [], "nodeIds": [], "outdoorNodeId": 0, "linkIds": []})]
        return [("replace-airflow-network", {"present": True, "zoneIds": [pair[0] for pair in network["zone_node_ids"]], "nodeIds": [pair[1] for pair in network["zone_node_ids"]], "outdoorNodeId": network["outdoor_node_id"], "linkIds": network["link_ids"]})]
    if kind == "add-output-variable":
        return [("remove-output-variable", {"name": payload["name"], "key": payload["key"]})]
    if kind == "remove-output-variable":
        index, spec = next((position, spec) for position, spec in enumerate(before["model"]["output_variables"]) if spec["name"] == payload["name"] and spec["key"] == payload["key"])
        return [("add-output-variable", {"name": spec["name"], "key": spec["key"], "reportingFrequency": spec["reporting_frequency"], "index": index})]
    if kind in ("bind-weather-file", "unbind-weather-file"):
        existing = before.get("weatherLink")
        return [("bind-weather-file", {"target": copy.deepcopy(existing["target"])})] if existing else [("unbind-weather-file", {})]
    if kind in ("connect-referenced-model", "disconnect-referenced-model"):
        existing = before.get("referencedModel")
        return [("connect-referenced-model", {"target": copy.deepcopy(existing["target"])})] if existing else [("disconnect-referenced-model", {})]
    zone = _zone(before, payload["id"])
    if kind == "rename-zone":
        return [("rename-zone", {"id": payload["id"], "newName": zone["name"]})]
    if kind == "change-zone-volume":
        return [("change-zone-volume", {"id": payload["id"], "newVolumeM3": zone["volume_m3"]})]
    if kind == "change-zone-multiplier":
        return [("change-zone-multiplier", {"id": payload["id"], "newMultiplier": zone["multiplier"]})]
    if kind == "change-zone-conditioned":
        return [("change-zone-conditioned", {"id": payload["id"], "newConditioned": zone["conditioned"]})]
    if kind == "change-zone-floor-area-participation":
        return [("change-zone-floor-area-participation", {"id": payload["id"], "newPartOfTotalFloorArea": zone["part_of_total_floor_area"]})]
    raise AssertionError(f"no inverse is written for {kind!r}")


def _uri_of(link):
    """🔗️ Flattens a link's `ArtifactRef` back to the URI form `bind`/`connect` payloads carry."""
    target = link["target"]
    dialect = target["dialect"]
    return f"{target['artifactId']}!{dialect['artifactKind']}@{dialect['standard']}/{dialect['subset']}"
# endregion 🔖️Inverse


# region 🔖️TextCarrier
ARTIFACT_MARK = "semio energy.model.dsl v1"
"""🔖️ The carrier's first line."""

DOCUMENT_MEMBERS = (("schema", "schema"), ("model", "model"), ("structure", "structure"), ("zones", "zones"), ("referenced_model", "referencedModel"), ("weather_link", "weatherLink"))
"""📖️ The document's top-level members in carrier order, each with the member name the committed JSON vectors spell."""

CHILD_MEMBERS = ("structure", "zones")
"""🧩️ The composed child handles, written inline as `child_id=<atom> target="<uri>"` without braces."""

LINK_MEMBERS = ("referenced_model", "weather_link")
"""🔗️ The optional link slots; the carrier omits an unset one."""

TOKEN = re.compile(r'\s*(?:(?P<string>"(?:[^"\\]|\\.)*")|(?P<punct>[\[\]{}=])|(?P<atom>[^\s\[\]{}="]+))')
INTEGER = re.compile(r"-?\d+")
FLOAT = re.compile(r"-?(?:\d+\.\d*|\.\d+|\d+)(?:[eE][-+]?\d+)?|-?inf|NaN")


def tokenize(text):
    """🔤️ The carrier body as `(kind, text)` tokens: quoted strings, `[ ] { } =`, and bare atoms."""
    tokens, at = [], 0
    while at < len(text):
        match = TOKEN.match(text, at)
        if match is None or match.end() == at:
            if text[at:].strip() == "":
                break
            raise AssertionError("identity-round-trip: unreadable carrier text at offset %d: %r" % (at, text[at:at + 40]))
        at = match.end()
        tokens.append((match.lastgroup, match.group(match.lastgroup)))
    return tokens


def scalar(text):
    """🔢️ A bare atom as its value: `null`, a boolean, an integer, a float, or the atom itself (`energy.model`)."""
    if text == "null":
        return None
    if text in ("true", "false"):
        return text == "true"
    if INTEGER.fullmatch(text):
        return int(text)
    if FLOAT.fullmatch(text):
        return float(text)
    return text


class Reader:
    """📖️ A token-stream reader: a record reads `key=value` members until its closing brace, a list reads values until
    its closing bracket, and a member name may never repeat inside one record."""

    def __init__(self, tokens):
        self.tokens, self.at = tokens, 0

    def peek(self):
        return self.tokens[self.at] if self.at < len(self.tokens) else (None, None)

    def take(self, expected=None):
        token = self.peek()
        if token[0] is None or (expected is not None and token[1] != expected):
            raise AssertionError("identity-round-trip: expected %r, found %r at token %d" % (expected, token[1], self.at))
        self.at += 1
        return token

    def value(self):
        kind, text = self.take()
        if (kind, text) == ("punct", "{"):
            record = {}
            while self.peek()[1] != "}":
                key_kind, key = self.take()
                if key_kind != "atom" or key in record:
                    raise AssertionError("identity-round-trip: %r is not a fresh member name at token %d" % (key, self.at - 1))
                self.take("=")
                record[key] = self.value()
            self.take("}")
            return record
        if (kind, text) == ("punct", "["):
            items = []
            while self.peek()[1] != "]":
                items.append(self.value())
            self.take("]")
            return items
        if kind == "punct":
            raise AssertionError("identity-round-trip: expected a value, found %r at token %d" % (text, self.at - 1))
        return json.loads(text) if kind == "string" else scalar(text)

    def child(self):
        record = {}
        for member in ("child_id", "target"):
            self.take(member)
            self.take("=")
            record[member] = self.value()
        match = re.fullmatch(r"([^!]+)!([^@]+)@([^/]+)/(.+)", record["target"])
        if match is None or match.group(1) != record["child_id"]:
            raise AssertionError("identity-round-trip: %r is not the child target URI of %r" % (record["target"], record["child_id"]))
        artifact, kind, standard, subset = match.groups()
        return {"childId": record["child_id"], "target": {"artifactId": artifact, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}}


def read_carrier(text):
    """📥️ Reads a `.dsl.semio` energy model into the document the committed JSON vectors spell."""
    head, _, body = text.partition("\n")
    if head.strip() != ARTIFACT_MARK:
        raise AssertionError("identity-round-trip: the carrier starts with %r, not %r" % (head, ARTIFACT_MARK))
    reader, document = Reader(tokenize(body)), {}
    for member, field in DOCUMENT_MEMBERS:
        if reader.peek()[1] != member:
            if member not in LINK_MEMBERS:
                raise AssertionError("identity-round-trip: the carrier lacks its %r member" % member)
            document[field] = None
            continue
        if member in LINK_MEMBERS:
            raise AssertionError("identity-round-trip: no committed asset spells the %r link slot, so this reader refuses to guess it" % member)
        reader.take(member)
        reader.take("=")
        document[field] = reader.child() if member in CHILD_MEMBERS else reader.value()
    if reader.peek()[0] is not None:
        raise AssertionError("identity-round-trip: unread carrier text from token %r on" % (reader.peek()[1],))
    return document


def print_value(value):
    """📤️ One value as carrier tokens: records with their members in ascending name order, as the committed carrier writes them."""
    if value is None:
        return ["null"]
    if isinstance(value, bool):
        return ["true" if value else "false"]
    if isinstance(value, (int, float)):
        return [repr(value)]
    if isinstance(value, str):
        return [json.dumps(value, ensure_ascii=False)]
    if isinstance(value, list):
        return ["["] + [token for item in value for token in print_value(item)] + ["]"]
    return ["{"] + [token for key in sorted(value) for token in [key, "="] + print_value(value[key])] + ["}"]


def print_carrier(document):
    """📤️ The document back as carrier text: the mark line, then every member's tokens space-separated."""
    tokens = []
    for member, field in DOCUMENT_MEMBERS:
        value = document[field]
        if member in LINK_MEMBERS:
            if value is not None:
                raise AssertionError("identity-round-trip: no committed asset spells the %r link slot, so this printer refuses to guess it" % member)
            continue
        tokens += [member, "="]
        if member in CHILD_MEMBERS:
            tokens += ["child_id", "=", value["childId"], "target", "=", json.dumps(_uri_of(value), ensure_ascii=False)]
        elif member == "schema":
            tokens.append(value)
        else:
            tokens += print_value(value)
    return ARTIFACT_MARK + "\n" + " ".join(tokens) + "\n"


def same_tokens(left, right):
    """⚖️ Two carrier texts spell the same token stream; numeric atoms compare by kind and value, so `1e-05` and `1e-5` agree."""
    def canonical(token):
        kind, text = token
        value = scalar(text) if kind == "atom" else None
        return (kind, type(value).__name__, value) if isinstance(value, (int, float)) and not isinstance(value, bool) else (kind, text)
    return [canonical(token) for token in tokenize(left)] == [canonical(token) for token in tokenize(right)]
# endregion 🔖️TextCarrier


# region 🔖️Oracle
def _kind_of(scenario, wire):
    tag, payload = unwrap(wire)
    for kind in VOCABULARY:
        if wire_tag(kind) == tag:
            return kind, payload
    raise AssertionError(f"unexpected wire tag {tag!r} for scenario {scenario!r}")


def _mutate_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, expected_after, expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, outcome = VOCABULARY[kind](before, payload)
        assert after == expected_after, f"mutate-{scenario}: {after} != committed after-snapshot {expected_after}"
        assert outcome == expected_outcome, f"mutate-{scenario}: {outcome} != committed outcome {expected_outcome}"
        return Outcome(projection=after, raw=json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def _inverse_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, _expected_after, _expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, _outcome = VOCABULARY[kind](before, payload)
        restored = after
        for step_kind, step_payload in invert(kind, before, payload):
            restored, step_outcome = VOCABULARY[step_kind](restored, step_payload)
            assert step_outcome["status"] != "rejected", f"inverse-{scenario}: the undo step {step_kind} was itself refused"
        assert restored == before, f"inverse-{scenario}: {restored} != committed before-snapshot {before}"
        return Outcome(projection=restored, raw=json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def identity_handler(ctx: Context) -> Outcome:
    """🔁️ Reads the real committed `.dsl.semio` model through this file's own carrier reader and answers the document it
    holds. In role it also requires this printer to round-trip it (print, read back, print again is a fixpoint), to
    reproduce the committed carrier's token stream, and the document to carry zones and surfaces, so the record, list,
    nested-list and child-handle grammar are all exercised."""
    uri = next((uri for uri in ctx.step_input_uris() if uri.endswith("🗣️.dsl.semio")), None)
    if uri is None:
        raise AssertionError("identity-round-trip: the scenario names no committed .dsl.semio document")
    committed = ctx.input_bytes(uri).decode("utf-8")
    document = read_carrier(committed)
    printed = print_carrier(document)
    if read_carrier(printed) != document or print_carrier(read_carrier(printed)) != printed:
        raise AssertionError("identity-round-trip: printing the document and reading it back is not a fixpoint")
    if not same_tokens(printed, committed):
        raise AssertionError("identity-round-trip: this printer does not reproduce the committed carrier's token stream")
    if not document["model"]["zones"] or not document["model"]["surfaces"]:
        raise AssertionError("identity-round-trip: the committed model must carry zones and surfaces, or it would not exercise the carrier grammar")
    return Outcome(projection=document, raw=json.dumps(document, sort_keys=True, separators=(",", ":")).encode("utf-8"))
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only: registering these handlers as subjects too would make the
    reference its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for scenario in VECTOR_ROOTS:
        built = built.oracle(f"mutate-{scenario}", _mutate_for(scenario)).oracle(f"inverse-{scenario}", _inverse_for(scenario))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration
