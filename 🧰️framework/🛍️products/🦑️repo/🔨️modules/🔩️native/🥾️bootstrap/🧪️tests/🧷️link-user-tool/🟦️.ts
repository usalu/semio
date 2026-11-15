/** 🧷️ The bootstrap does not swap the user command back and forth: `link_user_tool` leaves a link that already points to the source alone (same inode), leaves the native command (a real file named repo, installed by `repo-mcp:install`) alone, still creates a missing link and replaces a stale one or a real file of any other name. Run for real in bash with a throwaway HOME. */
import {expect,test} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";

const script=readFileSync(join(import.meta.dirname,"..","..","🐚️.sh"),"utf8"),body=/link_user_tool\(\) \{[\s\S]*?\n\}\n/u.exec(script)?.[0];

const run=(local:string,source:string,name:string)=>spawnSync("bash",["-c",`LOCAL_BIN='${local}'; ${body}\nlink_user_tool '${source}' '${name}'`],{encoding:"utf8"});

test("link_user_tool leaves what is right alone and the native command in place, and still links what is missing or stale",()=>{
  expect(body).toBeDefined();
  const box=mkdtempSync(join(tmpdir(),"link-user-tool-")),local=join(box,"bin"),launcher=join(box,"launcher","repo"),other=join(box,"other","tool");
  mkdirSync(local);mkdirSync(join(box,"launcher"));mkdirSync(join(box,"other"));writeFileSync(launcher,"#!/bin/sh\n");writeFileSync(other,"#!/bin/sh\n");chmodSync(launcher,0o755);chmodSync(other,0o755);
  try{
    expect(run(local,launcher,"repo").status).toBe(0);
    expect(readlinkSync(join(local,"repo"))).toBe(launcher);
    const before=lstatSync(join(local,"repo"));
    expect(run(local,launcher,"repo").status).toBe(0);
    const after=lstatSync(join(local,"repo"));expect({ino:after.ino,mtimeMs:after.mtimeMs}).toEqual({ino:before.ino,mtimeMs:before.mtimeMs});
    rmSync(join(local,"repo"));writeFileSync(join(local,"repo"),"native command bytes");
    const native=lstatSync(join(local,"repo"));
    expect(run(local,launcher,"repo").status).toBe(0);
    const kept=lstatSync(join(local,"repo"));expect(kept.isSymbolicLink()).toBe(false);expect({ino:kept.ino,mtimeMs:kept.mtimeMs}).toEqual({ino:native.ino,mtimeMs:native.mtimeMs});expect(readFileSync(join(local,"repo"),"utf8")).toBe("native command bytes");
    writeFileSync(join(local,"tool"),"old real file");
    expect(run(local,other,"tool").status).toBe(0);expect(readlinkSync(join(local,"tool"))).toBe(other);
    symlinkSync("/nowhere",join(local,"stale"));
    expect(run(local,other,"stale").status).toBe(0);expect(readlinkSync(join(local,"stale"))).toBe(other);
    expect(run(local,join(box,"missing"),"gone").status).toBe(0);expect(() => lstatSync(join(local,"gone"))).toThrow();
  }finally{rmSync(box,{recursive:true,force:true});}
});
