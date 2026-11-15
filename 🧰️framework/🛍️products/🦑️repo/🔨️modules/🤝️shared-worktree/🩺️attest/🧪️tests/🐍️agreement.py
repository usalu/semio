"""Independent Python implementation of the checkpoint agreement (buckets, gaps, comparison, verdict).
Shares no code with the TypeScript module; imported by the oracle 🐍️.py of the attest member (copied there) and by the fixture generator."""
import hashlib,unicodedata

BUCKETS=256
FRESH=60
SKEW=30
REALM_BASE=".cache/f/index-base"
REALM_ENTRIES=".cache/f/index/"

def bucket_of(path):
    h=0x811c9dc5
    for byte in unicodedata.normalize("NFC",path).encode("utf-8"):
        h^=byte
        h=(h*0x01000193)&0xffffffff
    return h&(BUCKETS-1)

def fold_bucket(buckets,bucket,term):
    for index in range(2): buckets[bucket*2+index]^=term[index]

def manifest_term(entry):
    path=entry["path"]
    if entry["operation"]=="delete" or path==REALM_BASE or path.startswith(REALM_ENTRIES): return None
    return hashlib.sha256((path+"\0"+entry["operation"]+"\0"+entry["value"]).encode("utf-8")).digest()

def manifest_buckets(entries):
    buckets=bytearray(BUCKETS*2)
    for entry in entries:
        term=manifest_term(entry)
        if term is not None: fold_bucket(buckets,bucket_of(entry["path"]),term)
    return buckets.hex()

def view_buckets(lines):
    buckets=bytearray(BUCKETS*2)
    for line in lines: fold_bucket(buckets,bucket_of(line[3:]),hashlib.sha256(line.encode("utf-8")).digest())
    return buckets.hex()

def mask_of(buckets):
    value=0
    for bucket in buckets: value|=1<<bucket
    return "%064x"%value

def mask_buckets(mask):
    value=int(mask,16)
    return set(bucket for bucket in range(BUCKETS) if (value>>bucket)&1)

def git_path(path):
    if path==REALM_BASE or path.startswith(REALM_ENTRIES): return True
    if path in (".git/HEAD",".git/packed-refs",".git/shallow") or path.startswith(".git/refs/heads/"): return True
    return path.rsplit("/",1)[-1] in (".gitignore",".gitattributes",".gitmodules")

def before(left,right): return (left["segment"],left["bytes"])<(right["segment"],right["bytes"])
ZERO={"segment":0,"bytes":0}

def span_of(checkpoint): return (checkpoint["cursors"],checkpoint.get("until",checkpoint["cursors"]))

def gap_between(left,right,windows):
    buckets=set()
    events=0
    git=False
    origins=set(left[0])|set(left[1])|set(right[0])|set(right[1])
    for origin in sorted(origins):
        lows=[left[0].get(origin,ZERO),right[0].get(origin,ZERO)]
        highs=[left[1].get(origin,ZERO),right[1].get(origin,ZERO)]
        low=lows[0] if before(lows[0],lows[1]) else lows[1]
        high=highs[1] if before(highs[0],highs[1]) else highs[0]
        if not before(low,high): continue
        window=windows.get(origin)
        if window is None or before(low,window["from"]) or before(window["to"],high): return None
        for event in window["events"]:
            if before(low,event["end"]) and not before(high,event["end"]):
                events+=1
                buckets.add(bucket_of(event["path"]))
                if git_path(event["path"]): git=True
    return {"events":events,"buckets":buckets,"git":git}

def oldest_unseen(cut,heads,windows):
    oldest=None
    for origin,head in heads.items():
        at=cut.get(origin,ZERO)
        if not before(at,head): continue
        window=windows.get(origin)
        if window is None or before(at,window["from"]) or before(window["to"],head): return "undefined"
        for event in window["events"]:
            if before(at,event["end"]) and not before(head,event["end"]):
                oldest=event["ms"] if oldest is None else min(oldest,event["ms"])
    return oldest

def recent(now,wall):
    raw=(now-wall)//1000
    return raw<=FRESH and raw>=-SKEW

def slice_of(buckets,bucket): return buckets[bucket*4:bucket*4+4]

COMPARED=["manifest","head.commit","indexTree","view.digest"]

def spell(bucket,value): return value if bucket<0 else "#%d=%s"%(bucket,value)

def spread(items,field,skip,one,two):
    for bucket in range(BUCKETS):
        if bucket in skip: continue
        items.append({"field":field,"bucket":bucket,"left":slice_of(one,bucket),"right":slice_of(two,bucket),"same":slice_of(one,bucket)==slice_of(two,bucket)})

def compare(left,right,now,windows):
    items=[]
    known=set()
    one,two=left.get("manifest"),right.get("manifest")
    if one and two and recent(now,one["wallMs"]) and recent(now,two["wallMs"]):
        gap=gap_between(span_of(one),span_of(two),windows)
        if gap is not None and (gap["events"]==0 or len(gap["buckets"])<BUCKETS):
            known.add("manifest")
            spread(items,"manifest",gap["buckets"],one["buckets"],two["buckets"])
    first,second=left.get("git"),right.get("git")
    if first and second and recent(now,first["wallMs"]) and recent(now,second["wallMs"]):
        gap=gap_between(span_of(first),span_of(second),windows)
        if gap is not None and not gap["git"]:
            known.add("git")
            items.append({"field":"head.commit","bucket":-1,"left":first["head"] or "-","right":second["head"] or "-","same":first["head"]==second["head"]})
            items.append({"field":"indexTree","bucket":-1,"left":first["indexTree"] or "-","right":second["indexTree"] or "-","same":first["indexTree"]==second["indexTree"]})
    above,below=left.get("view"),right.get("view")
    if above is None or below is None: known.add("view")
    elif recent(now,above["wallMs"]) and recent(now,below["wallMs"]):
        gap=gap_between(span_of(above),span_of(below),windows)
        if gap is not None and not gap["git"]:
            skip=set(gap["buckets"])|mask_buckets(above["pending"])|mask_buckets(below["pending"])
            if len(skip)<BUCKETS:
                known.add("view")
                spread(items,"view.digest",skip,above["buckets"],below["buckets"])
    return {"items":items,"known":known}
