"""🧪 Portable optimization teaching contracts with independent NumPy and SciPy checks."""

import ast
import importlib.util
import itertools
import json
import math
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
CLIP = Path(__file__).resolve().parent.parent
ROOT = next(parent for parent in CLIP.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
KEYS = ("orientation", "wwr", "overhang", "insulation")


def model_api():
    """🧮 Import teaching mathematics without drawing or speech dependencies."""
    spec = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model/🐍️.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def narration(chapter):
    """🗣️ Read the language-neutral scene's authored bilingual clauses."""
    tree = ast.parse((CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8"))
    scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
    return next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign)
                and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))


def reference_evaluate(study, designs):
    """🔭 Vectorized NumPy re-implementation of the annual model for many designs at once."""
    import numpy as np

    p = {section: {name: item["value"] for name, item in items.items()} for section, items in study["parameters"].items()}
    theta, wwr, depth, thickness = np.asarray(designs, dtype=float).T
    phi = np.radians(theta - 180)
    room, climate, envelope, daylight = (p[name] for name in ("room", "climate", "envelope", "daylight"))
    facade, floor = room["width"] * room["height"], room["width"] * room["depth"]
    window, wall = wwr * facade, facade * (1 - wwr)
    u_wall = 1 / (envelope["r_si"] + envelope["r_se"] + envelope["r_masonry"] + thickness / envelope["conductivity_insulation"])
    conductance = u_wall * wall + envelope["u_window"] * window + envelope["h_other"]
    irradiance = lambda season: np.maximum(0, climate[f"{season}_irradiance_mean"] + climate[f"{season}_irradiance_cos1"] * np.cos(phi)
                                           + climate[f"{season}_irradiance_cos2"] * np.cos(2 * phi))
    gap = room["height"] - room["head"]
    height = np.minimum(wwr * room["height"], room["head"] - 0.05)
    sun = study["sun"]

    def shade(season):
        delta = np.radians(sun["declination"] if season == "summer" else -sun["declination"])
        angle = np.radians(15.0 * np.arange(-sun["hours"], sun["hours"] + 1))
        latitude = np.radians(sun["latitude"])
        altitude = np.degrees(np.arcsin(np.sin(latitude) * np.sin(delta) + np.cos(latitude) * np.cos(delta) * np.cos(angle)))
        bearing = (180 + np.degrees(np.arctan2(np.sin(angle), np.cos(angle) * np.sin(latitude) - np.tan(delta) * np.cos(latitude)))) % 360
        across = np.abs((bearing[None, :] - theta[:, None] + 180) % 360 - 180)
        valid = (altitude[None, :] >= sun["minimum_altitude"]) & (across < 90)
        profile = np.minimum(85.0, np.degrees(np.arctan(np.tan(np.radians(altitude))[None, :] / np.cos(np.radians(across)))))
        share = np.clip((depth[:, None] * np.tan(np.radians(profile)) - gap) / height[:, None], 0, 1)
        weight = np.where(valid, np.cos(np.radians(altitude))[None, :] * np.cos(np.radians(across)), 0.0)
        total = weight.sum(axis=1)
        return total, np.where(depth > 0, (weight * share).sum(axis=1), 0.0) / np.maximum(total, climate["beam_reference"])

    beams = {season: shade(season) for season in ("winter", "summer")}
    beam, shading = ({season: beams[season][index] for season in beams} for index in (0, 1))
    direct = lambda season: climate[f"direct_share_{season}"]
    solar = {season: window * envelope["g_value"] * irradiance(season) * (1 - direct(season) * shading[season]) for season in ("winter", "summer")}


    def utilization(ratio, exponent):
        """🎛️ Vectorized utilization factor with the ratio-one limit."""
        close = np.isclose(ratio, 1.0, atol=1e-9)
        safe = np.where(close, 0.5, ratio)
        return np.where(close, exponent / (exponent + 1), (1 - safe ** exponent) / (1 - safe ** (exponent + 1)))

    losses = conductance * climate["heating_degree_hours"] / 1000
    gains = solar["winter"] + climate["internal_gains_winter"]
    heating_eta = utilization(gains / losses, climate["utilization_exponent_heating"])
    heating_demand = np.maximum(0, losses - heating_eta * gains)
    removable = conductance * climate["cooling_degree_hours"] / 1000
    heat = solar["summer"] + climate["internal_gains_summer"]
    cooling_eta = utilization(removable / heat, climate["utilization_exponent_cooling"])
    cooling_demand = np.maximum(0, heat - cooling_eta * removable)
    angle = daylight["sky_angle"] * (1 - daylight["sky_angle_loss"] * depth ** 2 / (depth ** 2 + daylight["overhang_half_depth"] ** 2))
    factor = daylight["transmittance"] * window * angle / (daylight["surface_area"] * (1 - daylight["reflectance"] ** 2))
    autonomy = daylight["max_autonomy"] * (1 - np.exp(-factor * (1 + daylight["orientation_gain"] * np.cos(phi)) / daylight["reference_factor"]))
    heating, cooling = heating_demand / p["systems"]["cop"], cooling_demand / p["systems"]["eer"]
    lighting = p["lighting"]["power_density"] * floor * p["lighting"]["hours"] * (1 - autonomy) / 1000
    prices, carbon = p["prices"], p["carbon"]
    lamps = 1 - autonomy
    return {"heating": heating, "cooling": cooling, "lighting": lighting, "electricity": heating + cooling + lighting, "daylight": factor,
            "solar_winter": solar["winter"], "solar_summer": solar["summer"], "shading_winter": shading["winter"], "shading_summer": shading["summer"],
            "conductance": conductance, "heating_losses": losses, "heating_gains": gains, "heating_eta": heating_eta, "heating_used": heating_eta * gains,
            "cooling_gains": heat, "cooling_removable": removable, "cooling_eta": cooling_eta, "cooling_removed": cooling_eta * removable,
            "beam_winter": beam["winter"], "beam_summer": beam["summer"],
            "direct_winter": direct("winter") * np.minimum(1, beam["winter"] / climate["beam_reference"]),
            "direct_summer": direct("summer") * np.minimum(1, beam["summer"] / climate["beam_reference"]),
            "lighting_power": p["lighting"]["power_density"] * floor / 1000 + 0 * theta, "lamps_on": lamps,
            "cost": prices["fixed"] + wall * (prices["wall"] + prices["insulation_per_cm"] * 100 * thickness) + window * prices["window"] + prices["overhang"] * room["width"] * depth,
            "carbon": wall * (carbon["wall"] + carbon["insulation_per_cm"] * 100 * thickness) + window * carbon["window"] + carbon["overhang"] * room["width"] * depth}


def dominance(*columns):
    """🥊 Boolean matrix whose entry [i, j] says design i dominates design j (all columns minimized)."""
    import numpy as np

    values = np.column_stack(columns)
    return np.all(values[:, None, :] <= values[None, :, :], axis=2) & np.any(values[:, None, :] < values[None, :, :], axis=2)


