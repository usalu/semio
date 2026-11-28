"""🧮️ Independent oracle of the host-wide image commitment budget: the safety reserve, the ceiling, the admission of a request and the 32 GiB layout invariant, written from the policy text with the standard library only."""
import json,sys
GIB=1<<30
GROWTH_RESERVE=4*GIB;RESERVE_FLOOR=16*GIB;RESERVE_PERCENT=8;OVERCOMMIT=300;ALIGN=GIB
CAPS={"darwin":32*GIB,"linux":128*GIB,"win32":64*GIB}
def reserve(capacity):
    return max(RESERVE_FLOOR,-(-(capacity*RESERVE_PERCENT//100)//ALIGN)*ALIGN)
def ceiling(capacity,free,allocated):
    r=reserve(capacity)
    hard=capacity-r
    soft=allocated+max(0,free-r)*OVERCOMMIT//100
    return max(0,min(hard,soft))//ALIGN*ALIGN
def committed(rows):
    return sum(min(r["capacityBytes"],r["allocatedBytes"]+GROWTH_RESERVE) for r in rows)
def admission(ceil,committed,request):
    room=max(0,ceil-committed);short=max(0,committed+request-ceil)
    return {"admitted":short==0,"roomBytes":room,"shortfallBytes":short}
def cost(declared):
    return min(declared,GROWTH_RESERVE)
def declarable(room):
    return None if room>=GROWTH_RESERVE else max(0,room)//ALIGN*ALIGN
def answers(path):
    d=json.load(open(path))
    return {
        "ceiling":[{"reserveBytes":reserve(r["hostCapacityBytes"]),"ceilingBytes":ceiling(r["hostCapacityBytes"],r["hostFreeBytes"],r["allocatedBytes"])} for r in d["ceiling"]],
        "admission":[admission(r["ceilingBytes"],r["committedBytes"],r["requestBytes"]) for r in d["admission"]],
        "sum":[committed(r["rows"]) for r in d["sum"]],
        "layout":[r["capacityBytes"]<=CAPS[r["platform"]] for r in d["layout"]],
        "cost":[cost(r["declaredBytes"]) for r in d["cost"]],
        "declarable":[declarable(r["roomBytes"]) for r in d["declarable"]],
    }
if __name__=="__main__":
    if sys.argv[1]=="answers":print(json.dumps(answers(sys.argv[2])))
    else:raise SystemExit("unknown command")
