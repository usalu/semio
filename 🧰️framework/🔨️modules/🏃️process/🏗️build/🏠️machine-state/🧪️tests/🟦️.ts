import {afterEach,describe,expect,test} from "bun:test";
import {join} from "node:path";
import {MACHINE_STATE_PROTOCOL_V1,machinePathV1,machineStateDirectoryV1} from "../🟦️.ts";

const name=MACHINE_STATE_PROTOCOL_V1.environment,saved=process.env[name];
afterEach(()=>{if(saved===undefined)delete process.env[name];else process.env[name]=saved;});

/** 🏠️ The machine state root is one injectable directory resolved in one place; without a declaration every path is `join`. */
describe("machine state root",()=>{
 test("without a declaration machinePathV1 is join",()=>{
  delete process.env[name];
  for(const segments of [[".cache","f","ready"],[".cache/b/l/m/data"],[".nx","workspace-data"],["📋️project.json"],[".cache"],[]])expect(machinePathV1("/work/tree",...segments)).toBe(join("/work/tree",...segments));
 });
 test("a declared absolute directory stands for .cache and its nx child for .nx",()=>{
  process.env[name]="/machines/one";
  expect(machinePathV1("/work/tree",".cache")).toBe("/machines/one");
  expect(machinePathV1("/work/tree",".cache","f","ready")).toBe(join("/machines/one","f","ready"));
  expect(machinePathV1("/work/tree",".cache/b/l/m/data")).toBe(join("/machines/one","b","l","m","data"));
  expect(machinePathV1("/work/tree",".nx","workspace-data")).toBe(join("/machines/one","nx","workspace-data"));
  expect(machinePathV1("/work/tree","./.cache//f/")).toBe(join("/machines/one","f"));
 });
 test("sources and similar names stay in the checkout",()=>{
  process.env[name]="/machines/one";
  for(const segments of [["📋️project.json"],[".cachebust","x"],["src",".cache"],[".nxignore"],[".🧬semio","x"]])expect(machinePathV1("/work/tree",...segments)).toBe(join("/work/tree",...segments));
 });
 test("an empty, relative or overlong declaration is no declaration",()=>{
  for(const value of ["","state","./state","/"+"x".repeat(MACHINE_STATE_PROTOCOL_V1.maximumDirectoryCharacters)]){process.env[name]=value;expect(machineStateDirectoryV1()).toBeUndefined();expect(machinePathV1("/work/tree",".cache","f")).toBe(join("/work/tree",".cache","f"));}
 });
 test("a root inside the declared directory keeps its own state",()=>{
  process.env[name]="/machines/one";
  expect(machinePathV1("/machines/one/test-sandbox/r1",".cache","b","r")).toBe(join("/machines/one/test-sandbox/r1",".cache","b","r"));
  expect(machinePathV1("/machines/one",".cache","b")).toBe(join("/machines/one",".cache","b"));
  expect(machinePathV1("/machines/onetwo",".cache","b")).toBe(join("/machines/one","b"));
 });
 test("the declaration is read from the environment it is given",()=>{
  expect(machineStateDirectoryV1({[name]:"/a/b/"})).toBe("/a/b/");
  expect(machineStateDirectoryV1({})).toBeUndefined();
 });
});
