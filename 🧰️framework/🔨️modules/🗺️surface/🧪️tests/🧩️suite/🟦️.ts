import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";
import fixture from "../../🧫️fixtures/📇️bindings.json";
import ports from "../../🧫️fixtures/🔌️ports/🔣️.json";
import { parse as parseToml } from "@iarna/toml";
import { createHash } from "node:crypto";
import { parse as parseProtobuf } from "protobufjs";
import tileCorpus from "../../🗺️tiled-map/🧫️fixtures/🗺️vector-tiles/🔣️.json";
import ownership from "../../🧫️fixtures/🔌️ownership/🔣️.json";

type CargoManifest = {package: {name: string}; lib?: {name?: string}; dependencies: Record<string, {path?: string}>};
type TileInventory = {layers: {name: string; extent: number; features: {type: number}[]}[]};

test("surface neutral owner survives deleting each specific graph composition",()=>{
  const owner=resolve(import.meta.dir,"../.."),source=readFileSync(join(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8");
  const manifest=Bun.TOML.parse(source);expect(manifest).toEqual(parseToml(source));
  const visit=(value:unknown):void=>{if(value&&typeof value==="object"){for(const[key,child]of Object.entries(value)){expect(ownership.specificDependencies).not.toContain(key);if(child&&typeof child==="object"&&"package"in child)expect(ownership.specificDependencies).not.toContain(child.package);visit(child);}}};visit(manifest);
  const graph=readFileSync(join(owner,"🕸️node-graph/🦀️.rs"),"utf8");
  for(const name of ownership.neutralTypes)expect(graph).toContain(name);
  expect(graph).not.toContain("semio_framework_os_");expect(graph).not.toContain("semio_framework_artifact_infinite_dag");expect(graph).not.toContain("pub struct GraphHost");
});

test("surface retained refusal interfaces use the defining General Value owner",()=>{
 const owner=resolve(import.meta.dir,"../.."),manifestSource=readFileSync(join(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8"),manifestValue=Bun.TOML.parse(manifestSource);expect(manifestValue).toEqual(parseToml(manifestSource));const manifest=manifestValue as CargoManifest;const path=manifest.dependencies["semio-framework-value"]!.path!;expect(path).toBe("../../../🌱️value/📦️packages/🦀️rust");const text=readFileSync(resolve(owner,"📦️packages/🦀️rust",path,"Cargo.toml"),"utf8"),provider=Bun.TOML.parse(text)as CargoManifest;expect(provider).toEqual(parseToml(text));expect(provider.package.name).toBe("semio-framework-value");
 const cursor=readFileSync(join(owner,"🕸️node-graph/📡️scene/🦀️.rs"),"utf8");expect(cursor).toContain("Result<SceneDecodeStep,ValueError>");expect(cursor).toContain("ValueError::literal");expect(cursor).not.toContain("serde_json::Error");expect(cursor).not.toContain("store::");
});

test("surface vector tile fixtures belong to their owner and agree with independent protobuf inventory", () => {
  const owner = resolve(import.meta.dir, "../../🗺️tiled-map");
  const type = parseProtobuf(readFileSync(join(owner, "🧬️schema/🛰️.proto"), "utf8")).root.lookupType("surface.vector_tile.Tile");
  for (const row of tileCorpus.tiles) {
    const bytes = readFileSync(join(owner, "🧫️fixtures/🗺️vector-tiles", row.file));
    expect(bytes.length).toBe(row.bytes);
    expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);
    const tile = type.toObject(type.decode(bytes), {defaults: true, arrays: true}) as TileInventory;
    expect(tile.layers.map(layer => ({name: layer.name, extent: layer.extent, features: layer.features.length, geometryTypes: [0, 1, 2, 3].map(kind => layer.features.filter(feature => feature.type === kind).length)}))).toEqual(row.layers);
  }
  const source = readFileSync(join(owner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  expect(source.includes("MAP-VECTOR-TILES")).toBe(false);
  expect(source.includes("#[ignore")).toBe(false);
  console.log("[DEBUG] Owned vector tile bytes and layer/feature/geometry inventories agree with Protobuf.js across all six original tiles");
});

test("surface neutral interaction interfaces come directly from their defining packages", () => {
  const packageRoot = resolve(import.meta.dir, "../../📦️packages/🦀️rust");
  const parse = (path: string): CargoManifest => { const source = readFileSync(path, "utf8"), value = Bun.TOML.parse(source); expect(value).toEqual(parseToml(source)); return value as CargoManifest; };
  const manifest = parse(join(packageRoot, "Cargo.toml"));
  const source = readFileSync(resolve(import.meta.dir, "../../🕸️node-graph/🦀️.rs"), "utf8");
  for (const port of ports.ports) {
    const dependency = manifest.dependencies[port.providerPackage];
    expect(typeof dependency?.path).toBe("string");
    if (typeof dependency?.path !== "string") throw new Error("Surface port requires a direct provider path");
    const provider = parse(resolve(packageRoot, dependency.path, "Cargo.toml"));
    expect(provider.package.name).toBe(port.providerPackage);
    const crate = provider.lib?.name ?? port.providerPackage.replaceAll("-", "_");
    const imports = [...source.matchAll(new RegExp(`^use ${crate}::(?:\\{([^}]+)\\}|([A-Za-z_][A-Za-z_0-9]*))\\s*;`, "gm"))].flatMap(match => (match[1] ?? match[2]!).split(",").map(name => name.trim()));
    expect(imports).toContain(port.name);
  }
});


test("surface compiler companions keep their exact paired identity in the handpicked output owner", () => {
  
  const sourceRoot = join(import.meta.dir, "../../📦️packages/🦀️rust");
  const output = join(sourceRoot, fixture.directoryName);
  const names = [fixture.module, fixture.types, fixture.wasm, fixture.wasmTypes];
  expect(existsSync(join(sourceRoot, "pkg"))).toBe(false);
  expect(readdirSync(output).sort()).toEqual([...names, ".gitignore", "package.json"].sort());
  const js = readFileSync(join(output, fixture.module), "utf8");
  expect(js).toContain(`@ts-self-types="./${fixture.types}"`);
  expect(js).toContain(`new URL('${fixture.wasm}', import.meta.url)`);
  const resolved = ts.resolveModuleName(`./${fixture.module}`, join(output, "consumer.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler }, ts.sys).resolvedModule;
  expect(resolved?.resolvedFileName).toBe(join(output, fixture.types));
  expect(WebAssembly.validate(readFileSync(join(output, fixture.wasm)))).toBe(true);
  const manifest = JSON.parse(readFileSync(join(sourceRoot, "package.json"), "utf8"));
  expect(manifest.exports["."]).toBe(`./${fixture.directoryName}/${fixture.module}`);
  expect(manifest.exports[`./${fixture.directoryName}/${fixture.module}`]).toBe(manifest.exports["."]);
  const producer = readFileSync(join(sourceRoot, "📜️script.ts"), "utf8");
  expect(producer).toContain(`outputDirectory: "${fixture.directoryName}"`);
}, 30_000);
