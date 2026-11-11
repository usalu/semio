"""🧪 Validate the real clearance ring against fresh Cairo frames."""
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

class ClearanceRing(unittest.TestCase):
    def test_authored_timeline_matches_fresh_cairo(self):
        """⭕ Keep the growing clearance ring current throughout its authored timeline."""
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(CASES)
        source=Path(MODULE.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_check")
        assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="ring" for t in n.targets))
        show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
        self.assertEqual(show.args[0].value,"check")
        self.assertEqual([arg.elts[0].value for arg in show.args[3:5]],[CASES["reveal"],CASES["grow"]])
        for language in CASES["languages"]:
            with patch.dict(os.environ,{"SEMIO_TEACHING_LANGUAGE":language}),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
                scene=MODULE.Beat4_PartialModels();ring_value=ValueTracker(0)
                env={**vars(MODULE),"self":scene,"ring_value":ring_value,"ring_radius":CASES["radius"],"zoom":CASES["zoom"],"pipe_at":np.array(CASES["point"]),"dimension":MODULE.VGroup()}
                exec(compile(ast.Module(body=[assignment],type_ignores=[]),str(source),"exec"),env)
                ring=env["ring"]
                stages=[eval(compile(ast.Expression(body=arg),str(source),"eval"),env) for arg in show.args[3:5]]
                camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                differences=[];original=scene.renderer.update_frame
                def update(*args,**kwargs):
                    result=original(*args,**kwargs)
                    if .1*CASES["radius"]<ring_value.get_value()<.9*CASES["radius"]:
                        actual=np.asarray(scene.camera.get_image(),dtype=int)
                        camera.reset();camera.capture_mobjects([ring._builder()])
                        expected=np.asarray(camera.get_image(),dtype=int)
                        differences.append(int(np.abs(actual-expected).max()))
                    return result
                scene.renderer.update_frame=update
                animation,duration=scene._timeline(*stages);scene.play(animation,run_time=duration)
                self.assertTrue(differences)
                self.assertLessEqual(max(differences),CASES["tolerance"],str(differences))
    
if __name__=="__main__":unittest.main()
