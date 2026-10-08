import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";

test("captured Store source authority returns the original lease after funded closure",()=>{
  const law=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️source-authority/🔣️.json",import.meta.url),"utf8"));
  const original={owner:law.owner,registry:{root:law.owner,returned:0},captured:true};
  for(const currency of law.refusals){expect(["items","capacity","depth"]).toContain(currency);expect(applyPatch(original,[],true,false).newDocument).toEqual(original);}
  const terminal=applyPatch(original,[{op:"replace",path:"/captured",value:false},{op:"replace",path:"/registry/returned",value:1}],true,false).newDocument;
  expect(terminal.registry.root).toBe(original.owner);expect(terminal.registry.returned).toBe(law.afterSourceClose.returned);expect(law.afterSourceClose.rootInOriginalRegistry).toBe(true);expect(law.afterSourceClose.ownerCopied).toBe(false);
  console.log("[DEBUG] captured Store source independent RFC6902 oracle retains original returned registry root");
});
