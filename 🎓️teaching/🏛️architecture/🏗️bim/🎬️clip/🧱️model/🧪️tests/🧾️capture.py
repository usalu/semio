"""🧾️ Reject stale or partially identified expanded geometry captures before an independent audit."""
import json,unittest
from pathlib import Path
CAPTURE_KIND='whole-expanded-owned-solids-r92-v2'
def capture_issues(data: dict, source: str, producers: dict) -> list:
    """🔐 Compare complete source and producer identities without accepting an earlier capture namespace."""
    expected={'capture_kind':CAPTURE_KIND,'source':source,'source_after':source,'producers_before':producers,'producers_after':producers,'loaded_producer_changes':{},'untracked_loaded_producers':[],'capture_stable':True}
    return [key for key,value in expected.items()if data.get(key)!=value]
class CaptureIdentity(unittest.TestCase):
    def test_capture_identity_matches_independent_schema_constants(self):
        from jsonschema import Draft202012Validator
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));schema=json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'));source,producers=case['source'],case['producers']
        base={'capture_kind':case['kind'],'source':source,'source_after':source,'producers_before':producers,'producers_after':producers,'loaded_producer_changes':{},'untracked_loaded_producers':[],'capture_stable':True}
        for key,value in base.items():schema['properties'][key]['const']=value
        for row in case['cases']:
            with self.subTest(row=row['id']):
                data={**base,**row['changes']};native=not capture_issues(data,source,producers);oracle=not list(Draft202012Validator(schema).iter_errors(data));self.assertEqual(native,oracle);self.assertEqual(native,row['accepted'])
