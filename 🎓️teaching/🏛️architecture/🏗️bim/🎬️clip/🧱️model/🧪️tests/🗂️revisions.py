"""🗂️ Process-local geometry revisions bind complete source inputs and real producer identities."""
import importlib,json,unittest
from pathlib import Path
from unittest.mock import patch
from jsonschema import Draft202012Validator
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
class GeometryRevisionContract(unittest.TestCase):
    def test_input_revision_producer_invalidation_and_public_isolation(self):
        """🔄 Observe source/producer changes without exposing cached model or geometry ownership."""
        c=json.loads(Path(__file__).with_suffix('.json').read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(c);calls=[];m=dict(c['payloads'][0])
        def build(model):calls.append(model['counter']);return {'points':[[model['counter'],0]]}
        with patch.object(a,'_building_geometry_uncached',build):
            first=a.geometry_revision(m);self.assertIsInstance(first,int);self.assertEqual(a.geometry_revision(m),first);out=a.building_geometry(m);out['points'][0][0]=999;self.assertEqual(a.building_geometry(m)['points'],[[1,0]]);self.assertEqual(calls,[1]);m.update(c['payloads'][1]);self.assertNotEqual(a.geometry_revision(m),first);self.assertEqual(a.building_geometry(m)['points'],[[2,0]])
            for name in c['producer_names']:
                before=a.geometry_revision(m)
                with patch.object(a,name,lambda *args:None):self.assertNotEqual(a.geometry_revision(m),before)
            before=a.geometry_revision(m)
            with patch.object(a._analysis,c['analysis_producer'],lambda *args:None):self.assertNotEqual(a.geometry_revision(m),before)
            before=a.geometry_revision(m)
            with patch.object(a._ramp_finish_backend(),c['backend_producer'],lambda *args:None):self.assertNotEqual(a.geometry_revision(m),before)
            for payload in c['payloads']:a.geometry_revision(payload)
            self.assertLessEqual(a._geometry_models_cached.cache_info().currsize,c['maximum_models'])
