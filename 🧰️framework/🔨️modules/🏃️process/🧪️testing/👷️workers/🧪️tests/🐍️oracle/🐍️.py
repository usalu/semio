"""👷️ Independent Python oracle of the test worker policy: the facts of a process (its share of the admitted grant, else of the host) and the number of workers a runner may start. It shares no code with the TypeScript module and writes the language-agnostic vectors both read. Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`; `python3 🐍️.py --check` compares the stored vectors with this model and exits 1 on any difference."""
import itertools,json,math,os,sys

POLICY={
    "version":1,
    "hostShareBasisPoints":5000,
    "runners":[
        {"id":"bun-test","override":"SEMIO_BUN_TEST_WORKERS","baseMegabytes":101,"workerMegabytes":509,"wanted":1,"maximum":6,"oversubscription":1},
        {"id":"vitest","override":"SEMIO_VITEST_WORKERS","baseMegabytes":997,"workerMegabytes":151,"wanted":4,"maximum":8,"oversubscription":4},
        {"id":"cargo-test","override":"SEMIO_CARGO_TEST_THREADS","baseMegabytes":203,"workerMegabytes":197,"wanted":"cores","maximum":16,"oversubscription":2},
        {"id":"playwright","override":"SEMIO_PLAYWRIGHT_WORKERS","baseMegabytes":301,"workerMegabytes":803,"wanted":1,"maximum":4,"oversubscription":2},
        {"id":"e2e-stacks","override":"SEMIO_E2E_STACKS","baseMegabytes":257,"workerMegabytes":1501,"wanted":2,"maximum":2,"oversubscription":1},
    ],
}
PLAN_ENVIRONMENT="SEMIO_HOST_CAPACITY_PLAN"

