"""Independent model of the binary fact index (see the schema file next to the code): `encode` turns a logical index (JSON on stdin) into the document (hex on stdout), `decode` turns a document (hex on stdin) back into canonical JSON, `strings` lists the table strings. Standard library only."""
import json
import re
import struct
import sys

MAGIC = b"SFI1"
HEX = re.compile(r"^[a-f0-9]{64}$")
MAX_DEPTH = 64
MAX_SAFE = 2**53 - 1
NULL, FALSE, TRUE, UINT, NEGINT, FLOAT, STRING, ARRAY, OBJECT, HEXTAG = range(10)


def norm(value):
    if isinstance(value, float):
        if value != value or value in (float("inf"), float("-inf")):
            return None
        if value.is_integer() and abs(value) <= MAX_SAFE:
            return int(value)
        return value
    if isinstance(value, list):
        return [norm(v) for v in value]
    if isinstance(value, dict):
        return {k: norm(v) for k, v in value.items()}
    return value


def canonical(value):
    return json.dumps(norm(value), sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def varint(value):
    assert 0 <= value <= MAX_SAFE
    out = bytearray()
    while value >= 128:
        out.append(value % 128 | 128)
        value //= 128
    out.append(value)
    return bytes(out)


def zigzag(delta):
    return delta * 2 if delta >= 0 else -delta * 2 - 1


def strings_of(value, into):
    if isinstance(value, str):
        if not HEX.match(value):
            into.add(value)
    elif isinstance(value, list):
        for item in value:
            strings_of(item, into)
    elif isinstance(value, dict):
        for key, item in value.items():
            into.add(key)
            strings_of(item, into)


def encode(index):
    index = norm(index)
    facts, fact_ids, kinds, kind_ids, rows = [], {}, [], {}, []
    table = set(index["owners"])
    for row in index["rows"]:
        kind = row["kind"]
        if kind not in kind_ids:
            kind_ids[kind] = len(kinds)
            kinds.append(kind)
            table.add(kind)
        table.add(row["path"])
        key = canonical(row["fact"])
        if key not in fact_ids:
            fact_ids[key] = len(facts)
            facts.append(row["fact"])
            strings_of(row["fact"], table)
        rows.append((kind_ids[kind], row["path"], fact_ids[key]))
    ordered = sorted(table, key=lambda text: text.encode("utf-8"))
    ids = {text: rank for rank, text in enumerate(ordered)}
    out = bytearray(MAGIC)
    for count in (len(ordered), len(kinds), len(index["owners"]), len(facts), len(rows), len(index["classes"])):
        out += varint(count)
    previous = b""
    for text in ordered:
        data = text.encode("utf-8")
        lcp = 0
        while lcp < min(len(previous), len(data)) and previous[lcp] == data[lcp]:
            lcp += 1
        out += varint(lcp) + varint(len(data) - lcp) + data[lcp:]
        previous = data
    for kind in kinds:
        out += varint(ids[kind])
    for owner in index["owners"]:
        out += varint(ids[owner])

    def value(item, depth, out):
        if item is None:
            out.append(NULL)
        elif item is True:
            out.append(TRUE)
        elif item is False:
            out.append(FALSE)
        elif isinstance(item, int):
            if item >= 0:
                out.append(UINT)
                out += varint(item)
            else:
                out.append(NEGINT)
                out += varint(-item)
        elif isinstance(item, float):
            out.append(FLOAT)
            out += struct.pack("<d", item)
        elif isinstance(item, str):
            if HEX.match(item):
                out.append(HEXTAG)
                out += bytes.fromhex(item)
            else:
                out.append(STRING)
                out += varint(ids[item])
        elif isinstance(item, list):
            assert depth <= MAX_DEPTH
            out.append(ARRAY)
            out += varint(len(item))
            for child in item:
                value(child, depth + 1, out)
        else:
            assert depth <= MAX_DEPTH
            out.append(OBJECT)
            out += varint(len(item))
            for key in sorted(item, key=lambda name: ids[name]):
                out += varint(ids[key])
                value(item[key], depth + 1, out)

    for fact in facts:
        value(fact, 1, out)
    for kind, path, fact in rows:
        out += varint(kind) + varint(ids[path]) + varint(fact)
    for cls in index["classes"]:
        for name in ("r", "o"):
            out += varint(len(cls[name]))
            previous = 0
            for item in cls[name]:
                out += varint(zigzag(item - previous))
                previous = item
    return bytes(out)


class Refusal(Exception):
    pass


def decode(data):
    at = 0

    def need(count):
        if count < 0 or at + count > len(data):
            raise Refusal("truncated")

    def read_varint():
        nonlocal at
        result, scale = 0, 1
        for _ in range(9):
            need(1)
            byte = data[at]
            at += 1
            result += (byte & 127) * scale
            if byte < 128:
                if result > MAX_SAFE:
                    raise Refusal("unsafe")
                return result
            scale *= 128
        raise Refusal("varint")

    def read_count():
        value = read_varint()
        if value > len(data) - at:
            raise Refusal("count")
        return value

    if data[: len(MAGIC)] != MAGIC:
        raise Refusal("magic")
    at = len(MAGIC)
    string_count, kind_count, owner_count, fact_count, row_count, class_count = (read_count() for _ in range(6))
    table, previous = [], b""
    for rank in range(string_count):
        lcp, suffix = read_varint(), read_varint()
        if lcp > len(previous):
            raise Refusal("lcp")
        need(suffix)
        tail = data[at : at + suffix]
        at += suffix
        if rank == 0:
            ok = lcp == 0
        elif lcp == len(previous):
            ok = suffix > 0
        else:
            ok = suffix > 0 and tail[0] > previous[lcp]
        if not ok:
            raise Refusal("order")
        current = previous[:lcp] + tail
        try:
            table.append(current.decode("utf-8"))
        except UnicodeDecodeError:
            raise Refusal("utf8")
        previous = current

    def string():
        ident = read_varint()
        if ident >= string_count:
            raise Refusal("string id")
        return table[ident]

    kinds = [string() for _ in range(kind_count)]
    owners = [string() for _ in range(owner_count)]

    def read_value(depth):
        nonlocal at
        need(1)
        tag = data[at]
        at += 1
        if tag in (ARRAY, OBJECT) and depth > MAX_DEPTH:
            raise Refusal("depth")
        if tag == NULL:
            return None
        if tag == FALSE:
            return False
        if tag == TRUE:
            return True
        if tag == UINT:
            return read_varint()
        if tag == NEGINT:
            magnitude = read_varint()
            if magnitude == 0:
                raise Refusal("negative zero")
            return -magnitude
        if tag == FLOAT:
            need(8)
            number = struct.unpack("<d", data[at : at + 8])[0]
            at += 8
            if number != number or number in (float("inf"), float("-inf")):
                raise Refusal("float")
            return number
        if tag == STRING:
            return string()
        if tag == HEXTAG:
            need(32)
            text = data[at : at + 32].hex()
            at += 32
            return text
        if tag == ARRAY:
            return [read_value(depth + 1) for _ in range(read_count())]
        if tag == OBJECT:
            out, last = {}, -1
            for _ in range(read_count()):
                key = read_varint()
                if key >= string_count or key <= last:
                    raise Refusal("key order")
                last = key
                out[table[key]] = read_value(depth + 1)
            return out
        raise Refusal("tag")

    facts = [read_value(1) for _ in range(fact_count)]
    rows = []
    for _ in range(row_count):
        kind = read_varint()
        if kind >= kind_count:
            raise Refusal("kind")
        path = string()
        fact = read_varint()
        if fact >= fact_count:
            raise Refusal("fact")
        rows.append({"fact": facts[fact], "kind": kinds[kind], "path": path})
    classes = []
    for _ in range(class_count):
        cls = {}
        for name, limit in (("r", row_count), ("o", owner_count)):
            items, previous_id = [], 0
            for _ in range(read_count()):
                zig = read_varint()
                previous_id += -(zig + 1) // 2 if zig % 2 else zig // 2
                if previous_id < 0 or previous_id >= limit:
                    raise Refusal("id range")
                items.append(previous_id)
            cls[name] = items
        classes.append(cls)
    if at != len(data):
        raise Refusal("trailing")
    return {"version": 1, "owners": owners, "rows": rows, "classes": classes}, table


def main():
    command = sys.argv[1]
    if command == "encode":
        sys.stdout.write(encode(json.load(sys.stdin)).hex())
    elif command in ("decode", "strings"):
        try:
            index, table = decode(bytes.fromhex(sys.stdin.read().strip()))
        except Refusal as error:
            sys.stderr.write("refused: %s\n" % error)
            sys.exit(3)
        sys.stdout.write(canonical(index) if command == "decode" else json.dumps(table, ensure_ascii=False))
    else:
        sys.exit(2)


if __name__ == "__main__":
    main()
