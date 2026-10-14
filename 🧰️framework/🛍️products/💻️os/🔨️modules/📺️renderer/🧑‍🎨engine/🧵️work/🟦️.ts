import stock from "./🔣️stock.json";
import {KernelOriginalBootstrapStock,RootOriginalBootstrapOwner,originalBootstrapStockFromWire} from "../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";
const original=new RootOriginalBootstrapOwner(new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stock)));
/** 🏛️ The real React/WGPU composition shares one explicitly configured kernel work stock for its whole lifetime. */
export function rendererOriginalWorkStock():RootOriginalBootstrapOwner{return original}
