//! 🔢 Lossless scalar codecs for protocol payloads.

//#region 🔖️Scalars
/// @emoji 🧮️ Tagged, lossless-by-construction scalar codecs shared by `protocol_history`'s
/// payload codecs. `out`/`input` follow `crate::codec::ByteWriter`/`ByteReader` conventions exactly
/// (they take `&mut crate::codec::ByteWriter` / `&mut crate::codec::ByteReader<'_>` directly — no
/// reimplementation of the varint/byte primitives).
pub mod scalar {
    use crate::codec::{ByteReader, ByteWriter, PackError};

    //#region 🔖️Timestamp
    // Timestamp tag: 0 = raw string (len varint + utf8), 1 = epoch-ms varint (iff reprint is
    // byte-exact vs source), 2 = zigzag-varint delta-ms vs previous tag-1/2 timestamp in stream.
    //
    // 🎯️ Design choice (contract leaves the round-trip heuristic to the implementer): a timestamp
    // only ever gets tag 1/2 when `format_rfc3339_ms(parse_rfc3339_ms(raw).await).await == raw` byte-for-byte;
    // any input that isn't already exactly `YYYY-MM-DDTHH:MM:SS[.fff]Z` (fraction present iff its
    // ms component is nonzero) safely falls back to tag 0 raw text — correctness never depends on
    // the parser/formatter being calendar-complete, only on the equality check.

    /// @emoji ⏱️ Writes `raw` using the most compact of the three timestamp tags that reproduces
    /// it byte-exact. Returns `Some(epoch_ms)` iff tag 1/2 was written (thread this back in as
    /// `prev_epoch_ms` on the next call to keep deltas short); `None` iff tag 0 (raw) was written.
    pub async fn write_timestamp(out: &mut ByteWriter, raw: &str, prev_epoch_ms: Option<i64>) -> Option<i64> {
        let Some(epoch_ms) = round_trip_epoch_ms(raw).await else {
            write_raw_timestamp(out, raw).await;
            return None;
        };
        match prev_epoch_ms {
            Some(prev) => {
                out.write_u8(2).await;
                out.write_varint_i64(epoch_ms - prev).await;
                Some(epoch_ms)
            }
            None if epoch_ms >= 0 => {
                out.write_u8(1).await;
                out.write_varint_u64(epoch_ms as u64).await;
                Some(epoch_ms)
            }
            None => {
                // Tag 1 stores an unsigned varint; a pre-1970 timestamp with no prior delta base
                // has no lossless absolute encoding here, so it falls back to raw text.
                write_raw_timestamp(out, raw).await;
                None
            }
        }
    }

    /// @emoji ⏱️ Reads one tagged timestamp, returning the reconstructed string and, iff tag 1/2,
    /// the `epoch_ms` to feed back in as `prev_epoch_ms` for the next call.
    pub async fn read_timestamp(input: &mut ByteReader<'_>, prev_epoch_ms: Option<i64>) -> Result<(String, Option<i64>), PackError> {
        let tag = input.read_u8().await?;
        match tag {
            0 => {
                let len = input.read_varint_u64().await? as usize;
                let bytes = input.read_bytes(len).await?;
                Ok((utf8(bytes, input.position().await as u64).await?.to_string(), None))
            }
            1 => {
                let epoch_ms = input.read_varint_u64().await? as i64;
                Ok((format_rfc3339_ms(epoch_ms).await, Some(epoch_ms)))
            }
            2 => {
                let prev = match prev_epoch_ms {
                    Some(v) => v,
                    None => return Err(malformed("timestamp", input.position().await as u64, "tag 2 delta with no previous timestamp in scope").await),
                };
                let delta = input.read_varint_i64().await?;
                let epoch_ms = match prev.checked_add(delta) {
                    Some(v) => v,
                    None => return Err(malformed("timestamp", input.position().await as u64, "epoch_ms overflow").await),
                };
                Ok((format_rfc3339_ms(epoch_ms).await, Some(epoch_ms)))
            }
            other => Err(malformed("timestamp tag", input.position().await as u64, &format!("unknown timestamp tag {other:#x}")).await),
        }
    }

    async fn write_raw_timestamp(out: &mut ByteWriter, raw: &str) {
        out.write_u8(0).await;
        out.write_varint_u64(raw.len() as u64).await;
        out.write_bytes(raw.as_bytes()).await;
    }

    async fn round_trip_epoch_ms(raw: &str) -> Option<i64> {
        let epoch_ms = parse_rfc3339_ms(raw).await?;
        (format_rfc3339_ms(epoch_ms).await == raw).then_some(epoch_ms)
    }

