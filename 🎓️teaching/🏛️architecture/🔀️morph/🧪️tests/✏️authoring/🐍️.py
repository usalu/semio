"""🧪 Tests of morph authoring: edit, save, reload, refusal, byte-stable round trip, curriculum references, plans, export requests and CLI exit codes."""

from __future__ import annotations

import contextlib
import importlib
import io
import json
import shutil
import tempfile
import unittest
from pathlib import Path

authoring = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.✏️authoring.🐍️")
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
jsonschema = importlib.import_module("jsonschema")

CASES = json.loads((Path(__file__).parent / "🧫️cases" / "🔣️.json").read_text(encoding="utf-8"))
SCHEMA = json.loads((Path(authoring.__file__).parent / "🧬️schema" / "🔣️.schema.json").read_text(encoding="utf-8"))
ID = "demo.birth.capsules-to-leaves"


def cli(*argv):
    """⌨ Run the CLI in-process; return `(exit code, stdout, stderr)`."""
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        try:
            code = authoring.main(list(argv))
        except SystemExit as exit_:
            code = exit_.code
    return code, out.getvalue(), err.getvalue()


class Workspace(unittest.TestCase):
    """🗂 A temporary workspace with one clip folder, its transitions document and a curriculum scene that plays it."""

    def setUp(self):
        self.root = Path(tempfile.mkdtemp(prefix="morph-authoring-"))
        self.addCleanup(shutil.rmtree, self.root, ignore_errors=True)
        base = self.root / "🎓️teaching" / "🏛️architecture"
        self.clip = base / "demo" / "🎬️clip"
        (self.clip / "🔀️transitions").mkdir(parents=True)
        (base / "⚡️energy" / "🎬️clip").mkdir(parents=True)
        fixture = CASES["fixture"]
        (base / "⚡️energy" / "🎬️clip" / "🔣️.json").write_text(json.dumps(fixture["curriculum"]), encoding="utf-8")
        (self.clip / "🐍️.py").write_text(fixture["script"], encoding="utf-8")
        self.path = self.clip / "🔀️transitions" / "🔣️.json"
        self.path.write_text(spec.dumps(fixture["document"]), encoding="utf-8")

    def run_cli(self, *argv, language="en"):
        return cli("--language", language, "--root", str(self.root), *argv)

    def stored(self):
        return json.loads(self.path.read_text(encoding="utf-8"))["transitions"][0]


class CanonicalTests(Workspace):
    """🧬 Canonical serialization."""

    def test_round_trip_is_byte_stable(self):
        text = self.path.read_text(encoding="utf-8")
        self.assertTrue(text.endswith("}\n"))
        self.assertEqual(spec.dumps(json.loads(text)), text)
        self.assertEqual(spec.dumps(spec.dump(spec.parse(json.loads(text))) | {"$schema": CASES["fixture"]["document"]["$schema"]}), text)

    def test_two_space_indent_and_schema_first(self):
        lines = self.path.read_text(encoding="utf-8").splitlines()
        self.assertTrue(lines[1].startswith('  "$schema"'))
        self.assertEqual(lines[0], "{")

    def test_format_rewrites_noncanonical_file_and_validate_flags_it(self):
        self.path.write_text(json.dumps(json.loads(self.path.read_text(encoding="utf-8"))), encoding="utf-8")
        code, _, err = self.run_cli("validate")
        self.assertEqual(code, 1)
        self.assertIn("[format]", err)
        self.assertEqual(self.run_cli("format", str(self.clip))[0], 0)
        self.assertEqual(self.run_cli("validate")[0], 0)