class StudyModel(unittest.TestCase):
    """🏛️ The seminar room model against an independent NumPy implementation and plausibility bounds."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.study = cls.api.load_study()

    def test_evaluate_matches_a_vectorized_numpy_reimplementation(self):
        import numpy as np

        generator = np.random.default_rng(20261027)
        designs = np.column_stack((generator.uniform(90, 450, 300), generator.uniform(.1, 1, 300), generator.uniform(0, 1.5, 300), generator.uniform(.1, .3, 300)))
        designs[:20, 2] = 0
        reference = reference_evaluate(self.study, designs)
        for key, expected in reference.items():
            actual = [self.api.evaluate(dict(zip(KEYS, row)))[key] for row in designs]
            np.testing.assert_allclose(actual, expected, rtol=1e-9, err_msg=key)

    def test_evaluate_reports_every_documented_quantity_consistently(self):
        import numpy as np

        result = self.api.evaluate()
        self.assertEqual(result, self.api.evaluate(self.study["baseline"]))
        for key in ("window_area", "wall_area", "u_wall", "heating_demand", "cooling_demand", "heating", "cooling", "lighting", "electricity", "intensity",
                    "solar_winter", "solar_summer", "shading_winter", "shading_summer", "sun_winter", "sun_summer", "daylight", "autonomy", "cost", "carbon",
                    *CASES["baseline"]):
            self.assertIn(key, result)
        self.assertAlmostEqual(result["window_area"] + result["wall_area"], 24)
        self.assertAlmostEqual(result["window_area"], .45 * 24)
        self.assertAlmostEqual(result["electricity"], result["heating"] + result["cooling"] + result["lighting"])
        self.assertAlmostEqual(result["intensity"], result["electricity"] / 48)
        self.assertAlmostEqual(result["u_wall"], 1 / (.13 + .04 + .45 + .16 / .035))
        self.assertAlmostEqual(result["heating"] * 3.5, result["heating_demand"])
        self.assertAlmostEqual(result["cooling"] * 3.0, result["cooling_demand"])
        for key, expected in CASES["baseline"].items():
            self.assertAlmostEqual(result[key], expected, places=4, msg=key)
        for key in ("shading_winter", "shading_summer", "sun_winter", "sun_summer", "autonomy"):
            self.assertTrue(0 <= result[key] <= 1, key)
        self.assertEqual(self.api.evaluate({"orientation": 180})["sun_winter"], 1.0)
        self.assertLess(self.api.evaluate({"orientation": 360})["sun_winter"], .3)
        self.assertGreater(self.api.evaluate({"orientation": 90})["sun_summer"], self.api.evaluate({"orientation": 180})["sun_summer"])
        np.testing.assert_allclose(self.api.evaluate({"orientation": 450})["electricity"], self.api.evaluate({"orientation": 90})["electricity"])

    def test_numbers_are_plausible_for_a_swiss_seminar_room(self):
        ranges = CASES["ranges"]
        designs = [self.api.evaluate(), *self.api.variants(extra=True), *self.api.pareto()["designs"], *self.api.samples("grid"), *self.api.samples("lhs")]
        for item in designs:
            with self.subTest(electricity=item["electricity"]):
                self.assertTrue(ranges["electricity"][0] <= item["electricity"] <= ranges["electricity"][1])
                self.assertTrue(ranges["intensity"][0] <= item["electricity"] / 48 <= ranges["intensity"][1])
                self.assertTrue(ranges["cost"][0] <= item["cost"] <= ranges["cost"][1])
        for angle in range(90, 451, 10):
            electricity = self.api.evaluate({**self.api.orientation_study()["design"], "orientation": angle})["electricity"]
            self.assertTrue(ranges["electricity"][0] <= electricity <= ranges["electricity"][1])

    def test_invalid_designs_are_rejected_and_unknown_keys_refused(self):
        for design in ({"wwr": 0}, {"wwr": 1.2}, {"overhang": -.1}, {"insulation": 0}, {"wwr": float("nan")}, {"colour": 1}):
            with self.subTest(design=design), self.assertRaises(ValueError):
                self.api.evaluate(design)

    def test_study_documents_every_coefficient_in_both_languages(self):
        count = 0
        for section, items in self.study["parameters"].items():
            for name, item in items.items():
                count += 1
                with self.subTest(parameter=f"{section}.{name}"):
                    self.assertTrue(item["unit"].strip())
                    self.assertTrue(item["note"]["en"].strip() and item["note"]["de"].strip())
                    self.assertNotEqual(item["note"]["en"], item["note"]["de"])
        self.assertGreaterEqual(count, 50)
        self.assertIsNot(self.api.load_study(), self.api.load_study())

    def test_shading_is_the_geometric_share_of_the_window_height_in_the_overhang_shadow(self):
        import numpy as np

        gap, head = .3, 2.7
        samples = self.api.sun_samples(180, "summer")
        self.assertEqual([item["hour"] for item in samples], list(range(-4, 5)))
        self.assertAlmostEqual(samples[4]["altitude"], self.study["sun"]["summer_altitude"], delta=.01)
        self.assertAlmostEqual(samples[4]["profile"], samples[4]["altitude"])
        for wwr, depth in itertools.product((.2, .45, .8), (.3, .6, 1.2)):
            height = 3 * wwr
            weights = np.array([item["weight"] for item in samples])
            shares = np.clip((depth * np.tan(np.radians([item["profile"] for item in samples])) - gap) / height, 0, 1)
            result = self.api.evaluate({"wwr": wwr, "overhang": depth})
            self.assertAlmostEqual(result["shading_summer"], float((weights * shares).sum() / weights.sum()))
        noon = self.api.sun_state(180, "summer")
        self.assertAlmostEqual(noon["profile"], 66.04, delta=.01)
        self.assertEqual(self.api.evaluate({"overhang": .6})["shading_winter"], 0)
        self.assertLess(self.api.evaluate({"overhang": 1.2, "wwr": .45})["shading_winter"], .1)
        self.assertGreater(self.api.evaluate({"overhang": .6, "wwr": .45})["shading_summer"], .5)
        depths = [self.api.evaluate({"overhang": depth})["shading_summer"] for depth in np.linspace(0, 1.2, 25)]
        self.assertTrue(all(a <= b for a, b in zip(depths, depths[1:])))
        self.assertEqual(depths[0], 0)
        self.assertAlmostEqual(depths[-1], 1.0, delta=.02)
        heights = [self.api.evaluate({"overhang": .6, "wwr": wwr})["shading_summer"] for wwr in np.linspace(.2, .8, 13)]
        self.assertTrue(all(a >= b for a, b in zip(heights, heights[1:])))
        self.assertEqual(self.api.evaluate({"overhang": 0})["shading_summer"], 0)

    def test_winter_sun_stays_under_a_short_overhang_while_summer_sun_is_blocked(self):
        winter = self.api.sun_state(180, "winter")
        summer = self.api.sun_state(180, "summer")
        self.assertAlmostEqual(winter["altitude"], 19.16, delta=.01)
        self.assertLess(.6 * math.tan(math.radians(winter["profile"])), .3)
        self.assertGreater(.6 * math.tan(math.radians(summer["profile"])), 1.0)
        behind = self.api.sun_state(360, "winter")
        self.assertFalse(behind["front"])
        self.assertEqual(self.api.sun_samples(360, "winter"), [])
        self.assertEqual(self.api.evaluate({"orientation": 360, "overhang": 1.2})["shading_winter"], 0)
        sun = self.study["sun"]
        for season in ("winter", "summer"):
            track = self.api._sun_track(season)
            self.assertAlmostEqual(max(altitude for _, altitude in track), sun[f"{season}_altitude"], delta=.01)
            self.assertAlmostEqual(min(bearing for bearing, _ in track), sun[f"{season}_azimuth"][0], delta=.1)
            self.assertAlmostEqual(max(bearing for bearing, _ in track), sun[f"{season}_azimuth"][1], delta=.1)

    def test_overhang_shades_most_in_summer_on_the_sun_facing_side(self):
        south = self.api.evaluate({"orientation": 180, "overhang": .6})
        east = self.api.evaluate({"orientation": 90, "overhang": .6})
        self.assertGreater(south["shading_summer"], south["shading_winter"])
        self.assertGreater(south["shading_summer"], east["shading_summer"])
        self.assertEqual(self.api.evaluate({"overhang": 0})["shading_summer"], 0)
        self.assertLess(self.api.evaluate({"overhang": 1.2})["solar_summer"], self.api.evaluate({"overhang": 0})["solar_summer"])

    def test_energy_balance_terms_close_each_demand_and_each_conversion(self):
        import numpy as np

        generator = np.random.default_rng(20261028)
        rows = np.column_stack((generator.uniform(90, 450, 60), generator.uniform(.1, 1, 60), generator.uniform(0, 1.5, 60), generator.uniform(.1, .3, 60)))
        for row in rows:
            result = self.api.evaluate(dict(zip(KEYS, row)))
            with self.subTest(design=row.tolist()):
                self.assertAlmostEqual(result["heating_gains"], result["solar_winter"] + result["internal_winter"])
                self.assertAlmostEqual(result["heating_losses"], result["conductance"] * 70)
                self.assertAlmostEqual(result["heating_used"], result["heating_eta"] * result["heating_gains"])
                self.assertAlmostEqual(result["heating_demand"], max(0.0, result["heating_losses"] - result["heating_used"]))
                self.assertAlmostEqual(result["heating"] * result["cop"], result["heating_demand"])
                self.assertAlmostEqual(result["cooling_gains"], result["solar_summer"] + result["internal_summer"])
                self.assertAlmostEqual(result["cooling_removable"], result["conductance"] * 8)
                self.assertAlmostEqual(result["cooling_removed"], result["cooling_eta"] * result["cooling_removable"])
                self.assertAlmostEqual(result["cooling_demand"], max(0.0, result["cooling_gains"] - result["cooling_removed"]))
                self.assertAlmostEqual(result["cooling"] * result["eer"], result["cooling_demand"])
                self.assertAlmostEqual(result["lighting"], result["lighting_power"] * result["lighting_hours"] * result["lamps_on"])
                self.assertAlmostEqual(result["lamps_on"], 1 - result["autonomy"])
                self.assertTrue(0 < result["heating_eta"] <= 1 and 0 < result["cooling_eta"] <= 1)
                self.assertTrue(0 <= result["direct_winter"] <= .5 and 0 <= result["direct_summer"] <= .6)
        self.assertEqual((self.api.evaluate()["cop"], self.api.evaluate()["eer"]), (3.5, 3.0))

    def test_direct_gain_fades_with_the_beam_and_the_diffuse_rest_never_jumps(self):
        grazing = CASES["continuity"]["grazing"]
        design = grazing["design"]
        for theta, shading, cooling, electricity in zip(grazing["bearings"], grazing["shading_summer"], grazing["cooling"], grazing["electricity"], strict=True):
            result = self.api.evaluate({**design, "orientation": theta})
            self.assertAlmostEqual(result["shading_summer"], shading, places=5, msg=theta)
            self.assertAlmostEqual(result["cooling"], cooling, places=3, msg=theta)
            self.assertAlmostEqual(result["electricity"], electricity, places=3, msg=theta)
        last = self.api.sun_samples(354, "summer")
        self.assertEqual([item["hour"] for item in last], [grazing["last_hour"]["hour"]])
        self.assertAlmostEqual(last[0]["across"], grazing["last_hour"]["across"], places=5)
        self.assertAlmostEqual(last[0]["weight"], grazing["last_hour"]["weight"], places=5)
        self.assertEqual(self.api.sun_samples(355, "summer"), [])
        reach = self.api.evaluate({**design, "orientation": 354})
        self.assertAlmostEqual(reach["direct_summer"], grazing["last_hour"]["direct_summer"], places=5)
        self.assertLess(reach["shading_summer"], reach["direct_summer"] / self.study["parameters"]["climate"]["direct_share_summer"]["value"] + 1e-12)
        self.assertLess(abs(reach["cooling"] - self.api.evaluate({**design, "orientation": 355})["cooling"]), 4)
        fade = [self.api.evaluate({**design, "orientation": 350 + step / 10})["shading_summer"] for step in range(52)]
        self.assertTrue(all(a >= b - 1e-12 for a, b in zip(fade, fade[1:])))
        self.assertEqual(fade[-1], 0)
        north = self.api.evaluate({**design, "orientation": 360, "overhang": 1.2})
        bare = self.api.evaluate({**design, "orientation": 360, "overhang": 0})
        self.assertEqual((north["beam_summer"], north["direct_summer"], north["shading_summer"]), (0, 0, 0))
        self.assertEqual(north["solar_summer"], bare["solar_summer"])
        self.assertEqual(north["solar_winter"], bare["solar_winter"])

    def test_the_reference_beam_leaves_every_well_lit_façade_unchanged(self):
        climate = self.study["parameters"]["climate"]
        south = self.api.evaluate({"orientation": 180, "overhang": .6})
        for season in ("winter", "summer"):
            self.assertGreater(south[f"beam_{season}"], 4 * climate["beam_reference"]["value"])
            self.assertEqual(south[f"direct_{season}"], climate[f"direct_share_{season}"]["value"])
            self.assertAlmostEqual(south[f"beam_{season}"], sum(item["weight"] for item in self.api.sun_samples(180, season)))


class BearingContinuity(unittest.TestCase):
    """🌀 Heating, cooling, lighting and total electricity are smooth curves of the façade bearing for every overhang and window share."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.study = cls.api.load_study()
        cls.setup = CASES["continuity"]

    def curves(self, step, wwr, depth):
        import numpy as np

        bearings = self.setup["bearings"]
        theta = np.arange(bearings["from"], bearings["to"] + step / 2, step)
        designs = np.column_stack((theta, np.full_like(theta, wwr), np.full_like(theta, depth), np.full_like(theta, self.setup["insulation"])))
        return reference_evaluate(self.study, designs)

    def test_no_curve_has_a_step_above_the_smooth_bound_and_steps_scale_with_the_bearing_step(self):
        import numpy as np

        largest = dict.fromkeys(self.setup["step_bound"], 0.0)
        for wwr, depth in itertools.product(self.setup["wwr"], self.setup["overhang"]):
            coarse, fine = self.curves(self.setup["bearings"]["step"], wwr, depth), self.curves(self.setup["bearings"]["fine_step"], wwr, depth)
            for key, bound in self.setup["step_bound"].items():
                step, finer = np.abs(np.diff(coarse[key])).max(), np.abs(np.diff(fine[key])).max()
                largest[key] = max(largest[key], step)
                with self.subTest(wwr=wwr, overhang=depth, quantity=key):
                    self.assertLessEqual(step, bound)
                    self.assertLessEqual(finer, step / self.setup["scaling_minimum"] + 1e-9)
        for key, expected in self.setup["max_step_found"].items():
            self.assertAlmostEqual(largest[key], expected, places=4, msg=key)
            self.assertLessEqual(largest[key], self.setup["before_fix_max_step"][key] + 1e-5)
        self.assertGreater(self.setup["before_fix_max_step"]["cooling"], 50 * self.setup["step_bound"]["cooling"])

    def test_the_public_function_follows_the_reference_across_the_grazing_bearings(self):
        import numpy as np

        bearings = np.array([300.0, 309.75, 310.0, 345.0, 350.0, 353.9, 354.0, 354.1, 354.9, 355.0, 355.1, 365.0, 365.1, 366.0, 380.0, 420.0, 289.5])
        designs = np.column_stack((bearings, np.full(bearings.shape, .8), np.full(bearings.shape, 1.2), np.full(bearings.shape, .16)))
        reference = reference_evaluate(self.study, designs)
        for key in ("heating", "cooling", "lighting", "electricity", "shading_summer", "shading_winter", "beam_summer", "direct_summer"):
            actual = [self.api.evaluate(dict(zip(KEYS, row)))[key] for row in designs]
            np.testing.assert_allclose(actual, reference[key], rtol=1e-9, atol=1e-12, err_msg=key)

    def test_the_public_function_has_no_step_on_the_api_grid_either(self):
        for wwr, depth in ((.8, .6), (.3, 1.2)):
            previous = None
            for index in range(int((450 - 90) / .25) + 1):
                current = self.api.evaluate({"orientation": 90 + index * .25, "wwr": wwr, "overhang": depth, "insulation": self.setup["insulation"]})
                if previous is not None:
                    for key, bound in self.setup["step_bound"].items():
                        self.assertLessEqual(abs(current[key] - previous[key]), bound, msg=(wwr, depth, index, key))
                previous = current


