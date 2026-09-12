#!/usr/bin/env bun
import { ScriptRouter, runBundleScriptMain } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { GenerateScript, PreviewGeneratedScript, CheckGeneratedScript, CheckScript } from "./📽️projection/🟦️.ts";
import { NewScript } from "./🌳️surface-scaffold/🟦️.ts";
import { RustTaxonomyMountsCheckScript, PluginRootOwnershipCheckScript } from "./🗿️taxonomy-validation/🟦️.ts";
import { NativeCatalogSelectionCheckScript, CatalogCompleteScript, RegistryTestScript } from "./✅️catalog-verification/🟦️.ts";
import { TrustedCatalogPublishScript } from "./✅️trusted-stdio-catalog/🟦️.ts";
import { SessionScript } from "./🎮️playground/🧭️session/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("session", SessionScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("rust-taxonomy-mounts-check", RustTaxonomyMountsCheckScript).register("plugin-root-ownership-check", PluginRootOwnershipCheckScript).register("native-catalog-selection-check", NativeCatalogSelectionCheckScript).register("catalog-complete", CatalogCompleteScript).register("trusted-catalog-publish", TrustedCatalogPublishScript).register("check", CheckScript).register("test", RegistryTestScript).register("new", NewScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
