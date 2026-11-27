import json, sys

def gate(row, threshold):
    if row["mode"] == "never": return (False, False, "never")
    if row["mode"] == "pin": return (True, True, "pin")
    if row.get("fleetRequired"): return (True, True, "fleet-required")
    if row["expensive"]:
        if row["fits"]: return (True, True, "final")
        return (True, True, "final-beyond-host") if row.get("beyondHost") else (False, False, "final-wait")
    pressured = row["slotNow"] and row["host"] == "wait"
    if row["slotNow"] and not pressured: return (False, False, "local-slot")
    if row["worth"] == "cheap": return (False, False, "cheap")
    if row["worth"] == "unmeasured" and not row["slotNow"] and row["slotWait"]: return (False, False, "unmeasured")
    if row["door"]: return (True, True, "door")
    if row["slotQueueMilliseconds"] >= threshold: return (True, True, "slot-queue")
    if row["host"] == "wait": return (True, True, "host-wait")
    if row["broker"] == "wait": return (True, True, "broker-wait")
    return (True, False, "headroom")

request = json.load(sys.stdin)
print(json.dumps([list(gate(row, request["threshold"])) for row in request["rows"]]))
