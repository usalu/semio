"""Independent model of the generation pin of a nested build command.

Commands read one JSON document on stdin and print one JSON document:
  encode {"owner","generation","manifest","owners":[...],"covered":[...]|null,"atMs"} -> {"text": canonical pin text}
  read   {"text": any}                                                              -> {"accepted": bool}
  state  {"environment": {...}, "alive": [pid,...], "self": pid, "now": ms}                               -> {"state": "none"|"bound"|"foreign"|"refused"}
  open   {"owners": [...], "covered": [...], "needed": [...]|null}                  -> {"open": [...]}
  owner  {"environment": {...}, "pid": n}                                           -> {"owner": n}
Standard library only; names are ordered by code point like the store (Python str order).
"""
import base64
import binascii
import json
import re
import sys

BASE64 = re.compile(r"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$")
HEX = re.compile(r"^[a-f0-9]{64}$")
MAXIMUM_BYTES = 32768
MAXIMUM_OWNERS = 65536
MAXIMUM_AGE_MILLISECONDS = 14400000
ENVIRONMENT = "SEMIO_BUILD_GENERATION_PIN"
INVOCATION = "SEMIO_BUILD_INVOCATION_PID"
CHAIN = "SEMIO_BUILD_INVOCATION_PIDS"


def is_int(value):
    return isinstance(value, int) and not isinstance(value, bool)


def bitmap(owners, covered):
    covered = set(covered)
    out = bytearray((len(owners) + 7) // 8)
    for at, name in enumerate(owners):
        if name in covered:
            out[at // 8] |= 1 << (at % 8)
    return out


def encode(request):
    owners = request["owners"]
    covered = owners if request["covered"] is None else request["covered"]
    raw = bitmap(owners, covered)
    pin = {
        "version": 1,
        "owner": request["owner"],
        "generation": request["generation"],
        "manifest": request["manifest"],
        "covered": {"atMs": request["atMs"], "count": len(owners), "owners": base64.b64encode(bytes(raw)).decode("ascii")},
    }
    return {"text": json.dumps(pin, separators=(",", ":"), ensure_ascii=False)}


def stamp_accepted(stamp):
    if not isinstance(stamp, dict) or set(stamp) != {"atMs", "count", "owners"}:
        return False
    at, count, owners = stamp["atMs"], stamp["count"], stamp["owners"]
    if not is_int(at) or not is_int(count) or at < 0 or count < 0 or count > MAXIMUM_OWNERS or at > 9007199254740991:
        return False
    if not isinstance(owners, str) or not BASE64.match(owners):
        return False
    try:
        raw = base64.b64decode(owners, validate=True)
    except (binascii.Error, ValueError):
        return False
    if len(raw) != (count + 7) // 8 or base64.b64encode(raw).decode("ascii") != owners:
        return False
    return not (count % 8 and raw[-1] >> (count % 8))


def read(text):
    if not isinstance(text, str) or len(text.encode("utf-8")) > MAXIMUM_BYTES:
        return None
    try:
        value = json.loads(text)
    except ValueError:
        return None
    if not isinstance(value, dict) or set(value) != {"version", "owner", "generation", "manifest", "covered"}:
        return None
    if value["version"] != 1 or not is_int(value["version"]):
        return None
    if not is_int(value["owner"]) or value["owner"] < 1 or value["owner"] > 9007199254740991:
        return None
    if not is_int(value["generation"]) or value["generation"] < 0 or value["generation"] > 9007199254740991:
        return None
    if not isinstance(value["manifest"], str) or not HEX.match(value["manifest"]):
        return None
    if not stamp_accepted(value["covered"]):
        return None
    return value


def number(value):
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def state(request):
    environment, alive = request["environment"], set(request["alive"])
    text = environment.get(ENVIRONMENT)
    if text is None or text == "":
        return {"state": "none"}
    pin = read(text)
    if pin is None:
        return {"state": "refused"}
    owners = [number(environment.get(INVOCATION))] + [number(part) for part in (environment.get(CHAIN) or "").split(",")]
    young = request["now"] - pin["covered"]["atMs"] <= MAXIMUM_AGE_MILLISECONDS
    return {"state": "bound" if pin["owner"] != request.get("self") and young and pin["owner"] in owners and pin["owner"] in alive else "foreign"}


def open_owners(request):
    covered = set(request["covered"])
    needed = request["owners"] if request["needed"] is None else request["needed"]
    return {"open": sorted(name for name in needed if name not in covered)}


def owner(request):
    value = number(request["environment"].get(INVOCATION))
    return {"owner": value if value is not None and value > 0 else request["pid"]}


def main():
    command = sys.argv[1]
    request = json.load(sys.stdin)
    if command == "encode":
        result = encode(request)
    elif command == "read":
        result = {"accepted": read(request["text"]) is not None}
    elif command == "state":
        result = state(request)
    elif command == "open":
        result = open_owners(request)
    elif command == "owner":
        result = owner(request)
    else:
        raise SystemExit("unknown command")
    print(json.dumps(result, ensure_ascii=False))


main()
