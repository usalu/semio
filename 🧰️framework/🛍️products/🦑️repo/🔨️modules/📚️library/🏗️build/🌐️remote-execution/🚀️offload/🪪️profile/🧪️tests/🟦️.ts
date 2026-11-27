/** 🪪️ Contract: the peak evidence of a unit is keyed by the compile-relevant profile of its rustc command line, so a changed profile starts without evidence.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {readFamilyPeakV1,readUnitExpenseV1,readUnitPeakV1,readUnitPeaksV1,writeUnitPeakV1} from "../../🚦️admission/🟦️.ts";
import {unitPeakKeyV1,unitProfileArgumentsV1,unitProfileIdentityV1,unitProfileKeyV1} from "../🟦️.ts";
import fixture from "./🔣️.json";

const profileSettingsV1=(row:{optLevel:string;debug:number;codegenUnits:number;lto:string|boolean;target:string}):Parameters<typeof unitProfileArgumentsV1>[0]=>({...row,lto:row.lto===false?false:String(row.lto)});

test("every case has the identity written in the fixture",()=>{
 for(const row of fixture.cases)expect(unitProfileIdentityV1(row.args),row.name).toBe(row.identity);
});

test("the profile key ignores the crate and tracks every compile-relevant flag",()=>{
 for(const [name,left,right] of fixture.sameKey)expect(unitProfileKeyV1(left as string[]),name as string).toBe(unitProfileKeyV1(right as string[]));
 for(const [name,left,right] of fixture.differentKey)expect(unitProfileKeyV1(left as string[]),name as string).not.toBe(unitProfileKeyV1(right as string[]));
 expect(unitProfileKeyV1([])).toMatch(/^[a-f0-9]{10}$/u);
});

test("resolved profile settings give the key of the command line cargo derives from them",()=>{
 for(const row of fixture.settingsKeys)expect(unitProfileKeyV1(unitProfileArgumentsV1(profileSettingsV1(row.settings))),row.name).toBe(unitProfileKeyV1(row.sameAs));
});

test("every case equals the independent Python oracle",()=>{
 const result=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 const model=JSON.parse(result.stdout) as {settings:string[];identities:string[];same:boolean[];different:boolean[];peakKey:string};
 expect(model.identities).toEqual(fixture.cases.map(row=>unitProfileIdentityV1(row.args)));
 expect(model.settings).toEqual(fixture.settingsKeys.map(row=>unitProfileKeyV1(unitProfileArgumentsV1(profileSettingsV1(row.settings)))));
 expect(model.same.every(Boolean)).toBe(true);
 expect(model.different.every(Boolean)).toBe(true);
 expect(model.peakKey).toBe(unitPeakKeyV1(fixture.peakKey.crate,fixture.peakKey.args));
});

test("a stale peak of the former profile is no evidence for the lean profile, and the lean profile learns its own",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-profile-")),crate="semio_hub_playbook_component",fat=["-C","opt-level=3","-C","lto=fat","-C","codegen-units=1"],lean=["-C","opt-level=z","-C","lto=off","-C","codegen-units=16"];
 try{
  writeUnitPeakV1(root,unitPeakKeyV1(crate,fat),5784,1);
  expect(readUnitPeakV1(root,unitPeakKeyV1(crate,fat),1)).toBe(5784);
  expect(readUnitPeaksV1(root,unitPeakKeyV1(crate,lean))).toEqual({});
  expect(readUnitPeakV1(root,unitPeakKeyV1(crate,lean),1)).toBeUndefined();
  const stale=readUnitExpenseV1(root,crate,["cdylib"],"wasm32-wasip2",unitProfileKeyV1(fat)),fresh=readUnitExpenseV1(root,crate,["cdylib"],"wasm32-wasip2",unitProfileKeyV1(lean));
  expect(stale.peak).toBeGreaterThanOrEqual(5784);
  expect(fresh.peak).toBeLessThan(stale.peak);
  writeUnitPeakV1(root,unitPeakKeyV1(crate,lean),1800,1);
  expect(readUnitPeakV1(root,unitPeakKeyV1(crate,lean),1)).toBe(1800);
  expect(readUnitPeakV1(root,unitPeakKeyV1(crate,fat),1)).toBe(5784);
  expect(readFamilyPeakV1(root,unitProfileKeyV1(lean))).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});
