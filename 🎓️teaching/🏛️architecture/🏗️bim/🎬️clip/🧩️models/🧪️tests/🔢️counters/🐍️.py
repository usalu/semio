"""🧪 Validate source-owned counter and chart animation against fresh Cairo."""
import ast,importlib,json,os,sys,unittest
from pathlib import Path
from unittest.mock import patch
import numpy as np
from jsonschema import Draft202012Validator
from manim import Camera,ValueTracker,Wait,tempconfig

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
MODULE=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
HERE=Path(__file__).parent
CASES=json.loads((HERE/"🔣️.json").read_text(encoding="utf8"))

class LiveCounters(unittest.TestCase):
    def settings(self,language):
        return patch.dict(os.environ,{"SEMIO_TEACHING_LANGUAGE":language})
    def camera(self,scene):
        return Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    def compare(self,scene,stages,tracker,fresh,limit):
        camera=self.camera(scene);differences=[];original=scene.renderer.update_frame
        def update(*args,**kwargs):
            result=original(*args,**kwargs)
            if .2*limit<tracker.get_value()<.9*limit:
                actual=np.asarray(scene.camera.get_image(),dtype=int)
                camera.reset();camera.capture_mobjects(fresh())
                expected=np.asarray(camera.get_image(),dtype=int)
                differences.append(int(np.abs(actual-expected).max()))
            return result
        scene.renderer.update_frame=update
        animation,duration=scene._timeline(*stages);scene.play(animation,run_time=duration)
        self.assertTrue(differences)
        self.assertLessEqual(max(differences),CASES["tolerance"],str(differences))

    def test_actual_class_row_in_both_languages(self):
        """🔢 The source row counter changes during the authored class step."""
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(CASES)
        for language in CASES["languages"]:
            with self.subTest(language=language),self.settings(language),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
                scene=MODULE.Beat4_PartialModels()
                scene.info={"architecture":{"counts_by_class":{"IfcWall":CASES["count"]}}}
                scene._lit=lambda *args:Wait(.01);scene._tags=lambda *args:[]
                made=[]
                def tracker(value):
                    result=ValueTracker(value);made.append(result);return result
                with patch.object(MODULE,"ValueTracker",side_effect=tracker):
                    rows,stages=scene._rows("architecture",[("IfcWall",("Wall","Wand"))],1.3,None,seconds=.55)
                self.assertEqual(stages[0][0],.55)
                self.compare(scene,stages,made[0],lambda:[scene._class_row(made[0],"IfcWall",("Wall","Wand"),1.3,MODULE.ARCH)],CASES["count"])

    def test_unlit_class_waits(self):
        """💡 A class with no drawn parts waits instead of an empty lagged start."""
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
            scene=MODULE.Beat4_PartialModels()
            scene._keys=lambda *args:[]
            animation=scene._lit(object(),["IfcWall"])
            self.assertIsInstance(animation,Wait)
            self.assertGreater(animation.run_time,0)

    def test_actual_slab_formula_in_both_languages(self):
        """📐 The gross-area label changes while the source formula is visible."""
        source=Path(MODULE.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_slab")
        assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="formula" for t in n.targets))
        show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
        for language in CASES["languages"]:
            with self.subTest(language=language),self.settings(language),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
                scene=MODULE.Beat4_PartialModels();gross,cut,net=ValueTracker(0),ValueTracker(0),ValueTracker(0)
                slab={"gross_area":20.,"void_area":3.,"net_area":17.}
                env={**vars(MODULE),"self":scene,"gross":gross,"cut":cut,"net":net,"slab":slab,"edge":MODULE.VGroup()}
                exec(compile(ast.Module(body=[assignment],type_ignores=[]),str(source),"exec"),env)
                stage=eval(compile(ast.Expression(body=show.args[2]),str(source),"eval"),env)
                self.assertEqual(stage[0],.9)
                fresh=lambda:[eval(compile(ast.Expression(body=assignment.value),str(source),"eval"),env)]
                self.compare(scene,[stage],gross,fresh,slab["gross_area"])

    def test_actual_trace_chart_and_counters_in_both_languages(self):
        """📊 Multiple counters and the loss chart grow with the same tracker values."""
        source=Path(MODULE.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_trace")
        show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
        for language in CASES["languages"]:
            with self.subTest(language=language),self.settings(language),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
                scene=MODULE.Beat4_PartialModels();records,kinds,fill=ValueTracker(0),ValueTracker(0),ValueTracker(0)
                groups=[{"id":key,"count":i+1} for i,key in enumerate(MODULE.KIND_NAMES)]
                scene.analysis=analysis={"losses":list(range(CASES["count"])),"loss_groups":groups}
                footer,squares=scene._trace_footer(records,kinds),scene._kind_chart(fill)
                env={**vars(MODULE),"self":scene,"footer":footer,"squares":squares,"records":records,"kinds":kinds,"fill":fill,"analysis":analysis}
                stage=eval(compile(ast.Expression(body=show.args[-1]),str(source),"eval"),env)
                self.assertEqual(stage[0],1.2)
                self.compare(scene,[stage],fill,lambda:[scene._trace_footer(records,kinds),scene._kind_chart(fill)],1.)

if __name__=="__main__":unittest.main()
