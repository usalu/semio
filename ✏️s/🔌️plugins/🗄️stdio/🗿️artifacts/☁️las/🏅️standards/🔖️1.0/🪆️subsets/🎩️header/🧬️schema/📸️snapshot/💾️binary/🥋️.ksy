meta:
  id: stdio_las_snapshot
  endian: le
doc: |
  §LAS 1.0 public header block (227 bytes) + Variable Length Records + point data records
  (formats 0-3). Matches `engine::{encode_las, decode_las}` exactly. `file_source_id`/
  `global_encoding`/`project_id_guid` are spec-real but out of this artifact's contracted
  field list -- skipped, not indexed by the codec.
seq:
  - id: magic
    contents: "LASF"
  - id: file_source_id
    size: 2
  - id: global_encoding
    size: 2
  - id: project_id_guid
    size: 16
  - id: version_major
    type: u1
  - id: version_minor
    type: u1
  - id: system_identifier
    size: 32
    type: strz
    encoding: ASCII
  - id: generating_software
    size: 32
    type: strz
    encoding: ASCII
  - id: creation_day_of_year
    type: u2
  - id: creation_year
    type: u2
  - id: header_size
    type: u2
    doc: STRUCTURAL -- engine::encode_las always recomputes this (fixed 227).
  - id: offset_to_point_data
    type: u4
    doc: STRUCTURAL -- always recomputed as 227 + Σ(vlr header + vlr data).
  - id: number_of_vlrs
    type: u4
    doc: STRUCTURAL -- always recomputed as vlrs.len().
  - id: point_data_format_id
    type: u1
    doc: STRUCTURAL -- chosen from which optional point fields are populated.
  - id: point_data_record_length
    type: u2
    doc: STRUCTURAL -- derived from point_data_format_id.
  - id: number_of_point_records
    type: u4
    doc: STRUCTURAL -- always recomputed as points.len().
  - id: points_by_return
    type: u4
    repeat: expr
    repeat-expr: 5
  - id: x_scale
    type: f8
  - id: y_scale
    type: f8
  - id: z_scale
    type: f8
  - id: x_offset
    type: f8
  - id: y_offset
    type: f8
  - id: z_offset
    type: f8
  - id: max_x
    type: f8
  - id: min_x
    type: f8
  - id: max_y
    type: f8
  - id: min_y
    type: f8
  - id: max_z
    type: f8
  - id: min_z
    type: f8
  - id: vlrs
    type: vlr
    repeat: expr
    repeat-expr: number_of_vlrs
  - id: points
    type:
      switch-on: point_data_format_id
      cases:
        0: point_format0
        1: point_format1
        2: point_format2
        3: point_format3
    repeat: expr
    repeat-expr: number_of_point_records
types:
  vlr:
    doc: |
      One Variable Length Record. `data` is retained byte-verbatim -- proprietary/unmodeled
      per-registered `(user_id, record_id)`, the recipe's typed raw-retention exception.
    seq:
      - id: reserved
        type: u2
      - id: user_id
        size: 16
        type: strz
        encoding: ASCII
      - id: record_id
        type: u2
      - id: record_length_after_header
        type: u2
      - id: description
        size: 32
        type: strz
        encoding: ASCII
      - id: data
        size: record_length_after_header
  point_common:
    seq:
      - id: x
        type: s4
      - id: y
        type: s4
      - id: z
        type: s4
      - id: intensity
        type: u2
      - id: return_flags
        type: u1
        doc: bits 0-2 return_number, 3-5 number_of_returns, 6 scan_direction_flag, 7 edge_of_flight_line
      - id: classification
        type: u1
      - id: scan_angle_rank
        type: s1
      - id: user_data
        type: u1
      - id: point_source_id
        type: u2
  point_format0:
    seq:
      - id: common
        type: point_common
  point_format1:
    seq:
      - id: common
        type: point_common
      - id: gps_time
        type: f8
  point_format2:
    seq:
      - id: common
        type: point_common
      - id: red
        type: u2
      - id: green
        type: u2
      - id: blue
        type: u2
  point_format3:
    seq:
      - id: common
        type: point_common
      - id: gps_time
        type: f8
      - id: red
        type: u2
      - id: green
        type: u2
      - id: blue
        type: u2
