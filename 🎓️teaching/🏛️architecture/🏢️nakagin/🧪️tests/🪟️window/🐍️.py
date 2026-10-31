"""🪟 Validate the portable local glazing circle against raw values, real glazing vertices and Rhino."""
import copy
import importlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
DATA=json.loads((ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json').read_text(encoding='utf-8'))
CASE=json.loads((ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🪟️window/🔣️.json').read_text(encoding='utf-8'))

class WindowCircle(unittest.TestCase):
    def expected(self,data,parameters,family):
        casing=data['capsule']['casing']
        width=parameters['width']+casing['width']['value']-data['capsule']['width']['value']
        length=parameters['length']+casing['length']['value']-data['capsule']['length']['value']
        front=(parameters['length']+length)/2
        right=family['window_wall']=='right'
        inset=casing['glazing_recess_side' if right else 'glazing_recess_front']['value']
        center=(width/2-inset,parameters['length']/2+family['window_longitudinal_offset']['value']+parameters['window_u'],parameters['window_v']) if right else (parameters['window_u'],front-inset,parameters['window_v'])
        return center,(1,0,0) if right else (0,1,0),parameters['window_diameter']/2

    def assert_circle(self,actual,expected):
        center,normal,radius=expected
        np.testing.assert_allclose(actual.center,center,atol=CASE['tolerance'],rtol=0)
        np.testing.assert_allclose(actual.normal,normal,atol=CASE['tolerance'],rtol=0)
        self.assertAlmostEqual(actual.radius,radius,places=8)
        oracle=rh.Circle(rh.Point3d(*center),radius)
        plane=rh.Plane(rh.Point3d(*center),rh.Vector3d(*normal))
        np.testing.assert_allclose(actual.normal,(plane.ZAxis.X,plane.ZAxis.Y,plane.ZAxis.Z),atol=CASE['tolerance'],rtol=0)
        self.assertTrue(oracle.IsValid)
        np.testing.assert_allclose(actual.center,(oracle.Center.X,oracle.Center.Y,oracle.Center.Z),atol=CASE['tolerance'],rtol=0)

    def test_api_matches_actual_glazing_midplanes(self):
        for family in CASE['families']:
            with self.subTest(family=family):
                circle=M.capsule_window(CASE['type'],family)
                assembly=M.capsule_assembly(CASE['type'],detail='exterior',layout=family)
                glass=next(part for part in assembly.parts if part.role=='glazing')
                vertices=np.asarray(glass.solid.vertices)
                center=vertices.mean(axis=0)
                np.testing.assert_allclose(circle.center,center,atol=CASE['tolerance'],rtol=0)
                centered=vertices-center
                _,_,axes=np.linalg.svd(centered,full_matrices=False)
                normal=axes[-1]
                self.assertAlmostEqual(abs(float(normal@np.asarray(circle.normal))),1,places=8)
                radii=np.linalg.norm(centered-np.outer(centered@normal,normal),axis=1)
                self.assertAlmostEqual(float(radii.mean()),circle.radius,places=8)
                parameters={key:DATA['capsule'][key]['value'] for key in ('width','length')}
                parameters.update({key:DATA['types'][0][key]['value'] for key in ('window_u','window_v','window_diameter')})
                self.assert_circle(circle,self.expected(DATA,parameters,next(f for f in DATA['families'] if f['key']==family)))

    def test_dynamic_family_offset_and_parameter_dictionary(self):
        for offset in CASE['offset_overrides']:
            data=copy.deepcopy(DATA)
            for family in data['families']:
                if family['window_wall']=='right':
                    family['window_longitudinal_offset']['value']=offset
            with patch.object(M,'_data',return_value=data):
                for family in data['families']:
                    circle=M.capsule_window(CASE['prototype'],family['key'])
                    self.assert_circle(circle,self.expected(data,CASE['prototype'],family))

if __name__=='__main__':
    unittest.main(verbosity=2)
