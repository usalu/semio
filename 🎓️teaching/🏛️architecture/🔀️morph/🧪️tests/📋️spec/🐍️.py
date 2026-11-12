"""🧪 Tests of the declarative morph spec and stage: language-agnostic JSON cases, third-party jsonschema oracle, persist/reload round trip, stage binding and dispatch."""

from __future__ import annotations

import copy
import importlib
import json
import math
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

import numpy as np

jsonschema = importlib.import_module("jsonschema")
manim = importlib.import_module("manim")
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")

CASES = Path(__file__).resolve().parent / "🧫️cases"
BASE = json.loads((CASES / "base.json").read_text(encoding="utf-8"))
ORACLE = jsonschema.Draft202012Validator(spec.SCHEMA)


def _walk(pointer, document):
    parts = [p.replace("~1", "/").replace("~0", "~") for p in pointer.split("/")[1:]]
    node = document
    for part in parts[:-1]:
        node = node[int(part)] if isinstance(node, list) else node[part]
    return node, parts[-1]


def apply(case):
    """🧰 Apply a case's `set`/`remove` JSON-pointer edits to a copy of the base document."""
    document = copy.deepcopy(BASE)
    for pointer, value in case.get("set", {}).items():
        if not pointer.startswith("/"):
            document[pointer] = value
            continue
        node, last = _walk(pointer, document)
        if isinstance(node, list):
            index = int(last)
            node.append(value) if index == len(node) else node.__setitem__(index, value)
        else:
            node[last] = value
    for pointer in case.get("remove", []):
        node, last = _walk(pointer, document)
        del node[int(last) if isinstance(node, list) else last]
    return document


class FakeBound:
    """🧸 Stage duck type entry for validator tests."""

    def __init__(self, parts, revision, provides):
        self.parts, self.revision, self.provides = parts, revision, provides

    def part_names(self):
        return None if self.parts is None else tuple(self.parts)


class FakeStage:
    """🧸 Stage duck type for validator tests."""

    def __init__(self, bindings, correspondences):
        self.items = {k: FakeBound(v["parts"], v["revision"], v["provides"]) for k, v in bindings.items()}
        self.correspondences = None if correspondences is None else frozenset(correspondences)

    def bound(self, entity_id):
        return self.items.get(entity_id)


def keywords(node, found):
    """🔑 Collect every schema keyword used in `node` (descending only into schema positions)."""
    if isinstance(node, dict):
        found.update(node)
        for key in ("properties", "$defs"):
            for child in node.get(key, {}).values():
                keywords(child, found)
        for key in ("items", "if", "then"):
            keywords(node.get(key), found)
        for child in node.get("allOf", []):
            keywords(child, found)
    return found


class SchemaOracleTests(unittest.TestCase):
    def test_schema_is_a_valid_draft_2020_12_schema(self):
        jsonschema.Draft202012Validator.check_schema(spec.SCHEMA)

    def test_interpreter_supports_every_schema_keyword(self):
        unsupported = {k for k in keywords(spec.SCHEMA, set()) if k not in spec.SUPPORTED_KEYWORDS and k not in self._property_names()}
        self.assertEqual(unsupported, set())

    @staticmethod
    def _property_names():
        names = set()

        def visit(node):
            if isinstance(node, dict):
                names.update(node.get("properties", {}))
                for value in node.values():
                    visit(value)
            elif isinstance(node, list):
                for value in node:
                    visit(value)

        visit(spec.SCHEMA)
        return names

    def test_valid_cases_are_valid_for_oracle_and_spec(self):
        for case in json.loads((CASES / "valid.json").read_text(encoding="utf-8")):
            with self.subTest(case["name"]):
                document = apply(case)
                self.assertEqual(list(ORACLE.iter_errors(document)), [])
                self.assertEqual([str(p) for p in spec.validate(document)], [])

    def test_invalid_cases_report_expected_problems_and_match_oracle(self):
        for case in json.loads((CASES / "invalid.json").read_text(encoding="utf-8")):
            with self.subTest(case["name"]):
                document = apply(case)
                problems = spec.validate(document)
                found = {(p.path, p.code) for p in problems}
                for expected in case["expect"]:
                    self.assertIn((expected["path"], expected["code"]), found, [str(p) for p in problems])
                self.assertEqual(ORACLE.is_valid(document), case["schemaValid"])
                self.assertEqual(spec.structural(document) == [], case["schemaValid"])
                self.assertTrue(problems)
                for problem in problems:
                    self.assertTrue(problem.path.startswith("$"), problem)
                    self.assertIn("Fix:", problem.message, problem)

    def test_oracle_agrees_on_every_case_validity(self):
        for name in ("valid", "invalid"):
            for case in json.loads((CASES / f"{name}.json").read_text(encoding="utf-8")):
                document = apply(case)
                self.assertEqual(spec.structural(document) == [], ORACLE.is_valid(document), case["name"])

    def test_non_finite_durations_are_rejected(self):
        for bad in (math.nan, math.inf, -math.inf):
            document = copy.deepcopy(BASE)
            document["transitions"][0]["timing"]["duration"] = bad
            self.assertIn("duration", {p.code for p in spec.validate(document)}, bad)

    def test_messages_include_close_match_hint(self):
        document = copy.deepcopy(BASE)
        document["transitions"][0]["recepie"] = "parts"
        message = spec.validate(document)[0].message
        self.assertIn("recipe", message)


