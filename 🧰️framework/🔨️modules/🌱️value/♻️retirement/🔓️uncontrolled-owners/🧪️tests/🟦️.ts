import {test,expect} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const repoRoot=resolve(import.meta.dir,"../../../../../..");
const header=/impl\s*(?:<[^{]*?>)?\s*(?:[\w]+::)*RetireOwned\s+for\s+([^{]*?)(?:\swhere[^{]*)?\{/gs;
const declaresFalse=/fn\s+controlled_retirement_supported\s*\(\s*\)\s*->\s*bool\s*\{\s*false\s*\}/;

function sources():string[]{
  const listing=Bun.spawnSync(["git","-c","core.fsmonitor=false","ls-files","-z","--cached","--others","--exclude-standard","--",...fixture.roots.map(root=>`${root}/**/*.rs`)],{cwd:repoRoot});
  return new TextDecoder().decode(listing.stdout).split("\0").filter(path=>path.endsWith(".rs"));
}

function body(text:string,open:number):string{
  let depth=1;let index=open;
  while(index<text.length&&depth>0){const symbol=text[index];if(symbol==="{")depth+=1;else if(symbol==="}")depth-=1;index+=1;}
  return text.slice(open,index);
}

/** 🔓️ Independent scanner: every hand-written owner that declares `false`, as `file :: owner`. */
function uncontrolledOwners():string[]{
  const found:string[]=[];
  for(const path of sources()){
    let text:string;
    try{text=readFileSync(join(repoRoot,path),"utf8");}catch{continue;}
    if(!text.includes("RetireOwned"))continue;
    for(const match of text.matchAll(header)){
      const open=(match.index??0)+match[0].length;
      if(declaresFalse.test(body(text,open)))found.push(`${path} :: ${match[1].trim()}`);
    }
  }
  return found.sort();
}

test("the uncontrolled owner list is schema-valid",()=>{
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture)).toBe(true);
});

test("exactly the reasoned owners declare uncontrolled retirement",()=>{
  const allowed=fixture.allowed.map(row=>`${row.file} :: ${row.owner}`).sort();
  expect(uncontrolledOwners()).toEqual(allowed);
},120000);
