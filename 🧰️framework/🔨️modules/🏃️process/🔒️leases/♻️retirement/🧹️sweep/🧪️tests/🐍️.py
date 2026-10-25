"""🐍️ Independent oracle of the lease sweep: predicts retirement from physical facts with Python's own sqlite3 and replays acquire/release logs for mutual exclusion."""
import hashlib, json, os, sqlite3, sys


def digest(text):
    return hashlib.sha256(text.encode()).hexdigest()


def locked(path):
    if not os.path.isfile(path):
        return False
    connection = sqlite3.connect("file:" + path + "?mode=rw", uri=True, timeout=0)
    connection.isolation_level = None
    try:
        connection.execute("BEGIN EXCLUSIVE")
        connection.execute("ROLLBACK")
        return False
    except sqlite3.OperationalError as error:
        if "locked" in str(error) or "busy" in str(error):
            return True
        raise
    finally:
        connection.close()


def predict(spec):
    result = {}
    for row in spec["cases"]:
        base = os.path.join(spec["directory"], digest(row["resource"]))
        database, queue = base + ".sqlite", base + ".queue"
        has_database = os.path.isfile(database)
        has_queue = os.path.lexists(queue)
        plain = has_queue and os.path.isdir(queue) and not os.path.islink(queue)
        foreign = has_queue and not plain
        tickets = sorted(os.listdir(queue)) if plain else []
        live = [name for name in tickets if locked(os.path.join(spec["directory"], "claims", digest("queued-original:" + digest(row["resource"]) + ":" + name) + ".sqlite"))]
        dead = [name for name in tickets if name not in live]
        held = has_database and locked(database)
        age_database = spec["now"] - os.stat(database).st_mtime * 1000 if has_database else float("inf")
        age_queue = spec["now"] - os.lstat(queue).st_mtime * 1000 if has_queue else float("inf")
        reaped = bool(dead) and not foreign
        used = age_database < spec["idle"] or (has_queue and age_queue < spec["idle"]) or reaped
        database_retired = has_database and not held and not live and not foreign and not used
        orphan = not has_database and plain
        queue_retired = plain and not live and not used and (database_retired or orphan)
        eventually = (has_database and not held and not live and not foreign) or (orphan and not live)
        result[row["name"]] = {"databaseRetired": bool(database_retired), "queueRetired": bool(queue_retired), "ticketsReaped": bool(reaped), "eventuallyRetired": bool(eventually)}
    return result


def replay(events):
    active, violations = {}, []
    for index, event in sorted(enumerate(events), key=lambda pair: (int(pair[1]["t"]), pair[0])):
        holders = active.setdefault(event["r"], {})
        if event["e"] == "acq":
            if (event["m"] == "exclusive" and holders) or any(mode == "exclusive" for mode in holders.values()):
                violations.append({"index": index, "event": event, "holders": dict(holders)})
            holders[event["w"]] = event["m"]
        else:
            holders.pop(event["w"], None)
    return {"violations": violations, "events": len(events)}


if __name__ == "__main__":
    mode, path = sys.argv[1], sys.argv[2]
    with open(path, encoding="utf-8") as handle:
        document = json.load(handle)
    print(json.dumps(predict(document) if mode == "predict" else replay(document)))
