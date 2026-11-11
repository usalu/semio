"""🧪 Validate actual wall buildup frames against a fresh Cairo drawing."""
import ast,importlib,json,os,sys,unittest
from pathlib import Path
from unittest.mock import patch
import numpy as np
from jsonschema import Draft202012Validator
from manim import Camera,ValueTracker,tempconfig

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
MODULE=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
HERE=Path(__file__).parent
CASES=json.loads((HERE/"🔣️.json").read_text(encoding="utf8"))

class WallBuildup(unittest.TestCase):
    def test_authored_timeline_matches_fresh_cairo_in_both_languages(self):
        """🧱 Keep the actual animated material strip current during reveal and growth."""
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(CASES)
        source=Path(MODULE.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_buildup")
        builder=next(n for n in method.body if isinstance(n,ast.FunctionDef) and n.name=="strip")
        show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
        self.assertEqual(show.args[0].value,"buildup")
        for language in CASES["languages"]:
            with self.subTest(language=language),patch.dict(os.environ,{"SEMIO_TEACHING_LANGUAGE":language}),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
                scene=MODULE.Beat4_PartialModels()
                wall=MODULE._model.element("W-S")["material"]
                layers,total=wall["layers"],wall["total_thickness"]
                zoom,x0,y0,height,grow=8.,1.2,1.25,1.5,ValueTracker(0)
                tones=[(MODULE.mix(MODULE.BACKGROUND,MODULE.INK,.6),MODULE.INK,45,.07),(MODULE.mix(MODULE.BACKGROUND,MODULE.WARN,.5),MODULE.WARN,45,.12),(MODULE.mix(MODULE.BACKGROUND,MODULE.INK,.3),MODULE.INK,135,.07)]
                env={**vars(MODULE),"self":scene,"layers":layers,"total":total,"zoom":zoom,"x0":x0,"y0":y0,"height":height,"grow":grow,"tones":tones}
                exec(compile(ast.Module(body=[builder],type_ignores=[]),str(source),"exec"),env)
                strip=env["strip"];live=scene.live(strip,key=lambda:round(grow.get_value(),3))
                env.update({"live":live,"names":MODULE.VGroup()})
                self.assertEqual([arg.elts[0].value for arg in show.args[3:5]],[CASES["reveal"],CASES["grow"]])
                stages=[eval(compile(ast.Expression(body=arg),str(source),"eval"),env) for arg in show.args[3:5]]
                camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                differences=[];original=scene.renderer.update_frame
                def update(*args,**kwargs):
                    result=original(*args,**kwargs)
                    if CASES["sample_min"]<grow.get_value()<CASES["sample_max"]:
                        actual=np.asarray(scene.camera.get_image(),dtype=int)
                        camera.reset();camera.capture_mobjects([strip()])
                        expected=np.asarray(camera.get_image(),dtype=int)
                        differences.append(int(np.abs(actual-expected).max()))
                    return result
                scene.renderer.update_frame=update
                animation,duration=scene._timeline(*stages)
                scene.play(animation,run_time=duration)
                self.assertTrue(differences)
                self.assertLessEqual(max(differences),CASES["tolerance"],str(differences))

if __name__=="__main__":unittest.main()
