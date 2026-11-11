"""🗼 Verify keyed tower presentations track discrete model changes."""
import importlib
import json
from pathlib import Path
import unittest
import numpy as np
from manim import Camera, FadeIn, FadeOut, Group, ImageMobject, Scene, Square, VGroup, ValueTracker, UpdateFromAlphaFunc, tempconfig

drafting = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
casting = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
anatomy = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
tower = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️")

class SharedKeyedTest(unittest.TestCase):
    def test_full_drawing_snapshot_keeps_pixels_and_independent_target(self):
        import gc,weakref
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=0)
        drawing=VGroup(Square(fill_color="#00cc88",fill_opacity=1))
        target=drawing[0].copy();anchor=target.get_center().copy();reference=weakref.ref(drawing)
        display=tower._snapshot.capture(drawing,camera);pixels=display.pixel_array.copy()
        del drawing
        gc.collect();self.assertIsNone(reference())
        np.testing.assert_allclose(target.get_center(),anchor)
        np.testing.assert_array_equal(display.pixel_array,pixels)
        camera.capture_mobjects([display,target])
        self.assertGreater(np.asarray(camera.get_image())[:,:,3].max(),0)

    def test_tower_display_releases_semantic_vector_after_last_use(self):
        import gc,weakref
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=0)
        source=Square(fill_color="#ff0000",fill_opacity=1)
        reference=weakref.ref(source)
        target=source.copy();anchor=target.get_center().copy()
        display=tower._keyed.create(lambda:Square(),lambda:0,camera,initial=source,retain=True)
        copy=display.copy();pixels=display.submobjects[0].pixel_array.copy()
        del source
        gc.collect();self.assertIsNotNone(reference())
        tower._release_drawing(display)
        gc.collect();self.assertIsNone(reference())
        self.assertIsNone(copy._drawing_ref.value)
        np.testing.assert_array_equal(display.submobjects[0].pixel_array,pixels)
        np.testing.assert_allclose(target.get_center(),anchor)
        self.assertGreater(len(target.points),0)
        camera.capture_mobjects([display,target])
        self.assertGreater(np.asarray(camera.get_image())[:,:,3].max(),0)

    def test_tower_retired_note_does_not_reappear(self):
        case=fixture["note_lifecycle"]
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=tower.Beat3_Tower();scene.language=lambda:"en"
            def label(text):
                result=scene._say(text,(-2.35,-2.3),22,tower.TXT);result.plain=True
                return result
            old=label(case["old"])
            a=tower._morph.token("76",kind="number",color=tower.ARCH,size=1.3).move_to([.7,-2.3,0])
            b=tower._morph.token("64",kind="number",color=tower.WARN,size=1.3).move_to([1.7,-2.3,0])
            group=VGroup(old,a,b)
            sources_a=[Square(side_length=.1).shift([i*.2,1,0]) for i in range(case["target_count"])]
            sources_b=[Square(side_length=.1).shift([i*.2,2,0]) for i in range(case["target_count"])]
            scene.add(*sources_a,*sources_b)
            scene.play(FadeIn(old),tower._morph.gather(sources_a,a,run_time=.4),tower._morph.gather(sources_b,b,run_time=.4),run_time=.4)
            scene.remove(*group.get_family()[1:]);scene.add(group)
            total=label(case["total"])
            targets_a=[Square(side_length=.1).shift([i*.2,1,0]) for i in range(case["target_count"])]
            targets_b=[Square(side_length=.1).shift([i*.2,2,0]) for i in range(case["target_count"])]
            scene.play(FadeOut(group),FadeIn(total),tower._morph.split(a,targets_a,run_time=.4),tower._morph.split(b,targets_b,run_time=.4),run_time=.4)
            scene.play(FadeOut(VGroup(*targets_a,*targets_b)),run_time=.4);scene.remove(*targets_a,*targets_b)
            tower._retire(scene,group)
            self.assertNotIn(old,scene.get_mobject_family_members())
            self.assertIn(total,scene.get_mobject_family_members())
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            oracle.capture_mobjects([total]);expected=np.asarray(oracle.get_image(),dtype=int)
            differences=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                differences.append(int(np.abs(np.asarray(scene.camera.get_image(),dtype=int)-expected).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(tower._morph.retext(total,label(case["next"]),run_time=1),run_time=1)
            self.assertTrue(differences)
            self.assertLessEqual(differences[0],2,repr(differences))

    def test_casting_spin_and_order_displays_have_no_old_frame(self):
        for name in ("spin", "order"):
            with self.subTest(name=name),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
                scene=casting.Beat4_Casting();scene.language=lambda:"en"
                tracker=ValueTracker(0)
                build=(lambda:scene._stage(spin=tracker.get_value(),arrows=True,arc=True,legend=True).over) if name=="spin" else (lambda:scene._order(tracker.get_value(),walker=False))
                display=scene._changing_display(build,lambda:round(tracker.get_value(),2))
                scene.add(display)
                oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                differences=[];original=scene.renderer.update_frame
                def update(*args,**kwargs):
                    result=original(*args,**kwargs);actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                    oracle.reset();oracle.capture_mobjects([build()])
                    differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                    return result
                scene.renderer.update_frame=update
                scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(alpha)),run_time=1)
                self.assertLessEqual(max(differences),2,repr(differences))

    def test_casting_storey_overlay_has_no_old_frame(self):
        from types import SimpleNamespace
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=casting.Beat4_Casting();scene.language=lambda:"en"
            scene._tower=lambda n=None:None
            scene._towers={None:SimpleNamespace(at=lambda x,y,z:np.array([x*.08,y*.08+z*.12,0.0]))}
            tracker=ValueTracker(9);count=lambda:int(round(tracker.get_value()))
            display=scene._lit_display(count)
            shell=Square(side_length=3,fill_color="#224466",fill_opacity=1,stroke_width=0)
            scene.add(shell,display)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                oracle.reset();oracle.capture_mobjects([shell,scene._lit(count())])
                differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(9-7*alpha)),run_time=1)
            self.assertEqual(len(differences),7)
            self.assertEqual(len(differences[1:]),6)
            self.assertLessEqual(max(differences[1:]),2,repr(differences))

    def test_mixed_sprite_and_vector_pixels_match_camera(self):
        keyed=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️keyed")
        sprites=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        vector=Square(side_length=1,fill_color="#3399cc",fill_opacity=1,stroke_width=0).shift([-1,0,0])
        sprite=sprites.capture(vector,camera)
        overlay=Square(side_length=.6,fill_color="#ff8800",fill_opacity=.6,stroke_width=0).shift([-.7,0,0])
        background=Square(side_length=1.6,fill_color="#aa3366",fill_opacity=.4,stroke_width=0).shift([-1,0,0])
        drawing=Group(background,sprite,overlay)
        camera.capture_mobjects([drawing]);expected=np.asarray(camera.get_image(),dtype=int).copy()
        display=keyed.create(lambda:drawing,lambda:0,camera)
        camera.reset();camera.capture_mobjects([display])
        actual=np.asarray(camera.get_image(),dtype=int)
        delta=np.abs(expected-actual)
        position=np.unravel_index(np.argmax(delta),delta.shape)
        self.assertLessEqual(int(delta.max()),2,repr((position,expected[position[0],position[1]],actual[position[0],position[1]])))

    def test_stable_identity_cache_and_semantic_reference(self):
        keyed=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️keyed")
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        state={"key":0}; calls=[]
        def build():
            calls.append(state["key"])
            return VGroup(Square(fill_opacity=1,fill_color="#ff0000" if state["key"] else "#0000ff"))
        live=keyed.create(build,lambda:state["key"],camera,retain=True,cache=True)
        child=live[0]
        original=live._drawing_ref.value
        state["key"]=1;live.update(0)
        self.assertIs(live[0],child)
        self.assertIsNot(live._drawing_ref.value,original)
        state["key"]=0;live.update(0)
        self.assertIs(live[0],child)
        self.assertIs(live._drawing_ref.value,original)
        self.assertEqual(calls,[0,1])
        self.assertIs(live.copy()._drawing_ref.value,live._drawing_ref.value)

