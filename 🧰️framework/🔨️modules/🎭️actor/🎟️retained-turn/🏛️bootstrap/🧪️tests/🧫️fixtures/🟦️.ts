import stock from "./📋️composition.json";
import {KernelOriginalBootstrapStock,RootOriginalBootstrapOwner,originalBootstrapStockFromWire} from "../../🟦️.ts";
/** 🧪️ Each fixture composition explicitly admits its finite initial stock once. */
export function fixtureOriginalWorkStock():RootOriginalBootstrapOwner{return new RootOriginalBootstrapOwner(new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stock)))}
