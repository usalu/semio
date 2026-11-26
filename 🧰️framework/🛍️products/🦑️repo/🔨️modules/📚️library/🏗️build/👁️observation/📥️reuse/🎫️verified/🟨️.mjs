import { lstatSync } from "node:fs";

/** 🏷️ The stamp of one inventory leaf: device, inode, mode, links, size and both clocks in nanoseconds; a remount moves the device, a replacement the inode and the clocks, so an unchanged stamp is the same content-addressed record.
 * @see ../🟨️.mjs `inventoryStampV1` (the same stamp the presence session compares) */
export const inventoryLeafStampV1 = row => [row.dev, row.ino, row.mode, row.nlink, row.size, row.mtimeNs, row.ctimeNs].join(":");

/** 🎫️ The inventory names a process found present under a storage consumer, each with the path and stamp of its leaf and the no-follow ancestry the session admitted, bounded to `maximum` names (the oldest leaves first): `remember(name, path, stamp, ancestry)` after a session saw the leaf, `holds(name, recheck)` answers whether the admitted ancestry still holds (`recheck(ancestry)` throws when a directory above was replaced, moved or became a link) and one `lstat` finds the same single-link file; a name whose leaf or ancestry changed or vanished is forgotten and must be asked in a session again (which then decides: a vanished leaf is a miss, a change during that look a refusal), `stats()` counts names held, hits, misses and names newly remembered (a name seen again while held is not counted twice). Every inventory record is content addressed (its name is its digest), so a leaf with an unchanged stamp is the record the session verified; a later ask needs no consumer of the mounted image.
 * @see ../🧬️schema/🔣️.json `verifiedNamesMaximum` */
export function createVerifiedPresenceV1({ maximum, stat = path => lstatSync(path, { bigint: true }) }) {
  const rows = new Map(), counts = { hits: 0, misses: 0, remembered: 0 };
  return Object.freeze({
    remember(name, path, stamp, ancestry) {
      if (!rows.delete(name)) counts.remembered++;
      rows.set(name, { path, stamp, ancestry });
      while (rows.size > maximum) rows.delete(rows.keys().next().value);
    },
    /** @param {string} name @param {(ancestry: unknown) => void} [recheck] */
    holds(name, recheck = () => {}) {
      const row = rows.get(name);
      if (row === undefined) return false;
      try { recheck(row.ancestry); const now = stat(row.path); if (now.isFile() && !now.isSymbolicLink() && now.nlink === 1n && inventoryLeafStampV1(now) === row.stamp) { counts.hits++; return true; } } catch {}
      rows.delete(name); counts.misses++; return false;
    },
    has: name => rows.has(name),
    stats: () => Object.freeze({ names: rows.size, ...counts }),
  });
}
