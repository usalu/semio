#!/usr/bin/env python3
"""Independent oracle of the kill record of a termination: the English and the German line from the schema's texts and the fixture's records, `<English> / <German>`. Usage: oracle.py <schema.json> <fixture.json>"""
import json,re,sys
texts=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Texts"]["const"]
fx=json.load(open(sys.argv[2],encoding="utf8"))
def fill(lang,r):return re.sub(r"\{(\w+)\}",lambda m:str(r.get(m.group(1),"")),texts[lang]["kill-record"])
print(json.dumps([fill("en",r)+" / "+fill("de",r) for r in fx["records"]],ensure_ascii=False))
