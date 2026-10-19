import{PersistentListBuilder}from"../../../📋️list/🔗️persistent/📥️build/🟦️.ts";
type Row={label:string;rank:number};
/** 🧳️ Typed original array members enter fixed pages through separate validation and adoption turns. */
export class NativePersistentListReceiving{
 #original:unknown;#rows:readonly unknown[]|null=null;#builder:PersistentListBuilder<Row>|null=null;#row:Row|null=null;#index=0;#phase:"bind"|"row"|"adopt"|"build"|"finish"|"ready"|"spent"|"closed"="bind";
 constructor(original:unknown){this.#original=original;}
 get ready():boolean{return this.#phase==="ready";}
 get empty():boolean{return this.#phase==="closed"&&this.#rows===null&&this.#builder===null&&this.#row===null&&this.#original===null;}
 advance():void{
  if(this.#phase==="bind"){if(!Array.isArray(this.#original))throw Error("original native list source is not an array");this.#rows=this.#original;this.#builder=new PersistentListBuilder<Row>(this.#rows.length);this.#phase=this.#rows.length?"row":"finish";return;}
  const builder=this.#builder;if(!builder||!this.#rows)throw Error("original native list receiving is closed or transferred");
  if(this.#phase==="row"){const row=this.#rows[this.#index];if(!row||typeof row!=="object"||Array.isArray(row))throw Error("original native row has a different type");const fields=row as Record<string,unknown>;if(Object.keys(fields).length!==2||typeof fields.label!=="string"||typeof fields.rank!=="number"||!Number.isSafeInteger(fields.rank))throw Error("original native row has different declared fields");this.#row={label:fields.label,rank:fields.rank};this.#phase="adopt";return;}
  if(this.#phase==="adopt"){if(!this.#row)throw Error("original native row has no retained output");builder.adopt(this.#row);this.#row=null;this.#index++;this.#phase=!builder.acceptsRow?"build":this.#index<this.#rows.length?"row":"finish";return;}
  if(this.#phase==="build"){builder.advance();if(builder.acceptsRow)this.#phase=this.#index<this.#rows.length?"row":"finish";else if(builder.ready)this.#phase="ready";return;}
  if(this.#phase==="finish"){builder.finish();this.#phase="build";return;}
  throw Error("original native list has no pending receiving turn");
 }
 take():ReturnType<PersistentListBuilder<Row>["take"]>{if(!this.ready||!this.#builder)throw Error("original native list output is not ready");this.#phase="spent";return this.#builder.take();}
 close():void{this.#builder?.close();this.#builder=null;this.#row=null;this.#rows=null;this.#original=null;this.#phase="closed";}
}
