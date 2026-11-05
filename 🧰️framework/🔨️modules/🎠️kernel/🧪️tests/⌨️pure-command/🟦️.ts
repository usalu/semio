import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../../🧬️schema/🔣️pure-command/🔣️.json" with {type:"json"};
import view from "../../../🛂️manifest/🪟️view-context/🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};

test("pure invocation schema preserves explicit caller context and refuses raw invocation payloads",()=>{
 const validate=new Ajv({strict:true}).addSchema(view).compile(schema);
 for(const row of corpus.cases)expect(validate(row.request),row.id).toBe(row.valid);
});

test("the independent relational oracle resolves the same exact caller surface and refuses foreign owners",()=>{
 const database=new Database(":memory:");
 try {
  const query=database.query(`with input as(select json(?1) request,json_extract(?2,'$') retained),
   context as(select request,coalesce(json_extract(request,'$.viewState'),retained) value from input),
   selected as(select request,value,json_extract(value,'$.activeModeId') mode,coalesce(json_extract(value,'$.windowId'),json_extract(value,'$.focusedWindowId')) window from context),
   bound as(select *,coalesce((select json_extract(item.value,'$.windowKindId') from json_each(selected.value,'$.windowInstances') item where json_extract(item.value,'$.id')=window limit 1),json_extract(value,'$.activeWindowKindId')) kind from selected)
   select case when json_type(request,'$.invocation.action')='object' and (mode is null or window is null) then 'plugin.unavailable'
    when json_type(request,'$.invocation.action')='object' and json_extract(request,'$.invocation.action.address.windowKindId')!='*' and kind is not json_extract(request,'$.invocation.action.address.windowKindId') then 'input.invalid'
    when json_type(request,'$.invocation.command.address.owner.mode')='object' and mode is not json_extract(request,'$.invocation.command.address.owner.mode.modeId') then 'plugin.unavailable'
    else null end fault,mode,window,json_extract(value,'$.locale') locale from bound`);
  for(const row of corpus.contextCases){
   const request=structuredClone(corpus.cases[row.request]!.request) as Record<string,unknown>;
   if("context" in row)request.viewState=row.context;
   const actual=query.get(JSON.stringify(request),JSON.stringify("retained" in row?row.retained:null)) as {fault:string|null;mode:string|null;window:string|null;locale:string|null};
   expect(actual.fault,row.id).toBe("expectedFault" in row?row.expectedFault:null);
   if("expectedWindow" in row)expect(actual.window,row.id).toBe(row.expectedWindow);
   if("expectedMode" in row)expect(actual.mode,row.id).toBe(row.expectedMode);
   if("expectedContext" in row)expect(actual.locale,row.id).toBe(row.expectedContext.locale);
  }
 } finally {database.close();}
});
