/** ⭐️ Collections: the reader's favorites, the manifest's curated collections and the collections the reader saved, all kept on this device.
 *
 * Favorites and saved collections are persisted local-only events (`favorite-added`, `collection-saved`, …). A curated collection can be compared (its first four assets), placed on the playground side by side, or saved as one's own.
 * `#/collections/<id>` focuses one curated or saved collection.
 *
 * @see ./🧮️actions/🟦️.ts — the pure rules
 * @see ../🗃️browse/🟦️.tsx — the asset card and favorite toggle
 */
import { CatalogProblem } from "../🖼️previews/🟦️.tsx";
import { useMemo, useState, type ReactElement, type ReactNode } from "react";
import type { CategoryId, Slug, Text } from "@teaching/staffage";
import { useArchive, useLocal, useRows } from "../🧭️context/🟦️.tsx";
import { AssetCard } from "../🗃️browse/🟦️.tsx";
import { Bar, Button, CardGrid, CardGridCell, ButtonGroup, ButtonLink, Cell, Column, Note, Notice, PageFrame, ProgressNotice, SectionHeading, SelectField, TextField } from "../🪟️chrome/🟦️.tsx";
import type { PaneView } from "../🥞️levels/🟦️.tsx";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { formatAddress, navigate, useAddress } from "../🗺️routing/🟦️.ts";
import type { SavedCollection } from "../💾️persistence/🟦️.ts";
import { compareFirstFour, placeAllEvents, savedCollection, withAsset } from "./🧮️actions/🟦️.ts";

/** ⭐️ The edges a collection paints when a grid of cards closes it: the cards own the bottom line. */
const GRID_BELOW = ["top", "right", "left"] as const;

function Grid(props: { readonly slugs: readonly Slug[]; readonly footer?: (slug: Slug) => ReactNode }): ReactElement {
  const { rows } = useRows();
  return (
    <CardGrid as="ul" minColumn="16rem" className="m-0 list-none">
      {props.slugs.map((slug) => {
        const row = rows.find((candidate) => candidate.entry.id === slug);
        return (
          <CardGridCell as="li" key={slug} className="h-72">
            {row === undefined ? <Notice kind="unavailable" compact plain title={slug} /> : <AssetCard row={row} {...(props.footer === undefined ? {} : { footer: props.footer(slug) })} />}
          </CardGridCell>
        );
      })}
    </CardGrid>
  );
}

function SavedEditor(props: { readonly collection: SavedCollection }): ReactElement {
  const { collection } = props;
  const { t, store, language } = useArchive();
  const { rows } = useRows();
  const [name, setName] = useState(collection.name);
  const [confirm, setConfirm] = useState(false);
  const [adding, setAdding] = useState("");
  const addable = rows.filter((row) => !collection.assets.includes(row.entry.id));
  const save = (next: SavedCollection): void => store.dispatch({ type: "collection-saved", collection: next });
  return (
    <Column as="section" aria-labelledby={`saved-${collection.id}`} data-saved-collection={collection.id} {...(collection.assets.length === 0 ? {} : { edges: GRID_BELOW })}>
      <Bar below="tablet">
        <Cell className="flex-1"><h3 id={`saved-${collection.id}`} className="m-0 font-serif text-lg">{collection.name}</h3></Cell>
        <TextField label={t("collections.rename")} value={name} onChange={setName} className="lg:max-w-72" />
        <Button grouped icon="check" disabled={name.trim() === "" || name.trim() === collection.name} onClick={() => save({ ...collection, name: name.trim() })}>{t("collections.renameApply")}</Button>
        <ButtonLink grouped href={formatAddress({ page: "collections", collection: collection.id })} icon="eye">{t("collections.focus")}</ButtonLink>
        {confirm ? (
          <>
            <Button grouped variant="primary" icon="trash-2" onClick={() => store.dispatch({ type: "collection-removed", id: collection.id })}>{t("collections.deleteConfirm")}</Button>
            <Button grouped onClick={() => setConfirm(false)}>{t("action.cancel")}</Button>
          </>
        ) : (
          <Button grouped icon="trash-2" onClick={() => setConfirm(true)}>{t("collections.delete")}</Button>
        )}
      </Bar>
      <Bar below="tablet">
        <SelectField label={t("collections.addAsset")} value={adding} onChange={setAdding} options={[{ value: "", label: t("collections.choose") }, ...addable.map((row) => ({ value: row.entry.id, label: row.entry.title[language] }))]} />
        <Button grouped icon="plus" disabled={adding === ""} onClick={() => { save(withAsset(collection, adding, true)); setAdding(""); }}>{t("collections.add")}</Button>
      </Bar>
      {collection.assets.length === 0 ? <Note>{t("collections.savedEmpty")}</Note> : <Grid slugs={collection.assets} footer={(slug) => <Button variant="quiet" icon="x" onClick={() => save(withAsset(collection, slug, false))}>{t("collections.removeFrom")}</Button>} />}
    </Column>
  );
}

