import {expect,test} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {resolve} from "node:path";
import {parse} from "@iarna/toml";
import Ajv from "ajv";
import JSON5 from "json5";
/** 🗂️ Reads one TOML document as a table of unknown cells, so a test must narrow before it reads a nested cell. */
const tomlTable=(text:string):Readonly<Record<string,unknown>>=>Object.fromEntries(Object.entries(Bun.TOML.parse(text)));

import schema from "../../🧬️schema/🎬️draw-list/🔣️.json";
import themeSchema from "../../🧬️schema/🎨️theme-color-fields/🔣️.json";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import {readCatalogSources,renderCatalogArtifacts} from "@semio-tech/assets/icons/builder/projection";
import {RUNTIME_ASSET_CHANNELS,RUNTIME_ASSET_FILES,installRuntimeAssets,runtimeAssetDeliveries} from "../../🔌️runtime-assets/🟦️.ts";

const owner=resolve(import.meta.dir,"../.."),root=resolve(owner,"../../.."),old=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas");
test("canvas definitions have a General owner independent of Infinite",()=>{
 const source=readFileSync(resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8"),primary=tomlTable(source),oracle=parse(source);
 expect(primary).toEqual(oracle);expect((primary.package as {name:string}).name).toBe("semio-framework-canvas");
 expect(source.includes("semio-framework-os-")).toBe(false);expect(source.includes("artifact-infinite-dag")).toBe(false);
 expect(existsSync(resolve(owner,"🦀️.rs"))).toBe(true);expect(existsSync(resolve(old,"🦀️.rs"))).toBe(false);
 for(const [module,package_] of [["✍️editor","📦️packages/🦀️rust"],["🗺️surface","🎨️paint/📦️packages/🦀️rust"]]){const consumer=readFileSync(resolve(root,"🧰️framework/🔨️modules",module,package_,"Cargo.toml"),"utf8");expect(consumer.includes('package = "semio-framework-canvas"')).toBe(true);expect(consumer.includes('semio-framework-os-infinite')).toBe(false);}
});
test("the original draw list corpus follows the variable production command contract",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 const input=readFileSync(resolve(owner,"🧪️tests/🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json"),"utf8"),value=JSON.parse(input);
 expect(validate(value)).toBe(true);expect(value).toEqual(JSON5.parse(input));
 expect(validate({version:1,truncated:false,commands:[]})).toBe(true);
 for(const value of [{version:2,truncated:false,commands:[]},{version:1,truncated:false,commands:[["po",0]]},{version:1,truncated:false,commands:[["f",0,[256,0,0,0],[1,0,0,1,0,0],["r",0,0,1,1]]]},{version:1,truncated:false,commands:[["pc",0,[1,0,0,1,0],["r",0,0,1,1]]]}])expect(validate(value)).toBe(false);
});

test("theme fields follow owned color bytes and independent schema oracles",()=>{
 const input=readFileSync(resolve(owner,"🧪️tests/🧫️fixtures/🎨️theme-color-fields/🔣️.json"),"utf8"),fixture=JSON.parse(input),oracle=new Ajv({strict:true,allErrors:true}).compile(themeSchema);
 expect(fixture).toEqual(JSON5.parse(input));expect(fixture.modelSchema).toBe(themeSchema.$id);
 for(const row of fixture.cases){expect(validateJsonSchemaSubset(themeSchema,row.fields)).toEqual([]);expect(oracle(row.fields)).toBe(true);}
 for(const fields of [{paint:[1,2,3]},{paint:[1,2,3,4,5]},{paint:[-1,0,0,0]},{paint:[0,0,0,256]},{paint:[0,0,0,0.5]},{paint:null}]){expect(validateJsonSchemaSubset(themeSchema,fields).length>0).toBe(true);expect(oracle(fields)).toBe(false);}
 const source=readModuleTreeSource(resolve(owner,"🎬️scene/🦀️.rs"));
 expect(source.includes("pub trait ThemeColorFields")).toBe(true);expect(source.includes("pub fn color_from_json_rgba8")).toBe(false);expect(source.includes("fields: &impl ThemeColorFields")).toBe(true);
});

