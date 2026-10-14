/** 🎟️ A new copy maximum selects its own UTF8 event without remembering an earlier quote. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
test("language-neutral reselected copy authority agrees with the independent UTF8 decoder",()=>{expect(new Ajv().compile(schema)(fixture)).toBe(true);for(const row of fixture.copySelection){let own="",bytes=0;for(const char of row.source){const width=new TextEncoder().encode(char).length;if(bytes+width>row.admittedBytes)break;own+=char;bytes+=width;}const source=Buffer.from(row.source,"utf8");let end=Math.min(source.length,row.admittedBytes),oracle="";while(end>0){try{oracle=new TextDecoder("utf8",{fatal:true}).decode(source.subarray(0,end));break}catch{end--;}}expect(own).toBe(oracle);expect(own).toBe(row.expectedPrefix);expect(bytes).toBe(row.expectedCopiedBytes);expect(bytes).toBeLessThanOrEqual(row.admittedBytes);expect(row.admittedBytes).toBeLessThan(row.requestedBytes);}});

test("nested original completion transitions retain structural authority after the child is ready or terminal",()=>{const validate=new Ajv().compile(schema);expect(validate(fixture)).toBe(true);for(const row of fixture.completionSelection){expect(row.minimumDepth).toBeGreaterThan(0);const denied=structuredClone(fixture);denied.completionSelection=denied.completionSelection.map(x=>({...x,minimumDepth:0}));expect(validate(denied)).toBe(false);expect(row.copiedItems).toBe(1);expect(row.copiedBytes).toBe(0);}});