class EditTests(Workspace):
    """✍ Validated edits, driven by language-agnostic JSON cases."""

    def test_cases(self):
        for case in CASES["edits"]:
            with self.subTest(case["name"]):
                self.setUp()
                before = self.path.read_bytes()
                code, out, err = self.run_cli("set", case["id"], case["pointer"], case["value"])
                self.assertEqual(code, case["exit"], err)
                if case["exit"] == 0:
                    stored = self.stored()
                    for pointer, value in case["expect"].items():
                        node = stored
                        for key in pointer[1:].split("/"):
                            node = node[key]
                        self.assertEqual(node, value)
                    self.assertEqual(self.path.read_text(encoding="utf-8"), spec.dumps(json.loads(self.path.read_text(encoding="utf-8"))))
                    self.assertEqual(spec.load(self.path).transition(case["id"]).id, case["id"])
                else:
                    self.assertIn(case["message"], err)
                    self.assertEqual(self.path.read_bytes(), before)
                self.assertEqual([p.name for p in self.clip.joinpath("🔀️transitions").iterdir()], ["🔣️.json"])

    def test_edit_save_reload_equal(self):
        self.assertEqual(self.run_cli("set", ID, "/timing/duration", "1.5")[0], 0)
        reloaded = spec.load(self.path)
        self.assertEqual(reloaded.transition(ID).timing.duration, 1.5)
        again = self.run_cli("set", ID, "/timing/duration", "1.5")
        self.assertEqual(again[0], 0)
        self.assertIn("nothing written", again[1])

    def test_refusal_message_names_location_and_fix(self):
        code, _, err = self.run_cli("set", ID, "/timing/easing", "bouncy")
        self.assertEqual(code, 1)
        self.assertIn("edit refused", err)
        self.assertIn("transitions[0].timing.easing", err)

    def test_german_messages(self):
        code, _, err = self.run_cli("set", "demo.nothing", "/timing/duration", "1", language="de")
        self.assertEqual(code, 1)
        self.assertIn("keinem Übergangsdokument", err)
        self.assertIn("Änderung abgelehnt", self.run_cli("set", ID, "/timing/duration", "-1", language="de")[2])

    def test_list_index_pointer_and_append(self):
        self.assertEqual(self.run_cli("set", ID, "/correspondence", '{"kind":"pairs","pairs":[["0","a"]]}')[0], 0)
        self.assertEqual(self.stored()["correspondence"]["pairs"], [["0", "a"]])
        self.assertEqual(self.run_cli("set", ID, "/correspondence/pairs/-", '["1","b"]')[0], 0)
        self.assertEqual(self.stored()["correspondence"]["pairs"], [["0", "a"], ["1", "b"]])
        self.assertEqual(self.run_cli("set", ID, "/correspondence/pairs/7", '["1","b"]')[0], 1)

    def test_duplicate_id_across_documents_refused(self):
        other = self.root / "🎓️teaching" / "🏛️architecture" / "other" / "🎬️clip" / "🔀️transitions"
        other.mkdir(parents=True)
        document = json.loads(self.path.read_text(encoding="utf-8"))
        document["transitions"][0]["id"] = "other.one"
        (other / "🔣️.json").write_text(spec.dumps(document), encoding="utf-8")
        code, _, err = self.run_cli("set", "other.one", "/id", ID)
        self.assertEqual(code, 1)
        self.assertIn("duplicate-id", err)


