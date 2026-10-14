import {test,expect} from "bun:test";
import {posix,win32} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {isNxSystemPathV1} from "../🟦️.ts";
test("Nx system paths distinguish native descendants from sibling product outputs",()=>{const validate=new Ajv({strict:true}).compile(schema);for(const row of fixture.cases){const request={version:fixture.version,platform:row.platform,root:row.root,path:row.path};expect(validate(request)).toBe(true);const flavor=row.platform==="windows"?win32:posix,ancestor=flavor.relative(row.root,row.path),oracle=ancestor===""||!flavor.isAbsolute(ancestor)&&ancestor.split(flavor.sep)[0]!=="..";expect(isNxSystemPathV1(request as Parameters<typeof isNxSystemPathV1>[0])).toBe(row.inside);expect(oracle).toBe(row.inside);}console.log("[DEBUG] Independent Node/Ajv native path oracle closes Windows/POSIX Nx system descendants");});
