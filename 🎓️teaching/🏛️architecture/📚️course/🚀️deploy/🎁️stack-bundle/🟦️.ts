/** 🎁️ The `docker-stack-bundle` verb: what the proctor host needs without a registry — the stack files out of the image, an empty `certificates` directory, an `.env` that pins the tag, the image as `proctor-image.tar` and the host's steps.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { constants } from "node:fs";
import { open as openArchive, lstat } from "node:fs/promises";
import { createHash } from "node:crypto";
import { copyFileSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import { imageLabels } from "../🐳️docker-image/🟦️.ts";
import { COURSE_BUNDLE_DIRECTORY, COURSE_BUNDLE_IMAGE, COURSE_IMAGE_REPOSITORY, COURSE_PROCTOR_HOST, COURSE_PROCTOR_ORIGIN, COURSE_SITE_ORIGIN, COURSE_STACK_CERTIFICATES, COURSE_STACK_FILES, docker, dockerOk, dockerServerVersion, flag, interruption, say } from "../🟦️.ts";
import { expectCheckoutStack, extractCourseStack } from "../🩺️docker-image-check/🟦️.ts";

//#region 🎁️Bundle
/** 🧳️ Enumerates the complete authored stack image set, keeping the proxy's exact digest for offline transfer. */
export function stackBundleImages(tag: string, compose: string): readonly [string, string] {
  const images = [...compose.matchAll(/^ {4}image:\s*(\S+)$/gmu)].map(match => match[1]!);
  const caddy = /^ {2}caddy:\r?\n {4}image:\s*(caddy:[\w.-]+@sha256:[0-9a-f]{64})\r?$/mu.exec(compose)?.[1];
  if (images.length !== 2 || images[0] !== `${COURSE_IMAGE_REPOSITORY}:\${PROCTOR_TAG:-latest}` || images[1] !== caddy || !caddy) throw Error("The offline stack requires exactly its proctor and pinned Caddy images");
  return [`${COURSE_IMAGE_REPOSITORY}:${tag}`, caddy];
}

/** 🏷️ Gives the pinned proxy a portable archive name and prevents the staged stack from consulting a registry. */
export function stackBundlePlan(tag: string, compose: string): { readonly images: readonly [string, string, string]; readonly caddy: string; readonly compose: string } {
  const images = stackBundleImages(tag, compose);
  const caddy = `caddy:sha256-${images[1].split("@sha256:")[1]}`;
  const source = compose.replace(/\r\n/gu, "\n");
  const proctor = `  proctor:\n    image: ${COURSE_IMAGE_REPOSITORY}:\${PROCTOR_TAG:-latest}\n    pull_policy: missing\n`;
  if (!source.includes(proctor)) throw Error("The offline stack requires its authored proctor pull policy");
  const proxy = `  caddy:\n    image: ${images[1]}\n    pull_policy: missing\n`;
  if (!source.includes(proxy)) throw Error("The offline stack requires its authored Caddy pull policy");
  return { images: [...images, caddy], caddy, compose: source.replace(proctor, proctor.replace("pull_policy: missing", "pull_policy: never")).replace(proxy, `  caddy:\n    image: ${caddy}\n    pull_policy: never\n`) };
}


