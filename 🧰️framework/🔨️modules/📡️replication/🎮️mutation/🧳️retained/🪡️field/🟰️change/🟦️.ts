export type FieldGrant={maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number};
export type FieldProgress={copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number};
export type ByteFieldSource=Readonly<{value:Uint8Array|null}>;
const idle=():FieldProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
const valid=(g:FieldGrant):void=>{const n=(v:number)=>Number.isSafeInteger(v)&&v>=0;if(!n(g.maximumItems)||!n(g.maximumCopyBytes)||!n(g.maximumCapacityBytes)||!n(g.maximumReleaseBytes)||!n(g.maximumDepth))throw Error("optional-grant-invalid");};
const fits=(g:FieldGrant,copy:number,capacity=0,release=0):boolean=>g.maximumItems>0&&g.maximumDepth>0&&g.maximumCopyBytes>=copy&&g.maximumCapacityBytes>=capacity&&g.maximumReleaseBytes>=release;

/** 🎚️ Original caller-held byte fields yield exact unchanged, authored and inverse ownership in bounded turns. */
export class RetainedByteFieldChange{
 readonly #word:4|8;#authored:ByteFieldSource|undefined;#original:ByteFieldSource|undefined;#output:Uint8Array|undefined;#offset=0;#phase=0;#equal:boolean|undefined;#closing=false;#closed=false;
 constructor(wordBytes:4|8){this.#word=wordBytes;}
 get ready():boolean{return this.#phase===3&&!this.#closing;}
 get inverseReady():boolean{return this.#phase===6&&!this.#closing;}
 get terminal():boolean{return this.#closed;}
 get fixedCopyBytes():number{return this.#word*8;}
 #verify(authored:ByteFieldSource,original:ByteFieldSource):void{if(this.#authored&&this.#authored!==authored||this.#original&&this.#original!==original)throw Error("optional-original-source-changed");}
 #receipt(copy:number,capacity=0,release=0):FieldProgress{return{copiedItems:1,copiedBytes:copy,retainedCapacityBytes:capacity,releasedBytes:release};}
 decisionDemand(authored:ByteFieldSource,original:ByteFieldSource):{copyBytes:number;capacityBytes:number}{this.#verify(authored,original);const fixed=this.fixedCopyBytes;if(this.#phase===1&&authored.value!==null&&original.value!==null&&authored.value.length===original.value.length)return{copyBytes:fixed+Math.min(1,authored.value.length-this.#offset)*2,capacityBytes:0};if(this.#phase===2&&authored.value!==null)return{copyBytes:fixed+(this.#output?Math.min(1,authored.value.length-this.#offset):0),capacityBytes:this.#output?0:authored.value.length};return{copyBytes:fixed,capacityBytes:0};}
 decide(authored:ByteFieldSource,original:ByteFieldSource,g:FieldGrant):FieldProgress{
  valid(g);if(this.#closing||this.#phase>3)throw Error("optional-decision-spent");this.#verify(authored,original);const fixed=this.fixedCopyBytes;
  if(this.#phase===0){if(!fits(g,fixed))return idle();this.#authored=authored;this.#original=original;this.#phase=1;return this.#receipt(fixed);}
  if(this.#phase===1){const left=authored.value,right=original.value;if(left===null||right===null||left.length!==right.length){if(!fits(g,fixed))return idle();this.#equal=left===null&&right===null;this.#phase=this.#equal?3:2;this.#offset=0;return this.#receipt(fixed);}const minimum=fixed+Math.min(1,left.length-this.#offset)*2;if(!fits(g,minimum))return idle();const end=Math.min(left.length,this.#offset+Math.floor((g.maximumCopyBytes-fixed)/2));let copied=0;for(let at=this.#offset;at<end;at++){copied+=2;if(left[at]!==right[at]){this.#equal=false;break;}}this.#offset=end;if(this.#equal===false||end===left.length){this.#equal??=true;this.#phase=this.#equal?3:2;this.#offset=0;}return this.#receipt(fixed+copied);}
  if(this.#phase===2){return this.#copy(authored.value,3,g);}
  return idle();
 }
 #copy(source:Uint8Array|null,next:number,g:FieldGrant):FieldProgress{const fixed=this.fixedCopyBytes;if(source===null){if(!fits(g,fixed))return idle();this.#phase=next;return this.#receipt(fixed);}if(!this.#output){if(!fits(g,fixed,source.length))return idle();this.#output=new Uint8Array(source.length);return this.#receipt(fixed,source.length);}const minimum=fixed+Math.min(1,source.length-this.#offset);if(!fits(g,minimum))return idle();const end=Math.min(source.length,this.#offset+g.maximumCopyBytes-fixed);for(let at=this.#offset;at<end;at++)this.#output[at]=source[at]!;const copied=end-this.#offset;this.#offset=end;if(end===source.length)this.#phase=next;return this.#receipt(fixed+copied);}
 takeChange(g:FieldGrant):{change:Uint8Array|null|undefined;progress:FieldProgress}|undefined{valid(g);if(!this.ready)throw Error("optional-change-not-ready");if(!fits(g,this.fixedCopyBytes))return;const change=this.#equal?undefined:this.#authored!.value===null?null:this.#output!;this.#output=undefined;this.#offset=0;this.#phase=4;return{change,progress:this.#receipt(this.fixedCopyBytes)};}
 inverse(original:ByteFieldSource,g:FieldGrant):FieldProgress{valid(g);if(this.#closing||this.#phase!==4&&this.#phase!==5)throw Error("optional-inverse-spent");if(original!==this.#original)throw Error("optional-original-source-changed");return this.#copy(original.value,6,g);}
 takeInverse(g:FieldGrant):{value:Uint8Array|null;progress:FieldProgress}|undefined{valid(g);if(!this.inverseReady)throw Error("optional-inverse-not-ready");if(!fits(g,this.fixedCopyBytes))return;const value=this.#original!.value===null?null:this.#output!;this.#output=undefined;this.#phase=7;return{value,progress:this.#receipt(this.fixedCopyBytes)};}
 beginClose():void{this.#closing=true;}
 closeDemand():{copyBytes:number;releaseBytes:number}{return{copyBytes:this.#closed?0:this.fixedCopyBytes,releaseBytes:this.#output?.byteLength??0};}
 close(g:FieldGrant):FieldProgress{valid(g);if(!this.#closing)throw Error("optional-close-not-started");if(this.#closed)return idle();const release=this.#output?.byteLength??0;if(!fits(g,this.fixedCopyBytes,0,release))return idle();this.#output=undefined;this.#authored=undefined;this.#original=undefined;this.#closed=true;return this.#receipt(this.fixedCopyBytes,0,release);}
}

export type NumberFieldSource=Readonly<{value:number}>;
/** 🔢️ Native scalar fields copy only inline words and preserve IEEE comparison and exact original inverse. */
export class RetainedNumberFieldChange{
 #authored:NumberFieldSource|undefined;#original:NumberFieldSource|undefined;#phase=0;#equal=false;#closing=false;#closed=false;
 constructor(readonly wordBytes:4|8){}
 get fixedCopyBytes(){return this.wordBytes*8+16;}
 get ready(){return this.#phase===2&&!this.#closing;}get inverseReady(){return this.#phase===4&&!this.#closing;}get terminal(){return this.#closed;}
 #receipt():FieldProgress{return{copiedItems:1,copiedBytes:this.fixedCopyBytes,retainedCapacityBytes:0,releasedBytes:0};}
 decide(authored:NumberFieldSource,original:NumberFieldSource,g:FieldGrant):FieldProgress{valid(g);if(this.#closing||this.#phase>2)throw Error("scalar field decision spent");if(this.#authored&&(this.#authored!==authored||this.#original!==original))throw Error("scalar field original source-changed");if(!fits(g,this.fixedCopyBytes))return idle();if(this.#phase===0){this.#authored=authored;this.#original=original;this.#phase=1;}else if(this.#phase===1){this.#equal=authored.value===original.value;this.#phase=2;}return this.#receipt();}
 takeChange(g:FieldGrant):{change:number|undefined;progress:FieldProgress}|undefined{valid(g);if(!this.ready)throw Error("scalar change pending");if(!fits(g,this.fixedCopyBytes))return;this.#phase=3;return{change:this.#equal?undefined:this.#authored!.value,progress:this.#receipt()};}
 inverse(original:NumberFieldSource,g:FieldGrant):FieldProgress{valid(g);if(this.#closing||this.#phase!==3||original!==this.#original)throw Error("scalar inverse original source changed");if(!fits(g,this.fixedCopyBytes))return idle();this.#phase=4;return this.#receipt();}
 takeInverse(g:FieldGrant):{value:number;progress:FieldProgress}|undefined{valid(g);if(!this.inverseReady)throw Error("scalar inverse pending");if(!fits(g,this.fixedCopyBytes))return;this.#phase=5;return{value:this.#original!.value,progress:this.#receipt()};}
 beginClose(){this.#closing=true;}closeDemand(){return{copyBytes:this.#closed?0:this.fixedCopyBytes,releaseBytes:0};}
 close(g:FieldGrant):FieldProgress{valid(g);if(!this.#closing)throw Error("scalar close not begun");if(this.#closed)return idle();if(!fits(g,this.fixedCopyBytes))return idle();this.#authored=undefined;this.#original=undefined;this.#closed=true;return this.#receipt();}
}
