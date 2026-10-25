import hmac,json,re,sys
MAXIMUM_AGE=120000
def decide(facts):
    match=re.fullmatch(r"([A-Za-z0-9_-]{8,32})\.([A-Za-z0-9_-]{22})",facts["offered"])
    if not match:return "token"
    ident,token=match.groups()
    grant=facts["grant"]
    if grant is None or not facts["grantCurrent"] or grant["id"]!=ident:return "gone"
    offer=grant.get("handover")
    if offer is None or not hmac.compare_digest(token.encode(),offer["token"].encode()):return "token"
    if facts["nowMs"]-offer["offeredAtMs"]>MAXIMUM_AGE:return "expired"
    parent=facts["parent"]
    if parent is None or parent["pid"]!=offer["fromPid"] or parent["started"]!=offer["fromStarted"] or grant["pid"]!=offer["fromPid"]:return "identity"
    if grant["operation"]!=facts["operation"]:return "operation"
    return "adopt"
rows=json.load(open(sys.argv[1],encoding="utf8"))["cases"]
print(json.dumps({row["id"]:decide(row["input"]) for row in rows},ensure_ascii=False))
