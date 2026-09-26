//! 🌐️ System-ICU temporal adapter shared by Linux and Windows loaders.

use super::super::{invalid_text, relative_amount, source_epoch_ms, PlatformTemporalBatch};
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;
use ui_contract::{HostHourCycleV1, HostTemporalFormatRequestV1, HostTemporalFormatV1};

const U_BUFFER_OVERFLOW_ERROR: UErrorCode = 15;
const UDAT_PATTERN: i32 = -2;

pub(super) trait IcuLibrary {
    fn symbol(&self, name: &str) -> Option<*mut c_void>;
}

type UErrorCode = i32;
type OpenPatternGenerator = unsafe extern "C" fn(*const c_char, *mut UErrorCode) -> *mut c_void;
type ClosePatternGenerator = unsafe extern "C" fn(*mut c_void);
type BestPattern = unsafe extern "C" fn(*const c_void, *const u16, i32, *mut u16, i32, *mut UErrorCode) -> i32;
type OpenDateFormatter = unsafe extern "C" fn(i32, i32, *const c_char, *const u16, i32, *const u16, i32, *mut UErrorCode) -> *mut c_void;
type CloseDateFormatter = unsafe extern "C" fn(*mut c_void);
type FormatDate = unsafe extern "C" fn(*const c_void, f64, *mut u16, i32, *mut c_void, *mut UErrorCode) -> i32;
type DefaultTimeZone = unsafe extern "C" fn(*mut u16, i32, *mut UErrorCode) -> i32;
type DefaultLocale = unsafe extern "C" fn() -> *const c_char;
type ToLanguageTag = unsafe extern "C" fn(*const c_char, *mut c_char, i32, i8, *mut UErrorCode) -> i32;
type OpenRelativeFormatter = unsafe extern "C" fn(*const c_char, *mut c_void, i32, i32, *mut UErrorCode) -> *mut c_void;
type CloseRelativeFormatter = unsafe extern "C" fn(*mut c_void);
type FormatRelative = unsafe extern "C" fn(*const c_void, f64, i32, *mut u16, i32, *mut UErrorCode) -> i32;

pub(super) struct IcuApi {
    open_pattern_generator: OpenPatternGenerator,
    close_pattern_generator: ClosePatternGenerator,
    best_pattern: BestPattern,
    open_date_formatter: OpenDateFormatter,
    close_date_formatter: CloseDateFormatter,
    format_date: FormatDate,
    default_time_zone: DefaultTimeZone,
    default_locale: DefaultLocale,
    to_language_tag: ToLanguageTag,
    open_relative_formatter: OpenRelativeFormatter,
    close_relative_formatter: CloseRelativeFormatter,
    format_relative: FormatRelative,
}

unsafe impl Send for IcuApi {}
unsafe impl Sync for IcuApi {}

impl IcuApi {
    pub(super) fn load(library: &dyn IcuLibrary) -> Result<Self, String> {
        macro_rules! symbol {
            ($name:literal, $kind:ty) => {{
                let pointer = library.symbol($name).ok_or_else(|| concat!("host-temporal-format.icu-symbol-", $name).to_string())?;
                unsafe { std::mem::transmute::<*mut c_void, $kind>(pointer) }
            }};
        }
        Ok(Self {
            open_pattern_generator: symbol!("udatpg_open", OpenPatternGenerator),
            close_pattern_generator: symbol!("udatpg_close", ClosePatternGenerator),
            best_pattern: symbol!("udatpg_getBestPattern", BestPattern),
            open_date_formatter: symbol!("udat_open", OpenDateFormatter),
            close_date_formatter: symbol!("udat_close", CloseDateFormatter),
            format_date: symbol!("udat_format", FormatDate),
            default_time_zone: symbol!("ucal_getDefaultTimeZone", DefaultTimeZone),
            default_locale: symbol!("uloc_getDefault", DefaultLocale),
            to_language_tag: symbol!("uloc_toLanguageTag", ToLanguageTag),
            open_relative_formatter: symbol!("ureldatefmt_open", OpenRelativeFormatter),
            close_relative_formatter: symbol!("ureldatefmt_close", CloseRelativeFormatter),
            format_relative: symbol!("ureldatefmt_formatNumeric", FormatRelative),
        })
    }
}

