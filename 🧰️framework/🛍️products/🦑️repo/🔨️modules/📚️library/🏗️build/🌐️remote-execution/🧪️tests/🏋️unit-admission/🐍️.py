"""🐍️ Independent Python oracle of the admission of one offloaded unit by its own size: the pressure gate (critical refuses all, warn alone refuses only a large unit or while swap grows), the memory a unit may take above the floor (the spare a machine reports) with the units it already runs, the peak a unit is placed with (learned plus margin, else the unknown default), what a crate is known by after a measurement and after a worker killed it. Reads the numbers from the policies. `python3 🐍️.py` rewrites the vectors, `--check` fails when they are stale."""
import json,math,os,statistics,sys

HERE=os.path.dirname(os.path.abspath(__file__))
BUILD=os.path.join(HERE,"..","..","..")
HOST=json.load(open(os.path.join(BUILD,"🔣️policy.json")))["hostCapacity"]
ADMISSION=json.load(open(os.path.join(HERE,"..","..","🌡️load","🧬️schema","🔣️.json")))["definitions"]["Policy"]["const"]
OFFLOAD=json.load(open(os.path.join(HERE,"..","..","🚀️offload","🧬️schema","🔣️.json")))["policy"]
TARGET=os.path.join(HERE,"🔣️.json")
CODEGEN=json.load(open(os.path.join(BUILD,"🔣️policy.json")))["remoteExecution"]["worker"]["codegen"]
TOTAL=16384

def gate(pressure,total,swap_out,megabytes):
    if pressure=="critical":return "pressure-critical"
    if pressure=="normal":return "normal"
    swapping=swap_out is not None and swap_out*10000>=total*HOST["swap"]["activeBasisPointsPerSecond"]
    return "pressure-warn" if megabytes>ADMISSION["warnMaximumMegabytes"] or swapping else "relaxed"

