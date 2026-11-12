"""🧪 Ventilation clips use declarative transitions only: no direct Manim transforms, every id resolves, durations equal the authored play run_time and the terminal frames are scheduled."""

import ast
import importlib
import json
import re
import sys
import unittest
from pathlib import Path

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIPS = Path(__file__).resolve().parent.parent
CASES = json.loads(Path(__file__).with_name("🔣️.transitions.json").read_text(encoding="utf-8"))
SPEC = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
TIMELINE = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
STAGES = ("stage", "morphs")


def clip_dirs():
    return sorted(p.parent for p in CLIPS.glob("*/🐍️.py") if p.parent.name != "🧪️architecture")


def parse(clip):
    return ast.parse((clip / "🐍️.py").read_text(encoding="utf-8"))


def parents(tree):
    return {child: node for node in ast.walk(tree) for child in ast.iter_child_nodes(node)}


def enclosing(node, up, kinds):
    while node in up:
        node = up[node]
        if isinstance(node, kinds):
            return node
    return None


def transition_calls(tree):
    return [n for n in ast.walk(tree) if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute) and n.func.attr == "transition" and isinstance(n.func.value, ast.Name) and n.func.value.id in STAGES]


def id_pattern(node):
    if isinstance(node, ast.Constant):
        return re.compile(re.escape(node.value))
    parts = [re.escape(v.value) if isinstance(v, ast.Constant) else r"[a-z0-9-]+" for v in node.values]
    return re.compile("".join(parts))


def document(clip):
    return SPEC.load(clip / "🔀️transitions" / "🔣️.json")


class VentilationTransitions(unittest.TestCase):
    def test_no_direct_manim_transforms(self):
        forbidden = set(CASES["forbidden"])
        for clip in clip_dirs():
            for node in ast.walk(parse(clip)):
                name = node.id if isinstance(node, ast.Name) else node.attr if isinstance(node, ast.Attribute) else None
                self.assertNotIn(name, forbidden, f"{clip.name}:{getattr(node, 'lineno', '?')} uses a direct Manim transform; declare it in 🔀️transitions/🔣️.json and play stage.transition(id)")

    def test_become_only_at_listed_analytical_or_static_sites(self):
        found = set()
        for clip in clip_dirs():
            tree = parse(clip)
            up = parents(tree)
            for node in ast.walk(tree):
                if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "become":
                    function = enclosing(node, up, (ast.FunctionDef,))
                    found.add((clip.name, function.name if function else "<module>"))
        self.assertEqual(found, {(c["clip"], c["function"]) for c in CASES["become"]})

    def test_documents_validate_and_counts(self):
        for clip in clip_dirs():
            doc = document(clip) if (clip / "🔀️transitions").is_dir() else None
            if doc is None:
                self.assertNotIn(clip.name, CASES["counts"])
                continue
            with self.subTest(clip=clip.name):
                self.assertEqual(SPEC.validate(doc), [])
                self.assertEqual(len(doc.transitions), CASES["counts"][clip.name])
                self.assertTrue(all(t.recipe == "replace" and t.timing.duration > 0 for t in doc.transitions))

    def test_every_call_resolves_and_every_transition_is_played(self):
        for clip in clip_dirs():
            if not (clip / "🔀️transitions").is_dir():
                continue
            with self.subTest(clip=clip.name):
                ids = [t.id for t in document(clip).transitions]
                patterns = [id_pattern(call.args[0]) for call in transition_calls(parse(clip))]
                for pattern in patterns:
                    self.assertTrue(any(pattern.fullmatch(i) for i in ids), f"no transition matches {pattern.pattern}")
                for ident in ids:
                    self.assertTrue(any(p.fullmatch(ident) for p in patterns), f"{ident} is declared but never played")

    def test_durations_equal_the_authored_play_run_time(self):
        derived = [(re.compile(d["pattern"]), d["duration"]) for d in CASES["derivedDurations"]]
        checked = 0
        for clip in clip_dirs():
            if not (clip / "🔀️transitions").is_dir():
                continue
            tree = parse(clip)
            up = parents(tree)
            durations = {t.id: t.timing.duration for t in document(clip).transitions}
            for call in transition_calls(tree):
                pattern = id_pattern(call.args[0])
                matched = [i for i in durations if pattern.fullmatch(i)]
                play = enclosing(call, up, (ast.Call,))
                while play is not None and not (isinstance(play.func, ast.Attribute) and play.func.attr == "play"):
                    play = enclosing(play, up, (ast.Call,))
                self.assertIsNotNone(play, f"{clip.name}:{call.lineno} transition outside a play call")
                run_time = next((k.value for k in play.keywords if k.arg == "run_time"), None)
                for ident in matched:
                    if isinstance(run_time, ast.Constant):
                        expected = run_time.value
                    else:
                        expected = next(d for p, d in derived if p.fullmatch(ident))
                    self.assertAlmostEqual(durations[ident], expected, places=9, msg=f"{clip.name}:{call.lineno} {ident}")
                    checked += 1
        self.assertGreaterEqual(checked, sum(CASES["counts"].values()))

    def test_every_transition_has_a_terminal_frame_on_every_output_rate(self):
        for clip in clip_dirs():
            if not (clip / "🔀️transitions").is_dir():
                continue
            for item in document(clip).transitions:
                for fps in CASES["fps"]:
                    schedule, window = TIMELINE.Schedule(fps), TIMELINE.Window(0.0, item.timing.duration)
                    n = len(np.arange(0, item.timing.duration, 1 / fps))
                    f0, f1 = TIMELINE.frames(window, schedule)
                    self.assertEqual((f1 - f0 + 1, f0), (n, 0), (item.id, fps))
                    self.assertEqual(TIMELINE.owner(window, schedule, f1), "target")
                    self.assertEqual(TIMELINE.owner(window, schedule, f1 - 1), "source" if n > 1 else "before")

    def test_teaching_critical_transitions(self):
        by_id = {t.id: t for clip in clip_dirs() if (clip / "🔀️transitions").is_dir() for t in document(clip).transitions}
        for case in CASES["critical"]:
            with self.subTest(case["id"]):
                item = by_id[case["id"]]
                self.assertEqual(item.recipe, "replace")
                self.assertAlmostEqual(item.timing.duration, case["duration"], places=9)
                for fps, (f0, f1) in case.get("frames", {}).items():
                    window = TIMELINE.Window(0.0, item.timing.duration)
                    self.assertEqual(TIMELINE.frames(window, TIMELINE.Schedule(int(fps))), (f0, f1))


if __name__ == "__main__":
    unittest.main()
