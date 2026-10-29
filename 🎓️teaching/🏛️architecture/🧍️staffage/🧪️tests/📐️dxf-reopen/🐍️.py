"""🐍️ Oracle for the DXF writer: ezdxf reads the file, audits it and reports what it found as JSON."""
import json
import sys

import ezdxf
from ezdxf import bbox
from ezdxf.lldxf.encoding import decode_dxf_unicode

document = ezdxf.readfile(sys.argv[1])
auditor = document.audit()
msp = document.modelspace()
lines = list(msp.query("LWPOLYLINE"))
extents = bbox.extents(lines)
print(json.dumps({
    "version": document.dxfversion,
    "insunits": document.header["$INSUNITS"],
    "measurement": document.header["$MEASUREMENT"],
    "extmin": list(document.header["$EXTMIN"]),
    "extmax": list(document.header["$EXTMAX"]),
    "handseed": document.header["$HANDSEED"],
    "errors": [str(e.message) for e in auditor.errors],
    "fixes": [str(e.message) for e in auditor.fixes],
    "layers": {layer.dxf.name: {"aci": layer.dxf.color, "lineweight": layer.dxf.lineweight} for layer in document.layers},
    "lwpolylines": [{"layer": e.dxf.layer, "closed": e.closed, "points": [list(p[:2]) for p in e.get_points("xy")]} for e in lines],
    "texts": [{"layer": e.dxf.layer, "text": decode_dxf_unicode(e.dxf.text), "height": e.dxf.height, "at": list(e.dxf.insert)[:2]} for e in msp.query("TEXT")],
    "bbox": [list(extents.extmin)[:2], list(extents.extmax)[:2]] if extents.has_data else None,
    "layouts": [layout.name for layout in document.layouts],
    "handles": len(set(e.dxf.handle for e in document.entitydb.values())) == len(list(document.entitydb.values())),
}))
