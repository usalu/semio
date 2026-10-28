/** 🧵️ Stored ZIP streaming counterpart; no file bodies are retained. */
interface Entry {name:string;crc:number;size:number;offset:number;flags:number}
const encode=new TextEncoder();
const crc=(initial:number,bytes:Uint8Array):number=>{let value=initial;for(const byte of bytes){value^=byte;for(let bit=0;bit<8;bit++)value=(value>>>1)^((value&1)?0xedb88320:0);}return value>>>0;};
const field=(out:number[],value:number,size:number):void=>{for(let i=0;i<size;i++)out.push((value>>>(8*i))&255);};
export class StoredZipStream {
    static readonly maximumChunkBytes=4096;
    private entries:Entry[]=[];
    private active:Entry|undefined;
    private offset=0;
    private centralOffset:number|undefined;
    private centralIndex=0;
    private finished=false;
    private admit(name:string):void{if(this.finished||this.centralOffset!==undefined||this.active)throw Error("zip.stream.phase");if(this.entries.length>=1024||!name||encode.encode(name).length>256||this.entries.some(entry=>entry.name===name))throw Error("zip.stream.entry");}
    private publish(bytes:Uint8Array):Uint8Array{if(bytes.length>4096)throw Error("zip.stream.chunk-limit");if(this.offset+bytes.length>0xffffffff)throw Error("zip.stream.size");this.offset+=bytes.length;return bytes;}
    private header(name:string,flags:number,checksum:number,size:number):number[]{const bytes=encode.encode(name),out:number[]=[];field(out,0x04034b50,4);for(const value of[20,flags,0,0,0x21])field(out,value,2);for(const value of[checksum,size,size])field(out,value,4);field(out,bytes.length,2);field(out,0,2);out.push(...bytes);return out;}
    knownEntry(name:string,bytes:Uint8Array):Uint8Array{this.admit(name);if(30+encode.encode(name).length+bytes.length>4096)throw Error("zip.stream.chunk-limit");const checksum=(crc(0xffffffff,bytes)^0xffffffff)>>>0,out=this.header(name,0x0800,checksum,bytes.length);out.push(...bytes);this.entries.push({name,crc:checksum,size:bytes.length,offset:this.offset,flags:0x0800});return this.publish(Uint8Array.from(out));}
    beginEntry(name:string):Uint8Array{this.admit(name);this.active={name,crc:0xffffffff,size:0,offset:this.offset,flags:0x0808};return this.publish(Uint8Array.from(this.header(name,0x0808,0,0)));}
    writeChunk(bytes:Uint8Array):Uint8Array{if(bytes.length>4096)throw Error("zip.stream.chunk-limit");if(!this.active)throw Error("zip.stream.no-entry");this.active.crc=crc(this.active.crc,bytes);this.active.size+=bytes.length;if(this.active.size>0xffffffff)throw Error("zip.stream.size");return this.publish(bytes.slice());}
    endEntry():Uint8Array{if(!this.active)throw Error("zip.stream.no-entry");const entry=this.active;this.active=undefined;entry.crc=(entry.crc^0xffffffff)>>>0;const out:number[]=[];for(const value of[0x08074b50,entry.crc,entry.size,entry.size])field(out,value,4);this.entries.push(entry);return this.publish(Uint8Array.from(out));}
    finishOne():Uint8Array|undefined{if(this.active)throw Error("zip.stream.open-entry");if(this.finished)return undefined;this.centralOffset??=this.offset;const entry=this.entries[this.centralIndex];if(entry){const name=encode.encode(entry.name),out:number[]=[];field(out,0x02014b50,4);for(const value of[20,20,entry.flags,0,0,0x21])field(out,value,2);for(const value of[entry.crc,entry.size,entry.size])field(out,value,4);for(const value of[name.length,0,0,0,0])field(out,value,2);field(out,0,4);field(out,entry.offset,4);out.push(...name);this.centralIndex++;return this.publish(Uint8Array.from(out));}const out:number[]=[];field(out,0x06054b50,4);for(const value of[0,0,this.entries.length,this.entries.length])field(out,value,2);field(out,this.offset-this.centralOffset,4);field(out,this.centralOffset,4);field(out,0,2);this.finished=true;return this.publish(Uint8Array.from(out));}
    close():void{this.active=undefined;this.entries=[];this.finished=true;}
}
