"""🧪 Certify native pixel coverage across opaque and translucent material cells."""
import importlib,json,sys,unittest
from pathlib import Path
sys.dont_write_bytecode=True

class MaterialCoverage(unittest.TestCase):
    def test_native_cell_coverage_matches_independent_unsplit_cairo_reference(self):
        import manim,numpy as np
        from jsonschema import Draft202012Validator
        adapter=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️surfaces.🎨️coverage")
        fixture=Path(__file__).parent.parent/"🧪️tests/🔣️.json";cases=json.loads(fixture.read_text(encoding="utf8"));Draft202012Validator(json.loads(fixture.with_name("🔣️.schema.json").read_text(encoding="utf8"))).validate(cases)
        def rectangle(a,b,opacity):
            """🟫 Author an ordinary native reference region without cell-specific behavior."""
            return manim.Polygon((a,-1,0),(b,-1,0),(b,1,0),(a,1,0),fill_color="#D0C8C0",fill_opacity=opacity,stroke_width=0)
        def pixels(shapes):
            """📷 Rasterize through the actual locked native Cairo camera."""
            camera=manim.Camera(pixel_width=256,pixel_height=144,frame_width=8,frame_height=4.5,background_color="#00171F")
            camera.capture_mobjects(shapes);return camera.pixel_array.copy()
        for row in cases["coverage_cases"]:
            with self.subTest(case=row["id"]):
                reference=rectangle(-2,2,row["opacity"]);expected=pixels([reference]);left,right=rectangle(-2,row["split"],row["opacity"]),rectangle(row["split"],2,row["opacity"])
                baseline=pixels([left,right]);self.assertTrue(np.any(baseline[44:100,70:186]!=expected[44:100,70:186]))
                original_left=left.points.copy();original_right=right.points.copy()
                adapter.material_fill(left);adapter.material_fill(right);actual=pixels([left,right])
                np.testing.assert_array_equal(actual[44:100,70:186],expected[44:100,70:186])
                np.testing.assert_array_equal(left.points,original_left);np.testing.assert_array_equal(right.points,original_right)
                np.testing.assert_array_equal(pixels([reference]),expected)
