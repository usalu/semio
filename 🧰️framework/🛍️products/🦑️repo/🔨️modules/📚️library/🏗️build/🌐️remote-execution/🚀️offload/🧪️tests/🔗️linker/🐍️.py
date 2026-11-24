import hashlib, os, sys

LINKERS = {"rust-lld", "rust-lld.exe", "wasm-component-ld", "wasm-component-ld.exe"}

def identity(sysroot):
    rows = []
    base = os.path.join(sysroot, "lib", "rustlib")
    if not os.path.isdir(base):
        return None
    for host in sorted(os.listdir(base)):
        bin_dir = os.path.join(base, host, "bin")
        if not os.path.isdir(bin_dir):
            continue
        for name in sorted(os.listdir(bin_dir)):
            path = os.path.join(bin_dir, name)
            if os.path.isdir(path) and name == "gcc-ld":
                rows += [os.path.join(path, inner) for inner in sorted(os.listdir(path)) if os.path.isfile(os.path.join(path, inner))]
            elif os.path.isfile(path) and name in LINKERS:
                rows.append(path)
    if not rows:
        return None
    digest = hashlib.sha256()
    for path in rows:
        relative = path[len(sysroot):].replace("\\", "/")
        digest.update((relative + "\0" + hashlib.sha256(open(path, "rb").read()).hexdigest() + "\n").encode())
    return digest.hexdigest()[:16]

print(identity(sys.argv[1]) or "")
