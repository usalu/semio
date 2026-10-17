import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
test("Config paged original source preserves order and retained backing",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",import.meta.url),"utf8")).pagedSource;
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_source(ordinal INTEGER PRIMARY KEY,value TEXT NOT NULL)");law.values.forEach((value:string,ordinal:number)=>db.run("INSERT INTO original_source VALUES(?,?)",[ordinal,value]));for(const prefix of law.prefixes){expect(db.query("SELECT value FROM original_source WHERE ordinal<? ORDER BY ordinal").all(prefix)).toEqual(law.values.slice(0,prefix).map((value:string)=>({value})));expect(db.query("SELECT value FROM original_source WHERE ordinal>=? ORDER BY ordinal").all(prefix)).toEqual(law.values.slice(prefix).map((value:string)=>({value})));expect(db.query("SELECT count(*) AS n FROM original_source").get()).toEqual({n:law.values.length});}}finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("struct PagedSourceIterator");expect(source).toContain("fn take_next_into");expect(source).toContain("next_get_depth_demand");
});

test("Config exhausted paged publication remains in repeated controlled cleanup",()=>{
 const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",import.meta.url),"utf8")).pagedSource;const db=new Database(":memory:");try{db.exec("CREATE TABLE original_source(ordinal INTEGER PRIMARY KEY,value TEXT NOT NULL)");law.values.forEach((value:string,ordinal:number)=>db.run("INSERT INTO original_source VALUES(?,?)",[ordinal,value]));expect(db.query("SELECT count(*) AS n FROM original_source WHERE ordinal>=?").get(law.values.length)).toEqual({n:0});}finally{db.close();}
 const backing=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");const config=readFileSync(new URL("../../../🎚️config/📥️retained/🦀️.rs",import.meta.url),"utf8");expect(backing).toContain("fn publication_complete");const begin=config.slice(config.indexOf("Phase::BeginEdit => {"),config.indexOf("Phase::DecodeForward | Phase::DecodeInverse => {"));expect(begin.indexOf("publication_complete()")).toBeGreaterThanOrEqual(0);expect(begin.indexOf("publication_complete()")).toBeLessThan(begin.indexOf("take_next_into"));
});
