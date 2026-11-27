import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const place=schema.definitions.Protocol.const,segment=new RegExp("^(.*?[\\\\/])"+place.directory+"[\\\\/]([0-9a-f]{"+place.idCharacters+"})(?:[\\\\/](.*))?$","u");

/** 🧭️ The root of the checkout snapshot (`<state>/s/<id>`) a path lies in or is, or `undefined` for every other path. It needs no file access, so the compiler wrapper of every unit can ask it. */
export function checkoutSnapshotRootOfV1(path:string):string|undefined{
 const match=segment.exec(path);
 return match===null?undefined:match[1]+place.directory+path.charAt(match[1]!.length+place.directory.length)+match[2];
}

/** 🧭️ The workspace-relative path (posix) of a file below ANY checkout snapshot root (`<state>/s/<id>/<path>`), or `undefined` for every other path: it lets a receipt written from one snapshot be compared with the files of another or of the live workspace. */
export function checkoutSnapshotRelativeV1(path:string):string|undefined{
 const match=segment.exec(path);
 return match===null||match[3]===undefined||match[3]===""?undefined:match[3].split(/[\\/]/u).join("/");
}