class LiveSnapshotTest(unittest.TestCase):
    def test_live_vector_can_return_for_semantic_morph(self):
        from manim import ReplacementTransform
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":4}):
            scene=Scene()
            tracker=ValueTracker(0)
            key=lambda:round(tracker.get_value(),2)
            live=drafting.Live(lambda:VGroup(Square(fill_opacity=1).shift([key(),0,0])),key=key)
            source=Square(side_length=.2)
            scene.add(source)
            scene.play(ReplacementTransform(source,live),run_time=.5)
            display=tower._keyed.create(lambda:(live.update(0),live)[1],key,scene.camera)
            scene.remove(live,*live.get_family())
            scene.add(display)
            scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(alpha)),run_time=1)
            scene.remove(display)
            live.update(0)
            scene.add(live)
            target=Square(side_length=.4).shift([2,0,0])
            scene.play(ReplacementTransform(live,target),run_time=.5)
            self.assertIn(target,scene.mobjects)
            self.assertNotIn(display,scene.mobjects)

    def test_discrete_keys_and_unchanged_vector(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        state = {"key": 0, "builds": 0}
        shape = Square(fill_opacity=1, stroke_width=0, fill_color="#ff0000")
        points = shape.points.copy()
        def build():
            state["builds"] += 1
            return shape
        live = anatomy._keyed.create(build, lambda: state["key"], camera)
        pixels = live.submobjects[0].pixel_array.copy()
        live.update(0)
        self.assertEqual(state["builds"], 1)
        shape.set_fill("#00ff00")
        state["key"] = 1
        live.update(0)
        self.assertEqual(state["builds"], 2)
        self.assertFalse(np.array_equal(pixels, live.submobjects[0].pixel_array))
        self.assertTrue(np.array_equal(points, shape.points))
        live.update(0)
        self.assertEqual(state["builds"], 2)

    def test_storey_keyed_display(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        state = {"key": 1, "builds": 0}
        shape = Square(fill_opacity=1, fill_color="#ff0000")
        current = [shape]
        def build():
            state["builds"] += 1
            return current[0]
        live = tower._keyed.create(build, lambda: state["key"], camera, retain=True)
        self.assertIs(live._drawing_ref.value, shape)
        self.assertIs(live.copy()._drawing_ref.value, shape)
        animation = FadeIn(live)
        animation.begin()
        animation.interpolate(.5)
        camera.capture_mobjects([live])
        animation.finish()
        live.update(0)
        self.assertEqual(state["builds"], 1)
        newer = Square(fill_opacity=1, fill_color="#00ff00").shift([0, 1, 0])
        current[0] = newer
        state["key"] = 2
        live.update(0)
        self.assertEqual(state["builds"], 2)
        self.assertIs(live._drawing_ref.value, newer)
        self.assertAlmostEqual(live._drawing_ref.value.get_top()[1], newer.get_top()[1])

class TowerPresentationTest(unittest.TestCase):
    def test_scene_display_keeps_vector_and_overlay(self):
        camera=Camera(pixel_width=320,pixel_height=180,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        capsule=Square(side_length=1,fill_color=fixture["tower_color"],fill_opacity=1,stroke_width=0)
        vector=VGroup(capsule); vector.capsules={(0,0,0,0,0):capsule}
        axes=Square(side_length=.4,stroke_color="#ffcc00",stroke_width=3).shift([1,1,0])
        original=capsule.points.copy()
        camera.capture_mobjects([vector,axes]); baseline=np.asarray(camera.get_image(),dtype=int).copy()
        display=tower._tower_display(vector,camera,axes)
        self.assertIsInstance(display,Group)
        self.assertIsInstance(display[0],ImageMobject)
        self.assertIs(display[1],axes)
        self.assertIs(vector.capsules[(0,0,0,0,0)],capsule)
        self.assertTrue(np.array_equal(capsule.points,original))
        camera.reset(); camera.capture_mobjects([display])
        self.assertLessEqual(int(np.abs(baseline-np.asarray(camera.get_image(),dtype=int)).max()),2)

    def test_tower_vector_releases_after_last_lookup(self):
        import gc
        import weakref
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        vector=VGroup(Square(fill_opacity=1))
        original=weakref.ref(vector)
        display=tower._tower_display(vector,camera,Square(side_length=.2))
        pixels=display[0].pixel_array.copy()
        del vector
        gc.collect()
        self.assertIsNone(original())
        self.assertTrue(np.array_equal(display[0].pixel_array,pixels))

    def test_scene_fade_cleanup(self):
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":4}):
            scene=Scene()
            vector=VGroup(Square(fill_opacity=1))
            axes=Square(side_length=.3).shift([1,1,0])
            display=tower._tower_display(vector,scene.camera,axes)
            scene.play(FadeIn(display),run_time=.5)
            self.assertIn(display,scene.mobjects)
            scene.play(FadeOut(display),run_time=.5)
            self.assertNotIn(display,scene.mobjects)
            self.assertEqual(len(vector),1)


class CastingPresentationTest(unittest.TestCase):
    def test_domain_vector_can_be_replaced_after_morph(self):
        from manim import ReplacementTransform
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":4}):
            scene=Scene()
            capsule=Square(fill_opacity=1)
            bar=Square(side_length=.2).shift([1,1,0])
            world=VGroup(capsule,bar)
            start=Square(side_length=.3)
            scene.add(start)
            scene.play(ReplacementTransform(start,capsule),run_time=.5)
            scene.add(bar)
            snapshot=casting._snapshot.capture(world,scene.camera)
            scene.remove(world,*world.get_family())
            scene.add(snapshot)
            self.assertNotIn(capsule,scene.get_mobject_family_members())
            self.assertNotIn(bar,scene.get_mobject_family_members())
            scene.play(FadeOut(snapshot),run_time=.5)
            self.assertNotIn(snapshot,scene.mobjects)

    def test_window_state_changes_during_scene_animation(self):
        from manim import UpdateFromAlphaFunc, ValueTracker
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":4}):
            scene=Scene()
            tracker=ValueTracker(0)
            calls=[]
            def key():
                return tracker.get_value() > .5
            def build():
                calls.append(key())
                return VGroup(Square(fill_opacity=1,fill_color="#ff0000" if key() else "#0000ff"))
            display=casting._keyed.create(build,key,scene.camera,cache=True)
            scene.play(FadeIn(display),run_time=.5)
            scene.play(UpdateFromAlphaFunc(tracker,lambda obj,alpha:obj.set_value(alpha)),run_time=1)
            self.assertTrue(display._snapshot_key)
            self.assertEqual(calls,[False,True])
            scene.play(FadeOut(display),run_time=.5)
            self.assertNotIn(display,scene.mobjects)

    def test_window_states_cache_and_copy(self):
        state={"window":fixture["initial_window"]}
        calls=[]
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        def build():
            calls.append(state["window"])
            return VGroup(Square(fill_color="#ff0000" if state["window"] else "#0000ff",fill_opacity=1,stroke_width=0))
        display=casting._keyed.create(build,lambda:state["window"],camera,cache=True)
        self.assertEqual(calls,[False])
        display.update(0)
        state["window"]=True; display.update(0)
        self.assertEqual(calls,[False,True])
        state["window"]=False; display.update(0)
        self.assertEqual(calls,[False,True])
        clone=display.copy()
        clone[0].set_opacity(.2)
        self.assertNotEqual(int(clone[0].pixel_array[:,:,3].max()),int(display[0].pixel_array[:,:,3].max()))
        state["window"]=True
        clone.update(0)
        self.assertFalse(display._snapshot_key)
        self.assertTrue(clone._snapshot_key)
        display.update(0)
        self.assertTrue(display._snapshot_key)
        self.assertEqual(calls,[False,True])
        camera.capture_mobjects([display])

class BooleanTimingTest(unittest.TestCase):
    def test_boolean_timing(self):
        self.check_timing(fixture["off_start"],fixture["off_end"],fixture["toggle_duration"],False)
    def test_integer_timing(self):
        self.check_timing(fixture["integer_off_start"],fixture["integer_off_end"],fixture["integer_duration"],True)
    def check_timing(self,start,end,duration,integer):
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":10}):
            scene=Scene(); tracker=ValueTracker(1); values=[]
            on=lambda:tracker.get_value()>.5
            def build():
                return VGroup(Square(fill_color="#ff0000" if on() else "#0000ff",fill_opacity=1,stroke_width=0))
            widget=drafting.Live(build,key=on)
            canvas=VGroup(widget)
            image=casting._keyed.create(build,on,scene.camera,cache=True)
            observer=Square(side_length=.01,fill_opacity=0,stroke_width=0)
            def observe(_):
                rgb=np.asarray(widget.current[0].get_fill_color().to_rgb())
                pixels=image[0].pixel_array
                pixel=pixels[pixels.shape[0]//2,pixels.shape[1]//2,:3]
                if pixel.max()>15:
                    values.append((bool(rgb[0]>rgb[2]),bool(pixel[0]>pixel[2])))
            observer.add_updater(observe); scene.add(observer)
            flip=UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(0 if start<alpha<end else 2 if integer and alpha>=end else 1),run_time=duration)
            scene.play(FadeIn(canvas),FadeIn(image),run_time=1)
            scene.play(flip,run_time=duration)
            mismatches=[pair for pair in values if pair[0]!=pair[1]]
            self.assertGreater(len(values),0)
            self.assertEqual(mismatches,[])
if __name__ == "__main__":
    unittest.main()
