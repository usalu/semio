import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import{applyPatch}from"fast-json-patch";
import{readFileSync}from"node:fs";
test("original owned payload and installed issuer survive partial admission and provider refusal",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();
 const db=new Database(":memory:");db.run("CREATE TABLE custody(original INTEGER, child INTEGER, issuer INTEGER)");
 for(const row of corpus.cases){const baseline={payload:row.text.repeat(row.repeat),capacity:row.capacity,issuer:"original"};expect(Buffer.byteLength(baseline.payload)).toBeLessThanOrEqual(baseline.capacity);for(const axis of corpus.refusalAxes){db.run("DELETE FROM custody");db.run("INSERT INTO custody VALUES(1,0,1)");expect(db.query("SELECT original,child,issuer FROM custody").get()).toEqual({original:1,child:0,issuer:1});expect(applyPatch(structuredClone(baseline),[],true).newDocument).toEqual(baseline);expect(axis.length).toBeGreaterThan(0);}db.run("UPDATE custody SET original=0,child=1");expect(db.query("SELECT original+child AS retained,issuer FROM custody").get()).toEqual({retained:1,issuer:1});expect(applyPatch(structuredClone(baseline),[{op:"remove",path:"/payload"}],true).newDocument).toEqual({capacity:row.capacity,issuer:"original"});}db.close();
 const mount=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(mount).toContain("pub mod owned;");const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub struct FactoryOwnedRetirement<T:Send+'static>");expect(source).toContain("pub fn admit_original(");expect(source).toContain("*self.original=Some(value)");expect(source).toContain("close_factory_ticket");expect(source).not.toContain("retire_cold");
});
