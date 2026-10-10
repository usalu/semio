/** 📸️ Immutable logical publication retains the original version while a private candidate advances. */
import {ToolRunEntityEdit} from "../🟦️.ts";

export class ToolRunEntityVersion {
 constructor(readonly revision:number,readonly entities:readonly string[]){}
}
export class ToolRunEntityVersionEdit {
 private copied:string[]=[];
 private edit:ToolRunEntityEdit|undefined;
 private closing=false;
 private spent=false;
 private owned=true;
 constructor(private original:ToolRunEntityVersion|undefined,private targets:string[],private readonly kind:"append"|"retract"){}
 get complete():boolean{return !this.closing&&!this.spent&&this.edit?.complete===true;}
 get retained():boolean{return this.owned;}
 advance(maximumItems:number):number{
  if(maximumItems<1||this.closing||this.spent||this.complete)return 0;
  if(this.edit)return this.edit.advance(maximumItems);
  if(this.copied.length<this.original!.entities.length){this.copied.push(this.original!.entities[this.copied.length]!);return 1;}
  this.edit=new ToolRunEntityEdit(this.copied,this.targets,this.kind);this.copied=[];this.targets=[];return 1;
 }
 take(maximumItems:number):ToolRunEntityVersion|undefined{if(maximumItems<1||!this.complete)return;const entities=this.edit!.take()!;this.spent=true;return new ToolRunEntityVersion(this.original!.revision+1,entities);}
 cancel():void{this.closing=true;this.edit?.cancel();}
 close(maximumItems:number):number{
  if(maximumItems<1||!this.owned)return 0;
  this.cancel();
  if(this.edit?.retained)return this.edit.close(maximumItems);
  if(this.copied.length)this.copied.pop();else if(this.targets.length)this.targets.pop();else if(this.original)this.original=undefined;else{this.edit=undefined;this.owned=false;}
  return 1;
 }
}
