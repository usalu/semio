/** 🧷️ The launcher installer touches nothing that would not change: the second run leaves inode, mtime and hash of the installed launcher (and of the link to it) as they are, a changed source installs a new version beside the old one (which stays untouched for a running service) and says that macOS asks once more, a rebuild to the same bytes replaces nothing. Run with the real pinned Go. */
import {expect,test} from "bun:test";
import {cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,rmSync,statSync,appendFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {homedir,tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";

const launcher=join(import.meta.dirname,"..",".."),repoRoot=join(launcher,"..","..","..","..","..");
const go=[process.env.SEMIO_TEST_GO,join(homedir(),".local","bin","go")].find(path=>path!==undefined&&existsSync(path));
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const facts=(path:string)=>{const stat=statSync(path);return{ino:stat.ino,mtimeMs:stat.mtimeMs,sha:sha(path)};};

test.skipIf(go===undefined)("the installer builds the launcher reproducibly, leaves an unchanged one alone, installs a changed one beside the old and says so",()=>{
  const box=mkdtempSync(join(tmpdir(),"launcher-no-touch-")),checkout=join(box,"checkout"),relative=launcher.slice(repoRoot.length+1),home=join(box,"home"),bin=join(box,"bin");
  mkdirSync(home,{recursive:true});mkdirSync(join(checkout,relative),{recursive:true});
  cpSync(join(launcher,"📥️install"),join(checkout,relative,"📥️install"),{recursive:true});cpSync(join(launcher,"📦️packages"),join(checkout,relative,"📦️packages"),{recursive:true});
  const pins="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json";mkdirSync(dirname(join(checkout,pins)),{recursive:true});cpSync(join(repoRoot,pins),join(checkout,pins));
  const run=()=>spawnSync("bash",[join(checkout,relative,"📥️install","🐚️.sh"),bin],{env:{PATH:`${dirname(go!)}:/usr/bin:/bin`,HOME:home,SEMIO_GO_ROOT:join(box,"gocache")},encoding:"utf8",timeout:300_000});
  try{
    const first=run();expect(first.status,first.stderr).toBe(0);
    expect(first.stderr).toContain("Installed the repo launcher");expect(first.stderr).not.toContain("asks once more");
    const link=join(bin,"repo");expect(lstatSync(link).isSymbolicLink()).toBe(true);
    const target=join(bin,readlinkSync(link)),before={target:facts(target),link:lstatSync(link).ino,stamp:readFileSync(join(bin,"repo.stamp"),"utf8")};
    expect(readlinkSync(link)).toMatch(/^versions\/[0-9a-f]{16}\/repo$/u);expect(statSync(target).mode&0o111).not.toBe(0);
    if(process.platform==="darwin"){const sign=spawnSync("codesign",["-dv",target],{encoding:"utf8"});expect(sign.stderr).toContain("Identifier=tech.semio.repo");}
    const second=run();expect(second.status,second.stderr).toBe(0);
    expect(second.stderr).toContain("is current");expect(second.stderr).not.toContain("Installed the repo launcher");
    expect({target:facts(target),link:lstatSync(link).ino,stamp:readFileSync(join(bin,"repo.stamp"),"utf8")}).toEqual(before);
    rmSync(join(bin,"repo.stamp"));
    const same=run();expect(same.status,same.stderr).toBe(0);
    expect(same.stderr).toContain("same bytes");expect({target:facts(target),link:lstatSync(link).ino}).toEqual({target:before.target,link:before.link});
    const source=join(checkout,relative,"📦️packages","🐹️go","🐹️.go");appendFileSync(source,"\nfunc init() { println(\"a changed source\") }\n");
    const changed=run();expect(changed.status,changed.stderr).toBe(0);
    expect(changed.stderr).toContain("Installed the repo launcher");expect(changed.stderr).toContain("macOS asks once more");expect(changed.stderr).toContain("fragt einmal erneut");
    expect(readlinkSync(link)).not.toBe(readlinkSync(link).replace(/[0-9a-f]{16}/u,"0".repeat(16)));
    expect(sha(link)).not.toBe(before.target.sha);
    expect(facts(target),"the old version stays untouched for a running service").toEqual(before.target);
    expect(readdirSync(join(bin,"versions")).length).toBe(2);
  }finally{rmSync(box,{recursive:true,force:true});}
},600_000);
