"""🧹 Prove class fixture release after every native unittest observer and on failed setup."""
import gc
import importlib
import json
import unittest
import weakref
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

import numpy as np
from jsonschema import Draft202012Validator

CASES=json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


class FixtureLifetimeContract(unittest.TestCase):
    """🧪 Compare weak ownership with actual independent NumPy buffer lifetimes."""

    def test_completed_failed_and_partial_class_fixtures_release_owned_references(self):
        """🧵 Preserve per-test observer access, mutation errors and independent array release."""
        module=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
        schema=json.loads(Path(__file__).with_name("🧬️.schema.json").read_text(encoding="utf-8"))
        Draft202012Validator(schema).validate(CASES)
        class Tracked(dict):
            pass
        for case in CASES["scenarios"]:
            with self.subTest(scenario=case["id"]):
                references=[];observed=[];released=[]
                def remember(value):
                    references.append(weakref.ref(value))
                    weakref.finalize(value,released.append,len(references))
                    return value
                def geometry(model):
                    if case["setup_error"]:raise RuntimeError("explicit neutral setup failure")
                    return remember(np.arange(24,dtype=float).reshape(2,3,4).copy())
                api=SimpleNamespace(load_model=lambda:remember(Tracked(value=0)),building_geometry=geometry)
                class Fixture(module.ModelCase):
                    @classmethod
                    def setUpClass(cls):
                        super().setUpClass()
                        references.append(weakref.ref(cls.snapshot))
                        weakref.finalize(cls.snapshot,released.append,"snapshot")
                        for name in ("analysis","results","_sample_cache","_cell_cache"):
                            setattr(cls,name,remember(Tracked(buffer=np.ones(4))))
                    def test_observe(self):
                        self.assertTrue(all(ref() is not None for ref in references))
                        if case["mutate"]:self.model["value"]=1
                class Observer(unittest.TestResult):
                    def stopTest(self,test):
                        observed.append(all(ref() is not None for ref in references))
                        super().stopTest(test)
                result=Observer()
                with patch.object(module,"model_api",return_value=api),patch.object(module,"registry",side_effect=lambda model:remember(Tracked(model=model))):
                    unittest.TestSuite([Fixture("test_observe")]).run(result)
                gc.collect()
                self.assertEqual(len(result.errors),case["expected_errors"],result.errors)
                self.assertEqual(result.failures,[])
                self.assertEqual(observed,[] if case["setup_error"] else [True])
                self.assertTrue(all(ref() is None for ref in references),"A completed test class still owns a model or native buffer")
                self.assertEqual(len(released),len(references))
                self.assertTrue(all(name not in vars(Fixture) for name in CASES["owned_attributes"]))
                if case["mutate"]:self.assertIn("a model function mutated its input",result.errors[0][1])
