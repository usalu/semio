"""🗼 Verify discrete spreadsheet tower highlights update their display."""
import importlib
import json
from pathlib import Path
import unittest
from types import SimpleNamespace
import numpy as np
from manim import Camera, FadeIn, Scene, Square, VGroup, ValueTracker, UpdateFromAlphaFunc, tempconfig
anatomy = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🦴️anatomy.🐍️")
references = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️")
program = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📋️program.🐍️")
energy = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️")
case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

class ReferenceTowerSnapshotTest(unittest.TestCase):
    def test_selection_states(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        state = {"key": -1, "builds": 0}
        shape = Square(fill_opacity=1, stroke_width=0, fill_color="#cc4422")
        def build():
            state["builds"] += 1
            return shape
        live = references._keyed.create(build, lambda: state["key"], camera)
        child = live.submobjects[0]
        first = child.pixel_array.copy()
        live.update(0)
        self.assertEqual(state["builds"], 1)
        shape.set_fill("#2288cc")
        state["key"] = 0
        live.update(0)
        self.assertEqual(state["builds"], 2)
        self.assertIs(live.submobjects[0], child)
        self.assertFalse(np.array_equal(first, live.submobjects[0].pixel_array))
        live.update(0)
        self.assertEqual(state["builds"], 2)

    def test_selection_change_during_fade(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        state = {"key": 0}
        shape = Square(fill_opacity=1, fill_color="#ff0000")
        live = references._keyed.create(lambda: shape, lambda: state["key"], camera)
        before = live.submobjects[0].pixel_array.copy()
        fade = FadeIn(live)
        fade.begin()
        state["key"] = 1
        shape.set_fill("#00ff00")
        fade.update_mobjects(0)
        fade.finish()
        live.update(0)
        self.assertFalse(np.array_equal(before, live.submobjects[0].pixel_array))

    def test_energy_counter_keeps_coordinates_on_vector(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        drawing = VGroup(Square(fill_opacity=1).shift([2, 1, 0]))
        scene = SimpleNamespace(EN="energy", vals_base={("energy", "D8"): 1}, camera=camera,
                                tower_axon=lambda **kwargs: drawing, _readout=lambda *args, **kwargs: Square(side_length=.1))
        display, vector = energy.Beat4_Energy._tower_counter(scene, (0, 0), {})
        self.assertIs(vector, drawing)
        self.assertEqual(tuple(vector.get_center()[:2]), (2, 1))
        animation = FadeIn(display)
        animation.begin()
        animation.interpolate(.5)
        camera.capture_mobjects([display])

    def test_program_retains_current_vector(self):
        camera = Camera(pixel_width=160, pixel_height=90, frame_width=14.222222222222221, frame_height=8, background_opacity=0)
        state = {"key": 0}
        shape = Square(fill_opacity=1)
        current = [shape]
        live = program._keyed.create(lambda: current[0], lambda: state["key"], camera, retain=True)
        child = live.submobjects[0]
        self.assertIs(live._drawing_ref.value, shape)
        self.assertIs(live.copy()._drawing_ref.value, shape)
        newer = Square(fill_opacity=1, fill_color="#00ff00")
        current[0] = newer
        state["key"] = 1
        live.update(0)
        self.assertIs(live._drawing_ref.value, newer)
        self.assertIs(live.submobjects[0], child)


class StableClientFrameTest(unittest.TestCase):
    def run_frame_oracle(self,scene,build,key,tracker,keyed):
        display=keyed.create(build,key,scene.camera)
        caption=Square(side_length=.3,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([5,3,0])
        scene.add(display,caption)
        oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
        differences=[]
        original=scene.renderer.update_frame
        def update(*args,**kwargs):
            result=original(*args,**kwargs)
            actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
            oracle.reset();oracle.capture_mobjects([build(),caption])
            differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
            return result
        scene.renderer.update_frame=update
        scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(1+2*alpha)),run_time=1)
        self.assertLessEqual(max(differences),2,repr(differences))

    def test_actual_error_window_handoff_written_frames(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en";scene.s=SimpleNamespace(status=None)
            book=anatomy._xl.workbook(language="en")
            div=anatomy._xl.apply(book,[{"type":"clear-cell","sheet":anatomy.SHEET,"cell":"B6"}])
            name=anatomy._xl.apply(book,[{"type":"set-cell","sheet":anatomy.SHEET,"cell":"G16","formula":"=SUMM(G12:G15)"}])
            old=scene._window(div,selected="B15",rows=(6,15));new=scene._window(name,selected="G16",cols="C:G",rows=(10,16))
            self.assertEqual(old.cell("B15").string,"#DIV/0!")
            self.assertEqual(new.cell("G16").string,"#NAME?")
            caption=scene.text("#DIV/0! example",(0,-3.5,0),size=20,color="#ff0000")
            badge=scene.text("fix: length",(4,0,0),size=20,color="#ff0000")
            state={"phase":"old"};frames=[];scene.add(old,caption,badge)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            oracle.capture_mobjects([new]);window_mask=np.asarray(oracle.get_image())[:,:,:3].max(axis=2)>80
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                frames.append((state["phase"],np.asarray(scene.camera.get_image(),dtype=int).copy()))
                return result
            scene.renderer.update_frame=update
            def change():
                state["phase"]="gap"
                caption.become(scene.text("#NAME? example",(0,-3.5,0),size=20,color="#0000ff"))
                badge.become(scene.text("fix: SUM",(4,0,0),size=20,color="#0000ff"))
            scene._handoff(old,badge,change)
            scene.play(UpdateFromAlphaFunc(ValueTracker(0),lambda mob,alpha:mob.set_value(alpha)),run_time=.5)
            state["phase"]="new"
            scene.play(FadeIn(new),FadeIn(badge),run_time=1)
            self.assertTrue(any(phase=="gap" for phase,_ in frames))
            for phase,frame in frames:
                if phase=="gap":
                    self.assertFalse((frame[:,:,:3].max(axis=2)[window_mask]>20).any())
                if phase=="new" and (frame[60:120,220:320,2]>100).any():
                    self.assertTrue((frame[:,:,:3].max(axis=2)[window_mask]>20).any())

    def test_repair_waits_for_actual_workbook_recalculation(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en";scene.s=SimpleNamespace(err=True,status=None);scene._fittings=[]
            book=anatomy._xl.workbook(language="en")
            broken=anatomy._xl.apply(book,[{"type":"set-cell","sheet":anatomy.SHEET,"cell":"B6","value":"3.8 m"}])
            window=scene._window(broken,selected="B9",rows=(4,13))
            self.assertEqual(window.cell("B9").string,"#VALUE!")
            line=Square(side_length=.3,fill_color="#00ff00",fill_opacity=1)
            capsule=Square(side_length=.3,fill_color="#ff0000",fill_opacity=1).shift([3,0,0])
            scene.add(window,line,capsule)
            during=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs);during.append(scene.s.err);return result
            scene.renderer.update_frame=update
            scene._repair_window(window,book,line,capsule)
            self.assertTrue(during);self.assertTrue(all(during))
            self.assertFalse(scene.s.err)
            self.assertNotEqual(window.cell("B9").string,"#VALUE!")

    def test_error_handoff_keeps_old_window_with_old_state(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en"
            old=Square(side_length=1,fill_color=case["handoff"]["old"],fill_opacity=1,stroke_width=0).shift([-2,0,0])
            badge=Square(side_length=1,fill_color=case["handoff"]["old"],fill_opacity=1,stroke_width=0).shift([2,0,0])
            caption=Square(side_length=.5,fill_color=case["handoff"]["old"],fill_opacity=1,stroke_width=0).shift([0,2,0])
            scene.add(old,badge,caption)
            frames=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                frames.append(np.asarray(scene.camera.get_image(),dtype=int).copy())
                return result
            scene.renderer.update_frame=update
            def change():
                badge.set_fill(case["handoff"]["new"])
                caption.set_fill(case["handoff"]["new"])
            scene._handoff(old,badge,change)
            scene.play(FadeIn(badge),run_time=.5)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            oracle.capture_mobjects([old,badge,caption]);template=np.asarray(oracle.get_image(),dtype=int)
            old_mask=(template[:,:,0]>200)&(template[:,:,2]<10)
            badge_mask=(template[:,:,2]>200)&(template[:,:,0]<10)
            self.assertTrue(old_mask.any());self.assertTrue(badge_mask.any())
            for frame in frames[1:]:
                self.assertFalse((frame[:,:,0][old_mask]>20).any() and (frame[:,:,2][badge_mask]>200).any())

    def test_reference_row_reveal_precedes_advance(self):
        for state in case["row_sequences"]:
            with self.subTest(target=state["target"]),tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":2,"background_color":"#000000"}):
                scene=references.Beat2_References();scene.language=lambda:"en";scene.row=ValueTracker(3)
                book=references._xl.workbook();values=references._xl.evaluate(book)
                def build():
                    window=scene._window(book,references.SHEET,"C:G",(2,12),"G4",values=values)
                    scene._bind_reveal(window,[f"G{i}" for i in range(3,10)],book,values,scene.row)
                    window.update(0)
                    return window
                window=build();cursor=scene._cursor(window,scene.row,"G")
                if state["target"]==8:
                    scene.add(window)
                oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
                differences=[];original=scene.renderer.update_frame
                def update(*args,**kwargs):
                    result=original(*args,**kwargs)
                    if scene.row.get_value()>3.01:
                        actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                        fresh=build();fresh_cursor=scene._cursor(fresh,scene.row,"G");fresh_cursor.update(0)
                        oracle.reset();oracle.capture_mobjects([fresh,fresh_cursor])
                        differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                    return result
                scene.renderer.update_frame=update
                views=(cursor,) if state["target"]==8 else (window,cursor)
                sequence=scene._row_sequence(views,state["target"],state["duration"])
                self.assertAlmostEqual(sequence.run_time,state["duration"])
                scene.play(sequence,run_time=state["duration"])
                self.assertTrue(differences);self.assertLessEqual(max(differences),2,repr(differences))
                self.assertEqual(scene.row.get_value(),state["target"])

    def test_reference_label_and_faces(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=references.Beat2_References();scene.language=lambda:"en"
            tracker=ValueTracker(1);key=lambda:int(round(tracker.get_value()))
            self.run_frame_oracle(scene,lambda:VGroup(scene._say("Tower "+str(key()),(references.RIGHT_X,2.35),size=references.LABEL,color=references.DATA)),key,tracker,references._keyed)
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=references.Beat2_References();scene.language=lambda:"en";scene.w=ValueTracker(1);scene.row=ValueTracker(3)
            scene.st={"mode":"none","loss":None,"card":False,"hatch":(),"dims":()}
            scene.capsule_axon=lambda spec,at,**kw:Square(side_length=spec["width"]*.3,fill_opacity=1,stroke_width=0).move_to(at)
            faces=scene.live(scene._faces,scene._faces_key)
            self.run_frame_oracle(scene,lambda:(faces.update(0),faces)[1],scene._faces_key,scene.w,references._keyed)

    def test_program_counter(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=program.Beat3_Program();scene.language=lambda:"en";tracker=ValueTracker(1);key=lambda:str(round(tracker.get_value()))
            self.run_frame_oracle(scene,lambda:VGroup(scene.text(key(),(0,0,0),size=20)),key,tracker,program._keyed)

    def test_energy_bound_window_tracks_workbook_values(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=energy.Beat4_Energy();scene.language=lambda:"en";scene._prepare()
            scene.vs.b=scene.books["window_upgrade"][1]
            def build():
                window=scene._window(scene.EN,(3,14),"B14","A:D")
                window.update(0)
                return window
            display=build();scene.add(display)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                if scene.vs.t.get_value()>0:
                    actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                    oracle.reset();oracle.capture_mobjects([build()])
                    differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(scene.vs.t,lambda mob,alpha:mob.set_value(alpha)),run_time=1)
            self.assertTrue(differences);self.assertLessEqual(max(differences),2,repr(differences))

    def test_energy_readout_changes_glyphs_without_ghosts(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=energy.Beat4_Energy();scene.language=lambda:"en"
            tracker=ValueTracker(1)
            key=lambda:"1.0 m²" if tracker.get_value()<2 else "123.45 kWh"
            display=scene._readout(key,(0,0),size=20)
            identity=id(display)
            scene.add(display)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                oracle.reset();oracle.capture_mobjects([VGroup(scene._say(key(),(0,0),size=20,color=energy.DATA,anchor="c"))])
                differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(1+2*alpha)),run_time=1)
            self.assertEqual(id(display),identity)
            self.assertLessEqual(max(differences),5,repr(differences))
            self.assertIsInstance(VGroup(display),VGroup)

    def test_anatomy_bound_window_and_vector_restore(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":3,"background_color":"#000000"}):
            scene=anatomy.Beat1_Anatomy();scene.language=lambda:"en"
            tracker=ValueTracker(2.3);scene.s=SimpleNamespace(iw=tracker,status=None)
            book=anatomy._xl.workbook(language="en");window=scene._window(book,selected="B5")
            fn=lambda:format(anatomy._interior(round(tracker.get_value(),3))["B5"],".2f")+" m"
            window.bind("B5",fn)
            key=lambda:round(tracker.get_value(),3)
            display=anatomy._keyed.create(lambda:(window.update(0),window)[1],key,scene.camera)
            scene.add(display)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[];original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs);actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                window.update(0);oracle.reset();oracle.capture_mobjects([window])
                differences.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(2.3+.2*alpha)),run_time=1)
            self.assertLessEqual(max(differences),2,repr(differences))
            scene.remove(display);window.update(0);scene.add(window)
            self.assertIn(window,scene.mobjects)
            self.assertNotIn(display,scene.mobjects)

if __name__ == "__main__":
    unittest.main()
