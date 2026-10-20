"""Independent model of the settlement repeat law (stdlib only): reads the fixture, replays the decision table without the JavaScript implementation and prints the expected outcome of every scenario as JSON."""
import json,sys

def model(scenario,limit):
    previous=None
    events=[]
    for index,row in enumerate(scenario["rounds"]):
        outcome=row["outcome"]
        if outcome=="value":
            return {"settled":"value","rounds":index+1,"events":events}
        if outcome=="failure":
            signature=(row.get("code"),None if row.get("undefined") else row.get("message"))
            if row.get("code")=="BUILD_AUTHORITY_CHANGED" or signature==previous:
                return {"settled":"failure","rounds":index+1,"failure":signature[1],"events":events}
            if row.get("holds",True) and not row.get("young",True):
                return {"settled":"failure","rounds":index+1,"failure":signature[1],"events":events}
            previous=signature
            events.append("failure")
        else:
            previous=None
            events.append("conflict")
        if index+1>=limit:
            if outcome=="failure":
                return {"settled":"failure","rounds":index+1,"failure":signature[1],"events":events}
            return {"settled":"conflict","rounds":index+1,"events":events}
    raise SystemExit("scenario ends without a terminal round: "+scenario["name"])

if __name__=="__main__":
    document=json.load(open(sys.argv[1],encoding="utf8"))
    print(json.dumps({s["name"]:model(s,document["limit"]) for s in document["scenarios"]},sort_keys=True))
