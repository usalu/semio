import {expect,test} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {resolve} from "node:path";
import {parse} from "@iarna/toml";
import Ajv from "ajv";
import JSON5 from "json5";
import schema from "../../🧬️schema/🎬️draw-list/🔣️.json";
import themeSchema from "../../🧬️schema/🎨️theme-color-fields/🔣️.json";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";

const owner=resolve(import.meta.dir,"../.."),root=resolve(owner,"../../.."),old=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas");
test("canvas definitions have a General owner independent of Infinite",()=>{
 const source=readFileSync(resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8"),primary=Bun.TOML.parse(source),oracle=parse(source);
 expect(primary).toEqual(oracle);expect((primary.package as {name:string}).name).toBe("semio-framework-canvas");
 expect(source.includes("semio-framework-os-")).toBe(false);expect(source.includes("artifact-infinite-dag")).toBe(false);
 expect(existsSync(resolve(owner,"🦀️.rs"))).toBe(true);expect(existsSync(resolve(old,"🦀️.rs"))).toBe(false);
 for(const module of ["✍️editor","🗺️surface"]){const consumer=readFileSync(resolve(root,"🧰️framework/🔨️modules",module,"📦️packages/🦀️rust/Cargo.toml"),"utf8");expect(consumer.includes('package = "semio-framework-canvas"')).toBe(true);expect(consumer.includes('semio-framework-os-infinite')).toBe(false);}
});
test("the original draw list corpus follows the variable production command contract",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 const input=readFileSync(resolve(owner,"🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json"),"utf8"),value=JSON.parse(input);
 expect(validate(value)).toBe(true);expect(value).toEqual(JSON5.parse(input));
 expect(validate({version:1,truncated:false,commands:[]})).toBe(true);
 for(const value of [{version:2,truncated:false,commands:[]},{version:1,truncated:false,commands:[["po",0]]},{version:1,truncated:false,commands:[["f",0,[256,0,0,0],[1,0,0,1,0,0],["r",0,0,1,1]]]},{version:1,truncated:false,commands:[["pc",0,[1,0,0,1,0],["r",0,0,1,1]]]}])expect(validate(value)).toBe(false);
});

test("theme fields follow owned color bytes and independent schema oracles",()=>{
 const input=readFileSync(resolve(owner,"🧫️fixtures/🎨️theme-color-fields/🔣️.json"),"utf8"),fixture=JSON.parse(input),oracle=new Ajv({strict:true,allErrors:true}).compile(themeSchema);
 expect(fixture).toEqual(JSON5.parse(input));expect(fixture.modelSchema).toBe(themeSchema.$id);
 for(const row of fixture.cases){expect(validateJsonSchemaSubset(themeSchema,row.fields)).toEqual([]);expect(oracle(row.fields)).toBe(true);}
 for(const fields of [{paint:[1,2,3]},{paint:[1,2,3,4,5]},{paint:[-1,0,0,0]},{paint:[0,0,0,256]},{paint:[0,0,0,0.5]},{paint:null}]){expect(validateJsonSchemaSubset(themeSchema,fields).length>0).toBe(true);expect(oracle(fields)).toBe(false);}
 const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8");
 expect(source.includes("pub trait ThemeColorFields")).toBe(true);expect(source.includes("pub fn color_from_json_rgba8")).toBe(false);expect(source.includes("fields: &impl ThemeColorFields")).toBe(true);
});

test("backend implementation stays behind owned Canvas ports",()=>{
 const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8");
 expect(source.includes("pub fn vello_scene(")).toBe(false);
 expect(source.includes("impl From<Cap> for kurbo::Cap")).toBe(false);
 expect(source.includes("impl From<Join> for kurbo::Join")).toBe(false);
 expect(source.includes("impl From<FillRule> for peniko::Fill")).toBe(false);
 expect(source.includes("impl From<BlendMode> for peniko::Mix")).toBe(false);
 for(const name of ["usvg_options_icons","render_svg_tree_literal","svg_icon_content_bounds","render_svg_tree_themed","usvg_options_map_labels"])expect(source.includes("pub fn "+name+"(")).toBe(false);
 expect(source.includes("pub struct CanvasAttachSession")).toBe(true);
 expect(source.includes("env!(\"OUT_DIR\")")).toBe(false);
 expect(existsSync(resolve(owner,"../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs"))).toBe(true);
});
