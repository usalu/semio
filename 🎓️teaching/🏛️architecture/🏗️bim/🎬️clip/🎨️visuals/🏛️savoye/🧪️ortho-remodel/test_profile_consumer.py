"""🧪 Execute the production profile detail against a geometry-recording drawing interface."""
import ast
import importlib
import json
from pathlib import Path
from types import SimpleNamespace
import unittest

import numpy as np

drafting_geometry=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry')

native = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Shape:
    """⬟ Small drawing-interface recorder; geometry remains production-calculated."""

    def __init__(self,*members):
        self.submobjects = list(members)
        self.parts = {}

    def add(self,*members):
        self.submobjects.extend(members)
        return self

    def part(self,tag,shape,anchor=None):
        self.parts[tag] = shape


class ProfileDetailConsumer(unittest.TestCase):
    """🏛️ A noncircular staged source profile must never request a radius or Circle."""

    def test_actual_outline_and_part_heights(self):
        """📐 Record production output and compare its widths and heights using NumPy."""
        root = Path(__file__).resolve().parent.parent
        tree = ast.parse((root / "🔩️detail" / "🐍️.py").read_text(encoding="utf-8"))
        nodes = [n for n in tree.body if isinstance(n,(ast.FunctionDef,ast.ClassDef)) and n.name in ("_part","Detail")]
        column = json.loads((root / "🧪️ortho-remodel" / "profiles.json").read_text(encoding="utf-8"))["staged_column"]
        model = {"pilotis":[column]}
        def polygon(points,*args):
            """📝 Preserve exactly the vertices emitted by production drawing code."""
            shape = Shape()
            shape.vertices = np.asarray(points,float)
            return shape
        def view(kind,point,scale,centre,**kwargs):
            """📽️ Orthographic interface with the same declared coordinate projection."""
            return lambda x,y,z:np.asarray(point)+scale*np.asarray([x-centre[0],z-centre[2],0])
        drafting = SimpleNamespace(View=view,Drawing=lambda view:Shape(),P=drafting_geometry.P,polygon=polygon,
                                  CONCRETE="concrete",STRUCT="structure",W_EDGE=1,S_LABEL=1,DATA="data")
        engine = SimpleNamespace(VGroup=Shape,RIGHT=np.asarray([1,0,0]),UP=np.asarray([0,1,0]))
        scope = {"np":np,"_d":drafting,"_engine":engine,"_c":SimpleNamespace(geometry=lambda ignored:model),"_model":SimpleNamespace(role=lambda name,ignored:"19",part_outline_at=native.part_outline_at,part_elevation=native.part_elevation)}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        scene = SimpleNamespace(text=lambda *args,**kwargs:Shape())
        drawing = scope["Detail"].piloti_detail(scene,model=model,dims=False)
        section = drawing.parts["section"]
        expected=column["detail_expectations"]
        self.assertAlmostEqual(drawing.section_z,expected["section_z"])
        np.testing.assert_allclose(np.ptp(section.vertices,axis=0),[*np.asarray(expected["section_extent"])*.62,0],rtol=0,atol=1e-12)
        elevation = drawing.parts["elevation"]
        self.assertEqual(len(elevation.submobjects),2)
        np.testing.assert_allclose([np.ptp(shape.vertices[:,0]) for shape in elevation.submobjects],np.asarray(expected["elevation_extents"])[:,0]*.62,rtol=0,atol=1e-12)
        np.testing.assert_allclose([np.ptp(shape.vertices[:,1]) for shape in elevation.submobjects],np.asarray(expected["elevation_extents"])[:,1]*.62,rtol=0,atol=1e-12)


    def test_regional_strip_uses_occupied_parts(self):
        """🟫 Execute the production strip with poisonous root thickness and a through void."""
        root = Path(__file__).resolve().parent.parent
        tree = ast.parse((root / "🔩️detail" / "🐍️.py").read_text(encoding="utf-8"))
        nodes = [n for n in tree.body if isinstance(n,(ast.FunctionDef,ast.ClassDef)) and n.name in ("_part","Detail")]
        plan_tree = ast.parse((root / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        helpers = [n for n in plan_tree.body if isinstance(n,ast.FunctionDef) and n.name in ("profile_cut","slab_cut")]
        helper_scope = {"_model":native}
        exec(compile(ast.Module(body=helpers,type_ignores=[]),str(root),"exec"),helper_scope)
        slab = json.loads((root / "🧪️ortho-remodel/slab-regions.json").read_text(encoding="utf-8"))["slab"]
        contour = [[.4,1.9],[.6,1.9],[.6,2.1],[.4,2.1]]
        column = {"tag":"19","x":.5,"y":2,"base":-2.4,"top":-.2,"parts":[{"outline":contour,"z0":-2.4,"z1":-.2}]}
        model = {"pilotis":[column],"slabs":[slab],"storeys":{"ground":-2.4}}
        def polygon(points,*args):
            """📝 Record production slab vertices without substituting physical calculations."""
            shape = Shape()
            shape.vertices = np.asarray(points,float)
            return shape
        def view(kind,point,scale,centre,**kwargs):
            """📽️ Record the declared orthographic strip projection."""
            return lambda x,y,z:np.asarray(point)+scale*np.asarray([x-centre[0],z-centre[2],0])
        drafting = SimpleNamespace(View=view,Drawing=lambda view:Shape(),polygon=polygon,CONCRETE="concrete",STRUCT="structure",W_EDGE=1,W_THIN=.5)
        scope = {"_d":drafting,"_engine":SimpleNamespace(VGroup=Shape),"_c":SimpleNamespace(geometry=lambda ignored:model),
                 "_model":SimpleNamespace(role=lambda name,ignored:"19" if name == "column_example" else "SL-MIX",part_intervals=native.part_intervals,part_section=native.part_section,part_outline_at=native.part_outline_at),"_plan":SimpleNamespace(**helper_scope)}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        drawing = scope["Detail"].slab_strip(SimpleNamespace(),model=model)
        shapes = drawing.parts["SL-MIX"].submobjects
        np.testing.assert_allclose(sorted(np.ptp(shape.vertices[:,0])/.62 for shape in shapes),[1,2,4],rtol=0,atol=1e-12)
        np.testing.assert_allclose(sorted(np.ptp(shape.vertices[:,1])/.62 for shape in shapes),[.2,.2,.6],rtol=0,atol=1e-12)
        area = sum(abs(np.dot(shape.vertices[:,0],np.roll(shape.vertices[:,1],-1))-np.dot(shape.vertices[:,1],np.roll(shape.vertices[:,0],-1)))/2 for shape in shapes)/(.62*.62)
        self.assertAlmostEqual(area,3)


if __name__ == "__main__":
    unittest.main()
