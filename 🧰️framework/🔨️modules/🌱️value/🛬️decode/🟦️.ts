/** 🛬️ Cumulative ownership admission spanning native parsing and typed construction. */
export interface NativeDecodeProgress { readonly completed:number;readonly total:number;readonly ownedBytes:number }
/** 🧮️ One explicit native allocation budget and cancellable decoder work cursor. */
export class NativeDecodeControl {
  private owned=0;
  private completed=0;
  private total=0;
  private started=false;
  private stage=0;
  private maximum:number;
  constructor(maximumBytes:number,private readonly progress:(event:NativeDecodeProgress)=>boolean,private readonly signal?:AbortSignal) {
    if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0)throw new Error("Invalid native decoding allocation ceiling");
    this.maximum=maximumBytes;
  }
  /** 📏️ Current domain allocation ceiling within the caller allowance. */
  get maximumBytes():number{return this.maximum;}
  /** 📊️ Ownership admitted throughout the complete decoding operation. */
  get ownedBytes():number{return this.owned;}
  /** 🛑️ Yield before expensive ownership and receive caller cancellation. */
  async checkpoint():Promise<void>{
    if(this.signal?.aborted||!this.progress({completed:this.completed,total:this.total,ownedBytes:this.owned}))throw new Error("native decoding canceled");
    this.started=true;
    await new Promise<void>(resolve=>setTimeout(resolve,0));
    if(this.signal?.aborted)throw new Error("native decoding canceled");
  }
  /** 🔢️ Advance decoder work and publish every256units. */
  async step():Promise<void>{await this.advance(1);}
  /** 🧭️ Begin a known stage while retaining the complete ownership budget. */
  async beginStage(total:number):Promise<void>{if(!Number.isSafeInteger(total)||total<0||!Number.isSafeInteger(this.stage+1))throw new Error("Invalid native decoding workload");this.stage++;this.completed=0;this.total=total;this.started=false;await this.checkpoint();}
  /** 🪆️ Preserve parent work after a child begins a stage while retaining cumulative ownership. */
  async scopedStage<T>(operation:(control:NativeDecodeControl)=>Promise<T>):Promise<T>{const parent={completed:this.completed,total:this.total,started:this.started,stage:this.stage};try{return await operation(this);}finally{if(this.stage!==parent.stage){this.completed=parent.completed;this.total=parent.total;this.started=parent.started;this.stage=parent.stage;}}}
  /** 📐️ Restrict one stage's owned allocation without resetting cumulative charges. */
  async scopedMaximum<T>(maximum:number,operation:(control:NativeDecodeControl)=>Promise<T>):Promise<T>{if(!Number.isSafeInteger(maximum)||maximum<0)throw new Error("Invalid native stage allowance");const parent=this.maximum;this.maximum=Math.min(parent,maximum);try{if(this.owned>this.maximum)throw new Error("native decoding ownership exceeds stage limit");return await operation(this);}finally{this.maximum=parent;}}
  /** 📍️ Advance consumed units across bounded cancellation frontiers. */
  async advance(units:number):Promise<void>{if(!Number.isSafeInteger(units)||units<0)throw new Error("Invalid native decoding work");if(!this.started)await this.checkpoint();const previous=this.completed;this.completed+=units;if(!Number.isSafeInteger(this.completed)||this.total!==0&&this.completed>this.total)throw new Error("native decoding exceeded declared stage workload");if(Math.floor(previous/256)!==Math.floor(this.completed/256)||this.total!==0&&this.completed===this.total)await this.checkpoint();}
  /** 📦️ Admit cumulative bytes before a copy or reservation. */
  async charge(bytes:number):Promise<void>{
    const next=this.owned+bytes;
    if(!Number.isSafeInteger(bytes)||bytes<0||!Number.isSafeInteger(next)||next>this.maximumBytes)throw new Error("native decoding ownership exceeds caller limit");
    if(!this.started||bytes>65536)await this.checkpoint();
    const committed=this.owned+bytes;
    if(!Number.isSafeInteger(committed)||committed>this.maximumBytes)throw new Error("native decoding ownership exceeds caller limit");
    this.owned=committed;
  }
  /** 🗂️ Admit explicitly declared typed collection slot width before allocation. */
  async admitSlots(count:number,width:number):Promise<void>{if(!Number.isSafeInteger(count)||count<0||!Number.isSafeInteger(width)||width<0)throw new Error("native decoding collection size overflow");await this.charge(count*width);}
  private async copyCheckpoint(completed:number,total:number):Promise<void>{if(this.signal?.aborted||!this.progress({completed,total,ownedBytes:this.owned}))throw new Error("native decoding canceled");await new Promise<void>(resolve=>setTimeout(resolve,0));if(this.signal?.aborted)throw new Error("native decoding canceled");}
  /** 🔎️ Validate borrowed UTF-8 without constructing a second owned text buffer. */
  async validateUtf8(bytes:Uint8Array):Promise<void>{
    await this.copyCheckpoint(0,bytes.byteLength);let position=0,last=0;
    while(position<bytes.byteLength){
      const first=bytes[position++];let count=0,low=0x80,high=0xbf;
      if(first<=0x7f)count=0;else if(first>=0xc2&&first<=0xdf)count=1;else if(first>=0xe0&&first<=0xef){count=2;if(first===0xe0)low=0xa0;if(first===0xed)high=0x9f;}else if(first>=0xf0&&first<=0xf4){count=3;if(first===0xf0)low=0x90;if(first===0xf4)high=0x8f;}else throw new Error("invalid native UTF-8");
      for(let i=0;i<count;i++){if(position===bytes.byteLength)throw new Error("invalid native UTF-8");const next=bytes[position++];if(next<(i===0?low:0x80)||next>(i===0?high:0xbf))throw new Error("invalid native UTF-8");}
      if(position-last>=65536){await this.copyCheckpoint(position,bytes.byteLength);last=position;}
    }
    if(last!==position)await this.copyCheckpoint(position,bytes.byteLength);
  }
  /** 🧬️ Copy intrinsic octets in cancellable64KiB spans after complete ownership admission. */
  async copyBytes(bytes:Uint8Array):Promise<Uint8Array>{await this.charge(bytes.byteLength);await this.copyCheckpoint(0,bytes.byteLength);const output=new Uint8Array(bytes.byteLength);for(let offset=0;offset<bytes.byteLength;offset+=65536){const end=Math.min(offset+65536,bytes.byteLength);output.set(bytes.subarray(offset,end),offset);await this.copyCheckpoint(end,bytes.byteLength);}return output;}
}
