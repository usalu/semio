import {expect,test} from "bun:test";
import Ajv from "ajv";
import difference from "lodash/difference";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🧬️.json";
test("No-example is an explicit capability in either locale",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){const ids=difference(row.options.map(option=>option.id),["__none__","empty"]);const offered=row.includeNoExample===true?["__none__",...ids]:ids;expect(offered).toEqual(row.expected.offered);expect(row.select==="__none__"?"":row.select).toBe(row.expected.emitted);}
});
test("native select default preserves strict source refusal rather than assuming cheap blank birth",async()=>{
 const source=await Bun.file(new URL("../🟦️.tsx",import.meta.url)).text();expect(source).toContain("includeNoExample = false");expect(source).toContain('SelectValue placeholder={resolvedLabel}');
 const native=await Bun.file("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs").text();expect(native).toContain('for id in ["","unknown-example","demo"]');
});
import * as React from "react";
import {renderToStaticMarkup} from "react-dom/server";
import {NavbarExampleSelect} from "../🟦️.tsx";
import {UiI18nProvider} from "../../📚️I18n/⚛️context/🟦️.ts";
import {createShellI18nInstance,disposeShellI18nInstance} from "../../../🌐️i18n/🟦️.ts";
for(const row of fixture.cases)test("actual React markup "+row.id+" keeps the localized selected value",()=>{
 const instance=createShellI18nInstance(row.locale==="en"?"en":"de");
 try{const tree=React.createElement(UiI18nProvider,{instance},React.createElement(NavbarExampleSelect,{id:"capability.fixture",value:row.value,options:row.options.map(option=>({...option,icon:"file"as const})),onValueChange:()=>{},...("includeNoExample"in row?{includeNoExample:row.includeNoExample}:{})}));const markup=renderToStaticMarkup(tree);const selected=/<span[^>]*data-slot="select-value"[^>]*>([\s\S]*?)<\/span>/u.exec(markup)?.[1]?.replace(/<[^>]+>/gu,"");expect(selected).toBe(row.expected.text);expect(markup).toContain('role="combobox"');expect(markup).toContain('aria-expanded="false"');expect(markup).toContain('aria-label="'+(row.expected.label)+'"');}finally{disposeShellI18nInstance(instance);}
});
