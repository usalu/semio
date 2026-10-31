"""🧮️ True native partition quantities with overlap-aware composite fallback."""
import importlib,json,unittest
from copy import deepcopy
from pathlib import Path
from unittest.mock import patch
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
from jsonschema import Draft202012Validator
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
def case():
    """🗂️ Validate language-neutral native cell and overlapping construction inputs."""
    c=json.loads(Path(__file__).with_suffix('.json').read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(c);return c
class NativePartitionQuantityContract(unittest.TestCase):
    def test_true_mesh_section_taper_and_partial_hole_volumes(self):
        """📐 Compare each native solid against independent SciPy hull or GEOS difference volume."""
        for row in case()['cells']:
            p=deepcopy(row['part'])
            if p.get('holes'):expected=Polygon(p['outline']).difference(unary_union([Polygon(h) for h in p['holes']])).area*(p['z1']-p['z0'])
            else:expected=ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume;p.update(a._polyhedron_part(a.part_vertices(p),a.part_faces(p)))
            self.assertAlmostEqual(expected,row['volume']);self.assertAlmostEqual(a._partition_volume([p]),expected,places=10)
    def test_actual_inclined_stringer_quantity_uses_native_material(self):
        """🪜 Preserve actual inclined twelve-by-twenty-six-centimetre stringer volume and mass."""
        c=case()['stringer'];m=a.load_model();g=a.solid_geometry(c['tag'],m);expected=sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in g['parts']);self.assertAlmostEqual(expected,c['volume'],places=10);q=a.quantities(c['tag'],m);self.assertAlmostEqual(q['volume'],expected,places=8);self.assertAlmostEqual(sum(x['volume'] for x in q['material_quantities']),expected,places=8)
    def test_one_constructed_flight_does_not_repeat_union_integration(self):
        """🎯 Use exact disjoint kernel cells for a constructed flight without arbitrary-body shortcuts."""
        c=case()['query'];m=a.load_model()
        with patch.object(a,'_occupied_volume',side_effect=AssertionError('A proven single-stock partition must not repeat occupied unions')):q=a.quantities(c['tag'],m)
        self.assertAlmostEqual(q['volume'],c['volume'],places=8)
    def test_overlapping_parent_landing_keeps_actual_union_semantics(self):
        """🧩 Prevent a native ramp class or owner reference from certifying overlapping source components."""
        c=case()['overlap'];m=a.load_model();parent=deepcopy(a._find(m,'RM-1'));flight=deepcopy(a._find(m,'RF-1A'));quantity=lambda v:{'value':v,'unit':'m','basis':'inferred'};parent['geometry'].update(from_level=0.,to_level=1.,landings=[{'tag':'NEUTRAL-MID','owner':parent['tag'],'ownership':'ramp_component','portion':.5,'outline':c['landing'],'walking_outline':c['landing'],'thickness':quantity(c['thickness'])}],finish_thickness=quantity(c['finish']));parent['geometry'].pop('interface_owners',None);parent['attributes']['width']=quantity(c['width']);flight['geometry'].update(path=c['path'],from_level=0.,to_level=1.,portion=[0,1],structural_offsets=[quantity(-.5),quantity(.5)],walking_offsets=[quantity(-.5),quantity(.5)],finish_thickness=quantity(c['finish']),finish_direction='vertical',thickness=quantity(c['thickness']));flight['geometry'].pop('interface_owners',None);flight['geometry'].pop('flight_joint_owners',None);flight['attributes']['width']=quantity(c['width']);m['elements']=[parent,flight];g=a.ramp_geometry(parent['tag'],m)
        o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces");expected=sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in g['parts']+g['child_parts'])-sum(o.common_volume(p,q) for p in g['parts'] for q in g['child_parts']);self.assertAlmostEqual(expected,c['volume'],places=10)
        original=a._occupied_volume
        with patch.object(a,'_occupied_volume',wraps=original) as union:q=a.quantities(parent['tag'],m)
        self.assertTrue(union.called);self.assertAlmostEqual(q['volume'],expected,places=8)