class OrientationExtrema(unittest.TestCase):
    """📉 Local and global optima of electricity over the façade bearing."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.study = cls.api.orientation_study()
        design = cls.study["design"]
        cls.energy = staticmethod(lambda angle: cls.api.evaluate({**design, "orientation": angle})["electricity"])

    def test_orientation_design_is_the_documented_sixty_percent_wall_without_overhang(self):
        self.assertEqual(self.study["design"], {"wwr": .6, "overhang": 0, "insulation": .16})
        self.assertEqual(self.study["range"], [90, 450])
        self.assertEqual((self.study["constraint"], self.study["reference"]), ([270, 450], 90))

    def test_two_local_minima_south_global_and_north_local_on_the_one_degree_grid(self):
        expected = CASES["orientation"]
        self.assertEqual(self.study["local_minima"], expected["local_minima"])
        self.assertEqual(self.study["local_maxima"], expected["local_maxima"])
        self.assertEqual(self.study["global_minimum"], expected["global_minimum"])
        self.assertEqual(self.study["global_minimum"], 180)
        self.assertEqual(self.study["feasible_minimum"], 360)
        self.assertEqual(len(self.study["samples"]), expected["samples"])
        self.assertEqual([angle for angle, _ in self.study["samples"]], list(range(90, 451)))
        south, north = (self.energy(angle) for angle in self.study["local_minima"])
        self.assertAlmostEqual(south, expected["values"]["south"], places=4)
        self.assertAlmostEqual(north, expected["values"]["north"], places=4)
        self.assertLess(south, north)
        self.assertGreaterEqual(north / south - 1, expected["minimum_penalty"])
        self.assertAlmostEqual(north / south - 1, expected["values"]["north_penalty"], places=5)

    def test_extrema_agree_with_scipy_bounded_and_brent_searches(self):
        from scipy.optimize import brentq, minimize_scalar

        for minimum in self.study["local_minima"]:
            found = minimize_scalar(self.energy, bounds=(minimum - 8, minimum + 8), method="bounded", options={"xatol": 1e-6})
            self.assertAlmostEqual(found.x, minimum, delta=1.0)
            slope = lambda angle: self.energy(angle + .01) - self.energy(angle - .01)
            self.assertAlmostEqual(brentq(slope, minimum - 8, minimum + 8), minimum, delta=1.0)
        for maximum in self.study["local_maxima"]:
            found = minimize_scalar(lambda angle: -self.energy(angle), bounds=(maximum - 15, maximum + 15), method="bounded", options={"xatol": 1e-6})
            self.assertAlmostEqual(found.x, maximum, delta=1.5)
        self.assertLessEqual(min(self.study["local_maxima"]), 290)
        self.assertGreater(len(self.study["local_maxima"]), 1)
        self.assertTrue(all(maximum in range(260, 300) or maximum in range(420, 450) for maximum in self.study["local_maxima"]))
        for value in self.study["local_maxima_electricity"]:
            self.assertGreater(value, 1.5 * min(self.study["local_minima_electricity"]))

    def test_bearing_is_periodic_and_symmetric_about_south(self):
        import numpy as np

        self.assertAlmostEqual(self.energy(450), self.energy(90))
        angles = np.arange(0, 91, 7.0)
        np.testing.assert_allclose([self.energy(180 + a) for a in angles], [self.energy(180 - a) for a in angles])
        self.assertEqual(self.energy(90), self.study["reference_electricity"])

    def test_courtyard_constraint_selects_the_north_local_minimum(self):
        import numpy as np

        angles = np.arange(self.study["constraint"][0], self.study["constraint"][1] + 1)
        values = np.array([self.energy(angle) for angle in angles])
        self.assertEqual(angles[int(np.argmin(values))], self.study["feasible_minimum"])
        self.assertGreater(values.min(), self.energy(self.study["global_minimum"]))
        self.assertAlmostEqual(values.min(), CASES["orientation"]["values"]["north"], places=4)

    def test_savings_against_the_east_reference_have_the_same_best_bearing(self):
        import numpy as np

        savings = np.array(self.study["savings"])
        self.assertEqual(savings[int(np.argmax(savings[:, 1])), 0], self.study["global_minimum"])
        np.testing.assert_allclose(savings[:, 1], self.study["reference_electricity"] - np.array([value for _, value in self.study["samples"]]))
        np.testing.assert_allclose(savings[:, 2], savings[:, 1] * self.study["tariff"])
        self.assertEqual(self.study["tariff"], .25)
        self.assertAlmostEqual(self.study["savings"][360 - 90][2], CASES["orientation"]["savings_north_chf"], places=4)
        feasible = savings[savings[:, 0] >= self.study["constraint"][0]]
        self.assertEqual(feasible[int(np.argmax(feasible[:, 1])), 0], self.study["feasible_minimum"])

    def test_greedy_neighbour_descent_from_northwest_ends_in_the_north_basin(self):
        descent = self.study["descent"]
        self.assertEqual((self.study["start"], self.study["neighbour_step"]), (315, 5))
        self.assertEqual([item["theta"] for item in descent], CASES["orientation"]["descent"])
        self.assertEqual(descent[0]["theta"], 315)
        self.assertEqual(descent[-1]["theta"], 360)
        values = [item["electricity"] for item in descent]
        self.assertTrue(all(a > b for a, b in zip(values, values[1:])))
        for item in descent:
            self.assertEqual({neighbour["theta"] for neighbour in item["neighbours"]} <= {item["theta"] - 5, item["theta"] + 5}, True)
            self.assertAlmostEqual(item["electricity"], self.energy(item["theta"]))
        self.assertTrue(all(neighbour["electricity"] >= descent[-1]["electricity"] for neighbour in descent[-1]["neighbours"]))
        self.assertGreater(self.energy(315), self.energy(360))
        self.assertTrue(all(self.energy(angle) > self.energy(angle + 1) for angle in range(300, 359)))

    def test_sun_geometry_is_physical_and_covers_the_expected_seasons(self):
        sun = self.study["sun"]
        self.assertEqual({key: sun[key] for key in CASES["orientation"]["sun"]}, CASES["orientation"]["sun"])
        self.assertAlmostEqual(sun["winter_altitude"], 90 - sun["latitude"] - 23.44, delta=1.0)
        self.assertAlmostEqual(sun["summer_altitude"], 90 - sun["latitude"] + 23.44, delta=1.0)
        self.assertLess(sun["low_summer_altitude"], sun["summer_altitude"])
        self.assertGreater(sun["low_summer_altitude"], sun["winter_altitude"])
        self.assertTrue(sun["winter_azimuth"][0] < 180 < sun["winter_azimuth"][1])
        self.assertTrue(sun["summer_azimuth"][0] < sun["winter_azimuth"][0] and sun["summer_azimuth"][1] > sun["winter_azimuth"][1])


class SurfaceAndSampling(unittest.TestCase):
    """🎲 The WWR × overhang surface and the three equal-budget sampling plans."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.surface = cls.api.surface()
        cls.study = cls.api.load_study()
        cls.sets = {kind: cls.api.samples(kind) for kind in ("grid", "random", "lhs")}

    def test_surface_is_a_dense_41_by_41_grid_matching_the_numpy_model(self):
        import numpy as np

        wwr, overhang = np.array(self.surface["wwr"]), np.array(self.surface["overhang"])
        self.assertEqual((len(wwr), len(overhang)), (41, 41))
        np.testing.assert_allclose([wwr[0], wwr[-1], overhang[0], overhang[-1]], [.1, .8, 0, 1.2])
        mesh = np.array(np.meshgrid(wwr, overhang, indexing="ij")).reshape(2, -1).T
        rows = np.column_stack((np.full(len(mesh), 180.0), mesh, np.full(len(mesh), .16)))
        np.testing.assert_allclose(np.array(self.surface["electricity"]).ravel(), reference_evaluate(self.study, rows)["electricity"], rtol=1e-9)
        self.assertEqual(self.surface["design"]["orientation"], 180)
        self.assertEqual(self.surface["design"]["insulation"], .16)

    def test_surface_has_one_interior_minimum_and_a_curved_valley(self):
        import numpy as np

        grid = np.array(self.surface["electricity"])
        i, j = np.unravel_index(np.argmin(grid), grid.shape)
        self.assertTrue(0 < i < 40 and 0 < j < 40)
        minimum = self.surface["minimum"]
        self.assertAlmostEqual(minimum["electricity"], grid.min())
        self.assertAlmostEqual(minimum["design"]["wwr"], self.surface["wwr"][i])
        self.assertAlmostEqual(minimum["design"]["overhang"], self.surface["overhang"][j])
        for key, expected in CASES["surface"]["minimum"].items():
            self.assertAlmostEqual(minimum["electricity"] if key == "electricity" else minimum["design"][key], expected, places=4)
        padded = np.pad(grid, 1, constant_values=np.inf)
        local = [(a, b) for a in range(1, 40) for b in range(1, 40)
                 if grid[a, b] <= np.delete(padded[a:a + 3, b:b + 3].ravel(), 4).min()]
        self.assertEqual(local, [(int(i), int(j))])
        shelf = grid[:, 0] - grid[:, 1]
        self.assertLess(np.abs(shelf[10:25]).max(), .5)
        valley = np.array(self.surface["overhang"])[np.argmin(grid, axis=1)]
        self.assertGreater(valley.max() - valley.min(), .5)
        self.assertGreater(np.ptp(valley[10:35]), .3)
        for corner, expected in zip(((0, 0), (-1, 0), (0, -1), (-1, -1)), CASES["surface"]["corners"].values()):
            self.assertAlmostEqual(grid[corner], expected, places=4)
            self.assertGreater(grid[corner], 1.005 * grid.min())

    def test_surface_minimum_agrees_with_scipy_nelder_mead_from_several_starts(self):
        from scipy.optimize import minimize

        base = {**self.surface["design"]}
        objective = lambda x: self.api.evaluate({**base, "wwr": x[0], "overhang": x[1]})["electricity"]
        for start in ([.75, .1], [.3, .3], [.7, .9], [.2, .2], [.5, .5]):
            found = minimize(objective, start, method="Nelder-Mead", bounds=[(.1, .8), (0, 1.2)], options={"xatol": 1e-6, "fatol": 1e-7})
            self.assertLess(found.fun, 1.01 * self.surface["minimum"]["electricity"])
            self.assertGreater(found.fun, .995 * self.surface["minimum"]["electricity"])
            self.assertTrue(.3 < found.x[0] < .79 and .5 < found.x[1] < 1.19)

    def test_plans_share_the_budget_domain_and_unknown_kinds_fail(self):
        import numpy as np

        for kind, points in self.sets.items():
            with self.subTest(kind=kind):
                self.assertEqual(len(points), CASES["sampling"]["counts"][kind])
                self.assertEqual(len(points), 9)
                self.assertTrue(all(.1 <= item["wwr"] <= .8 and 0 <= item["overhang"] <= 1.2 for item in points))
                np.testing.assert_allclose([item["electricity"] for item in points], CASES["sampling"]["electricity"][kind], atol=1e-4)
                rows = np.column_stack((np.full(9, 180.0), [item["wwr"] for item in points], [item["overhang"] for item in points], np.full(9, .16)))
                np.testing.assert_allclose([item["electricity"] for item in points], reference_evaluate(self.study, rows)["electricity"], rtol=1e-9)
        levels = self.api.levels()
        self.assertEqual({key: len(values) for key, values in levels.items()}, {"wwr": 3, "overhang": 3, "orientation": 4, "insulation": 3})
        expected = np.array(np.meshgrid(levels["wwr"], levels["overhang"], indexing="ij")).reshape(2, -1).T
        np.testing.assert_allclose([[item["wwr"], item["overhang"]] for item in self.sets["grid"]], expected)
        self.assertEqual(levels["orientation"], [90, 180, 270, 360])
        with self.assertRaises(ValueError):
            self.api.samples("unknown")

    def test_random_draws_leave_the_upper_right_empty_and_contain_one_close_pair(self):
        from scipy.spatial.distance import pdist

        import numpy as np

        points = np.array([[item["wwr"], item["overhang"]] for item in self.sets["random"]])
        empty = CASES["sampling"]["empty_corner"]
        self.assertFalse(np.any((points[:, 0] > empty["wwr_above"]) & (points[:, 1] > empty["overhang_above"])))
        unit = (points - [.1, 0]) / [.7, 1.2]
        distances = pdist(unit)
        diagnostics = self.api.sampling_diagnostics()
        self.assertAlmostEqual(diagnostics["closest_pair"][2], distances.min())
        i, j = diagnostics["closest_pair"][:2]
        self.assertLessEqual(abs(points[i, 0] - points[j, 0]), CASES["sampling"]["closest"]["wwr_below"])
        self.assertLessEqual(abs(points[i, 1] - points[j, 1]), CASES["sampling"]["closest"]["overhang_below"])
        self.assertEqual(np.count_nonzero(distances < .08), 1)
        xs = np.unique(np.concatenate(([0, 1], unit[:, 0])))
        ys = np.unique(np.concatenate(([0, 1], unit[:, 1])))
        best = 0.0
        for left, right in itertools.combinations(xs, 2):
            for bottom, top in itertools.combinations(ys, 2):
                inside = np.any((unit[:, 0] > left) & (unit[:, 0] < right) & (unit[:, 1] > bottom) & (unit[:, 1] < top))
                best = max(best, 0 if inside else (right - left) * (top - bottom))
        self.assertAlmostEqual(diagnostics["area"], best)
        box = diagnostics["largest_empty_box"]
        self.assertAlmostEqual((box["wwr"][1] - box["wwr"][0]) / .7 * (box["overhang"][1] - box["overhang"][0]) / 1.2, best)
        self.assertGreater(diagnostics["area"], .3)
        self.assertGreater(box["wwr"][1], .6)
        self.assertGreater(box["overhang"][1], 1.1)
        for key, expected in CASES["sampling"]["diagnostics"]["largest_empty_box"].items():
            np.testing.assert_allclose(box[key], expected, atol=1e-4)

    def test_latin_hypercube_uses_each_stratum_once_per_axis_and_exposes_the_strata(self):
        import numpy as np
        from scipy.stats import qmc

        strata = self.api.lhs_strata()
        points = np.array([[item["wwr"], item["overhang"]] for item in self.sets["lhs"]])
        unit = (points - [.1, 0]) / [.7, 1.2]
        self.assertEqual(strata["count"], 9)
        np.testing.assert_allclose(strata["wwr"], np.linspace(.1, .8, 10))
        np.testing.assert_allclose(strata["overhang"], np.linspace(0, 1.2, 10))
        for axis in range(2):
            np.testing.assert_array_equal(np.sort(np.floor(unit[:, axis] * 9).astype(int)), np.arange(9))
        index = np.array([item["strata_index"] for item in self.sets["lhs"]])
        np.testing.assert_array_equal(index, np.floor(unit * 9).astype(int))
        np.testing.assert_array_equal(index, strata["index"])
        self.assertTrue(np.all(points[:, 0] > np.array(strata["wwr"])[index[:, 0]]) and np.all(points[:, 0] < np.array(strata["wwr"])[index[:, 0] + 1]))
        self.assertTrue(np.all(points[:, 1] > np.array(strata["overhang"])[index[:, 1]]) and np.all(points[:, 1] < np.array(strata["overhang"])[index[:, 1] + 1]))
        random = (np.array([[item["wwr"], item["overhang"]] for item in self.sets["random"]]) - [.1, 0]) / [.7, 1.2]
        self.assertLess(qmc.discrepancy(unit, method="L2-star"), qmc.discrepancy(random, method="L2-star"))
        self.assertTrue(all("strata_index" not in item for item in self.sets["grid"] + self.sets["random"]))

    def test_lhs_beats_random_and_nothing_reaches_the_dense_optimum(self):
        best = {kind: min(item["electricity"] for item in points) for kind, points in self.sets.items()}
        optimum = self.surface["minimum"]["electricity"]
        self.assertLess(best["lhs"], best["random"])
        for kind, value in best.items():
            with self.subTest(kind=kind):
                self.assertGreater(value, optimum)
        self.assertTrue(all(item["electricity"] > optimum for points in self.sets.values() for item in points))
        self.assertLess(best["lhs"], 1.01 * optimum)
        self.assertGreater(best["grid"], best["lhs"])
        self.assertGreater(max(item["electricity"] for item in self.sets["random"]), 1.2 * optimum)


