import errno,json,sys

def transient(number):
    return number == errno.EINTR or number not in errno.errorcode

def settle(steps, attempts):
    previous = 0
    for attempt in range(1, attempts + 1):
        step = steps[min(attempt - 1, len(steps) - 1)]
        if step == "ok":
            return {"status": 0, "number": 0, "reads": 0}
        if not transient(step):
            if step == previous:
                return {"status": -1, "number": step, "reads": 2}
            previous = step
        if attempt >= attempts:
            return {"status": -1, "number": step, "reads": 0 if transient(step) else 1}

print(json.dumps([settle(case["steps"], case["attempts"]) for case in json.load(sys.stdin)]))
