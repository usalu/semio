/** 🌐️ The sealed document, CDN artifact, stack bundle, readiness gate and response header tests. */
import { createRequire } from "node:module";
import type { Readable, Writable } from "node:stream";
import { sha256 } from "@noble/hashes/sha2.js";
import { writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import Ajv from "ajv";
import { expect, it } from "vitest";
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };
import bundleCases from "../../../🧫️fixtures/🚛️bundle/🔣️.json" with { type: "json" };
import bundlePolicySchema from "../../../🧫️fixtures/🚛️bundle/🧬️schema/🔣️.json" with { type: "json" };
import { saveStackBundleImages, verifyStackBundleArchive, stackBundleImages, stackBundlePlan } from "../../../🚀️deploy/🎁️stack-bundle/🟦️.ts";
import readiness from "../../../🧫️fixtures/🚀️readiness/🔣️.json" with { type: "json" };
import { COURSE_BUNDLE_DIRECTORY, COURSE_BUNDLE_IMAGE, COURSE_PAGES_DIRECTORY, COURSE_PROCTOR_ORIGIN, COURSE_SITE_BUDGET, COURSE_SITE_HEADERS, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, hostZone } from "../../../🚀️deploy/🟦️.ts";
import { siteArtifactProblems, siteDocumentProblems } from "../../../🚀️deploy/🌐️site/🟦️.ts";
import { stackBundleEnvironment, stackBundleReadme } from "../../../🚀️deploy/🎁️stack-bundle/🟦️.ts";
import { deployCheckSteps } from "../../../🚀️deploy/⚖️readiness/🟦️.ts";
import { BOOT, STYLE, compose, digest, read, released, sealed, site, tree } from "../🧰️deploy-fixtures/🟦️.ts";
import type { Service } from "../🧰️deploy-fixtures/🟦️.ts";

type BundleTarHeader={name:string;size?:number;type?:"file"|"directory";mode?:number;pax?:Record<string,string>};
interface BundleTarWriter extends Readable {entry(header:BundleTarHeader,body:Buffer,complete:(error?:Error|null)=>void):void;finalize():void}
const tarStream=createRequire(join(site,"📦️packages/🟦️typescript/package.json"))("tar-stream") as {pack():BundleTarWriter;extract():Writable};