class FactorialExperiments(unittest.TestCase):
    """🔢 The 2² full factorial and the 2^(3-1) half fraction on the cooling electricity."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.result = cls.api.factorial()
        cls.study = cls.api.load_study()

    def test_four_runs_in_order_with_full_evaluations(self):
        runs = self.result["runs"]
        self.assertEqual([run["id"] for run in runs], ["r1", "r2", "r3", "r4"])
        self.assertEqual([run["coded"] for run in runs], [[-1, -1], [-1, 1], [1, -1], [1, 1]])
        self.assertEqual([(run["design"]["wwr"], run["design"]["overhang"]) for run in runs], [(.3, 0), (.3, 1), (.7, 0), (.7, 1)])
        for run in runs:
            expected = self.api.evaluate(run["design"])
            self.assertEqual(run["response"], expected["cooling"])
            self.assertTrue(all(run[key] == value for key, value in expected.items()))
        self.assertEqual(self.result["response"], "cooling")
        self.assertAlmostEqual([run["response"] for run in runs][0], CASES["factorial"]["responses"][0], places=4)

    def test_effects_and_coefficients_are_exact_contrasts_confirmed_by_least_squares(self):
        import numpy as np

        coded = np.array([run["coded"] for run in self.result["runs"]], dtype=float)
        response = np.array([run["response"] for run in self.result["runs"]])
        matrix = np.column_stack((np.ones(4), coded[:, 0], coded[:, 1], coded[:, 0] * coded[:, 1]))
        np.testing.assert_allclose(matrix @ self.result["coefficients"], response)
        np.testing.assert_allclose(self.result["coefficients"], np.linalg.lstsq(matrix, response, rcond=None)[0])
        effects = self.result["effects"]
        np.testing.assert_allclose([effects["a"], effects["b"], effects["ab"]], 2 * np.array(self.result["coefficients"][1:]))
        r1, r2, r3, r4 = response
        self.assertAlmostEqual(effects["a"], (r3 + r4) / 2 - (r1 + r2) / 2)
        self.assertAlmostEqual(effects["b"], (r2 + r4) / 2 - (r1 + r3) / 2)
        self.assertAlmostEqual(effects["ab"], .5 * ((r4 - r3) - (r2 - r1)))
        self.assertAlmostEqual(self.result["coefficients"][0], response.mean())
        for key, expected in CASES["factorial"]["effects"].items():
            self.assertAlmostEqual(effects[key], expected, places=4)
        np.testing.assert_allclose(self.result["coefficients"], CASES["factorial"]["coefficients"], atol=1e-4)

    def test_overhang_saves_much_more_at_large_windows_a_clear_interaction(self):
        r1, r2, r3, r4 = (run["response"] for run in self.result["runs"])
        small, large = r1 - r2, r3 - r4
        self.assertGreater(small, 0)
        self.assertGreaterEqual(large, CASES["factorial"]["minimum_saving_ratio"] * small)
        self.assertGreater(abs(self.result["effects"]["ab"]), .2 * abs(self.result["effects"]["b"]))
        self.assertLess(self.result["effects"]["ab"], 0)
        self.assertGreater(self.result["effects"]["a"], 0)
        self.assertLess(self.result["effects"]["b"], 0)

    def test_centre_run_differs_from_the_bilinear_prediction_by_curvature(self):
        centre = self.result["centre"]
        self.assertEqual((centre["design"]["wwr"], centre["design"]["overhang"]), (.5, .5))
        self.assertAlmostEqual(centre["predicted"], self.result["coefficients"][0])
        self.assertAlmostEqual(centre["actual"], self.api.evaluate(centre["design"])["cooling"])
        self.assertGreaterEqual(abs(centre["actual"] - centre["predicted"]) / centre["predicted"], CASES["factorial"]["minimum_curvature"])
        self.assertAlmostEqual(centre["curvature"], (centre["actual"] - centre["predicted"]) / centre["predicted"])
        self.assertAlmostEqual(centre["predicted"], CASES["factorial"]["centre"]["predicted"], places=4)
        self.assertAlmostEqual(centre["actual"], CASES["factorial"]["centre"]["actual"], places=4)

    def test_half_overhang_delivers_more_than_half_of_the_full_saving_at_fifty_percent(self):
        savings = self.result["centre_savings"]
        self.assertGreater(savings["share"], .5)
        self.assertAlmostEqual(savings["share"], CASES["factorial"]["half_share"], places=5)
        cool = lambda depth: self.api.evaluate({"wwr": .5, "overhang": depth})["cooling"]
        self.assertAlmostEqual(savings["half"], cool(0) - cool(.5))
        self.assertAlmostEqual(savings["full"], cool(0) - cool(1))

    def test_summer_sun_and_shading_per_run_follow_the_study_altitude(self):
        runs = self.result["runs"]
        sun = self.api.load_study()["sun"]
        self.assertEqual(self.api.orientation_study()["sun"]["summer_altitude"], sun["summer_altitude"])
        self.assertEqual(self.api.orientation_study()["sun"]["winter_altitude"], sun["winter_altitude"])
        self.assertEqual(runs[0]["shading_summer"], 0)
        self.assertGreater(runs[1]["shading_summer"], 0)
        self.assertGreater(runs[2]["solar_summer"], runs[0]["solar_summer"])
        self.assertLess(runs[3]["solar_summer"], runs[2]["solar_summer"])

    def test_half_fraction_confounds_insulation_with_the_interaction(self):
        import numpy as np

        fraction = self.api.fractional()
        self.assertEqual(fraction["generator"], "C=AB")
        self.assertEqual(len(fraction["runs"]), 4)
        self.assertTrue(all(run["coded"][2] == run["coded"][0] * run["coded"][1] for run in fraction["runs"]))
        self.assertEqual([factor["id"] for factor in fraction["factors"]], ["wwr", "overhang", "insulation"])
        self.assertEqual(fraction["factors"][2]["levels"], [.1, .3])
        for run in fraction["runs"]:
            self.assertAlmostEqual(run["response"], self.api.evaluate(run["design"])["cooling"])
        codes = np.array(list(itertools.product((-1.0, 1.0), repeat=3)))
        responses = np.array([item["response"] for item in fraction["full"]])
        np.testing.assert_array_equal(codes, [item["coded"] for item in fraction["full"]])
        matrix = np.column_stack((np.ones(8), codes, codes[:, 0] * codes[:, 1], codes[:, 0] * codes[:, 2], codes[:, 1] * codes[:, 2], codes.prod(axis=1)))
        coefficients = np.linalg.lstsq(matrix, responses, rcond=None)[0]
        self.assertAlmostEqual(fraction["true"]["c"], 2 * coefficients[3])
        self.assertAlmostEqual(fraction["true"]["ab"], 2 * coefficients[4])
        self.assertAlmostEqual(fraction["true"]["sum"], fraction["true"]["c"] + fraction["true"]["ab"])
        signs = np.array([run["coded"][2] for run in fraction["runs"]])
        y = np.array([run["response"] for run in fraction["runs"]])
        self.assertAlmostEqual(fraction["alias_contrast"], (y * signs).sum() / 2)
        self.assertAlmostEqual(fraction["alias_contrast"], fraction["true"]["sum"], delta=.05 * abs(fraction["true"]["sum"]))
        self.assertGreater(abs(fraction["true"]["ab"]), 5 * abs(fraction["true"]["c"]))
        self.assertAlmostEqual(fraction["alias_contrast"], CASES["fractional"]["alias_contrast"], places=4)
        for key, expected in CASES["fractional"]["true"].items():
            self.assertAlmostEqual(fraction["true"][key], expected, places=4)
        np.testing.assert_allclose(y, CASES["fractional"]["responses"], atol=1e-4)


class IndexOfVariants(unittest.TestCase):
    """⚖️ Five façade variants, min–max normalization and two weight sets."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.study = cls.api.load_study()
        cls.weights = cls.study["weights"]

    def test_criteria_order_directions_and_units(self):
        criteria = self.api.criteria()
        self.assertEqual([item["id"] for item in criteria], ["electricity", "cost", "daylight", "carbon"])
        self.assertEqual([item["direction"] for item in criteria], ["min", "min", "max", "min"])
        self.assertEqual([item["unit"] for item in criteria], ["kWh/(m²·a)", "CHF", "%", "kg CO₂e"])
        for item in criteria:
            self.assertTrue(item["title"]["en"] and item["title"]["de"])

    def test_variants_carry_titles_designs_and_evaluations_and_the_sixth_is_extra(self):
        variants = self.api.variants()
        self.assertEqual([item["id"] for item in variants], ["punched", "ribbon", "glass_shaded", "punched_insulated", "glass_bare"])
        for item in variants:
            self.assertNotEqual(item["title"]["en"], item["title"]["de"])
            self.assertLessEqual(len(item["title"]["en"]), 32)
            expected = self.api.evaluate(item["design"])
            self.assertTrue(all(item[key] == value for key, value in expected.items()))
        extra = self.api.variants(extra=True)
        self.assertEqual([item["id"] for item in extra][:5], [item["id"] for item in variants])
        self.assertEqual(len(extra), 6)
        self.assertTrue(extra[5]["extra"])
        self.assertGreater(extra[5]["cost"], max(item["cost"] for item in variants))
        self.assertTrue(all("extra" not in item for item in variants))
        for item, expected in zip(extra, CASES["index"]["variants"], strict=True):
            for key in ("intensity", "cost", "daylight", "carbon"):
                self.assertAlmostEqual(item[key], expected[key], places=4)

    def test_no_variant_wins_everywhere_and_at_least_three_win_something(self):
        variants = self.api.variants()
        winners = [(min if item["direction"] == "min" else max)(variants, key=lambda variant: variant[item["key"]])["id"] for item in self.api.criteria()]
        self.assertGreaterEqual(len(set(winners)), 3)
        self.assertGreater(len(set(winners)), 1)

    def test_index_matches_a_numpy_min_max_normalization(self):
        import numpy as np

        criteria, variants = self.api.criteria(), self.api.variants()
        matrix = np.array([[variant[item["key"]] for item in criteria] for variant in variants])
        normalized = (matrix - matrix.min(axis=0)) / (matrix.max(axis=0) - matrix.min(axis=0))
        for column, item in enumerate(criteria):
            if item["direction"] == "max":
                normalized[:, column] = 1 - normalized[:, column]
        for name, weights in self.weights.items():
            with self.subTest(weights=name):
                vector = np.array([weights[item["id"]] for item in criteria])
                result = self.api.index(weights)
                np.testing.assert_allclose([[row["raw"][item["id"]] for item in criteria] for row in result], matrix)
                np.testing.assert_allclose([[row["normalized"][item["id"]] for item in criteria] for row in result], normalized)
                np.testing.assert_allclose([row["score"] for row in result], normalized @ (vector / vector.sum()))
                self.assertEqual([row["rank"] for row in result], list(np.argsort(np.argsort(normalized @ vector)) + 1))
                self.assertEqual(sorted(row["rank"] for row in result), [1, 2, 3, 4, 5])
        np.testing.assert_allclose([row["score"] for row in self.api.index(self.weights["balanced"])], CASES["index"]["scores_balanced"], atol=1e-4)

    def test_client_budget_is_authored_and_excludes_exactly_the_stated_variants(self):
        brief = self.api.brief()
        self.assertEqual(brief["budget_chf"], CASES["brief"]["budget_chf"])
        self.assertEqual(set(brief["note"]), {"en", "de"})
        variants = self.api.variants()
        over = [item["id"] for item in variants if item["cost"] > brief["budget_chf"]]
        self.assertEqual(over, CASES["brief"]["excluded"])
        allowed = [item["id"] for item in variants if item["id"] not in over]
        ranked = sorted(self.api.index(self.weights["balanced"], ids=allowed), key=lambda row: row["rank"])
        self.assertEqual([row["id"] for row in ranked], CASES["brief"]["allowed_order"])

    def test_two_weight_sets_choose_different_winners_with_distinguishable_scores(self):
        winners = {name: min(self.api.index(weights), key=lambda row: row["score"])["id"] for name, weights in self.weights.items()}
        self.assertNotEqual(winners["balanced"], winners["cost_heavy"])
        self.assertEqual(winners, CASES["index"]["winner"])
        scores = sorted(row["score"] for row in self.api.index(self.weights["balanced"]))
        self.assertTrue(all(b - a >= .02 for a, b in zip(scores, scores[1:])))
        self.assertAlmostEqual(sum(self.weights["balanced"].values()), 1)
        self.assertEqual(self.weights["balanced"], {"electricity": .4, "cost": .3, "daylight": .2, "carbon": .1})

    def test_reweigh_scales_the_other_criteria_in_their_old_proportions(self):
        heavy = self.api.reweigh(self.weights["balanced"], "cost", .6)
        self.assertAlmostEqual(sum(heavy.values()), 1)
        self.assertEqual(heavy["cost"], .6)
        self.assertAlmostEqual(heavy["electricity"] / heavy["daylight"], .4 / .2)
        self.assertEqual([round(100 * heavy[key]) for key in ("electricity", "daylight", "carbon")], [23, 11, 6])
        for key, value in self.weights["cost_heavy"].items():
            self.assertAlmostEqual(heavy[key], value)
        with self.assertRaises(ValueError):
            self.api.reweigh(self.weights["balanced"], "noise", .5)
        with self.assertRaises(ValueError):
            self.api.reweigh(self.weights["balanced"], "cost", 1.5)

    def test_adding_the_sixth_variant_changes_the_normalization_and_flips_a_pair(self):
        ids = [item["id"] for item in self.api.variants(extra=True)]
        five = [row["id"] for row in sorted(self.api.index(self.weights["balanced"]), key=lambda row: row["rank"])]
        six_rows = sorted(self.api.index(self.weights["balanced"], ids=ids), key=lambda row: row["rank"])
        self.assertEqual(five, CASES["index"]["order_five"])
        self.assertEqual([row["id"] for row in six_rows], CASES["index"]["order_six"])
        self.assertNotEqual([i for i in (row["id"] for row in six_rows) if i != ids[5]], five)
        self.assertNotEqual(six_rows[0]["id"], ids[5])
        heavy_six = min(self.api.index(self.weights["cost_heavy"], ids=ids), key=lambda row: row["score"])["id"]
        self.assertNotEqual(heavy_six, ids[5])
        subset = self.api.index(self.weights["balanced"], ids=["punched", "ribbon"])
        self.assertEqual({row["id"] for row in subset}, {"punched", "ribbon"})
        for item in self.api.criteria():
            self.assertEqual(sorted(row["normalized"][item["id"]] for row in subset), [0.0, 1.0])

    def test_naive_sums_of_raw_numbers_rank_differently_with_chf_and_thousand_chf(self):
        variants = self.api.variants()
        for scale in (1, 1000):
            raw = {variant["id"]: variant["intensity"] + variant["cost"] / scale - variant["daylight"] + variant["carbon"] / scale for variant in variants}
            order = sorted(raw, key=raw.get)
            if scale == 1:
                chf = order
        self.assertNotEqual(chf, order)

    def test_invalid_weights_are_rejected(self):
        good = self.weights["balanced"]
        for weights in ({}, {**good, "cost": -1}, {**good, "cost": float("nan")}, {key: 0 for key in good}, {**good, "extra": 1}, [.25] * 4):
            with self.subTest(weights=weights), self.assertRaises(ValueError):
                self.api.index(weights)
        with self.assertRaises(TypeError):
            self.api.index()


