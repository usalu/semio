"""🪟 Caption layout of mixed prose and math: unbreakable clusters, shared baseline chunks, wrapping, and the rendered baseline."""

from __future__ import annotations

import importlib
import json
import textwrap
import unittest
from pathlib import Path

NARRATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
METRICS = CASES["metrics"]


def line_width(line):
    total = 0.0
    chunks = NARRATION.line_chunks(line)
    for index, (kind, content, gap) in enumerate(chunks):
        width = len(content) * METRICS["character"] if kind == "text" else METRICS["math"].get(content, float(len(content)))
        total += width + (METRICS["space"] if gap and index > 0 else 0.0)
    return total


def show(line):
    out = ""
    for index, (kind, content, gap) in enumerate(NARRATION.line_chunks(line)):
        out += (" " if gap and index > 0 else "") + (content if kind == "text" else "$" + content + "$")
    return out


class LayoutTest(unittest.TestCase):
    def test_wrap_keeps_formulas_whole_and_punctuation_attached(self):
        for case in CASES["wrap"]:
            with self.subTest(text=case["text"], width=case["maxWidth"]):
                lines = NARRATION.wrap(NARRATION.clusters(case["text"]), line_width, case["maxWidth"])
                self.assertEqual([show(line) for line in lines], case["lines"])

    def test_plain_prose_wraps_like_textwrap(self):
        text = "Die Wärme fließt von der warmen Seite zur kalten Seite der Wand und nimmt dabei ab"
        for width in (20, 31, 47):
            with self.subTest(width=width):
                lines = NARRATION.wrap(NARRATION.clusters(text), line_width, width)
                self.assertEqual([show(line) for line in lines], textwrap.wrap(text, width=width))

    def test_chunks_merge_text_runs_between_formulas(self):
        for case in CASES["chunks"]:
            with self.subTest(text=case["text"]):
                items = NARRATION.clusters(case["text"])
                self.assertEqual([list(chunk) for chunk in NARRATION.line_chunks(items)], case["chunks"])

    def test_read_time_counts_printed_words(self):
        for case in CASES["words"]:
            with self.subTest(text=case["text"]):
                self.assertEqual(NARRATION.caption_words(case["text"]), case["words"])


class RenderedCaptionTest(unittest.TestCase):
    def test_formula_sits_on_the_text_baseline_in_the_caption_size(self):
        text = "Treiber ist die Temperaturdifferenz $\\Delta\\theta$, innen minus außen."
        bar = VISUALS.caption_bar(text)
        label = bar[1]
        self.assertLessEqual(label.width, VISUALS.CAPTION_MAX_WIDTH + 0.05)
        baselines = [float(box.base.get_center()[1]) for box in label.submobjects if hasattr(box, "base")]
        self.assertEqual(len(label.submobjects), 4)
        self.assertEqual(len(baselines), 3)
        self.assertAlmostEqual(max(baselines) - min(baselines), 0.0, places=4)
        plain = VISUALS.caption_bar("Treiber ist die Temperaturdifferenz, innen minus außen.")
        self.assertAlmostEqual(label.height, plain[1].height, delta=0.25)

    def test_two_line_caption_with_math_stays_inside_the_frame(self):
        text = "Der Wärmestrom $\\Phi$ ist das Produkt aus $U$, $A$ und der Temperaturdifferenz $\\Delta\\theta$ zwischen innen und außen im Winter."
        bar = VISUALS.caption_bar(text)
        self.assertLessEqual(bar.width, 13.5)
        self.assertGreaterEqual(bar.get_bottom()[1], -4.0)

    def test_too_long_math_caption_fails_loudly(self):
        text = ("Dies ist ein sehr langer Satz mit einer Formel $\\Delta\\theta$ " * 4).strip()
        with self.assertRaises(ValueError):
            VISUALS.caption_bar(text)

    def test_unsupported_tex_fails_even_when_subtitles_are_hidden(self):
        with self.assertRaises(NARRATION.TexError):
            VISUALS.caption_bar("Text $\\unknown$ Ende")


if __name__ == "__main__":
    unittest.main()
