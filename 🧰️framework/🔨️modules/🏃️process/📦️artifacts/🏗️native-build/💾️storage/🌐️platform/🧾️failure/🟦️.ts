import schema from"./🧬️schema/🔣️.json" with{type:"json"};

type RecoveredChildFactsV1=Readonly<{allocationId:string;phase:string;kind:string;child:boolean;backingRoot:string;policyRoot:string;parentRoot:string}>;

/** 🧾️ Keeps the primary retirement refusal and bounds its content-free custody predicate evidence. */
export function nativeRecoveredChildRetirementFailureV1(input:RecoveredChildFactsV1):Error{
 const names=schema.properties.truncated.items.enum as readonly(keyof Omit<RecoveredChildFactsV1,"child">)[],limit=schema.properties.allocationId.maxLength,truncated=names.filter(name=>input[name].length>limit),fields=Object.fromEntries(names.map(name=>[name,input[name].slice(0,limit)])),checks={child:input.child,committed:input.phase==="committed",compilerState:input.kind==="compiler-state",backingRoot:input.backingRoot===input.parentRoot,policyRoot:input.policyRoot===input.parentRoot};
 return Error("Actual recovered committed child required",{cause:{version:1,purpose:"native-recovered-child-retirement",...fields,checks,truncated}});
}
