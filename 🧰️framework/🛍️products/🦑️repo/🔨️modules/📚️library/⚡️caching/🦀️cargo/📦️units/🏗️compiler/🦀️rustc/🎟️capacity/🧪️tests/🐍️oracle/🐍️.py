"""🎟 Independent Python oracle of the Rust compiler capacity request (standard library only): a profile with the default backend, more than one codegen unit and no link time optimisation is bounded by its codegen units; every other profile asks for the counted compile units of the host. Both are capped by the share of the cores (half, at least one) and start once half of that is free. Requests arrive as JSON on stdin."""
import json, sys


def valid(profile):
    if not {"codegen_backend", "codegen_units", "lto"} <= set(profile):
        return False
    backend, units, lto = profile["codegen_backend"], profile["codegen_units"], profile["lto"]
    if not (backend is None or (isinstance(backend, str) and len(backend) > 0)):
        return False
    if not (units is None or (isinstance(units, int) and not isinstance(units, bool) and 1 <= units <= 2 ** 53 - 1)):
        return False
    return lto in ("false", "off", "thin", "fat")


def share(cores):
    preferred = max(1, cores // 2)
    return preferred, max(1, -(-preferred // 2))


def decide(profile, cores):
    if not valid(profile):
        return {"refused": True}
    bounded = profile["codegen_backend"] is None and profile["codegen_units"] is not None and profile["codegen_units"] > 1 and profile["lto"] in ("false", "off")
    preferred, _ = share(cores)
    maximum = min(profile["codegen_units"], preferred) if bounded else preferred
    return {"resourceClass": "jobserver-codegen" if bounded else "counted-units", "request": {"kind": "available", "maximumParallelism": maximum, "minimumParallelism": max(1, -(-maximum // 2))}}


request = json.load(sys.stdin)
print(json.dumps([decide(p, request["cores"]) for p in request["profiles"]]))
