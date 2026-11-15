"""Independent Python implementation of the separation of outliers among workers that differ in one compared component.
Shares no code with the TypeScript module; reads a JSON list of cases from stdin and writes the verdict of each case."""
import json,sys

def separate(apart,alike,pointed):
    victims=[];ambiguous=[];removed=set()
    rest=list(apart)
    while rest:
        score={}
        for one,two in rest:
            score[one]=score.get(one,0)+1
            score[two]=score.get(two,0)+1
        for one,two in alike:
            if one in removed or two in removed: continue
            if one in score: score[one]-=1
            if two in score: score[two]-=1
        most=None;tied=[]
        for origin in sorted(score):
            if most is None or score[origin]>most: most=score[origin];tied=[origin]
            elif score[origin]==most: tied.append(origin)
        if len(tied)>1 and pointed>=3:
            kept=[pair for pair in rest if not (pair[0] in tied and pair[1] in tied)]
            if len(kept)<len(rest):
                for origin in tied:
                    if origin not in ambiguous: ambiguous.append(origin)
                rest=kept
                continue
        victim=tied[-1]
        first=next(pair for pair in rest if pair[0]==victim or pair[1]==victim)
        victims.append([victim,list(first)])
        removed.add(victim)
        rest=[pair for pair in rest if pair[0]!=victim and pair[1]!=victim]
    return {"victims":victims,"ambiguous":ambiguous}

if __name__=="__main__":
    cases=json.load(sys.stdin)
    json.dump([separate(case["apart"],case["alike"],case["pointed"]) for case in cases],sys.stdout)
