"""📦 Check preview keys and analytical midpoint geometry."""
import os
import importlib
import json
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch
import numpy as np
from shapely.geometry import MultiPoint, box
from manim import Camera, Scene, Square, ValueTracker, VGroup, UpdateFromAlphaFunc, tempconfig

anatomy = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
drawing = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

class PreviewTest(unittest.TestCase):
    def test_live_slider_and_readout_reveal_before_ramp(self):
        for state in case["live_reveals"]:
            with self.subTest(kind=state["kind"]),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
                scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en";scene.W=ValueTracker(state["start"])
                build=(lambda:scene._slider("w",(-1,0,0),scene.W)) if state["kind"]=="slider" else (lambda:scene.live_label(lambda:f"X = {scene.number(scene.W.get_value(),1)} m",(0,0,0),size=anatomy.SMALL,color=anatomy.SOFT))
                view=build();view.update(0)
                oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                differences=[];original=scene.renderer.update_frame
                def update(*args,**kwargs):
                    result=original(*args,**kwargs)
                    if scene.W.get_value()>state["start"]+1e-6:
                        actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                        fresh=build();fresh.update(0);oracle.reset();oracle.capture_mobjects([fresh])
                        differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                    return result
                scene.renderer.update_frame=update
                sequence=scene._reveal_then(view,scene._ramp(scene.W,state["start"],state["end"],state["duration"]-.4),state["duration"])
                self.assertAlmostEqual(sequence.run_time,state["duration"])
                scene.play(sequence,run_time=state["duration"])
                self.assertTrue(differences);self.assertLessEqual(max(differences),2,repr(differences))

    def test_value_sequence_reveals_before_type_changes(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en";scene.vl_index=ValueTracker(0);scene.view_state={"type":"A","mode":"type"}
            names=["Type "+t.key for t in anatomy._nk.capsule_types()]
            build_list=lambda:scene.gh_value_list(names,int(scene.vl_index.get_value()),at=(-1.9,0,0),name="Value List",is_open=True)
            definition=anatomy._gh.canvas("anatomy")
            panel_build=lambda:scene.gh_panel(None,rows=scene._rows(anatomy._gh.evaluate(definition,{"vl":int(scene.vl_index.get_value())}).tree("vlp")),at=(.4,0,0))
            note_build=lambda:scene._note("index "+str(int(scene.vl_index.get_value()))+" → Type "+"ABCD"[int(scene.vl_index.get_value())],(.4,-.6,0),color=anatomy.SOFT)
            vlist=scene.live(build_list,lambda:int(scene.vl_index.get_value()))
            panel=scene.live(panel_build,lambda:int(scene.vl_index.get_value()))
            note=scene.live(lambda:VGroup(note_build()),lambda:int(scene.vl_index.get_value()))
            typenote=scene._type_note()
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[];indices=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs);index=int(scene.vl_index.get_value())
                if index:
                    actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                    fresh_type=scene._type_note();fresh_type.update(0)
                    oracle.reset();oracle.capture_mobjects([build_list(),panel_build(),VGroup(note_build()),fresh_type])
                    differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()));indices.append(index)
                return result
            scene.renderer.update_frame=update
            sequence=scene._value_sequence(vlist,panel,note,typenote)
            self.assertAlmostEqual(sequence.run_time,case["value_sequence"]["duration"])
            scene.play(sequence,run_time=case["value_sequence"]["duration"])
            self.assertTrue(differences)
            self.assertTrue(set(case["value_sequence"]["indices"][1:]).issubset(indices))
            self.assertLessEqual(max(differences),2,repr(differences))

    def test_fixed_type_ignores_unrelated_trackers(self):
        state = {"visible": True, "mode": "type", "type": "A", "window": True, "dims": (), "sun": False, "scale": .5, "box": "cb", "overrides": {}}
        keys = [anatomy._preview_key(state, (value, 4, 2)) for value in case["type_tracker_values"]]
        self.assertEqual(len(set(keys)), 1)
        state["mode"] = "box"
        self.assertNotEqual(anatomy._preview_key(state, (2, 4, 2)), anatomy._preview_key(state, (3, 4, 2)))

    def test_preview_uses_real_box_and_skips_unchanged_type(self):
        state = {"visible": True, "mode": "type", "type": "A", "window": False, "dims": (), "sun": False, "scale": .5,
                 "box": "cb", "overrides": {}, "definition": SimpleNamespace(objects={})}
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        calls = []
        def facet():
            result = VGroup(Square(fill_opacity=1, stroke_width=0))
            result.dims, result.window = {}, VGroup()
            return result
        def capsule(*args, **kwargs):
            calls.append(("capsule",))
            return facet()
        def analytical(*args, **kwargs):
            calls.append(("box", *args[:3]))
            return facet()
        scene = SimpleNamespace(view_state=state, W=ValueTracker(2), L=ValueTracker(4), H=ValueTracker(2),
                                camera=camera, capsule_axon=capsule, analytical_box_axon=analytical,
                                _dims_of=lambda *args: tuple(case["box_midpoint"]))
        preview = anatomy.Beat1_Anatomy._preview(scene)
        self.assertEqual(calls, [("capsule",)])
        scene.W.set_value(3)
        preview.update(0)
        self.assertEqual(calls, [("capsule",)])
        state["mode"] = "box"
        with patch.object(anatomy._gh, "evaluate", return_value=None):
            preview.update(0)
        self.assertEqual(calls[-1], ("box", *case["box_midpoint"]))
        self.assertEqual(len(calls), 2)
        camera.capture_mobjects([preview])

    def test_analytical_box_midpoint_matches_independent_plan(self):
        width, length, height = case["box_midpoint"]
        expected = (np.array(case["box_min"]) + np.array(case["box_max"])) / 2
        self.assertTrue(np.allclose([width, length, height], expected))
        view = drawing.NakaginDrawing().analytical_box_axon(width, length, height, dims=False)
        facets = [member.world_vertices for member in view.get_family() if isinstance(member, drawing._Facet) and hasattr(member, "world_vertices")]
        points = np.vstack(facets)
        self.assertTrue(np.allclose(points.min(axis=0), [-width / 2, 0, 0]))
        self.assertTrue(np.allclose(points.max(axis=0), [width / 2, length, height]))
        plan = MultiPoint(points[:, :2]).convex_hull
        self.assertTrue(plan.equals(box(-width / 2, 0, width / 2, length)))
        camera = Camera(pixel_width=320, pixel_height=180, frame_width=14.222222222222221, frame_height=8, background_opacity=1)
        camera.capture_mobjects([view])
        expected_pixels = np.array(camera.get_image(), dtype=int)
        preview = anatomy._snapshot.capture(view, camera)
        camera.reset()
        camera.capture_mobjects([preview])
        self.assertLessEqual(int(np.abs(expected_pixels - np.array(camera.get_image(), dtype=int)).max()), 2)


    def test_snapshot_and_selected_copy_release_original(self):
        import gc
        import weakref
        original = VGroup(Square(fill_opacity=1))
        original.capsules = {"selected": original[0]}
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=1)
        image = anatomy._snapshot.capture(original, camera)
        chosen = original.capsules["selected"].copy()
        group_ref, capsule_ref = weakref.ref(original), weakref.ref(original[0])
        pixels = image.pixel_array.copy()
        del original
        gc.collect()
        self.assertIsNone(group_ref())
        self.assertIsNone(capsule_ref())
        self.assertTrue(np.array_equal(image.pixel_array, pixels))
        self.assertGreater(len(chosen.points), 0)

    def test_anatomy_strip_is_immutable_full_detail_image(self):
        from manim import ImageMobject
        kinds = [SimpleNamespace(key=str(i)) for i in range(4)]
        camera = Camera(pixel_width=320, pixel_height=180, frame_width=14.222222222222221, frame_height=8, background_opacity=1)
        calls = []
        def capsule(kind, at, **kwargs):
            calls.append(kind.key)
            return Square(side_length=.8, fill_color="#2A5060", fill_opacity=1, stroke_width=0).move_to(at)
        scene = SimpleNamespace(camera=camera, capsule_axon=capsule, _note=lambda *args, **kwargs: VGroup(), tr=lambda en, de: en)
        result = anatomy.Beat1_Anatomy._strip(scene, kinds, 140)
        self.assertIsInstance(result, ImageMobject)
        self.assertEqual(calls, ["0", "1", "2", "3"])
        originals = [capsule(kind, anatomy._at(-3.0 + 2.0 * i, .2)) for i, kind in enumerate(kinds)]
        camera.reset()
        camera.capture_mobjects(originals)
        reference = np.array(camera.get_image(), dtype=int)
        camera.reset()
        camera.capture_mobjects([result])
        self.assertLessEqual(int(np.abs(reference - np.array(camera.get_image(), dtype=int)).max()), 2)
        self.assertLess(len(result.get_family()), sum(len(m.get_family()) for m in originals))
        from manim import FadeIn, Scene
        rendered = Scene()
        earlier = Square()
        rendered.add(earlier, result)
        moving = rendered.get_moving_mobjects(FadeIn(earlier))
        self.assertIn(result, moving)
        self.assertEqual(len(result.get_family()), 1)

