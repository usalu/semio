import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readMatrix,readSchema} from "../../../📇️matrix/🟦️.ts";
import {validateJourneyOutput,parseEntryMode,assertActualLocale,assertSeatedLocale,selectJourneyRows,assertOwnedProgram} from "../🟦️.ts";
test("authored Text entry schema pins exact note and preserves independent natural fixture",()=>{
 const matrix=readMatrix();expect(new Ajv2020({strict:false}).compile(readSchema())(matrix)).toBe(true);
 const row=matrix.artifacts.find(row=>row.artifact==="semio")!;expect(row.authoredExample?.id).toBe("note");expect(row.authoredExample?.documentSchema).toBe("stdio.semio.text");expect(row.sample).toContain("mutate-semio-text");expect(row.authoredExample?.source).not.toBe(row.sample);
});
test("entry selector agrees with SQLite and refuses unsupported authored rows",()=>{
 const rows=readMatrix().artifacts;const db=new Database(":memory:");db.run("CREATE TABLE row(artifact TEXT,example TEXT)");for(const row of rows)db.run("INSERT INTO row VALUES(?,?)",row.artifact,row.authoredExample?.id??null);
 expect(selectJourneyRows(rows,"authored-example",[]).map(row=>row.artifact)).toEqual(db.query("SELECT artifact FROM row WHERE example IS NOT NULL").all().map(row=>(row as {artifact:string}).artifact));
 expect(selectJourneyRows(rows,"natural-file",[])).toHaveLength(38);expect(()=>selectJourneyRows(rows,"authored-example",["json"])).toThrow();db.close();
});
test("entry and actual locale are explicit and cannot silently fall back",()=>{
 expect(parseEntryMode("natural-file")).toBe("natural-file");expect(parseEntryMode("authored-example")).toBe("authored-example");for(const value of [undefined,"example",""])expect(()=>parseEntryMode(value)).toThrow();
 for(const locale of ["en","de"]as const){expect(()=>assertActualLocale(locale,locale)).not.toThrow();for(const actual of ["",null,locale==="en"?"de":"en"])expect(()=>assertActualLocale(actual,locale)).toThrow();}
});
test("reopen ownership resolves every exact editor window and rejects compatible foreign owners",()=>{
 const expected={pluginId:"stdio-semio",appId:"s.stdio.semio@v1/text#editor"};const programs=[{id:"one",...expected},{id:"one::other",pluginId:"foreign",appId:expected.appId}];
 expect(()=>assertOwnedProgram(["one::main","one::details"],programs,expected)).not.toThrow();
 for(const ids of [[],["one::main","one::other::main"],["unknown::main"]])expect(()=>assertOwnedProgram(ids,programs,expected)).toThrow();
});