class ParetoFront(unittest.TestCase):
    """🏆 Cost versus electricity over a 30-design grid, with paybacks and a third criterion."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.result = cls.api.pareto()
        cls.designs = {item["id"]: item for item in cls.result["designs"]}

    def test_grid_covers_six_window_ratios_three_insulations_and_two_overhangs(self):
        designs = self.result["designs"]
        self.assertEqual(len(designs), CASES["pareto"]["count"])
        self.assertEqual(len({(item["design"]["wwr"], item["design"]["insulation"], item["design"]["overhang"]) for item in designs}), 36)
        self.assertEqual(len({item["design"]["wwr"] for item in designs}), 6)
        self.assertEqual(len({item["design"]["insulation"] for item in designs}), 3)
        self.assertEqual(len({item["design"]["overhang"] for item in designs}), 2)
        for item in designs:
            expected = self.api.evaluate(item["design"])
            self.assertEqual((item["cost"], item["electricity"], item["daylight"]), (expected["cost"], expected["electricity"], expected["daylight"]))
            self.assertTrue(all(key in item for key in ("heating", "cooling", "lighting")))

    def test_front_matches_a_numpy_dominance_matrix_and_has_five_to_nine_members(self):
        import numpy as np

        designs = self.result["designs"]
        cost, electricity = np.array([item["cost"] for item in designs]), np.array([item["electricity"] for item in designs])
        mask = ~dominance(cost, electricity).any(axis=0)
        expected = sorted((item["id"] for item, kept in zip(designs, mask) if kept), key=lambda identifier: self.designs[identifier]["cost"])
        self.assertEqual(self.result["front"], expected)
        self.assertEqual(self.result["front"], CASES["pareto"]["front"])
        self.assertTrue(5 <= len(expected) <= 9)
        costs = [self.designs[identifier]["cost"] for identifier in expected]
        self.assertEqual(costs, sorted(costs))
        electricities = [self.designs[identifier]["electricity"] for identifier in expected]
        self.assertTrue(all(a > b for a, b in zip(electricities, electricities[1:])))
        used = {key: {self.designs[identifier]["design"][key] for identifier in expected} for key in ("wwr", "insulation", "overhang")}
        self.assertTrue(all(len(values) > 1 for values in used.values()))

    def test_dominated_example_is_a_concrete_well_separated_pair_with_larger_cooling_gap(self):
        pair = self.result["dominated_example"]
        winner, loser = self.designs[pair["winner"]], self.designs[pair["loser"]]
        self.assertEqual(pair, CASES["pareto"]["dominated_example"])
        self.assertLessEqual(winner["cost"], loser["cost"])
        self.assertLess(winner["electricity"], loser["electricity"])
        self.assertIn(pair["winner"], self.result["front"])
        self.assertNotIn(pair["loser"], self.result["front"])
        self.assertGreaterEqual(loser["design"]["wwr"], .65)
        self.assertEqual(loser["design"]["overhang"], 0)
        costs, electricities = [item["cost"] for item in self.designs.values()], [item["electricity"] for item in self.designs.values()]
        self.assertGreaterEqual((loser["cost"] - winner["cost"]) / (max(costs) - min(costs)), .15)
        self.assertGreaterEqual((loser["electricity"] - winner["electricity"]) / (max(electricities) - min(electricities)), .15)
        self.assertGreater(loser["cooling"] - winner["cooling"], abs(loser["heating"] - winner["heating"]))

    def test_first_front_step_names_the_added_cost_and_saved_electricity(self):
        step = self.result["step"]
        start, end = self.designs[step["from"]], self.designs[step["to"]]
        self.assertAlmostEqual(step["cost"], end["cost"] - start["cost"])
        self.assertAlmostEqual(step["electricity"], end["electricity"] - start["electricity"])
        self.assertGreater(step["cost"], 0)
        self.assertLess(step["electricity"], 0)
        front = self.result["front"]
        self.assertEqual(front.index(step["to"]) - front.index(step["from"]), 1)

    def test_steps_have_rising_prices_per_saved_kwh_and_payback_crossing_the_service_life_once(self):
        steps, life = self.result["steps"], self.result["service_life"]
        self.assertEqual(len(steps), len(self.result["front"]) - 1)
        self.assertEqual((self.result["tariff"], self.result["tariff_hi"], life), (.25, .4, 30))
        for step in steps:
            self.assertAlmostEqual(step["ratio"], step["cost"] / step["saved"])
            self.assertAlmostEqual(step["payback"], step["cost"] / (step["saved"] * self.result["tariff"]))
            self.assertAlmostEqual(step["payback_hi"], step["cost"] / (step["saved"] * self.result["tariff_hi"]))
            self.assertAlmostEqual(step["electricity"], -step["saved"])
        ratios = [step["ratio"] for step in steps]
        self.assertTrue(all(a < b for a, b in zip(ratios, ratios[1:])))
        self.assertLessEqual(steps[0]["payback"], .5 * life)
        self.assertGreaterEqual(steps[-1]["payback"], 2 * life)
        for key in ("payback", "payback_hi"):
            crossings = sum((a[key] <= life) != (b[key] <= life) for a, b in zip(steps, steps[1:]))
            self.assertEqual(crossings, 1)
        for expected, step in zip(CASES["pareto"]["steps"], steps, strict=True):
            self.assertEqual((expected["from"], expected["to"]), (step["from"], step["to"]))
            self.assertAlmostEqual(step["payback"], expected["payback"], places=4)

    def test_knee_moves_with_the_tariff(self):
        knee, front = self.result["knee"], self.result["front"]
        self.assertEqual(knee, CASES["pareto"]["knee"])
        self.assertNotEqual(knee["tariff"], knee["tariff_hi"])
        self.assertLess(front.index(knee["tariff"]), front.index(knee["tariff_hi"]))
        for name, key in (("tariff", "payback"), ("tariff_hi", "payback_hi")):
            reached = [step["to"] for step in self.result["steps"] if step[key] <= self.result["service_life"]]
            self.assertEqual(knee[name], reached[-1])
        self.assertGreater(front.index(knee["tariff"]), 0)
        self.assertLess(front.index(knee["tariff_hi"]), len(front) - 1)

    def test_refinement_is_an_off_grid_design_that_dominates_a_sampled_front_member(self):
        import numpy as np

        refinement = self.result["refinement"]
        dense = np.array(self.api.surface()["electricity"])
        i, j = np.unravel_index(int(np.argmin(dense)), dense.shape)
        surface = self.api.surface()
        optimum = {**surface["design"], "wwr": surface["wwr"][i], "overhang": surface["overhang"][j]}
        self.assertEqual(refinement["design"], optimum)
        self.assertEqual(refinement["design"], surface["minimum"]["design"])
        self.assertAlmostEqual(refinement["electricity"], dense.min())
        for key in ("orientation", "insulation"):
            self.assertEqual(refinement["design"][key], self.api.load_study()["surface"]["design"][key])
        costs = np.array([item["cost"] for item in self.result["designs"]])
        energies = np.array([item["electricity"] for item in self.result["designs"]])
        beaten = costs > refinement["cost"]
        beaten &= energies > refinement["electricity"]
        self.assertEqual({item["id"] for item, flag in zip(self.result["designs"], beaten) if flag and item["id"] in self.result["front"]}, set(refinement["dominates"]))
        member = self.designs[refinement["member"]]
        self.assertIn(refinement["member"], self.result["front"])
        expected = self.api.evaluate(refinement["design"])
        self.assertEqual((refinement["cost"], refinement["electricity"]), (expected["cost"], expected["electricity"]))
        self.assertLess(refinement["cost"], member["cost"])
        self.assertLess(refinement["electricity"], member["electricity"])
        self.assertIn(refinement["member"], refinement["dominates"])
        grid = {key: {item["design"][key] for item in self.designs.values()} for key in ("wwr", "overhang", "insulation")}
        self.assertTrue(any(refinement["design"][key] not in grid[key] for key in grid))
        self.assertEqual({"member": refinement["member"], "dominates": refinement["dominates"]}, CASES["pareto"]["refinement"])

    def test_third_criterion_revives_a_dominated_design_that_has_more_daylight(self):
        import numpy as np

        designs = self.result["designs"]
        columns = ([item["cost"] for item in designs], [item["electricity"] for item in designs], [-item["daylight"] for item in designs])
        expected = [item["id"] for item, dominated in zip(designs, dominance(*columns).any(axis=0)) if not dominated]
        self.assertEqual(self.result["front3"], expected)
        self.assertEqual(len(expected), CASES["pareto"]["front3_count"])
        example = self.result["third_example"]
        revived, dominator = self.designs[example["dominated_2d"]], self.designs[example["dominator"]]
        self.assertNotIn(example["dominated_2d"], self.result["front"])
        self.assertIn(example["dominated_2d"], self.result["front3"])
        self.assertIn(example["dominator"], self.result["front"])
        self.assertTrue(dominator["cost"] <= revived["cost"] and dominator["electricity"] <= revived["electricity"])
        self.assertGreater(revived["daylight"], dominator["daylight"])
        self.assertEqual(example, CASES["pareto"]["third_example"])
        self.assertTrue(set(self.result["front"]) <= set(self.result["front3"]))
        self.assertGreater(len(np.unique(expected)), len(self.result["front"]))

    def test_weighted_pick_walks_along_the_front_as_the_electricity_weight_grows(self):
        import numpy as np

        front = self.result["front"]
        cost = np.array([self.designs[i]["cost"] for i in front])
        electricity = np.array([self.designs[i]["electricity"] for i in front])
        normal = lambda column: (column - column.min()) / (column.max() - column.min())
        positions = []
        for weight in np.linspace(0, 1, 21):
            expected = front[int(np.argmin((1 - weight) * normal(cost) + weight * normal(electricity)))]
            self.assertEqual(self.api.weighted_pick(float(weight)), expected)
            positions.append(front.index(expected))
        self.assertEqual(positions, sorted(positions))
        self.assertEqual((positions[0], positions[-1]), (0, len(front) - 1))
        self.assertEqual(self.result["picks"], CASES["pareto"]["picks"])
        for pick in self.result["picks"]:
            self.assertEqual(self.api.weighted_pick(pick["weight"]), pick["id"])
        for weight in (-.1, 1.1):
            with self.assertRaises(ValueError):
                self.api.weighted_pick(weight)


class AlgorithmPaths(unittest.TestCase):
    """🧬 Gradient descent, compass search and a genetic algorithm on the south surface."""

    @classmethod
    def setUpClass(cls):
        cls.api = model_api()
        cls.surface = cls.api.surface()
        cls.optimum = cls.surface["minimum"]["electricity"]
        cls.gradient = cls.api.gradient_path()
        cls.pattern = cls.api.pattern_path()
        cls.genetic = cls.api.genetic()
        cls.study = cls.api.load_study()

    def objective(self, design):
        return self.api.evaluate({**self.surface["design"], "wwr": design["wwr"], "overhang": design["overhang"]})["electricity"]

    def test_gradient_descent_converges_in_about_ten_iterates_and_overshoots_once(self):
        path = self.gradient
        self.assertTrue(8 <= len(path) <= 14)
        self.assertEqual((len(path), path[0]["design"]["wwr"], path[0]["design"]["overhang"]), (CASES["algorithms"]["gradient"]["length"], .75, .1))
        self.assertLess(path[-1]["electricity"], 1.025 * self.optimum)
        self.assertAlmostEqual(path[-1]["electricity"], CASES["algorithms"]["gradient"]["last"], places=4)
        self.assertGreater(path[0]["electricity"], 1.3 * self.optimum)
        values = [item["electricity"] for item in path]
        rises = [k for k in range(1, len(values)) if values[k] > values[k - 1]]
        self.assertGreaterEqual(len(rises), 1)
        self.assertLess(min(values[rises[0]:]), values[rises[0] - 1])
        self.assertLess(values[-1], values[rises[0] - 1])
        target = self.surface["minimum"]["design"]["wwr"]
        sides = [item["design"]["wwr"] > target for item in path]
        self.assertTrue(sides[0] and not all(sides))

    def test_gradient_probes_slopes_steps_and_run_counts_are_consistent(self):
        import numpy as np

        previous = None
        for number, item in enumerate(self.gradient, 1):
            with self.subTest(iterate=number):
                self.assertAlmostEqual(item["electricity"], self.objective(item["design"]))
                self.assertEqual((item["runs"], item["evaluations"]), (5, 5 * number))
                self.assertEqual([probe["direction"] for probe in item["probes"]], ["wwr-", "wwr+", "overhang-", "overhang+"])
                for probe in item["probes"]:
                    self.assertAlmostEqual(probe["electricity"], self.objective(probe["design"]))
                low, high = item["probes"][0], item["probes"][1]
                self.assertAlmostEqual(item["gradient"]["wwr"], (high["electricity"] - low["electricity"]) / (high["design"]["wwr"] - low["design"]["wwr"]))
                low, high = item["probes"][2], item["probes"][3]
                self.assertAlmostEqual(item["gradient"]["overhang"], (high["electricity"] - low["electricity"]) / (high["design"]["overhang"] - low["design"]["overhang"]))
                self.assertAlmostEqual(item["slope"]["wwr"], item["gradient"]["wwr"] / 100)
                self.assertAlmostEqual(item["slope"]["overhang"], item["gradient"]["overhang"])
                if previous:
                    for key in ("wwr", "overhang"):
                        self.assertAlmostEqual(item["step"][key], item["design"][key] - previous["design"][key])
                else:
                    self.assertEqual(item["step"], {"wwr": 0.0, "overhang": 0.0})
                previous = item
        first = self.gradient[0]
        numeric = np.gradient([self.objective({"wwr": w, "overhang": .1}) for w in (.74, .75, .76)], .01)[1]
        self.assertAlmostEqual(first["gradient"]["wwr"], numeric, delta=.5)
        self.assertGreater(first["slope"]["wwr"], 0)
        self.assertLess(first["slope"]["overhang"], 0)
        self.assertLess(abs(self.gradient[-1]["slope"]["wwr"]), abs(self.gradient[0]["slope"]["wwr"]))

    def test_gradient_update_is_range_normalized_fixed_rate_descent(self):
        settings = self.study["algorithms"]["gradient"]
        for item, following in zip(self.gradient, self.gradient[1:]):
            expected_wwr = item["design"]["wwr"] - settings["learning_rate"] * item["gradient"]["wwr"] * .7 ** 2
            expected_overhang = item["design"]["overhang"] - settings["learning_rate"] * item["gradient"]["overhang"] * 1.2 ** 2
            self.assertAlmostEqual(following["design"]["wwr"], min(.8, max(.1, expected_wwr)))
            self.assertAlmostEqual(following["design"]["overhang"], min(1.2, max(0, expected_overhang)))

    def test_pattern_search_polls_four_neighbours_halves_stalled_steps_and_never_worsens(self):
        path = self.pattern
        self.assertEqual((path[0]["design"]["wwr"], path[0]["design"]["overhang"]), (.75, .1))
        self.assertEqual((path[0]["evaluations"], path[0]["probes"], path[0]["moved"]), (1, [], False))
        values = [item["electricity"] for item in path]
        self.assertTrue(all(a >= b for a, b in zip(values, values[1:])))
        self.assertLess(values[-1], 1.005 * self.optimum)
        halvings = [(a, b) for a, b in zip(path[1:], path[2:]) if not a["moved"]]
        self.assertGreaterEqual(len(halvings), 1)
        self.assertEqual(sum(not item["moved"] for item in path[1:]), CASES["algorithms"]["pattern"]["halvings"])
        for before, item in zip(path, path[1:]):
            self.assertEqual([probe["direction"] for probe in item["probes"]], ["wwr+", "wwr-", "overhang+", "overhang-"])
            self.assertEqual(item["evaluations"], before["evaluations"] + 4)
            self.assertEqual(item["moved"], item["design"] != before["design"])
            best = min(item["probes"], key=lambda probe: probe["electricity"])
            for probe in item["probes"]:
                self.assertAlmostEqual(probe["electricity"], self.objective(probe["design"]))
            if item["moved"]:
                self.assertEqual(item["design"], best["design"])
        for before, item, following in zip(path, path[1:], path[2:]):
            if not item["moved"]:
                self.assertEqual(following["step"], {key: item["step"][key] / 2 for key in item["step"]})
            else:
                self.assertEqual(following["step"], item["step"])
        self.assertEqual(path[-1]["evaluations"], CASES["algorithms"]["pattern"]["evaluations"])
        self.assertAlmostEqual(path[-1]["electricity"], CASES["algorithms"]["pattern"]["last"], places=4)

    def test_genetic_algorithm_has_eight_individuals_elitism_and_monotone_improving_best(self):
        generations = self.genetic["generations"]
        self.assertEqual(len(generations), 8)
        self.assertTrue(all(len(entry["population"]) == 8 for entry in generations))
        best = [entry["best"]["electricity"] for entry in generations]
        self.assertTrue(all(a >= b for a, b in zip(best, best[1:])))
        self.assertLess(best[-1], best[0] - 30)
        self.assertGreaterEqual(sum(a > b + .05 for a, b in zip(best, best[1:])), 3)
        self.assertLess(best[-1], 1.01 * self.optimum)
        self.assertGreater(best[0], 1.05 * self.optimum)
        for expected, actual in zip(CASES["algorithms"]["genetic"]["best"], best, strict=True):
            self.assertAlmostEqual(expected, actual, places=4)
        self.assertEqual([entry["evaluations"] for entry in generations], [8 + 6 * k for k in range(8)])
        for entry in generations:
            for item in entry["population"]:
                self.assertAlmostEqual(item["electricity"], self.objective(item["design"]))
                self.assertTrue(.1 <= item["design"]["wwr"] <= .8 and 0 <= item["design"]["overhang"] <= 1.2)
            self.assertEqual(entry["best"]["electricity"], min(item["electricity"] for item in entry["population"]))

    def test_selection_crossover_and_mutation_follow_the_recorded_rules(self):
        generations = self.genetic["generations"]
        mutated_better = mutated_worse = 0
        for entry, following in zip(generations, generations[1:]):
            lookup = {item["id"]: item for item in entry["population"]}
            ranked = sorted(entry["population"], key=lambda item: item["electricity"])
            self.assertEqual(entry["elites"], [item["id"] for item in ranked[:2]])
            self.assertEqual(entry["survivors"], [item["id"] for item in following["population"]])
            self.assertEqual(entry["survivors"][:2], entry["elites"])
            self.assertEqual(len(entry["children"]), 6)
            self.assertEqual(len(entry["parents"]), 6)
            for duel in entry["tournaments"]:
                self.assertEqual(len(duel), 3)
                self.assertEqual(duel[2], min(duel[:2], key=lambda identifier: lookup[identifier]["electricity"]))
            self.assertGreaterEqual(len(entry["tournaments"]), 12)
            for child, parents in zip(entry["children"], entry["parents"]):
                self.assertEqual(child["from"], parents)
                self.assertNotEqual(*parents)
                self.assertEqual(sorted(set(child["genes"].values())), sorted(parents))
                donors = {key: lookup[identifier]["design"][key] for key, identifier in child["genes"].items()}
                for key in ("wwr", "overhang"):
                    self.assertAlmostEqual(child["design"][key] - child["mutation"][key], donors[key])
                self.assertAlmostEqual(child["electricity"], self.objective(child["design"]))
                if any(child["mutation"].values()):
                    crossover = self.objective({key: donors[key] for key in donors})
                    mutated_better += child["electricity"] < crossover
                    mutated_worse += child["electricity"] > crossover
        self.assertGreaterEqual(mutated_better, 1)
        self.assertGreaterEqual(mutated_worse, 1)
        self.assertEqual(generations[-1]["children"], [])

    def test_recorded_draws_reproduce_the_initial_population(self):
        lcg = self.genetic["lcg"]
        state, draws = lcg["seed"], []
        for _ in range(16):
            state = (lcg["multiplier"] * state + lcg["increment"]) % lcg["modulus"]
            draws.append(state / lcg["modulus"])
        self.assertEqual((lcg["multiplier"], lcg["increment"], lcg["modulus"]), (48271, 0, 2 ** 31 - 1))
        population = self.genetic["generations"][0]["population"]
        for index, item in enumerate(population):
            self.assertAlmostEqual(item["design"]["wwr"], .1 + draws[2 * index] * .7)
            self.assertAlmostEqual(item["design"]["overhang"], draws[2 * index + 1] * 1.2)
        self.assertEqual(self.api.genetic(), self.api.genetic())

    def test_convergence_compares_three_methods_on_a_shared_run_budget(self):
        data = self.api.convergence()
        self.assertEqual((data["budget"], data["runs_map"]), (50, 1681))
        self.assertAlmostEqual(data["map_minimum"], self.optimum)
        self.assertEqual(set(data["series"]), {"gradient", "pattern", "genetic"})
        for method, series in data["series"].items():
            with self.subTest(method=method):
                self.assertTrue(48 <= len(series) <= 50)
                self.assertEqual([runs for runs, _ in series], list(range(1, len(series) + 1)))
                values = [value for _, value in series]
                self.assertTrue(all(a >= b for a, b in zip(values, values[1:])))
                self.assertLess(values[-1], 1.02 * self.optimum)
                self.assertAlmostEqual(values[-1], CASES["algorithms"]["convergence"][method], places=4)
        gradient, pattern, genetic = ([value for _, value in data["series"][key]] for key in ("gradient", "pattern", "genetic"))
        self.assertEqual(gradient[0], self.gradient[0]["electricity"])
        self.assertEqual(pattern[0], self.pattern[0]["electricity"])
        self.assertEqual(genetic[7], min(item["electricity"] for item in self.genetic["generations"][0]["population"]))
        self.assertEqual(genetic[-1], self.genetic["generations"][-1]["best"]["electricity"])
        self.assertGreater(len(set(round(value, 6) for value in genetic)), 4)
        self.assertLess(len(set(round(value, 6) for value in genetic)), 20)
        self.assertLess(gradient[29], 1.06 * self.optimum)
        self.assertGreater(genetic[9], gradient[29])
        self.assertGreater(gradient[0], 1.3 * self.optimum)


class SchemaFirstStudy(unittest.TestCase):
    """📜 The schema, the authored study and the invalid models share one contract."""

    def test_schema_file_exists_and_is_a_draft07_document_forbidding_extras(self):
        schema = json.loads((CLIP / "🧮️model/🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
        self.assertEqual(schema["$schema"], "http://json-schema.org/draft-07/schema#")
        self.assertIs(schema["additionalProperties"], False)
        self.assertEqual(set(schema["required"]), set(schema["properties"]))
        self.assertGreaterEqual(len(CASES["invalidModels"]), 10)

    def test_cases_have_no_legacy_expectations(self):
        for legacy in ("extrema", "generation", "landscape"):
            self.assertNotIn(legacy, CASES)
        api = model_api()
        for removed in ("landscape", "extrema", "factorial_effects", "normalized_index", "pareto_front", "generation", "load_examples"):
            self.assertFalse(hasattr(api, removed), removed)


class OptimizationClipContract(unittest.TestCase):
    """🎬 Require six standalone bilingual clips in the shared presentation."""

    def test_each_scene_has_bilingual_narration_and_native_discovery(self):
        from manim.utils.module_ops import scene_classes_from_file

        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                clauses = narration(chapter)
                self.assertGreaterEqual(len(clauses), 4)
                self.assertEqual(len({clause[0] for clause in clauses}), len(clauses))
                for clause in clauses:
                    self.assertEqual(len(clause), 3)
                    self.assertTrue(all(isinstance(text, str) and text.strip() for text in clause))
                    self.assertNotEqual(clause[1], clause[2])
                discovered = scene_classes_from_file(CLIP / chapter["folder"] / "🐍️.py", full_list=True)
                self.assertEqual([scene.__name__ for scene in discovered], [chapter["scene"]])

    def test_all_clauses_fit_both_languages_and_audiences(self):
        import importlib
        import os
        import tempfile
        from unittest.mock import patch
        from manim import tempconfig

        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
        visuals._fonts.apply_body_font()
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as folder, tempconfig({"media_dir": folder}):
            for chapter in CASES["chapters"]:
                for audience in CASES["audiences"]:
                    for index, language in enumerate(CASES["languages"], 1):
                        with patch.dict(os.environ, SEMIO_TEACHING_LANGUAGE=language, SEMIO_TEACHING_AUDIENCE=audience["id"]):
                            for clause in narration(chapter):
                                with self.subTest(video=chapter["video_id"], audience=audience["id"], language=language, clause=clause[0]):
                                    caption = visuals._art.caption_bar(clause[index])
                                    self.assertEqual(len(caption), 2 if audience["captions"] else 1)
                                    self.assertLessEqual(caption.width, visuals._engine.config.frame_width)
                                    self.assertGreaterEqual(caption.get_bottom()[1], -visuals._engine.config.frame_height / 2)

    def test_curriculum_registers_six_independent_videos(self):
        manifest = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
        videos = {video["id"]: video for video in manifest["videos"]}
        expected = [chapter["video_id"] for chapter in CASES["chapters"]]
        self.assertEqual([video["id"] for video in manifest["videos"] if video["id"] in expected], expected)
        self.assertNotIn("optimization", videos)
        outputs = []
        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                video = videos[chapter["video_id"]]
                self.assertEqual(video["script"], chapter["entry"])
                self.assertEqual(len(video["chapters"]), 1)
                self.assertEqual(len(video["chapters"][0]["segments"]), 1)
                segment = video["chapters"][0]["segments"][0]
                self.assertEqual(segment["scene"], chapter["scene"])
                self.assertEqual((ROOT / "🎓️teaching" / segment["script"]).resolve(), (CLIP / chapter["folder"] / "🐍️.py").resolve())
                outputs.append(video["output"])
        self.assertEqual(len(set(outputs)), 6)
        presentation = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json").read_text(encoding="utf-8"))
        topic = next(topic for topic in presentation["topics"] if topic["id"] == "optimization")
        self.assertEqual([video["id"] for video in topic["videos"]], expected)
        for video in topic["videos"]:
            self.assertEqual(set(video["title"]), set(CASES["languages"]))

    def test_entry_points_route_only_their_own_scenes(self):
        from unittest.mock import patch

        for chapter in CASES["chapters"]:
            with self.subTest(video=chapter["video_id"]):
                spec = importlib.util.spec_from_file_location(chapter["video_id"], ROOT / "🎓️teaching" / chapter["entry"])
                module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(module)
                self.assertEqual(module.VIDEO_ID, chapter["video_id"])
                for language in CASES["languages"]:
                    with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
                        module.compose(language=language, audience="speaker", chapters=[chapter["scene"]], force=True)
                    compose.assert_called_once_with([chapter["video_id"]], language=language, audience="speaker", chapter_scenes=[chapter["scene"]], force=True)

    def test_playlist_routes_the_six_videos(self):
        from unittest.mock import patch

        spec = importlib.util.spec_from_file_location("optimization_playlist", CLIP / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        expected = [chapter["video_id"] for chapter in CASES["chapters"]]
        self.assertEqual(list(module.VIDEO_IDS), expected)
        with patch.object(module._runtime, "compose_videos", return_value={}) as compose:
            module.compose(language="en", audience="spectator", force=True)
        compose.assert_called_once_with(expected, language="en", audience="spectator", chapter_scenes=None, force=True)


if __name__ == "__main__":
    unittest.main()
