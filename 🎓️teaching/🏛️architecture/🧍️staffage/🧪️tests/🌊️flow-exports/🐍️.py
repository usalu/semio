"""🌊️ Oracles of the flow exports. `svg <file>`: every path of an SVG in document order with its box in the user units of the root, millimetres (svgelements, whose pixels are divided by the pixels per user unit of the root), its own or its group's id, its layer group and its paint (xml.etree), the texts and the scene metadata. `travel <json>`: the travel time of points on polylines with times at their vertices (numpy: nearest point on the polyline, linear in between) and the distance to it. `aci <index…>`: the RGB of AutoCAD colour indices (ezdxf). `frames <zip>`: the manifest of a PNG sequence and every PNG member with its size from the IHDR chunk and its SHA-256 (zipfile, with its CRC test)."""
import json
import sys
import xml.etree.ElementTree as ET

SVG = "{http://www.w3.org/2000/svg}"
INKSCAPE = "{http://www.inkscape.org/namespaces/inkscape}"


def svg(path):
    from svgelements import SVG as Document, Path

    root = ET.parse(path).getroot()
    parents = {child: parent for parent in root.iter() for child in parent}

    def ancestors(node):
        while node in parents:
            node = parents[node]
            yield node

    def layer(node):
        return next((a.get("id") for a in ancestors(node) if a.get(INKSCAPE + "groupmode") == "layer"), None)

    def name(node):
        return node.get("id") or next((a.get("id") for a in ancestors(node) if a.tag == SVG + "g" and a.get("id") and a.get(INKSCAPE + "groupmode") != "layer"), None)

    def paint(node, key):
        return next((n.get(key) for n in (node, *ancestors(node)) if n.get(key) is not None), None)

    drawn = [node for node in root.iter(SVG + "path") if not any(a.tag in (SVG + "defs", SVG + "pattern", SVG + "clipPath") for a in ancestors(node))]
    shapes = [e for e in Document.parse(path, reify=True).elements() if isinstance(e, Path)]
    if len(drawn) != len(shapes):
        raise SystemExit(f"{len(drawn)} path elements, svgelements reads {len(shapes)} paths")
    unit = float(root.get("width").removesuffix("mm")) * 96 / 25.4 / float(root.get("viewBox").split()[2])
    items = []
    for node, shape in zip(drawn, shapes):
        box = shape.bbox()
        items.append({"id": name(node), "layer": layer(node), "box": None if box is None else [value / unit for value in box], "stroke": paint(node, "stroke"), "fill": paint(node, "fill"), "strokeWidth": paint(node, "stroke-width"), "strokeOpacity": paint(node, "stroke-opacity"), "fillOpacity": paint(node, "fill-opacity"), "dash": paint(node, "stroke-dasharray")})
    texts = [{"id": name(node), "layer": layer(node), "text": "".join(node.itertext())} for node in root.iter(SVG + "text")]
    meta = {"scene": None, "views": []}
    for node in root.iter():
        if node.tag.endswith("}scene"):
            meta["scene"] = dict(node.attrib)
        if node.tag.endswith("}view") and "viewpoint" in node.attrib:
            meta["views"].append(dict(node.attrib))
    return {"width": root.get("width"), "height": root.get("height"), "items": items, "texts": texts, **meta}


def travel(path):
    import numpy as np

    task = json.load(open(path, encoding="utf8"))
    out = []
    for line in task["lines"]:
        p = np.asarray(line["points"], dtype=float)
        t = np.asarray(line["travelS"], dtype=float)
        a, b = p[:-1], p[1:]
        d = b - a
        l2 = np.maximum((d * d).sum(axis=1), 1e-300)
        found = []
        for q in np.asarray(line["at"], dtype=float).reshape(-1, 2):
            u = np.clip(((q - a) * d).sum(axis=1) / l2, 0.0, 1.0)
            gap = np.linalg.norm(a + u[:, None] * d - q, axis=1)
            i = int(np.argmin(gap))
            found.append({"travelS": float(t[i] + u[i] * (t[i + 1] - t[i])), "distance": float(gap[i])})
        out.append(found)
    return out


def aci(indices):
    from ezdxf import colors

    return {index: list(colors.aci2rgb(int(index))) for index in indices}


def frames(path):
    import hashlib
    import struct
    import zipfile

    with zipfile.ZipFile(path) as archive:
        if archive.testzip() is not None:
            raise SystemExit("corrupt member")
        manifest = json.loads(archive.read("manifest.json"))
        listed = []
        for name in sorted(n for n in archive.namelist() if n.endswith(".png")):
            data = archive.read(name)
            if data[:8] != b"\x89PNG\r\n\x1a\n" or data[12:16] != b"IHDR":
                raise SystemExit(f"{name} is not a PNG")
            width, height = struct.unpack(">II", data[16:24])
            listed.append({"name": name, "width": width, "height": height, "digest": hashlib.sha256(data).hexdigest()})
    return {"manifest": manifest, "frames": listed}


COMMANDS = {"svg": lambda: svg(sys.argv[2]), "travel": lambda: travel(sys.argv[2]), "aci": lambda: aci(sys.argv[2:]), "frames": lambda: frames(sys.argv[2])}
print(json.dumps(COMMANDS[sys.argv[1]]()))
