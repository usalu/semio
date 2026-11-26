/** 🔎️ Contract: a read that took a record of the fleet and found it stale says the first row that does not hold, in both languages, and the read prints that line from the look answer of the farm child.
 * @see ../../🟨️.mjs `createTakeLedgerV1` */
import{expect,test}from"bun:test";
import{createTakeLedgerV1}from"../../🟨️.mjs";
const source=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();

test("the stale row line names the owner and the row in English and German",()=>{
 const ledger=createTakeLedgerV1(3),row={kind:"closure",path:"[\"/r\",\"abc\",[\"generated\"]]",fact:"[]"};
 expect(ledger.staleRowLine("en","owner-a",row)).toBe("owner owner-a: the first row of its stored record that does not hold here is closure [\"/r\",\"abc\",[\"generated\"]] (fact [])");
 expect(ledger.staleRowLine("de","owner-a",row)).toContain("Projekt owner-a: die erste Zeile seines gespeicherten Eintrags");
 expect(ledger.staleRowLine("de","owner-a",row)).toContain("(Fakt [])");
});

test("the read prints the line from the stale look answer, at most 400 times",()=>{
 expect(source).toContain("looked?.verdict===\"stale\"&&looked.staleRow!==undefined&&staleRowTold++<400");
 expect(source).toContain("ledger.staleRowLine(language,name,row)");
});
