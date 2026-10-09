/** 🪪️ Original history commits borrow the existing caller identity authority. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
test("original history commit preserves the mandatory same caller identity recipient",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,newAuthority:true})).toBe(false);expect(validate({...fixture,extra:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE recipient(route TEXT,authority TEXT,receipt TEXT)");for(const route of fixture.routes)db.query("INSERT INTO recipient VALUES(?,?,?)").run(route,fixture.authority,fixture.receipt);expect(db.query("SELECT count(DISTINCT authority) AS authorities,count(DISTINCT receipt) AS receipts,count(*) AS routes FROM recipient").get()).toEqual({authorities:1,receipts:1,routes:3});}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");const parent=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source).not.toContain("TimeTravelStoreCommand::Commit");expect(source).toContain("store.commit_finished_replay(finished, finalization, identity)");expect(source).toContain("self.store.commit_finished_replay(result, finalization.clone(), identity)");expect(source).toContain("TimeTravelMemberCommit { owners: &mut member.owners, drafts, finalization, actor, identity }");expect(source).toContain("time_travel_commit(drafts, finalization, meta.map(|meta| meta.actor.as_str()), identity)");expect(parent).toContain("self.drive_time_travel_turn(self.mounted_policy.maintenance, identity)");expect(parent).toContain("self.dispatch_time_travel_action(action, args, meta, identity)");const start=source.indexOf("async fn commit_finished(");const end=source.indexOf("///",start+1);expect(source.slice(start,end)).not.toContain("EntityIdentityAuthority::new");console.log("[DEBUG] SQLite preserves one existing caller authority and receipt across original document/member/supersede commit recipients; schema rejects minted authority and unknown fields");
});
