import json,sys
from collections import deque

fixture=json.load(sys.stdin)

def conflict(left,right):
    return left=="exclusive" or right=="exclusive"

def explore(processes):
    count=len(processes)
    resources=sorted({step["resource"] for process in processes for step in process["steps"]})
    initial=(tuple(0 for _ in processes),tuple(() for _ in processes),tuple(() for _ in resources))
    seen={initial}
    queue=deque([initial])
    deadlocks=0
    states=0
    while queue:
        index,holds,waits=queue.popleft()
        states+=1
        moves=[]
        for who,process in enumerate(processes):
            position=index[who]
            if position>=len(process["steps"]):
                continue
            step=process["steps"][position]
            slot=resources.index(step["resource"])
            queued=[entry for entry in waits[slot] if entry[0]==who]
            held=[(other,mode) for other in range(count) if other!=who for resource,mode in holds[other] if resource==step["resource"]]
            if not queued:
                new_waits=list(waits)
                new_waits[slot]=waits[slot]+((who,step["mode"]),)
                moves.append((index,holds,tuple(new_waits)))
                continue
            earlier=[entry for entry in waits[slot][:waits[slot].index(queued[0])] if conflict(entry[1],step["mode"])]
            if earlier or any(conflict(mode,step["mode"]) for _,mode in held):
                continue
            new_waits=list(waits)
            new_waits[slot]=tuple(entry for entry in waits[slot] if entry[0]!=who)
            new_holds=list(holds)
            new_holds[who]=holds[who]+((step["resource"],step["mode"]),)
            new_index=list(index)
            new_index[who]=position+1
            moves.append((tuple(new_index),tuple(new_holds),tuple(new_waits)))
        for who,process in enumerate(processes):
            if index[who]==len(process["steps"]) and holds[who]:
                new_holds=list(holds)
                new_holds[who]=()
                moves.append((index,tuple(new_holds),waits))
        if not moves:
            if any(index[who]<len(process["steps"]) for who,process in enumerate(processes)):
                deadlocks+=1
            continue
        for move in moves:
            if move not in seen:
                seen.add(move)
                queue.append(move)
    return {"states":states,"deadlock":deadlocks>0}

print(json.dumps({row["id"]:explore(row["processes"]) for row in fixture["scenarios"]}))
