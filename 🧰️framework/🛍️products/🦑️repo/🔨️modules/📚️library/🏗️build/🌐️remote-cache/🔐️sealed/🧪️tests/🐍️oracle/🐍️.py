#!/usr/bin/env python3
"""Independent oracle of the fleet sealed transport: Ed25519 public key from a seed, the Edwards to Montgomery map, X25519 (RFC 7748), HKDF-SHA256 (RFC 5869), all from the Python standard library only. `check <fixture>` recomputes the server key, the request key and IV and the reply key and IV of every vector and prints `ok`; `fill <fixture>` writes those values into the fixture."""
import hashlib, hmac, json, sys

P = 2**255 - 19
L = 2**252 + 27742317777372353535851937790883648493
D = -121665 * pow(121666, P - 2, P) % P
I = pow(2, (P - 1) // 4, P)


def inv(x):
    return pow(x, P - 2, P)


def recover_x(y, sign):
    xx = (y * y - 1) * inv(D * y * y + 1) % P
    x = pow(xx, (P + 3) // 8, P)
    if (x * x - xx) % P != 0:
        x = x * I % P
    if x & 1 != sign:
        x = P - x
    return x


BY = 4 * inv(5) % P
BX = recover_x(BY, 0)
BASE = (BX, BY, 1, BX * BY % P)


def add(a, b):
    x1, y1, z1, t1 = a
    x2, y2, z2, t2 = b
    A = (y1 - x1) * (y2 - x2) % P
    B = (y1 + x1) * (y2 + x2) % P
    C = t1 * 2 * D * t2 % P
    E2 = z1 * 2 * z2 % P
    E, F, G, H = B - A, E2 - C, E2 + C, B + A
    return (E * F % P, G * H % P, F * G % P, E * H % P)


def scalar_mult(k, point):
    result = (0, 1, 1, 0)
    addend = point
    while k:
        if k & 1:
            result = add(result, addend)
        addend = add(addend, addend)
        k >>= 1
    return result


def ed25519_public(seed):
    digest = hashlib.sha512(seed).digest()
    a = int.from_bytes(digest[:32], "little")
    a &= (1 << 254) - 8
    a |= 1 << 254
    x, y, z, _ = scalar_mult(a, BASE)
    zi = inv(z)
    x, y = x * zi % P, y * zi % P
    return (y | ((x & 1) << 255)).to_bytes(32, "little")


def montgomery_of(ed_public):
    y = int.from_bytes(ed_public, "little") & ((1 << 255) - 1)
    return ((1 + y) * inv((1 - y) % P) % P).to_bytes(32, "little")


def x25519(scalar, u_bytes):
    k = bytearray(scalar)
    k[0] &= 248
    k[31] &= 127
    k[31] |= 64
    k = int.from_bytes(k, "little")
    u = int.from_bytes(u_bytes, "little") & ((1 << 255) - 1)
    a24 = 121665
    x1, x2, z2, x3, z3, swap = u, 1, 0, u, 1, 0
    for t in reversed(range(255)):
        bit = (k >> t) & 1
        swap ^= bit
        if swap:
            x2, x3, z2, z3 = x3, x2, z3, z2
        swap = bit
        A, B = (x2 + z2) % P, (x2 - z2) % P
        AA, BB = A * A % P, B * B % P
        E = (AA - BB) % P
        C, Dd = (x3 + z3) % P, (x3 - z3) % P
        DA, CB = Dd * A % P, C * B % P
        x3 = pow(DA + CB, 2, P)
        z3 = x1 * pow(DA - CB, 2, P) % P
        x2 = AA * BB % P
        z2 = E * (AA + a24 * E) % P
    if swap:
        x2, x3, z2, z3 = x3, x2, z3, z2
    return (x2 * inv(z2) % P).to_bytes(32, "little")


def hkdf(ikm, salt, info, length):
    prk = hmac.new(salt, ikm, hashlib.sha256).digest()
    out, block = b"", b""
    counter = 1
    while len(out) < length:
        block = hmac.new(prk, block + info + bytes([counter]), hashlib.sha256).digest()
        out += block
        counter += 1
    return out[:length]


def compute(vector):
    origin = bytes.fromhex(vector["originSeed"])
    asker = bytes.fromhex(vector["askerSeed"])
    server = bytes.fromhex(vector["serverSeed"])
    group = vector["group"].encode()
    server_public = montgomery_of(ed25519_public(origin))
    scalar = hashlib.sha512(origin).digest()[:32]
    base = (9).to_bytes(32, "little")
    same = x25519(scalar, base) == server_public
    epk = x25519(asker, base)
    first = x25519(asker, server_public)
    request = hkdf(first, epk + server_public, b"semio-fleet-sealed/1 request\x00" + group, 44)
    server_epk = x25519(server, base)
    second = x25519(server, epk)
    request_hash = hashlib.sha256(bytes.fromhex(vector["requestCiphertext"])).digest()
    reply = hkdf(first + second, epk + server_epk + request_hash, b"semio-fleet-sealed/1 reply\x00" + group, 44)
    return same, {
        "serverX25519": server_public.hex(),
        "requestKey": request[:32].hex(), "requestIv": request[32:].hex(),
        "replyKey": reply[:32].hex(), "replyIv": reply[32:].hex(),
    }


def main():
    mode, path = sys.argv[1], sys.argv[2]
    fixture = json.load(open(path, encoding="utf8"))
    bad = []
    for vector in fixture["vectors"]:
        same, values = compute(vector)
        if not same:
            bad.append(vector["name"] + ": the converted public key differs from the key of the converted private key")
        if mode == "fill":
            vector.update(values)
        else:
            for key, value in values.items():
                if vector[key] != value:
                    bad.append("%s: %s differs" % (vector["name"], key))
    if mode == "fill":
        open(path, "w", encoding="utf8").write(json.dumps(fixture, indent=1, ensure_ascii=False) + "\n")
    if bad:
        print("\n".join(bad))
        sys.exit(1)
    print("ok")


main()
