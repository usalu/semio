/** 👁️ Tests watch-event filtering against actual filesystem entries and a portable independent oracle. */
import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import gitignore from "ignore";
import {compileIgnoreV1} from "../../🚧️boundary/🟦️.ts";
import {ignoreWatchEventV1} from "../👁️watch-ignore/🟦️.ts";
import cases from "./👁️watch-ignore.json";
import schema from "./👁️watch-ignore.schema.json";

test("directory-only watch exclusions preserve missing files and replacements",()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"watch-ignore-")),ignore=compileIgnoreV1("**/.nx/\n"),oracle=gitignore().add("**/.nx/");
 expect(new Ajv().validate(schema,cases)).toBe(true);
 try{for(const row of cases){const path=row.name==="shared-directory"?"shared":".nx",target=join(root,path);rmSync(target,{recursive:true,force:true});if(row.type==="directory")mkdirSync(target);if(row.type==="file")writeFileSync(target,"shared");const expected=!row.tracked&&row.type!=="missing"&&oracle.ignores(path+(row.type==="directory"?"/":""));expect(expected).toBe(row.ignored);expect(ignoreWatchEventV1(root,path,ignore,row.tracked)).toBe(expected);}}
 finally{rmSync(root,{recursive:true,force:true});}
});
