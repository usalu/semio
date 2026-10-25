import json,sys
from graphlib import CycleError,TopologicalSorter

def conflict(left,right):
    return left=="exclusive" or right=="exclusive"

def has_cycle(nodes):
    by={row["id"]:row for row in nodes}
    sorter=TopologicalSorter()
    for row in nodes:
        waits=set()
        wait=row.get("wait")
        if wait:
            for other in nodes:
                if other["id"]==row["id"]:continue
                if any(hold["key"]==wait["key"] and conflict(hold["mode"],wait["mode"]) for hold in other["holds"]):
                    waits.add(other["id"])
                other_wait=other.get("wait")
                if other_wait and other_wait["key"]==wait["key"] and other_wait["queued"] and wait["queued"] and (other_wait["sinceMs"]<wait["sinceMs"] or other_wait["sinceMs"]==wait["sinceMs"] and other["id"]<row["id"]) and conflict(other_wait["mode"],wait["mode"]):
                    waits.add(other["id"])
        if row["holds"]:
            for other in nodes:
                if other["id"]!=row["id"] and other.get("wait") and row["id"] in other.get("ancestors",[]):
                    waits.add(other["id"])
        sorter.add(row["id"],*sorted(waits))
    try:
        sorter.prepare()
        return False
    except CycleError:
        return True

print(json.dumps({graph["id"]:has_cycle(graph["nodes"]) for graph in json.load(sys.stdin)["graphs"]}))
