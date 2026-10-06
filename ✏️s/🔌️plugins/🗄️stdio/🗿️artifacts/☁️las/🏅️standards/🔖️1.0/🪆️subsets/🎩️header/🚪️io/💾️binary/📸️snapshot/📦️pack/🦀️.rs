//! 📦️ Typed LAS snapshot records keep file encoding outside snapshot identity.
use crate::standards::v1_0::subsets::any::schema::snapshot::{LasHeader, LasPoint, LasSnapshot, LasVlr};
#[path="🚦️native/🦀️.rs"]
pub(crate) mod native;

#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Header {
    version_major: u8,
    version_minor: u8,
    system_identifier: String,
    generating_software: String,
    creation_day_of_year: u16,
    creation_year: u16,
    header_size: u16,
    offset_to_point_data: u32,
    number_of_vlrs: u32,
    point_data_format_id: u8,
    point_data_record_length: u16,
    number_of_point_records: u32,
    points_by_return: [u32; 5],
    x_scale_ieee754_bits: u64,
    y_scale_ieee754_bits: u64,
    z_scale_ieee754_bits: u64,
    x_offset_ieee754_bits: u64,
    y_offset_ieee754_bits: u64,
    z_offset_ieee754_bits: u64,
    max_x_ieee754_bits: u64,
    min_x_ieee754_bits: u64,
    max_y_ieee754_bits: u64,
    min_y_ieee754_bits: u64,
    max_z_ieee754_bits: u64,
    min_z_ieee754_bits: u64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Rgb { red: u16, green: u16, blue: u16 }

#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Point {
    x_ieee754_bits: u64,
    y_ieee754_bits: u64,
    z_ieee754_bits: u64,
    intensity: u16,
    return_number: u8,
    number_of_returns: u8,
    scan_direction_flag: bool,
    edge_of_flight_line: bool,
    classification: u8,
    scan_angle_rank: i8,
    user_data: u8,
    point_source_id: u16,
    gps_time_ieee754_bits: Option<u64>,
    rgb: Option<Rgb>,
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Snapshot {
    schema: String,
    header: Header,
    vlrs: Vec<LasVlr>,
    points: Vec<Point>,
}

impl From<&LasSnapshot> for Snapshot {
    fn from(snapshot: &LasSnapshot) -> Self {
        let h = &snapshot.header;
        Self {
            schema: snapshot.schema.clone(),
            header: Header {
                version_major: h.version_major, version_minor: h.version_minor,
                system_identifier: h.system_identifier.clone(), generating_software: h.generating_software.clone(),
                creation_day_of_year: h.creation_day_of_year, creation_year: h.creation_year,
                header_size: h.header_size, offset_to_point_data: h.offset_to_point_data,
                number_of_vlrs: h.number_of_vlrs, point_data_format_id: h.point_data_format_id,
                point_data_record_length: h.point_data_record_length, number_of_point_records: h.number_of_point_records,
                points_by_return: h.points_by_return,
                x_scale_ieee754_bits: h.x_scale.to_bits(), y_scale_ieee754_bits: h.y_scale.to_bits(), z_scale_ieee754_bits: h.z_scale.to_bits(),
                x_offset_ieee754_bits: h.x_offset.to_bits(), y_offset_ieee754_bits: h.y_offset.to_bits(), z_offset_ieee754_bits: h.z_offset.to_bits(),
                max_x_ieee754_bits: h.max_x.to_bits(), min_x_ieee754_bits: h.min_x.to_bits(),
                max_y_ieee754_bits: h.max_y.to_bits(), min_y_ieee754_bits: h.min_y.to_bits(),
                max_z_ieee754_bits: h.max_z.to_bits(), min_z_ieee754_bits: h.min_z.to_bits(),
            },
            vlrs: snapshot.vlrs.clone(),
            points: snapshot.points.iter().map(|p| Point {
                x_ieee754_bits: p.x.to_bits(), y_ieee754_bits: p.y.to_bits(), z_ieee754_bits: p.z.to_bits(),
                intensity: p.intensity, return_number: p.return_number, number_of_returns: p.number_of_returns,
                scan_direction_flag: p.scan_direction_flag, edge_of_flight_line: p.edge_of_flight_line,
                classification: p.classification, scan_angle_rank: p.scan_angle_rank,
                user_data: p.user_data, point_source_id: p.point_source_id,
                gps_time_ieee754_bits: p.gps_time.map(f64::to_bits), rgb: p.rgb.map(|(red,green,blue)|Rgb{red,green,blue}),
            }).collect(),
        }
    }
}

impl From<Snapshot> for LasSnapshot {
    fn from(snapshot: Snapshot) -> Self {
        let h = snapshot.header;
        Self {
            schema: snapshot.schema,
            header: LasHeader {
                version_major: h.version_major, version_minor: h.version_minor,
                system_identifier: h.system_identifier, generating_software: h.generating_software,
                creation_day_of_year: h.creation_day_of_year, creation_year: h.creation_year,
                header_size: h.header_size, offset_to_point_data: h.offset_to_point_data,
                number_of_vlrs: h.number_of_vlrs, point_data_format_id: h.point_data_format_id,
                point_data_record_length: h.point_data_record_length, number_of_point_records: h.number_of_point_records,
                points_by_return: h.points_by_return,
                x_scale: f64::from_bits(h.x_scale_ieee754_bits), y_scale: f64::from_bits(h.y_scale_ieee754_bits), z_scale: f64::from_bits(h.z_scale_ieee754_bits),
                x_offset: f64::from_bits(h.x_offset_ieee754_bits), y_offset: f64::from_bits(h.y_offset_ieee754_bits), z_offset: f64::from_bits(h.z_offset_ieee754_bits),
                max_x: f64::from_bits(h.max_x_ieee754_bits), min_x: f64::from_bits(h.min_x_ieee754_bits),
                max_y: f64::from_bits(h.max_y_ieee754_bits), min_y: f64::from_bits(h.min_y_ieee754_bits),
                max_z: f64::from_bits(h.max_z_ieee754_bits), min_z: f64::from_bits(h.min_z_ieee754_bits),
            },
            vlrs: snapshot.vlrs,
            points: snapshot.points.into_iter().map(|p| LasPoint {
                x: f64::from_bits(p.x_ieee754_bits), y: f64::from_bits(p.y_ieee754_bits), z: f64::from_bits(p.z_ieee754_bits),
                intensity: p.intensity, return_number: p.return_number, number_of_returns: p.number_of_returns,
                scan_direction_flag: p.scan_direction_flag, edge_of_flight_line: p.edge_of_flight_line,
                classification: p.classification, scan_angle_rank: p.scan_angle_rank,
                user_data: p.user_data, point_source_id: p.point_source_id,
                gps_time: p.gps_time_ieee754_bits.map(f64::from_bits), rgb: p.rgb.map(|rgb|(rgb.red,rgb.green,rgb.blue)),
            }).collect(),
        }
    }
}



impl store::ArtifactPack for LasSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let snapshot = Snapshot::from(self);
        let inner = store::pack_rt::encode_document(&Snapshot::__dsl_spec(), &snapshot.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.las", store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity("stdio.las", store::semio_format::Component::Pack, 1) { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "LAS snapshot pack identity differs"))); }
        let (record, _) = store::pack_rt::decode_document(&inner, &Snapshot::__dsl_spec(), options)?;
        Snapshot::__dsl_from_record(&record).map(Into::into).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Snapshot::__dsl_spec()) }
}
