"""🧪 Acceptance case 7: persist, drop every in-process cache, reopen and render a real migrated transition with identical frames and records; then a real authoring edit shows up in playback frames, the runtime record and `preview-plan`.

Each render runs in a fresh interpreter (`child`), so no cache, module state or profile survives between renders.
"""

from __future__ import annotations

import contextlib
import hashlib
import importlib
import io
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

CASES = json.loads((Path(__file__).parent / "🧫️cases" / "🔣️.json").read_text(encoding="utf-8"))
TEACHING = ROOT / "🎓️teaching"
PROFILE_MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️"


def child(profile, document, transition, out):
    """🎞 Render one transition of `document` at the profile's schedule in this fresh process and write frame hashes, counters and the stage record."""
    engine = importlib.import_module("manim")
    profiles = importlib.import_module(PROFILE_MODULE)
    instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
    morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")
    job = profiles.job()
    stand = CASES["standIns"]
    frames = []
    options = {"write_to_movie": False, "disable_caching": True, "progress_bar": "none", "verbosity": "ERROR", "save_last_frame": False}
    with engine.tempconfig(options):
        job.apply(engine.config)
        scene = engine.Scene()
        stage = morph.stage(scene, document)
        source = engine.Square(side_length=stand["source"]["side"], color=stand["source"]["color"], fill_opacity=0.8).move_to(stand["source"]["at"])
        target = engine.Circle(radius=stand["target"]["radius"], color=stand["target"]["color"], fill_opacity=0.8).move_to(stand["target"]["at"])
        scene.add(source)
        original = scene.update_to_time

        def spy(t):
            original(t)
            scene.renderer.update_frame(scene)
            frames.append(hashlib.sha256(scene.renderer.camera.pixel_array.tobytes()).hexdigest())

        scene.update_to_time = spy
        scene.play(stage.transition(transition, source=source, target=target))
        played = len(frames)
        scene.wait(0.2)
    instrument.write(Path(out).parent / "work.json")
    Path(out).write_text(json.dumps({"frames": frames, "played": played, "fps": job.fps}, indent=2) + "\n", encoding="utf-8")


class Persistence(unittest.TestCase):
    """💾 Persist, reopen, render."""

    @classmethod
    def setUpClass(cls):
        cls.authoring = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.✏️authoring.🐍️")
        cls.profiles = importlib.import_module(PROFILE_MODULE)

    def setUp(self):
        self.root = Path(tempfile.mkdtemp(prefix="morph-persist-"))
        self.addCleanup(shutil.rmtree, self.root, ignore_errors=True)
        self.document = self.root / "🎓️teaching" / CASES["document"]
        self.document.parent.mkdir(parents=True)
        shutil.copyfile(TEACHING / CASES["document"], self.document)
        self.id = CASES["transition"]

    def cli(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = self.authoring.main(["--language", "en", "--root", str(self.root), *argv])
        return code, out.getvalue(), err.getvalue()

    def render(self, profile, label):
        """🎞 Run `child` in a fresh interpreter; return `(frames record, work.json text, transitions.json text)`."""
        folder = self.root / "renders" / f"{label}-{profile}"
        folder.mkdir(parents=True)
        env = {**os.environ, "PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8", "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_TEACHING_WORK": str(folder / "work.json"),
               **self.profiles.environment(profile, job_id="persist", resolution=CASES["resolution"])}
        done = subprocess.run([sys.executable, str(Path(__file__).resolve()), "child", profile, str(self.document), self.id, str(folder / "frames.json")], env=env, capture_output=True, text=True, encoding="utf-8", cwd=ROOT, timeout=900)
        self.assertEqual(done.returncode, 0, done.stderr[-3000:])
        return (json.loads((folder / "frames.json").read_text(encoding="utf-8")), (folder / "work.json").read_text(encoding="utf-8"), (folder / "transitions.json").read_text(encoding="utf-8"))

    def plan(self, profile):
        code, out, err = self.cli("preview-plan", self.id, "--profile", profile, "--json")
        self.assertEqual(code, 0, err)
        return json.loads(out)

    def test_identity_round_trip_and_real_edit(self):
        original = self.document.read_bytes()
        before = {p: self.render(p, "before") for p in CASES["profiles"]}
        for profile in CASES["profiles"]:
            frames, work, record = before[profile]
            self.assertEqual(frames["played"], self.plan(profile)["frames"]["count"], "playback frame count equals the authoring plan")
            self.assertGreater(len(set(frames["frames"][:frames["played"]])), 1, "the transition frames actually change")
            counts = json.loads(work)["counts"]
            worked = counts.get("full.prepare", 0) + counts.get("full.evaluate", 0) > 0
            self.assertEqual(worked, CASES["expect"][profile]["fullMorphWork"], counts)
            self.assertEqual(json.loads(record)["transitions"][0]["duration"], CASES["edit"]["before"])

        self.document.write_text(json.dumps(json.loads(original.decode("utf-8"))), encoding="utf-8")
        self.assertEqual(self.cli("format", str(self.document.parent))[0], 0)
        self.assertEqual(self.cli("set", self.id, "/timing/duration", str(CASES["edit"]["before"]))[0], 0)
        self.assertEqual(self.document.read_bytes(), original, "persist is byte-stable")
        after = {p: self.render(p, "after") for p in CASES["profiles"]}
        for profile in CASES["profiles"]:
            self.assertEqual(after[profile][0], before[profile][0], f"{profile}: frame hashes identical after reopening")
            self.assertEqual(after[profile][1], before[profile][1], f"{profile}: work.json identical")
            self.assertEqual(after[profile][2], before[profile][2], f"{profile}: transitions.json identical")

        edit = CASES["edit"]
        self.assertEqual(self.cli("set", self.id, edit["pointer"], edit["value"])[0], 0)
        self.assertNotEqual(self.document.read_bytes(), original)
        for profile in CASES["profiles"]:
            frames, _, record = self.render(profile, "edited")
            plan = self.plan(profile)
            self.assertEqual(plan["duration"], edit["after"])
            self.assertEqual(frames["played"], plan["frames"]["count"])
            self.assertLess(frames["played"], before[profile][0]["played"])
            self.assertNotEqual(frames["frames"], before[profile][0]["frames"])
            self.assertEqual(json.loads(record)["transitions"][0]["duration"], edit["after"])
            self.assertEqual(plan["frames"]["terminal"], json.loads(record)["transitions"][0]["f1"])


if __name__ == "__main__":
    if len(sys.argv) == 6 and sys.argv[1] == "child":
        child(*sys.argv[2:])
    else:
        unittest.main()
