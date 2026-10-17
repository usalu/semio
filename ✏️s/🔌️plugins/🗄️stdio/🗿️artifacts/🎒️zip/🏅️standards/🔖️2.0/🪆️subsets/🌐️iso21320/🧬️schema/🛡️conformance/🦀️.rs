//! 🛡️ ISO/IEC 21320-1 header policy over an owned `ZipSnapshot`, shared by the composer, builder, analyzer, wire inspection and the SQLite snapshot guard.
use crate::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot;
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};

/// 🚩️ General-purpose bit 0 -- entry is encrypted (APPNOTE 4.4.4).
pub const FLAG_ENCRYPTED: u16 = 0x0001;
/// 🚩️ General-purpose bit 3 -- sizes/CRC live in a trailing data descriptor (APPNOTE 4.4.4).
pub const FLAG_DATA_DESCRIPTOR: u16 = 0x0008;
/// 🚩️ General-purpose bit 6 -- Strong Encryption extension in use (APPNOTE 4.4.4).
pub const FLAG_STRONG_ENCRYPTION: u16 = 0x0040;
/// 🚩️ General-purpose bit 13 -- central directory encrypted / local header values masked
/// (APPNOTE 4.4.4, paired with bit 6's Strong Encryption extension).
pub const FLAG_MASKED_LOCAL_HEADERS: u16 = 0x2000;
/// 🔢️ APPNOTE 4.4.3.2's ZIP64 version-needed threshold -- above this, an entry is declaring a
/// feature ISO/IEC 21320-1's restricted profile has no honest reason to need.
pub const VERSION_NEEDED_SOFT_CEILING: u16 = 45;

pub const CODE_ENCRYPTED: &str = "stdio.zip.iso21320.entry-encrypted";
pub const CODE_STRONG_ENCRYPTION: &str = "stdio.zip.iso21320.strong-encryption-or-masked-headers";
pub const CODE_DATA_DESCRIPTOR: &str = "stdio.zip.iso21320.data-descriptor-present";
pub const CODE_VERSION_NEEDED: &str = "stdio.zip.iso21320.version-needed-high";
pub const CODE_COMPRESSION_METHOD:&str="stdio.zip.iso21320.compression-method-unsupported";

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_iso21320_header(index:usize,name:&str,flags:u16,version_needed:u16,compression_method:u16,out:&mut Vec<Diagnostic>){
if flags & FLAG_ENCRYPTED != 0 {
    out.push(hard(CODE_ENCRYPTED, format!("entry {index} ({:?}) has general-purpose bit 0 (encryption) set -- ISO/IEC 21320-1 §4.1 forbids encrypted entries", name)));
}
if flags & (FLAG_STRONG_ENCRYPTION | FLAG_MASKED_LOCAL_HEADERS) != 0 {
    out.push(hard(
        CODE_STRONG_ENCRYPTION,
        format!("entry {index} ({:?}) has general-purpose bit 6 and/or bit 13 (Strong Encryption / masked local header values) set -- ISO/IEC 21320-1 forbids the Strong Encryption extension entirely", name),
    ));
}
if flags & FLAG_DATA_DESCRIPTOR != 0 {
    out.push(soft(CODE_DATA_DESCRIPTOR, format!("entry {index} ({:?}) has general-purpose bit 3 (trailing data descriptor) set -- interoperability warning: not every ISO/IEC 21320-1 reader trusts streamed sizes", name)));
}
if version_needed > VERSION_NEEDED_SOFT_CEILING {
    out.push(soft(
        CODE_VERSION_NEEDED,
        format!("entry {index} ({:?}) declares version-needed-to-extract {} > {VERSION_NEEDED_SOFT_CEILING} -- signals a feature ISO/IEC 21320-1's restricted Stored/Deflate profile shouldn't require", name, version_needed),
    ));
}
    if !matches!(compression_method,0|8){out.push(hard(CODE_COMPRESSION_METHOD,format!("entry {index} ({name:?}) declares compression method {compression_method} -- ISO/IEC 21320-1 §4.4 admits only Stored (0) and Deflate (8)")));}
}

/// 🛡️ Checks both independently owned member headers without wire encoding.
pub fn check_iso21320_conformance(snapshot:&ZipSnapshot)->Vec<Diagnostic>{
    let mut out=Vec::new();
    for(index,entry)in snapshot.entries.iter().enumerate(){check_iso21320_header(index,&entry.name,entry.metadata.local.flags|entry.metadata.central.flags,entry.metadata.local.version_needed.max(entry.metadata.central.version_needed),entry.metadata.compression_method,&mut out);}
    out
}

/// 🛡️ Bounds cancellation while checking explicit typed header policy.
pub fn check_iso21320_conformance_controlled(snapshot:&ZipSnapshot,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,store::sqlite_snapshot::ValueError>{
    use store::sqlite_snapshot::SqliteSnapshotPhase;
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,snapshot.entries.len())?;
    let mut out=Vec::new();
    for(index,entry)in snapshot.entries.iter().enumerate(){
        check_iso21320_header(index,&entry.name,entry.metadata.local.flags|entry.metadata.central.flags,entry.metadata.local.version_needed.max(entry.metadata.central.version_needed),entry.metadata.compression_method,&mut out);
        if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index+1,snapshot.entries.len())?;}
    }
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,snapshot.entries.len(),snapshot.entries.len())?;
    Ok(out)
}
