import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parseResolvedPluginViewState} from "../../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../../🧫️fixtures/♻️original/🔣️.json" with {type:"json"};

test("original view context dictionaries retain explicit languages and complete JSON fields",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 for(const context of fixture.contexts){expect(validate(context)).toBe(true);const parsed=parseResolvedPluginViewState(context);expect<unknown>(parsed).toEqual(context);expect(parsed).not.toBe(context);expect(JSON.parse(JSON.stringify(parsed))).toEqual(context);for(const[key,value]of Object.entries(context.activeUtilityByWindowId)){expect(parsed.activeUtilityByWindowId?.[key]).toBe(value);}for(const[key,value]of Object.entries(context.toolRunTraceCursorByWindowId)){expect(parsed.toolRunTraceCursorByWindowId?.[key]).toEqual(value);}}
 expect(fixture.contexts.map(row=>row.locale)).toEqual(["en","de"]);
 console.log("[DEBUG] Original EN/DE view context retains all fields through first-party parser, Ajv and independent JSON wire");
});