class StageValidationTests(unittest.TestCase):
    def test_stage_cases(self):
        data = json.loads((CASES / "stage.json").read_text(encoding="utf-8"))
        for case in data["cases"]:
            with self.subTest(case["name"]):
                bindings = {**data["bindings"], **case.get("bind", {})}
                for gone in case.get("unbind", []):
                    bindings.pop(gone, None)
                document = apply(case)
                problems = spec.validate(document, FakeStage(bindings, data["correspondences"]), only=case.get("only"))
                found = {(p.path, p.code) for p in problems}
                self.assertEqual(found if case["expect"] == [] else found >= {(e["path"], e["code"]) for e in case["expect"]}, set() if case["expect"] == [] else True, [str(p) for p in problems])

    def test_unknown_correspondence_names_are_skipped_without_registry(self):
        data = json.loads((CASES / "stage.json").read_text(encoding="utf-8"))
        document = apply({"set": {"/transitions/0/correspondence": {"kind": "recipe", "name": "mystery"}}})
        problems = spec.validate(document, FakeStage(data["bindings"], None), only="trees.birth.capsules-to-leaves")
        self.assertEqual(problems, [])


class RoundTripTests(unittest.TestCase):
    def test_persist_and_reload_is_equal(self):
        with tempfile.TemporaryDirectory() as folder:
            first = Path(folder) / "a.json"
            first.write_text(json.dumps(BASE, ensure_ascii=False), encoding="utf-8")
            loaded = spec.load(first)
            second = Path(folder) / "b.json"
            second.write_text(json.dumps(spec.dump(loaded), ensure_ascii=False, indent=2), encoding="utf-8")
            reloaded = spec.load(second)
            self.assertEqual(loaded, reloaded)
            self.assertEqual(spec.dump(reloaded), spec.dump(loaded))
            self.assertTrue(ORACLE.is_valid(spec.dump(loaded)))

    def test_defaults_become_explicit(self):
        document = spec.parse(BASE)
        relabel = document.transition("trees.leaf.relabel")
        self.assertEqual(relabel.timing, spec.Timing(0.8, "smooth", 0.0, None))
        self.assertEqual(relabel.placement, spec.Placement("scene", "center", "contain", True))
        self.assertEqual(relabel.final, spec.Final(0.0, 0.0, 0.0))
        self.assertEqual(relabel.source.view, None)
        self.assertEqual(document.groups[0].transitions[1], "trees.leaf.relabel")

    def test_every_case_document_round_trips(self):
        for case in json.loads((CASES / "valid.json").read_text(encoding="utf-8")):
            document = spec.parse(apply(case))
            self.assertEqual(spec.parse(spec.dump(document)), document, case["name"])

    def test_documents_are_frozen_and_hashable(self):
        document = spec.parse(BASE)
        with self.assertRaises(Exception):
            document.transitions[0].id = "x"
        self.assertEqual(hash(document), hash(spec.parse(BASE)))

    def test_load_reports_actionable_file_errors(self):
        with tempfile.TemporaryDirectory() as folder:
            missing = Path(folder) / "none.json"
            with self.assertRaises(spec.SpecError) as raised:
                spec.load(missing)
            self.assertEqual(raised.exception.problems[0].code, "file")
            broken = Path(folder) / "broken.json"
            broken.write_text("{", encoding="utf-8")
            with self.assertRaises(spec.SpecError) as raised:
                spec.load(broken)
            self.assertEqual(raised.exception.problems[0].code, "json")
            nan = Path(folder) / "nan.json"
            nan.write_text('{"version": 1, "transitions": [{"timing": {"duration": NaN}}]}', encoding="utf-8")
            with self.assertRaises(spec.SpecError) as raised:
                spec.load(nan)
            self.assertEqual(raised.exception.problems[0].code, "json")

    def test_parse_raises_with_all_problems(self):
        bad = copy.deepcopy(BASE)
        bad["transitions"][0]["timing"]["duration"] = -1
        bad["transitions"][0]["recipe"] = "melt"
        with self.assertRaises(spec.SpecError) as raised:
            spec.parse(bad)
        self.assertEqual({p.code for p in raised.exception.problems}, {"duration", "recipe"})


