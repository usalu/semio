import{SolidValueWorkspace,type SolidValueReservation}from'./workspace.ts';
/** 🏕️ Entered numeric backings stay owned at their logical high water until kernel retirement. */
export class SolidArena{
 readonly #backings=new Map<object,number>();readonly #lease:SolidValueReservation;#closed=false;
 constructor(readonly workspace:SolidValueWorkspace,readonly borrowed=false){this.#lease=workspace.reserve(0);}
 #grow(values:number):void{if(this.#closed)throw Error('boolean-workspace-closed');this.workspace.grow(this.#lease,values);}
 born<T extends object>(values:number,make:()=>T):T{this.#grow(values);const value=make();this.#backings.set(value,values);return value;}
 borrow(value:object,values:number):void{if(this.borrowed&&!this.#backings.has(value)){this.#grow(values);this.#backings.set(value,values);}}
 push<T>(values:T[],value:T):void{const size=this.#backings.get(values)??0,next=Math.max(size,values.length+1);this.#grow(next-size);this.#backings.set(values,next);values.push(value);}
 entry<K,V>(map:Map<K,V>,key:K,value:V,width:number):void{if(!map.has(key)){this.#grow(width);this.#backings.set(map,(this.#backings.get(map)??0)+width);}map.set(key,value);}
 member(set:Set<number>,value:number):void{if(!set.has(value)){this.#grow(1);this.#backings.set(set,(this.#backings.get(set)??0)+1);}set.add(value);}
 *takeSteps(groups:object[][]):Generator<void,SolidValueReservation,void>{const seen=new Set<object>();let count=0;for(const rows of groups)for(const row of rows){yield;if(seen.has(row))continue;seen.add(row);const capacity=this.#backings.get(row);if(capacity===undefined)throw Error('boolean-workspace-backing');count+=capacity;}return this.workspace.split(this.#lease,count);}
 close():void{if(this.#closed)return;this.#closed=true;this.#backings.clear();this.workspace.release(this.#lease);}
}
