"""🐍️ Independent model of the bounded least recently used cache (stdlib only, `collections.OrderedDict`): replays operations and prints the counters and the key order.

usage: 🐍️.py replay CORPUS   CORPUS = {"cases": [{"maxEntries", "maxBytes", "operations"}]}; prints one result per case:
{hits, misses, evictions, rejected, bytes, keys}  (keys least recently used first)"""
import json
import sys
from collections import OrderedDict


def replay(case):
    cache, total = OrderedDict(), 0
    hits = misses = evictions = rejected = 0
    for operation in case["operations"]:
        kind, key = operation[0], operation[1]
        if kind == "get":
            if key in cache:
                hits += 1
                cache.move_to_end(key)
            else:
                misses += 1
        elif kind == "delete":
            if key in cache:
                total -= cache.pop(key)
        elif kind == "set":
            size = operation[2]
            if key in cache:
                total -= cache.pop(key)
            if size > case["maxBytes"]:
                rejected += 1
                continue
            cache[key] = size
            total += size
            while len(cache) > case["maxEntries"] or total > case["maxBytes"]:
                _, old = cache.popitem(last=False)
                total -= old
                evictions += 1
    return {"hits": hits, "misses": misses, "evictions": evictions, "rejected": rejected, "bytes": total, "keys": list(cache.keys())}


if __name__ == "__main__":
    if sys.argv[1] != "replay":
        raise SystemExit("unknown command")
    corpus = json.load(open(sys.argv[2], encoding="utf8"))
    print(json.dumps([replay(case) for case in corpus["cases"]], ensure_ascii=False))