/** 🪧️ Pins the two offline tags and the proxy's authored digest source to immutable image configs. */
export interface StackBundleSelection {readonly proctor:{readonly reference:string;readonly id:string};readonly proxy:{readonly source:string;readonly reference:string;readonly id:string}}
/** 🚛️ Keeps image inspection, tagged export and archive verification behind the bundle's own tool port. */
export interface StackBundleExportPort {inspect(reference:string,signal:AbortSignal):Promise<string>;save(references:readonly string[],path:string,signal:AbortSignal):Promise<void>;verify(path:string,images:readonly {readonly reference:string;readonly id:string}[],signal:AbortSignal):Promise<void>}
/** 🧷️ Retains tag associations before and after export, including the bytes actually selected during an ABA race. */
export async function saveStackBundleImages(selection:StackBundleSelection,path:string,port:StackBundleExportPort,signal:AbortSignal):Promise<void>{
 const images=[selection.proctor,selection.proxy],references=[selection.proctor,{reference:selection.proxy.source,id:selection.proxy.id},selection.proxy];if(images.some(image=>!/^sha256:[a-f0-9]{64}$/u.test(image.id))||images[0]!.reference===images[1]!.reference)throw Error("Pinned offline image configs and distinct tags required");
 const recheck=async()=>{for(const image of references){signal.throwIfAborted();if(await port.inspect(image.reference,signal)!==image.id)throw Error("Offline image reference changed: "+image.reference);signal.throwIfAborted();}};await recheck();await port.save(images.map(image=>image.reference),path,signal);await recheck();await port.verify(path,images,signal);await recheck();
}

