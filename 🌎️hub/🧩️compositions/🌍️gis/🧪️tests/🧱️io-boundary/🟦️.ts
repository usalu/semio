/** 🌍️ Consumer-owned GIS service declarations and receiver wiring. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {admitDocumentServiceDeclarationV1} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🟦️.ts";
import {admitDocumentServiceWireDeclarationV1,encodeDocumentServiceWireDeclarationV1} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🚪️io/🟦️.ts";
import {gisMapDocumentServiceDeclarationV1} from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client/🟦️.ts";
import {gisMapDocumentServiceWireDeclarationV1} from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client/🚪️io/🟦️.ts";

test("GIS native declarations preserve the installed service protocol through explicit IO",()=>{
  const typed=gisMapDocumentServiceDeclarationV1(),wire=gisMapDocumentServiceWireDeclarationV1();
  expect(typed.operations.length).toBe(6);expect(typeof typed.operations[0]!.inputSchema).toBe("object");expect(typeof wire.operations[0]!.inputSchema).toBe("string");
  expect(admitDocumentServiceWireDeclarationV1("gis",wire)).toEqual(typed);expect(encodeDocumentServiceWireDeclarationV1(typed)).toEqual(wire);
  expect(()=>admitDocumentServiceDeclarationV1("gis",wire)).toThrow();expect(()=>admitDocumentServiceWireDeclarationV1("gis",typed)).toThrow();
  for(const operation of wire.operations){const schema=JSON.parse(operation.inputSchema);expect(schema.$id.startsWith("gis:")).toBe(true);new Ajv({strict:true}).compile(schema);}
  console.log("[DEBUG] GIS six-operation declarations roundtrip explicit IO and independent Ajv");
});

test("GIS receiving configuration selects its owner and all fixture URLs resolve",()=>{
  const configuration=readFileSync(new URL("../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts",import.meta.url),"utf8");
  expect(configuration).toContain("🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker");
  const path=new URL("../../../../../✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts",import.meta.url);
  for(const match of readFileSync(path,"utf8").matchAll(/new URL\("([^"]+)",\s*import.meta.url\)/g))expect(()=>readFileSync(new URL(match[1]!,path))).not.toThrow();
  console.log("[DEBUG] GIS receiver configuration and every fixture/schema URL resolve at the composition owner");
});

test("GIS receiving schema compilers select the canonical test IO owner",()=>{
  const owner=new URL("../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧰️schema/🟦️.ts",import.meta.url);
  for(const name of ["🌉️component-cold-map-patch","💡️inference-control","📇️native-codecs"]){
    const path=new URL(`../${name}/🟦️.ts`,import.meta.url),source=readFileSync(path,"utf8"),match=source.match(/import \{ compileGisScopeExport \} from "([^"]+)"/);
    expect(match).not.toBeNull();expect(new URL(match![1]!,path).href).toBe(owner.href);
    for(const link of source.matchAll(/from "(\.[^"]+)"/g))expect(()=>readFileSync(new URL(link[1]!,path))).not.toThrow();
  }
  console.log("[DEBUG] Three GIS receivers select canonical test-only schema IO and resolve their imports");
});