fn success(status: UErrorCode) -> Result<(), String> {
    (status <= 0).then_some(()).ok_or_else(|| format!("host-temporal-format.icu-{status}"))
}

fn utf16(value: &str) -> Vec<u16> {
    value.encode_utf16().collect()
}

fn utf16_text(buffer: &[u16], length: i32) -> Result<String, String> {
    let length = usize::try_from(length).map_err(|_| "host-temporal-format.icu-output-length".to_string())?;
    String::from_utf16(buffer.get(..length).ok_or_else(|| "host-temporal-format.icu-output-capacity".to_string())?).map_err(|_| "host-temporal-format.icu-output-utf16".to_string())
}

fn locale(api: &IcuApi) -> Result<(CString, String), String> {
    let raw = unsafe { (api.default_locale)() };
    if raw.is_null() {
        return Err("host-temporal-format.icu-locale".to_string());
    }
    let raw = unsafe { CStr::from_ptr(raw) };
    let locale = CString::new(raw.to_bytes()).map_err(|_| "host-temporal-format.icu-locale-nul".to_string())?;
    let mut output = vec![0i8; 64];
    let length = loop {
        let mut status = 0;
        let length = unsafe { (api.to_language_tag)(locale.as_ptr(), output.as_mut_ptr(), output.len() as i32, 0, &mut status) };
        if status == U_BUFFER_OVERFLOW_ERROR {
            output.resize(usize::try_from(length).map_err(|_| "host-temporal-format.icu-locale-length".to_string())?.saturating_add(1), 0);
            continue;
        }
        success(status)?;
        break usize::try_from(length).map_err(|_| "host-temporal-format.icu-locale-length".to_string())?;
    };
    let bytes = output.get(..length).ok_or_else(|| "host-temporal-format.icu-locale-capacity".to_string())?.iter().map(|byte| *byte as u8).collect::<Vec<_>>();
    let language_tag = String::from_utf8(bytes).map_err(|_| "host-temporal-format.icu-locale-utf8".to_string())?;
    (!language_tag.is_empty()).then_some((locale, language_tag)).ok_or_else(|| "host-temporal-format.icu-locale-empty".to_string())
}

fn time_zone(api: &IcuApi) -> Result<(Vec<u16>, String), String> {
    let mut output = vec![0u16; 64];
    let length = loop {
        let mut status = 0;
        let length = unsafe { (api.default_time_zone)(output.as_mut_ptr(), output.len() as i32, &mut status) };
        if status == U_BUFFER_OVERFLOW_ERROR {
            output.resize(usize::try_from(length).map_err(|_| "host-temporal-format.icu-time-zone-length".to_string())?.saturating_add(1), 0);
            continue;
        }
        success(status)?;
        break length;
    };
    let name = utf16_text(&output, length)?;
    let length = usize::try_from(length).map_err(|_| "host-temporal-format.icu-time-zone-length".to_string())?;
    (!name.is_empty()).then_some((output[..length].to_vec(), name)).ok_or_else(|| "host-temporal-format.icu-time-zone-empty".to_string())
}

struct PatternGenerator<'a> {
    api: &'a IcuApi,
    raw: *mut c_void,
}

impl Drop for PatternGenerator<'_> {
    fn drop(&mut self) {
        unsafe { (self.api.close_pattern_generator)(self.raw) };
    }
}

