#!/usr/bin/env bun
/** 🗂️ Runs Block catalog native and portable ownership laws. */
import {resolve} from "node:path";
import {runArtifactRustPackageMain} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runOwnedCommand} from "../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
const [command,...args]=process.argv.slice(2);
if(command==="conformance"){if(args.length)throw Error("conformance accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(import.meta.dir,"../../🧪️tests/🟦️.ts")],resolve(import.meta.dir,"../../../../../.."),"block-catalog-conformance",45000,{env:process.env});}else await runArtifactRustPackageMain(import.meta.dir,"semio-s-plugin-block-catalog");
