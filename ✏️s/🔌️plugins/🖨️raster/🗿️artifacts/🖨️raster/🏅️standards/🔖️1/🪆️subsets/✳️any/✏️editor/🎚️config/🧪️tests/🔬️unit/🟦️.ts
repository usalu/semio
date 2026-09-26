/** 🧪️ Shared brush configuration vectors against JSON Schema and libvips color parsing. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {parseRasterConfig} from "../../🧬️schema/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🖌️brush-style/🔣️.json";

const validate=new Ajv({strict:false,validateFormats:false}).compile(schema);
const config=(brushColor:string,brushHardness:number)=>({brushSize:24,brushOpacity:1,brushColor,brushHardness,camera:{x:0,y:0,zoom:1}});
for(const row of fixture.cases) test(`brush style ${row.color} at hardness ${row.hardness}`,async()=>{
  const value=config(row.color,row.hardness);
  expect(validate(value)).toBe(true);
  expect(parseRasterConfig(value)).toEqual({...value,compositeViewport:undefined});
  const pixels=await sharp({create:{width:1,height:1,channels:4,background:row.color}}).raw().toBuffer();
  expect([...pixels]).toEqual(row.rgba);
});
test("brush style schema and parser reject malformed colors and hardness",()=>{
  for(const color of fixture.invalidColors){
    const value=config(color,0.5);
    expect(validate(value)).toBe(false);
    expect(()=>parseRasterConfig(value)).toThrow();
  }
  for(const hardness of [...fixture.invalidHardness,NaN,Infinity]){
    const value=config("#2878dc",hardness);
    if(Number.isFinite(hardness)) expect(validate(value)).toBe(false);
    expect(()=>parseRasterConfig(value)).toThrow();
  }
});
