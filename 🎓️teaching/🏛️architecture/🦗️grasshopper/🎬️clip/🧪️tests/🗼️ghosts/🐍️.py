"""🗼 Compare mistake ghosts with the actual deliberately unmasked engine solution."""
import ast
import importlib
import json
from pathlib import Path
import sys
from types import SimpleNamespace
import unittest
import numpy as np

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
CLIP=ROOT/"🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip"
C=json.loads((CLIP/"🧪️cases/🗼️ghosts/🔣️.json").read_text(encoding="utf8"))
SOURCE=ast.parse((CLIP/"🗼️tower/🐍️.py").read_text(encoding="utf8"))
GH=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
M=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
HELPERS={node.name:node for node in SOURCE.body if isinstance(node,ast.FunctionDef) and node.name in ("_rejected_capsules","_rejected_ghosts")}

class TowerGhosts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        definition=GH.canvas(C["canvas"])
        selected=GH.evaluate(definition,{})
        cls.masks=selected.tree("cullp")
        cls.wrong=GH.evaluate(definition,{"cullp":{"tree":{GH.format_path(path):[] for path in cls.masks.paths()}}})

    def helpers(self,environment=None):
        self.assertEqual(set(HELPERS),{"_rejected_capsules","_rejected_ghosts"})
        namespace=dict(environment or {})
        exec(compile(ast.Module(body=list(HELPERS.values()),type_ignores=[]),"tower-ghost-functions","exec"),namespace)
        return namespace

    def test_rejected_rows_use_actual_unmasked_geometry_type_family_and_body_plane(self):
        helpers=self.helpers()
        rows=helpers["_rejected_capsules"](self.wrong,self.masks)
        actual=[{"storey_index":path[-1],"index":index,"type":geometry.tag["type"],"family":geometry.tag["family"]} for path,index,plane,geometry in rows]
        self.assertEqual(actual,C["rows"])
        for path,index,plane,geometry in rows:
            canonical=M.capsule_window(M.capsule_type(geometry.tag["type"]),geometry.tag["family"])
            basis=np.asarray([plane.x.t,plane.y.t,plane.z.t]).T
            expected=np.asarray(plane.origin.t)+basis@np.asarray(canonical.center)
            np.testing.assert_allclose(expected,geometry.window().plane.origin.t,atol=C["tolerance"],rtol=0)
            door=next(part for part in M.capsule_assembly(geometry.tag["type"],"exterior",geometry.tag["family"]).parts if part.key=="entry-door")
            expected=np.asarray(plane.origin.t)+basis@np.asarray(door.solid.vertices).mean(axis=0)
            actual=next(part for part in geometry.parts() if part["key"]=="entry-door")
            np.testing.assert_allclose(expected,np.asarray(actual["vertices"]).mean(axis=0),atol=C["tolerance"],rtol=0)

    def test_render_dispatch_uses_each_evaluated_family_type_and_shared_body_plane(self):
        calls=[]
        class Ghost:
            def shift(self,delta):
                self.delta=np.asarray(delta)
                return self
            def set_opacity(self,opacity):
                self.opacity=opacity
                return self
        def draw(cap,plane,view,base,*,layout):
            ghost=Ghost()
            calls.append((cap.key,layout,plane,view,base,ghost))
            return ghost
        environment={"_engine":SimpleNamespace(VGroup=lambda *items:list(items)),"_nakd":SimpleNamespace(_cap=lambda value:value,_capsule_lite=draw),"WARN":"sentinel"}
        helpers=self.helpers(environment)
        rows=helpers["_rejected_capsules"](self.wrong,self.masks)
        reference=SimpleNamespace(view=object(),delta=np.asarray([.3,.2,0]))
        ghosts=helpers["_rejected_ghosts"](rows,{t.key:t for t in M.capsule_types()},reference,lambda plane:plane)
        self.assertEqual([(row[0],row[1]) for row in calls],[(row["type"],row["family"]) for row in C["rows"]])
        self.assertEqual(len(ghosts),len(rows))
        for call,row in zip(calls,rows):
            self.assertIs(call[2],row[2])
            self.assertIs(call[3],reference.view)
            np.testing.assert_array_equal(call[-1].delta,reference.delta)
            self.assertEqual(call[-1].opacity,.35)

    def test_narrated_mistake_stage_calls_the_verified_helpers(self):
        scene=next(node for node in SOURCE.body if isinstance(node,ast.ClassDef) and node.name=="Beat3_Tower")
        method=next(node for node in scene.body if isinstance(node,ast.FunctionDef) and node.name=="present_tower")
        calls={node.func.id for node in ast.walk(method) if isinstance(node,ast.Call) and isinstance(node.func,ast.Name)}
        self.assertTrue({"_rejected_capsules","_rejected_ghosts"}<=calls)

if __name__=="__main__":
    unittest.main()
