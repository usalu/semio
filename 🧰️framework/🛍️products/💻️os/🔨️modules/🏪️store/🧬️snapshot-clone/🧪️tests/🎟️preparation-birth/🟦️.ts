import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";

test("funded preparation birth preserves original request on independent currency refusal",()=>{
  const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎟️preparation-birth/🔣️.json",import.meta.url),"utf8"));
  for(const currency of law.refusals){expect(["items","capacity","depth"]).toContain(currency);expect(applyPatch(law.request,[],true,false).newDocument).toEqual(law.request);}
  expect(law.afterRefusal).toEqual({requestReturned:true,allocatedBytes:0,releasedBytes:0});
  expect(law.deferred).toEqual(["snapshot source","mutation source","clone cursor","edit cursor","publication assembly"]);
  const admitted=applyPatch({pending:law.request,retained:null},[{op:"replace",path:"/retained",value:law.request},{op:"replace",path:"/pending",value:null}],true,false).newDocument;
  expect(admitted.retained).toEqual(law.request);expect(admitted.pending).toBeNull();expect(law.preflight.afterRefusalCalls).toBe(0);expect(applyPatch({calls:law.preflight.afterRefusalCalls},[{op:"replace",path:"/calls",value:1}],true,false).newDocument.calls).toBe(law.preflight.afterBirthCalls);
  expect([law.grant.maximumCopyBytes,law.grant.maximumCapacityBytes,law.grant.maximumReleaseBytes,law.grant.maximumDepth]).toEqual([3,512,97,7]);
});

test("mounted World WindowConfig funds preparation frames and returns each allocation", () => {
  const root = new URL("../../", import.meta.url);
  const { windowPreparation } = JSON.parse(readFileSync(new URL("🧪️tests/🧫️fixtures/🎟️preparation-birth/🔣️.json", root), "utf8"));
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE receipts(turn INTEGER, frame TEXT, copy INTEGER, birth INTEGER, release INTEGER)");
    windowPreparation.events.forEach((event: {frame: string, copy: number, birth: number, release: number}, turn: number) => db.run("INSERT INTO receipts VALUES (?, ?, ?, ?, ?)", [turn, event.frame, event.copy, event.birth, event.release]));
    expect(db.query("SELECT COUNT(*) AS oversized FROM receipts WHERE copy > ?").get(windowPreparation.maximumCopyBytes)).toEqual({ oversized: 0 });
    expect(db.query<{ live: number }, []>("SELECT SUM(birth - release) OVER (ORDER BY turn) AS live FROM receipts").all().map(({ live }) => live)).toEqual(windowPreparation.liveBytes);
    expect(windowPreparation.formerBirthCopyBytes).toBeGreaterThan(windowPreparation.maximumCopyBytes);
  } finally {
    db.close();
  }
  const source = readFileSync(new URL("🦀️.rs", root), "utf8");
  const birth = source.slice(source.indexOf("Ok((Box::new(RetainedClonePreparation"), source.indexOf("enum RetainedClonePreparationPhase"));
  expect(birth).toContain("clone_cursor: None");
  expect(birth).toContain("edit_cursor: None");
  expect(birth).toContain("assembly:None");
  expect(source).toContain("self.clone_handoff=Some(RetainedCloneCursorHandoff::new(cursor))");
  expect(source).toContain("release_frame(&mut self.edit_cursor,effect)");
  expect(source).toContain("release_frame(&mut self.assembly,effect)");
});
