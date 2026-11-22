import json,sys
from jsonschema import Draft7Validator
document=json.load(open(sys.argv[1],encoding="utf8"))
definitions=document["definitions"]
validator=Draft7Validator({"$ref":"#/definitions/Class","definitions":definitions})
problems=[]
for name,value in definitions["Classes"]["const"].items():
    problems+=[name+": "+error.message for error in validator.iter_errors(value)]
roles=definitions["Roles"]["const"];services=definitions["Services"]["const"];classes=definitions["Classes"]["const"]
problems+=["role "+role+" names no class" for role,name in roles.items() if name not in classes]
problems+=["service "+service+" names no class" for service,name in services.items() if name not in classes]
print(json.dumps({"problems":problems,"roles":roles,"services":services}))