class Made:
    """🧸 Weak-referenceable stand-in for an animation."""

    def __init__(self, name):
        self.name = name


def scene():
    with manim.tempconfig({"write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR", "save_last_frame": False, "progress_bar": "none"}):
        return manim.Scene(skip_animations=True)


class StageTests(unittest.TestCase):
    def setUp(self):
        self.folder = tempfile.TemporaryDirectory()
        self.addCleanup(self.folder.cleanup)
        self.path = Path(self.folder.name) / "t.json"
        self.path.write_text(json.dumps(BASE, ensure_ascii=False), encoding="utf-8")
        self.calls = []
        self.profile = SimpleNamespace(morph="swap", lod="low")
        self.originals = (stage_module._profile, stage_module._module)
        stage_module._profile = lambda: self.profile
        preview = SimpleNamespace(PreviewSwap=lambda *a, **k: self.calls.append(("preview", a, k)) or Made("swap-animation"))
        full = SimpleNamespace(execute=lambda *a, **k: self.calls.append(("full", a, k)) or Made("full-animation"), CORRESPONDENCES={"capsule-item"})
        modules = {stage_module.PREVIEW_MODULE: preview, stage_module.FULL_MODULE: full}
        stage_module._module = modules.__getitem__
        self.addCleanup(lambda: (setattr(stage_module, "_profile", self.originals[0]), setattr(stage_module, "_module", self.originals[1])))

    def bound(self, factory_log=None):
        st = stage_module.stage(scene(), self.path)
        st.bind("tower.capsules", {"a": manim.Square(), "b": manim.Circle().shift(2 * manim.RIGHT)})
        log = factory_log if factory_log is not None else []

        def leaves(lod):
            log.append(lod)
            return manim.VGroup(manim.Dot(), manim.Dot())

        st.bind("diagram.leaves", leaves, bounds=((0, 0, 0), (2, 1, 0)))
        return st

    def test_swap_profile_dispatches_to_preview_swap_with_resolved_subjects(self):
        log = []
        st = self.bound(log)
        self.assertEqual(log, [])
        self.assertEqual(st.transition("trees.birth.capsules-to-leaves").name, "swap-animation")
        kind, args, kwargs = self.calls[0]
        source, target = args
        self.assertEqual((kind, log, target.lod, source.id, target.id), ("preview", ["low"], "low", "tower.capsules", "diagram.leaves"))
        self.assertEqual(kwargs["transition"].id, "trees.birth.capsules-to-leaves")
        self.assertEqual(set(source.parts), {"a", "b"})
        np.testing.assert_allclose(target.bounds, [[0, 0, 0], [2, 1, 0]])

    def test_full_profile_dispatches_to_execute_and_factories_get_requested_lod_once(self):
        log = []
        st = self.bound(log)
        self.profile = SimpleNamespace(morph="full", lod="high")
        self.assertEqual(st.transition("trees.birth.capsules-to-leaves").name, "full-animation")
        st.transition("trees.birth.capsules-to-leaves")
        self.assertEqual((self.calls[0][0], log), ("full", ["high"]))

    def test_unknown_profile_morph_is_rejected(self):
        self.profile = SimpleNamespace(morph="turbo", lod="low")
        with self.assertRaisesRegex(ValueError, "turbo"):
            self.bound().transition("trees.birth.capsules-to-leaves")

    def test_bounds_and_anchors_freeze_at_bind(self):
        st = stage_module.stage(scene(), self.path)
        square = manim.Square(side_length=2)
        st.bind("tower.capsules", square)
        before = st.bound("tower.capsules").bounds.copy()
        square.shift(5 * manim.RIGHT)
        np.testing.assert_allclose(st.bound("tower.capsules").bounds, before)
        np.testing.assert_allclose(st.bound("tower.capsules").anchors["bottom"], [0, -1, 0], atol=1e-9)
        with self.assertRaises(ValueError):
            st.bound("tower.capsules").bounds[0, 0] = 1

    def test_factory_requires_explicit_bounds_and_double_bind_fails(self):
        st = stage_module.stage(scene(), self.path)
        with self.assertRaisesRegex(ValueError, "bounds"):
            st.bind("diagram.leaves", lambda lod: manim.Dot())
        st.bind("tower.capsules", manim.Dot())
        with self.assertRaisesRegex(ValueError, "already bound"):
            st.bind("tower.capsules", manim.Dot())

    def test_unbound_entity_raises_spec_error_with_fix(self):
        st = stage_module.stage(scene(), self.path)
        st.bind("tower.capsules", manim.Dot())
        with self.assertRaises(spec.SpecError) as raised:
            st.transition("trees.birth.capsules-to-leaves")
        self.assertEqual(raised.exception.problems[0].code, "unbound-entity")
        self.assertIn("stage.bind", raised.exception.problems[0].message)

    def test_zero_duration_requires_cut_and_cut_swaps_atomically(self):
        data = copy.deepcopy(BASE)
        data["transitions"][1]["timing"]["duration"] = 0
        self.path.write_text(json.dumps(data), encoding="utf-8")
        host = scene()
        st = stage_module.stage(host, self.path)
        before, after = manim.Square(), manim.Circle()
        st.bind("tree.leaf", before)
        host.add(before)
        with self.assertRaisesRegex(ValueError, "cut"):
            st.transition("trees.leaf.relabel")
        self.assertEqual(st.played, ())
        with self.assertRaises(spec.SpecError):
            st.cut("trees.leaf.relabel").__class__
        self.assertEqual(self.calls, [])

    def test_cut_swaps_source_for_target_and_records_play(self):
        data = copy.deepcopy(BASE)
        self.profile.morph = "full"
        data["transitions"][1].update({"source": {"entity": "tree.old"}, "target": {"entity": "tree.new"}, "correspondence": {"kind": "order"}, "timing": {"duration": 0}})
        self.path.write_text(json.dumps(data), encoding="utf-8")
        host = scene()
        st = stage_module.stage(host, self.path)
        old, new = manim.Square(), manim.Circle()
        st.bind("tree.old", old)
        st.bind("tree.new", new)
        host.add(old)
        st.cut("trees.leaf.relabel")
        self.assertNotIn(old, host.mobjects)
        self.assertTrue(any(m is new for m in host.mobjects))
        self.assertEqual(st.played, ("trees.leaf.relabel",))
        self.assertEqual(self.calls, [])

    def test_vanish_passes_a_null_target_and_cut_only_removes(self):
        data = copy.deepcopy(BASE)
        data["transitions"][1].update({"target": None, "recipe": "vanish", "intent": "abstraction"})
        del data["transitions"][1]["correspondence"]
        self.path.write_text(json.dumps(data), encoding="utf-8")
        host = scene()
        st = stage_module.stage(host, self.path)
        leaf = manim.Square()
        st.bind("tree.leaf", leaf)
        self.assertEqual(st.transition("trees.leaf.relabel").name, "swap-animation")
        source, target = self.calls[0][1]
        self.assertEqual((source.id, target), ("tree.leaf", None))
        data["transitions"][1]["timing"] = {"duration": 0}
        self.profile.morph = "full"
        self.path.write_text(json.dumps(data), encoding="utf-8")
        host = scene()
        st = stage_module.stage(host, self.path)
        st.bind("tree.leaf", leaf)
        host.add(leaf)
        st.cut("trees.leaf.relabel")
        self.assertEqual(host.mobjects, [])

    def test_cut_rejects_positive_duration(self):
        st = self.bound()
        with self.assertRaisesRegex(ValueError, "transition"):
            st.cut("trees.birth.capsules-to-leaves")
        self.assertEqual(st.played, ())

    def test_played_records_requested_transitions_in_order(self):
        st = self.bound()
        st.transition("trees.birth.capsules-to-leaves")
        self.assertEqual(st.played, ("trees.birth.capsules-to-leaves",))

    def test_staffage_binder_extension_point(self):
        seen = []
        stage_module.Stage.register_binder("probe", lambda st, entity, *a, **k: seen.append((entity, a, k)) or st.bind(entity, manim.Dot()))
        self.addCleanup(stage_module.Stage.binders.pop, "probe")
        st = stage_module.stage(scene(), self.path)
        st.bind_with("probe", "tower.capsules", "ref", lod="x")
        self.assertEqual(seen, [("tower.capsules", ("ref",), {"lod": "x"})])
        self.assertIsNotNone(st.bound("tower.capsules"))
        with self.assertRaisesRegex(KeyError, "binder"):
            st.bind_with("missing", "x")

    def test_mobject_resolves_at_active_lod_lazily(self):
        log = []
        st = self.bound(log)
        self.assertEqual(len(st.mobject("diagram.leaves").submobjects), 2)
        self.assertEqual(log, ["low"])
        with self.assertRaises(KeyError):
            st.mobject("nope")


class FacadeTests(unittest.TestCase):
    def test_facade_exports_declarative_api_and_toolkit(self):
        morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
        for name in ("stage", "load", "validate", "parse", "dump", "token", "travel", "tether"):
            self.assertTrue(callable(getattr(morph, name)), name)


if __name__ == "__main__":
    unittest.main()
