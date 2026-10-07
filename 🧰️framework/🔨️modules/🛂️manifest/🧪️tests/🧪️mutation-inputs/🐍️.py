#!/usr/bin/env python3
"""🐍️ Third-party oracle for the mutation input descriptors, in Python, over ``jsonschema`` (Draft 7).

The Rust reader (``manifest::mutation_input_defs``) and its TypeScript twin (``mutationInputDefs``) are both held
byte-for-byte to the ``expectedInputs`` of ``shared://🧫️mutation-inputs/🔣️.json``. This adapter asks an independent
validator whether those descriptors tell the truth about their leaf schemas: every hard bound, option and
requirement a descriptor declares must be exactly what ``jsonschema`` enforces — a value at an inclusive bound, every
option and every required input present validate; a value past a bound, an undeclared option, a fraction for an
integer, a missing required input and an oversized vector are refused. It validates the domain glossary and each supplied leaf schema independently; examples remain plain test input.

Run directly from the repository root: ``python3 "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py"``.
Exits non-zero on any disagreement.

@see ./🟦️.ts
@see ./🦀️.rs
@see ../../🧫️fixtures/🧫️mutation-inputs/🔣️.json
"""

# region 🔖️Imports
import json
import os
import re
import sys

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

# endregion 🔖️Imports


# region 🔖️Reference
HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "..", "..")
CORPUS = os.path.join(MANIFEST, "🧫️fixtures", "🧫️mutation-inputs", "🔣️.json")
SCHEMA = os.path.join(MANIFEST, "🧬️schema", "🔣️.json")
GLOSSARY = os.path.join(MANIFEST, "🔣️input-labels.json")


def load(path):
    """📄️ One JSON document from disk."""
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def registry(documents):
    """🗂️ Every schema document a ``$ref`` may reach, addressed by its ``$id``."""
    return Registry().with_resources((identifier, Resource.from_contents(document, default_specification=DRAFT7)) for identifier, document in documents.items())


def export(manifest, name):
    """🛡️ A Draft 7 validator of one manifest ``$defs`` export."""
    return jsonschema.Draft7Validator({"$ref": "%s#/$defs/%s" % (manifest["$id"], name)}, registry=registry({manifest["$id"]: manifest}))


def sample(schema):
    """🧪️ A value the declared descriptor admits, built from the descriptor alone."""
    kind = schema["kind"]
    if kind == "string":
        options = schema.get("options", [])
        return options[0]["value"] if options else "a" * max(schema.get("minLen", 0), 1)
    if kind == "number":
        step = 1 if schema["integer"] else 0.5
        if "min" in schema:
            return schema["min"] + step if schema.get("minExclusive") else schema["min"]
        if "max" in schema:
            return schema["max"] - step if schema.get("maxExclusive") else schema["max"]
        return 0
    if kind == "boolean":
        return False
    if kind == "vector":
        return [schema["min"] if "min" in schema else min(schema.get("max", 0), 0)] * schema["dims"]
    if kind == "reference":
        identity = (lambda index: index) if schema.get("idType") == "integer" else (lambda index: "id-%d" % index)
        return [identity(index) for index in range(max(schema.get("minItems", 0), 1))] if schema.get("many") else identity(0)
    if kind == "array":
        return [sample(schema["items"]) for _ in range(schema.get("minItems", 0))]
    if kind == "object":
        return {field["id"][1:]: sample(field["schema"]) for field in schema["fields"] if field["required"]}
    return None


def selector_of(inputs):
    """🔀️ The variant selector of a discriminated union: the first input, a choice whose values name the other inputs' groups."""
    if not inputs or inputs[0]["schema"]["kind"] != "string":
        return None
    values = [option["value"] for option in inputs[0]["schema"].get("options", [])]
    return inputs[0] if values and any(descriptor.get("group") in values for descriptor in inputs[1:]) else None


