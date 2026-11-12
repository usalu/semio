"""🧪 Tests of the shared morph primitives: tokens, travel and tether."""

from __future__ import annotations

import importlib
import time
import unittest

import numpy as np

manim = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")


def play(*start, animation):
    """▶ Play one animation in a tiny skipped scene that already holds `start`; return the scene."""
    with manim.tempconfig({"write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR", "save_last_frame": False, "progress_bar": "none"}):
        scene = manim.Scene(skip_animations=True)
        scene.add(*start)
        scene.play(animation)
    return scene


def capsule(x=0.0, color=manim.BLUE):
    """💊 A detailed multi-face stand-in for a capsule."""
    box = manim.RoundedRectangle(width=1.0, height=0.5, corner_radius=0.2, fill_color=color, fill_opacity=0.8, stroke_color=manim.WHITE)
    window = manim.Square(0.2, fill_color=manim.YELLOW, fill_opacity=0.6).shift(0.25 * manim.LEFT)
    door = manim.Rectangle(width=0.1, height=0.3, fill_color=manim.GREY, fill_opacity=0.9).shift(0.3 * manim.RIGHT)
    return manim.VGroup(box, window, door).shift(x * manim.RIGHT)


def fill_of(mob):
    """🎨 Fill colour of a mobject as RGB."""
    return np.array(manim.ManimColor(mob.get_fill_color()).to_rgb())


class TokenTests(unittest.TestCase):
    """🪙 Tokens, travel, tether and retext."""

    def test_every_kind_builds_with_size_and_colour(self):
        for kind in morph.KINDS:
            tok = morph.token("{0;1;3;1}" if kind == "path" else "42", kind=kind, color=manim.GREEN)
            self.assertEqual(tok.kind, kind)
            self.assertGreater(tok.width, 0.1)
            self.assertLess(tok.height, 1.0)
            np.testing.assert_allclose(tok.get_center(), [0, 0, 0], atol=0.2)
        with self.assertRaises(ValueError):
            morph.token("x", kind="unknown")

    def test_size_scales_tokens(self):
        self.assertGreater(morph.token("1", kind="item", size=2.0).width, 1.8 * morph.token("1", kind="item").width)

    def test_item_keeps_type_colour(self):
        np.testing.assert_allclose(fill_of(morph.token("1", kind="item", color=manim.RED)[1]), np.array(manim.ManimColor(manim.RED).to_rgb()), atol=1e-6)

    def test_travel_ends_at_path_end(self):
        tok = morph.token("1", kind="number")
        scene = play(tok, animation=morph.travel(tok, [[-3, 0, 0], [0, 1, 0], [3, 0, 0]]))
        np.testing.assert_allclose(tok.get_center(), [3, 0, 0], atol=0.05)
        self.assertEqual(scene.mobjects, [tok])

    def test_tether_follows_both_ends(self):
        a, b = manim.Square(0.5), manim.Square(0.5).shift(3 * manim.RIGHT)
        line = morph.tether(a, b)
        scene = play(a, b, line, animation=manim.Wait(0.1))
        before = line.get_start(), line.get_end()
        b.shift(2 * manim.UP)
        line.update()
        self.assertGreater(abs(line.get_end()[1] - before[1][1]), 0.5)
        np.testing.assert_allclose(line.get_start()[0], 0.25, atol=0.05)


if __name__ == "__main__":
    unittest.main()
