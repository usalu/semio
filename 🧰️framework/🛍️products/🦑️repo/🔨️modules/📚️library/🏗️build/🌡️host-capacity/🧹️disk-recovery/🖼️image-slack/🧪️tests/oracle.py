import json,sys
doc=json.load(sys.stdin)
p=doc["policy"];need=doc["need"];out=[]
rows=sorted(doc["rows"],key=lambda r:(-r["slackBytes"],r["id"]))
for r in rows:
    floor=max(p["minimumSlackBytes"],-(-r["allocatedBytes"]*p["minimumSlackPercent"]//100))
    if r["id"] not in p["regrowProfiles"]:v="not-elastic"
    elif not r["mounted"]:v="unmounted"
    elif r["slackBytes"]<floor:v="below-floor"
    elif need<=0:v="not-needed"
    else:v="slack";need-=r["slackBytes"]
    out.append({"id":r["id"],"action":"retire" if v=="slack" else "skip","reason":v})
json.dump(out,sys.stdout)
