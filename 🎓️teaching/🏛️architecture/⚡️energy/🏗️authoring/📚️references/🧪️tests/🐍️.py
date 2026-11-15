"""🧪 The reference registry is valid, verified per entry, and every designation shown anywhere in the five energy videos comes from it."""

from __future__ import annotations

import ast
import importlib
import json
import re
import unittest
from pathlib import Path

import jsonschema

R = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
HERE = Path(__file__).resolve()
ENERGY = next(parent for parent in HERE.parents if parent.name == "⚡️energy")
MODULE = HERE.parents[1]
CASES = json.loads(HERE.with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SCAN = CASES["scan"]
DESIGNATION = re.compile(SCAN["designationPattern"])
LAW = re.compile(SCAN["lawPattern"])
KEY = re.compile(SCAN["narrationKeyPattern"])
EXCLUDED = set(SCAN["excludedDirs"])
EDITION = re.compile(r":\d{4}(-\d{2})?$")
STATUS_CHIPS = frozenset(R.status_chip(i, language) for i, item in R.registry().items() if item["status"] != "current" for language in R.LANGUAGES)
OLD_EDITIONS =frozenset(R.designation(i) for i, item in R.registry().items() if item["status"] != "current" and not item.get("shownWithoutStatus"))


def call(case: dict):
    """📞 Run one language-agnostic API case against the module."""
    args = [tuple(a) if isinstance(a, list) else a for a in case["args"]]
    if case["call"] == "chip":
        parts = [tuple(p) if isinstance(p, list) else p for p in case["args"][0]]
        return R.chip(*parts, language=case["language"])
    function = getattr(R, case["call"])
    return function(*args, language=case["language"]) if "language" in case else function(*args)


def narration_lines(tree: ast.AST) -> set[int]:
    """🗣️ Line numbers of the English and German text of every ``(key, en, de)`` clause."""
    lines: set[int] = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Tuple) and len(node.elts) == 3 and all(isinstance(e, ast.Constant) and isinstance(e.value, str) for e in node.elts) and KEY.match(node.elts[0].value):
            for element in node.elts[1:]:
                lines.update(range(element.lineno, element.end_lineno + 1))
    return lines


