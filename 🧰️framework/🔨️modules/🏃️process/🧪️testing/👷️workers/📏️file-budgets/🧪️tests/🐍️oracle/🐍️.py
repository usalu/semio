"""📏️ Independent Python oracle of the test file budgets: the limits of one test file (budget and ceiling), the size of a cohort against the default of a file process, and whether the next file may start. It shares no code with the TypeScript module; `--check` compares the stored vectors with this model, no argument rewrites them."""
import itertools,json,os,sys

TABLE={
    "version":1,
    "ceilingBasisPoints":15137,
    "unlistedCeilingMegabytes":3001,
    "files":[
        {"path":"alpha/b/c/🟦️.ts","megabytes":731,"reason":"odd size so a floor instead of a ceiling shows"},
        {"path":"alpha/d/🟦️.ts","megabytes":64,"reason":"the smallest budget"},
        {"path":"zulu/🟦️.ts","megabytes":2999,"reason":"above the unlisted ceiling"},
    ],
}

def ceil_div(a,b):
    return -(-a//b)

def limits(table,path,default):
    share=lambda megabytes:ceil_div(megabytes*table["ceilingBasisPoints"],10000)
    for row in table["files"]:
        if row["path"]==path:
            return {"budgetMegabytes":row["megabytes"],"ceilingMegabytes":share(row["megabytes"]),"declared":True}
    return {"budgetMegabytes":default,"ceilingMegabytes":max(table["unlistedCeilingMegabytes"],share(default)),"declared":False}

def ratio(table,paths,default):
    sizes=[limits(table,path,default)["budgetMegabytes"] for path in paths]+[default]
    return ceil_div(max(sizes)*10000,default)

def admit(held,nxt,share):
    return len(held)==0 or sum(held)+nxt<=share

def build():
    cases=[]
    paths=["alpha/b/c/🟦️.ts","alpha/d/🟦️.ts","zulu/🟦️.ts","omega/🟦️.ts","alpha/b/c/🟦️.ts.bak"]
    for path,default in itertools.product(paths,[1,64,509,1024,2999,3001,5000]):
        cases.append({"name":f"limits {path} default {default}","kind":"limits","input":{"path":path,"default":default},"expect":limits(TABLE,path,default)})
    for cohort,default in itertools.product([[],["omega/🟦️.ts"],["alpha/d/🟦️.ts"],["alpha/b/c/🟦️.ts","alpha/d/🟦️.ts"],["zulu/🟦️.ts","omega/🟦️.ts"],paths],[1,64,509,731,1024,2999,4000]):
        cases.append({"name":f"ratio {len(cohort)} files default {default}","kind":"ratio","input":{"paths":cohort,"default":default},"expect":ratio(TABLE,cohort,default)})
    for held,nxt,share in itertools.product([[],[500],[500,500],[1,2,3,4],[1024]],[1,500,1000,1024,2048],[1000,1024,2048,2047]):
        cases.append({"name":f"admit held {held} next {nxt} share {share}","kind":"admit","input":{"held":held,"next":nxt,"share":share},"expect":admit(held,nxt,share)})
    return {"version":1,"table":TABLE,"cases":cases}

def main():
    here=os.path.dirname(os.path.abspath(__file__))
    path=os.path.join(here,"..","🧫️fixtures","🔣️.json")
    corpus=build()
    if "--check" in sys.argv:
        stored=json.load(open(path,encoding="utf8"))
        if stored!=corpus:
            print("the stored vectors differ from the model")
            sys.exit(1)
        print(f"{len(corpus['cases'])} vectors equal the model")
        return
    json.dump(corpus,open(path,"w",encoding="utf8"),ensure_ascii=False,indent=1)
    open(path,"a",encoding="utf8").write("\n")
    print(f"wrote {len(corpus['cases'])} vectors")

main()
