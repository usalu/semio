"""🐍️ Independent oracle for image reclaim: decides the cases of the shared corpus and measures the host allocation of a file with the standard library."""
import json,os,sys
def due(rule,allocated,used,free):
    floor=rule["reclaimFloorBytes"]//8 if free<rule["reclaimTightHostFreeBytes"] else rule["reclaimFloorBytes"]
    tight=free<rule["reclaimTightHostFreeBytes"]
    return allocated-used>=floor and (tight or allocated*100>=used*rule["reclaimRatioPercent"])
def effective(volume,container,host):
    if container is None or container-volume<=max(268435456,container//16):
        return volume
    return max(volume,container if host is None else min(container,host))
def real(volume,container,host):
    return min(host,effective(volume,container,host))
def tolerance(container):
    return max(268435456,container//16)
def refresh_due(volume,container,host):
    return container is not None and effective(volume,container,host)-volume>tolerance(container)
def probe_due(volume,host):
    return volume<17179869184 and host-volume>tolerance(host)
if sys.argv[1]=="floor":
    corpus=json.load(open(sys.argv[2]))
    print(json.dumps([[real(a["volumeAvailableBytes"],a.get("containerFreeBytes"),a["hostFreeBytes"]),real(a["volumeAvailableBytes"],a.get("containerFreeBytes"),a["hostFreeBytes"])<a["reserveBytes"]] for a in corpus["floor"]]))
elif sys.argv[1]=="refresh":
    corpus=json.load(open(sys.argv[2]))
    print(json.dumps([[probe_due(a["volumeAvailableBytes"],a["hostFreeBytes"]),refresh_due(a["volumeAvailableBytes"],a.get("containerFreeBytes"),a["hostFreeBytes"])] for a in corpus["refresh"]]))
elif sys.argv[1]=="effective":
    corpus=json.load(open(sys.argv[2]))
    print(json.dumps([effective(a["volumeAvailableBytes"],a.get("containerFreeBytes"),a.get("hostFreeBytes")) for a in corpus["availability"]]))
elif sys.argv[1]=="decide":
    corpus=json.load(open(sys.argv[2]))
    print(json.dumps([due(corpus["rule"],c["allocatedBytes"],c["usedBytes"],c["hostFreeBytes"]) for c in corpus["cases"]]))
else:
    print(os.stat(sys.argv[2]).st_blocks*512)
