/** 🏷️ Contract: every stale verdict of an owner record, in the take pass and in the inference of the owner, names the first moved row (kind, path, recorded and current digest) in the adoption line and in one structured `[graph-stale]` event that the schema `StaleRowEvent` closes; before this a fleet-fetched stale record logged `stale (-)` and no one could tell what moved.
 * @see ../../🟨️.mjs `staleRowEventV1` */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
const source=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();
const{createTakeLedgerV1,createAdoptCountersV1,staleRowEventV1}=await import("../../🟨️.mjs") as any;

const row={kind:"file",path:"/r/a.ts",fact:"{}",recorded:"aaaaaaaaaaaa",current:"bbbbbbbbbbbb"};

test("the line names recorded and current digest in English and German, and none for a row without digests",()=>{
 const ledger=createTakeLedgerV1(1);
 expect(ledger.staleRowLine("en","o",row)).toBe("owner o: the first row of its stored record that does not hold here is file /r/a.ts (fact {}); recorded aaaaaaaaaaaa, now bbbbbbbbbbbb");
 expect(ledger.staleRowLine("de","o",row)).toBe("Projekt o: die erste Zeile seines gespeicherten Eintrags, die hier nicht gilt, ist file /r/a.ts (Fakt {}); aufgezeichnet aaaaaaaaaaaa, jetzt bbbbbbbbbbbb");
 expect(ledger.staleRowLine("en","o",{kind:"file",path:"/r/a",fact:"{}"})).not.toContain("recorded");
});

test("the event is closed under its schema and carries both sentences",()=>{
 const ledger=createTakeLedgerV1(1),ajv=new Ajv({strict:true}),valid=ajv.compile(schema.definitions.StaleRowEvent);
 for(const origin of["take","infer"]){const event=staleRowEventV1("@a/b",origin,{...row,why:"closure-value"},ledger);expect(valid(event),JSON.stringify(valid.errors)).toBe(true);expect(event.text.en).toContain("recorded aaaaaaaaaaaa, now bbbbbbbbbbbb");expect(event.text.de).toContain("jetzt bbbbbbbbbbbb");}
 const bare=staleRowEventV1("@a/b","take",{kind:"closure",path:"p",fact:"[]"},ledger);
 expect(valid(bare)).toBe(true);expect([bare.recorded,bare.current]).toEqual(["-","-"]);
 expect(valid({...bare,extra:1})).toBe(false);expect(valid({...bare,source:"fleet"})).toBe(false);
});

test("the adoption tally counts the kind of a stale row taken here",()=>{
 const adopt=createAdoptCountersV1();adopt.staleRow("file");adopt.staleRow("file");adopt.staleRow("closure");
 expect(adopt.line("en")).toContain("(closure 1, file 2)");
});

test("the read tells every stale local look through the hook of the inference, once per moved row",()=>{
 expect(source).toContain("stale:row=>staleOwnerRowV1(name,row,away.take===true)");
 expect(source).toContain('process.stderr.write("[graph-stale] "+JSON.stringify(staleRowEventV1(name,taking?"take":"infer",row,ledger))');
 expect(source).toContain("if(taking)adopt.staleRow(row.kind);takeSay(language=>ledger.staleRowLine(language,name,row))");
});
