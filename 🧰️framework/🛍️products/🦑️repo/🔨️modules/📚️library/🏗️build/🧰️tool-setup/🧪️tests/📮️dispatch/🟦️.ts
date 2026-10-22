import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import catalogue from"../../../⚙️operations/🔣️catalog.json"with{type:"json"};

const build=join(import.meta.dirname,"../../.."),read=(path:string)=>readFileSync(join(build,path),"utf8");
const sites:Readonly<Record<string,Readonly<{file:string;dispatch:RegExp;implementation:string}>>>={
  setup:{file:"⌨️command/⚙️runtime/🟦️.ts",dispatch:/resolved\.args\[1\]!=="setup"/u,implementation:"runToolchainsSetupV1"},
  verify:{file:"⌨️command/⚙️runtime/🟦️.ts",dispatch:/command==="tools"&&resolved\.args\[1\]==="verify"/u,implementation:"runToolsVerifyV1"},
  list:{file:"📜️script.ts",dispatch:/\["exec","list"\]\.includes\(diagnostic\[1\]/u,implementation:"runToolsCommandV1"},
  exec:{file:"📜️script.ts",dispatch:/\["exec","list"\]\.includes\(diagnostic\[1\]/u,implementation:"runToolsCommandV1"},
};

test("the subcommands of tools in the catalogue are exactly the dispatched ones",()=>{
  const declared=(catalogue.operations.find(operation=>operation.name==="tools")!.subcommands as readonly{name:string}[]).map(row=>row.name).sort();
  expect(declared).toEqual(Object.keys(sites).sort());
});

for(const[name,site]of Object.entries(sites))test(`repo tools ${name} has a dispatch site that runs its implementation`,()=>{
  const text=read(site.file);
  expect(site.dispatch.test(text)).toBe(true);
  expect(text).toContain(site.implementation);
});
