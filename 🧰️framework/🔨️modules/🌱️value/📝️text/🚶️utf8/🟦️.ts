export type Utf8Grant={maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number};
export type Utf8Progress={copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number};
const empty=():Utf8Progress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
function valid(grant:Utf8Grant):void{if(Object.values(grant).some(value=>!Number.isSafeInteger(value)||value<0))throw Error("utf8-grant-invalid");}
function boundary(bytes:Uint8Array,index:number):boolean{return index===bytes.length||index>=0&&(bytes[index]!&192)!==128;}
function width(bytes:Uint8Array,index:number):number{const lead=bytes[index]!;let count=lead<128?1:lead>=194&&lead<224?2:lead>=224&&lead<240?3:lead>=240&&lead<245?4:0;if(!count||index+count>bytes.length)throw Error("utf8-source-invalid");for(let at=1;at<count;at++)if((bytes[index+at]!&192)!==128)throw Error("utf8-source-invalid");if(count===3&&(lead===224&&bytes[index+1]!<160||lead===237&&bytes[index+1]!>=160)||count===4&&(lead===240&&bytes[index+1]!<144||lead===244&&bytes[index+1]!>=144))throw Error("utf8-source-invalid");return count;}
function page(bytes:Uint8Array,start:number,allowance:number):number{if(!Number.isSafeInteger(start)||start<0||start>bytes.length||!boundary(bytes,start))throw Error("utf8-source-boundary");let end=start+Math.min(bytes.length-start,Math.max(0,allowance));while(end>start&&!boundary(bytes,end))end--;return end;}
/** 🚶️ The caller keeps original source/version custody while scalar pages retain only fixed positions. */
export class RetainedUtf8Cursor{
 #byte=0;#scalar=0;#ready=false;readonly fixedCopyBytes:number;
 constructor(wordBytes:4|8){this.fixedCopyBytes=wordBytes*8;}
 get ready():boolean{return this.#ready;}
 get byteOffset():number{return this.#byte;}
 get scalarOffset():number{return this.#scalar;}
 advance(source:Uint8Array,grant:Utf8Grant):{startByte:number;endByte:number;startScalar:number;endScalar:number;progress:Utf8Progress}{valid(grant);const result={startByte:this.#byte,endByte:this.#byte,startScalar:this.#scalar,endScalar:this.#scalar,progress:empty()};if(this.#ready)return result;const end=page(source,this.#byte,grant.maximumCopyBytes-this.fixedCopyBytes),minimum=this.fixedCopyBytes+(this.#byte===source.length?0:width(source,this.#byte));if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<minimum)return result;let count=0;for(let cursor=this.#byte;cursor<end;cursor+=width(source,cursor))count++;this.#byte=end;this.#scalar+=count;this.#ready=end===source.length;return{...result,endByte:end,endScalar:this.#scalar,progress:{...empty(),copiedItems:1,copiedBytes:this.fixedCopyBytes+end-result.startByte}};}
}
/** 🧳️ One declared original byte allocation is appended without hidden capacity growth. */
export class RetainedUtf8Buffer{
 #output:Uint8Array|undefined;#length=0;#spent=false;readonly #capacity:number;
 constructor(capacity:number){if(!Number.isSafeInteger(capacity)||capacity<0)throw Error("utf8-capacity-invalid");this.#capacity=capacity;}
 reserve(grant:Utf8Grant):Utf8Progress{valid(grant);if(this.#output||this.#spent)return empty();if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<64||grant.maximumCapacityBytes<this.#capacity)return empty();this.#output=new Uint8Array(this.#capacity);return{...empty(),copiedItems:1,copiedBytes:64,retainedCapacityBytes:this.#capacity};}
 append(source:Uint8Array,start:number,grant:Utf8Grant):{endByte:number;progress:Utf8Progress}{valid(grant);const output=this.#output;if(!output||this.#spent)throw Error("utf8-buffer-not-owned");const end=page(source,start,grant.maximumCopyBytes-64),minimum=64+(start===source.length?0:width(source,start));if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<minimum)return{endByte:start,progress:empty()};if(this.#length+end-start>output.length)throw Error("utf8-capacity-exceeded");for(let cursor=start;cursor<end;cursor+=width(source,cursor)){}output.set(source.subarray(start,end),this.#length);this.#length+=end-start;return{endByte:end,progress:{...empty(),copiedItems:1,copiedBytes:64+end-start}};}
 take(grant:Utf8Grant):{bytes:Uint8Array;length:number;progress:Utf8Progress}|undefined{valid(grant);if(!this.#output||this.#spent||grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<50)return;const bytes=this.#output;this.#output=undefined;this.#spent=true;return{bytes,length:this.#length,progress:{...empty(),copiedItems:1,copiedBytes:50}};}
 close(grant:Utf8Grant):Utf8Progress{valid(grant);if(!this.#output)return empty();if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<24||grant.maximumReleaseBytes<this.#output.length)return empty();const released=this.#output.length;this.#output=undefined;this.#spent=true;return{...empty(),copiedItems:1,copiedBytes:24,releasedBytes:released};}
}
