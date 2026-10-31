"""📍 Measured-archive relative placement contracts, independent of an unverified metric scan scale."""
import importlib
import json
from pathlib import Path
import sys
import unittest
import numpy as np
ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
M=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
C=json.loads((ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/📍️survey/🔣️.json").read_text(encoding="utf-8"))
class SurveyPlacement(unittest.TestCase):
    def test_first_floor_membership_and_phase(self):
        for key, expected in C["first"].items():
            actual=sorted([s.facade,s.layout,s.sublevel] for s in M.slots() if M.towers()[s.tower].key==key and s.floor==3)
            self.assertEqual(actual,sorted(expected),key)
    def test_regular_b_phase_order(self):
        for floor in C["regular_b"]["floors"]:
            for facade,item,phase in C["regular_b"]["targets"]:
                selected=[s for s in M.slots() if M.towers()[s.tower].key=="B" and s.floor==floor and s.facade==facade and s.item==item]
                self.assertEqual(len(selected),1,(floor,facade,item))
                self.assertEqual(selected[0].sublevel,phase)
    def test_scan_signed_roofs_independently_resolve_one_phase(self):
        samples=np.asarray(C["regular_b"]["roof_samples"])
        delta=samples[:,2]-samples[:,1]
        self.assertTrue(np.all(delta>0))
        np.testing.assert_allclose(delta,C["regular_b"]["raw_pitch"]/3,atol=C["regular_b"]["raw_tolerance"],rtol=0)
    def test_actual_ragged_branches(self):
        branches=M.tree()
        self.assertEqual(len(branches),C["branches"])
        self.assertEqual(sum(len(v)==1 for v in branches.values()),C["singletons"])
        self.assertEqual(sum(map(len,branches.values())),C["total"])
    def test_a_exceptional_east_room_is_observed(self):
        facade,item=C["a_exceptional"]["present"]
        for floor in C["a_exceptional"]["floors"]:
            selected=[s for s in M.slots() if M.towers()[s.tower].key=="A" and s.floor==floor and s.facade==facade and s.item==item]
            self.assertEqual(len(selected),1,(floor,facade,item))
        self.assertTrue(np.all(np.asarray(C["a_exceptional"]["front_support_raw_area"])>C["a_exceptional"]["minimum_positive_area"]))
    def test_a_top_room_absence_matches_seven_positive_roofs(self):
        facade,item=C["a_exceptional"]["top_absent"]
        top=[s for s in M.slots() if M.towers()[s.tower].key=="A" and s.floor==13]
        self.assertEqual(len(top),7)
        self.assertFalse(any(s.facade==facade and s.item==item for s in top))
if __name__=="__main__":
    unittest.main()
