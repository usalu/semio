import json,re,sys
fixture=json.load(open(sys.argv[1],encoding="utf8"))
p=json.load(open(sys.argv[2],encoding="utf8"))["definitions"]["RelativeWindowsFactsChange"]["const"]
def bound(value,points,tail):
    value=re.sub(r"[\x00-\x1f\x7f]","?",value)
    if len(value)<=points:return value
    return "…"+value[len(value)-points+1:] if tail else value[:points-1]+"…"
def show(field,value):
    if value is None:return p["absent"]
    if field in p["hexFields"]:return "0x%x"%(value&0xffffffff)
    return bound(str(value),p["maximumValuePoints"],False)
def fill(template,values):return re.sub(r"\{(\w+)\}",lambda m:values.get(m.group(1),""),template)
out={}
for row in fixture["rows"]:
    expected=dict(fixture["base"]);actual=dict(fixture["base"]);actual.update(row["actualChanges"])
    differences=[]
    for field in p["identityFields"]+p["fields"]:
        a,b=expected.get(field),actual.get(field)
        if a!=b:differences.append({"field":field if "role" not in row else row["role"]+"."+field,"expected":show(field,a),"actual":show(field,b)})
    texts={language:fill(p["texts"][language]["message"],{"refusal":p["refusals"][row["refusal"]][language],"directory":bound(row["directory"],p["maximumDirectoryPoints"],True),"differences":p["separator"].join(fill(p["texts"][language]["difference"],d) for d in differences)}) for language in ("en","de")} if differences else None
    out[row["id"]]={"differences":differences,"texts":texts}
print(json.dumps(out,ensure_ascii=False))
