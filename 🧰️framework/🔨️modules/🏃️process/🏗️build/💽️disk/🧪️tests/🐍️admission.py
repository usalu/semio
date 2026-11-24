"""💽️ Independent oracle for the upfront disk admission: the need and the missing bytes of every case of the shared corpus, with the standard library only."""
import json,math,sys
def decide(c):
    plan=max(c["minimumBytes"],-(-c["plannedBytes"]*c["planShareBasisPoints"]//10000))
    credit=min(plan,max(0,c["slackBytes"])*c["slackCreditBasisPoints"]//10000)
    need=plan-credit
    return [plan,credit,need,max(0,need+c["reserveBytes"]-c["hostFreeBytes"])]
print(json.dumps([decide(c) for c in json.load(open(sys.argv[1]))["admission"]]))
