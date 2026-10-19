import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {ExactKeyCursor,type ExactKeyGrant} from "../🟦️.ts";

test("exact key cursor agrees with SQLite opaque identity and original denied pages",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:"),encoder=new TextEncoder();database.exec("CREATE TABLE keys(idx INTEGER PRIMARY KEY,key TEXT COLLATE BINARY)");
 const prefix="p".repeat(fixture.wide.prefixLength),cases=[...fixture.cases,{id:"wide",key:prefix+fixture.wide.wanted,candidates:fixture.wide.suffixes.map(suffix=>prefix+suffix),expected:fixture.wide.expected,pageBytes:fixture.wide.pageBytes}];
 try{for(const law of cases){database.exec("DELETE FROM keys");for(const [index,key]of law.candidates.entries())database.query("INSERT INTO keys VALUES(?,?)").run(index,key);expect(database.query<{idx:number},[string]>("SELECT idx FROM keys WHERE key=? ORDER BY idx LIMIT 1").get(law.key)?.idx??null).toBe(law.expected);
  const key=encoder.encode(law.key),candidates=law.candidates.map(value=>encoder.encode(value));
  for(const wordBytes of fixture.wordBytes){const owner=new ExactKeyCursor(wordBytes as 4|8);let turns=0;
   while(!owner.ready){const candidate=candidates[owner.nextIndex],demand=owner.demand(key,candidate,owner.fixedCopyBytes+2*law.pageBytes),grant:ExactKeyGrant={maximumItems:1,maximumCopyBytes:demand,maximumDepth:1};const position=owner.nextIndex;
    expect(owner.advance(key,candidate,{...grant,maximumItems:0}).copiedBytes).toBe(0);expect(owner.nextIndex).toBe(position);
    expect(owner.advance(key,candidate,{...grant,maximumCopyBytes:owner.fixedCopyBytes-1}).copiedBytes).toBe(0);expect(owner.nextIndex).toBe(position);
    const receipt=owner.advance(key,candidate,grant);expect(receipt.copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);expect(++turns).toBeLessThan(2*key.length+2*law.candidates.length+16);
   }
   expect(owner.found??null).toBe(law.expected);
  }
 }
 console.log("[DEBUG] nine exact UTF8 lookups match independent SQLite binary identity on32/64-bit metadata widths, denied original pages and65537-byte prefixes");
 }finally{database.close();}
});