class ValidateTests(Workspace):
    """🔍 Validation of documents and curriculum references."""

    def test_valid_workspace(self):
        code, out, err = self.run_cli("validate")
        self.assertEqual((code, err), (0, ""))
        self.assertIn("1 transitions document(s) valid", out)

    def test_invalid_document_reports_actionable_problem(self):
        document = json.loads(self.path.read_text(encoding="utf-8"))
        document["transitions"][0]["timing"]["easing"] = "bouncy"
        self.path.write_text(json.dumps(document), encoding="utf-8")
        code, _, err = self.run_cli("validate", str(self.path))
        self.assertEqual(code, 1)
        self.assertIn("easing", err)

    def test_unreadable_and_malformed_files_fail(self):
        self.path.write_text("{ nope", encoding="utf-8")
        code, _, err = self.run_cli("validate")
        self.assertEqual(code, 1)
        self.assertIn("[json]", err)

    def test_scene_playing_undeclared_transition_fails(self):
        (self.clip / "🐍️.py").write_text(CASES["fixture"]["script"].replace(ID, "demo.missing"), encoding="utf-8")
        code, _, err = self.run_cli("validate")
        self.assertEqual(code, 1)
        self.assertIn("Beat1", err)
        self.assertIn("demo.missing", err)

    def test_unplayed_transition_is_a_warning_only(self):
        (self.clip / "🐍️.py").write_text("class Beat1:\n    pass\n", encoding="utf-8")
        code, out, err = self.run_cli("validate")
        self.assertEqual(code, 0)
        self.assertIn("not played", err)

    def record(self, *items):
        path = self.root / "transitions.json"
        path.write_text(json.dumps({"transitions": list(items)}), encoding="utf-8")
        return str(path)

    def test_runtime_record_matches_documents(self):
        ok = self.record({"id": ID, "start": 1.0, "duration": 1.2, "f0": 15, "f1": 32})
        self.assertEqual(self.run_cli("validate", "--played", ok)[0], 0)

    def test_runtime_record_with_unknown_id_or_changed_duration_fails(self):
        code, _, err = self.run_cli("validate", "--played", self.record({"id": "demo.ghost", "duration": 1.0}))
        self.assertEqual(code, 1)
        self.assertIn("[played-unknown]", err)
        code, _, err = self.run_cli("validate", "--played", self.record({"id": ID, "duration": 9.0}))
        self.assertEqual(code, 1)
        self.assertIn("[played-duration]", err)

    def test_list_and_show(self):
        code, out, _ = self.run_cli("list", "--json")
        row = json.loads(out)[0]
        self.assertEqual((code, row["id"], row["intent"], row["recipe"], row["duration"]), (0, ID, "abstraction", "parts", 1.2))
        code, out, _ = self.run_cli("show", ID)
        self.assertIn('"recipe": "parts"', out)
        self.assertEqual(self.run_cli("show", "demo.nothing")[0], 1)


class PlanTests(Workspace):
    """🗺 Resolved plans."""

    def plan(self, *extra):
        code, out, err = self.run_cli("preview-plan", ID, "--json", *extra)
        self.assertEqual(code, 0, err)
        return json.loads(out)

    def test_preview_plan_matches_manim_frame_count(self):
        result = self.plan("--profile", "preview", "--fps", "15", "--start", "2")
        self.assertEqual(result["frames"]["count"], 18)
        self.assertEqual((result["frames"]["first"], result["frames"]["terminal"]), (30, 47))
        self.assertEqual(result["frames"]["source"], [30, 46])
        self.assertFalse(result["work"]["fullMorph"])
        self.assertEqual(result["lod"]["source"]["lod"], "preview")
        numpy = importlib.import_module("numpy")
        self.assertEqual(len(numpy.arange(0, 1.2, 1 / 15)), 18)

    def test_final_plan_declares_full_morph_and_same_frames_per_fps(self):
        preview, final = self.plan("--profile", "preview", "--fps", "30"), self.plan("--profile", "final", "--fps", "30")
        self.assertEqual(preview["frames"], final["frames"])
        self.assertTrue(final["work"]["fullMorph"])
        self.assertEqual(final["work"]["recipe"], "parts")
        self.assertNotEqual(preview["fingerprint"], final["fingerprint"])

    def test_frames_owner_and_progress(self):
        samples = self.plan("--profile", "preview", "--fps", "15", "--frames")["samples"]
        self.assertEqual([s["owner"] for s in samples], ["source"] * 17 + ["target"])
        self.assertEqual(samples[0]["progress"], 0.0)
        self.assertEqual(samples[-1]["progress"], 1.0)

    def test_affine_from_bounds(self):
        placement = self.plan("--profile", "preview", "--source-bounds", "0,0,2,1", "--target-bounds", "0,0,4,2")["placement"]
        self.assertEqual(placement["affine"], {"scale": [2.0, 2.0], "translation": [0.0, 0.0]})

    def test_zero_duration_is_atomic(self):
        self.assertEqual(self.run_cli("set", ID, "/timing/duration", "0")[0], 1)
        self.assertEqual(self.run_cli("set", ID, "/timing/lag", "0")[0], 0)
        self.assertEqual(self.run_cli("set", ID, "/timing/duration", "0")[0], 0)
        result = self.plan("--profile", "preview", "--fps", "15")
        self.assertEqual((result["frames"]["count"], result["frames"]["source"]), (0, None))

    def test_profile_is_required_and_validated(self):
        self.assertEqual(self.run_cli("preview-plan", ID)[0], 2)
        self.assertEqual(self.run_cli("preview-plan", ID, "--profile", "draft")[0], 2)
        self.assertEqual(self.run_cli("preview-plan", ID, "--profile", "preview", "--source-bounds", "1,2")[0], 1)


