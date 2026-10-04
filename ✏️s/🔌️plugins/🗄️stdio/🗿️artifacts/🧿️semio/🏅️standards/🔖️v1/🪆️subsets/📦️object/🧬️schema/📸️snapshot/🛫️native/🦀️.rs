//! 📦️ Emits explicitly authored placement and independent child fields under caller control.
use super::SemioObjectSnapshot;
use crate::standards::v1::subsets::base::schema::snapshot::native_encoding::{self,Writer};
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};
pub(super) fn encode(snapshot:&SemioObjectSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{native_encoding::encode(encoding,"semio stdio.semio.object.dsl v1\n","stdio.semio.object.pack v1",control,|writer,encoding|fields(snapshot,writer,encoding))}
pub(crate) fn fields(snapshot:&SemioObjectSnapshot,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding)->Result<(),ValueError>{writer.entities(1)?;if encoding==SnapshotEncoding::Binary{writer.byte(1)?;}else{writer.bytes(b"schema=")?;}writer.string(&snapshot.schema,encoding)?;writer.delimiter(b"\ntransform=",encoding)?;writer.transform(&snapshot.transform,encoding)?;writer.delimiter(b"\nbrep=",encoding)?;writer.optional_child(&snapshot.brep,encoding)?;writer.delimiter(b"\nmesh=",encoding)?;writer.optional_child(&snapshot.mesh,encoding)?;writer.delimiter(b"\nproperties=",encoding)?;writer.optional_child(&snapshot.properties,encoding)}
