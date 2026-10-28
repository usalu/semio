import type {ExpressValue} from './evaluator.ts';
/** 🪟 A procedural read owns accessed containers while retaining entity-reference aliases. */
export class ExpressReadScope {
 readonly values=new WeakMap<ExpressValue,ExpressValue>();
 readonly #pending=new WeakMap<ExpressValue,Promise<ExpressValue>>();
 constructor(readonly tick:()=>Promise<void>){}
 async read(value:ExpressValue):Promise<ExpressValue>{
  if(value.kind!=='entity'&&value.kind!=='aggregate')return value;
  const existing=this.values.get(value);if(existing)return existing;
  const pending=this.#pending.get(value);if(pending)return pending;
  const copy=this.copy(value);this.#pending.set(value,copy);try{const local=await copy;this.values.set(value,local);this.values.set(local,local);return local;}finally{this.#pending.delete(value);}
 }
 private async copy(value:ExpressValue):Promise<ExpressValue>{
  await this.tick();const local:ExpressValue={...value};
  if(value.kind==='aggregate'){const result:ExpressValue[]=[];local.value=result;for(const item of value.value as ExpressValue[]){await this.tick();result.push(item);}}
  else{
   const maps=new Map<Map<string,ExpressValue>,Map<string,ExpressValue>>();
   const copy=async(source:Map<string,ExpressValue>)=>{const existing=maps.get(source);if(existing)return existing;const result=new Map<string,ExpressValue>();maps.set(source,result);for(const [name,item] of source){await this.tick();result.set(name,item);}return result;};
   if(value.fields)local.fields=await copy(value.fields);
   if(value.groups){local.groups=new Map();for(const [name,group] of value.groups){await this.tick();local.groups.set(name,await copy(group));}}
  }
  return local;
 }
 /** 📤 Publish computed local links without traversing untouched source references. */
 async publish(value:ExpressValue):Promise<ExpressValue>{
  const result=this.values.get(value)??value,queue=[result],seen=new WeakSet<ExpressValue>();
  while(queue.length){await this.tick();const current=queue.pop()!;if(seen.has(current)||this.values.get(current)!==current)continue;seen.add(current);
   const resolve=(item:ExpressValue)=>{const local=this.values.get(item)??item;if(this.values.get(local)===local&&!seen.has(local))queue.push(local);return local;};
   if(current.kind==='aggregate'){const items=current.value as ExpressValue[];for(let index=0;index<items.length;index++){await this.tick();items[index]=resolve(items[index]!);}}
   else if(current.kind==='entity'){if(current.fields)for(const [name,item] of current.fields){await this.tick();current.fields.set(name,resolve(item));}if(current.groups)for(const group of current.groups.values()){await this.tick();for(const [name,item] of group){await this.tick();group.set(name,resolve(item));}}}
  }
  return result;
 }
}
