"""🪵️ Independent Python oracle of the failed action output tail (standard library only): the last N bytes of all lines joined with newlines, advanced past UTF-8 continuation bytes, decoded; cases arrive as JSON on stdin."""
import json, sys


def tail(maximum, lines):
    data = b"".join(line.encode("utf-8") + b"\n" for line in lines)
    cut = max(0, len(data) - maximum)
    while cut < len(data) and data[cut] & 0xC0 == 0x80:
        cut += 1
    kept = data[cut:]
    return {"text": kept.decode("utf-8"), "bytes": len(kept), "droppedBytes": cut}


def main():
    cases = json.load(sys.stdin)
    out = []
    for case in cases:
        lines = case.get("lines")
        if lines is None:
            generate = case["generate"]
            lines = [generate["template"].replace("{n}", str(n)) for n in range(generate["lines"])]
        out.append(tail(case["maximumBytes"], lines))
    sys.stdout.buffer.write(json.dumps(out, ensure_ascii=False).encode("utf-8"))


main()
