"""🧪 Reference chip: one top-right footnote that stays a single line when it fits and flows or scales when it does not."""

from __future__ import annotations

import importlib
import json
import unittest
from pathlib import Path

import numpy as np
from manim import DL, Rectangle, Text, UR, config

FONTS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️reference_chip.json").read_text(encoding="utf-8"))
SIZE = FONTS.LABEL_FONT_SIZE - 3


def title_ending_at(right: float):
    return Rectangle(width=2.0, height=CASES["flowing"]["title_height"]).move_to([right - 1.0, 0, 0])


def lines_of(chip):
    return list(chip.submobjects)


class ReferenceChipTest(unittest.TestCase):
    def setUp(self):
        FONTS.apply_body_font()

    def test_a_fitting_text_is_the_plain_label_in_the_corner(self):
        for text in CASES["single_line"]:
            with self.subTest(text=text):
                expected = Text(text, font=FONTS.BODY_FONT, font_size=SIZE, color=VISUALS.P_PRIMARY, disable_ligatures=True)
                expected.set_opacity(CASES["opacity"]).to_corner(UR, buff=CASES["margin"])
                chip = VISUALS.reference_chip(text)
                self.assertEqual(len(lines_of(chip)), 1)
                np.testing.assert_allclose(chip.get_corner(UR), expected.get_corner(UR), atol=1e-9)
                np.testing.assert_allclose(chip.get_corner(DL), expected.get_corner(DL), atol=1e-9)
                self.assertAlmostEqual(chip.width, expected.width, places=9)
                self.assertAlmostEqual(chip.height, expected.height, places=9)
                self.assertAlmostEqual(chip.get_stroke_opacity(), expected.get_stroke_opacity(), places=9)
                self.assertAlmostEqual(chip.get_fill_opacity(), expected.get_fill_opacity(), places=9)
                self.assertEqual(chip[0].get_color().to_hex(), expected.get_color().to_hex())

    def test_lower_moves_the_corner_chip_straight_down(self):
        text = CASES["single_line"][0]
        base, lowered = VISUALS.reference_chip(text), VISUALS.reference_chip(text, lower=CASES["lower"])
        np.testing.assert_allclose(lowered.get_center() - base.get_center(), [0, -CASES["lower"], 0], atol=1e-9)

    def test_a_long_text_flows_into_right_aligned_lines_that_stay_right_of_the_title(self):
        case = CASES["flowing"]
        title = title_ending_at(case["title_right"])
        chip = VISUALS.reference_chip(case["text"], title)
        room = config.frame_width / 2 - CASES["margin"] - (title.get_right()[0] + CASES["margin"])
        lines = lines_of(chip)
        self.assertGreaterEqual(len(lines), case["min_lines"])
        self.assertLessEqual(chip.width, room + 1e-9)
        self.assertGreaterEqual(chip.get_left()[0], title.get_right()[0] + CASES["margin"] - 1e-9)
        self.assertAlmostEqual(chip.get_right()[0], config.frame_width / 2 - CASES["margin"], places=9)
        for line in lines:
            self.assertAlmostEqual(line.get_right()[0], chip.get_right()[0], places=9)
        for upper, lower in zip(lines, lines[1:]):
            self.assertAlmostEqual(upper.get_bottom()[1] - lower.get_top()[1], CASES["line_gap"], places=9)

    def test_flowing_keeps_every_part_in_order_and_drops_no_text(self):
        case = CASES["flowing"]
        chip = VISUALS.reference_chip(case["text"], title_ending_at(case["title_right"]))
        parts = case["text"].replace("\n", " · ").split(" · ")
        flowed = " · ".join(line.original_text for line in lines_of(chip)).split(" · ")
        self.assertEqual(flowed, parts)

    def test_a_single_part_wider_than_the_room_is_scaled_down_to_it(self):
        chip = VISUALS.reference_chip(CASES["over_wide"], title_ending_at(CASES["flowing"]["title_right"]))
        room = config.frame_width / 2 - 2 * CASES["margin"] - CASES["flowing"]["title_right"]
        self.assertAlmostEqual(chip.width, room, places=9)

    def test_without_a_title_the_room_is_the_default_width(self):
        chip = VISUALS.reference_chip(CASES["over_wide"])
        self.assertLessEqual(chip.width, CASES["default_room"] + 1e-9)


if __name__ == "__main__":
    unittest.main()