def ceil_div(a,b):
    return -(-a//b)

def facts(environment,host):
    text=environment.get(PLAN_ENVIRONMENT)
    if not text:
        return {"source":"host","cores":host["cores"],"hostCores":host["cores"],"memoryMegabytes":host["totalMegabytes"]*POLICY["hostShareBasisPoints"]//10000}
    plan=json.loads(text)
    jobs=max(1,plan["jobs"])
    memory=plan["memoryMegabytes"]//jobs if plan["memoryMegabytes"]>0 else plan["fixedMegabytes"]
    return {"source":"grant","cores":max(1,plan["cpu"]//jobs),"hostCores":host["cores"],"memoryMegabytes":memory}

def parse_override(runner,text):
    if text is None:
        return None
    if not (text.isascii() and text.isdigit() and text[0]!="0" and len(text)<=3):
        raise ValueError("override")
    count=int(text)
    if count<1 or count>runner["maximum"]:
        raise ValueError("override")
    return count

def plan(runner_id,fact,request=None,ratio=10000,override=None):
    runner=next(row for row in POLICY["runners"] if row["id"]==runner_id)
    base=ceil_div(runner["baseMegabytes"]*ratio,10000)
    each=max(1,ceil_div(runner["workerMegabytes"]*ratio,10000))
    asked=request if request is not None else (fact["hostCores"] if runner["wanted"]=="cores" else runner["wanted"])
    wanted=max(1,min(asked,runner["maximum"]))
    clipped="maximum" if asked>runner["maximum"] else ("request" if request is not None else "wanted")
    memory_cap=(fact["memoryMegabytes"]-base)//each if fact["memoryMegabytes"]>=base+each else 0
    core_cap=fact["cores"]*runner["oversubscription"]
    forced=parse_override(runner,override)
    if forced is not None:
        workers=max(1,min(forced,runner["maximum"],core_cap))
        bound="override" if workers==forced else "cores"
    else:
        workers=max(1,min(wanted,memory_cap,core_cap))
        if memory_cap<wanted and memory_cap<=core_cap:
            bound="memory"
        elif core_cap<wanted:
            bound="cores"
        else:
            bound=clipped
    return {"workers":workers,"bound":bound}

def case(name,runner,fact,**options):
    row={"name":name,"runner":runner,"facts":fact}
    for key in ("request","ratioBasisPoints","override"):
        if key in options and options[key] is not None:
            row[key]=options[key]
    try:
        row["expect"]=plan(runner,fact,options.get("request"),options.get("ratioBasisPoints") or 10000,options.get("override"))
    except ValueError:
        row["refuse"]="override"
        row["expect"]={"workers":1,"bound":"override"}
    return row

def grant(jobs,cpu,memory,fixed=1536):
    return json.dumps({"version":1,"id":"k"*22,"jobs":jobs,"cpu":cpu,"memoryMegabytes":memory,"fixedMegabytes":fixed,"floorMegabytes":2048},separators=(",",":"))

def build():
    cases=[]
    for runner in (row["id"] for row in POLICY["runners"]):
        for cores,memory,ratio,request in itertools.product((1,3,10),(0,500,2048,8192),(3000,10000,20000),(None,2,64)):
            fact={"source":"grant","cores":cores,"hostCores":10,"memoryMegabytes":memory}
            cases.append(case(f"{runner} grid {cores} cores {memory} MB ratio {ratio} request {request}",runner,fact,ratioBasisPoints=None if ratio==10000 else ratio,request=request))
    for row in POLICY["runners"]:
        for ratio in (3000,7777,10000,13333,20000):
            base=ceil_div(row["baseMegabytes"]*ratio,10000)
            each=max(1,ceil_div(row["workerMegabytes"]*ratio,10000))
            for count in (1,2,3,4):
                for delta in (-1,0):
                    cases.append(case(f"{row['id']} threshold of {count} worker(s) at ratio {ratio}{' minus one MB' if delta else ''}",row["id"],{"source":"grant","cores":64,"hostCores":64,"memoryMegabytes":max(0,base+count*each+delta)},ratioBasisPoints=ratio,request=64))
    for runner in (row["id"] for row in POLICY["runners"]):
        cases.append(case(f"{runner} on a 40-core host with 64 GB",runner,{"source":"host","cores":40,"hostCores":40,"memoryMegabytes":32768}))
    host={"source":"host","cores":10,"hostCores":10,"memoryMegabytes":8192}
    small={"source":"grant","cores":1,"hostCores":10,"memoryMegabytes":2048}
    for runner in (row["id"] for row in POLICY["runners"]):
        for text in ("1","2","6","7","8","16","0","00","01","-1","1.5","four","","1000"):
            cases.append(case(f"{runner} override {text!r} on the host",runner,host,override=text))
            cases.append(case(f"{runner} override {text!r} under a small grant",runner,small,override=text))
    environments=[
        ("no grant: the host share",{},{"cores":10,"totalMegabytes":16384}),
        ("no grant on an 8 GB laptop",{},{"cores":4,"totalMegabytes":8192}),
        ("one job of 2048 MB",{PLAN_ENVIRONMENT:grant(1,1,2048)},{"cores":10,"totalMegabytes":16384}),
        ("four jobs share the plan",{PLAN_ENVIRONMENT:grant(4,4,8192)},{"cores":10,"totalMegabytes":32768}),
        ("two cores per job",{PLAN_ENVIRONMENT:grant(3,6,6144)},{"cores":10,"totalMegabytes":32768}),
        ("a fixed operation carries its memory in the fixed block",{PLAN_ENVIRONMENT:grant(1,1,0,2048)},{"cores":10,"totalMegabytes":16384}),
        ("an empty plan variable is no plan",{PLAN_ENVIRONMENT:""},{"cores":6,"totalMegabytes":12288}),
    ]
    rows=[{"name":name,"environment":environment,"host":host_row,"expect":facts(environment,host_row)} for name,environment,host_row in environments]
    return {"policy":POLICY,"cases":cases,"facts":rows}

def main():
    here=os.path.dirname(os.path.abspath(__file__))
    path=os.path.join(here,"..","🧫️fixtures","🔣️.json")
    corpus=build()
    if "--check" in sys.argv:
        stored=json.load(open(path,encoding="utf8"))
        if stored!=corpus:
            print("the stored vectors differ from the model")
            sys.exit(1)
        print(f"{len(corpus['cases'])} plan vectors and {len(corpus['facts'])} fact vectors equal the model")
        return
    json.dump(corpus,open(path,"w",encoding="utf8"),ensure_ascii=False,indent=1)
    open(path,"a",encoding="utf8").write("\n")
    print(f"wrote {len(corpus['cases'])} plan vectors and {len(corpus['facts'])} fact vectors")

main()
