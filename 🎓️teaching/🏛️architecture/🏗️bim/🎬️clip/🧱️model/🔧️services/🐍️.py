"""🛠️ Source-evidence building services graphs independent of unregistered metric pipe routes."""
from copy import deepcopy

def validated(graph: dict, products: set) -> dict:
    """🧾 Validate source-node ownership and logical edges without assigning concealed coordinates."""
    if graph.get("kind")!="source_evidence_graph":raise ValueError("Expected a source-evidence service graph")
    nodes={n["id"]:n for n in graph["nodes"]}
    if len(nodes)!=len(graph["nodes"]):raise ValueError("Service node identities must be unique")
    for node in nodes.values():
        if any(tag not in products for tag in node["products"]):raise ValueError("Service nodes must reference existing native products")
        if (node["kind"]=="boundary")==bool(node["products"]):raise ValueError("Only physical/outlet nodes own products; boundary locations stay unknown")
    for edge in graph["edges"]:
        if edge["from"] not in nodes or edge["to"] not in nodes or edge["from"]==edge["to"]:raise ValueError("A service edge needs two distinct existing nodes")
        if any(nodes[edge[k]]["system"]!=edge["system"] for k in ("from","to")):raise ValueError("A service edge must remain within its source system")
    return deepcopy(graph)

def reachable(graph: dict, start: str) -> list:
    """🔗 Return logical source reachability; this does not establish physical or hydraulic continuity."""
    ids={n["id"] for n in graph["nodes"]}
    if start not in ids:raise ValueError("Unknown service graph node: "+start)
    adjacent={key:[] for key in ids}
    for edge in graph["edges"]:adjacent[edge["from"]].append(edge["to"])
    seen={start};pending=[start]
    for node in pending:
        for peer in adjacent[node]:
            if peer not in seen:seen.add(peer);pending.append(peer)
    return sorted(seen)
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
