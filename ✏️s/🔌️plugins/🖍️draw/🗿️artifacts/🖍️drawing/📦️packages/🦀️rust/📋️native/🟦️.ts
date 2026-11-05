/** 📋️ Declares every native Drawing compilation and assertion owner. */
import catalog from "./🔣️.json" with { type: "json" };

export const drawingNativeOptions = Object.freeze({
  packages: Object.freeze(catalog.packages.filter(row => row.check).map(row => row.name)),
  testPackages: Object.freeze(catalog.packages.filter(row => row.test).map(row => row.name)),
});
