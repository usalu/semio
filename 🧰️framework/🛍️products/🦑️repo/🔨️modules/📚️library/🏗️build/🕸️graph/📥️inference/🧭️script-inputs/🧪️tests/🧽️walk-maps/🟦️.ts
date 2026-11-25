/** 🧽️ Contract: the digest-keyed derived facts of the layers of the closure entries are a bounded, named cache (it was an unbounded map that only a process restart emptied); a command walk is no layer, so no walk is a member of one.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{boundedCacheStatsV1}from"../../../../../🧊️bounded/🟨️.mjs";
import{resetClosureEntriesV1}from"../../🟨️.mjs";

test("the walk fact cache is registered with finite bounds and a reset empties it",()=>{
 resetClosureEntriesV1();
 const rows=boundedCacheStatsV1().filter(row=>row.name.startsWith("inference:closure-walk-"));
 expect(rows.map(row=>row.name).sort()).toEqual(["inference:closure-walk-facts"]);
 for(const row of rows){expect(Number.isSafeInteger(row.maxEntries)).toBe(true);expect(row.entries).toBe(0);}
});
