"""⚡ Private slab member reuse without exposing mutable memo entries."""
import importlib,json,copy,unittest
from pathlib import Path
from unittest.mock import patch
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

class SlabMemberMemoContract(unittest.TestCase):
    """🔐 Copy selected output ownership while reusing large unrelated slab payloads privately."""
    def fixture(self):
        """🧪 Load the portable memo contract and independent source object."""
        from jsonschema import Draft202012Validator
        case=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(case);model=api.load_model();obj={"tag":case["member"],"guid":api.global_id(case["member"]),"class":"IfcBeam","storey":"ground","material":"MAT-RC","name":{"en":"Neutral member","de":"Neutrales Bauteil"},"metadata":{"domains":["structure"]},"geometry":{"kind":"beam","base_level":0.,"parts":copy.deepcopy(case["source_parts"]),"embedded_slabs":["SL-1"]}};return case,model,obj

    def test_embedded_member_copies_only_selected_slab_parts(self):
        """📦 Prove zero complete-payload copies and compare material volume with independent GEOS."""
        from shapely.geometry import Polygon
        case,model,obj=self.fixture();memo=copy.deepcopy(case["cached_slab"]);api._SLAB_GEOMETRIES.clear()
        with patch.object(api,"_slab_geometry_uncached",return_value=memo),patch.object(api,"deepcopy",wraps=copy.deepcopy) as clone:
            body=api._solid_geometry(model,obj);self.assertFalse(any(call.args and call.args[0] is memo for call in clone.call_args_list),"The complete unrelated slab payload was copied")
        self.assertEqual(len(body["shared_parts"]),case["expected"]["shared_count"]);volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in body["full_parts"]);self.assertAlmostEqual(volume,case["expected"]["full_volume"])
        body["shared_parts"][0]["outline"][0][0]=999.;self.assertEqual(memo,case["cached_slab"])

    def test_public_slab_values_and_producer_changes_are_isolated(self):
        """🛡️ Keep public slab copies isolated and producer identities distinct."""
        case,model,obj=self.fixture();memo=copy.deepcopy(case["cached_slab"]);api._SLAB_GEOMETRIES.clear();slab=api._find(model,"SL-1")
        with patch.object(api,"_slab_geometry_uncached",return_value=memo) as build:
            first=api._slab_geometry(model,slab);first["parts"][0]["outline"][0][0]=999.;second=api._slab_geometry(model,slab);self.assertEqual(second,case["cached_slab"]);self.assertEqual(build.call_count,1)
        changed=copy.deepcopy(memo);changed["marker"]="changed-producer"
        with patch.object(api,"_slab_geometry_uncached",return_value=changed):self.assertEqual(api._slab_geometry(model,slab)["marker"],"changed-producer")

    def test_scalar_soffit_does_not_clone_unrelated_payload(self):
        """📍 Read a scalar datum without copying unrelated roof finish geometry."""
        case,model,obj=self.fixture();memo=copy.deepcopy(case["cached_slab"]);api._SLAB_GEOMETRIES.clear()
        with patch.object(api,"_slab_geometry_uncached",return_value=memo),patch.object(api,"deepcopy",wraps=copy.deepcopy) as clone:
            self.assertEqual(api._top(model,"soffit:SL-1"),case["expected"]["soffit"]);self.assertFalse(any(call.args and call.args[0] is memo for call in clone.call_args_list))
