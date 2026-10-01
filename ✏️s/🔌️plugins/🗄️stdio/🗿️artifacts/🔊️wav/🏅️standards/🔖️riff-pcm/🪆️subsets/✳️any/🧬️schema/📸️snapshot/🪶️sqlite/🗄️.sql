CREATE TABLE wav_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL
);
CREATE TABLE wav_format (
 id INTEGER PRIMARY KEY REFERENCES wav_document(id) CHECK (id = 1),
 audio_format INTEGER NOT NULL CHECK (audio_format BETWEEN 0 AND 65535),
 channels INTEGER NOT NULL CHECK (channels BETWEEN 0 AND 65535),
 sample_rate INTEGER NOT NULL CHECK (sample_rate BETWEEN 0 AND 4294967295),
 byte_rate INTEGER NOT NULL CHECK (byte_rate BETWEEN 0 AND 4294967295),
 block_align INTEGER NOT NULL CHECK (block_align BETWEEN 0 AND 65535),
 bits_per_sample INTEGER NOT NULL CHECK (bits_per_sample BETWEEN 0 AND 65535),
 pad_byte INTEGER NOT NULL CHECK (pad_byte BETWEEN 0 AND 255)
);
CREATE TABLE wav_format_extension (
 id INTEGER PRIMARY KEY REFERENCES wav_format(id) CHECK (id = 1)
);
CREATE TABLE wav_format_extension_byte (
 id INTEGER PRIMARY KEY, extension_id INTEGER NOT NULL REFERENCES wav_format_extension(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
CREATE TABLE wav_data (
 id INTEGER PRIMARY KEY REFERENCES wav_document(id) CHECK (id = 1),
 sample_kind TEXT NOT NULL CHECK (sample_kind IN ('pcm16','pcm8','float32','raw')),
 pad_byte INTEGER NOT NULL CHECK (pad_byte BETWEEN 0 AND 255)
);
CREATE TABLE wav_pcm16_sample (
 id INTEGER PRIMARY KEY, data_id INTEGER NOT NULL REFERENCES wav_data(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), sample INTEGER NOT NULL CHECK (sample BETWEEN -32768 AND 32767)
);
CREATE TABLE wav_pcm8_sample (
 id INTEGER PRIMARY KEY, data_id INTEGER NOT NULL REFERENCES wav_data(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), sample INTEGER NOT NULL CHECK (sample BETWEEN 0 AND 255)
);
CREATE TABLE wav_float32_sample (
 id INTEGER PRIMARY KEY, data_id INTEGER NOT NULL REFERENCES wav_data(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 ieee754_binary32_bits INTEGER NOT NULL CHECK (ieee754_binary32_bits BETWEEN 0 AND 4294967295),
 numeric_class TEXT NOT NULL CHECK (numeric_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')),
 sample REAL,
 CHECK ((numeric_class IN ('finite','negative_zero') AND sample IS NOT NULL) OR (numeric_class IN ('nan','positive_infinity','negative_infinity') AND sample IS NULL))
);
CREATE TABLE wav_raw_data_byte (
 id INTEGER PRIMARY KEY, data_id INTEGER NOT NULL REFERENCES wav_data(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
CREATE TABLE wav_other_chunk (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wav_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 fourcc TEXT NOT NULL, pad_byte INTEGER NOT NULL CHECK (pad_byte BETWEEN 0 AND 255)
);
CREATE TABLE wav_other_chunk_byte (
 id INTEGER PRIMARY KEY, chunk_id INTEGER NOT NULL REFERENCES wav_other_chunk(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
CREATE TABLE wav_chunk_order (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wav_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 chunk_kind TEXT NOT NULL CHECK (chunk_kind IN ('format','samples','other')),
 format_id INTEGER REFERENCES wav_format(id), data_id INTEGER REFERENCES wav_data(id), other_chunk_id INTEGER REFERENCES wav_other_chunk(id),
 auxiliary_chunk_index TEXT CHECK (auxiliary_chunk_index IS NULL OR (length(auxiliary_chunk_index) BETWEEN 1 AND 20 AND auxiliary_chunk_index NOT GLOB '*[^0-9]*' AND (auxiliary_chunk_index = '0' OR substr(auxiliary_chunk_index,1,1) <> '0') AND (length(auxiliary_chunk_index) < 20 OR auxiliary_chunk_index <= '18446744073709551615'))),
 CHECK ((chunk_kind = 'format' AND format_id IS NOT NULL AND data_id IS NULL AND other_chunk_id IS NULL AND auxiliary_chunk_index IS NULL) OR
        (chunk_kind = 'samples' AND format_id IS NULL AND data_id IS NOT NULL AND other_chunk_id IS NULL AND auxiliary_chunk_index IS NULL) OR
        (chunk_kind = 'other' AND format_id IS NULL AND data_id IS NULL AND auxiliary_chunk_index IS NOT NULL))
);
