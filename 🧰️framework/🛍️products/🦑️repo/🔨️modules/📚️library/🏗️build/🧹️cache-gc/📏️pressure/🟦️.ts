import{readFile}from"node:fs/promises";
import{join}from"node:path";
import{CACHE_SIZE_FILE_V1,cacheLedgerV1,cacheRootDirectoryV1,readCacheSizeLedgerV1,type CacheLedgerV1}from"../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import{overBudgetAreasV1}from"../🧮️sweep/🟦️.ts";

/** 📏️ The cheap trigger of the ledger collector: the ids of the collectable areas whose recorded size is above their budget, read from the small size ledger of the repository cache root (no tree is walked and no command graph is loaded; only the ledger, the sweep rule and the file system). An unreadable or torn ledger answers none, because sizes are only a remembered measurement. Never throws.
 * @see ../🧮️sweep/🟦️.ts the rule that names the areas */
export async function ledgerOverBudgetAreasV1(workspace:string,ledger:CacheLedgerV1=cacheLedgerV1()):Promise<readonly string[]>{
 try{
  const text=await readFile(join(cacheRootDirectoryV1(workspace,ledger,"repository"),CACHE_SIZE_FILE_V1),"utf8");
  return overBudgetAreasV1(ledger,readCacheSizeLedgerV1(JSON.parse(text)));
 }catch{return Object.freeze([]);}
}
