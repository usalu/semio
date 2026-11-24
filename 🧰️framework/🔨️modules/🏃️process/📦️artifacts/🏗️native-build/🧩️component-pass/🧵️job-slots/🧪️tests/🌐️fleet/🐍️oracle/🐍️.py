"""🌐 Independent model of the fleet slot admission of one build.

Reads for every case on stdin (a JSON list of hosts) whether one more component pass may start on a fleet slot: no more
than `maximum` fleet jobs, a worker slot must be free (none answered = the fleet is silent), critical memory pressure
refuses, and the footprint must fit the free memory above the hard floor once the footprints already held are charged
(unreadable memory admits). Raised, not critical, pressure never refuses. It shares no code with the TypeScript
implementation.
"""
import json
import sys


def decide(host):
    free = 0 if host["availableMegabytes"] is None else max(-4194304, host["availableMegabytes"] - host["floorMegabytes"] - host["reservedMegabytes"])
    if host["running"] >= host["maximum"]:
        reason = "fleet-cap"
    elif host["slots"] is None:
        reason = "fleet-none"
    elif host["slots"] <= 0:
        reason = "fleet-busy"
    elif host["pressure"] == "critical":
        reason = "fleet-pressure"
    elif host["availableMegabytes"] is not None and host["footprintMegabytes"] > free:
        reason = "fleet-memory"
    else:
        reason = "fleet-fits"
    return {"admit": reason == "fleet-fits", "reason": reason, "freeMegabytes": free}


if __name__ == "__main__":
    json.dump([decide(host) for host in json.load(sys.stdin)], sys.stdout)
