import stock from "./🔣️stock.json" with { type: "json" };
import {KernelOriginalBootstrapStock,RootOriginalBootstrapOwner,originalBootstrapStockFromWire} from "@semio-tech/framework-actor/retained-turn/bootstrap";
const original=new RootOriginalBootstrapOwner(new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stock)));
/** 🏛️ The real React/WGPU composition shares one explicitly configured kernel work stock for its whole lifetime. */
export function rendererOriginalWorkStock():RootOriginalBootstrapOwner{return original}
