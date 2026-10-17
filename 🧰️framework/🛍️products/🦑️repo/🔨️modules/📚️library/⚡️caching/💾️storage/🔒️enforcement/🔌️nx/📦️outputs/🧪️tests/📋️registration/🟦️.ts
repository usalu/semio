import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import fixture from"./🔣️.json";
import schema from"../../🧬️schema/🔣️.json";
import editor from"./📥️editor.json";
import editorSchema from"./🧬️schema/📥️editor.json";
test("toolkit output laws have a complete neutral permanent owner and editor routes",()=>{
 const root=process.cwd(),cache=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"),project=JSON.parse(readFileSync(join(cache,"📋️project.json"),"utf8")),script=readFileSync(join(cache,"📜️script.ts"),"utf8"),db=new Database(":memory:");
 try{expect(new Ajv({strict:true}).compile(schema.definitions.TestRegistration)(fixture)).toBe(true);expect(project.targets[fixture.target]?.options.command).toBe(fixture.command);expect(project.targets[fixture.target]?.cache).toBe(false);expect(project.targets[fixture.target]?.dependsOn).toEqual([]);expect(script).toContain('"toolkit-outputs":"'+fixture.source+'"');expect(script).toContain('selected==="toolkit-outputs"');
 for(const name of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const parsed=ts.parseConfigFileTextToJson(name,readFileSync(join(root,name),"utf8"));expect(parsed.error).toBeUndefined();const command="bun repo run repo:"+fixture.target+" --exclude-dependencies --skip-cache",rows=parsed.config.configurations.filter((row:any)=>row.command===command);expect(rows).toHaveLength(2);const oracle=db.query("select count(*) count,count(distinct json_extract(value,'$.name')) names from json_each(?)").get(JSON.stringify(rows))as{count:number;names:number};expect(oracle).toEqual({count:2,names:2});for(const language of fixture.languages){const row=rows.find((item:any)=>item.name.endsWith("["+language+"]"));expect(row?.presentation.group).toBe("4_gate");expect(row?.env.SEMIO_TEST_ARTIFACT_DIR).toBe("${workspaceFolder}/${input:processContractArtifacts}");}}
 }finally{db.close();}
});

test("permanent graph publication toolkit use and selected graph have paired editor routes",()=>{
 const root=process.cwd(),cache=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"),project=JSON.parse(readFileSync(join(cache,"📋️project.json"),"utf8")),db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(editorSchema);
 try{expect(validate(editor)).toBe(true);expect(validate({...editor,extra:true})).toBe(false);db.exec("create table expected(target text,language text,name text,ordering real,primary key(target,language));create table actual(target text,language text,name text,ordering real)");for(const target of editor.targets){expect(project.targets[target.target]?.options.command).toBe(target.command);expect(readFileSync(join(cache,target.source),"utf8").length).toBeGreaterThan(0);for(const row of target.rows)db.query("insert into expected values(?,?,?,?)").run(target.target,row.language,row.name,row.order);}
 for(const name of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const parsed=ts.parseConfigFileTextToJson(name,readFileSync(join(root,name),"utf8"));expect(parsed.error).toBeUndefined();db.exec("delete from actual");for(const target of editor.targets){const command="bun repo run repo:"+target.target+" --exclude-dependencies --skip-cache",rows=parsed.config.configurations.filter((row:any)=>row.command===command);expect(rows).toHaveLength(2);for(const expected of target.rows){const row=rows.find((value:any)=>value.name===expected.name);expect(row?.presentation).toEqual({group:"4_gate",order:expected.order});expect(row?.cwd).toBe("${workspaceFolder}");expect(row?.env).toEqual({SEMIO_TEST_ARTIFACT_DIR:"${workspaceFolder}/${input:processContractArtifacts}",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"});db.query("insert into actual values(?,?,?,?)").run(target.target,expected.language,row.name,row.presentation.order);}}
 const oracle=db.query("select (select count(*) from actual) count,not exists(select * from expected except select * from actual) and not exists(select * from actual except select * from expected) matched").get();expect(oracle).toEqual({count:6,matched:1});}}finally{db.close();}
});
