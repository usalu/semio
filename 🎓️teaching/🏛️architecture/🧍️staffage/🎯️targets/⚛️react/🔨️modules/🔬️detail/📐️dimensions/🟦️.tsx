/** 📐️ Dimensions and metadata of an asset, and what its size is on paper at every preset scale: the scale semantics made explicit.
 * @see ../../../../../../📐️units/🟦️.ts — `mmPerMetre`, `lodFor`, `lineWeightMm` */
import type { ReactElement } from "react";
import { SCALE_PRESETS, lineWeightMm, lodFor, mmPerMetre } from "@teaching/staffage";
import type { Asset } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Tag } from "../../🪟️chrome/🟦️.tsx";
import type { Row as CatalogRow } from "../../🔎️search/🟦️.ts";
import { DataTable, Fact, Row, num } from "../🧱️tables/🟦️.tsx";

/** 📐️ The dimension table, metadata list and size-on-paper table of one asset. */
export function DimensionsSection(props: { readonly asset: Asset; readonly row: CatalogRow }): ReactElement {
  const { asset, row } = props;
  const { t, language } = useArchive();
  const { dimensions } = asset;
  return (
    <section aria-labelledby="detail-dimensions" className="flex flex-col gap-double" data-section="dimensions">
      <h2 id="detail-dimensions" className="m-0 font-serif text-lg">{t("detail.dimensions.title")}</h2>
      <p className="m-0 max-w-[70ch] text-sm leading-normal text-muted-foreground">{asset.summary[language]}</p>
      <dl className="m-0 grid gap-x-4 sm:grid-cols-2 xl:grid-cols-3">
        <Fact term={t("detail.dimensions.height")}>{num(dimensions.heightM, language)} m</Fact>
        <Fact term={t("detail.dimensions.width")}>{num(dimensions.widthM, language)} m</Fact>
        <Fact term={t("detail.dimensions.depth")}>{num(dimensions.depthM, language)} m</Fact>
        <Fact term={t("detail.dimensions.mass")}>{num(dimensions.massKg, language)} kg</Fact>
        <Fact term={t("detail.dimensions.viewpoints")}>{asset.presentation.viewpoints.map((viewpoint) => t(`detail.viewpoint.${viewpoint}`)).join(", ")}</Fact>
        <Fact term={t("detail.dimensions.completion")}>{t(`completion.${asset.completion}`)}</Fact>
        <Fact term={t("detail.dimensions.category")}>{row.entry.category}</Fact>
        <Fact term={t("detail.dimensions.id")}>{asset.id}</Fact>
        <Fact term={t("detail.dimensions.titles")}><span lang="en">{asset.title.en}</span> · <span lang="de">{asset.title.de}</span></Fact>
        <Fact term={t("detail.dimensions.tags")}>
          <span className="flex flex-wrap gap-single">{asset.tags.map((tag) => <Tag key={tag}>{tag}</Tag>)}</span>
        </Fact>
      </dl>
      <DataTable caption={t("detail.paper.caption", { height: num(dimensions.heightM, language), width: num(dimensions.widthM, language) })} head={[t("detail.paper.scale"), t("detail.paper.height"), t("detail.paper.width"), t("detail.paper.lod"), t("detail.paper.line")]}>
        {SCALE_PRESETS.map((denominator) => {
          const scale = { denominator };
          const k = mmPerMetre(scale);
          return (
            <Row key={denominator} head={<span data-scale={denominator}>1:{denominator}</span>} cells={[`${num(dimensions.heightM * k, language)} mm`, `${num(dimensions.widthM * k, language)} mm`, t("detail.paper.lodLevel", { level: lodFor(scale) }), `${num(lineWeightMm(scale, "body") ?? 0, language)} mm`]} />
          );
        })}
      </DataTable>
      <p className="m-0 text-xs text-muted-foreground">{t("detail.paper.note")}</p>
    </section>
  );
}
