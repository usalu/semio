"""🐍️ Independent oracle of the language protocol: reads only `🔣️.json` and the shared vectors, prints one JSON line per row (resolution and notice languages) and exits 1 when a row differs from its expectation; other oracles import `choose` and `notices`.

usage: python3 -I 🐍️.py [<protocol.json> <vectors.json>]
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def subtag(protocol, value):
    """🔤️ The language subtag of a locale, None for neutral or empty values."""
    if value is None:
        return None
    tag = re.split("[" + re.escape(protocol["subtagSeparators"]) + "]", value.strip().lower())[0]
    return None if tag == "" or tag in protocol["neutralTags"] else tag


def flag(protocol, args):
    """🏳️ The last flag value before the arguments end, None when absent."""
    head = args[: args.index(protocol["argumentsEnd"])] if protocol["argumentsEnd"] in args else args
    found, index = None, 0
    while index < len(head):
        if head[index] == protocol["flag"]:
            found = head[index + 1] if index + 1 < len(head) else ""
            index += 1
        elif head[index].startswith(protocol["flag"] + "="):
            found = head[index][len(protocol["flag"]) + 1 :]
        index += 1
    return found


def choose(protocol, languages, args, environment, system):
    """⚖️ The protocol order: flag, explicit variable, preference list, first non-empty locale variable, operating system list."""
    given = flag(protocol, args)
    if given is not None:
        return {"language": given, "source": protocol["flag"]} if given in languages else {"refusal": "flag"}
    explicit = environment.get(protocol["explicitVariable"], "")
    if explicit != "":
        tag = subtag(protocol, explicit)
        return {"language": tag, "source": protocol["explicitVariable"]} if tag in languages else {"refusal": "variable"}
    for entry in environment.get(protocol["preferenceListVariable"], "").split(protocol["preferenceListSeparator"]):
        if subtag(protocol, entry) in languages:
            return {"language": subtag(protocol, entry), "source": protocol["preferenceListVariable"]}
    named = next((name for name in protocol["environmentOrder"] if environment.get(name, "") != ""), None)
    if named is not None and subtag(protocol, environment[named]) in languages:
        return {"language": subtag(protocol, environment[named]), "source": named}
    for entry in system:
        if subtag(protocol, entry) in languages:
            return {"language": subtag(protocol, entry), "source": "os"}
    return {"refusal": "none"}


def notices(protocol, languages, environment, args=()):
    """🌐️ The languages a notice is written in: the one the arguments and the environment choose (never the operating system), every offered language without a choice."""
    got = choose(protocol, list(languages), list(args), environment, [])
    return [got["language"]] if "language" in got else list(languages)


def main():
    protocol_path = sys.argv[1] if len(sys.argv) > 2 else os.path.join(HERE, "..", "🔣️.json")
    vectors_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "🔣️.json")
    with open(protocol_path, encoding="utf8") as handle:
        protocol = json.load(handle)
    with open(vectors_path, encoding="utf8") as handle:
        vectors = json.load(handle)
    assert protocol["withoutChoice"] == {"commands": "operatingSystemThenRefuse", "notices": "everyOffered"}
    failed = 0
    for row in vectors["resolve"]:
        got = choose(protocol, vectors["languages"], row["args"], row["environment"], row["os"])
        spoken = notices(protocol, vectors["languages"], row["environment"], row["args"])
        want = row.get("expected") or {"refusal": row["refusal"]}
        failed += got != want
        print(json.dumps({"name": row["name"], "choice": got, "notices": spoken, "ok": got == want}, ensure_ascii=False))
    for value, language in vectors["tags"]:
        failed += subtag(protocol, value) != language
    for value in vectors["neutral"]:
        failed += subtag(protocol, value) is not None
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
