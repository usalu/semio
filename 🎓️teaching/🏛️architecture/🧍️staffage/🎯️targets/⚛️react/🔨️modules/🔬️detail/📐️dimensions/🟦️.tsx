/** 📐️ Dimensions and metadata of an asset, and what its size is on paper at every preset scale: the scale semantics made explicit.
 * @see ../../../../../../📐️units/🟦️.ts — `mmPerMetre` */
import type { ReactElement } from "react";
import { SCALE_PRESETS, mmPerMetre, powerOf } from "@teaching/staffage";
import type { Asset } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { Note, Section, SectionTitle, Tag } from "../../🪟️chrome/🟦️.tsx";
import { formatPowerW, type Row as CatalogRow } from "../../🔎️search/🟦️.ts";
import { DataTable, Fact, FactList, Row, num } from "../🧱️tables/🟦️.tsx";

/** 📐️ The dimension table, metadata list and size-on-paper table of one asset. */
export function DimensionsSection(props: { readonly asset: Asset; readonly row: CatalogRow }): ReactElement {
  const { asset, row } = props;
  const { t, language } = useArchive();
  const { dimensions } = asset;
  const power = powerOf(asset);
  return (
    <Section aria-labelledby="detail-dimensions" data-section="dimensions">
      <SectionTitle id="detail-dimensions">{t("detail.dimensions.title")}</SectionTitle>
      <Note>{asset.summary[language]}</Note>
      <FactList>
        <Fact term={t("detail.dimensions.height")}>{num(dimensions.heightM, language)} m</Fact>
        <Fact term={t("detail.dimensions.width")}>{num(dimensions.widthM, language)} m</Fact>
        <Fact term={t("detail.dimensions.depth")}>{num(dimensions.depthM, language)} m</Fact>
        <Fact term={t("detail.dimensions.mass")}>{num(dimensions.massKg, language)} kg</Fact>
        <Fact term={t("detail.dimensions.power")}>{formatPowerW(power.totalW, language)}</Fact>
        <Fact term={t("detail.dimensions.completion")}>{t(`completion.${asset.completion}`)}</Fact>
        <Fact term={t("detail.dimensions.category")}>{row.entry.category}</Fact>
        <Fact term={t("detail.dimensions.id")}>{asset.id}</Fact>
        <Fact term={t("detail.dimensions.titles")}><span lang="en">{asset.title.en}</span> · <span lang="de">{asset.title.de}</span></Fact>
        <Fact term={t("detail.dimensions.tags")}>
          <span className="flex flex-wrap gap-single">{asset.tags.map((tag) => <Tag key={tag}>{tag}</Tag>)}</span>
        </Fact>
      </FactList>
      <DataTable caption={t("detail.paper.caption", { height: num(dimensions.heightM, language), width: num(dimensions.widthM, language) })} head={[t("detail.paper.scale"), t("detail.paper.height"), t("detail.paper.width"), t("detail.paper.drawing"), t("detail.paper.line")]}>
        {SCALE_PRESETS.map((denominator) => {
          const scale = { denominator };
          const k = mmPerMetre(scale);
          const representation = asset.representations[`${denominator}`];
          const pen = representation.status === "authored" ? representation.pens.body : null;
          return (
            <Row key={denominator} head={<span data-scale={denominator}>1:{denominator}</span>} cells={[`${num(dimensions.heightM * k, language)} mm`, `${num(dimensions.widthM * k, language)} mm`, t(representation.status === "authored" ? "detail.paper.authored" : "detail.paper.unavailable"), pen === null ? "—" : `${num(pen, language)} mm`]} />
          );
        })}
      </DataTable>
      <Note>{t("detail.paper.note")}</Note>
    </Section>
  );
}
