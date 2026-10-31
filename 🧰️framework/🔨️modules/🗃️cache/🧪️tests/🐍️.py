"""🧪 Verify portable resource traces and owner lifecycle without renderer imports."""
import gc
import importlib.util
import json
from pathlib import Path
import sys
import unittest
import weakref

BASE = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("owned_template_cache", BASE / "🐍️.py")
C = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = C
SPEC.loader.exec_module(C)
CASES = json.loads((BASE / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
TRACE = []


class CacheContracts(unittest.TestCase):
    """📋 Exercise externally visible retention, identity, ordering and error contracts."""

    def test_portable_traces(self):
        for case in CASES["cases"]:
            cache = C.TemplateCache(C.CachePolicy(*case["policy"]))
            for step in case["steps"]:
                with self.subTest(case=case["name"], step=step):
                    if step["op"] == "request":
                        value = {"key": step["key"]}
                        def factory():
                            if "reserve" not in step:
                                self.assertEqual(cache.stats()["entries"], 0)
                            if step.get("fail") == "factory":
                                raise RuntimeError("factory")
                            return value
                        def measure(item):
                            if step.get("fail") == "measure":
                                raise RuntimeError("measure")
                            return C.CacheCost(*step["cost"])
                        def clone(item):
                            if step.get("fail") == "clone":
                                raise RuntimeError("clone")
                            return dict(item)
                        options = {"valid": lambda item: not step.get("invalid", False), "cancelled": lambda: step.get("fail") == "cancel"}
                        if "reserve" in step:
                            options["reservation"] = C.CacheCost(*step["reserve"])
                        if step["outcome"] == "error":
                            with self.assertRaises((RuntimeError, C.CacheCancelled)):
                                cache.resolve(step["key"], factory, measure, clone, **options)
                        else:
                            result = cache.resolve(step["key"], factory, measure, clone, **options)
                            self.assertEqual(result is value, step["outcome"] == "original")
                    elif step["op"] == "release":
                        cache.release()
                    elif step["op"] == "invalidate":
                        cache.invalidate(step["key"])
                    else:
                        cache.configure(C.CachePolicy(*step["policy"]))
                    stats = cache.stats()
                    self.assertEqual(list(stats["keys"]), step["keys"])
                    self.assertEqual([stats["members"], stats["points"]], step["total"])
                    self.assertEqual(stats["reserved_members"], 0)
                    self.assertEqual(stats["reserved_points"], 0)
                    TRACE.append({"case": case["name"], **stats})

    def test_validation_is_atomic(self):
        self.assertIs(type(C.CacheCost(2.0, 0.0).members), int)
        self.assertIs(type(C.CachePolicy(2.0).max_entries), int)
        for value in [*CASES["invalid_numbers"], float("inf"), float("nan")]:
            with self.subTest(value=value), self.assertRaises((TypeError, ValueError)):
                C.CacheCost(value, 1)
            with self.assertRaises((TypeError, ValueError)):
                C.CachePolicy(value, 1, 1, 1)
        cache = C.TemplateCache()
        for key in (None, True, 2, "", "x" * 4097):
            with self.assertRaises((TypeError, ValueError)):
                cache.resolve(key, lambda: {}, lambda value: C.CacheCost(1, 1), dict)

    def test_owned_borrowed_and_nested_failure(self):
        for scenario in CASES["ownership"]:
            owner = C.CacheOwner()
            outer = C.TemplateCache()
            outer.resolve("outer", lambda: {}, lambda value: C.CacheCost(1, 1), dict)
            with owner.scope(cache=outer):
                self.assertIs(owner.current, outer)
                if scenario == "borrowed":
                    with owner.scope(cache=outer):
                        self.assertIs(owner.current, outer)
                else:
                    inner = None
                    try:
                        with owner.scope() as inner:
                            inner.resolve("a", lambda: {}, lambda value: C.CacheCost(1, 1), dict)
                            if scenario == "nested_failure":
                                raise RuntimeError("scope")
                    except RuntimeError:
                        self.assertEqual(scenario, "nested_failure")
                    self.assertEqual(inner.stats()["entries"], 0)
                    self.assertIs(owner.current, outer)
            self.assertIsNone(owner.current)
            self.assertEqual(outer.stats()["entries"], 1)

    def test_history_bounds_known_reservations_and_count_limits(self):
        cache = C.TemplateCache(C.CachePolicy(8, 100, 100, 2))
        for index in range(9):
            cache.resolve(str(index), dict, lambda value: C.CacheCost(1, 1), dict, reservation=C.CacheCost(1, 1))
        self.assertEqual(cache.stats()["keys"], tuple(str(index) for index in range(1, 9)))
        self.assertEqual(cache.stats()["history_keys"], ("7", "8"))
        cache.configure(C.CachePolicy(8, 2, 2, 2))
        cache.resolve("7", dict, lambda value: C.CacheCost(1, 1), dict)
        self.assertEqual(cache.stats()["keys"], ("8", "7"))
        cache.invalidate("7")
        self.assertNotIn("7", cache.stats()["history_keys"])
        cache.resolve("7", lambda: self.assertEqual(cache.stats()["entries"], 0) or {}, lambda value: C.CacheCost(1, 1), dict)

    def test_underestimate_and_hit_clone_failure(self):
        cache = C.TemplateCache(C.CachePolicy(8, 5, 5, 2))
        cache.resolve("a", dict, lambda value: C.CacheCost(4, 4), dict, reservation=C.CacheCost(1, 1))
        self.assertEqual(cache.stats()["underestimates"], 1)
        with self.assertRaises(RuntimeError):
            cache.resolve("a", dict, lambda value: C.CacheCost(1, 1), lambda value: (_ for _ in ()).throw(RuntimeError("clone")))
        self.assertEqual(cache.stats()["keys"], ("a",))
        with self.assertRaises(TypeError):
            cache.configure({"max_entries": 0})
        self.assertEqual(cache.stats()["keys"], ("a",))

    def test_eviction_releases_before_factory_and_clone_isolation(self):
        class Value:
            pass
        cache = C.TemplateCache()
        references = []
        def first():
            value = Value()
            references.append(weakref.ref(value))
            return value
        returned = cache.resolve("a", first, lambda value: C.CacheCost(1, 1), lambda value: Value())
        def second():
            gc.collect()
            self.assertIsNone(references[0]())
            return Value()
        cache.resolve("b", second, lambda value: C.CacheCost(1, 1), lambda value: Value())
        self.assertIsInstance(returned, Value)
        cache.release()
        self.assertIsInstance(returned, Value)

    def test_callback_failures_cancel_and_reentry(self):
        cache = C.TemplateCache()
        for stage in ("measure", "clone", "late_cancel", "reentry"):
            calls = [0]
            def cancelled():
                calls[0] += 1
                return stage == "late_cancel" and calls[0] == 5
            def measure(value):
                if stage == "measure":
                    return (1, 1)
                if stage == "reentry":
                    cache.release()
                return C.CacheCost(1, 1)
            def clone(value):
                if stage == "clone":
                    raise RuntimeError("clone")
                return {}
            with self.assertRaises((TypeError, RuntimeError, C.CacheCancelled)):
                cache.resolve(stage, lambda: {}, measure, clone, cancelled=cancelled)
            self.assertEqual(cache.stats()["entries"], 0)
            self.assertEqual(cache.stats()["reserved_members"], 0)


if __name__ == "__main__":
    record = None
    if "--record" in sys.argv:
        index = sys.argv.index("--record")
        record = Path(sys.argv[index + 1])
        del sys.argv[index:index + 2]
    result = unittest.main(exit=False).result
    if record is not None and result.wasSuccessful():
        record.write_text(json.dumps(TRACE, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    sys.exit(0 if result.wasSuccessful() else 1)