/** 🚛 Registers the sealed document, CDN artifact, stack bundle, readiness gate and response header tests. */
export function registerSealedPublicationAndBundleTests(): void {
  it("accepts exactly a document sealed for the baked proctor origin", () => {
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteDocumentProblems(sealed("https://localhost:18443"), "https://localhost:18443")).toEqual([]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN), "https://staging.example")).toEqual([`connect-src admits ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}, not exactly https://staging.example wss://staging.example`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(/\s*<meta http-equiv[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document declares no Content-Security-Policy"]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(BOOT, `${BOOT};fetch('/')`), COURSE_PROCTOR_ORIGIN)).toEqual([`script-src does not admit the inline script ${digest(`${BOOT};fetch('/')`)}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace(STYLE, "body{margin:1px}"), COURSE_PROCTOR_ORIGIN)).toEqual([`style-src does not admit the inline style ${digest("body{margin:1px}")}`]);
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN).replace("<html>", "<html lang=\"en\">").replace(/\s*<meta name="description"[^>]*>/u, ""), COURSE_PROCTOR_ORIGIN)).toEqual(["the document has no description", "the document declares a default language"]);
    const moved = sealed(COURSE_PROCTOR_ORIGIN);
    const meta = /\s*<meta http-equiv[^>]*>/u.exec(moved)![0];
    expect(siteDocumentProblems(moved.replace(meta, "").replace("<style>", `${meta.trim()}<style>`), COURSE_PROCTOR_ORIGIN)).toEqual(["the Content-Security-Policy comes after the first script"]);
    const loose = `default-src *; script-src 'self' 'unsafe-inline' 'unsafe-eval' https:; style-src 'self' 'unsafe-inline'; connect-src *; object-src 'self'`;
    expect(siteDocumentProblems(sealed(COURSE_PROCTOR_ORIGIN, loose), COURSE_PROCTOR_ORIGIN)).toEqual(["default-src admits *", "script-src admits 'unsafe-inline'", "script-src admits 'unsafe-eval'", "script-src admits https:", "style-src admits 'unsafe-inline'", `script-src does not admit the inline script ${digest(BOOT)}`, `style-src does not admit the inline style ${digest(STYLE)}`, `connect-src admits *, not exactly ${COURSE_PROCTOR_ORIGIN} wss://${deployment.proctor.host}`, "object-src is not 'none'", "base-uri is not 'none'", "form-action is not restricted"]);
  });
  it("publishes exactly a CDN artifact that bakes the proctor origin", () => {
    expect(siteArtifactProblems(tree(released(COURSE_PROCTOR_ORIGIN)), COURSE_PROCTOR_ORIGIN)).toEqual([]);
    expect(siteArtifactProblems(tree(released("https://localhost:18443")), "https://localhost:18443")).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), CNAME: "quizze.example\n" }), COURSE_PROCTOR_ORIGIN)).toEqual([`CNAME names "quizze.example", not ${deployment.site.host}`]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/dev-AbCd1234.js": "fetch('http://127.0.0.1:8791')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["the site names 127.0.0.1 outside the baked proctor origin"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "404.html": "<!doctype html>" }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is not the document of index.html"]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js.map": "{}", "assets/chunk.js": "//# sourceMappingURL=chunk.js.map\nconsole.log('[DEBUG] x')" }), COURSE_PROCTOR_ORIGIN)).toEqual(["assets/app-AbCd1234.js.map carries no content hash, yet /assets/* is served as immutable", "assets/chunk.js carries no content hash, yet /assets/* is served as immutable", "assets/app-AbCd1234.js.map is a source map", "the site carries the development leftover sourceMappingURL=", "the site carries the development leftover [DEBUG]"]);
    const { "assets/app-EfGh5678.css": _stylesheet, "robots.txt": _robots, ...partial } = released(COURSE_PROCTOR_ORIGIN);
    expect(siteArtifactProblems(tree(partial), COURSE_PROCTOR_ORIGIN)).toEqual(["robots.txt is missing", "the document loads assets/app-EfGh5678.css, which the artifact lacks"]);
    expect(siteArtifactProblems(tree({ "index.html": sealed(COURSE_PROCTOR_ORIGIN), CNAME: `${deployment.site.host}\n` }), COURSE_PROCTOR_ORIGIN)).toEqual(["404.html is missing", ".nojekyll is missing", "robots.txt is missing", "manifest.webmanifest is missing", "favicon.svg is missing", "the document loads assets/app-AbCd1234.js, which the artifact lacks", "the document loads assets/app-EfGh5678.css, which the artifact lacks", `no script bakes the proctor origin ${COURSE_PROCTOR_ORIGIN}`]);
    const heavy = `${released(COURSE_PROCTOR_ORIGIN)["assets/app-AbCd1234.js"] as string}\n/* ${randomBytes(COURSE_SITE_BUDGET.scriptGzipBytes * 2).toString("base64")} */`;
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "assets/app-AbCd1234.js": heavy }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the document's scripts weigh \d+ bytes compressed, over the budget of \d+$/u)]);
    expect(siteArtifactProblems(tree({ ...released(COURSE_PROCTOR_ORIGIN), "🖼️assets/font.woff2": randomBytes(COURSE_SITE_BUDGET.totalBytes) }), COURSE_PROCTOR_ORIGIN)).toEqual([expect.stringMatching(/^the artifact weighs \d+ bytes, over the budget of \d+$/u)]);
  });
  it("bundles the proctor stack for a host without a registry: the tag pinned, the defaults of compose.yaml listed, every step told", () => {
    const defaults = Object.fromEntries([...read("compose.yaml").matchAll(/\$\{([A-Z_]+):-([^}]*)\}/gu)].map((substitution) => [substitution[1]!, substitution[2]!]));
    expect(Object.keys(defaults).sort()).toEqual(["COURSE_HTTPS_PORT", "COURSE_HTTP_PORT", "PROCTOR_ALLOWED_ORIGINS", "PROCTOR_HOST", "PROCTOR_TAG"]);
    const settings = (tag: string): string[] => stackBundleEnvironment(tag).split("\n").filter((line) => line.includes("="));
    expect(settings(defaults.PROCTOR_TAG!).map((line) => line.replace(/^# /u, "")).sort()).toEqual(Object.entries(defaults).map(([name, value]) => `${name}=${value}`).sort());
    expect(settings("sha-0123456789ab").filter((line) => !line.startsWith("#"))).toEqual(["PROCTOR_TAG=sha-0123456789ab"]);
    expect(stackBundleEnvironment("latest")).toBe(`# Overrides of compose.yaml; both services read this file as their environment.\nPROCTOR_TAG=latest\n# PROCTOR_HOST=${deployment.proctor.host}\n# PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}\n# COURSE_HTTP_PORT=80\n# COURSE_HTTPS_PORT=443\n`);
    const readme = stackBundleReadme("sha-0123456789ab", compose.services.caddy!.image, bundleCases.proxy.image);
    const policy = new Ajv({ strict: true }).compile(bundlePolicySchema);
    expect(policy({ proctor: bundleCases.proctor, proxy: bundleCases.proxy })).toBe(true);
    for (const row of bundleCases.cases) {
      const images = stackBundleImages(row.tag, read("compose.yaml"));
      expect(images).toEqual(row.images);
      expect(images).toEqual(Object.values(compose.services).map(service => service.image.replace("${PROCTOR_TAG:-latest}", row.tag)));
      const plan = stackBundlePlan(row.tag, read("compose.yaml"));
      expect(plan.images).toEqual([...row.images, bundleCases.proxy.image]);
      expect(plan.caddy).toBe(bundleCases.proxy.image);
      const staged = parse(plan.compose) as { services: Record<string, Service> };
      expect(staged.services.caddy).toEqual({ ...compose.services.caddy, ...bundleCases.proxy });
      expect(staged.services.proctor).toEqual({ ...compose.services.proctor, ...bundleCases.proctor });
      expect(policy({ proctor: { pull_policy: staged.services.proctor!.pull_policy }, proxy: { image: staged.services.caddy!.image, pull_policy: staged.services.caddy!.pull_policy } })).toBe(true);
      for (const image of plan.images) expect(stackBundleReadme(row.tag, images[1], plan.caddy)).toContain(image);
    }
    for (const row of bundleCases.invalidPolicies) expect(() => stackBundlePlan("latest", read("compose.yaml").replace(row.replace, row.with)), row.id).toThrow(row.error);
    for (const row of bundleCases.updates) {
      const guide = stackBundleReadme(row.tag, compose.services.caddy!.image, bundleCases.proxy.image);
      expect(guide).toContain("Update an existing stack from this bundle:");
      expect(guide).toContain("In the existing .env, set this image tag and keep every other override:");
      expect(guide).toContain(row.setting);
      expect(guide).not.toContain(`PROCTOR_TAG=${row.currentTag}`);
      const selected = /^PROCTOR_TAG=(.+)$/mu.exec(stackBundleEnvironment(row.tag))![1]!;
      expect(parse(stackBundlePlan(row.tag, read("compose.yaml")).compose).services.proctor.image.replace("${PROCTOR_TAG:-latest}", selected)).toBe(row.image);
      const runbook = readFileSync(join(site, "README.md"), "utf8");
      expect(runbook).toContain("set `PROCTOR_TAG` in the retained `.env` to the exact value from the new bundle's `.env`");
      expect(runbook).toContain("Keep the other overrides and `certificates/`");
      const update = runbook.slice(runbook.indexOf("### Update"), runbook.indexOf("### Rollback"));
      expect(update).toContain("set `PROCTOR_TAG` in the retained `.env` to the exact value from the new bundle's `.env`");
      expect(update).toContain("Keep the other overrides and `certificates/`");
    }
    for (const row of bundleCases.registryUpdates) {
      const selected = { ...row.retained, PROCTOR_TAG: row.selectedTag };
      expect({ ...selected, PROCTOR_TAG: row.retained.PROCTOR_TAG }).toEqual(row.retained);
      expect(compose.services.proctor!.image.replace("${PROCTOR_TAG:-latest}", selected.PROCTOR_TAG)).toBe(row.image);
      const runbook = readFileSync(join(site, "README.md"), "utf8");
      const registry = runbook.slice(runbook.indexOf("### Update"), runbook.indexOf("The backup comes first:"));
      expect(registry).toContain("set `PROCTOR_TAG` in the retained `.env` to the intended release tag");
      expect(registry).toContain("Keep every other override");
      expect(registry).toContain('image=$(docker compose config --images proctor)');
      expect(registry).toContain('container=$(docker create "$image")');
      expect(registry.indexOf("proctor backup")).toBeLessThan(registry.indexOf("set `PROCTOR_TAG`"));
      expect(registry).not.toContain(`docker create ${deployment.proctor.image}:latest`);
    }
    for (const row of bundleCases.invalid) {
      const changed = "replace" in row ? read("compose.yaml").replace(row.replace!, row.with!) : read("compose.yaml") + row.append;
      expect(() => stackBundleImages("latest", changed), row.id).toThrow(row.error);
    }
    const steps = ["1. Copy this whole directory to the host", `2. docker load --input ${COURSE_BUNDLE_IMAGE}`, "3. The certificate, one of:", "4. docker compose up --detach --wait", `5. curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`];
    expect(readme.match(/^\d+\. /gmu)).toEqual(steps.map((step) => step.slice(0, 3)));
    const told = steps.map((step) => readme.indexOf(`\n${step}`));
    expect(told.every((at, index) => at > (told[index - 1] ?? 0)), JSON.stringify(told)).toBe(true);
    for (const fact of [COURSE_SITE_ORIGIN, "linux/amd64", "Docker Engine 25 or newer", "compose plugin 2.24 or newer", `${deployment.proctor.image}:sha-0123456789ab`, "pins the image tag sha-0123456789ab", "80/tcp, 443/tcp and 443/udp", `${deployment.proctor.host} are open from the internet`, `${COURSE_STACK_CERTIFICATES}/<any name>.pem`, "full chain first", "key after it", "owned by root with mode 0600", "docker compose up --detach --force-recreate caddy", "docker compose exec -T proctor proctor backup - > proctor.sqlite", "docker compose down\n", "NEVER docker compose down --volumes", "docker compose logs --since 1h proctor caddy", ...COURSE_STACK_FILES, ".env", `${COURSE_STACK_CERTIFICATES}/`, COURSE_BUNDLE_IMAGE]) expect(readme, fact).toContain(fact);
    const named = new Set(readme.match(new RegExp(`(?:[a-z0-9_-]+\\.)*(?:${[deployment.site.host, deployment.proctor.host].map(hostZone).join("|").replace(/\./gu, "\\.")})`, "gu")));
    expect([...named].sort()).toEqual([deployment.site.host, deployment.proctor.host].sort());
    expect(readme.split("\n").every((line) => line.length <= 120)).toBe(true);
    expect(compose.services.caddy!.volumes).toContain(`./${COURSE_STACK_CERTIFICATES}:/${COURSE_STACK_CERTIFICATES}:ro`);
    expect(compose.services.proctor!.pull_policy).toBe("missing");
    expect(compose.services.caddy!.pull_policy).toBe("missing");
    const origin = COURSE_PROCTOR_ORIGIN;
    expect(siteArtifactProblems(tree({ ...released(origin), [`${COURSE_BUNDLE_DIRECTORY}/probe.js`]: "fetch('http://localhost')", [`${COURSE_PAGES_DIRECTORY}/index.html`]: "<!doctype html>" }), origin)).toEqual([]);
    expect(siteArtifactProblems(tree({ ...released(origin), "elsewhere/probe.js": "fetch('http://localhost')" }), origin)).toEqual(["the site names localhost outside the baked proctor origin"]);
  });
  it("reaches the compiler and the site through the graph before the readiness gate starts, and ends with the end-to-end gate", () => {
    const project = JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { options?: { command?: string }; dependsOn?: string[] }> };
    expect(project.targets.typecheck?.options?.command).toBe("bun ./📜️script.ts typecheck");
    expect(project.targets["deploy-check"]?.dependsOn).toContain(readiness.sourceProducer);
    expect(project.targets.typecheck?.dependsOn).toEqual(["@semio-tech/framework-rs:generate", "@semio-tech/ui-contract-rs:generate", "@semio-tech/ui-rs:generate", "@semio-tech/framework-actor-rs:typegen", "@semio-tech/ui-styling-tokens:generate", "@semio-tech/assets:build"]);
    const steps = deployCheckSteps(project.targets);
    expect(steps).toEqual(["no drift from 🚀️deploy/🔣️.json", "course valid in the Rust core", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"]);
    expect(deployCheckSteps({})).toEqual(steps.filter((step) => step !== "end-to-end gate"));
    const readme = readFileSync(join(site, "README.md"), "utf8");
    const gate = readme.slice(readme.indexOf("### Readiness gate"));
    const told = gate.slice(0, gate.indexOf("\n#", 1)).match(/^\d+\. /gmu) ?? [];
    expect(told).toEqual(steps.map((_, index) => `${index + 1}. `));
    const include = (JSON.parse(readFileSync(join(site, "📦️packages/🟦️typescript/tsconfig.json"), "utf8")) as { include: string[] }).include;
    for (const covered of ["./📜️script.ts", "../../🟦️.ts", "../../🏗️builder/**/*.ts", "../../🧱️stack/**/*.ts", "../../🚀️deploy/**/*.ts", "../../🎭️e2e/**/*.ts", "../../🧪️tests/**/*.ts"]) expect(include).toContain(covered);
  });
  it("hardens every response and caches hashed assets, without two rules setting one header for a path", () => {
    const rules = COURSE_SITE_HEADERS.trimEnd().split("\n").filter((line) => !line.startsWith(" "));
    expect(rules).toEqual(["/*", "/assets/*", "/", "/index.html", "/404.html"]);
    expect(COURSE_SITE_HEADERS).toContain("/assets/*\n  Cache-Control: public, max-age=31536000, immutable\n");
    expect(COURSE_SITE_HEADERS.match(/Cache-Control: no-cache/gu)).toHaveLength(3);
    const everywhere = COURSE_SITE_HEADERS.slice(0, COURSE_SITE_HEADERS.indexOf("/assets/*"));
    expect(everywhere).not.toContain("Cache-Control");
    for (const header of ["X-Content-Type-Options: nosniff", "X-Frame-Options: DENY", "Content-Security-Policy: frame-ancestors 'none'", "Referrer-Policy: no-referrer"]) expect(everywhere).toContain(`  ${header}\n`);
  });
  it("exports the inspected immutable image identities with their offline tags even across a mutable-tag ABA", async () => {
    const fixture=bundleCases.imageExport,ajv=new Ajv({strict:true}),validate=ajv.compile(bundlePolicySchema.definitions.imageExportFixture),hash=(bytes:Uint8Array|string)=>Buffer.from(sha256(typeof bytes==="string"?Buffer.from(bytes):bytes)).toString("hex");
    expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
    expect(JSON.parse(fixture.proctor.config).config.Labels["org.opencontainers.image.revision"]).toMatch(/-dirty$/u);
    type Entry={name:string;body:Buffer;type?:"directory";pax?:Record<string,string>};
    const encode=async(entries:Entry[]):Promise<Buffer>=>{const writer=tarStream.pack(),chunks:Buffer[]=[],done=new Promise<Buffer>((accept,reject)=>{writer.on("data",(chunk:Buffer)=>chunks.push(Buffer.from(chunk)));writer.once("error",reject);writer.once("end",()=>accept(Buffer.concat(chunks)));});for(const entry of entries)await new Promise<void>((accept,reject)=>writer.entry({name:entry.name,size:entry.body.length,type:entry.type??"file",mode:0o644,...(entry.pax?{pax:entry.pax}:{})},entry.body,error=>error?reject(error):accept()));writer.finalize();return done;};
    const decode=async(body:Buffer):Promise<Map<string,Buffer>>=>new Promise((accept,reject)=>{const reader=tarStream.extract(),entries=new Map<string,Buffer>();reader.on("entry",(header:BundleTarHeader,stream:Readable,next:()=>void)=>{const chunks:Buffer[]=[];stream.on("data",(chunk:Buffer)=>chunks.push(Buffer.from(chunk)));stream.once("error",reject);stream.once("end",()=>{if(entries.has(header.name))return reject(Error("duplicate entry"));if(header.type==="file")entries.set(header.name,Buffer.concat(chunks));next();});stream.resume();});reader.once("error",reject);reader.once("finish",()=>accept(entries));reader.end(body);});
    for(const row of fixture.cases){
      let proctorBody=Buffer.from(fixture.proctor.config),proxyBody=Buffer.from(fixture.proxy.config);if(row.id==="wrong-platform")proctorBody=Buffer.from(fixture.proctor.config.replace('"amd64"','"arm64"'));if(row.id==="shared-config-tags")proxyBody=proctorBody;
      const selection={proctor:{reference:fixture.proctor.reference,id:"sha256:"+hash(proctorBody)},proxy:{source:fixture.proxy.source,reference:fixture.proxy.reference,id:"sha256:"+hash(proxyBody)}},images=[selection.proctor,selection.proxy],configPath=(body:Buffer)=>"blobs/sha256/"+hash(body),layer=Buffer.alloc(1024),layerPath="blobs/sha256/"+hash(layer),manifest=images.map((image,index)=>({Config:configPath(index?proxyBody:proctorBody),RepoTags:[image.reference],Layers:[layerPath]})),stored=new Map<string,Buffer>([[configPath(proctorBody),proctorBody],[configPath(proxyBody),proxyBody],[layerPath,layer]]);
      if(row.id==="aba-during-save"){const changed=Buffer.from(fixture.proctor.config+" ");manifest[0]!.Config=configPath(changed);stored.set(configPath(changed),changed);}
      if(row.id==="wrong-config-bytes")stored.set(configPath(proctorBody),Buffer.from("wrong bytes"));
      if(row.id==="missing-config")stored.delete(configPath(proctorBody));
      if(row.id==="missing-proctor-tag")manifest[0]!.RepoTags=[];
      if(row.id==="missing-caddy-alias")manifest[1]!.RepoTags=[];
      if(row.id==="duplicate-tag")manifest.push({...manifest[0]!});
      const entries:Entry[]=[...stored].map(([name,body])=>({name,body}));
      if(row.id==="duplicate-config-entry")entries.push({name:configPath(proctorBody),body:proctorBody});
      if(row.id==="root-dot"||row.id==="root-dot-slash")entries.unshift({name:row.id==="root-dot"?".":"./",body:Buffer.alloc(0),type:"directory"});
      if(row.id.startsWith("hybrid-")){
        const descriptors=images.map((image,index)=>{const body=index?proxyBody:proctorBody,config={mediaType:"application/vnd.oci.image.config.v1+json",digest:image.id,size:body.length},leaf=Buffer.from(JSON.stringify({schemaVersion:2,mediaType:"application/vnd.oci.image.manifest.v1+json",config:row.id==="hybrid-wrong-config"&&index===0?{...config,digest:selection.proxy.id}:config,layers:[{mediaType:"application/vnd.oci.image.layer.v1.tar",digest:"sha256:"+hash(layer),size:layer.length}]}));const name=configPath(leaf);entries.push({name,body:leaf});const full=image.reference.startsWith("caddy:")?"docker.io/library/"+image.reference:image.reference;return{mediaType:"application/vnd.oci.image.manifest.v1+json",digest:"sha256:"+hash(leaf),size:leaf.length,annotations:{"io.containerd.image.name":row.id==="hybrid-wrong-tag"&&index===0?"other:wrong":full,"org.opencontainers.image.ref.name":image.reference.slice(image.reference.lastIndexOf(":")+1)}};});
        entries.push({name:"index.json",body:Buffer.from(JSON.stringify({schemaVersion:2,mediaType:"application/vnd.oci.image.index.v1+json",manifests:descriptors}))},{name:"oci-layout",body:Buffer.from('{"imageLayoutVersion":"1.0.0"}')});
      }
      entries.push({name:"manifest.json",body:Buffer.from(JSON.stringify(manifest)),...(row.id==="timestamp-pax"?{pax:{mtime:"1.5"}}:row.id==="path-pax"?{pax:{path:"foreign.json"}}:{})});
      let archive=await encode(entries);if(row.id==="bad-header-checksum"){archive=Buffer.from(archive);archive[0]^=1;}if(row.id==="truncated-archive")archive=archive.subarray(0,archive.length-600);
      const output=join(tree({}),COURSE_BUNDLE_IMAGE),controller=new AbortController(),inspected:{reference:string;id:string}[]=[],saved:string[][]=[];let didSave=false,accepted=true;if(row.id==="cancel-before-save")controller.abort(Error("neutral before"));
      try{await saveStackBundleImages(selection,output,{inspect:async reference=>{const expected=reference===selection.proctor.reference?selection.proctor.id:selection.proxy.id,id=row.id==="retag-before-save"&&!didSave||row.id==="retag-after-save"&&didSave?"sha256:"+"f".repeat(64):expected;inspected.push({reference,id});return id;},save:async references=>{saved.push([...references]);writeFileSync(output,archive);didSave=true;},verify:async(path,bindings,signal)=>verifyStackBundleArchive(path,bindings,signal,async()=>{if(row.id==="cancel-during-verification")controller.abort(Error("neutral during"));})},controller.signal);}catch{accepted=false;}
      let oracle=false;try{const files=await decode(archive),records=JSON.parse(files.get("manifest.json")!.toString()),observed=records.flatMap((record:any)=>record.RepoTags.map((reference:string)=>({reference,id:"sha256:"+hash(files.get(record.Config)!),platform:JSON.parse(files.get(record.Config)!.toString())}))),binding=ajv.compile({type:"object",required:["reference","id"],properties:{reference:{enum:images.map(image=>image.reference)},id:{enum:[...new Set(images.map(image=>image.id))]}}});oracle=records.length<=2&&images.every(image=>observed.filter((entry:any)=>entry.reference===image.reference&&entry.id===image.id&&entry.platform.os==="linux"&&entry.platform.architecture==="amd64").length===1)&&observed.length===2&&observed.every((entry:any)=>binding(entry));if(files.has("index.json")){const index=JSON.parse(files.get("index.json")!.toString());oracle&&=index.manifests.length===2&&index.manifests.every((descriptor:any)=>{const body=files.get("blobs/sha256/"+descriptor.digest.slice(7))!,leaf=JSON.parse(body.toString()),name=descriptor.annotations["io.containerd.image.name"].replace(/^docker[.]io\/library\//u,""),image=images.find(image=>image.reference===name);return body.length===descriptor.size&&hash(body)===descriptor.digest.slice(7)&&image?.id===leaf.config.digest&&descriptor.annotations["org.opencontainers.image.ref.name"]===name.slice(name.lastIndexOf(":")+1);});}oracle&&=inspected.every(entry=>entry.id===(entry.reference===selection.proctor.reference?selection.proctor.id:selection.proxy.id))&&!controller.signal.aborted&&didSave;}catch{oracle=false;}
      expect(accepted,row.id).toBe(row.accepted);expect(oracle,row.id+" independent tar-stream/noble/Ajv oracle").toBe(row.accepted);if(didSave)expect(saved).toEqual([[selection.proctor.reference,selection.proxy.reference]]);if(row.id==="cancel-before-save")expect(inspected).toEqual([]);
    }
  });

}