class RequestTests(Workspace):
    """📦 Export requests persist an explicit profile."""

    def write_request(self, *flags):
        target = self.root / "request.json"
        return self.run_cli("request", ID, "--output", str(target), *flags), target

    def test_request_is_schema_valid_and_carries_profile(self):
        for purpose, profile in (("review", "preview"), ("review", "final"), ("publication", "final")):
            with self.subTest(purpose=purpose, profile=profile):
                (code, _, err), target = self.write_request("--profile", profile, "--purpose", purpose)
                self.assertEqual(code, 0, err)
                value = json.loads(target.read_text(encoding="utf-8"))
                jsonschema.validate(value, SCHEMA)
                self.assertEqual((value["profile"], value["purpose"], value["language"], value["transitions"]), (profile, purpose, "en", [ID]))
                self.assertEqual(target.read_text(encoding="utf-8"), authoring.request_text(value))

    def test_publication_cannot_use_preview(self):
        (code, _, err), target = self.write_request("--profile", "preview", "--purpose", "publication")
        self.assertEqual(code, 1)
        self.assertIn("final", err)
        self.assertFalse(target.exists())

    def test_profile_and_purpose_are_required(self):
        self.assertEqual(self.write_request("--purpose", "review")[0][0], 2)
        self.assertEqual(self.write_request("--profile", "final")[0][0], 2)

    def test_unknown_transition_refused(self):
        code, _, err = self.run_cli("request", "demo.nothing", "--profile", "final", "--purpose", "review", "--output", str(self.root / "r.json"))
        self.assertEqual(code, 1)
        self.assertIn("not declared", err)

    def test_schema_rejects_preview_publication(self):
        bad = {"version": 1, "purpose": "publication", "profile": "preview", "fingerprint": "x", "language": "en", "transitions": [ID]}
        with self.assertRaises(jsonschema.ValidationError):
            jsonschema.validate(bad, SCHEMA)


class CommandLineTests(Workspace):
    """⌨ Exit codes and language handling."""

    def test_language_is_required(self):
        for argv in ([], ["list"], ["--root", "x", "list"], ["-h"]):
            code, _, err = cli(*argv)
            self.assertEqual(code, 2)
            self.assertIn("--language", err)
            self.assertIn("Standardsprache", err)

    def test_unknown_language_and_command_are_usage_errors(self):
        self.assertEqual(cli("--language", "fr", "list")[0], 2)
        self.assertEqual(cli("--language", "en", "--root", str(self.root), "dance")[0], 2)

    def test_format_never_defaults_to_the_whole_repository(self):
        code, _, err = self.run_cli("format")
        self.assertEqual(code, 2)
        self.assertIn("paths", err)

    def test_help_follows_the_language(self):
        self.assertIn("Author the declarative morph", cli("--language", "en", "-h")[1])
        self.assertIn("Die deklarativen Morph", cli("--language", "de", "-h")[1])

    def test_real_repository_validates(self):
        self.assertEqual(cli("--language", "en", "validate")[0], 0)


if __name__ == "__main__":
    unittest.main()
