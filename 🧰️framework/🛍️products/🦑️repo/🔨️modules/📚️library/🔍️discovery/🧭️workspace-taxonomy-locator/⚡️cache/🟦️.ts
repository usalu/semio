import { ephemeralBox } from "@semio-tech/framework-transient";
import type { Taxonomy } from "../../📑️taxonomy-document/🟦️.ts";

export const cachedTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);
export const cachedCatalogTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedCatalogTaxonomy", undefined);
