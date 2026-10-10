/** 👥️ Logical twin of the owned native entity workspace; physical allocation receipts belong to its native issuer. */
export class ToolRunEntityEdit {
 private phase=0;
 private ordinal=0;
 private low=0;
 private high=0;
 private slot=0;
 private shift=0;
 private found=false;
 private closing=false;
 private owned=true;
 constructor(private entities:string[],private targets:string[],private readonly kind:"append"|"retract"){}
 get complete():boolean{return this.phase===4&&!this.closing;}
 get retained():boolean{return this.owned;}
 advance(maximumItems:number):number{
  if(this.closing||this.complete||maximumItems<1)return 0;
  if(this.phase===0){if(this.ordinal===this.targets.length){this.phase=4;return 1;}this.low=0;this.high=this.entities.length;this.found=false;this.phase=1;return 1;}
  const target=this.targets[this.ordinal]!;
  if(this.phase===1){if(this.low===this.high){this.slot=this.low;this.phase=2;return 1;}const middle=this.low+Math.floor((this.high-this.low)/2),value=BigInt(this.entities[middle]!),key=BigInt(target);if(value===key){this.slot=middle;this.found=true;this.phase=2;}else if(value<key)this.low=middle+1;else this.high=middle;return 1;}
  if(this.phase===2){if((this.kind==="append"&&this.found)||(this.kind==="retract"&&!this.found)){this.ordinal++;this.phase=0;return 1;}if(this.kind==="append"){this.shift=this.entities.length;this.entities.push(target);}else this.shift=this.slot;this.phase=3;return 1;}
  if(this.kind==="append"){if(this.shift>this.slot){this.entities[this.shift]=this.entities[this.shift-1]!;this.shift--;return 1;}this.entities[this.slot]=target;}else{if(this.shift+1<this.entities.length){this.entities[this.shift]=this.entities[this.shift+1]!;this.shift++;return 1;}this.entities.pop();}
  this.ordinal++;this.phase=0;return 1;
 }
 take():string[]|undefined{if(!this.complete)return;const original=this.entities;this.entities=[];return original;}
 cancel():void{this.closing=true;}
 close(maximumItems:number):number{if(!this.closing||!this.owned||maximumItems<1)return 0;if(this.entities.length)this.entities.pop();else if(this.targets.length)this.targets.pop();else this.owned=false;return 1;}
}
