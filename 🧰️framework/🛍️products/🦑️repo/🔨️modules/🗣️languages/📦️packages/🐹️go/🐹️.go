// 🗜️Package languages owns the repo language table and the parsers built on it.
//
// The table itself is the single source of truth shared with the Rust twin
// (semio-framework-repo-languages), which embeds the same file with include_str!. Nothing in this
// package may re-declare a comment prefix, a region marker, a definition keyword or a header format:
// read it from the table.
package languages