def probes(leaf, inputs):
    """⚖️ The accept/reject verdicts the descriptors imply for one leaf: ``(label, payload, admitted)``, per variant of a union."""
    discriminators = {key: node["const"] for key, node in leaf.get("properties", {}).items() if isinstance(node, dict) and "const" in node}
    selector = selector_of(inputs)
    variants = [option["value"] for option in selector["schema"]["options"]] if selector else [None]
    verdicts = []
    for variant in variants:
        active = [descriptor for descriptor in inputs if descriptor is not selector and descriptor.get("group") == variant] if selector else inputs
        if any(descriptor["required"] and descriptor["schema"]["kind"] == "any" for descriptor in active):
            continue
        prefix = "" if variant is None else "%s/" % variant
        base = dict(discriminators, **({selector["id"][1:]: variant} if selector else {}))
        base.update({descriptor["id"][1:]: sample(descriptor["schema"]) for descriptor in active if descriptor["required"]})
        verdicts.append(("%sbase" % prefix, base, True))
        if selector:
            verdicts.append(("%snot-a-variant" % prefix, dict(base, **{selector["id"][1:]: "not-a-variant"}), False))
        for descriptor in active:
            key = descriptor["id"][1:]
            schema = descriptor["schema"]
            if descriptor["required"]:
                verdicts.append(("%s%s:absent" % (prefix, key), {name: value for name, value in base.items() if name != key}, False))
            elif schema["kind"] != "any":
                verdicts.append(("%s%s:present" % (prefix, key), dict(base, **{key: sample(schema)}), True))
            if schema["kind"] != "any":
                verdicts.append(("%s%s:null" % (prefix, key), dict(base, **{key: None}), bool(descriptor.get("nullable"))))
            if schema["kind"] == "number":
                if "min" in schema:
                    verdicts += [("%s%s:min" % (prefix, key), dict(base, **{key: schema["min"]}), not schema.get("minExclusive")), ("%s%s:below" % (prefix, key), dict(base, **{key: schema["min"] - 1}), False)]
                if "max" in schema:
                    verdicts += [("%s%s:max" % (prefix, key), dict(base, **{key: schema["max"]}), not schema.get("maxExclusive")), ("%s%s:above" % (prefix, key), dict(base, **{key: schema["max"] + 1}), False)]
                if schema["integer"]:
                    verdicts.append(("%s%s:fraction" % (prefix, key), dict(base, **{key: sample(schema) + 0.5}), False))
            if schema["kind"] == "string" and schema.get("options"):
                verdicts += [("%s%s:%s" % (prefix, key, option["value"]), dict(base, **{key: option["value"]}), True) for option in schema["options"]]
                verdicts.append(("%s%s:not-an-option" % (prefix, key), dict(base, **{key: "not-an-option"}), False))
            if schema["kind"] == "string" and "maxLen" in schema:
                verdicts.append(("%s%s:too-long" % (prefix, key), dict(base, **{key: "a" * (schema["maxLen"] + 1)}), False))
            if schema["kind"] == "string" and schema.get("minLen", 0) > 0:
                verdicts.append(("%s%s:too-short" % (prefix, key), dict(base, **{key: ""}), False))
            if schema["kind"] == "vector":
                verdicts.append(("%s%s:too-many" % (prefix, key), dict(base, **{key: sample(schema) + sample(schema)[:1]}), False))
                if "min" in schema:
                    verdicts += [("%s%s:component-min" % (prefix, key), dict(base, **{key: [schema["min"]] * schema["dims"]}), True), ("%s%s:component-below" % (prefix, key), dict(base, **{key: [schema["min"] - 1] * schema["dims"]}), False)]
                if "max" in schema:
                    verdicts += [("%s%s:component-max" % (prefix, key), dict(base, **{key: [schema["max"]] * schema["dims"]}), True), ("%s%s:component-above" % (prefix, key), dict(base, **{key: [schema["max"] + 1] * schema["dims"]}), False)]
            if schema["kind"] == "reference" and schema.get("many") and schema.get("minItems", 0) > 0:
                verdicts.append(("%s%s:empty" % (prefix, key), dict(base, **{key: []}), False))
            if schema["kind"] == "reference":
                other = "id-0" if schema.get("idType") == "integer" else 0
                verdicts.append(("%s%s:other-id-type" % (prefix, key), dict(base, **{key: [other] if schema.get("many") else other}), False))
    return verdicts


# endregion 🔖️Reference


# region 🔖️Handlers
def reference_id_value(id_type, text):
    """🎯️ Python's own reading of a selected id text as a payload id: the text for a string reference, the decimal integer it
    spells (within ±(2^53 − 1)) for an integer reference, ``None`` otherwise."""
    if text == "":
        return None
    if id_type == "string":
        return text
    if not re.fullmatch(r"-?[0-9]+", text):
        return None
    value = int(text)
    return value if abs(value) <= 2 ** 53 - 1 else None


def reference_id_failures(rows):
    """🔢️ Holds the corpus' reference id table to ``reference_id_value`` and every converted id to ``jsonschema``'s id type."""
    failures = []
    for row in rows:
        value = reference_id_value(row["idType"], row["text"])
        if value != row["value"] or type(value) is not type(row["value"]):
            failures.append("referenceIds %s %r: Python reads %r, the table says %r" % (row["idType"], row["text"], value, row["value"]))
        if value is not None and not jsonschema.Draft7Validator({"type": row["idType"]}).is_valid(value):
            failures.append("referenceIds %s %r: jsonschema refuses %r as a %s id" % (row["idType"], row["text"], value, row["idType"]))
        if value is not None and str(value) != row.get("spelled"):
            failures.append("referenceIds %s %r: spelled %r, the table says %r" % (row["idType"], row["text"], str(value), row.get("spelled")))
    return failures


def main():
    """🧭️ Validates the glossary, then holds every declared descriptor to ``jsonschema``'s verdicts."""
    manifest = load(SCHEMA)
    corpus = load(CORPUS)
    failures = []
    failures += ["glossary %s: %s" % ("/".join(map(str, error.absolute_path)), error.message) for error in export(manifest, "InputLabelGlossary").iter_errors(load(GLOSSARY))]
    failures += reference_id_failures(corpus["referenceIds"])
    documents = registry(corpus["documents"])
    judged = 0
    for case in corpus["cases"]:
        if "expectedInputs" not in case:
            continue
        validator = jsonschema.Draft7Validator(case["input"]["leafSchema"], registry=documents)
        for probe, payload, admitted in probes(case["input"]["leafSchema"], case["expectedInputs"]):
            judged += 1
            if validator.is_valid(payload) != admitted:
                failures.append("%s %s: jsonschema says %s, the descriptors say %s for %s" % (case["name"], probe, "valid" if not admitted else "invalid", "valid" if admitted else "invalid", json.dumps(payload, ensure_ascii=False)))
    if judged < 60:
        failures.append("only %d verdicts were judged — an empty sweep is no evidence" % judged)
    for failure in failures:
        print("✘ %s" % failure)
    print("%s %d jsonschema verdicts over %d corpus cases, %d reference ids" % ("✘" if failures else "✔", judged, len(corpus["cases"]), len(corpus["referenceIds"])))
    return 1 if failures else 0


# endregion 🔖️Handlers

if __name__ == "__main__":
    sys.exit(main())