class FrameTest(unittest.TestCase):
 def test_count_ui_each_frame(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
   old=os.environ.get("SEMIO_TEACHING_LANGUAGE");os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
   try:
    scene=anatomy.Beat1_Anatomy();tracker=ValueTracker(1);key=lambda:int(round(tracker.get_value()))
    slider=scene.gh_slider("Count",1,1,3,at=(-3,0,0),decimals=0,tracker=tracker,width=3)
    panel=lambda:scene.gh_panel([str(i) for i in range(key())],at=(0,0,0),width=1)
    tower=lambda:VGroup(Square(side_length=.5,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([2+(key()-1)*.5,0,0]))
    sd=anatomy._keyed.create(lambda:(slider.update(0),slider)[1],lambda:round(tracker.get_value(),6),scene.camera)
    pd=anatomy._keyed.create(panel,key,scene.camera)
    td=anatomy._keyed.create(tower,key,scene.camera)
    caption=Square(side_length=.3,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
    scene.add(sd,pd,td,caption)
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    differences=[]; original=scene.renderer.update_frame
    def update(*args,**kwargs):
     result=original(*args,**kwargs); actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
     slider.update(0);camera.reset();camera.capture_mobjects([slider,panel(),tower(),caption])
     expected=np.asarray(camera.get_image(),dtype=int)
     differences.append(int(np.abs(actual-expected).max()))
     return result
    scene.renderer.update_frame=update
    scene.play(UpdateFromAlphaFunc(tracker,lambda m,alpha:m.set_value(1+2*alpha)),run_time=1)
    self.assertTrue(differences)
    self.assertLessEqual(max(differences),2,repr(differences))
   finally:
    if old is None:os.environ.pop("SEMIO_TEACHING_LANGUAGE",None)
    else:os.environ["SEMIO_TEACHING_LANGUAGE"]=old
class GhostTest(unittest.TestCase):
 def test_old_key_does_not_remain_visible(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
   scene=Scene(); tracker=ValueTracker(0); frames=[]
   key=lambda: tracker.get_value()>.5
   def build():
    return VGroup(Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([2 if key() else -2,0,0]))
   display=anatomy._keyed.create(build,key,scene.camera)
   caption=Square(side_length=.5,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
   scene.add(display,caption)
   original=scene.renderer.update_frame
   def update(*args,**kwargs):
    result=original(*args,**kwargs);frames.append(np.asarray(scene.camera.get_image()).copy());return result
   scene.renderer.update_frame=update
   scene.play(UpdateFromAlphaFunc(tracker,lambda m,alpha:m.set_value(alpha)),run_time=1)
   frame=frames[-1];h,w=frame.shape[:2]; left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3];right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
   self.assertLess(int(left.max()),10,repr((left,right)))
   self.assertGreater(int(right.max()),100,repr((left,right)))
   for frame in frames:
    left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3]
    right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
    self.assertFalse(int(left.max())>100 and int(right.max())>100,repr((left,right)))
    top=frame[round(h/2-2*h/scene.camera.frame_height),w//2,:3]
    self.assertGreater(int(top[1]),100)
if __name__ == "__main__":
    unittest.main()
