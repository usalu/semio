"""🔢️ Independent Python oracle of the front-end thread bound (standard library only): for each case, the arguments rustc must receive when the run was admitted `tokens` parallel jobs. A `threads` count of 0 becomes `tokens`, a larger count is capped to it, a smaller or non-numeric one stays; both spellings (`-Z threads=N`, `-Zthreads=N`) are read; without a grant (no tokens, or 0) nothing changes. usage: 🐍️.py <corpus.json> prints the expected arguments of every case as JSON."""
import json, re, sys


def bounded(args, tokens):
    if not tokens or tokens < 1:
        return list(args)
    def fix(value):
        return str(tokens) if re.fullmatch(r"[0-9]+", value) and (int(value) == 0 or int(value) > tokens) else value
    out, i = [], 0
    while i < len(args):
        arg = args[i]
        if arg == "-Z" and i + 1 < len(args) and args[i + 1].startswith("threads="):
            out += [arg, "threads=" + fix(args[i + 1][len("threads="):])]
            i += 2
            continue
        if arg.startswith("-Zthreads="):
            out.append("-Zthreads=" + fix(arg[len("-Zthreads="):]))
        else:
            out.append(arg)
        i += 1
    return out


if __name__ == "__main__":
    print(json.dumps([bounded(c["args"], c.get("tokens")) for c in json.load(open(sys.argv[1], encoding="utf-8"))["cases"]], ensure_ascii=False))
