#!/usr/bin/env python3
"""🧬️ Oracle of the quiz contract's conformance: ``jsonschema`` judges every quiz document the product meets.

``jsonschema``'s Draft 7 validator reads the normative ``🧬️schema/🔣️.json`` and judges three things:
every schema-typed document the committed vectors carry (table rows of the feature — each row names a
fixture, a JSON pointer with ``*`` wildcards and a ``$defs`` entry), every quiz and catalog authored under
``🎓️teaching``, and the committed rejected quizzes and catalogs, each of which must break the schema. Only
the quizzes and catalogs are projected, because only they have an owned validator on the subject side
(``quizIssues``/``catalogIssues``); every other typed document is still validated here and fails the
oracle phase when it breaks the contract.

@see https://python-jsonschema.readthedocs.io/en/stable/validate/
@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🧬️schema-conformance/🔣️.json
"""

# region 🔖️Imports
import json
import os

import jsonschema

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧬️schema", "🔣️.json")
REJECTED = "shared://🧬️schema-conformance/🔣️.json"
TEACHING = "🎓️teaching"
SKIPPED = {"node_modules", "target", "dist", "📤️dist", ".git", "🗑️generated", "__pycache__"}
QUIZ_DIRECTORY = "❓️quiz"
DOCUMENT = "🔣️.json"
DEFINITIONS = {"semio.quiz/v1": "Quiz", "semio.quiz.catalog/v1": "Catalog"}


def validator(definition):
    """🛡️ A Draft 7 validator of one ``$defs`` entry, resolving references inside the normative schema."""
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        schema = json.load(handle)
    return jsonschema.Draft7Validator({**schema, "$ref": "#/$defs/%s" % definition})


def matches(document, pointer):
    """📍️ Every value a JSON pointer with ``*`` wildcards reaches, with its concrete pointer."""
    found = [("", document)]
    for part in [segment for segment in pointer.split("/") if segment]:
        reached = []
        for path, value in found:
            if part == "*":
                children = enumerate(value) if isinstance(value, list) else value.items() if isinstance(value, dict) else []
                reached += [("%s/%s" % (path, key), child) for key, child in children]
            elif isinstance(value, dict) and part in value:
                reached.append(("%s/%s" % (path, part), value[part]))
            elif isinstance(value, list) and part.isdigit() and int(part) < len(value):
                reached.append(("%s/%s" % (path, part), value[int(part)]))
        found = reached
    return found


def violations(definition, document):
    """💥️ The schema's complaints about one document, as ``<path>: <validator>`` lines."""
    return ["/%s: %s" % ("/".join(str(part) for part in error.absolute_path), error.validator) for error in validator(definition).iter_errors(document)]


def rows(ctx):
    """📋️ The feature's table of ``(id, fixture, pointer, definition)`` rows."""
    table = next(step["dataTable"] for step in ctx.scenario["steps"] if step.get("dataTable"))
    header = table[0]
    return [dict(zip(header, row)) for row in table[1:]]


def teaching_documents(repo_root):
    """🔎️ Every ``❓️quiz/🔣️.json`` under the teaching area, repo-relative, in path order."""
    found = []
    for directory, children, files in os.walk(os.path.join(repo_root, TEACHING)):
        children[:] = sorted(child for child in children if child not in SKIPPED)
        if os.path.basename(directory) == QUIZ_DIRECTORY and DOCUMENT in files:
            found.append(os.path.relpath(os.path.join(directory, DOCUMENT), repo_root).replace(os.sep, "/"))
    return sorted(found)


# endregion 🔖️Reference


# region 🔖️Handlers
def fixture_documents(ctx):
    """🧫️ Every typed document of the committed vectors must conform; quizzes and catalogs are projected."""
    produced = {}
    for row in rows(ctx):
        documents = matches(json.loads(ctx.fixture_bytes(row["fixture"])), row["pointer"])
        if not documents:
            raise AssertionError("fixture-documents/%s: %s reaches nothing in %s" % (row["id"], row["pointer"], row["fixture"]))
        for path, document in documents:
            broken = violations(row["definition"], document)
            if broken:
                raise AssertionError("fixture-documents/%s: %s%s is not a %s — %s" % (row["id"], row["fixture"], path, row["definition"], "; ".join(broken)))
        if row["definition"] in ("Quiz", "Catalog"):
            produced[row["id"]] = {path: True for path, _ in documents}
    return Outcome(produced)


def repository_quizzes(ctx):
    """🎓️ Every quiz and catalog authored in the teaching area, judged by the schema its ``schema`` field names."""
    produced = {}
    for path in teaching_documents(ctx.repo_root):
        with open(os.path.join(ctx.repo_root, path), "r", encoding="utf-8") as handle:
            document = json.load(handle)
        definition = DEFINITIONS.get(document.get("schema")) if isinstance(document, dict) else None
        produced[path] = definition is not None and not violations(definition, document)
    if not produced:
        raise AssertionError("repository-quizzes: no %s/%s exists under %s — an empty sweep is no evidence" % (QUIZ_DIRECTORY, DOCUMENT, TEACHING))
    return Outcome(produced)


def rejected_quizzes(ctx):
    """🚫️ Every committed rejected document must break the schema at the rule its vector names; the unbroken bases conform."""
    vectors = json.loads(ctx.fixture_bytes(REJECTED))
    produced = {}
    for vector in vectors["accepted"]:
        broken = violations(vector["definition"], vector["document"])
        if broken:
            raise AssertionError("rejected-quizzes/%s: the unbroken base is not a %s — %s" % (vector["id"], vector["definition"], "; ".join(broken)))
        produced[vector["id"]] = True
    for vector in vectors["rejected"]:
        broken = violations(vector["definition"], vector["document"])
        if not any(line.endswith(": %s" % vector["violates"]) for line in broken):
            raise AssertionError("rejected-quizzes/%s: expected a %s violation, the schema reports %r" % (vector["id"], vector["violates"], broken))
        produced[vector["id"]] = False
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: ``jsonschema`` is the reference, the owned validators are the subjects."""
    return Adapter("python").oracle("fixture-documents", fixture_documents).oracle("repository-quizzes", repository_quizzes).oracle("rejected-quizzes", rejected_quizzes)


# endregion 🔖️Registration
