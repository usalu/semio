"""🪪️ Independent Python oracle of the Nx tooling install identity (standard library only): the key is the sha256 over a version tag and then the platform triple and every file name and content as length-prefixed fields (decimal byte count, a colon, the bytes), in the order given. The authorizer bundle key is the sha256 over its own version tag, the dependency generation key and every manifest name and content as the same length-prefixed fields. Requests arrive as JSON on stdin."""
import hashlib, json, sys


def key(environment, files):
    digest = hashlib.sha256()
    digest.update(b"nx-tooling-install-v5\0")

    def field(text):
        data = text.encode("utf8")
        digest.update(str(len(data)).encode() + b":" + data)

    for part in (environment["platform"], environment["architecture"], environment["libc"]):
        field(part)
    for name, text in files:
        field(name)
        field(text)
    return digest.hexdigest()


def bundle(dependencies, files):
    digest = hashlib.sha256()
    digest.update(b"nx-authorizer-bundle-v1\0")

    def field(text):
        data = text.encode("utf8")
        digest.update(str(len(data)).encode() + b":" + data)

    field(dependencies)
    for name, text in files:
        field(name)
        field(text)
    return digest.hexdigest()


request = json.load(sys.stdin)
print(json.dumps([bundle(row["dependencies"], row["files"]) if "dependencies" in row else key(row["environment"], row["files"]) for row in request["rows"]]))
