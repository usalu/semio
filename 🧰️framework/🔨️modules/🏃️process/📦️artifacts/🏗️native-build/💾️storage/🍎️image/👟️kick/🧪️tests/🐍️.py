"""👟️ Independent oracle for the in-place kick: decides the due cases of the shared corpus and measures the host allocation of a file with the standard library only."""
import json,os,sys
def reclaim_due(rule,allocated,used,free):
    tight=free<rule["reclaimTightHostFreeBytes"]
    floor=rule["reclaimFloorBytes"]//8 if tight else rule["reclaimFloorBytes"]
    return allocated-used>=floor and (tight or allocated*100>=used*rule["reclaimRatioPercent"])
if sys.argv[1]=="due":
    corpus=json.load(open(sys.argv[2]))
    interval=corpus["minimumIntervalMilliseconds"]
    print(json.dumps([c["sinceLastKickMilliseconds"]>=interval and reclaim_due(corpus["rule"],c["allocatedBytes"],c["usedBytes"],c["hostFreeBytes"]) for c in corpus["cases"]]))
elif sys.argv[1]=="allocated":
    print(os.stat(sys.argv[2]).st_blocks*512)