test("real driver wires explicit entry, actual locale and exact reopened owner",async()=>{
 const source=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();
 expect(source).toContain("parseEntryMode(flag(segments,\"--entry\"))");
 expect(source).toContain("parseJourneyLocale(flag(segments,\"--locale\"))");
 expect(source).toContain("assertActualLocale(actualLocale,options.locale)");
 expect(source).toContain("assertOwnedProgram(ids,await spawnedPrograms(),program!)");
 expect(source).toContain("Authored entry probe");expect(source).toContain("example.label[locale]");
 expect(source).toContain("reloaded authored document did not retain its exact edited field");
 expect(source).not.toContain('flag(segments, "--locale") ?? "en"');
});
import {seatLocale} from "@semio-tech/framework-os-dev/testing/program-matrix";
import type {Page} from "playwright";
for(const wanted of ["en","de"]as const)test("locale seating uses the real control for "+wanted,async()=>{
 const events:string[]=[];let actual=wanted==="en"?"de":"en";
 const page={locator(selector:string){const settings=selector.includes("os.openSettings");const language=selector==="select, [role=\"combobox\"]";const locator={first(){return locator;},filter(){return locator;},async count(){return settings||language?1:0;},async click(){events.push("settings");},async evaluate(){return "select";},async selectOption(value:string){events.push("select:"+value);actual=value;}};return locator;},async waitForTimeout(){},keyboard:{async press(){}},async evaluate(){return actual;}};
 const result=await seatLocale(page as unknown as Page,wanted);expect(events).toEqual(["settings","select:"+wanted]);expect(result).toBe(wanted+"="+wanted);
});
test("authored entry always commits a probe and explicitly executes the declared example action",async()=>{
 const source=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();
 expect(source).toContain('await control().waitFor({state:"visible",timeout:120_000})');
 expect(source).not.toContain("if(await control().count())");
 expect(source).not.toContain("if(!await settled()){");
 expect(source).toContain('fillStagedArgument(page,"exampleId",example.id,"@liveId")');
 expect(source).toContain("assertSeatedLocale");
});
for(const wanted of ["en","de"]as const)test("locale seating chooses exact combobox option for "+wanted,async()=>{
 const events:string[]=[];let actual=wanted==="en"?"de":"en";
 const page={locator(selector:string){const settings=selector.includes("os.openSettings"),language=selector==="select, [role=\"combobox\"]",option=selector==='[role="option"]';let pattern:RegExp|undefined;
 const locator={first(){return locator;},filter(value:{hasText:RegExp}){pattern=value.hasText;return locator;},async count(){return settings||language||option?1:0;},async click(){if(settings)events.push("settings");else if(language)events.push("open-language");else if(option){expect(pattern?.test(wanted==="en"?"Englisch":"Deutsch")).toBe(true);expect(pattern?.test(wanted==="en"?"Deutsch":"English")).toBe(false);actual=wanted;events.push("option:"+wanted);}},async evaluate(){return "button";}};return locator;},async waitForTimeout(){},keyboard:{async press(){}},async evaluate(){return actual;}};
 const result=await seatLocale(page as unknown as Page,wanted);expect(events).toEqual(["settings","open-language","option:"+wanted]);expect(result).toBe(wanted+"="+wanted);
});
test("seating cannot pass from a boot label or a missing control",()=>{
 for(const wanted of ["en","de"]as const){expect(()=>assertSeatedLocale(wanted+"="+wanted,wanted)).not.toThrow();for(const outcome of ["en(boot)",wanted+":no-settings-control",wanted+":no-language-control"])expect(()=>assertSeatedLocale(outcome,wanted)).toThrow();}
});
test("closed catalog can reopen before its new live actor reports persisted",async()=>{
 const fixture=await Bun.file(new URL("./🧫️fixtures/🔣️.json",import.meta.url)).json();const schema=await Bun.file(new URL("./🧫️fixtures/🧬️.json",import.meta.url)).json();expect(new Ajv2020({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){const db=new Database(":memory:");const output=db.query("SELECT ?=1 AS canClick, ?=1 AND ?=1 AND ?=1 AND ?=1 AS passed").get(Number(row.catalogVisible),Number(row.sameUuid),Number(row.fileOpened),Number(row.exactOwner),Number(row.persisted))as{canClick:number;passed:number};expect({canClick:!!output.canClick,passed:!!output.passed}).toEqual(row.expected);db.close();}
 const source=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();const begin=source.indexOf("const reopenKept =");const click=source.indexOf("await button.click()",begin);const end=source.indexOf("const pickNatural",click);const before=source.slice(begin,click),after=source.slice(click,end);
 expect(before).not.toContain('[data-semio-local-persisted="true"]');expect(before).toContain('button.waitFor({ state: "visible"');
 expect(after).toContain('[data-semio-local-persisted="true"]');expect(after).toContain("assertOwnedProgram");expect(after).toContain("file-opened|");
});
import {join} from "node:path";
test("ticket output validation rejects escaped or nonportable plans before effects",async()=>{
 const fixture=await Bun.file(new URL("./🧫️fixtures/output.json",import.meta.url)).json();const schema=await Bun.file(new URL("./🧫️fixtures/output.schema.json",import.meta.url)).json();expect(new Ajv2020({strict:false}).compile(schema)(fixture)).toBe(true);
 const root=join(process.cwd(),"virtual");const generated=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/TEXT/🗑️generated/browser");
 for(const row of fixture.cases){const run=()=>validateJourneyOutput(root,row.output==="generated"?generated:join(root,"outside"),row.tag,[{artifact:"semio",extension:".semio"}]);if(row.accepted)expect(run()).toBe(join(generated,row.tag));else expect(run).toThrow();}
 const db=new Database(":memory:");db.exec("PRAGMA encoding='UTF-16le'");db.exec("CREATE TABLE path(value TEXT)");const longRoot=join(root,"😀".repeat(80));const path=join(longRoot,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/TEXT/🗑️generated/browser","text-en","exports","semio-after-refusal.semio");db.run("INSERT INTO path VALUES(?)",path);expect(db.query("SELECT length(CAST(value AS BLOB))/2 AS units FROM path").get()).toEqual({units:path.length});expect(path.length).toBeGreaterThan(256);expect(()=>validateJourneyOutput(longRoot,join(longRoot,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/TEXT/🗑️generated/browser"),"text-en",[{artifact:"semio",extension:".semio"}])).toThrow();db.close();
 const source=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text();expect(source.indexOf("validateJourneyOutput",source.indexOf("export async function runJourney("))).toBeLessThan(source.indexOf("mkdirSync",source.indexOf("export async function runJourney(")));expect(source.indexOf("validateJourneyOutput",source.indexOf("export async function runJourneyCli"))).toBeLessThan(source.indexOf("await withDevServe",source.indexOf("export async function runJourneyCli")));
});