def python_strings(path: Path) -> list[tuple[int, str, bool]]:
    """🐍 ``(line, text, is_narration)`` of every string literal of a source file that is not a docstring."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    skipped = {id(node.value) for node in ast.walk(tree) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)}
    narration = narration_lines(tree)
    return [(node.lineno, node.value, node.lineno in narration) for node in ast.walk(tree)
            if isinstance(node, ast.Constant) and isinstance(node.value, str) and id(node) not in skipped]


def json_strings(value, line: int = 0):
    """🧾 Every string value of a JSON document, with a running index as pseudo line."""
    if isinstance(value, str):
        yield value
    elif isinstance(value, list):
        for item in value:
            yield from json_strings(item)
    elif isinstance(value, dict):
        for item in value.values():
            yield from json_strings(item)


def sources() -> list[tuple[str, int, str, bool]]:
    """📚 ``(file, line, text, is_narration)`` of everything shown in the scanned roots."""
    found: list[tuple[str, int, str, bool]] = []
    for root in SCAN["roots"]:
        base = ENERGY / root
        if not base.is_dir():
            continue
        for path in base.rglob("*"):
            relative = path.relative_to(ENERGY)
            if not path.is_file() or path.suffix not in SCAN["extensions"] or EXCLUDED & set(relative.parts) or any(p.startswith(".🧬") for p in relative.parts):
                continue
            name = relative.as_posix()
            if path.suffix == ".py":
                found.extend((name, line, text, narration) for line, text, narration in python_strings(path))
            elif path.suffix == ".md":
                found.extend((name, number, text, False) for number, text in enumerate(path.read_text(encoding="utf-8").splitlines(), 1))
            else:
                text = path.read_text(encoding="utf-8")
                try:
                    document = json.loads(text)
                except json.JSONDecodeError:
                    continue
                for number, value in enumerate(json_strings(document), 1):
                    found.append((name, number, value, False))
    return found


def name_forms() -> dict[str, list[str]]:
    """📕 For every watched author name the registry strings that legitimately contain it."""
    forms: dict[str, list[str]] = {}
    for name, reference_id in SCAN["namePatterns"].items():
        collected = []
        for language in R.LANGUAGES:
            collected += [R.citation(reference_id, language), R.bibliography(reference_id, language), R.base(reference_id, language)]
            collected += [R.ref(reference_id, locator, language=language) for locator in R.entry(reference_id)["locators"]]
        forms[name] = [form for form in collected if name in form]
    return forms


def problems_of(text: str, narration: bool, forms: frozenset[str], names: dict[str, list[str]]) -> list[str]:
    """🚨 Everything wrong with the references inside one shown string."""
    problems: list[str] = []
    matches = list(DESIGNATION.finditer(text))
    chips = [(m.start(), m.end()) for chip in STATUS_CHIPS for m in re.finditer(re.escape(chip), text)]
    for match in matches:
        found = match.group(0)
        if any(start <= match.start() and match.end() <= end for start, end in chips):
            continue
        if found not in forms:
            problems.append(f"{found!r} is not a registry form")
        elif found in OLD_EDITIONS and not any(word in text for word in SCAN["supersededContext"]):
            problems.append(f"{found!r} is withdrawn or superseded and shown without its status")
        elif not EDITION.search(found) and not (narration and "narration" in SCAN["shortAllowedIn"]):
            problems.append(f"{found!r} has no edition")
    for match in LAW.finditer(text):
        if not any(text[:match.start()].endswith(prefix) for prefix in SCAN["lawAliasPrefixes"]):
            problems.append("'GEG' outside the registry phrase 'vormals GEG'")
    if matches or LAW.search(text) or "GModG" in text:
        problems += [f"forbidden {word!r}" for word in SCAN["forbiddenText"] if word in text]
    if not narration:
        problems += [f"{name!r} outside a registry citation" for name, allowed in names.items() if name in text and not any(form in text for form in allowed)]
    return problems


def violations() -> list[str]:
    """📋 ``file:line problem: text`` for every shown string that breaks the registry rules."""
    forms = R.all_forms()
    names = name_forms()
    result: list[str] = []
    for file, line, text, narration in sources():
        for problem in problems_of(text, narration, forms, names):
            result.append(f"{file}:{line} {problem}: {text[:140]!r}")
    return result


class Registry(unittest.TestCase):
    def test_schema_validates_the_registry(self):
        schema = json.loads(MODULE.joinpath("🧬️schema", "🔣️.schema.json").read_text(encoding="utf-8"))
        data = json.loads(MODULE.joinpath("🔣️.json").read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator.check_schema(schema)
        errors = [f"{list(e.path)}: {e.message}" for e in jsonschema.Draft202012Validator(schema).iter_errors(data)]
        self.assertEqual(errors, [])

    def test_ids_are_unique_and_designations_with_edition_are_unique(self):
        data = json.loads(MODULE.joinpath("🔣️.json").read_text(encoding="utf-8"))["entries"]
        self.assertEqual(len({e["id"] for e in data}), len(data))
        forms = [R.designation(e["id"]) for e in data]
        self.assertEqual(len(set(forms)), len(forms))

    def test_every_entry_has_edition_or_year_a_url_and_a_verification_date(self):
        for reference_id, item in R.registry().items():
            for field in CASES["registry"]["requiredPerKind"][item["kind"]]:
                self.assertTrue(item.get(field), f"{reference_id} lacks {field}")

    def test_withdrawn_and_superseded_entries_name_status_and_a_current_successor(self):
        for reference_id, item in R.registry().items():
            if item["status"] != "current":
                successor = R.entry(item["successor"])
                self.assertEqual(successor["status"], "current", reference_id)
                for language in R.LANGUAGES:
                    chip = R.status_chip(reference_id, language)
                    self.assertIn(R.STATUS_WORDS[item["status"]][language], chip)
                    self.assertIn(R.short(item["successor"]), chip)
        for reference_id in CASES["registry"]["mustBeWithdrawnOrSuperseded"]:
            self.assertNotEqual(R.entry(reference_id)["status"], "current", reference_id)
        for reference_id in CASES["registry"]["mustBeCurrent"]:
            self.assertEqual(R.entry(reference_id)["status"], "current", reference_id)

    def test_every_locator_has_both_languages(self):
        for item in R.registry().values():
            for locator, text in item["locators"].items():
                self.assertTrue(text["de"] and text["en"], f"{item['id']}/{locator}")

    def test_language_agnostic_api_cases(self):
        for case in CASES["api"]:
            with self.subTest(case=case["call"], args=case["args"]):
                self.assertEqual(call(case), case["expect"])

    def test_unknown_ids_and_locators_fail_loudly(self):
        with self.assertRaises(KeyError):
            R.ref("din-9999", language="de")
        with self.assertRaises(KeyError):
            R.ref("din-4108-2", "nope", language="de")
        with self.assertRaises(ValueError):
            R.ref("din-4108-2", language="fr")

    def test_all_forms_contain_every_designation_and_locator_reference(self):
        forms = R.all_forms()
        for reference_id, item in R.registry().items():
            self.assertIn(R.designation(reference_id), forms)
            for language in R.LANGUAGES:
                for locator in item["locators"]:
                    self.assertIn(R.ref(reference_id, locator, language=language), forms)


class ScanRules(unittest.TestCase):
    def test_must_flag_cases_are_flagged_and_must_pass_cases_pass(self):
        forms, names = R.all_forms(), name_forms()
        for case in SCAN["mustFlag"]:
            self.assertTrue(problems_of(case["text"], case["narration"], forms, names), case["text"])
        for case in SCAN["mustPass"]:
            self.assertEqual(problems_of(case["text"], case["narration"], forms, names), [], case["text"])


class Screens(unittest.TestCase):
    def test_every_shown_designation_is_a_registry_form(self):
        found = violations()
        self.assertEqual(found, [], f"{len(found)} offenders:\n" + "\n".join(found[:400]))


if __name__ == "__main__":
    unittest.main()
