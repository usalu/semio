"""🧪 Check the real air-change house transition against Manim's interpolation oracle."""
from __future__ import annotations

import ast
import copy
import importlib
import json
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class AirChangeTransitionTest(unittest.TestCase):
    """🔄 Freeze live section topology before its copied geometry morphs onto the scale."""

    def setUp(self):
        """📦 Keep all generated geometry assets in the caller's declared output root."""
        from manim import tempconfig

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        self.config = tempconfig({"media_dir": str(output / "media"), "text_dir": str(output / "text")})
        self.config.__enter__()
        self.addCleanup(self.config.__exit__, None, None, None)
        self.module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🐍️")

    def house(self):
        """🏠 Build the authored section assembly and frozen full-air fill."""
        import numpy as np
        from manim import Dot, VGroup

        case, module = CASES["house"], self.module
        origin = np.array(case["origin"], dtype=float)
        physical = module._pf_house(module.HOUSE, origin, case["scale"])
        fill = module._poly(module.HOUSE["outer"], origin, case["scale"], stroke_width=0,
                            fill_color=module.P_BLUE, fill_opacity=0.45)
        return VGroup(physical["group"], fill, Dot()).scale_to_fit_height(case["height"]).move_to(case["center"])

    def drive(self, animation):
        """🎞️ Exercise real copied-mobject updates and NumPy point interpolation."""
        from manim.animation.animation import prepare_animation

        animation = prepare_animation(animation)
        animation.begin()
        for alpha in CASES["interpolation"]:
            animation.update_mobjects(0.25)
            animation.interpolate(alpha)
        animation.finish()

    def test_dynamic_topology_regression_and_frozen_third_party_oracle(self):
        """📐 Reproduce the original failure and compare the frozen result with the target."""
        import numpy as np
        from manim import Dot, ReplacementTransform

        with self.assertRaisesRegex(ValueError, CASES["regressionError"]):
            self.drive(ReplacementTransform(self.house(), Dot(CASES["target"])))
        frozen = self.house()
        self.assertGreaterEqual(sum(len(member.updaters) for member in frozen.get_family()), CASES["minimumDynamicUpdaters"])
        frozen.clear_updaters()
        self.assertFalse(any(member.updaters for member in frozen.get_family()))
        target = Dot(CASES["target"])
        self.drive(ReplacementTransform(frozen, target))
        np.testing.assert_allclose(frozen.get_center(), target.get_center(), atol=1e-8)

    def test_authored_show_use_freezes_descendants_before_actual_morph(self):
        """🔬 Execute the production nested transition rather than restating its fix."""
        import numpy as np
        from manim import Dot, Line, VGroup, ValueTracker

        tree = ast.parse(Path(self.module.__file__).read_text(encoding="utf-8"))
        beat = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == CASES["scene"])
        construct = next(node for node in beat.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
        transition = next(node for node in construct.body if isinstance(node, ast.FunctionDef) and node.name == CASES["transition"])
        extracted = ast.fix_missing_locations(ast.Module(body=[copy.deepcopy(transition)], type_ignores=[]))
        drive = self.drive

        class Driver:
            """🎭 Drive the authored play boundary with real Manim animation objects."""

            def play(self, *animations, **kwargs):
                """▶️ Interpolate each submitted native animation at the portable sample times."""
                for animation in animations:
                    drive(animation)

            def add(self, *objects):
                """➕ Accept the readout insertion without rendering an unrelated scene."""
                return self

        glyph = self.house()
        namespace = {**vars(self.module), "self": Driver(), "house": glyph, "count": ValueTracker(0),
                     "minutes": ValueTracker(0), "stage": np.array(CASES["house"]["center"]),
                     "stage_label": lambda *args: (Dot(), Dot()), "dots": [Dot(CASES["target"])],
                     "labels": [VGroup(Dot([1.2, 0, 0]))], "leaders": [Line(CASES["target"], [1.2, 0, 0])]}
        exec(compile(extracted, str(self.module.__file__), "exec"), namespace)
        namespace[CASES["transition"]](glyph, "home", self.module.N_RATE, self.module.P_ORANGE, 0)
        self.assertFalse(any(member.updaters for member in glyph.get_family()))


if __name__ == "__main__":
    unittest.main()