/** 🔍️ Verifies the saved Docker and optional OCI tag views against pinned Linux amd64 config bytes without extracting layers. */
export async function verifyStackBundleArchive(path:string,images:readonly {readonly reference:string;readonly id:string}[],signal:AbortSignal,progress:(headers:number,bytes:number)=>void|Promise<void>=()=>undefined):Promise<void>{
 signal.throwIfAborted();if(resolve(path).length>256||Buffer.byteLength(resolve(path))>256)throw Error("Bounded bundle archive path required");const maximumArchive=16*1024**3,maximumMetadata=16*1024**2,maximumJson=8*1024**2,ancestors:{path:string;dev:bigint;ino:bigint;mode:bigint}[]=[];for(let current=dirname(resolve(path));;current=dirname(current)){const row=await lstat(current,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw Error("Ordinary bundle archive ancestry required");ancestors.push({path:current,dev:row.dev,ino:row.ino,mode:row.mode});if(dirname(current)===current)break;}
 const before=await lstat(path,{bigint:true}),same=(a:typeof before,b:typeof before)=>["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"].every(key=>(a as any)[key]===(b as any)[key]);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<1024n||before.size>BigInt(maximumArchive)||before.size%512n)throw Error("Bounded ordinary Docker archive required");
 const file=await openArchive(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)),size=Number(before.size),entries=new Map<string,{offset:number;bytes:number;kind:number}>(),kinds=new Map<string,number>(),parents=new Set<string>(),metadata=new Map<string,Buffer>(),utf8=new TextDecoder("utf-8",{fatal:true});let metadataBytes=0,headers=0;
 try{
  if(!same(before,await file.stat({bigint:true})))throw Error("Original Docker archive descriptor required");
  const check=()=>signal.throwIfAborted(),read=async(offset:number,bytes:number)=>{check();if(!Number.isSafeInteger(offset)||!Number.isSafeInteger(bytes)||offset<0||bytes<0||bytes>maximumJson||offset+bytes>size)throw Error("Bounded Docker archive extent required");const body=Buffer.alloc(bytes);let count=0;while(count<bytes){check();const row=await file.read(body,count,bytes-count,offset+count);if(!row.bytesRead)throw Error("Truncated Docker archive");count+=row.bytesRead;}check();return body;},zero=(body:Uint8Array)=>body.every(byte=>byte===0),text=(body:Buffer)=>{const end=body.indexOf(0);if(end>=0&&!zero(body.subarray(end)))throw Error("Canonical Docker TAR text required");return utf8.decode(end<0?body:body.subarray(0,end));},number=(body:Buffer)=>{let value:bigint;if(body[0]!&128){if(body[0]!&64)throw Error("Negative Docker TAR field refused");value=BigInt(body[0]!&127);for(const byte of body.subarray(1))value=value*256n+BigInt(byte);}else{const digits=body.toString("ascii").replace(/^[ ]+|[\0 ]+$/gu,"");if(!/^[0-7]*$/u.test(digits))throw Error("Octal Docker TAR field required");value=digits?BigInt("0o"+digits):0n;}if(value>BigInt(Number.MAX_SAFE_INTEGER))throw Error("Finite Docker TAR field required");return Number(value);},canonical=(name:string,directory=false)=>{if(directory&&(name==="."||name==="./"))return"";name=name.replace(/^\.\//u,"").replace(directory?/\/$/u:/$^/u,"");if(!name||Buffer.byteLength(name)>256||name.includes("\\")||name.split("/").some(part=>!part||part==="."||part===".."||/[<>:"|?*\x00-\x1f]/u.test(part)))throw Error("Canonical Docker archive path required");return name;};
  let cursor=0,ended=false,pending:Record<string,string>|undefined;
  while(cursor<size){check();const header=await read(cursor,512);if(zero(header)){if(pending||cursor+1024>size)throw Error("Complete Docker TAR footer required");for(let offset=cursor;offset<size;offset+=65536)if(!zero(await read(offset,Math.min(65536,size-offset))))throw Error("Docker TAR trailing content refused");ended=true;break;}if(++headers>4096)throw Error("Docker TAR header ceiling exceeded");if(number(header.subarray(148,156))!==header.reduce((sum,byte,index)=>sum+(index>=148&&index<156?32:byte),0)||text(header.subarray(257,263)).trim()!=="ustar")throw Error("Docker TAR header checksum or format changed");const prefix=text(header.subarray(345,500)),name=(prefix?prefix+"/":"")+text(header.subarray(0,100)),kind=header[156]!,bytes=number(header.subarray(124,136)),offset=cursor+512,next=offset+Math.ceil(bytes/512)*512;if(!Number.isSafeInteger(next)||next>size)throw Error("Truncated Docker TAR body");if(!zero(await read(offset+bytes,next-offset-bytes)))throw Error("Docker TAR padding changed");cursor=next;
   if(kind===120||kind===103){if(pending||bytes>65536)throw Error("Bounded Docker PAX metadata required");const body=await read(offset,bytes),values:Record<string,string>={};let at=0;while(at<body.length){const space=body.indexOf(32,at),digits=body.subarray(at,space).toString("ascii"),length=Number(digits),end=at+length;if(space<at||space-at>10||!/^[1-9][0-9]*$/u.test(digits)||!Number.isSafeInteger(length)||end>body.length||end<=space+2||body[end-1]!==10)throw Error("Docker PAX record boundary refused");const record=utf8.decode(body.subarray(space+1,end-1)),equals=record.indexOf("="),key=record.slice(0,equals),value=record.slice(equals+1);if(equals<1||Object.hasOwn(values,key)||value.includes("\0")||!["path","size","mtime","atime","ctime","uid","gid","uname","gname"].includes(key)||kind===103&&["path","size"].includes(key))throw Error("Docker PAX metadata refused");if(["mtime","atime","ctime"].includes(key)&&!/^[-]?[0-9]+(?:[.][0-9]+)?$/u.test(value)||["size","uid","gid"].includes(key)&&!/^[0-9]+$/u.test(value))throw Error("Docker PAX numeric field refused");values[key]=value;at=end;}if(kind===120)pending=values;
   }else{if(![0,48,53].includes(kind)||kind===53&&bytes!==0||pending?.size!==undefined&&Number(pending.size)!==bytes)throw Error("Docker TAR regular entries and unambiguous PAX required");const path=canonical(pending?.path??name,kind===53);pending=undefined;if(path){const key=path.normalize("NFC").toLowerCase();if(kinds.has(key)||kind!==53&&parents.has(key))throw Error("Duplicate or conflicting Docker archive entry");const parts=key.split("/");for(let count=1;count<parts.length;count++){const parent=parts.slice(0,count).join("/");if(kinds.has(parent)&&kinds.get(parent)!==53)throw Error("Docker archive file ancestor refused");parents.add(parent);}kinds.set(key,kind);entries.set(path,{offset,bytes,kind});}}
   await progress(headers,cursor);check();
  }if(!ended)throw Error("Complete Docker TAR required");
  const entry=(name:string)=>{const normalized=canonical(name),row=entries.get(normalized);if(!row||![0,48].includes(row.kind))throw Error("Ordinary saved image metadata or layer required: "+name);return row;},body=async(name:string)=>{name=canonical(name);if(metadata.has(name))return metadata.get(name)!;const row=entry(name);if(row.bytes>maximumJson||metadataBytes+row.bytes>maximumMetadata)throw Error("Saved image metadata ceiling exceeded");const bytes=await read(row.offset,row.bytes);metadataBytes+=bytes.length;metadata.set(name,bytes);return bytes;},json=async(name:string)=>JSON.parse(utf8.decode(await body(name))),digest=(bytes:Uint8Array)=>"sha256:"+createHash("sha256").update(bytes).digest("hex"),expected=new Map(images.map(image=>[image.reference,image.id]));if(images.length!==2||expected.size!==2||images.some(image=>!/^sha256:[a-f0-9]{64}$/u.test(image.id)))throw Error("Two pinned offline image tags required");
  const manifest=await json("manifest.json"),tags=new Set<string>(),layersById=new Map<string,string>();if(!Array.isArray(manifest)||!manifest.length||manifest.length>2)throw Error("Exactly the offline saved images required");for(const record of manifest){check();if(!record||typeof record.Config!=="string"||!Array.isArray(record.RepoTags)||!record.RepoTags.length||!Array.isArray(record.Layers)||record.Layers.length>256)throw Error("Saved image tag/config manifest required");const configBytes=await body(record.Config),id=digest(configBytes),config=JSON.parse(utf8.decode(configBytes)),layers=record.Layers.map((name:unknown)=>{if(typeof name!=="string")throw Error("Saved layer path required");entry(name);return canonical(name);}),layerKey=JSON.stringify(layers);if(config.os!=="linux"||config.architecture!=="amd64"||layersById.has(id)&&layersById.get(id)!==layerKey)throw Error("Consistent Linux amd64 saved config required");layersById.set(id,layerKey);for(const reference of record.RepoTags){if(typeof reference!=="string"||tags.has(reference)||expected.get(reference)!==id)throw Error("Saved offline tag does not name its pinned config: "+reference);tags.add(reference);}}
  if(tags.size!==expected.size)throw Error("Required offline image tag missing");
  if(entries.has("index.json")){const index=await json("index.json"),layout=await json("oci-layout");if(layout.imageLayoutVersion!=="1.0.0"||index.schemaVersion!==2||index.mediaType!==undefined&&index.mediaType!=="application/vnd.oci.image.index.v1+json"||!Array.isArray(index.manifests)||index.manifests.length>4)throw Error("Bounded OCI image index required");const seenTags=new Set<string>();let descriptors=0;
   const resolveDescriptor=async(descriptor:any,id:string,depth:number,ancestors:Set<string>):Promise<number>=>{check();if(++descriptors>32||depth>4||!descriptor||!/^sha256:[a-f0-9]{64}$/u.test(descriptor.digest)||!Number.isSafeInteger(descriptor.size)||descriptor.size<1||descriptor.size>maximumJson||ancestors.has(descriptor.digest))throw Error("Bounded acyclic OCI descriptor required");if(descriptor.platform&&(descriptor.platform.os!=="linux"||descriptor.platform.architecture!=="amd64"))return 0;const path="blobs/sha256/"+descriptor.digest.slice(7),bytes=await body(path);if(bytes.length!==descriptor.size||digest(bytes)!==descriptor.digest)throw Error("Saved OCI descriptor bytes changed");const value=JSON.parse(utf8.decode(bytes)),visited=new Set([...ancestors,descriptor.digest]);if(value.schemaVersion!==2||value.mediaType&&value.mediaType!==descriptor.mediaType)throw Error("Saved OCI descriptor media changed");if(["application/vnd.oci.image.index.v1+json","application/vnd.docker.distribution.manifest.list.v2+json"].includes(descriptor.mediaType)){if(!Array.isArray(value.manifests)||!value.manifests.length||value.manifests.length>16)throw Error("Bounded OCI platform set required");let count=0;for(const child of value.manifests)count+=await resolveDescriptor(child,id,depth+1,visited);return count;}if(!["application/vnd.oci.image.manifest.v1+json","application/vnd.docker.distribution.manifest.v2+json"].includes(descriptor.mediaType)||value.config?.digest!==id||!["application/vnd.oci.image.config.v1+json","application/vnd.docker.container.image.v1+json"].includes(value.config?.mediaType)||!Number.isSafeInteger(value.config?.size)||!Array.isArray(value.layers))throw Error("OCI tag does not name its pinned config");const configBytes=await body("blobs/sha256/"+id.slice(7));if(configBytes.length!==value.config.size||digest(configBytes)!==id)throw Error("OCI config bytes changed");const layers=value.layers.map((layer:any)=>{if(!/^sha256:[a-f0-9]{64}$/u.test(layer?.digest)||!Number.isSafeInteger(layer.size)||layer.size<0)throw Error("OCI layer descriptor required");const path="blobs/sha256/"+layer.digest.slice(7);if(entry(path).bytes!==layer.size)throw Error("OCI layer extent changed");return path;});if(JSON.stringify(layers)!==layersById.get(id))throw Error("Docker and OCI layer mappings disagree");return 1;};
   for(const descriptor of index.manifests){const annotations=descriptor?.annotations,name=annotations?.["io.containerd.image.name"],reference=images.find(image=>name===image.reference||image.reference.startsWith("caddy:")&&name==="docker.io/library/"+image.reference)?.reference,short=reference?.slice(reference.lastIndexOf(":")+1),refName=annotations?.["org.opencontainers.image.ref.name"];if(!reference||seenTags.has(reference)||refName!==undefined&&refName!==short&&refName!==reference&&refName!==name)throw Error("OCI offline tag mapping changed");seenTags.add(reference);if(await resolveDescriptor(descriptor,expected.get(reference)!,0,new Set())!==1)throw Error("Exactly one Linux amd64 OCI image required per tag");}if(seenTags.size!==expected.size)throw Error("Required OCI offline tag missing");
  }
  check();if(!same(before,await file.stat({bigint:true}))||!same(before,await lstat(path,{bigint:true})))throw Error("Saved image archive changed during verification");for(const parent of ancestors){const actual=await lstat(parent.path,{bigint:true});if(!actual.isDirectory()||actual.isSymbolicLink()||actual.dev!==parent.dev||actual.ino!==parent.ino||actual.mode!==parent.mode)throw Error("Saved image archive ancestry changed");}check();
 }finally{await file.close();}
}

/** 🎚️ The `.env` of the bundle: the tag of the image the host loads, pinned so that compose never looks for another
 * one, and every other default of `compose.yaml` an operator may override, as a comment holding that default. */
export function stackBundleEnvironment(tag: string): string {
  return ["# Overrides of compose.yaml; both services read this file as their environment.", `PROCTOR_TAG=${tag}`, `# PROCTOR_HOST=${COURSE_PROCTOR_HOST}`, `# PROCTOR_ALLOWED_ORIGINS=${COURSE_SITE_ORIGIN}`, "# COURSE_HTTP_PORT=80", "# COURSE_HTTPS_PORT=443"].map((line) => `${line}\n`).join("");
}

/** 📖️ The steps of the proctor host, as they travel with the bundle for the image tag `tag`. */
export function stackBundleReadme(tag: string, caddyImage: string, localCaddy: string): string {
  return [
    `Architecture course proctor: the Docker stack of ${COURSE_PROCTOR_ORIGIN}`,
    "",
    `The backend of the site ${COURSE_SITE_ORIGIN}:`,
    "the proctor (an API over one SQLite file on a Docker volume) behind Caddy, which terminates TLS. This directory is",
    "everything the host needs; nothing is pulled from a registry.",
    "",
    `  ${COURSE_STACK_FILES.join(", ")}   the stack`,
    `  .env                      overrides of compose.yaml; pins the image tag ${tag}`,
    `  ${COURSE_STACK_CERTIFICATES}/             your own certificate, when Caddy cannot obtain one (step 3)`,
    `  ${COURSE_BUNDLE_IMAGE}         the image ${COURSE_IMAGE_REPOSITORY}:${tag}`,
    `                           and ${caddyImage}`,
    "                           loaded locally as",
    `                           ${localCaddy}`,
    "  compose.yaml binds Caddy to that local digest tag; both services use pull_policy: never.",
    "",
    "The host: linux/amd64 with Docker Engine 25 or newer and its compose plugin 2.24 or newer.",
    "",
    "1. Copy this whole directory to the host (scp -r, rsync, a USB stick) and work inside it.",
    `2. docker load --input ${COURSE_BUNDLE_IMAGE}`,
    "3. The certificate, one of:",
    "   a. Let's Encrypt, nothing to do: Caddy obtains and renews it once ports 80/tcp, 443/tcp and 443/udp of",
    `      ${COURSE_PROCTOR_HOST} are open from the internet.`,
    `   b. Your own: one file ${COURSE_STACK_CERTIFICATES}/<any name>.pem with the certificate and its full chain first and the private`,
    "      key after it, owned by root with mode 0600 (Caddy does not start on a file it cannot read). Caddy serves",
    "      it and orders none. After replacing it: docker compose up --detach --force-recreate caddy",
    "   Either way a learner's browser reaches the proctor only where 443/tcp of the host is open to it.",
    "4. docker compose up --detach --wait",
    `5. curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`,
    '   answers JSON that starts {"id":"teaching-proctor"',
    "",
    "Backup while serving (keep copies off the host):",
    "  docker compose exec -T proctor proctor backup - > proctor.sqlite",
    "",
    "Update an existing stack from this bundle:",
    "  Take the backup above before replacing files.",
    `  Copy ${COURSE_BUNDLE_IMAGE}, compose.yaml and Caddyfile into the existing stack directory.`,
    `  Keep its .env and ${COURSE_STACK_CERTIFICATES}/.`,
    "  In the existing .env, set this image tag and keep every other override:",
    `  PROCTOR_TAG=${tag}`,
    `  docker load --input ${COURSE_BUNDLE_IMAGE}`,
    "  docker compose up --detach --wait",
    `  curl --fail ${COURSE_PROCTOR_ORIGIN}/instance`,
    "  Loading the archive alone does not change the tag selected by the existing .env.",
    "",
    "Stop and keep the data:",
    "  docker compose down",
    "NEVER docker compose down --volumes: it deletes the database and the certificates Caddy obtained.",
    "Logs:",
    "  docker compose logs --since 1h proctor caddy",
    "",
  ].join("\n");
}

/** 🥞️ `docker-stack-bundle [--tag <tag>] [--out <directory>]` — stages everything the proctor host needs without a
 * registry in `--out` (default the package's `dist/proctor`): `compose.yaml` and the `Caddyfile` copied out of the image
 * `--tag` (default `latest`, built beforehand with `docker-image-build`; an image whose stack files are not those of this
 * checkout is refused), the `certificates` directory (what it already holds stays), the `.env`
 * ({@link stackBundleEnvironment}), `README.txt` ({@link stackBundleReadme}) and both stack images as
 * `proctor-image.tar` (`docker save`). Nothing is uploaded: the directory is copied to the host by hand. Answers the
 * directory. */
export async function stageCourseStackBundle(repoRoot: string, bundleRoot: string, segments: readonly string[], invocation: ScriptInvocation): Promise<string> {
  const tag = flag(segments, "--tag") ?? "latest";
  const image = `${COURSE_IMAGE_REPOSITORY}:${tag}`;
  const directory = resolve(flag(segments, "--out") ?? join(bundleRoot, "dist", COURSE_BUNDLE_DIRECTORY));
  const { signal: parentSignal, release } = interruption(invocation), signal = AbortSignal.any([parentSignal, AbortSignal.timeout(1_800_000)]);
  const pulse = setInterval(() => say(`bundling ${image}: operation remains active`), 10_000);
  let scratch = "";
  try {
    say(`docker ${await dockerServerVersion(signal)}; bundling ${image}`);
    if ((await docker(["image", "inspect", image], { signal })).code !== 0) throw new Error(`image ${image} does not exist; build it first with docker-image-build --tag ${tag}`);
    const imageId = await dockerOk(["image", "inspect", "--format", "{{.Id}}", image], { signal });
    if (!/^sha256:[a-f0-9]{64}$/u.test(imageId)) throw Error("Immutable proctor image config required");
    const labels = await imageLabels(imageId, signal);
    scratch = await extractCourseStack(imageId, signal);
    expectCheckoutStack(repoRoot, scratch);
    const plan = stackBundlePlan(tag, readFileSync(join(scratch, "compose.yaml"), "utf8"));
    if ((await docker(["image", "inspect", plan.images[1]], { signal })).code !== 0) await dockerOk(["pull", "--platform", "linux/amd64", plan.images[1]], { signal });
    const caddyId = await dockerOk(["image", "inspect", "--format", "{{.Id}}", plan.images[1]], { signal });
    if (!/^sha256:[a-f0-9]{64}$/u.test(caddyId)) throw Error("Immutable Caddy image config required");
    await dockerOk(["tag", caddyId, plan.caddy], { signal });
    mkdirSync(join(directory, COURSE_STACK_CERTIFICATES), { recursive: true });
    for (const file of COURSE_STACK_FILES) copyFileSync(join(scratch, file), join(directory, file));
    writeFileSync(join(directory, "compose.yaml"), plan.compose);
    writeFileSync(join(directory, ".env"), stackBundleEnvironment(tag));
    writeFileSync(join(directory, "README.txt"), stackBundleReadme(tag, plan.images[1], plan.caddy));
    say(`saving ${plan.images.join(" and ")} (${labels["org.opencontainers.image.version"]} of ${labels["org.opencontainers.image.revision"]}) as ${COURSE_BUNDLE_IMAGE}`);
    await saveStackBundleImages({ proctor: { reference: image, id: imageId }, proxy: { source: plan.images[1], reference: plan.caddy, id: caddyId } }, join(directory, COURSE_BUNDLE_IMAGE), { inspect: (reference, signal) => dockerOk(["image", "inspect", "--format", "{{.Id}}", reference], { signal }), save: async (references, path, signal) => { await dockerOk(["save", "--output", path, ...references], { signal }); }, verify: (path, images, signal) => verifyStackBundleArchive(path, images, signal) }, signal);
    say(`staged ${[...COURSE_STACK_FILES, `${COURSE_STACK_CERTIFICATES}/`, ".env", "README.txt"].join(", ")} and ${COURSE_BUNDLE_IMAGE} (${Math.round(statSync(join(directory, COURSE_BUNDLE_IMAGE)).size / 1024 ** 2)} MiB) in ${directory}`);
    return directory;
  } finally {
    clearInterval(pulse);
    release();
    if (scratch) rmSync(scratch, { recursive: true, force: true });
  }
}
//#endregion 🎁️Bundle
