"""🌍 Caption language and reading time cases against NumPy."""

import ast
import importlib
import json
import unittest
from pathlib import Path

import numpy as np

NARRATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).parents[2] / "🐍️.py"


def functions(language):
    tree = ast.parse(SOURCE.read_text(encoding="utf-8"))
    tree.body = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in {"subtitle_text", "subtitle_read_seconds"}]
    for node in tree.body:
        node.returns = None
        for arg in node.args.args:
            arg.annotation = None
    scope = {"_narration": NARRATION, "tr": lambda en, de: en if language == "en" else de, "SUBTITLE_READING_WPS": CASES["readingWordsPerSecond"], "SUBTITLE_MIN_SECONDS": CASES["minimumSeconds"]}
    exec(compile(tree, str(SOURCE), "exec"), scope)
    return scope


class Captions(unittest.TestCase):
    """🧪 Display language selects both caption and its reading budget."""

    def test_language(self):
        for language, expected in CASES["expected"].items():
            scope = functions(language)
            self.assertEqual(scope["subtitle_text"](CASES["clauses"], "coil"), expected)
            budget = np.maximum(len(expected.split()) / CASES["readingWordsPerSecond"], CASES["minimumSeconds"])
            np.testing.assert_allclose(scope["subtitle_read_seconds"](CASES["clauses"], "coil"), np.round(budget, 2))

    def test_math_clause_keeps_tex_for_the_caption_and_counts_printed_words(self):
        for language, expected in CASES["mathExpected"].items():
            scope = functions(language)
            self.assertEqual(scope["subtitle_text"](CASES["mathClauses"], "delta"), expected)
            budget = np.maximum(CASES["mathWords"][language] / CASES["readingWordsPerSecond"], CASES["minimumSeconds"])
            np.testing.assert_allclose(scope["subtitle_read_seconds"](CASES["mathClauses"], "delta"), np.round(budget, 2))


if __name__ == "__main__":
    unittest.main()
