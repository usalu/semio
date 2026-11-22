import json,sys
policy=json.load(open(sys.argv[1]))["definitions"]["Policy"]["const"]
rows=[]
for hold in json.loads(sys.argv[2]):
    for phase in [None,"reading","executing"]:
        for age in json.loads(sys.argv[3]):
            for quiet in json.loads(sys.argv[4]):
                if hold>=policy["holderWaitRestartMilliseconds"]:
                    out="holder"
                elif phase!="executing" and age>=policy["staleStageMilliseconds"] and quiet>=policy["codeQuietMilliseconds"]:
                    out="stale"
                else:
                    out=None
                rows.append([hold,phase,age,quiet,out])
print(json.dumps(rows))
