"""Independent oracle of the Node native corpus: the same fixture is executed with Python ctypes, a foreign function interface that shares no code with Bun's or with ours. Usage: python3 -I this.py <fixture.json> <scratch-directory>"""
import ctypes
import json
import math
import os
import platform
import struct
import sys

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
scratch = sys.argv[2]
for key, value in fixture["environment"].items():
    os.environ[key] = value
    ctypes.CDLL(None).setenv(key.encode(), value.encode(), 1)

libc = ctypes.CDLL(None, use_errno=True)
darwin = sys.platform == "darwin"
integer = {"i8": ("b", 8, True), "u8": ("B", 8, False), "i16": ("h", 16, True), "u16": ("H", 16, False), "i32": ("i", 32, True), "u32": ("I", 32, False), "i64": ("q", 64, True), "u64": ("Q", 64, False)}
restypes = {"i8": ctypes.c_int8, "u8": ctypes.c_uint8, "i16": ctypes.c_int16, "u16": ctypes.c_uint16, "i32": ctypes.c_int32, "u32": ctypes.c_uint32, "i64": ctypes.c_int64, "u64": ctypes.c_uint64, "bool": ctypes.c_bool, "ptr": ctypes.c_void_p, "cstring": ctypes.c_void_p, "void": None}


def wrap(value, bits, signed):
    value &= (1 << bits) - 1
    return value - (1 << bits) if signed and value >= 1 << (bits - 1) else value


def make_buffer(spec):
    raw = bytes.fromhex(spec["bytes"])
    buffer = ctypes.create_string_buffer(raw, len(raw))
    return buffer, ctypes.addressof(buffer) + spec.get("offset", 0)


def run_call(spec):
    buffers, ranges, keep = {}, [], []
    for row in spec.get("buffers", []):
        buffer, address = make_buffer(row)
        buffers[row["id"]] = (buffer, address)
        ranges.append((row["id"], address, len(buffer)))
    words = []
    for type_name, value in spec["args"]:
        if type_name in integer:
            _, bits, signed = integer[type_name]
            if value is None:
                number = 0
            elif isinstance(value, bool):
                number = int(value)
            elif isinstance(value, float):
                number = math.trunc(value)
            elif isinstance(value, dict):
                number = int(value["bigint"])
            else:
                number = value
            words.append(wrap(number, bits, signed))
        else:
            if value is None:
                words.append(None)
            elif isinstance(value, dict) and "string" in value:
                buffer = ctypes.create_string_buffer(value["string"].encode("utf-8"))
                keep.append(buffer)
                words.append(ctypes.addressof(buffer))
            elif isinstance(value, dict) and "bytes" in value:
                buffer, address = make_buffer(value)
                keep.append(buffer)
                words.append(address)
            elif isinstance(value, dict) and "ref" in value:
                words.append(buffers[value["ref"]][1])
            elif isinstance(value, dict) and "pointer" in value:
                words.append(buffers[value["pointer"]][1] + value.get("offset", 0))
            else:
                raise ValueError(value)
    function = getattr(libc, spec["symbol"])
    function.restype = restypes[spec["returns"]]
    function.argtypes = [restypes[t] if t not in ("cstring",) else ctypes.c_void_p for t, _ in spec["args"]]
    ctypes.set_errno(0)
    result = function(*words)
    errno = ctypes.get_errno()
    kind = spec["returns"]
    if kind == "void":
        encoded = {"void": True}
    elif kind in ("i64", "u64"):
        encoded = {"bigint": str(result)}
    elif kind == "cstring":
        encoded = None if result is None else {"string": ctypes.string_at(result).decode("utf-8")}
    elif kind == "ptr":
        encoded = result
        if result is not None:
            for identifier, start, length in ranges:
                if start <= result < start + max(length, 1):
                    encoded = {"pointer": identifier, "offset": result - start}
                    break
    else:
        encoded = result
    row = {"value": encoded}
    if "errno" in spec:
        row["errno"] = errno
    if "after" in spec:
        row["after"] = {identifier: bytes((ctypes.c_ubyte * len(buffers[identifier][0])).from_buffer(buffers[identifier][0])).hex() for identifier in spec["after"]}
    return row


def read_memory(spec):
    raw = bytes.fromhex(spec["bytes"])
    codes = {"i8": "b", "u8": "B", "i16": "h", "u16": "H", "i32": "i", "u32": "I", "i64": "q", "u64": "Q", "ptr": "Q"}
    out = []
    for type_name, offset, _ in spec["reads"]:
        value = struct.unpack_from("<" + codes[type_name], raw, offset)[0]
        out.append({"bigint": str(value)} if type_name in ("i64", "u64") else float(value) if type_name == "ptr" else value)
    return out


def exchange(first, second):
    at = -2 if darwin else -100
    function = libc.renameatx_np if darwin else libc.renameat2
    function.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
    function.restype = ctypes.c_int
    ctypes.set_errno(0)
    status = function(at, first.encode(), at, second.encode(), 2)
    return status, ctypes.get_errno()


def host():
    out = {}
    root = os.path.join(scratch, "swap")
    os.makedirs(os.path.join(root, "a"), exist_ok=True)
    os.makedirs(os.path.join(root, "b"), exist_ok=True)
    open(os.path.join(root, "a", "x"), "w").write("A")
    open(os.path.join(root, "b", "y"), "w").write("B")
    swapped, _ = exchange(os.path.join(root, "a"), os.path.join(root, "b"))
    missing, errno = exchange(os.path.join(root, "missing-one"), os.path.join(root, "missing-two"))
    out["rename-swap"] = {"swapped": swapped, "namesA": sorted(os.listdir(os.path.join(root, "a"))), "namesB": sorted(os.listdir(os.path.join(root, "b"))), "missing": {"status": missing, "errno": errno}}
    stat = os.statvfs(scratch)
    out["statfs"] = {"status": 0, "totalBytes": stat.f_blocks * stat.f_frsize, "bavailPositive": stat.f_bavail > 0}
    case_root = os.path.join(scratch, "case")
    os.makedirs(case_root, exist_ok=True)
    open(os.path.join(case_root, "Probe-Case.TXT"), "w").write("x")
    out["case-sensitivity"] = {"exact": 0 if os.access(os.path.join(case_root, "Probe-Case.TXT"), os.F_OK) else -1, "folded": 0 if os.access(os.path.join(case_root, "probe-case.txt"), os.F_OK) else -1}
    out["process-tree"] = {"pid": os.getpid(), "ppid": os.getppid(), "pgid": os.getpgid(0), "uid": os.getuid()}
    return out


result = {"calls": {spec["id"]: run_call(spec) for spec in fixture["calls"]}, "memory": {spec["id"]: read_memory(spec) for spec in fixture["memory"]}, "host": host(), "python": platform.python_version()}
sys.stdout.write(json.dumps(result))
