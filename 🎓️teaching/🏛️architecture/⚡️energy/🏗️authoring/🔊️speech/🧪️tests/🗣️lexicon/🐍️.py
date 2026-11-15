"""🗣️ Spoken lexicon: TeX to words in German and English, whole-symbol priority, loud failures, schema, recording identity."""

from __future__ import annotations

import importlib
import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import jsonschema

LEXICON = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🗣️lexicon.🐍️")
SPEECH = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
TEX = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧮️tex.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
LEXICON_DIR = Path(LEXICON.__file__).parent


class SpokenFormTest(unittest.TestCase):
    def test_math_is_spoken_in_both_languages(self):
        for case in CASES["math"]:
            for language in ("de", "en"):
                with self.subTest(tex=case["tex"], language=language):
                    self.assertEqual(LEXICON.spoken_math(case["tex"], language), case[language])

    def test_clauses_keep_prose_and_speak_segments(self):
        for case in CASES["clauses"]:
            for language in ("de", "en"):
                with self.subTest(text=case["text"], language=language):
                    self.assertEqual(LEXICON.spoken_text(case["text"], language), case[language])

    def test_no_language_is_assumed(self):
        with self.assertRaises(LEXICON.SpeechError):
            LEXICON.spoken_math("x", "fr")
        with self.assertRaises(TypeError):
            LEXICON.spoken_clauses([("a", "x", "y")])

    def test_unspeakable_tex_fails_loudly_with_clause_and_file(self):
        for case in CASES["failures"]:
            with self.subTest(tex=case["tex"]):
                with self.assertRaises(LEXICON.SpeechError) as raised:
                    LEXICON.spoken_math(case["tex"], "de")
                self.assertIn(case["contains"], str(raised.exception))
        narration = [("ok", "fine", "fein"), ("bad", "Value $\\mathrm{Foo}$", "Wert $\\mathrm{Foo}$")]
        with self.assertRaises(LEXICON.SpeechError) as raised:
            LEXICON.spoken_clauses(narration, language="de", source="some/file.py")
        self.assertIn("some/file.py", str(raised.exception))
        self.assertIn("'bad'", str(raised.exception))

    def test_whole_symbol_overrides_win_over_composition(self):
        changed = json.loads(json.dumps(LEXICON.LEXICON))
        changed["symbols"]["\\Delta\\theta"] = {"de": "Temperaturdifferenz", "en": "temperature difference"}
        with mock.patch.object(LEXICON, "LEXICON", changed):
            self.assertEqual(LEXICON._Speaker("de").sequence(TEX.parse_tex("\\Delta\\theta = 5")), "Temperaturdifferenz gleich 5")
        self.assertEqual(LEXICON.spoken_math("\\Delta\\theta = 5", "de"), "Delta Theta gleich 5")

    def test_every_tex_symbol_has_a_spoken_name_in_both_languages(self):
        names = LEXICON.LEXICON["names"]
        for name, entry in TEX.SYMBOLS.items():
            if "alias" in entry:
                continue
            with self.subTest(symbol=name):
                self.assertIn(name, names)
                self.assertTrue(names[name]["de"] and names[name]["en"])

    def test_every_unit_speaks_in_both_languages(self):
        for unit in LEXICON.LEXICON["units"]:
            for language in ("de", "en"):
                with self.subTest(unit=unit, language=language):
                    src = "1\\,\\mathrm{" + unit.replace("%", "\\%") + "}" if unit != "°C" and unit != "°F" else "1\\,\\mathrm{^\\circ " + unit[-1] + "}"
                    self.assertTrue(LEXICON.spoken_math(src, language).startswith("1 "))


class LexiconDataTest(unittest.TestCase):
    def test_data_matches_its_schema(self):
        schema = json.loads((LEXICON_DIR / "🧬️schema.json").read_text(encoding="utf-8"))
        jsonschema.validate(LEXICON.LEXICON, schema)

    def test_forbidden_spellings(self):
        for case in CASES["forbidden"]:
            with self.subTest(text=case["text"]):
                found = sorted({rule for rule, _, _ in LEXICON.forbidden_findings(case["text"])})
                self.assertEqual(found, sorted(set(case["rules"])))


class RecordingIdentityTest(unittest.TestCase):
    def test_changed_pronunciation_changes_identity_and_unchanged_audio_is_reused(self):
        backend = SPEECH._SpeechBackend()
        first = SPEECH.clause_identity(LEXICON.spoken_text("Wert $\\Delta\\theta$", "de"), backend=backend)
        again = SPEECH.clause_identity(LEXICON.spoken_text("Wert $\\Delta\\theta$", "de"), backend=backend)
        other = SPEECH.clause_identity(LEXICON.spoken_text("Wert $\\Delta\\theta_{i}$", "de"), backend=backend)
        self.assertEqual(first, again)
        self.assertNotEqual(first, other)
        with tempfile.TemporaryDirectory() as root:
            narration = [("a", "Value", "Wert $\\Delta\\theta$"), ("b", "Quiet", "Ruhig")]
            clause_dir = Path(root) / "clauses"
            clause_dir.mkdir()
            identities = {}
            for index, (_, _, text) in enumerate(narration):
                (clause_dir / f"clause_{index:02d}.wav").write_bytes(b"RIFF")
                identities[f"clause_{index:02d}"] = SPEECH.clause_identity(LEXICON.spoken_text(text, "de"), backend=backend)
            (clause_dir / "identity.json").write_text(json.dumps(identities), encoding="utf-8")
            with mock.patch.dict(os.environ, {"TTS_API_KEY": "", "OPENROUTER_API_KEY": ""}), mock.patch.object(SPEECH, "probe_duration", return_value=1.5), \
                    mock.patch.object(SPEECH, "_load_dotenv_file"):
                reused = SPEECH.synthesize_clause_audio(narration, clause_dir)
                self.assertEqual([path.name for _, path, _ in reused], ["clause_00.wav", "clause_01.wav"])
                changed = SPEECH.synthesize_clause_audio([narration[0], ("b", "Quiet", "Anders")], clause_dir)
                self.assertEqual([path.name if path else None for _, path, _ in changed], ["clause_00.wav", None])

    def test_unspeakable_clause_stops_before_any_request(self):
        with tempfile.TemporaryDirectory() as root, mock.patch.dict(os.environ, {"TTS_API_KEY": "key"}), mock.patch.object(SPEECH, "_load_dotenv_file"), \
                mock.patch("urllib.request.urlopen") as request:
            with self.assertRaises(LEXICON.SpeechError):
                SPEECH.synthesize_clause_audio([("x", "e", "Wert $\\Omega\\mathrm{Foo}$")], Path(root) / "clauses")
            request.assert_not_called()

    def test_voice_text_is_the_spoken_german_text(self):
        narration = [("a", "Value", "Treiber $\\Delta\\theta$"), ("b", "Gap", "")]
        self.assertEqual(SPEECH.subtitle_clauses(narration), ["Treiber Delta Theta", ""])
        self.assertEqual(SPEECH.spoken_clauses(narration, language="en"), ["Value", "Gap"])
        self.assertEqual(SPEECH.subtitle_narration_text(narration, "a"), "Treiber Delta Theta")


if __name__ == "__main__":
    unittest.main()
