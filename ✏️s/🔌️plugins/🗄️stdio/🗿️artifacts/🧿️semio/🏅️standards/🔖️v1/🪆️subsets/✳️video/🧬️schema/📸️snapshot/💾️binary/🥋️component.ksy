meta:
  id: semio_video_snapshot
  endian: le
doc: |
  stdio.semio.video snapshot binary envelope: magic + version + length-prefixed JSON body. The
  JSON body's own structure is normatively described by the sibling 🔣️component.json
  (SemioVideoSnapshot: schema, streams[]{kind,codec,width,height,rate,samples}).
seq:
  - id: magic
    contents: "stdio.semio.video"
  - id: version
    type: u1
  - id: body_len
    type: u4
  - id: body_bytes
    size: body_len
    doc: UTF-8 JSON, see ../🔣️component.json
