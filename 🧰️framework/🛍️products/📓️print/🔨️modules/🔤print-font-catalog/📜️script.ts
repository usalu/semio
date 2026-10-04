import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { stagePrintFonts } from "./🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Print font preparation accepts no arguments");
    const result = await stagePrintFonts(this.repoRoot);
    console.log(`Print fonts staged: ${result.total} authored TTF assets`);
  }
}


import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import type {PrintFontMetrics,PrintFontPositionTable} from "./📏️metrics/🟦️.ts";

/** 🔎️ Derives only the tracked Anta Latin default shaping and advance tables. */
export function derivePrintSansMetrics(workspace:string):PrintFontMetrics {
  const b=readFileSync(join(workspace,"🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/🅰️anta/🅰️Anta-Regular.ttf")),tables=new Map<string,number>();
  const u=(p:number)=>b.readUInt16BE(p),s=(p:number)=>b.readInt16BE(p);
  for(let i=0;i<u(4);i++){const p=12+16*i;tables.set(b.toString("ascii",p,p+4),b.readUInt32BE(p+8));}
  const count=u(tables.get("maxp")!+4),unitsPerEm=u(tables.get("head")!+18),horizontal=u(tables.get("hhea")!+34),hmtx=tables.get("hmtx")!;
  const coverage=(p:number):number[]=>{
    if(u(p)===1)return Array.from({length:u(p+2)},(_,i)=>u(p+4+2*i));
    if(u(p)!==2)throw Error("unsupported Anta coverage");
    return Array.from({length:u(p+2)},(_,i)=>{const q=p+4+6*i;return Array.from({length:u(q+2)-u(q)+1},(_,j)=>u(q)+j);}).flat();
  };
  const classes=(p:number):number[]=>{
    const result=Array<number>(count).fill(0);
    if(u(p)===1)for(let i=0;i<u(p+4);i++)result[u(p+2)+i]=u(p+6+2*i);
    else if(u(p)===2)for(let i=0;i<u(p+2);i++){const q=p+4+6*i;for(let gid=u(q);gid<=u(q+2);gid++)result[gid]=u(q+4);}
    else throw Error("unsupported Anta glyph classes");
    return result;
  };
  const cmap=tables.get("cmap")!,encoding=Array.from({length:u(cmap+2)},(_,i)=>cmap+4+i*8).find(p=>u(p)===3&&u(p+2)===1)!;
  const cm=cmap+b.readUInt32BE(encoding+4),segments=u(cm+6)/2,end=cm+14,start=end+2*segments+2,delta=start+2*segments,offset=delta+2*segments;
  if(u(cm)!==4)throw Error("Anta Latin metrics require Unicode cmap 4");
  const characters:Record<string,number>={};
  for(let i=0;i<segments;i++)for(let cp=u(start+2*i);cp<=u(end+2*i)&&cp<65535;cp++){
    const address=offset+2*i,range=u(address),raw=range===0?cp:u(address+range+2*(cp-u(start+2*i))),glyph=(raw+s(delta+2*i))&65535;
    if(raw!==0&&glyph!==0)characters[String(cp)]=glyph;
  }
  const lookup=(name:string,index:number):{base:number;ignoreMarks:boolean;subtables:number[]}=>{
    const table=tables.get(name)!,list=table+u(table+8),base=list+u(list+2+2*index),flags=u(base+2);
    if(flags!==0&&flags!==8)throw Error("unsupported Anta lookup flags");
    return {base,ignoreMarks:flags===8,subtables:Array.from({length:u(base+4)},(_,i)=>base+u(base+6+2*i))};
  };
  const substitution=(index:number):PrintFontMetrics["substitutions"][number]=>{
    const record=lookup("GSUB",index),kind=u(record.base),rules:{input:number[];output:number[]}[]=[];
    for(const p of record.subtables){
      if(u(p)!==1)throw Error("unsupported Anta substitution");
      const glyphs=coverage(p+u(p+2));
      if(kind===2)glyphs.forEach((glyph,i)=>{const q=p+u(p+6+2*i);rules.push({input:[glyph],output:Array.from({length:u(q)},(_,j)=>u(q+2+2*j))});});
      else if(kind===4)glyphs.forEach((glyph,i)=>{const set=p+u(p+6+2*i);for(let j=0;j<u(set);j++){const q=set+u(set+2+2*j);rules.push({input:[glyph,...Array.from({length:u(q+2)-1},(_,k)=>u(q+4+2*k))],output:[u(q)]});}});
      else throw Error("unsupported Anta Latin substitution lookup");
    }
    return {kind:kind===2?"multiple":"ligature",ignoreMarks:record.ignoreMarks,rules};
  };
  const value=(p:number,format:number):{advance:number;bytes:number}=>{
    let advance=0,bytes=0;for(let bit=0;bit<8;bit++)if(format&(1<<bit)){if(bit===2)advance=s(p+bytes);bytes+=2;}
    return {advance,bytes};
  };
  const positioning=Array.from({length:4},(_,index)=>{
    const record=lookup("GPOS",index);
    if(u(record.base)!==2)throw Error("Anta kerning requires pair positioning");
    const pairs=record.subtables.map((p):PrintFontPositionTable=>{
      const covered=coverage(p+u(p+2)),firstFormat=u(p+4),secondFormat=u(p+6);
      if(u(p)===1){
        const result:number[][]=[];
        covered.forEach((first,i)=>{const set=p+u(p+10+2*i);let q=set+2;for(let j=0;j<u(set);j++){const second=u(q),a=value(q+2,firstFormat),z=value(q+2+a.bytes,secondFormat);result.push([first,second,a.advance+z.advance]);q+=2+a.bytes+z.bytes;}});
        return {coverage:covered,pairs:result};
      }
      if(u(p)!==2)throw Error("unsupported Anta pair format");
      let q=p+16;
      const adjustments=Array.from({length:u(p+12)},()=>Array.from({length:u(p+14)},()=>{const a=value(q,firstFormat),z=value(q+a.bytes,secondFormat);q+=a.bytes+z.bytes;return a.advance+z.advance;}));
      return {coverage:covered,firstClasses:classes(p+u(p+8)),secondClasses:classes(p+u(p+10)),adjustments};
    });
    return {ignoreMarks:record.ignoreMarks,tables:pairs};
  });
  const gdef=tables.get("GDEF")!;
  return {family:"Anta",sha256:createHash("sha256").update(b).digest("hex"),unitsPerEm,characters,advances:Array.from({length:count},(_,gid)=>u(hmtx+4*Math.min(gid,horizontal-1))),classes:classes(gdef+u(gdef+4)),substitutions:[substitution(2),substitution(13)],positioning};
}
class MetricsScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length)throw Error("Font metrics derivation accepts no arguments");
    const output=join(import.meta.dir,"📏️metrics","🔣️.json");mkdirSync(join(import.meta.dir,"📏️metrics"),{recursive:true});
    writeFileSync(output,JSON.stringify(derivePrintSansMetrics(this.repoRoot),null,2)+"\n");
    console.log("[print-fonts] source-bound SemioSans metrics derived");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("metrics", MetricsScript);
if (import.meta.main) await router.run(process.argv.slice(2));
