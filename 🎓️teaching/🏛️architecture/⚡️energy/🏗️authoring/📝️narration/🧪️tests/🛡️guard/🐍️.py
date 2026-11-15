"""🛡️ Narration guard: every clause of every energy clip scene writes symbols as speakable `$…$` TeX, never as spelled words.

Loads the clips of the five energy videos (scripts under ``🏛️architecture/⚡️energy``) from the manifest ``⚡️energy/🎬️clip/🔣️.json`` and reports EVERY offender as ``file:line``.
Filter by script path with ``NARRATION_GUARD_FILTER=heating`` (comma separated substrings).
"""

from __future__ import annotations

import ast
import importlib
import json
import os
import unittest
from pathlib import Path

NARRATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
TEX = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧮️tex.🐍️")
LEXICON = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🗣️lexicon.🐍️")
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
HERE = Path(__file__).resolve()
WORKSPACE = next(parent for parent in HERE.parents if (parent / "🧰️framework").is_dir())
TEACHING = WORKSPACE / "🎓️teaching"
MANIFEST = TEACHING / "🏛️architecture" / "⚡️energy" / "🎬️clip" / "🔣️.json"
FILTER = [part.strip() for part in os.environ.get("NARRATION_GUARD_FILTER", "").split(",") if part.strip()]
LANGUAGES = {"en": 1, "de": 2}
ENERGY = "🏛️architecture/⚡️energy/"


def manifest_scenes() -> list[tuple[str, str]]:
    """🎞️ Unique ``(script, scene)`` pairs of every energy video, in manifest order, after the optional filter."""
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    seen: dict[tuple[str, str], None] = {}
    for video in data["videos"]:
        for chapter in video["chapters"]:
            for segment in chapter["segments"]:
                if segment["script"].startswith(ENERGY):
                    seen.setdefault((segment["script"], segment["scene"]), None)
    return [pair for pair in seen if not FILTER or any(part in pair[0] for part in FILTER)]


def narration_of(script: str, scene: str) -> tuple[list[tuple[str, str, str, int, int, int]], str | None]:
    """📜 ``(key, en, de, line_of_clause, line_of_en, line_of_de)`` rows of one scene's ``NARRATION`` and an error text if it cannot be read."""
    path = TEACHING / script
    try:
        tree = ast.parse(path.read_text(encoding="utf-8"))
    except (OSError, SyntaxError) as error:
        return [], f"cannot read {error}"
    cls = next((node for node in ast.walk(tree) if isinstance(node, ast.ClassDef) and node.name == scene), None)
    if cls is None:
        return [], f"scene class {scene} not found"
    assignment = next((node for node in cls.body if (isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
                       or (isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and node.target.id == "NARRATION")), None)
    if assignment is None:
        return [], "no NARRATION"
    try:
        rows = ast.literal_eval(assignment.value)
        nodes = assignment.value.elts
        return [(row[0], row[1], row[2], node.lineno, node.elts[1].lineno, node.elts[2].lineno) for row, node in zip(rows, nodes)], None
    except (ValueError, AttributeError, IndexError):
        module = "🎓️teaching." + Path(script).with_suffix("").as_posix().replace("/", ".")
        try:
            rows = list(getattr(importlib.import_module(module), scene).NARRATION)
        except Exception as error:
            return [], f"NARRATION is computed and the module does not import: {type(error).__name__}: {error}"
        return [(row[0], row[1], row[2], assignment.lineno, assignment.lineno, assignment.lineno) for row in rows], None


def offenders_of(rows, where: str) -> list[str]:
    """🚨 Every problem of the given rows as ``where:line [key lang] message``."""
    problems: list[str] = []
    for key, text_en, text_de, clause_line, line_en, line_de in rows:
        for language, text, line in (("en", text_en, line_en), ("de", text_de, line_de)):
            prefix = f"{where}:{line} [{key} {language}]"
            try:
                segments = TEX.split_segments(text)
            except TEX.TexError as error:
                problems.append(f"{prefix} {error}")
                continue
            for kind, content in segments:
                if kind != "math":
                    continue
                for spoken_language in LANGUAGES:
                    try:
                        LEXICON.spoken_math(content, spoken_language)
                    except LEXICON.SpeechError as error:
                        problems.append(f"{prefix} not speakable in {spoken_language}: {error}")
                try:
                    TEX.math_to_unicode(content)
                    VISUALS._math_parse_items(content)
                except (TEX.TexError, ValueError) as error:
                    problems.append(f"{prefix} not displayable: {error}")
            for rule, match, hint in LEXICON.forbidden_findings(text):
                problems.append(f"{prefix} spelled symbol {match!r} ({rule}): {hint}")
    return problems


class GuardMechanicsTest(unittest.TestCase):
    def test_flags_spelled_symbols_unicode_letters_and_unspeakable_tex(self):
        rows = [
            ("a", "The delta theta here", "Die Temperaturdifferenz Delta-Theta hier", 10, 11, 12),
            ("b", "Value Δθ", "Wert $\\Delta\\theta$ und Q-Punkt", 20, 21, 22),
            ("c", "Fine $\\Delta\\theta$", "Gut $\\Delta\\theta$", 30, 31, 32),
            ("d", "Bad $\\unknownmacro$", "Schlecht $\\foo$", 40, 41, 42),
        ]
        problems = offenders_of(rows, "file.py")
        joined = "\n".join(problems)
        self.assertIn("file.py:11 [a en]", joined)
        self.assertIn("file.py:12 [a de]", joined)
        self.assertIn("file.py:21 [b en]", joined)
        self.assertIn("file.py:22 [b de]", joined)
        self.assertNotIn("[c ", joined)
        self.assertIn("file.py:41 [d en]", joined)
        self.assertIn("file.py:42 [d de]", joined)


class EnergyNarrationGuardTest(unittest.TestCase):
    def test_manifest_lists_every_energy_scene(self):
        scenes = manifest_scenes()
        if not FILTER:
            self.assertGreater(len(scenes), 50)

    def test_all_clips_write_symbols_as_speakable_tex(self):
        problems: list[str] = []
        clauses = 0
        for script, scene in manifest_scenes():
            rows, error = narration_of(script, scene)
            where = f"{script}"
            if error:
                problems.append(f"{where}:1 [{scene}] {error}")
                continue
            clauses += len(rows)
            problems += offenders_of(rows, where)
        if problems:
            self.fail(f"{len(problems)} narration offender(s) in {clauses} clauses:\n" + "\n".join(sorted(problems)))
        if not FILTER:
            self.assertGreater(clauses, 500)


if __name__ == "__main__":
    unittest.main()
