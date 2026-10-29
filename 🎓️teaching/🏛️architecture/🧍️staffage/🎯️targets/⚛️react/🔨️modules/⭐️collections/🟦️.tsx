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
import { Button, ButtonLink, Notice, PageFrame, ProgressNotice, SectionHeading, TextField } from "../🪟️chrome/🟦️.tsx";
import type { PaneView } from "../🥞️levels/🟦️.tsx";
import { useAssetLoading } from "../📚️catalog/🟦️.ts";
import { formatAddress, navigate, useAddress } from "../🗺️routing/🟦️.ts";
import type { SavedCollection } from "../💾️persistence/🟦️.ts";
import { compareFirstFour, placeAllEvents, savedCollection, withAsset } from "./🧮️actions/🟦️.ts";

function Grid(props: { readonly slugs: readonly Slug[]; readonly footer?: (slug: Slug) => ReactNode }): ReactElement {
  const { rows } = useRows();
  return (
    <ul className="m-0 grid list-none gap-double p-0" style={{ gridTemplateColumns: "repeat(auto-fill, minmax(min(100%, 16rem), 1fr))" }}>
      {props.slugs.map((slug) => {
        const row = rows.find((candidate) => candidate.entry.id === slug);
        return (
          <li key={slug} className="flex min-w-0 flex-col gap-single" style={{ height: props.footer === undefined ? "18rem" : "21rem" }}>
            <div className="min-h-0 flex-1">{row === undefined ? <Notice kind="unavailable" compact title={slug} /> : <AssetCard row={row} />}</div>
            {props.footer?.(slug)}
          </li>
        );
      })}
    </ul>
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
    <section aria-labelledby={`saved-${collection.id}`} data-saved-collection={collection.id} className="flex flex-col gap-double border border-border bg-panel p-double">
      <div className="flex flex-wrap items-end gap-double">
        <h3 id={`saved-${collection.id}`} className="m-0 mr-auto font-serif text-lg">{collection.name}</h3>
        <TextField label={t("collections.rename")} value={name} onChange={setName} className="w-56" />
        <Button icon="check" disabled={name.trim() === "" || name.trim() === collection.name} onClick={() => save({ ...collection, name: name.trim() })}>{t("collections.renameApply")}</Button>
        <ButtonLink href={formatAddress({ page: "collections", collection: collection.id })} icon="eye">{t("collections.focus")}</ButtonLink>
        {confirm ? (
          <>
            <Button variant="primary" icon="trash-2" onClick={() => store.dispatch({ type: "collection-removed", id: collection.id })}>{t("collections.deleteConfirm")}</Button>
            <Button onClick={() => setConfirm(false)}>{t("action.cancel")}</Button>
          </>
        ) : (
          <Button icon="trash-2" onClick={() => setConfirm(true)}>{t("collections.delete")}</Button>
        )}
      </div>
      <div className="flex flex-wrap items-end gap-double">
        <label className="flex min-w-48 flex-col gap-single text-[0.65rem] uppercase tracking-wide text-muted-foreground">
          {t("collections.addAsset")}
          <select value={adding} onChange={(event) => setAdding(event.target.value)} className="min-h-6 border border-border bg-panel px-double py-single text-xs normal-case tracking-normal text-foreground">
            <option value="">{t("collections.choose")}</option>
            {addable.map((row) => <option key={row.entry.id} value={row.entry.id}>{row.entry.title[language]}</option>)}
          </select>
        </label>
        <Button icon="plus" disabled={adding === ""} onClick={() => { save(withAsset(collection, adding, true)); setAdding(""); }}>{t("collections.add")}</Button>
      </div>
      {collection.assets.length === 0 ? <p className="m-0 text-xs text-muted-foreground">{t("collections.savedEmpty")}</p> : <Grid slugs={collection.assets} footer={(slug) => <Button variant="quiet" icon="x" onClick={() => save(withAsset(collection, slug, false))}>{t("collections.removeFrom")}</Button>} />}
    </section>
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
      <div className="flex flex-wrap gap-single" role="group" aria-label={t("collections.actions", { title: label })}>
        <Button icon="combine" disabled={drawable.length < 2} onClick={() => compare(slugs)}>{t("collections.compare")}</Button>
        <Button icon="move" disabled={drawable.length === 0} onClick={() => place(slugs)}>{t("collections.place")}</Button>
        <Button icon="folder" disabled={slugs.length === 0} onClick={() => store.dispatch({ type: "collection-saved", collection: savedCollection(label, slugs) })}>{t("collections.saveAs")}</Button>
      </div>
    );
  };
  const create = (): void => {
    store.dispatch({ type: "collection-saved", collection: savedCollection(name, favorites) });
    setName("");
  };
  const header = loading.busy ? <ProgressNotice label={t("state.loadingAssets")} done={loading.done} total={loading.total} cancelLabel={t("action.cancel")} onCancel={loading.cancel} /> : null;
  if (focus !== null)
    return (
      <PageFrame page="collections" wide eyebrow={t("collections.title")} title={focusedCurated?.title[language] ?? focusedSaved?.name ?? t("collections.unknown")} lede={focusedCurated?.description[language]} actions={<ButtonLink href={formatAddress({ page: "collections", collection: null })} icon="arrow-left">{t("collections.all")}</ButtonLink>}>
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
          <form className="flex flex-wrap items-end gap-double" onSubmit={(event) => { event.preventDefault(); if (name.trim() !== "") create(); }}>
            <TextField label={t("collections.newName")} value={name} onChange={setName} className="w-64" />
            <Button type="submit" icon="plus" disabled={name.trim() === ""}>{t("collections.saveFavorites")}</Button>
          </form>
          <Grid slugs={favorites} />
        </>
      )}
      <SectionHeading id="collections-saved" aside={t("collections.count", { count: collections.length })}>{t("collections.saved")}</SectionHeading>
      {collections.length === 0 ? <Notice kind="empty" title={t("collections.savedNone")}>{t("collections.savedNoneBody")}</Notice> : <div className="flex flex-col gap-double">{collections.map((collection) => <SavedEditor key={collection.id} collection={collection} />)}</div>}
      <SectionHeading id="collections-curated" aside={t("collections.count", { count: curated.length })}>{t("collections.curated")}</SectionHeading>
      {manifest === null ? <CatalogProblem><Notice kind="loading" title={t("state.loading.title")} /></CatalogProblem> : curated.map((collection) => (
        <section key={collection.id} aria-labelledby={`curated-${collection.id}`} data-curated-collection={collection.id} className="flex flex-col gap-double">
          <div className="flex flex-wrap items-baseline justify-between gap-double">
            <h3 id={`curated-${collection.id}`} className="m-0 font-serif text-lg"><a href={formatAddress({ page: "collections", collection: collection.id })} className="text-inherit">{collection.title[language]}</a></h3>
            <p className="m-0 max-w-[70ch] text-xs text-muted-foreground">{collection.description[language]}</p>
          </div>
          {actions(collection.assets, collection.title)}
          <Grid slugs={collection.assets} />
        </section>
      ))}
      
    </PageFrame>
  );
}
