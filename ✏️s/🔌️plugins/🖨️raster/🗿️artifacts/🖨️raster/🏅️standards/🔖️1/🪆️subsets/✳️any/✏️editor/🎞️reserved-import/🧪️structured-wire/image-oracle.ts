import { inflateSync } from "node:zlib";
import Ajv2020 from "ajv/dist/2020.js";
const schema = await Bun.file(new URL("./image-schema.json", import.meta.url)).json();
const fixture = await Bun.file(new URL("./image-cases.json", import.meta.url)).json();
const validate = new Ajv2020().compile(schema);
if (!validate(fixture)) throw Error(JSON.stringify(validate.errors));
for (const row of fixture.cases) {
 const data = Buffer.from(row.base64, "base64");
 if (data.subarray(0,8).toString("hex") !== "89504e470d0a1a0a") throw Error("signature");
 let at=8, header, pixels, metadata=[];
 while (at<data.length) { const n=data.readUInt32BE(at), kind=data.toString("ascii",at+4,at+8), payload=data.subarray(at+8,at+8+n); at+=n+12; if(kind==="IHDR") header=payload; if(kind==="IDAT") pixels=inflateSync(payload); if(kind==="tEXt"){const z=payload.indexOf(0);metadata.push([payload.subarray(0,z).toString("latin1"),payload.subarray(z+1).toString("latin1")]);} }
 if(header.readUInt32BE(0)!==row.width||header.readUInt32BE(4)!==row.height||header[8]!==row.bitDepth||header[9]!==6||pixels[0]!==0||JSON.stringify([...pixels.subarray(1)])!==JSON.stringify(row.rgba)||JSON.stringify(metadata)!==JSON.stringify(row.metadata)) throw Error(row.name);
}
for(const row of fixture.errors){let failed=false;try{atob(row.text)}catch{failed=true}const index=[...row.text].findIndex(c=>!/[A-Za-z0-9+/=]/.test(c));if(!failed||index!==row.index||row.text.charCodeAt(index)!==row.byte)throw Error(row.name)}
console.log("Raster PNG import oracle: " + fixture.cases.length + " images, " + fixture.errors.length + " failures");
