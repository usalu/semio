"""🌳 Verify tower presentations retain vector capsule identities."""
import gc
import importlib
import weakref
import json
from pathlib import Path
import unittest
from types import SimpleNamespace
from unittest.mock import patch
import numpy as np
from manim import Camera, FadeIn, Group, Scene, Square, VGroup, tempconfig

trees = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🌳️trees.🐍️")
casting = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")

sequence_case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

class TreeTowerSnapshotTest(unittest.TestCase):
    def test_recap_counts_use_variable_branch_lengths_and_fit(self):
        import jsonschema
        case=sequence_case["recap"];lengths=case["branch_lengths"]
        jsonschema.validate(case,json.loads(Path(__file__).with_name("recap.schema.json").read_text(encoding="utf-8")))
        source=trees._model.Tree.from_dict({"{0;"+str(i)+"}":[str(i)+"-"+str(j) for j in range(n)] for i,n in enumerate(lengths)})
        self.assertGreater(len(set(lengths)),1)
        expected_items=int(np.asarray(lengths).sum())
        for language in ("en","de"):
            with self.subTest(language=language),tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"background_color":"#000000"}):
                scene=trees.Beat5_DataTrees();scene.language=lambda:language
                for result,path,expected_branches in ((source.flatten(),case["flatten_path"],1),(source.graft(),case["graft_path"],expected_items)):
                    self.assertEqual(result.path_count,expected_branches)
                    self.assertEqual(result.count,expected_items)
                    found=next((trees._model.format_path(p)+"["+str(i)+"]" for p in result.paths() for i,item in enumerate(result.branch(p)) if item==case["probe"]),None)
                    self.assertEqual(found,path)
                    text=scene._recap_operation_text(source,result,path)
                    self.assertIn(str(source.path_count),text)
                    self.assertIn(str(expected_branches),text)
                    self.assertIn(str(expected_items),text)
                    self.assertIn(path,text)
                    self.assertNotIn("× 2",text)
                    label=scene._recap_result_label(text,0)
                    self.assertLessEqual(label.width,case["max_width"]+1e-6)
                    camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                    camera.capture_mobjects([label])
                    self.assertGreater(np.asarray(camera.get_image())[:,:,:3].max(),0)

    def test_full_tower_recap_counts_and_cairo_fit(self):
        case=sequence_case["recap"];lengths=case["full_branch_lengths"]
        source=trees._model.Tree.from_dict({"{"+str(i)+"}":list(range(n)) for i,n in enumerate(lengths)})
        self.assertEqual(source.path_count,75)
        self.assertEqual(int(np.asarray(lengths).sum()),140)
        for language in ("en","de"):
            with self.subTest(language=language),tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"background_color":"#000000"}):
                scene=trees.Beat5_DataTrees();scene.language=lambda:language
                camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                for result,path in ((source.flatten(),case["flatten_path"]),(source.graft(),case["full_graft_path"])):
                    text=scene._recap_operation_text(source,result,path)
                    self.assertIn("75→"+str(result.path_count),text)
                    self.assertIn("140",text)
                    label=scene._recap_result_label(text,0)
                    self.assertLessEqual(label.width,case["max_width"]+1e-6)
                    self.assertAlmostEqual(label.get_left()[0],-2.15,places=5)
                    camera.reset();camera.capture_mobjects([label])
                    pixels=np.asarray(camera.get_image())[:,:,:3]
                    ys,xs=np.where(pixels.max(axis=2)>0)
                    self.assertGreater(len(xs),0)
                    left=(.5-2.15/camera.frame_width)*camera.pixel_width
                    right=(.5+(-2.15+case["max_width"])/camera.frame_width)*camera.pixel_width
                    self.assertGreaterEqual(xs.min(),int(left)-2)
                    self.assertLessEqual(xs.max(),int(right)+2)

    def test_capsule_identity_and_vector_source(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        capsule = Square(fill_color="#3399cc", fill_opacity=1)
        drawing = VGroup(capsule)
        drawing.capsules = {(0, 0, 0, 0, 0): capsule}
        scene = SimpleNamespace(camera=camera, tower_axon=lambda **kwargs: drawing)
        image = trees.Beat5_DataTrees._tower(scene, ring=False)
        clone = image.copy()
        self.assertIs(image.capsules[(0, 0, 0, 0, 0)], capsule)
        self.assertIs(clone.capsules[(0, 0, 0, 0, 0)], capsule)
        self.assertIs(image._drawing_ref.value, drawing)
        self.assertIs(clone._drawing_ref.value, drawing)
        camera.capture_mobjects([image])

    def test_large_hatch_uses_projected_capsules(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        keys = [(0, 0, i, 0, 0) for i in range(11)]
        capsules = {key: Square(side_length=.2, fill_opacity=1, stroke_width=0).shift([i * .2 - 1, 0, 0]) for i, key in enumerate(keys)}
        before = {key: shape.points.copy() for key, shape in capsules.items()}
        added = []
        scene = SimpleNamespace(camera=camera, _born=[], add=lambda *items: added.extend(items))
        points = [np.array([i * .2 - 1, 2, 0]) for i in range(11)]
        with patch.object(trees._morph, "token", side_effect=lambda *args, **kwargs: Square(side_length=.1, fill_opacity=1)):
            animation = trees.Beat5_DataTrees._hatch(scene, SimpleNamespace(capsules=capsules), keys, points, ["#ff0000"] * 11, .35)
        self.assertEqual(len(added), 11)
        self.assertTrue(all(isinstance(item, trees._sprites.Sprite) for item in added))
        self.assertEqual(len(scene._born), 11)
        self.assertTrue(all(np.array_equal(before[key], capsules[key].points) for key in keys))
        animation.begin()
        animation.interpolate(.5)
        camera.capture_mobjects(added)

    def test_large_hatch_scene_cleanup(self):
        with tempconfig({"dry_run": True, "pixel_width": 160, "pixel_height": 90, "frame_rate": 4}):
            scene = Scene()
            scene._born = []
            keys = [(0, 0, i, 0, 0) for i in range(11)]
            capsules = {key: Square(side_length=.2, fill_opacity=1, stroke_width=0).shift([i * .2 - 1, 0, 0]) for i, key in enumerate(keys)}
            points = [np.array([i * .2 - 1, 2, 0]) for i in range(11)]
            with patch.object(trees._morph, "token", side_effect=lambda *args, **kwargs: Square(side_length=.1, fill_opacity=1)):
                animation = trees.Beat5_DataTrees._hatch(scene, SimpleNamespace(capsules=capsules), keys, points, ["#ff0000"] * 11, .35)
            scene.play(animation)
            family = scene.get_mobject_family_members()
            self.assertTrue(all(target in family for target in scene._born))
            self.assertFalse(any(isinstance(item, trees._sprites.Sprite) for item in family))
            scene.remove(*scene._born)
            family = scene.get_mobject_family_members()
            self.assertTrue(all(target not in family for target in scene._born))

    def test_140_flights_keep_anchor_count_and_duration(self):
        case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        keys = [(0, 0, i, 0, 0) for i in range(case["capsule_count"])]
        capsules = {key: Square(side_length=.08, fill_opacity=1, stroke_width=0).shift([(i % 14) * .3 - 2, (i // 14) * .2 - 1, 0]) for i, key in enumerate(keys)}
        points = [np.array([(i % 14) * .3 - 2, (i // 14) * .2 + .5, 0]) for i in range(len(keys))]
        added = []
        scene = SimpleNamespace(camera=camera, _born=[], add=lambda *items: added.extend(items))
        with patch.object(trees._morph, "token", side_effect=lambda *args, **kwargs: Square(side_length=.08, fill_opacity=1)):
            animation = trees.Beat5_DataTrees._hatch(scene, SimpleNamespace(capsules=capsules), keys, points, ["#ff0000"] * len(keys), .35)
        self.assertEqual(len(added), case["capsule_count"])
        self.assertEqual(len(scene._born), case["capsule_count"])
        self.assertLessEqual(animation.run_time, case["duration_limit"])
        animation.begin()
        animation.interpolate(1)
        self.assertTrue(all(np.allclose(target.get_center(), point) for target, point in zip(scene._born, points)))

    def test_large_gather_and_recap_cleanup(self):
        with tempconfig({"dry_run": True, "pixel_width": 160, "pixel_height": 90, "frame_rate": 4}):
            scene = Scene()
            keys = [(0, 0, i, 0, 0) for i in range(11)]
            scene._order = {key: i for i, key in enumerate(keys)}
            capsules = {key: Square(side_length=.2, fill_opacity=1, stroke_width=0).shift([i * .2 - 1, 0, 0]) for i, key in enumerate(keys)}
            target = np.array([2., 1., 0.])
            animation = trees.Beat5_DataTrees._gather(scene, SimpleNamespace(capsules=capsules), keys, target, "#ff0000")
            scene.play(animation)
            self.assertFalse(any(isinstance(item, trees._sprites.Sprite) for item in scene.get_mobject_family_members()))
            source = VGroup(*capsules.values())
            before = [item.points.copy() for item in capsules.values()]
            label = Square(side_length=.4).shift(target)
            recap = trees._tower_flight(source, label, scene.camera)
            scene.play(recap)
            self.assertFalse(any(isinstance(item, trees._sprites.Sprite) for item in scene.get_mobject_family_members()))
            self.assertTrue(all(np.array_equal(points, item.points) for points, item in zip(before, capsules.values())))

    def test_compare_can_fade_two_tower_snapshots(self):
        with tempconfig({"dry_run": True, "pixel_width": 160, "pixel_height": 90, "frame_rate": 4}):
            scene = Scene()
            vectors = [VGroup(Square(fill_opacity=1).shift([x, 0, 0])) for x in (-2, 2)]
            images = [trees._snapshot.capture(vector, scene.camera) for vector in vectors]
            compare = Group(*images, Square(side_length=.2).shift([0, 2, 0]))
            scene.play(FadeIn(compare), run_time=.5)
            self.assertIn(compare, scene.mobjects)
            scene.camera.reset()
            scene.camera.capture_mobjects([compare])

    def test_swap_releases_only_previous_semantics(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        old_capsule = Square(fill_opacity=1)
        old_vector = VGroup(old_capsule)
        old_ref, capsule_ref = weakref.ref(old_vector), weakref.ref(old_capsule)
        old = trees._snapshot.capture(old_vector, camera)
        old.capsules = trees._Capsules({"old": old_capsule})
        old._drawing_ref = trees._DrawingRef(old_vector)
        pixels = old.pixel_array.copy()
        new_capsule = Square(fill_opacity=1).shift([1, 0, 0])
        new_vector = VGroup(new_capsule)
        new = trees._snapshot.capture(new_vector, camera)
        new.capsules = trees._Capsules({"new": new_capsule})
        new._drawing_ref = trees._DrawingRef(new_vector)
        scene = SimpleNamespace(_slots={"tower": old})
        del old_vector, old_capsule
        animation = trees.Beat5_DataTrees._swap(scene, "tower", new)
        gc.collect()
        self.assertIsNone(old_ref())
        self.assertIsNone(capsule_ref())
        self.assertTrue(np.array_equal(old.pixel_array, pixels))
        self.assertIs(scene._slots["tower"].capsules["new"], new_capsule)
        animation.begin()
        animation.interpolate(.5)
        camera.capture_mobjects([old, new])

    def test_casting_reuses_image_and_keeps_vector(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        drawing = VGroup(Square(fill_opacity=1))
        state = {"builds": 0}
        def build(**kwargs):
            state["builds"] += 1
            return drawing
        scene = SimpleNamespace(camera=camera, _towers={}, _tower_fit={}, tower_axon=build)
        first = casting.Beat4_Casting._tower(scene)
        second = casting.Beat4_Casting._tower(scene)
        self.assertEqual(state["builds"], 1)
        self.assertIs(scene._towers[None], drawing)
        self.assertIsNot(first, second)
        camera.capture_mobjects([first])

class PathTiming(unittest.TestCase):
 def test_future_stages_are_not_visible_before_their_turn(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":4,"background_color":"#000000"}):
   scene=Scene(); frames=[]
   shapes=[Square(side_length=1,fill_color=c,fill_opacity=1,stroke_width=0).shift([x,0,0]) for x,c in zip(sequence_case["positions"],sequence_case["colors"])]
   holder=SimpleNamespace(_slots={"diagram":shapes[0]})
   scene.add(shapes[0])
   stages=[lambda item=item:trees.Beat5_DataTrees._swap(holder,"diagram",item) for item in shapes[1:]]
   original=scene.renderer.update_frame
   def update(*args,**kwargs):
    result=original(*args,**kwargs)
    frames.append(np.asarray(scene.camera.get_image()).copy())
    return result
   scene.renderer.update_frame=update
   trees._sequence(scene,stages,3)
   self.assertTrue(frames)
   frame=frames[0]; h,w=frame.shape[:2]
   samples=[frame[h//2,round(w/2+x*w/scene.camera.frame_width),:3] for x in sequence_case["positions"]]
   self.assertGreater(int(samples[0].max()),100)
   self.assertTrue(all(int(sample.max())<10 for sample in samples[1:]),repr(samples))
   self.assertIs(holder._slots["diagram"],shapes[-1])
   self.assertAlmostEqual(scene.time,3)
   for stage,index in enumerate((0,4,8)):
    frame=frames[index]
    remaining=sequence_case["positions"][stage+2:]
    for x in remaining:
     pixel=frame[h//2,round(w/2+x*w/scene.camera.frame_width),:3]
     self.assertLess(int(pixel.max()),10)
 def test_future_tower_semantics_survive_until_their_stage(self):
  camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
  displays=[]
  for i in range(3):
   vector=VGroup(Square(fill_opacity=1))
   image=trees._snapshot.capture(vector,camera)
   image.capsules=trees._Capsules({(0,0,i,0,0):vector[0]})
   image._drawing_ref=trees._DrawingRef(vector)
   displays.append(image)
  holder=SimpleNamespace(_slots={"tower":displays[0]})
  seen=[]
  def build(index):
   def run():
    seen.append(tuple(display._drawing_ref.value is not None for display in displays))
    return trees.Beat5_DataTrees._swap(holder,"tower",displays[index])
   return run
  class Fake:
   def play(self,*args,**kwargs): pass
  trees._sequence(Fake(),[build(1),build(2)],2)
  self.assertEqual(seen,[(True,True,True),(False,True,True)])
  self.assertIs(holder._slots["tower"],displays[2])
  self.assertIsNotNone(displays[2]._drawing_ref.value)
 def test_caption_and_duration_span_all_stages(self):
  events=[]
  class Fake:
   NARRATION=[("path","Address","Adresse")]
   _caption=object()
   def language(self): return "en"
   def play(self,*args,run_time): events.append(("play",run_time,args))
  fake=Fake()
  def builder(index):
   def build():
    events.append(("build",index))
    return index
   return build
  with patch.object(trees._visuals._art,"swap_caption",side_effect=lambda scene,caption,text: events.append(("caption",text)) or caption), patch.object(trees._visuals._art,"hold_for",side_effect=lambda scene,rows,key: events.append(("hold",key))):
   trees.Beat5_DataTrees._staged_step(fake,"path",[builder(i) for i in range(4)],seconds=7,first=("heading",))
  self.assertEqual(events[0],("caption","Address"))
  self.assertEqual(events[-1],("hold","path"))
  plays=[event for event in events if event[0]=="play"]
  self.assertEqual(len(plays),4)
  self.assertAlmostEqual(sum(event[1] for event in plays),7)
  self.assertIn("heading",plays[0][2])
  self.assertEqual([event[1] for event in events if event[0]=="build"],[0,1,2,3])
 def test_failure_does_not_build_later_stages(self):
  calls=[]
  class Fake:
   def play(self,*args,**kwargs): pass
  def first():
   calls.append(1)
   raise RuntimeError("cancel")
  def later():
   calls.append(2)
  with self.assertRaisesRegex(RuntimeError,"cancel"):
   trees._sequence(Fake(),[first,later],2)
  self.assertEqual(calls,[1])
if __name__ == "__main__":
    unittest.main()
