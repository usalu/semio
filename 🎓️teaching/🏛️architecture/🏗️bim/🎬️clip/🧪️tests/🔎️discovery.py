"""🔎 Execute neutral discovery declarations through the standard native collector."""
import importlib
import json
import types
import unittest
from pathlib import Path


def test_colocated_discovery_executes_local_classes_functions_and_authoritative_hooks():
    from jsonschema import Draft202012Validator

    base = Path(__file__).with_suffix("")
    cases = json.loads(base.with_suffix(".json").read_text(encoding="utf-8"))
    Draft202012Validator(json.loads(base.with_suffix(".schema.json").read_text(encoding="utf-8"))).validate(cases)
    main = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
    for case in cases["cases"]:
        source = types.ModuleType("case_" + case["id"])
        bodies = []
        def function(name, label, module):
            def body(*args):
                bodies.append(label)
            body.__name__ = name
            body.__qualname__ = name
            body.__module__ = module
            return body
        for name in case["functions"]:
            setattr(source, name, function(name, name.removeprefix("test_"), source.__name__))
        for row in case["classes"]:
            methods = {name: function(name, name.removeprefix("test_"), source.__name__) for name in row["methods"]}
            cls = type(row["name"], (unittest.TestCase,), {"__module__": source.__name__, **methods})
            setattr(source, row["name"], cls)
            if case.get("aliases"):
                source.AliasContract = cls
        if case.get("aliases"):
            source.test_alias = getattr(source, case["functions"][0])
        if case.get("imports"):
            source.test_imported = function("test_imported", "forbidden-import", "foreign")
            source.ImportedContract = type("ImportedContract", (unittest.TestCase,), {"__module__": "foreign", "test_imported": source.test_imported})
        if case.get("hook"):
            authoritative = function("test_authoritative", "authoritative", source.__name__)
            source.load_tests = lambda loader, tests, pattern: unittest.TestSuite([unittest.FunctionTestCase(authoritative)])
        suite = main._colocated_tests(unittest.TestLoader(), source, None)
        def ids(value):
            return [(item._testFunc.__module__ + "." + item._testFunc.__qualname__) if isinstance(item, unittest.FunctionTestCase) else item.id() for item in value if not isinstance(item, unittest.TestSuite)] + [name for item in value if isinstance(item, unittest.TestSuite) for name in ids(item)]
        actual = sorted(ids(suite))
        Draft202012Validator({"const": sorted(case["expected"])}).validate(actual)
        result = unittest.TestResult()
        suite.run(result)
        assert result.wasSuccessful(), (case["id"], result.errors, result.failures)
        assert result.testsRun == len(case["expected"])
        Draft202012Validator({"const": sorted(case["bodies"])}).validate(sorted(bodies))
