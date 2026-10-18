import{test,expect}from"bun:test";
import Ajv from"ajv/dist/2020.js";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import schema from"../🧬️schema/🔣️.json";
import specimen from"./🧫️fixtures/🔣️.json";
import { readModuleTreeSource } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { familyText } from "../../../../🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";
import {pluginSubtreeText} from "../../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
test("original opening keeps its actual partial app and source until its issuer is ready",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(specimen)).toBe(true);for(const key of["defaultCaller","recordIdAsOperation","eraseUncreatedOnDeferred"])expect(validate({...specimen,[key]:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE opening(ordinal INTEGER,phase TEXT,partial INTEGER,published INTEGER)");for(const[i,row]of specimen.states.entries())db.query("INSERT INTO opening VALUES(?,?,?,?)").run(i,row.phase,Number(row.partialPa),Number(row.nativeCreated));expect(db.query("SELECT count(*) AS illegal FROM opening WHERE published=1 AND ordinal<5").get()).toEqual({illegal:0});expect(db.query("SELECT count(*) AS retained FROM opening WHERE phase IN ('issuer-denied','issuer-pending') AND partial=1 AND published=0").get()).toEqual({retained:2});}finally{db.close()}
 const runtime=pluginSubtreeText("⚛️reactor","🏃️runtime"),lifetime=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),turn=pluginSubtreeText("⚛️reactor","🔄️turn");expect(lifetime).toContain("pending_creation");expect(runtime).toContain("admit_cancellation_issuer(cx)");expect(runtime).toContain("plugin_resume_actor_openings");expect(turn).toContain("plugin_resume_actor_openings(runtime,identity,cx)");expect(runtime).toContain("OriginalTurnFailure");const ladder=pluginSubtreeText("🧵️instance"),instance=pluginSubtreeText("🧵️instance");expect(ladder).toContain("CloseRung::CancellationIssuer");expect(ladder).toContain("self.tools.tool_cancellations.close_step(parent,cx)");expect(instance).toContain("self.tools.tool_cancellations.terminal_is_empty()");
});
