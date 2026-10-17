/** 🪪️ Original history commits borrow the existing caller identity authority. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {pluginFamilyText, pluginSubtreeText} from "../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
test("original history commit preserves the mandatory same caller identity recipient",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,newAuthority:true})).toBe(false);expect(validate({...fixture,extra:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE recipient(route TEXT,authority TEXT,receipt TEXT)");for(const route of fixture.routes)db.query("INSERT INTO recipient VALUES(?,?,?)").run(route,fixture.authority,fixture.receipt);expect(db.query("SELECT count(DISTINCT authority) AS authorities,count(DISTINCT receipt) AS receipts,count(*) AS routes FROM recipient").get()).toEqual({authorities:1,receipts:1,routes:3});}finally{db.close();}
 const source=pluginSubtreeText("⏪️time-travel");const parent=pluginFamilyText();expect(source).not.toContain("TimeTravelStoreCommand::Commit");expect(source).toContain("store.commit_finished_replay(finished, finalization, identity)");expect(source).toContain("self.stores.store.commit_finished_replay(result, finalization.clone(), identity)");expect(source).toContain("TimeTravelMemberCommit { owners: &mut member.owners, drafts, finalization, actor, identity }");expect(source).toContain("time_travel_commit(drafts, finalization, meta.map(|meta| meta.actor.as_str()), identity)");expect(parent).toContain("original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained)");expect(parent).toContain("ctx.drive_time_travel_turn(grant, identity, cx)");expect(parent).toContain("ctx.dispatch_time_travel_action(action, args, meta, identity, cx)");const start=source.indexOf("async fn commit_finished(");const end=source.indexOf("///",start+1);expect(source.slice(start,end)).not.toContain("EntityIdentityAuthority::new");
});
