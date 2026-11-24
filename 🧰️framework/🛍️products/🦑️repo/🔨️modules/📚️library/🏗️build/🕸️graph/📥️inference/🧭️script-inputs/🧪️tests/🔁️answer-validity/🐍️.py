#!/usr/bin/env python3
"""Independent oracle of answer validity: for every scenario it walks the import graph of the closure before and after an edit with its own regular expression, answers the recorded queries (`some` over the file list) against both, and states whether an owner record that recorded the first answers stays valid, how many closure walks the take may need and why a record that does not hold fails."""
import json,re,sys

IMPORT=re.compile(r'import\s+"\./([^"]+)"')

def closure(files,entry):
    seen=[];todo=[entry]
    while todo:
        name=todo.pop()
        if name in seen or name not in files:continue
        seen.append(name)
        todo.extend("tools/"+match for match in IMPORT.findall(files[name]))
    return sorted(seen)

def some(listing,needles):return any(needle in path for path in listing for needle in needles)

def source(imports,tag):return "".join('import "./'+name+'";\n' for name in imports)+"export const "+tag+"=1;\n"

SCENARIOS=[
    {"name":"extra import keeps every answer","before":{"a":["b"],"b":["c"],"c":[]},"after":{"a":["b"],"b":["c","d"],"c":[],"d":[]},"queries":[["c.mjs"],["z.mjs"]]},
    {"name":"dropped import changes an answer","before":{"a":["b"],"b":["c"],"c":[]},"after":{"a":["b"],"b":[],"c":[]},"queries":[["c.mjs"],["z.mjs"]]},
    {"name":"added file makes a false answer true","before":{"a":["b"],"b":[],"z":[]},"after":{"a":["b","z"],"b":[],"z":[]},"queries":[["z.mjs"]]},
]

def build(spec):
    return {"tools/"+name+".mjs":source([i+".mjs" for i in imports],name) for name,imports in spec.items()}

def main():
    out=[]
    for scenario in SCENARIOS:
        before,after=build(scenario["before"]),build(scenario["after"])
        recorded=[some(closure(before,"tools/a.mjs"),query) for query in scenario["queries"]]
        now=[some(closure(after,"tools/a.mjs"),query) for query in scenario["queries"]]
        out.append({"name":scenario["name"],"before":before,"after":after,"queries":scenario["queries"],"recorded":recorded,"now":now,"holds":recorded==now,"walks":1,"failure":None if recorded==now else "closure-value"})
    json.dump({"scenarios":out},sys.stdout,ensure_ascii=False)

main()
