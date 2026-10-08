/** 🔗️ Complete internal graph admission for typed STEP snapshots. */
import type {StepSnapshot,StepValue} from "../🟦️.ts";

/** 🔗️ Semantic admission preserves original owners and returns the first exact native target. */
export function validateStepReferences(snapshot:StepSnapshot):{code:string;target:string[]}|null {
 const identities=new Set<bigint>();
 for(const entity of snapshot.entities){if(identities.has(entity.id))return {code:"mutation.apply.duplicate-entity-id",target:["entities",String(entity.id)]};identities.add(entity.id);}
 const values=(value:StepValue,target:string[]):{code:string;target:string[]}|null=>{
  const pending:{value:StepValue;target:string[]}[]=[{value,target}];
  while(pending.length){const {value,target}=pending.pop()!;if(typeof value!=="object")continue;
   if("reference" in value&&!identities.has(value.reference))return {code:"mutation.apply.dangling-reference",target};
   if("aggregate" in value)for(let i=value.aggregate.length-1;i>=0;i--)pending.push({value:value.aggregate[i]!,target:[...target,String(i)]});
   if("typedValue" in value)pending.push({value:value.typedValue.value,target:[...target,"value"]});
  }
  return null;
 };
 for(const entity of snapshot.entities){for(let i=0;i<entity.args.length;i++){const error=values(entity.args[i]!,["entities",String(entity.id),"args",String(i)]);if(error)return error;}for(let part=0;part<entity.complex.length;part++)for(let i=0;i<entity.complex[part]!.args.length;i++){const error=values(entity.complex[part]!.args[i]!,["entities",String(entity.id),"complex",String(part),"args",String(i)]);if(error)return error;}}
 return null;
}
