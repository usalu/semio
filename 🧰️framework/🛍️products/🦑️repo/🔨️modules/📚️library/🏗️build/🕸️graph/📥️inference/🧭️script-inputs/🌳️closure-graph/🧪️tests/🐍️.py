#!/usr/bin/env python3
"""🐍️ Independent oracle of the module closure graph: reads {"nodes":[{"key","name","kind","content","edges"}],"owners":[{"name","roots","stops":{key:tool},"redirect":{key:key}}]} and prints, per owner, the closure digest, the reached names and tools and keys, and the node digests from the canonical JSON of each content. It uses no Tarjan: plain recursive reachability per node (the stops cut to tool entries unless they are roots), the component of a node is the set of nodes it reaches that reach it back, and the component digests are computed by recursion over the components.
usage: 🐍️.py < graph"""
import hashlib, json, sys

sys.setrecursionlimit(100000)
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
data = json.load(sys.stdin)
nodes = {node["key"]: node for node in data["nodes"]}
digest = {key: sha(json.dumps(node["content"], sort_keys=True, separators=(",", ":"), ensure_ascii=False)) for key, node in nodes.items()}
entry = {key: sha(nodes[key]["name"] + "\0" + digest[key]) for key in nodes}
tool_entry = lambda name: sha("\0tool\0" + name)


def reach(key, stops, seen, tools, redirect={}):
    if key in seen:
        return
    seen.add(key)
    for edge in nodes[key]["edges"]:
        target = redirect.get(edge, edge)
        if target in stops:
            tools.add(stops[target])
        else:
            reach(target, stops, seen, tools, redirect)


def closure(stops, redirect):
    reached = {}
    for key in nodes:
        seen = set()
        reach(key, stops, seen, set(), redirect)
        reached[key] = seen
    memo = {}

    def component(key):
        return frozenset(other for other in reached[key] if key in reached[other])

    def component_digest(members):
        if members in memo:
            return memo[members]
        below = set()
        for member in members:
            for edge in nodes[member]["edges"]:
                target = redirect.get(edge, edge)
                if target in stops:
                    below.add(tool_entry(stops[target]))
                elif target not in members:
                    below.add(component_digest(component(target)))
        memo[members] = sha("\n".join(sorted(entry[member] for member in members)) + "\n\n" + "\n".join(sorted(below)))
        return memo[members]

    return lambda roots: sha("\n".join(sorted({component_digest(component(root)) for root in roots})))


families, out = {}, []
for owner in data["owners"]:
    redirect = owner.get("redirect", {})
    stops = {key: tool for key, tool in owner.get("stops", {}).items() if key not in owner["roots"] and key not in redirect.values()}
    family = json.dumps([sorted(stops.items()), sorted(redirect.items())])
    if family not in families:
        families[family] = closure(stops, redirect)
    seen, tools = set(), set()
    for root in owner["roots"]:
        reach(root, stops, seen, tools, redirect)
    out.append({"name": owner["name"], "digest": families[family](owner["roots"]), "names": sorted(nodes[key]["name"] for key in seen), "tools": sorted(tools), "keys": sorted(seen)})
print(json.dumps({"owners": out, "digests": digest}, ensure_ascii=False))
