import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";

const upperLower=(text:string):string=>text.toUpperCase().toLowerCase();

/** 🧮️ Computes the fold table from the case mapping of the runtime's ICU: for every code point the fixpoint of "upper case, then lower case" per character (so `ß`, `ẞ` and `SS` meet at `ss`, the final and the medial sigma meet, `ı` and `ſ` meet `i` and `s` like NTFS' upper-case table does, ligatures expand), composed to NFC; only entries that differ from the code point itself are kept, ASCII included in the rule but never listed (its fold is the plain lower case). */
export function computeFoldTableV1():Readonly<{unicode:string;icu:string;map:readonly (readonly [number,string])[]}>{
 const map:(readonly [number,string])[]=[];
 for(let point=128;point<=0x10ffff;point++){
  if(point>=0xd800&&point<=0xdfff)continue;
  const own=String.fromCodePoint(point);
  let folded=upperLower(own);
  for(let round=0;round<4;round++){const next=Array.from(folded).map(upperLower).join("");if(next===folded)break;folded=next;}
  folded=folded.normalize("NFC");
  if(folded!==own)map.push([point,folded]);
 }
 return {unicode:process.versions.unicode,icu:process.versions.icu,map};
}

/** 📝️ Renders the table as the checked-in JSON text: the header, then eight entries per line. */
export function renderFoldTableV1(table:ReturnType<typeof computeFoldTableV1>):string{
 const rows:string[]=[];
 for(let index=0;index<table.map.length;index+=8)rows.push(`  ${table.map.slice(index,index+8).map(entry=>JSON.stringify(entry)).join(",")}`);
 return `{\n "version": 1,\n "unicode": "${table.unicode}",\n "icu": "${table.icu}",\n "map": [\n${rows.join(",\n")}\n ]\n}\n`;
}

if(import.meta.main){
 const file=join(import.meta.dirname,"🔣️.json"),text=renderFoldTableV1(computeFoldTableV1());
 if(process.argv.includes("--check")){
  const current=readFileSync(file,"utf8");
  if(current!==text){console.error("fold table differs from this runtime's ICU: run the script without --check");process.exit(1);}
  console.log("fold table equals this runtime's ICU");
 }else{writeFileSync(file,text);console.log(`wrote ${file}`);}
}