fn pattern_generator<'a>(api: &'a IcuApi, locale: &CString) -> Result<PatternGenerator<'a>, String> {
    let mut status = 0;
    let raw = unsafe { (api.open_pattern_generator)(locale.as_ptr(), &mut status) };
    success(status)?;
    (!raw.is_null()).then_some(PatternGenerator { api, raw }).ok_or_else(|| "host-temporal-format.icu-pattern-generator".to_string())
}

fn hour_cycle(api: &IcuApi, generator: &PatternGenerator<'_>) -> Result<HostHourCycleV1, String> {
    let pattern = String::from_utf16(&best_pattern(api, generator, "j")?).map_err(|_| "host-temporal-format.icu-hour-cycle-utf16".to_string())?;
    let mut quoted = false;
    for character in pattern.chars() {
        if character == '\'' {
            quoted = !quoted;
        } else if !quoted {
            match character {
                'K' => return Ok(HostHourCycleV1::H11),
                'h' => return Ok(HostHourCycleV1::H12),
                'H' => return Ok(HostHourCycleV1::H23),
                'k' => return Ok(HostHourCycleV1::H24),
                _ => {}
            }
        }
    }
    Err("host-temporal-format.icu-hour-cycle".to_string())
}

fn skeleton(format: HostTemporalFormatV1, cycle: HostHourCycleV1) -> Result<&'static str, String> {
    match (format, cycle) {
        (HostTemporalFormatV1::Time, HostHourCycleV1::H11) => Ok("KKmmssa"),
        (HostTemporalFormatV1::Time, HostHourCycleV1::H12) => Ok("hhmmssa"),
        (HostTemporalFormatV1::Time, HostHourCycleV1::H23) => Ok("HHmmss"),
        (HostTemporalFormatV1::Time, HostHourCycleV1::H24) => Ok("kkmmss"),
        (HostTemporalFormatV1::Date, _) => Ok("yyyyMMdd"),
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H11) => Ok("yyyyMMddKKmma"),
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H12) => Ok("yyyyMMddhhmma"),
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H23) => Ok("yyyyMMddHHmm"),
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H24) => Ok("yyyyMMddkkmm"),
        (HostTemporalFormatV1::Relative, _) => Err("host-temporal-format.icu-relative-skeleton".to_string()),
    }
}

fn best_pattern(api: &IcuApi, generator: &PatternGenerator<'_>, skeleton: &str) -> Result<Vec<u16>, String> {
    let skeleton = utf16(skeleton);
    let mut output = vec![0u16; 64];
    let length = loop {
        let mut status = 0;
        let length = unsafe { (api.best_pattern)(generator.raw, skeleton.as_ptr(), skeleton.len() as i32, output.as_mut_ptr(), output.len() as i32, &mut status) };
        if status == U_BUFFER_OVERFLOW_ERROR {
            output.resize(usize::try_from(length).map_err(|_| "host-temporal-format.icu-pattern-length".to_string())?.saturating_add(1), 0);
            continue;
        }
        success(status)?;
        break usize::try_from(length).map_err(|_| "host-temporal-format.icu-pattern-length".to_string())?;
    };
    Ok(output.get(..length).ok_or_else(|| "host-temporal-format.icu-pattern-capacity".to_string())?.to_vec())
}

struct DateFormatter<'a> {
    api: &'a IcuApi,
    raw: *mut c_void,
}

impl Drop for DateFormatter<'_> {
    fn drop(&mut self) {
        unsafe { (self.api.close_date_formatter)(self.raw) };
    }
}

fn date_formatter<'a>(api: &'a IcuApi, locale: &CString, time_zone: &[u16], pattern: &[u16]) -> Result<DateFormatter<'a>, String> {
    let mut status = 0;
    let raw = unsafe { (api.open_date_formatter)(UDAT_PATTERN, UDAT_PATTERN, locale.as_ptr(), time_zone.as_ptr(), time_zone.len() as i32, pattern.as_ptr(), pattern.len() as i32, &mut status) };
    success(status)?;
    (!raw.is_null()).then_some(DateFormatter { api, raw }).ok_or_else(|| "host-temporal-format.icu-date-formatter".to_string())
}

