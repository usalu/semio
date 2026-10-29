"""🧪 Native occupied service bounds checked against portable cases and NumPy."""
import importlib
import json
from pathlib import Path
import unittest

import numpy as np

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class ServiceEnvelopeContract(unittest.TestCase):
    """🔧 Native equipment and hollow tubes share occupied geometry across drawing families."""

    def test_service_bounds(self):
        """📐 Independently bound world vertices and reject services without occupied material."""
        root=Path(__file__).resolve().parent
        cases=json.loads((root/"cases.json").read_text(encoding="utf-8"))["services"]
        for case in cases:
            with self.subTest(case=case["id"]):
                geometry={key:case[key] for key in ("points","size","tube") if key in case}
                attributes={"diameter":{"value":case["diameter"]}} if "diameter" in case else {}
                source={"tag":case["id"],"guid":case["id"],"class":case["class"],"storey":"first","geometry":geometry,"attributes":attributes}
                if "error" in case:
                    with self.assertRaisesRegex(ValueError,case["error"]):model._service({},source)
                    continue
                data=model._service({},source)
                vertices=np.concatenate([np.asarray(model.part_vertices(part)) for part in data["parts"]])
                lo,hi=vertices.min(axis=0),vertices.max(axis=0)
                np.testing.assert_allclose([lo[0],hi[0],lo[1],hi[1],lo[2],hi[2]],case["bounds"],rtol=0,atol=1e-12)
        authored=model.load_model()
        radiator=next(e for e in authored["elements"] if e["tag"]=="RAD-1")
        elevations={e["id"]:e["elevation"]["value"] for e in authored["storeys"]}
        z=np.asarray([elevations[p[2]] if isinstance(p[2],str) else p[2] for p in radiator["geometry"]["points"]],float)
        part=model.service_geometry(radiator["tag"],authored)["parts"][0]
        np.testing.assert_allclose([part["z0"],part["z1"]],[z.min(),max(z.max(),z.min()+radiator["geometry"]["size"][2])],rtol=0,atol=1e-12)


if __name__=="__main__":
    unittest.main()
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                