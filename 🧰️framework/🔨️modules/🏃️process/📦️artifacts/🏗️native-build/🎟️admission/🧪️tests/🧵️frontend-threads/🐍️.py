"""🔢️ Independent Python oracle of the front-end thread bound (standard library only): for each case, the arguments rustc must receive when the run was admitted `tokens` parallel jobs. A `threads` count of 0 becomes `tokens`, a larger count is capped to it, a smaller or non-numeric one stays; both spellings (`-Z threads=N`, `-Zthreads=N`) are read; without a grant (no tokens, or 0) nothing changes; a `cdylib` final (a plugin component, whose work is codegen and link-time optimisation of its dependency closure) always gets one front-end thread, and a serial LLVM back end (`-Z no-parallel-backend` appended) whatever the grant, because every extra codegen or ThinLTO worker adds a module's memory on top of the closure the final already holds; a wasm final (`--target wasm32-*`) also links on one wasm-ld thread (`-C link-arg=--threads=1` appended last), because lld threads every section pass and its peak grows with the thread count. usage: 🐍️.py <corpus.json> prints the expected arguments of every case as JSON."""
import json, re, sys


def bounded(args, tokens):
    granted = bool(tokens and tokens > 0)
    tokens = tokens if granted else 1
    def option(name):
        for i, arg in enumerate(args):
            if arg.startswith(name + "="):
                return arg[len(name) + 1:]
            if arg == name and i + 1 < len(args):
                return args[i + 1]
        return ""
    bounded_decoder = option("--crate-name") in ("semio_plugin_actor_input_decoder", "semio_plugin_actor_input_wire")
    final = "cdylib" in option("--crate-type").split(",")
    single = final or bounded_decoder
    serial = granted and tokens == 1 or single and (granted or any(value.startswith("-Z") for value in args))
    if single:
        tokens = 1
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
    out = out + ["-Z", "no-parallel-backend"] if serial else out
    return out + ["-C", "link-arg=--threads=1"] if final and option("--target").startswith("wasm32-") else out


if __name__ == "__main__":
    print(json.dumps([bounded(c["args"], c.get("tokens")) for c in json.load(open(sys.argv[1], encoding="utf-8"))["cases"]], ensure_ascii=False))