    /// @emoji 📆️ Parses a UTC RFC-3339 timestamp (`Z` or numeric `±HH:MM` offset, optional
    /// fractional seconds truncated to milliseconds).await into milliseconds since the Unix epoch.
    /// `None` on any deviation from the grammar — callers fall back to raw text, never panic.
    async fn parse_rfc3339_ms(s: &str) -> Option<i64> {
        let bytes = s.as_bytes();
        if bytes.len() < 20 {
            return None;
        }
        let digit = |i: usize| -> Option<i64> {
            let b = *bytes.get(i)?;
            b.is_ascii_digit().then_some((b - b'0') as i64)
        };
        let two = |i: usize| -> Option<i64> { Some(digit(i)? * 10 + digit(i + 1)?) };
        let year = digit(0)? * 1000 + digit(1)? * 100 + digit(2)? * 10 + digit(3)?;
        if bytes[4] != b'-' {
            return None;
        }
        let month = two(5)?;
        if bytes[7] != b'-' {
            return None;
        }
        let day = two(8)?;
        if bytes[10] != b'T' {
            return None;
        }
        let hour = two(11)?;
        if bytes[13] != b':' {
            return None;
        }
        let minute = two(14)?;
        if bytes[16] != b':' {
            return None;
        }
        let second = two(17)?;
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
            return None;
        }
        let mut pos = 19usize;
        let mut ms = 0i64;
        if bytes.get(pos) == Some(&b'.') {
            pos += 1;
            let frac_start = pos;
            while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
                pos += 1;
            }
            if pos == frac_start {
                return None;
            }
            let frac = &s[frac_start..pos];
            let mut ms_digits = [b'0'; 3];
            for (i, slot) in ms_digits.iter_mut().enumerate() {
                if let Some(&b) = frac.as_bytes().get(i) {
                    *slot = b;
                }
            }
            ms = std::str::from_utf8(&ms_digits).ok()?.parse::<i64>().ok()?;
        }
        let offset_minutes = match bytes.get(pos) {
            Some(b'Z') | Some(b'z') => {
                pos += 1;
                0i64
            }
            Some(b'+') | Some(b'-') => {
                let sign = if bytes[pos] == b'+' { 1 } else { -1 };
                pos += 1;
                let oh = two(pos)?;
                pos += 2;
                if bytes.get(pos) != Some(&b':') {
                    return None;
                }
                pos += 1;
                let om = two(pos)?;
                pos += 2;
                sign * (oh * 60 + om)
            }
            _ => return None,
        };
        if pos != bytes.len() {
            return None;
        }
        let days = days_from_civil(year, month, day).await;
        let total_seconds = days * 86_400 + hour * 3_600 + minute * 60 + second - offset_minutes * 60;
        Some(total_seconds * 1_000 + ms)
    }

    /// @emoji 📆️ Canonical UTC formatter: `YYYY-MM-DDTHH:MM:SSZ`, with `.fffZ` appended iff the
    /// millisecond component is nonzero. The single source of truth for the tag-1/2 round trip.
    async fn format_rfc3339_ms(epoch_ms: i64) -> String {
        let days = epoch_ms.div_euclid(86_400_000);
        let rem_ms = epoch_ms.rem_euclid(86_400_000);
        let (year, month, day) = civil_from_days(days).await;
        let hour = rem_ms / 3_600_000;
        let rem = rem_ms % 3_600_000;
        let minute = rem / 60_000;
        let rem = rem % 60_000;
        let second = rem / 1_000;
        let ms = rem % 1_000;
        if ms == 0 {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        } else {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{ms:03}Z")
        }
    }

    /// @emoji 🌍️ Howard Hinnant's `days_from_civil`: proleptic-Gregorian date to days-since-epoch.
    /// <https://howardhinnant.github.io/date_algorithms.html>
    async fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = if m > 2 { m - 3 } else { m + 9 };
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// @emoji 🌍️ Inverse of `days_from_civil`.
    async fn civil_from_days(z: i64) -> (i64, i64, i64) {
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y, m, d)
    }
    //#endregion 🔖️Timestamp

    //#region 🔖️Id
    // Id tag: 0 = raw string, 1 = dictref (varint index into a string dictionary — caller supplies
    // the resolve/intern closures), 2 = prefix_dictref + [u8;16] (iff id is "<prefix>-<uuid>"),
    // 3 = edit-ordinal varint (only valid where the referent is a previously-seen edit).
    //
    // 🎯️ Design choice: `write_id` tries edit-ordinal first (cheapest — a single small varint,
    // when the id names an edit already in scope), then the prefix+uuid split (16 raw bytes beats
    // a 36-byte string even after dictionary interning), then falls back to a plain dictref. Tag 0
    // (raw) is never emitted by this writer — it exists purely so `read_id` stays forward-
    // compatible with ids written directly by another layer that chooses to skip interning.

    /// @emoji 🪪️ Writes `id` using the most compact of the four id tags that preserves it exactly.
    // ✏️ `intern`/`resolve` are `AsyncFn(Mut)` (not plain `Fn`): their real-world arguments are
    // `DictBuilder::intern`/`DictReader::resolve`, which are themselves `async fn` — see the
    // `kernel-finish` lease-request this packet granted (`📡️spr/📜️history` is the one caller).
    pub async fn write_id(out: &mut ByteWriter, id: &str, mut intern: impl AsyncFnMut(&str) -> u32, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<(), PackError> {
        if let Some(ordinal) = edit_ordinal_of(id) {
            out.write_u8(3).await;
            out.write_varint_u64(ordinal).await;
            return Ok(());
        }
        if let Some((prefix, uuid_bytes)) = split_prefix_uuid(id).await {
            out.write_u8(2).await;
            out.write_varint_u64(intern(prefix).await as u64).await;
            out.write_bytes(&uuid_bytes).await;
            return Ok(());
        }
        out.write_u8(1).await;
        out.write_varint_u64(intern(id).await as u64).await;
        Ok(())
    }

    /// @emoji 🪪️ Reads one tagged id, resolving dictrefs/ordinals through the supplied closures.
    pub async fn read_id<'r>(input: &mut ByteReader<'_>, resolve: impl AsyncFn(u32) -> Result<&'r str, PackError>, ordinal_to_id: impl Fn(u64) -> Result<&'r str, PackError>) -> Result<String, PackError> {
        let tag = input.read_u8().await?;
        match tag {
            0 => {
                let len = input.read_varint_u64().await? as usize;
                let bytes = input.read_bytes(len).await?;
                Ok(utf8(bytes, input.position().await as u64).await?.to_string())
            }
            1 => {
                let idx = input.read_varint_u64().await? as u32;
                Ok(resolve(idx).await?.to_string())
            }
            2 => {
                let idx = input.read_varint_u64().await? as u32;
                let prefix = resolve(idx).await?;
                let uuid_bytes = input.read_bytes(16).await?;
                let mut array = [0u8; 16];
                array.copy_from_slice(uuid_bytes);
                Ok(format!("{prefix}-{}", format_uuid(&array).await))
            }
            3 => {
                let ordinal = input.read_varint_u64().await?;
                Ok(ordinal_to_id(ordinal)?.to_string())
            }
            other => Err(malformed("id tag", input.position().await as u64, &format!("unknown id tag {other:#x}")).await),
        }
    }

    /// @emoji 🔪️ Splits `"<prefix>-<uuid>"` into `(prefix, 16 raw uuid bytes)`, requiring the
    /// trailing 36 bytes to be a canonical lowercase-hex-with-dashes UUID and a non-empty prefix
    /// — so the round trip through `format_uuid` reproduces the original text exactly.
    async fn split_prefix_uuid(id: &str) -> Option<(&str, [u8; 16])> {
        let len = id.len();
        if len < 38 {
            return None;
        }
        let uuid_start = len - 36;
        let dash_idx = len - 37;
        if !id.is_char_boundary(dash_idx) || !id.is_char_boundary(uuid_start) {
            return None;
        }
        if id.as_bytes()[dash_idx] != b'-' {
            return None;
        }
        let prefix = &id[..dash_idx];
        if prefix.is_empty() {
            return None;
        }
        let uuid_str = &id[uuid_start..];
        let uuid_bytes = uuid_str.as_bytes();
        if uuid_bytes[8] != b'-' || uuid_bytes[13] != b'-' || uuid_bytes[18] != b'-' || uuid_bytes[23] != b'-' {
            return None;
        }
        let hex_ranges = [0..8, 9..13, 14..18, 19..23, 24..36];
        let mut out = [0u8; 16];
        let mut out_pos = 0usize;
        for range in hex_ranges {
            let group = uuid_str.get(range)?.as_bytes();
            let mut gi = 0usize;
            while gi + 1 < group.len() + 1 && gi < group.len() {
                let hi = lower_hex_val(group[gi]).await?;
                let lo = lower_hex_val(group[gi + 1]).await?;
                out[out_pos] = (hi << 4) | lo;
                out_pos += 1;
                gi += 2;
            }
        }
        Some((prefix, out))
    }

    async fn lower_hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    }

    /// @emoji 🎨️ Formats 16 raw bytes as a canonical lowercase `8-4-4-4-12` UUID string.
    async fn format_uuid(bytes: &[u8; 16]) -> String {
        let mut s = String::with_capacity(36);
        for (i, b) in bytes.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                s.push('-');
            }
            s.push_str(&format!("{b:02x}"));
        }
        s
    }
    //#endregion 🔖️Id

    //#region 🔖️Shared
    async fn utf8(bytes: &[u8], offset: u64) -> Result<&str, PackError> {
        match std::str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(malformed("utf8", offset, "invalid utf-8").await),
        }
    }

    async fn malformed(what: &'static str, offset: u64, detail: &str) -> PackError {
        PackError::Malformed { what, offset, detail: detail.to_string() }
    }
    //#endregion 🔖️Shared

    // Id list: count varint, entries* (each via write_id/read_id).
    // Minimal-varint enforcement reuses crate::codec::is_minimal_varint at Full verification.
}
//#endregion 🔖️Scalars

