/** 🧐️ Law: the line of a stale row carries the reason of an analysis row in English and German, and none for a row without one.
 * @see ../../🟨️.mjs `staleRowLine` */
import{expect,test}from"bun:test";
const{createTakeLedgerV1}=await import("../../🟨️.mjs") as any;

test("the reason follows the fact in both languages",()=>{
 const ledger=createTakeLedgerV1(1),row={kind:"analysis",path:"/r/a.ts",fact:"{}",why:"revision aaaaaaaaaaaa bbbbbbbbbbbb"};
 expect(ledger.staleRowLine("en","o",row)).toBe("owner o: the first row of its stored record that does not hold here is analysis /r/a.ts (fact {}); reason: revision aaaaaaaaaaaa bbbbbbbbbbbb");
 expect(ledger.staleRowLine("de","o",row)).toBe("Projekt o: die erste Zeile seines gespeicherten Eintrags, die hier nicht gilt, ist analysis /r/a.ts (Fakt {}); Grund: revision aaaaaaaaaaaa bbbbbbbbbbbb");
 expect(ledger.staleRowLine("en","o",{kind:"file",path:"/r/a",fact:"{}"})).toBe("owner o: the first row of its stored record that does not hold here is file /r/a (fact {})");
});