/** ⭐️ The collections page. */
export function CollectionsPage(props: { readonly view: PaneView }): ReactElement {
  const { view } = props;
  const { t, language, store, catalog } = useArchive();
  const { rows, manifest } = useRows();
  const { favorites, collections } = useLocal();
  const address = useAddress();
  const focus = view.opened && address.route.page === "collections" ? address.route.collection : null;
  const [name, setName] = useState("");
  const curated = manifest?.collections ?? [];
  const focusedCurated = curated.find((collection) => collection.id === focus);
  const focusedSaved = collections.find((collection) => collection.id === focus);
  const shownSlugs = useMemo(() => (focus === null ? [...favorites, ...curated.flatMap((collection) => collection.assets), ...collections.flatMap((collection) => collection.assets)] : focusedCurated?.assets ?? focusedSaved?.assets ?? []), [focus, favorites, curated, collections, focusedCurated, focusedSaved]);
  const categories = useMemo(() => [...new Set(shownSlugs.flatMap((slug) => rows.filter((row) => row.entry.id === slug).map((row) => row.entry.category)))] as CategoryId[], [shownSlugs.join(), rows.length]);
  const loading = useAssetLoading(catalog, view.opened || view.revealed, categories, categories.join());
  const assetOf = (slug: Slug) => rows.find((row) => row.entry.id === slug)?.asset;
  const place = (slugs: readonly Slug[]): void => {
    const assets = slugs.flatMap((slug) => assetOf(slug) ?? []);
    for (const event of placeAllEvents(assets, store.getSnapshot().playground)) store.dispatch(event);
    navigate({ page: "playground" });
  };
  const compare = (slugs: readonly Slug[]): void => {
    const ready = slugs.filter((slug) => assetOf(slug) !== undefined);
    const params = compareFirstFour(ready);
    if (params !== null) navigate({ page: "compare" }, params);
  };
  const actions = (slugs: readonly Slug[], title: Text | string): ReactElement => {
    const drawable = slugs.filter((slug) => assetOf(slug) !== undefined);
    const label = typeof title === "string" ? title : title[language];
    return (
      <ButtonGroup label={t("collections.actions", { title: label })}>
        <Button grouped className="flex-1" icon="combine" disabled={drawable.length < 2} onClick={() => compare(slugs)}>{t("collections.compare")}</Button>
        <Button grouped className="flex-1" icon="move" disabled={drawable.length === 0} onClick={() => place(slugs)}>{t("collections.place")}</Button>
        <Button grouped className="flex-1" icon="folder" disabled={slugs.length === 0} onClick={() => store.dispatch({ type: "collection-saved", collection: savedCollection(label, slugs) })}>{t("collections.saveAs")}</Button>
      </ButtonGroup>
    );
  };
  const create = (): void => {
    store.dispatch({ type: "collection-saved", collection: savedCollection(name, favorites) });
    setName("");
  };
  const header = loading.busy ? <ProgressNotice label={t("state.loadingAssets")} done={loading.done} total={loading.total} cancelLabel={t("action.cancel")} onCancel={loading.cancel} /> : null;
  if (focus !== null)
    return (
      <PageFrame page="collections" wide eyebrow={t("collections.title")} title={focusedCurated?.title[language] ?? focusedSaved?.name ?? t("collections.unknown")} lede={focusedCurated?.description[language]} actions={<ButtonLink grouped href={formatAddress({ page: "collections", collection: null })} icon="arrow-left">{t("collections.all")}</ButtonLink>}>
        {header}
        {focusedCurated === undefined && focusedSaved === undefined ? (
          <Notice kind="empty" title={t("collections.unknown")}>{t("collections.unknownBody", { id: focus })}</Notice>
        ) : (
          <>
            {actions(shownSlugs, focusedCurated?.title ?? focusedSaved!.name)}
            <Grid slugs={shownSlugs} />
          </>
        )}
      </PageFrame>
    );
  return (
    <PageFrame page="collections" wide eyebrow={t("collections.eyebrow")} title={t("collections.title")} lede={t("collections.lede")}>
      {header}
      <SectionHeading id="collections-favorites" aside={t("collections.favoritesCount", { count: favorites.length })}>{t("collections.favorites")}</SectionHeading>
      {favorites.length === 0 ? (
        <Notice kind="empty" title={t("collections.noFavorites")} actions={<ButtonLink href={formatAddress({ page: "all" })} icon="search">{t("collections.browse")}</ButtonLink>}>{t("collections.favoritesLocal")}</Notice>
      ) : (
        <>
          <Bar as="form" below="tablet" onSubmit={(event) => { event.preventDefault(); if (name.trim() !== "") create(); }}>
            <TextField label={t("collections.newName")} value={name} onChange={setName} />
            <Button grouped type="submit" icon="plus" disabled={name.trim() === ""}>{t("collections.saveFavorites")}</Button>
          </Bar>
          <Grid slugs={favorites} />
        </>
      )}
      <SectionHeading id="collections-saved" aside={t("collections.count", { count: collections.length })}>{t("collections.saved")}</SectionHeading>
      {collections.length === 0 ? <Notice kind="empty" title={t("collections.savedNone")}>{t("collections.savedNoneBody")}</Notice> : collections.map((collection) => <SavedEditor key={collection.id} collection={collection} />)}
      <SectionHeading id="collections-curated" aside={t("collections.count", { count: curated.length })}>{t("collections.curated")}</SectionHeading>
      {manifest === null ? <CatalogProblem><Notice kind="loading" title={t("state.loading.title")} /></CatalogProblem> : curated.map((collection) => (
        <Column as="section" key={collection.id} aria-labelledby={`curated-${collection.id}`} data-curated-collection={collection.id} edges={GRID_BELOW}>
          <Cell className="items-baseline justify-between">
            <h3 id={`curated-${collection.id}`} className="m-0 font-serif text-lg"><a href={formatAddress({ page: "collections", collection: collection.id })} data-shared-border-frame="" data-shared-border-edges="" className="text-inherit">{collection.title[language]}</a></h3>
            <p className="m-0 max-w-[70ch] text-xs text-muted-foreground">{collection.description[language]}</p>
          </Cell>
          {actions(collection.assets, collection.title)}
          <Grid slugs={collection.assets} />
        </Column>
      ))}
      
    </PageFrame>
  );
}
