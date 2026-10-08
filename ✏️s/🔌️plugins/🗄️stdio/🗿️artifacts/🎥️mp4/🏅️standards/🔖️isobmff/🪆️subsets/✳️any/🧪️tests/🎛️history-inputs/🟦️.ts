import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
const root=resolve(import.meta.dir,"../..");const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const before={schema:"stdio.mp4",ftyp:{majorBrand:"isom",minorVersion:0,compatibleBrands:[]},movie:{creationTime:0,modificationTime:0,timescale:1000,duration:0,rate:65536,volume:256,matrix:[65536,0,0,0,65536,0,0,0,1073741824],nextTrackId:1,title:"One",encoder:null},tracks:[]};
const mutation={mutation:"patchSnapshot",patch:{operation:"set",path:"/movie/title",value:"Two"}};
test("MP4 committed movie title intent agrees with independent RFC6902",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(read("../../../../../../📇️registry/🧬️schema/🔣️.json"));const schema=read("🧬️schema/📸️snapshot/🔣️.json");const title=ajv.compile(schema.$defs.Movie.properties.title);const patch=ajv.compile({$ref:"https://json.schemas.assets.semio-tech.com/s/stdio/registry/schema.json#/$defs/SnapshotPatch"});
 expect(title(before.movie.title)).toBe(true);expect(title(mutation.patch.value)).toBe(true);expect(patch(mutation.patch)).toBe(true);
 const after=applyPatch(structuredClone(before),[{op:"replace",path:"/movie/title",value:"Two"}],true,true).newDocument;expect(after.movie.title).toBe("Two");expect(after.tracks).toEqual(before.tracks);expect(after.ftyp).toEqual(before.ftyp);
 const path=resolve(root,"🧫️fixtures/🧬️history-edits/title/🦠️mutation/🔣️.json");expect(existsSync(path)).toBe(true);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation,before,after});
 console.log("[DEBUG] MP4 movie title intent agrees with RFC6902 and leaves media owners unchanged");
});
