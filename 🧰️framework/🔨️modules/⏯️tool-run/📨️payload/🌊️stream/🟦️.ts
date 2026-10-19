/** 🌊️ Closed publication ranges and bounded original-page reads are independent of payload content. */
export type PayloadUpdate = {readonly kind:"replace"}|{readonly kind:"range";readonly publication:bigint;readonly totalBytes:bigint;readonly offset:bigint;readonly chunkBytes:number;readonly final:boolean};
export interface PayloadIndexedSource{readonly identity:string;readonly publication:bigint;readonly totalBytes:number;readonly receivedBytes:number;readonly chunkBytes:number;readonly pageCount:number;page(index:number):Uint8Array|undefined}
export function validatePayloadUpdate(update:PayloadUpdate,length:number):void{
 if(!Number.isSafeInteger(length)||length<0)throw Error("invalid-payload-length");
 if(update.kind==="replace")return;
 if(!Number.isSafeInteger(update.chunkBytes)||update.chunkBytes<1||update.chunkBytes>65536)throw Error("invalid-payload-range");
 const stride=BigInt(update.chunkBytes),end=update.offset+BigInt(length);
 if(update.publication<1n||update.totalBytes<1n||update.offset<0n||update.offset%stride!==0n||length<1||length>update.chunkBytes||end>update.totalBytes||update.final!==(end===update.totalBytes)||!update.final&&length!==update.chunkBytes)throw Error("invalid-payload-range");
}
export function validatePayloadContinuation(previous:PayloadIndexedSource|undefined,identity:string,update:PayloadUpdate,length:number):void{
 validatePayloadUpdate(update,length);if(update.kind==="replace")return;
 if(update.offset===0n){if(previous?.identity===identity&&update.publication<=previous.publication)throw Error("replayed-payload-publication");return;}
 if(!previous||previous.identity!==identity||previous.publication!==update.publication||BigInt(previous.totalBytes)!==update.totalBytes||previous.chunkBytes!==update.chunkBytes||BigInt(previous.receivedBytes)!==update.offset||previous.receivedBytes===previous.totalBytes)throw Error("foreign-or-unordered-payload-continuation");
}
/** 🔎️ A fixed read visits original indexed pages without a full decode, flatten or clone. */
export class PayloadReadView{
 readonly source:PayloadIndexedSource;reads=0;copiedBytes=0;
 constructor(source:PayloadIndexedSource,identity:string){this.source=source;if(source.identity!==identity)throw Error("foreign-payload-generation");if(!Number.isSafeInteger(source.totalBytes)||!Number.isSafeInteger(source.receivedBytes)||source.receivedBytes<0||source.receivedBytes>source.totalBytes||!Number.isSafeInteger(source.chunkBytes)||source.chunkBytes<1)throw Error("invalid-payload-directory");}
 get complete():boolean{return this.source.receivedBytes===this.source.totalBytes;}
 get totalBytes():number{return this.source.totalBytes;}
 read(offset:number,count:number):Uint8Array|undefined{
  if(!Number.isSafeInteger(offset)||!Number.isSafeInteger(count)||offset<0||count<0||count>64||offset+count>this.source.receivedBytes)return undefined;
  const result=new Uint8Array(count);let copied=0;
  while(copied<count){const position=offset+copied,page=this.source.page(Math.floor(position/this.source.chunkBytes));if(!page)return undefined;const start=position%this.source.chunkBytes,length=Math.min(count-copied,page.length-start);if(length<1)return undefined;result.set(page.subarray(start,start+length),copied);copied+=length;}
  this.reads++;this.copiedBytes+=count;return result;
 }
}
