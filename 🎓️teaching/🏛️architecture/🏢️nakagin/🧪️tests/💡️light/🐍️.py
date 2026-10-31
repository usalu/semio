"""💡 Verify the source-visible long lamp position and physical housing/fixture contacts."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys
import unittest
import numpy as np

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin"
M=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
C=json.loads((BASE/"🧪️cases/💡️light/🔣️.json").read_text(encoding="utf8"))
spec=importlib.util.spec_from_file_location("reference_light_oracles",BASE/"🧪️tests/🔧️mounts/🐍️.py")
O=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=O
spec.loader.exec_module(O)

class LongWallLight(unittest.TestCase):
    def test_lamp_is_above_entry_grille_below_ceiling_and_over_its_wall_span(self):
        for layout in C["layouts"]:
            parts=M.capsule_assembly("A","full",layout).parts
            light=np.asarray([v for p in parts if p.key.startswith(C["prefix"]) for v in p.solid.vertices])
            grille=np.asarray([v for p in parts if p.key.startswith(C["vent_prefix"]) for v in p.solid.vertices])
            self.assertGreaterEqual(float(light[:,2].min()),float(grille[:,2].max())-C["tolerance"],layout)
            self.assertLessEqual(float(light[:,2].max()),M.capsule_interior("A").top+C["tolerance"],layout)
            self.assertLessEqual(M.capsule_interior("A").top-float(light[:,2].max()),C["maximum_ceiling_gap"],layout)
            self.assertGreaterEqual(float(light[:,1].min()),float(grille[:,1].min())-C["tolerance"],layout)
            self.assertLessEqual(float(light[:,1].max()),float(grille[:,1].max())+C["tolerance"],layout)

    def test_all_housing_diffuser_and_tube_contacts_are_real_material(self):
        for layout in C["layouts"]:
            parts={p.key:p for p in M.capsule_assembly("A","full",layout).parts}
            for a,b in C["links"]:
                a=C["prefix"]+a
                b=b if b=="media-wall" else C["prefix"]+b
                self.assertTrue(O.contact(parts[a].solid,parts[b].solid),(layout,a,b))
            for key,part in parts.items():
                if key.startswith(C["prefix"]):
                    self.assertEqual(part.solid.meta.get("source"),C["source"],key)
                    self.assertEqual(part.solid.meta.get("basis"),C["basis"],key)
                    self.assertTrue(O.rhino_closed(part.solid),key)

if __name__=="__main__":
    unittest.main()
