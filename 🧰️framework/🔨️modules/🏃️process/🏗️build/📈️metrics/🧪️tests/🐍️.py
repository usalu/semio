#!/usr/bin/env python3
"""🐍️ Independent oracle of the metrics rings of `../🟦️.ts` and `../📦️packages/🦀️rust`. It shares no code with them: it knows the ring layout only from `../🧬️schema/🔣️.json`.
usage:
  🐍️.py fold  < {"tiers":[{"bucketMs","buckets"}],"ids":[..],"samples":[{"at","values":{id:v}}]}   the rows the rings must hold after the samples and a clean close, folded straight from the raw samples
  🐍️.py delta < {"readings":[{"id","value"}]}                                                      the counter deltas of cumulative readings (first reading null, a decrease is a restart)
  🐍️.py series <directory> <fromMs> <toMs>                                                         the merged series read from the ring files, one TSV line `id start width tier count min max sum last partial` per point
  🐍️.py rows <file>                                                                                 every valid row of one ring file as JSON
Python 3.9 compatible."""
import json,os,struct,sys
HERE=os.path.dirname(os.path.abspath(__file__))
SCHEMA=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json"),encoding="utf8"))["definitions"]
RING=SCHEMA["Ring"]["const"]
SLOT=RING["slotBytes"]
STAMP=RING["stampBytes"]
def num(value):
    return str(int(value)) if value==int(value) else repr(value)
def fold(case):
    ids,tiers=case["ids"],case["tiers"]
    windows=[{} for _ in tiers]
    last=0
    for sample in sorted(case["samples"],key=lambda row:row["at"]):
        start=int(sample["at"])//tiers[0]["bucketMs"]*tiers[0]["bucketMs"]
        last=max(last,start)
        for metric,value in sample["values"].items():
            if metric not in ids: continue
            for index,tier in enumerate(tiers):
                key=int(sample["at"])//tier["bucketMs"]*tier["bucketMs"]
                slot=windows[index].setdefault(key,{}).setdefault(metric,[0,None,None,0,None])
                slot[0]+=1
                slot[1]=value if slot[1] is None else min(slot[1],value)
                slot[2]=value if slot[2] is None else max(slot[2],value)
                slot[3]+=value
                slot[4]=value
    out=[]
    for index,tier in enumerate(tiers):
        width=tier["bucketMs"]
        closed=[key for key in windows[index] if key<=last] if index==0 else [key for key in windows[index] if key<last//width*width]
        if not closed: out.append({"tier":index,"rows":[]}); continue
        newest=max(closed)
        kept=[key for key in sorted(closed) if key>newest-tier["buckets"]*width]
        out.append({"tier":index,"rows":[{"start":key,"slots":[windows[index][key].get(metric,[0,0,0,0,0]) for metric in ids]} for key in kept]})
    return out
def deltas(case):
    previous={}
    out=[]
    for reading in case["readings"]:
        before=previous.get(reading["id"]); previous[reading["id"]]=reading["value"]
        out.append(None if before is None else reading["value"] if reading["value"]<before else reading["value"]-before)
    return out
def fnv(raw):
    value=0xcbf29ce484222325
    for byte in raw: value=((value^byte)*0x100000001b3)&0xffffffffffffffff
    return value
class Ring:
    def __init__(self,path):
        self.data=open(path,"rb").read()
        data=self.data
        self.ok=len(data)>=RING["headerBytes"] and data[:4]==RING["magic"].encode()
        if not self.ok: return
        version,self.index,self.bucket,self.buckets,self.metrics,ids_bytes,self.row,self.offset=struct.unpack_from("<HHIIIIII",data,4)
        self.digest,self.head_stamp=struct.unpack_from("<QQ",data,32)
        self.ok=version==RING["version"] and self.row==2*STAMP+self.metrics*SLOT and self.bucket>0 and self.buckets>0 and len(data)==self.offset+self.buckets*self.row
        if not self.ok: return
        raw=data[RING["headerBytes"]:RING["headerBytes"]+ids_bytes]
        self.ids=raw.decode("utf8").split("\n") if self.metrics else []
        self.ok=len(self.ids)==self.metrics and fnv(raw)==self.digest
    def start_at(self,index):
        at=self.offset+index*self.row
        head=struct.unpack_from("<Q",self.data,at)[0]
        tail=struct.unpack_from("<Q",self.data,at+self.row-STAMP)[0]
        if head<=0 or head!=tail or head%self.bucket!=0 or (head//self.bucket)%self.buckets!=index: return 0
        return head
    def row_at(self,start):
        if start<=0 or start%self.bucket: return None
        index=(start//self.bucket)%self.buckets
        return self.slots(index) if self.start_at(index)==start else None
    def slots(self,index):
        at=self.offset+index*self.row+STAMP
        out=[]
        for metric in range(self.metrics):
            count,low,high,total,newest=struct.unpack_from("<Idddd",self.data,at+metric*SLOT)
            out.append([count,low,high,total,newest])
        return out
    def head(self):
        head=self.head_stamp
        if head==0 or self.row_at(head) is None:
            return max([self.start_at(index) for index in range(self.buckets)]+[0])
        for _ in range(self.buckets):
            if self.row_at(head+self.bucket) is None: break
            head+=self.bucket
        return head
    def rows(self):
        found=[(self.start_at(index),index) for index in range(self.buckets)]
        horizon=self.head()+self.bucket-self.buckets*self.bucket
        return sorted((start,self.slots(index)) for start,index in found if start>0 and start>=horizon)
def series(directory,start,end):
    tiers=[]
    for index in range(RING["maximumTiers"]):
        path=os.path.join(directory,"t%d.ring"%index)
        if not os.path.exists(path): continue
        ring=Ring(path)
        if not ring.ok or ring.index!=index or (tiers and ring.digest!=tiers[0].digest): continue
        tiers.append(ring)
    if not tiers: return []
    ids=tiers[0].ids
    boundary=end
    covered=False
    chunks=[]
    for ring in tiers:
        width=ring.bucket
        head=ring.head()
        if head==0: continue
        horizon=head+width-ring.buckets*width
        low=max(start//width*width,horizon)
        straddle=boundary//width*width if covered and boundary%width else -1
        high=min(head,straddle if straddle>=0 else (boundary-width)//width*width)
        rows=[]
        first=low
        while first<=high:
            slots=ring.row_at(first)
            if slots is not None: rows.append((first,slots,boundary-first if first==straddle else width,1 if first==straddle else 0))
            first+=width
        if rows:
            chunks.append((ring.index,width,rows))
            boundary=min(boundary,rows[0][0])
            covered=True
    lines=[]
    for column,metric in enumerate(ids):
        for index,width,rows in reversed(chunks):
            for first,slots,visible,partial in rows:
                count,low,high,total,newest=slots[column]
                if count>0: lines.append("\t".join([metric,num(first),num(visible),str(index),str(count),num(low),num(high),num(total),num(newest),str(partial)]))
    return lines
if __name__=="__main__":
    mode=sys.argv[1]
    if mode=="fold": print(json.dumps(fold(json.load(sys.stdin))))
    elif mode=="delta": print(json.dumps(deltas(json.load(sys.stdin))))
    elif mode=="series": print("\n".join(series(sys.argv[2],int(sys.argv[3]),int(sys.argv[4]))))
    elif mode=="rows":
        ring=Ring(sys.argv[2]); print(json.dumps([{"start":start,"slots":slots} for start,slots in ring.rows()]))
    elif mode=="platforms":
        print(json.dumps({row["id"]:row.get("platforms") for row in json.load(open(sys.argv[2],encoding="utf8"))["metrics"]}))
