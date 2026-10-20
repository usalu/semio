type TaxonomyFileKindsV1 = Readonly<{fileKinds:Readonly<Record<string,Readonly<{emoji:string;extensionChains:readonly string[]}>>>}>;

/** 📄️Queries authored file-kind names without loading taxonomy audits or caches. */
export function taxonomyFileKindFilenamesV1(kindId:string,taxonomy:TaxonomyFileKindsV1):string[]{
 const kind=taxonomy.fileKinds[kindId];return kind?kind.extensionChains.map(extension=>`${kind.emoji}${extension}`):[];
}

/** 📝️Requires the unique authored filename of a selected implementation kind. */
export function taxonomySingleFileKindFilenameV1(kindId:string,taxonomy:TaxonomyFileKindsV1):string{
 const names=taxonomyFileKindFilenamesV1(kindId,taxonomy);if(names.length!==1)throw Error(`File kind ${JSON.stringify(kindId)} must have exactly one extension chain, got ${names.length}.`);return names[0]!;
}