def floor_of(total):
    m=HOST["memory"]
    return min(total,max(m["floorMinimumMegabytes"],min(m["floorMaximumMegabytes"],total*m["floorBasisPoints"]//10000)))

def reserve_of(total):
    m=HOST["memory"]
    if "interactiveReserveBasisPoints" not in m:return 0
    share=total*m["interactiveReserveBasisPoints"]//10000
    return min(total,max(m.get("interactiveReserveMinimumMegabytes",0),min(m.get("interactiveReserveMaximumMegabytes",total),share)))

def spare_and_verdict(total,available,held,megabytes,pressure,swap_out):
    """The memory kept for the interactive apps of the host (`interactiveReserve*` of the memory policy) counts as unmanaged memory at least. A running unit is charged once: what it holds now (used) is in the available memory, the part of its peak it has not grown into yet is kept free."""
    floor=floor_of(total);cap=max(0,total-floor)
    used=sum(row["usedMegabytes"] for row in held);growth=sum(max(0,row["megabytes"]-row["usedMegabytes"]) for row in held);committed=sum(max(row["megabytes"],row["usedMegabytes"]) for row in held)
    unmanaged=max(max(0,total-available-used),reserve_of(total))
    share=max(0,cap-unmanaged)
    budget=share*HOST["memory"]["budgetBasisPoints"]//10000-committed
    observed=available-floor-growth
    spare=max(0,min(observed,budget))
    g=gate(pressure,total,swap_out,megabytes)
    need=min(megabytes,cap)
    if g in("pressure-warn","pressure-critical"):verdict=g
    elif need<=observed and need<=budget:verdict="admit"
    else:verdict="memory"
    return spare,verdict,g

def margin(megabytes):
    p=OFFLOAD["peak"]
    return min(p["maximumMegabytes"],max(p["minimumMegabytes"],math.ceil(megabytes*(100+p["marginPercent"])/100)))

def unit_peak(learned,wasm_linked,family=None):
    p=OFFLOAD["peak"]
    if learned is not None and learned>0:return margin(learned)
    if family is not None and family>0 and wasm_linked:return margin(family)
    return max(p["unknownMegabytes"],OFFLOAD["wasm"]["peakMegabytes"] if wasm_linked else 0)

def family_peak(samples):
    f=OFFLOAD["peak"]["family"];values=sorted(v for v in samples if v>0)
    if len(values)<f["minimumCrates"]:return None
    position=(len(values)-1)*f["quantilePercent"]/100;lower=math.floor(position)
    own=values[lower] if lower+1>=len(values) else values[lower]+(position-lower)*(values[lower+1]-values[lower])
    if len(values)>=2:
        library=statistics.quantiles(values,n=100,method="inclusive")[f["quantilePercent"]-1] if f["quantilePercent"]<100 else values[-1]
        assert math.isclose(own,library,rel_tol=1e-9,abs_tol=1e-9),(values,own,library)
    return math.ceil(own)

def learn(before,measured):
    return math.ceil(measured) if before is None or measured>=before else math.ceil((before+measured)/2)

def killed(exit_code,placed,learned):
    p=OFFLOAD["peak"]
    if exit_code!=p["killedExit"]:return None
    return min(p["maximumMegabytes"],max(learned or 0,placed*p["killedFactor"]))

def peak_at_threads(samples,threads,extra):
    rows=[(int(count),megabytes) for count,megabytes in samples.items() if int(count)>0 and megabytes>0]
    for count,megabytes in rows:
        if count==threads:return megabytes
    below=[row for row in rows if row[0]<threads];above=[row for row in rows if row[0]>threads]
    grown=None if not below else max(below)[1]+(threads-max(below)[0])*extra
    if grown is None:return min(above)[1] if above else None
    return grown if not above else min(grown,min(above)[1])

def build():
    gates=[{"pressure":pressure,"swapOutMegabytesPerSecond":swap,"megabytes":megabytes,"gate":gate(pressure,TOTAL,swap,megabytes)} for pressure in ("normal","warn","critical") for swap in (None,0,0.5,1.2,52) for megabytes in (256,1024,1025,2048,6144)]
    admissions=[]
    for available in (3000,4300,6000,8000,12000):
        for held in ([],[{"megabytes":2048,"usedMegabytes":0}],[{"megabytes":6144,"usedMegabytes":3000}],[{"megabytes":512,"usedMegabytes":512},{"megabytes":512,"usedMegabytes":100}],[{"megabytes":2048,"usedMegabytes":0},{"megabytes":2048,"usedMegabytes":2048},{"megabytes":2048,"usedMegabytes":4000}],[{"megabytes":5014,"usedMegabytes":1200}]):
            for megabytes in (256,512,1024,2048,6144):
                for pressure,swap in (("normal",0),("warn",0),("warn",52),("critical",0)):
                    spare,verdict,_=spare_and_verdict(TOTAL,available,held,megabytes,pressure,swap)
                    admissions.append({"totalMegabytes":TOTAL,"availableMegabytes":available,"held":held,"megabytes":megabytes,"pressure":pressure,"swapOutMegabytesPerSecond":swap,"spareMegabytes":spare,"verdict":verdict})
    peaks=[{"learned":learned,"wasmLinked":wasm,"placed":unit_peak(learned,wasm)} for learned in (None,0,10,512,1600,2000,13000,40000) for wasm in (False,True)]
    learns=[{"before":before,"measured":measured,"learned":learn(before,measured)} for before in (None,100,1000,5000) for measured in (50,100,1000,7000.4)]
    kills=[{"exit":code,"placed":placed,"learned":learned,"taught":killed(code,placed,learned)} for code in (0,1,101,137,143) for placed in (2048,6144,12000) for learned in (None,1000,9000)]
    measured={"semio_framework_editor_component":385,"semio_framework_surface_component":548,"semio_os_node_graph_component":1232,"semio_os_flow_component":1857,"semio_hub_puzzle_component":6148,"semio_hub_cad_component":6550}
    sets=[[],[100],[100,200],[100,200,300],[500,1500,2500,4500],[385,548,1232,1857,6148,6550],[3279,5022,6148,6550],[1000,1000,1000,1000],[200,400,600,800,1000,12000],[0,-5,400,800,1200]]
    families=[{"samples":samples,"family":family_peak(samples),"unseen":unit_peak(None,True,family_peak(samples)),"unseenNative":unit_peak(None,False,family_peak(samples)),"learned":unit_peak(3000,True,family_peak(samples))} for samples in sets]
    kills_in_family=[]
    for samples in sets[3:9]:
        for placed in (4096,7938,12000):
            taught=killed(137,placed,None);after=samples+[taught]
            kills_in_family.append({"samples":samples,"placed":placed,"taught":taught,"familyBefore":family_peak(samples),"familyAfter":family_peak(after),"unseenAfter":unit_peak(None,True,family_peak(after))})
    base=family_peak(list(measured.values()));unseen=unit_peak(None,True,base);taught=killed(137,unseen,None);after=family_peak(list(measured.values())+[taught])
    observed={"crates":measured,"family":base,"unseen":unseen,"killed":{"placed":unseen,"taught":taught,"familyAfter":after,"unseenAfter":unit_peak(None,True,after)}}
    extra=CODEGEN["extraMegabytesPerThread"]
    sample_sets=[{},{"1":4011},{"4":3787},{"1":4011,"4":4500},{"1":1000,"8":1200},{"2":2000,"6":2100},{"1":300,"2":5000}]
    thread_rows=[{"samples":samples,"threads":threads,"extra":extra,"peak":peak_at_threads(samples,threads,extra)} for samples in sample_sets for threads in (1,2,3,4,6,8)]
    local_final_rows=[]
    for samples in ({"1":2720},{"1":4100},{"1":3294,"4":6000},{"4":6000},{}):
        for managed in (True,False):
            count=1 if managed else CODEGEN["fairShareThreads"]
            peak=samples.get("1") if managed else peak_at_threads(samples,count,extra)
            local_final_rows.append({"samples":samples,"managed":managed,"localPeak":peak,"fleetPeak":peak_at_threads(samples,CODEGEN["fairShareThreads"],extra),"beyond16g":peak is not None and peak>4096})
    return {"localFinals":local_final_rows,"threads":thread_rows,"gates":gates,"admissions":admissions,"peaks":peaks,"learns":learns,"kills":kills,"families":families,"familyKills":kills_in_family,"observed":observed}

if __name__=="__main__":
    text=json.dumps(build(),ensure_ascii=False,indent=1)+"\n"
    if "--check" in sys.argv:
        if open(TARGET).read()!=text:sys.exit("stale vectors")
    else:open(TARGET,"w").write(text)
