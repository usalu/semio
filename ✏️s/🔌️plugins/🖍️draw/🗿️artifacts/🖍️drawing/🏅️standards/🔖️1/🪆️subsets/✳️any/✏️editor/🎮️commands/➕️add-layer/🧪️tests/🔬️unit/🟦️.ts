/** 🎨️ New artwork has visible paint without changing an explicit style. */
import {expect,test} from "bun:test";
import {produce} from "immer";
import {initialAppearance} from "../../🟦️.ts";
import cases from "../../🧫️fixtures/🎨️appearance/🔣️.json";
test("new layers use the shared visible appearance policy",()=>{
  for(const entry of cases) {
    const before={id:"new",name:"Untitled",attributes:{fill:null,stroke:null} as unknown};
    const expected=produce(before,draft=>{draft.attributes=entry.attributes;});
    expect({...before,attributes:initialAppearance(entry.kind)}).toEqual(expected);
  }
});
