"""Reads a DWG file with ezdwg, a DWG reader that shares no code with the staffage writer, and prints what it finds as JSON: the format version, the drawing units, the header extents, the layers with colour and lineweight, the layouts and every model space LWPOLYLINE and TEXT with the layer it sits on.

Usage: dwg.py <file.dwg>"""
import json
import sys

import ezdwg


def main(path):
    document = ezdwg.read(path)
    layers = document.layers()
    by_handle = {entry["handle"]: name for name, entry in layers.items()}
    header = document.header_variables()
    model = document.modelspace()
    entities = []
    for entity in model.query("LWPOLYLINE TEXT"):
        fields = entity.dxf
        row = {"type": entity.dxftype, "handle": entity.handle, "layer": by_handle.get(fields["layer_handle"]), "colorIndex": fields["color_index"], "resolvedColorIndex": fields["resolved_color_index"]}
        if entity.dxftype == "LWPOLYLINE":
            row.update({"closed": fields["closed"], "points": [[x, y] for x, y, _ in fields["points"]]})
        else:
            row.update({"text": fields["text"], "at": [fields["insert"][0], fields["insert"][1]], "height": fields["height"]})
        entities.append(row)
    print(
        json.dumps(
            {
                "version": document.version,
                "units": document.units,
                "insunits": header.get("insunits"),
                "extmin": header.get("extmin"),
                "extmax": header.get("extmax"),
                "layers": {name: {"colorIndex": entry["color_index"], "lineweight": entry["lineweight"], "off": entry["off"], "frozen": entry["frozen"]} for name, entry in layers.items()},
                "layouts": sorted(document.layouts().keys()),
                "totalEntities": sum(1 for _ in model.iter_entities()),
                "entities": entities,
            }
        )
    )


main(sys.argv[1])
