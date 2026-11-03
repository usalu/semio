"""🧪 Physics fundamentals clip: staffage casts, Beat1 device icons and the kettle and pot boil timing, validated against CoolProp."""

from __future__ import annotations

import ast
import importlib
import json
import sys
import unittest
from pathlib import Path

import numpy as np

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
ARCHITECTURE = next(p for p in Path(__file__).resolve().parents if p.name == "🏛️architecture")
CATALOG = ARCHITECTURE / "🧍️staffage" / "🗂️catalog"
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
GUARD = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only")


def tree():
    """🌳 Parse the clip script without importing it."""
    return ast.parse(SOURCE.read_text(encoding="utf-8"))


def constant(name: str):
    """📌 Literal value of a module-level assignment of the clip script."""
    for node in tree().body:
        if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == name for target in node.targets):
            return ast.literal_eval(node.value)
    raise KeyError(name)


def scene_class(name: str) -> ast.ClassDef:
    """🎬 Class definition of one scene of the clip script."""
    return next(node for node in tree().body if isinstance(node, ast.ClassDef) and node.name == name)


def narration(name: str):
    """🗣️ The ``NARRATION`` clauses of one scene as ``(key, english, german)`` tuples."""
    assignment = next(node for node in scene_class(name).body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
    return [tuple(clause) for clause in ast.literal_eval(assignment.value)]


def catalog_asset(slug: str) -> dict:
    """📦 Catalog entry of one staffage asset."""
    path = next(CATALOG.rglob(f"{slug}/🔣️.json"))
    return json.loads(path.read_text(encoding="utf-8"))["asset"]


def catalog_clip(slug: str, clip: str) -> dict:
    """🎞️ One clip of a catalog asset."""
    return next(entry for entry in catalog_asset(slug)["clips"] if entry["id"] == clip)


def phase_start(slug: str, clip: str, phase: str) -> float:
    """⏱️ Cast second at which a clip phase begins."""
    return float(next(entry for entry in catalog_clip(slug, clip)["phases"] if entry["id"] == phase)["fromS"])


def water_region(slug: str) -> dict:
    """💧 The water region of the thermal model of an asset."""
    return next(region for region in catalog_asset(slug)["thermal"]["regions"] if region["kind"] == "water")


def manifest() -> dict:
    """📜 The cast manifest of the course visuals."""
    return GUARD.manifest_casts()


def icon_calls() -> dict:
    """🖼️ Keyword arguments and slot centre of the ``_fitted_staffage`` device icons of Beat1, read from the source."""
    found = {}
    for node in ast.walk(scene_class(CASES["beat1"]["scene"])):
        if not (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "move_to"):
            continue
        inner = node.func.value
        if not (isinstance(inner, ast.Call) and isinstance(inner.func, ast.Name) and inner.func.id == "_fitted_staffage"):
            continue
        cast = inner.args[0].value
        found[cast] = {
            "kwargs": {keyword.arg: ast.literal_eval(keyword.value) for keyword in inner.keywords},
            "center": [float(value) for value in ast.literal_eval(node.args[0].args[0])][:2],
        }
    return found


def drawn_points(group) -> np.ndarray:
    """✏️ Points of every visibly drawn path of a staffage group."""
    chunks = []
    for member in group.family_members_with_points():
        visible = member.get_fill_opacity() > 0 or member.get_stroke_width() > 0
        points = member.get_all_points()
        if visible and len(points) and np.ptp(points[:, :2], axis=0).max() > 1e-9:
            chunks.append(points[:, :2])
    return np.vstack(chunks)


class StaffageCasts(unittest.TestCase):
    def test_every_cast_in_use_is_in_the_manifest_and_thermal(self):
        text = SOURCE.read_text(encoding="utf-8")
        casts = manifest()
        literal = set(GUARD.cast_names(text))
        used = GUARD.resolved_casts(text, casts)
        self.assertTrue(used)
        for name in sorted(literal | used):
            with self.subTest(cast=name):
                self.assertIn(name, casts)
                self.assertEqual(casts[name]["view"], CASES["view"])

    def test_every_physics_cast_of_the_manifest_is_used_by_the_clip(self):
        text = SOURCE.read_text(encoding="utf-8")
        owned = {name for name in manifest() if name.startswith(CASES["cast_prefix"])}
        self.assertEqual(owned - GUARD.resolved_casts(text, manifest()), set())

    def test_each_scene_draws_its_thermal_casts_on_one_span(self):
        text = SOURCE.read_text(encoding="utf-8")
        groups = GUARD.scene_spans(text, manifest())
        self.assertGreaterEqual(len(groups), 1)
        for spans in groups:
            with self.subTest(spans=spans):
                self.assertEqual(len(spans), 1)
                self.assertIsNotNone(spans[0])


class Beat1DeviceIcons(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.calls = icon_calls()
        cls.icons = {}
        for kind in CASES["beat1"]["icons"]:
            call = cls.calls[CASES["cast_prefix"] + kind]
            cls.icons[kind] = VISUALS.fitted_staffage(CASES["cast_prefix"] + kind, **call["kwargs"]).move_to(np.array([*call["center"], 0.0]))

    def test_every_icon_is_a_fitted_cast_in_the_same_slot(self):
        slot = CASES["beat1"]["slot"]
        for kind in CASES["beat1"]["icons"]:
            with self.subTest(icon=kind):
                self.assertEqual(self.calls[CASES["cast_prefix"] + kind]["center"], slot["center"])
                self.assertEqual(self.calls[CASES["cast_prefix"] + kind]["kwargs"]["rate"], 0.0)

    def test_icons_share_the_slot_centre(self):
        slot = CASES["beat1"]["slot"]
        for kind, icon in self.icons.items():
            with self.subTest(icon=kind):
                np.testing.assert_allclose(icon.get_center()[:2], slot["center"], atol=slot["tolerance"])

    def test_icons_fit_the_slot_height_and_their_declared_width(self):
        slot = CASES["beat1"]["slot"]
        for kind, icon in self.icons.items():
            kwargs = self.calls[CASES["cast_prefix"] + kind]["kwargs"]
            with self.subTest(icon=kind):
                self.assertLessEqual(icon.height, slot["height"] + slot["tolerance"])
                self.assertLessEqual(icon.width, kwargs["width"] + slot["tolerance"])
                self.assertAlmostEqual(max(icon.width / kwargs["width"], icon.height / kwargs["height"]), 1.0, places=5)

    def test_drawn_icons_stay_inside_the_device_card_while_playing(self):
        card = CASES["beat1"]["card"]
        low = np.array(card["center"]) - np.array([card["width"], card["height"]]) / 2
        high = np.array(card["center"]) + np.array([card["width"], card["height"]]) / 2
        for kind, icon in self.icons.items():
            for seconds in CASES["beat1"]["seek_seconds"]:
                with self.subTest(icon=kind, seconds=seconds):
                    icon.seek(seconds)
                    points = drawn_points(icon)
                    self.assertTrue((points.min(axis=0) >= low).all())
                    self.assertTrue((points.max(axis=0) <= high).all())


class Boiling(unittest.TestCase):
    @staticmethod
    def water_properties():
        """🧪 CoolProp second implementation of liquid water enthalpy at atmospheric pressure."""
        props = importlib.import_module("CoolProp.CoolProp").PropsSI
        water = CASES["water"]

        def enthalpy(celsius: float) -> float:
            kelvin = max(celsius, water["zero_c"]) + 273.15
            if celsius >= water["boil_c"]:
                return props("H", "T", kelvin, "Q", 0, "Water")
            return props("H", "T", kelvin, "P", water["pressure_pa"], "Water")

        return enthalpy

    def test_kettle_boils_within_the_kettle_beat_at_the_given_rate(self):
        case = CASES["kettle"]
        rate = constant(case["rate_constant"])
        boil = phase_start(case["catalog"], case["clip"], case["phase"])
        duration = float(catalog_clip(case["catalog"], case["clip"])["durationS"])
        self.assertEqual(boil, case["boil_start_s"])
        self.assertLess(boil, duration)
        VISUALS.set_vo_language("de")
        budget = max(VISUALS.narration_seconds(narration(CASES["beat1"]["scene"]), case["clause"]),
                     VISUALS.subtitle_read_seconds(narration(CASES["beat1"]["scene"]), case["clause"]))
        self.assertLess(boil / rate, budget)
        self.assertLessEqual(duration / rate, budget)
        self.assertGreater(boil / rate, 0.0)

    def test_kettle_boil_time_matches_heating_one_kilogram_with_the_element_power(self):
        case = CASES["kettle"]
        water = water_region(case["catalog"])
        enthalpy = self.water_properties()
        needed = case["water_kg"] * (enthalpy(case["end_c"]) - enthalpy(case["start_c"]))
        seconds = needed / water["operating"]["heatW"]
        self.assertAlmostEqual(seconds, phase_start(case["catalog"], case["clip"], case["phase"]), delta=case["tolerance_s"])

    def test_pot_water_region_holds_one_kilogram(self):
        case = CASES["pot"]
        water = water_region(case["catalog"])
        enthalpy = self.water_properties()
        per_kelvin = water["heatCapacityJM2K"] * water["areaM2"]
        reference = (enthalpy(CASES["water"]["boil_c"]) - enthalpy(CASES["water"]["zero_c"])) / (CASES["water"]["boil_c"] - CASES["water"]["zero_c"])
        self.assertAlmostEqual(per_kelvin / reference, case["water_kg"], delta=0.01)

    def test_pot_narrated_419_kilojoules_agree_with_coolprop(self):
        case = CASES["pot"]
        water = CASES["water"]
        enthalpy = self.water_properties()
        ours = case["water_kg"] * 4186.0 * (water["boil_c"] - 0.0) / 1000.0
        reference = case["water_kg"] * (enthalpy(water["boil_c"]) - enthalpy(0.0)) / 1000.0
        self.assertAlmostEqual(ours, case["narrated_kj"], delta=case["narrated_tolerance_kj"])
        self.assertAlmostEqual(reference, case["narrated_kj"], delta=case["narrated_tolerance_kj"])
        self.assertAlmostEqual(ours, reference, delta=case["narrated_tolerance_kj"])

    def test_pot_boil_time_lies_between_the_room_start_and_the_zero_start_energy(self):
        case = CASES["pot"]
        water = CASES["water"]
        enthalpy = self.water_properties()
        power = water_region(case["catalog"])["operating"]["heatW"]
        boil = phase_start(case["catalog"], case["clip"], case["phase"])
        self.assertEqual(boil, case["boil_start_s"])
        self.assertLessEqual(power, case["pot_input_w"])
        from_room = case["water_kg"] * (enthalpy(water["boil_c"]) - enthalpy(case["room_c"])) / power
        from_zero = case["water_kg"] * (enthalpy(water["boil_c"]) - enthalpy(0.0)) / power
        self.assertGreaterEqual(boil, from_room)
        self.assertLessEqual(boil, from_zero)

    def test_pot_boil_lands_inside_the_cast_clip_and_the_scene_rate_reproduces_it(self):
        case = CASES["pot"]
        rate = constant(case["rate_constant"])
        duration = float(catalog_clip(case["catalog"], case["clip"])["durationS"])
        boil = phase_start(case["catalog"], case["clip"], case["phase"])
        self.assertLess(boil, duration)
        self.assertAlmostEqual(boil / rate, case["scene_boil_s"], delta=case["scene_boil_tolerance_s"])


if __name__ == "__main__":
    unittest.main()
