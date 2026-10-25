"""Independent model of the verified stamp of a graph generation pointer.

Commands read one JSON document on stdin and print one JSON document:
  scope   {"owners": [...], "covered": [...]}                       -> {"count": n, "owners": base64}
  decode  {"owners": [...], "stamp": {...}}                         -> {"covered": [...]} or {"refused": text}
  merge   {"owners": [...], "scope": [...]|null, "observedAtMs": n, "carried": {"atMs": n, "names": [...]}|null} -> stamp
  cover   {"owners": [...], "stamp": {"atMs": n, "covered": [...]}|null, "implementation": s, "requestedImplementation": s, "requestedAtMs": n, "needed": [...]|null}
          -> {"covered": false} or {"covered": true, "atMs": n, "open": [...]}
  read    {"stamp": any}                                            -> {"accepted": bool}
Standard library only; owners are compared by code point exactly like the store.
"""
import base64
import binascii
import json
import re
import sys

BASE64 = re.compile(r"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$")
MAXIMUM_OWNERS = 65536


def encode(owners, covered):
    covered = set(covered)
    out = bytearray((len(owners) + 7) // 8)
    for at, name in enumerate(owners):
        if name in covered:
            out[at // 8] |= 1 << (at % 8)
    return {"count": len(owners), "owners": base64.b64encode(bytes(out)).decode("ascii")}


def accepts(stamp):
    if not isinstance(stamp, dict) or set(stamp) != {"atMs", "count", "owners"}:
        return False
    at, count, owners = stamp["atMs"], stamp["count"], stamp["owners"]
    for number in (at, count):
        if isinstance(number, bool) or not isinstance(number, int):
            return False
    if at < 0 or count < 0 or count > MAXIMUM_OWNERS or at > 9007199254740991:
        return False
    if not isinstance(owners, str) or not BASE64.match(owners):
        return False
    try:
        raw = base64.b64decode(owners, validate=True)
    except (binascii.Error, ValueError):
        return False
    if len(raw) != (count + 7) // 8 or base64.b64encode(raw).decode("ascii") != owners:
        return False
    if count % 8 and raw[-1] >> (count % 8):
        return False
    return True


def decode(owners, stamp):
    if not accepts(stamp):
        return {"refused": "stamp"}
    if len(owners) != stamp["count"]:
        return {"refused": "owners"}
    raw = base64.b64decode(stamp["owners"])
    return {"covered": [name for at, name in enumerate(owners) if raw[at // 8] >> (at % 8) & 1]}


def merge(owners, scope, observed, carried):
    names = set(owners) if scope is None else set(scope)
    at = observed
    if scope is not None and carried is not None:
        names |= set(carried["names"])
        at = min(carried["atMs"], observed)
    stamp = encode(owners, {name for name in owners if name in names})
    return {"atMs": at, **stamp}


def cover(owners, stamp, implementation, requested_implementation, requested_at, needed):
    if stamp is None or implementation != requested_implementation or stamp["atMs"] < requested_at:
        return {"covered": False}
    wanted = owners if needed is None else needed
    held = set(stamp["covered"])
    return {"covered": True, "atMs": stamp["atMs"], "open": sorted(name for name in wanted if name not in held)}


def main():
    command, document = sys.argv[1], json.load(sys.stdin)
    if command == "scope":
        result = encode(document["owners"], document["covered"])
    elif command == "decode":
        result = decode(document["owners"], document["stamp"])
    elif command == "merge":
        result = merge(document["owners"], document["scope"], document["observedAtMs"], document["carried"])
    elif command == "cover":
        result = cover(document["owners"], document["stamp"], document["implementation"], document["requestedImplementation"], document["requestedAtMs"], document["needed"])
    elif command == "read":
        result = {"accepted": accepts(document["stamp"])}
    else:
        sys.exit("unknown command")
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))


main()
