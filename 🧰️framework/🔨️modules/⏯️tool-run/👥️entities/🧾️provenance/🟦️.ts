/** 🧾️ Logical original provenance candidate with one comparison or shift per admitted turn. */
export type EntityMark={end:number,entity:string};
type Insertion<T>={value:T,index:number,shift:number,phase:number};
export class ToolRunEntityProvenanceEdit {
 private marks:EntityMark[]=[];
 private entities:string[]=[];
 private insertion:Insertion<EntityMark>|undefined;
 private entityInsertion:Insertion<string>|undefined;
 private index=0;
 private phase=0;
 private cancelled=false;
 private spent=false;
 constructor(readonly original:readonly EntityMark[],private readonly retractTo:number|null,private readonly end:number,private readonly append:readonly string[]){}
 get complete(){return this.phase===3&&!this.cancelled&&!this.spent;}
 get retained(){return this.marks.length>0||this.entities.length>0||this.insertion!==undefined||this.entityInsertion!==undefined||this.phase!==4;}
 cancel(){this.cancelled=true;}
 private insert<T>(array:T[],state:Insertion<T>,compare:(a:T,b:T)=>number){
  if(state.phase===0){if(state.index<array.length){const order=compare(state.value,array[state.index]!);if(order===0)return true;if(order>0){state.index++;return false;}}array.push(state.value);state.shift=array.length-1;state.phase=1;return false;}
  if(state.shift>state.index){array[state.shift]=array[state.shift-1]!;state.shift--;return false;}array[state.index]=state.value;return true;
 }
 advance(items:number){
  if(items<=0||this.cancelled||this.phase>=3)return 0;
  if(this.insertion){if(this.insert(this.marks,this.insertion,(a,b)=>a.end-b.end||(BigInt(a.entity)<BigInt(b.entity)?-1:BigInt(a.entity)>BigInt(b.entity)?1:0)))this.insertion=undefined;return 1;}
  if(this.entityInsertion){if(this.insert(this.entities,this.entityInsertion,(a,b)=>BigInt(a)<BigInt(b)?-1:BigInt(a)>BigInt(b)?1:0))this.entityInsertion=undefined;return 1;}
  if(this.phase===0){if(this.index===this.original.length){this.index=0;this.phase=1;return 1;}const mark=this.original[this.index++]!;if(this.retractTo===null||mark.end<=this.retractTo)this.insertion={value:mark,index:0,shift:0,phase:0};return 1;}
  if(this.phase===1){if(this.index===this.append.length){this.index=0;this.phase=2;return 1;}this.insertion={value:{end:this.end,entity:this.append[this.index++]!},index:0,shift:0,phase:0};return 1;}
  if(this.index===this.marks.length){this.phase=3;return 1;}this.entityInsertion={value:this.marks[this.index++]!.entity,index:0,shift:0,phase:0};return 1;
 }
 take(items:number){if(items<=0||!this.complete)return undefined;const result={marks:this.marks,entities:this.entities};this.marks=[];this.entities=[];this.spent=true;return result;}
 close(items:number){if(items<=0||!this.retained)return 0;if(this.insertion){this.insertion=undefined;return 1;}if(this.entityInsertion){this.entityInsertion=undefined;return 1;}if(this.marks.length){this.marks.pop();return 1;}if(this.entities.length){this.entities.pop();return 1;}this.phase=4;return 1;}
}