fn absolute_text(api: &IcuApi, generator: &PatternGenerator<'_>, locale: &CString, time_zone: &[u16], cycle: HostHourCycleV1, timestamp_ms: i64, format: HostTemporalFormatV1) -> Result<String, String> {
    let pattern = best_pattern(api, generator, skeleton(format, cycle)?)?;
    let formatter = date_formatter(api, locale, time_zone, &pattern)?;
    let mut output = vec![0u16; 128];
    loop {
        let mut status = 0;
        let length = unsafe { (api.format_date)(formatter.raw, timestamp_ms as f64, output.as_mut_ptr(), output.len() as i32, ptr::null_mut(), &mut status) };
        if status == U_BUFFER_OVERFLOW_ERROR {
            output.resize(usize::try_from(length).map_err(|_| "host-temporal-format.icu-date-length".to_string())?.saturating_add(1), 0);
            continue;
        }
        success(status)?;
        return utf16_text(&output, length);
    }
}

struct RelativeFormatter<'a> {
    api: &'a IcuApi,
    raw: *mut c_void,
}

impl Drop for RelativeFormatter<'_> {
    fn drop(&mut self) {
        unsafe { (self.api.close_relative_formatter)(self.raw) };
    }
}

fn relative_formatter<'a>(api: &'a IcuApi, locale: &CString) -> Result<RelativeFormatter<'a>, String> {
    let mut status = 0;
    let raw = unsafe { (api.open_relative_formatter)(locale.as_ptr(), ptr::null_mut(), 0, 0x100, &mut status) };
    success(status)?;
    (!raw.is_null()).then_some(RelativeFormatter { api, raw }).ok_or_else(|| "host-temporal-format.icu-relative-formatter".to_string())
}

fn relative_text(api: &IcuApi, formatter: &RelativeFormatter<'_>, timestamp_ms: i64, now_ms: i64) -> Result<String, String> {
    let (amount, unit) = relative_amount(timestamp_ms, now_ms);
    let unit = match unit {
        "year" => 0,
        "month" => 2,
        "day" => 4,
        "hour" => 5,
        "minute" => 6,
        "second" => 7,
        _ => return Err("host-temporal-format.icu-relative-unit".to_string()),
    };
    let mut output = vec![0u16; 128];
    loop {
        let mut status = 0;
        let length = unsafe { (api.format_relative)(formatter.raw, amount as f64, unit, output.as_mut_ptr(), output.len() as i32, &mut status) };
        if status == U_BUFFER_OVERFLOW_ERROR {
            output.resize(usize::try_from(length).map_err(|_| "host-temporal-format.icu-relative-length".to_string())?.saturating_add(1), 0);
            continue;
        }
        success(status)?;
        return utf16_text(&output, length);
    }
}

pub(super) fn format(api: &IcuApi, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
    let (locale, language_tag) = locale(api)?;
    let (time_zone, time_zone_name) = time_zone(api)?;
    let generator = pattern_generator(api, &locale)?;
    let cycle = hour_cycle(api, &generator)?;
    let relative = request.values.iter().any(|value| value.format == HostTemporalFormatV1::Relative).then(|| relative_formatter(api, &locale)).transpose()?;
    let labels = request
        .values
        .iter()
        .map(|value| match source_epoch_ms(&value.source) {
            None => Ok(invalid_text(&value.source)),
            Some(timestamp_ms) if value.format == HostTemporalFormatV1::Relative => relative_text(api, relative.as_ref().expect("relative request owns formatter"), timestamp_ms, request.now_ms),
            Some(timestamp_ms) => absolute_text(api, &generator, &locale, &time_zone, cycle, timestamp_ms, value.format),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PlatformTemporalBatch { locale: language_tag, time_zone: time_zone_name, hour_cycle: cycle, labels })
}
