/** 📚️ Provenance of an asset: who made it and how, its references, and which parameters rely on which reference.
 * @see ../../../../../../🧬️schema/🟦️.ts — `Provenance`, `Reference` */
import type { ReactElement } from "react";
import type { Asset, ParameterSource } from "@teaching/staffage";
import { Icon, Note, Section, SectionTitle } from "../../🪟️chrome/🟦️.tsx";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { DataTable, Fact, FactList, Row } from "../🧱️tables/🟦️.tsx";
import { SOURCES } from "../🧱️tables/🟦️.tsx";

/** 🔗️ How many parameters cite each reference id. */
export function referenceCounts(asset: Asset): Readonly<Record<string, number>> {
  const counts: Record<string, number> = {};
  const cite = (reference: string | undefined): void => {
    if (reference !== undefined) counts[reference] = (counts[reference] ?? 0) + 1;
  };
  asset.materials.forEach((item) => cite(item.reference));
  asset.rig.bones.forEach((bone) => cite(bone.mass?.reference));
  asset.thermal.regions.forEach((item) => cite(item.reference));
  asset.thermal.emitters.forEach((item) => cite(item.reference));
  asset.poses.forEach((pose) => pose.loads.forEach((load) => cite(load.reference)));
  return counts;
}

/** 🧮️ How many parameters carry each source tag. */
export function sourceCounts(asset: Asset): Readonly<Record<ParameterSource, number>> {
  const counts: Record<ParameterSource, number> = { measured: 0, literature: 0, calibrated: 0, "authored-estimate": 0, illustrative: 0 };
  const count = (source: ParameterSource | undefined): void => {
    if (source !== undefined) counts[source] += 1;
  };
  asset.materials.forEach((item) => count(item.source));
  asset.rig.bones.forEach((bone) => count(bone.mass?.source));
  asset.thermal.regions.forEach((item) => count(item.source));
  asset.thermal.emitters.forEach((item) => count(item.source));
  asset.poses.forEach((pose) => pose.loads.forEach((load) => count(load.source)));
  return counts;
}

/** 📚️ The provenance section. */
export function ProvenanceSection(props: { readonly asset: Asset }): ReactElement {
  const { asset } = props;
  const { t, language } = useArchive();
  const { provenance } = asset;
  const cited = referenceCounts(asset);
  const sources = sourceCounts(asset);
  return (
    <Section aria-labelledby="detail-provenance" data-section="provenance">
      <SectionTitle id="detail-provenance">{t("detail.provenance.title")}</SectionTitle>
      <FactList>
        <Fact term={t("detail.provenance.author")}>{provenance.author}</Fact>
        <Fact term={t("detail.provenance.method")}>{t(`detail.provenance.method.${provenance.method}`)}</Fact>
        <Fact term={t("detail.provenance.created")}>{provenance.created}</Fact>
      </FactList>
      {provenance.notes === undefined ? null : <Note tone="plain">{provenance.notes[language]}</Note>}
      <DataTable caption={t("detail.provenance.references")} head={[t("detail.provenance.reference"), t("detail.provenance.citation"), t("detail.provenance.cited")]}>
        {provenance.references.map((reference) => (
          <Row
            key={reference.id}
            head={reference.id}
            cells={[
              <span key="c" className="font-sans">
                {reference.citation}
                {reference.url === undefined ? null : (
                  <>
                    {" "}
                    <a href={reference.url} data-shared-border-frame="" data-shared-border-edges="" target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-single underline">{t("detail.provenance.open")}<Icon icon="external-link" size="tiny" /><span className="sr-only"> ({t("detail.newTab")})</span></a>
                  </>
                )}
              </span>,
              cited[reference.id] ?? 0,
            ]}
          />
        ))}
      </DataTable>
      <Note>
        <span data-sources="">{t("detail.provenance.sources")}: {SOURCES.map((source) => `${t(`detail.source.${source}`)} ${sources[source]}`).join(" · ")}</span>
      </Note>
    </Section>
  );
}
