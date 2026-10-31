"""⛏️ Source-neutral material joint between an inclined hollow chute and a strip footing."""
import importlib,json,unittest
from copy import deepcopy
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from jsonschema import Draft202012Validator
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces")
def contains(parts,point):
    """🔬 Classify strict occupied interiors through independent SciPy supporting half-spaces."""
    return any(np.all(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).equations[:,:3]@point+ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).equations[:,3]<0) for p in parts)
class CoalFootingInterfaceContract(unittest.TestCase):
    def test_declared_inclined_material_joint_keeps_air_and_foundation_bearing(self):
        """🪨 Compare one original source construction against GEOS/SciPy occupied material and void controls."""
        c=json.loads(Path(__file__).with_suffix('.json').read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(c);m=a.load_model();obj=a._find(m,c['footing']);obj['geometry']['interface_owners']=[c['owner']];raw=a._interface_reference_parts(m,c['footing']);chute=a._solid_geometry(m,a._find(m,c['owner']))['parts'];actual=a._solid_geometry(m,obj)['parts'];independent=sum(o.common_union_volume(p,chute) for p in raw);overlap=sum(o.common_union_volume(p,chute) for p in actual)
        self.assertAlmostEqual(independent,c['expected_removed'],delta=c['tolerance']);self.assertLess(overlap,c['tolerance'])
        before=sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in raw);after=sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in actual);self.assertAlmostEqual(before-after,independent,delta=c['tolerance'])
        for part in actual:
            a._polyhedron_part(a.part_vertices(part),a.part_faces(part));self.assertGreater(ConvexHull(np.asarray(a.part_vertices(part),dtype=float)).volume,0)
        for control in c['controls']:
            self.assertEqual(contains(actual,np.asarray(control['point'])),control['footing'],control['name']);self.assertEqual(contains(chute,np.asarray(control['point'])),control['chute'],control['name'])

    def test_source_declares_only_the_finite_coal_material_joint(self):
        """🏷️ Retain original foundation and chute source construction with an explicit finite joint."""
        c=json.loads(Path(__file__).with_suffix('.json').read_text());m=a.load_model();footing=a._find(m,c['footing']);chute=a._find(m,c['owner']);self.assertEqual(footing['geometry']['interface_owners'],[c['owner']]);self.assertEqual(len(footing['geometry']['parts']),5);self.assertEqual(len(chute['geometry']['parts']),7);self.assertEqual(footing['geometry']['base_level'],'cellar');self.assertNotIn('voids',footing['geometry'])
