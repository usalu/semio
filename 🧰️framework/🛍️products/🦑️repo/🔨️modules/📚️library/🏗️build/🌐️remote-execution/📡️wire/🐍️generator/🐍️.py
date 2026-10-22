#!/usr/bin/env python3
# usage: 🐍️.py <descriptors.json> <messages.json>
# Generates the wire schema (messages, enums, services) of the remote execution protocol from the serialized FileDescriptorProtos that protoc embedded in Bazel's own generated classes
# (fixture `descriptors.json`: {"source":{...},"files":{name: base64}}). Standard library only; deterministic output; refuses what the wire codec does not support (a map field is its repeated entry message on the wire).
import base64, json, sys

TYPES = {1: "double", 2: "float", 3: "int64", 4: "uint64", 5: "int32", 8: "bool", 9: "string", 11: "message", 12: "bytes", 13: "uint32", 14: "enum"}


def varint(data, i):
    shift = result = 0
    while True:
        b = data[i]; i += 1
        result |= (b & 0x7F) << shift
        if not b & 0x80:
            return result, i
        shift += 7


def fields(data):
    i = 0
    while i < len(data):
        key, i = varint(data, i)
        wire, number = key & 7, key >> 3
        if wire == 0:
            value, i = varint(data, i)
        elif wire == 2:
            length, i = varint(data, i); value = data[i:i + length]; i += length
        elif wire == 1:
            value = data[i:i + 8]; i += 8
        elif wire == 5:
            value = data[i:i + 4]; i += 4
        else:
            raise SystemExit(f"wire type {wire}")
        yield number, wire, value


def text(value):
    return value.decode("utf8")


def message(prefix, data, out):
    name = None
    rows, nested, enums = [], [], []
    for number, _, value in fields(data):
        if number == 1: name = text(value)
        elif number == 2: rows.append(value)
        elif number == 3: nested.append(value)
        elif number == 4: enums.append(value)
    full = prefix + "." + name
    spec = []
    for row in rows:
        entry = {"number": 0, "name": "", "type": "", "repeated": False}
        for number, _, value in fields(row):
            if number == 1: entry["name"] = text(value)
            elif number == 3: entry["number"] = value
            elif number == 4: entry["repeated"] = value == 3
            elif number == 5:
                if value not in TYPES: raise SystemExit(f"field type {value} in {full}")
                entry["type"] = TYPES[value]
            elif number == 6:
                entry["reference"] = text(value)[1:]
        reference = entry.pop("reference", None)
        if entry["type"] == "message": entry["message"] = reference
        if entry["type"] == "enum": entry["enum"] = reference
        spec.append(entry)
    out["messages"][full] = sorted(spec, key=lambda row: row["number"])
    for child in nested: message(full, child, out)
    for child in enums: enum(full, child, out)


def enum(prefix, data, out):
    name, values = None, {}
    for number, _, value in fields(data):
        if number == 1: name = text(value)
        elif number == 2:
            label, ordinal = None, 0
            for field, _, item in fields(value):
                if field == 1: label = text(item)
                elif field == 2: ordinal = item
            values[label] = ordinal
    out["enums"][prefix + "." + name] = values


def main():
    source = json.load(open(sys.argv[1]))
    out = {"version": 1, "source": source["source"], "messages": {}, "enums": {}, "services": {}}
    for _, blob in sorted(source["files"].items()):
        data = base64.b64decode(blob)
        package, bodies, services, enums = "", [], [], []
        for number, _, value in fields(data):
            if number == 2: package = text(value)
            elif number == 4: bodies.append(value)
            elif number == 5: enums.append(value)
            elif number == 6: services.append(value)
        for body in bodies: message(package, body, out)
        for body in enums: enum(package, body, out)
        for body in services:
            name, methods = None, {}
            for number, _, value in fields(body):
                if number == 1: name = text(value)
                elif number == 2:
                    method = {"request": "", "response": "", "clientStreaming": False, "serverStreaming": False}
                    label = None
                    for field, _, item in fields(value):
                        if field == 1: label = text(item)
                        elif field == 2: method["request"] = text(item)[1:]
                        elif field == 3: method["response"] = text(item)[1:]
                        elif field == 5: method["clientStreaming"] = bool(item)
                        elif field == 6: method["serverStreaming"] = bool(item)
                    methods[label] = method
            out["services"][package + "." + name] = dict(sorted(methods.items()))
    out["messages"] = dict(sorted(out["messages"].items()))
    out["enums"] = dict(sorted(out["enums"].items()))
    out["services"] = dict(sorted(out["services"].items()))
    for name, rows in out["messages"].items():
        for row in rows:
            target = row.get("message")
            if target is not None and target not in out["messages"]: raise SystemExit(f"unresolved {target} in {name}")
            target = row.get("enum")
            if target is not None and target not in out["enums"]: raise SystemExit(f"unresolved {target} in {name}")
    with open(sys.argv[2], "w", encoding="utf8") as handle:
        json.dump(out, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    print(len(out["messages"]), "messages,", len(out["enums"]), "enums,", len(out["services"]), "services")


main()
