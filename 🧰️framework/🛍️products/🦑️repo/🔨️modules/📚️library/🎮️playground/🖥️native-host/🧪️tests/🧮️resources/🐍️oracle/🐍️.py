import json,re,sys

def render(schema,fixture,language):
    messages=schema["$defs"]["NativeHostMessagesV1"]["const"]
    table,labels=messages["texts"][language],messages["fields"][language]
    host,facts=fixture["host"],fixture["facts"]
    modes=facts["modes"].get(host["target"],[])
    rows=[]
    if facts["package"]!=host["package"]:rows.append(("package",host["package"],json.dumps(facts["package"])))
    if facts["project"]!=host["project"]:rows.append(("project",host["project"],json.dumps(facts["project"])))
    if host["binary"] not in facts["binaries"]:rows.append(("binary",host["binary"],"["+", ".join(facts["binaries"])+"]"))
    if host["target"] not in facts["targets"]:rows.append(("target",host["target"],"["+", ".join(facts["targets"])+"]"))
    elif not all(mode in modes for mode in ("prototype","release")):rows.append(("modes","prototype, release","["+", ".join(modes)+"]"))
    fill=lambda text,values:re.sub(r"\{([a-zA-Z]+)\}",lambda match:str(values[match.group(1)]),text)
    differences="; ".join(fill(table["difference"],{"field":labels[field],"declared":declared,"authored":found}) for field,declared,found in rows) or table["none"]
    context=" ("+fixture["context"]+")" if fixture["context"] else ""
    return fill(table["mismatch"],{"context":context,"project":host["project"],"cratePath":host["cratePath"],"differences":differences})

if __name__=="__main__":
    schema=json.load(open(sys.argv[1],encoding="utf8"))
    fixture=json.load(open(sys.argv[2],encoding="utf8"))["mismatch"]
    print(json.dumps({language:render(schema,fixture,language) for language in ("en","de")},ensure_ascii=False))