test("backend implementation stays behind owned Canvas ports",()=>{
 const source=readModuleTreeSource(resolve(owner,"🎬️scene/🦀️.rs"));
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

const crates=["🎬️scene","📷️camera","🪪️icon-codec","🔌️runtime-assets"],assets=resolve(root,"🧰️framework/🔨️modules/🖼️assets"),fixture=JSON.parse(readFileSync(resolve(owner,"🔌️runtime-assets/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
test("the facade re-exports its producer crates explicitly and embeds no asset",()=>{
 const facade=readFileSync(resolve(owner,"🦀️.rs"),"utf8");
 expect(/pub use [^;]*::\*;/.test(facade)).toBe(false);
 for(const crate of ["scene","camera","icons","assets"])expect(facade.includes("semio_framework_canvas_"+crate+"::")).toBe(true);
 expect(parse(readFileSync(resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8")).lib).toEqual({name:"semio_framework_canvas",path:"../../🦀️.rs"});
 for(const crate of crates){
  const tree=readModuleTreeSource(resolve(owner,crate,"🦀️.rs"));
  expect(tree.includes("include_bytes!"),crate).toBe(false);
  expect(tree.match(/include_str!\(/g)?.length??0,crate).toBe(crate==="🎬️scene"?1:0);
 }
 expect(existsSync(resolve(owner,"🖼️assets/😀️NotoColorEmoji-subset.ttf"))).toBe(false);
});

test("the producer crates form a downward graph that never depends on the facade",()=>{
 const dependencies=(crate:string)=>Object.keys(tomlTable(readFileSync(resolve(owner,crate,"📦️packages/🦀️rust/Cargo.toml"),"utf8")).dependencies as Record<string,unknown>);
 const names:Record<string,string>={"🔌️runtime-assets":"semio-framework-canvas-assets","🎬️scene":"semio-framework-canvas-scene","📷️camera":"semio-framework-canvas-camera","🪪️icon-codec":"semio-framework-canvas-icons"};
 for(const crate of crates)expect(dependencies(crate)).not.toContain("semio-framework-canvas");
 expect(dependencies("🔌️runtime-assets").filter(name=>name.startsWith("semio-framework-canvas"))).toEqual([]);
 expect(dependencies("🎬️scene").filter(name=>name.startsWith("semio-framework-canvas"))).toEqual([names["🔌️runtime-assets"]]);
 expect(dependencies("🪪️icon-codec").filter(name=>name.startsWith("semio-framework-canvas"))).toEqual([names["🔌️runtime-assets"]]);
 expect(dependencies("📷️camera").filter(name=>name.startsWith("semio-framework-canvas"))).toEqual([]);
});

test("the runtime asset channels equal the shared fixture, the Rust catalog and the files on disk",()=>{
 expect(RUNTIME_ASSET_CHANNELS.map(({id,route,directory})=>({id,route,directory}))).toEqual(fixture.channels);
 expect(RUNTIME_ASSET_FILES.map(({channel,file})=>({channel,file}))).toEqual(fixture.assets.map((row:{channel:string;file:string})=>({channel:row.channel,file:row.file})));
 expect(runtimeAssetDeliveries()).toEqual(fixture.channels.map((channel:{route:string;directory:string})=>({kind:"static-dir",route:channel.route,root:channel.directory})));
 const rust=readFileSync(resolve(owner,"🔌️runtime-assets/🦀️.rs"),"utf8");
 for(const channel of fixture.channels)expect(rust.includes('"'+channel.directory+'"'),channel.id).toBe(true);
 expect(rust.includes("pub const VERSION: u32 = "+fixture.iconPackVersion+";")).toBe(true);
 for(const row of fixture.assets){
  const bytes=readFileSync(resolve(root,fixture.channels.find((channel:{id:string})=>channel.id===row.channel).directory,row.file));
  if(row.bytes!==undefined)expect(bytes.length,row.role).toBe(row.bytes);
  if(row.sfntTag!==undefined)expect(Buffer.from(bytes.subarray(0,4)).toString("hex"),row.role).toBe(row.sfntTag);
  if(row.channel!=="compiler-fonts")expect(rust.includes('"'+row.file+'"'),row.role).toBe(true);
 }
});

test("the icon pack carries exactly the vendored SVG sources, byte for byte",()=>{
 const pack=JSON.parse(readFileSync(resolve(assets,"🔣️icons/🤖️generated/🔤️shortcodes/🔣️.json"),"utf8"));
 expect(pack.version).toBe(fixture.iconPackVersion);
 const read=(directory:string)=>Object.fromEntries(readCatalogSources(directory).map(({id,path})=>[id,readFileSync(resolve(directory,path),"utf8")]));
 expect(pack.plain).toEqual(read(resolve(assets,"🔣️icons")));
 expect(pack.themed).toEqual(read(resolve(assets,"🌱️metabolism/🔣️icons")));
 for(const row of fixture.iconPackSamples.rows)expect(pack[row.kind][row.id]).toBe(readFileSync(resolve(root,fixture.iconPackSamples.sourceRoot,row.source),"utf8"));
 const artifact=renderCatalogArtifacts("rust").find(row=>row.path.endsWith("🔤️shortcodes/🔣️.json"));
 expect(JSON.parse(artifact!.content)).toEqual(pack);
 const emoji=readFileSync(resolve(assets,"🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs"),"utf8");
 expect(emoji.includes("include_str!")).toBe(false);
 expect(emoji.includes("pub fn icon_shortcode_emoji(")).toBe(true);
});

test("the installer fetches the seven channel files and refuses a partial set",async()=>{
 const base="https://canvas.invalid/index.html",calls:Uint8Array[][]=[],requested:string[]=[];
 const bytes=(file:string)=>Uint8Array.from(Array.from(file,character=>character.codePointAt(0)!&255));
 const delivered=await installRuntimeAssets({semioSetRuntimeAssets:(...assets:Uint8Array[])=>void calls.push(assets)},base,async url=>{requested.push(url);return new Response(bytes(decodeURIComponent(url.slice(url.lastIndexOf("/")+1))));});
 expect(delivered).toBe(true);expect(requested).toHaveLength(7);expect(calls).toHaveLength(1);expect(calls[0]).toEqual(RUNTIME_ASSET_FILES.map(({file})=>bytes(file)));
 let installs=0;
 await expect(installRuntimeAssets({semioSetRuntimeAssets:()=>void installs++},base,async url=>url.includes(encodeURIComponent("🔣️.json"))?new Response("no",{status:404}):new Response(new Uint8Array(1)))).rejects.toThrow(/runtime-assets: 404/u);
 expect(installs).toBe(0);
 expect(await installRuntimeAssets({},base,async()=>{throw Error("must not fetch");})).toBe(false);
});
