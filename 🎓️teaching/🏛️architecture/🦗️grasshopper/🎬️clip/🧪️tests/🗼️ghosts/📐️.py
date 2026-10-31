"""📐 Inspect the narrated ghost dispatch without importing a rendering runtime."""
import ast
from pathlib import Path
import unittest

CLIP=Path(__file__).resolve().parents[2]
SOURCE=ast.parse((CLIP/"🗼️tower/🐍️.py").read_text(encoding="utf8"))

class GhostDispatch(unittest.TestCase):
    def test_stage_uses_the_evaluated_rejected_geometry_helpers(self):
        scene=next(node for node in SOURCE.body if isinstance(node,ast.ClassDef) and node.name=="Beat3_Tower")
        present=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=="present_tower")
        calls={node.func.id for node in ast.walk(present) if isinstance(node,ast.Call) and isinstance(node.func,ast.Name)}
        self.assertTrue({"_rejected_capsules","_rejected_ghosts"}<=calls)

    def test_renderer_selects_both_family_and_type_from_the_actual_geometry(self):
        helper=next((node for node in SOURCE.body if isinstance(node,ast.FunctionDef) and node.name=="_rejected_ghosts"),None)
        self.assertIsNotNone(helper)
        constants={node.value for node in ast.walk(helper) if isinstance(node,ast.Constant) and isinstance(node.value,str)}
        self.assertTrue({"type","family"}<=constants)
        draws=[node for node in ast.walk(helper) if isinstance(node,ast.Call) and isinstance(node.func,ast.Attribute) and node.func.attr=="_capsule_lite"]
        self.assertEqual(len(draws),1)
        self.assertIn("layout",{key.arg for key in draws[0].keywords})

if __name__=="__main__":
    unittest.main()
