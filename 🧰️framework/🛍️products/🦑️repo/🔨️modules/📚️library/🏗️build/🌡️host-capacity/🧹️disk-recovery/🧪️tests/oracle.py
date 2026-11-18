import json,sys
cases=json.load(sys.stdin)
largest,divisor,factor,minimum=cases["budget"]["largest"],cases["budget"]["divisor"],cases["budget"]["factor"],cases["budget"]["minimum"]
out=[]
for case in cases["cases"]:
    total,free,need=case["total"],case["free"],case["need"]
    reserve=largest*factor;hyst=largest;cushion=-(-largest//divisor)
    derivable=total>=(reserve+hyst)*minimum
    if not derivable:
        out.append({"action":"fixed-shares"});continue
    must=need+cushion;target=need+reserve
    action="collect-now" if free<must else ("collect-background" if free<target else "allow")
    out.append({"action":action,"must":must,"target":target,"budget":max(0,min(total,case["footprint"]+free)-reserve)})
json.dump(out,sys.stdout)
