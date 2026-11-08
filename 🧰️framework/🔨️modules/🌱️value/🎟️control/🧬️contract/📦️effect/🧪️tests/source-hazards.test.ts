import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
const root=(()=>{let at=import.meta.dir;while(!existsSync(join(at,"AGENTS.md"))&&dirname(at)!==at)at=dirname(at);return at})();
const roots=["🧰️framework","✏️s"].map(name=>join(root,name)).filter(existsSync);
const scan=(pattern:string)=>spawnSync("/usr/bin/grep",["-rlE","--include=*.rs","--exclude-dir=node_modules","--exclude-dir=target","--exclude-dir=.cache","--exclude-dir=🧪️tests","--exclude-dir=🧪️testing",pattern,...roots],{maxBuffer:1<<26}).stdout.toString().split("\n").filter(Boolean).map(path=>path.slice(root.length)).sort();
const testRegion=["/🧰️framework/🔨️modules/🌱️value/🫴️receiving/🦀️.rs"];
const knownRecordDebt=["/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧱️primitives/🧩️records/🦀️.rs"];
/** 🕳️ A retirement that discards the Result of a page release reports no progress and no fault, so its caller steps it forever. */
test("no retirement discards a page release result",()=>{expect(scan("let _ ?= [^;]*release_empty_page_leased").filter(path=>!knownRecordDebt.includes(path))).toEqual([])},300000);
/** ⏳️ A synchronous cold driver that accepts a Pending poll as success spins; Pending must become a typed refusal. */
test("no cold driver accepts a pending poll as success",()=>{expect(scan("RetainedWorkPoll::Pending ?\\{ ?\\.\\. ?\\} ?=> ?\\{\\}").filter(path=>!testRegion.includes(path))).toEqual([])},300000);
/** 🧾️ A raw-pointer witness or wallet minted inside a turn is uncharged work: the audit pattern for `unwrap_or(u128::MAX)` admit-all stocks stays out. */
test("no authoring stock falls back to an admit-all u128",()=>{expect(scan("AuthoringWallet::new\\([^;]*unwrap_or\\(u128::MAX\\)")).toEqual([])},300000);
const hits=(path:string,pattern:string)=>spawnSync("/usr/bin/grep",["-nE",pattern,join(root,path)]).stdout.toString().split("\n").filter(Boolean);
/** 🧱️ A compositing job lost mid-flight is a typed error in the progress JSON, never a panic of the surface host. */
test("surface paint compositing never unwraps its job",()=>{expect(hits("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧩️compositing/🦀️.rs","\\.unwrap\\(\\)")).toEqual([])},60000);
/** 🕸️ A mesh close turn whose work quote is not admitted can never be admitted by the same stock: it refuses, it does not report zero progress for a caller loop to repeat. */
test("mesh owner close refuses an unadmitted quote",()=>{expect(hits("🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs","Ok\\(None\\)=>Ok\\(RetainedCloneProgress::default\\(\\)\\)")).toEqual([])},60000);
