/** 📏️ Contract: the semantics revision of the exchange moves with what inference produces. The declared stamp of every inferrer, the fixture corpus of the derived inferrers and the schema of the graph are pinned to the revision in a history; a change of one of them with the revision unchanged fails, a history that reuses a revision for two digests fails.
 * Not covered: a change of inferrer code that moves no stamp, no corpus and no schema (the stamps are bumped by hand, as before this law); the values of results of the inferrers without a derived stamp.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{SELECTED_EXCHANGE_POLICY_V1 as policy}from"../../🟦️.ts";
import{semanticsDigestV1,semanticsInputsV1}from"../🟦️.ts";

const graph=fileURLToPath(new URL("../../../../",import.meta.url)).replace(/\/$/u,""),history=JSON.parse(readFileSync(new URL("../../🧬️schema/📏️revision.json",import.meta.url),"utf8")) as {history:{revision:number;digest:string}[]};
const current=semanticsInputsV1(graph),digest=semanticsDigestV1(current);

test("the revision of the exchange is the last of the pinned history and its digest is the digest of what inference is declared to produce today",()=>{
 const last=history.history.at(-1)!;
 expect(last.revision).toBe(policy.revision);
 expect(last.digest).toBe(digest);
});

test("the history gives every revision one digest and every digest one revision, in rising order",()=>{
 const revisions=history.history.map(row=>row.revision),digests=history.history.map(row=>row.digest);
 expect(revisions).toEqual([...revisions].sort((a,b)=>a-b));
 expect(new Set(revisions).size).toBe(revisions.length);expect(new Set(digests).size).toBe(digests.length);
});

test("the digest moves with a stamp bump, a change of the corpus and a change of the graph schema, and with nothing else",()=>{
 expect(semanticsDigestV1(semanticsInputsV1(graph))).toBe(digest);
 const id=Object.keys(current.stamps)[0]!,bumped={...current,stamps:{...current.stamps,[id]:current.stamps[id]!.replace(/\/\d+$/u,match=>"/"+(Number(match.slice(1))+1))}};
 expect(semanticsDigestV1(bumped)).not.toBe(digest);
 expect(semanticsDigestV1({...current,corpus:"0".repeat(64)})).not.toBe(digest);
 expect(semanticsDigestV1({...current,graphSchema:"1".repeat(64)})).not.toBe(digest);
 expect(semanticsDigestV1({...current,derived:[...current.derived].reverse()})).toBe(digest);
});

test("every inferrer of the protocol is pinned with a stamp of a positive version",()=>{
 expect(Object.keys(current.stamps).length).toBeGreaterThanOrEqual(19);
 for(const[id,stamp]of Object.entries(current.stamps))expect(stamp).toMatch(new RegExp(`^semio\\.graph-inferrer\\.${id}/[1-9][0-9]*$`,"u"));
});
