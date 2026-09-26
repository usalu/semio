//! 🍎️ macOS Foundation-backed temporal presentation.

use super::{invalid_text, relative_amount, source_epoch_ms, PlatformTemporalAdapter, PlatformTemporalBatch};
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;
use ui_contract::{HostHourCycleV1, HostTemporalFormatRequestV1, HostTemporalFormatV1};

#[cfg(test)]
#[path = "../🌐️icu/🦀️.rs"]
mod icu_compile_test;

type CfRef = *const c_void;
type CfStringRef = *const c_void;
type CfIndex = isize;
type ObjcId = *mut c_void;
type ObjcSel = *mut c_void;

const UTF8: u32 = 0x0800_0100;
const CF_UNIX_EPOCH_DELTA: f64 = 978_307_200.0;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFLocaleCopyCurrent() -> CfRef;
    fn CFLocaleCreate(allocator: CfRef, identifier: CfStringRef) -> CfRef;
    fn CFLocaleGetIdentifier(locale: CfRef) -> CfStringRef;
    fn CFTimeZoneCopySystem() -> CfRef;
    fn CFTimeZoneCreateWithName(allocator: CfRef, name: CfStringRef, try_abbreviation: u8) -> CfRef;
    fn CFTimeZoneGetName(zone: CfRef) -> CfStringRef;
    fn CFStringCreateWithCString(allocator: CfRef, text: *const c_char, encoding: u32) -> CfStringRef;
    fn CFStringGetLength(text: CfStringRef) -> CfIndex;
    fn CFStringGetMaximumSizeForEncoding(length: CfIndex, encoding: u32) -> CfIndex;
    fn CFStringGetCString(text: CfStringRef, buffer: *mut c_char, capacity: CfIndex, encoding: u32) -> bool;
    fn CFDateCreate(allocator: CfRef, absolute_time: f64) -> CfRef;
    fn CFDateFormatterCreate(allocator: CfRef, locale: CfRef, date_style: CfIndex, time_style: CfIndex) -> CfRef;
    fn CFDateFormatterCreateDateFormatFromTemplate(allocator: CfRef, template: CfStringRef, options: usize, locale: CfRef) -> CfStringRef;
    fn CFDateFormatterSetFormat(formatter: CfRef, format: CfStringRef);
    fn CFDateFormatterSetProperty(formatter: CfRef, key: CfStringRef, value: CfRef);
    fn CFDateFormatterCreateStringWithDate(allocator: CfRef, formatter: CfRef, date: CfRef) -> CfStringRef;
    fn CFRelease(value: CfRef);
    static kCFDateFormatterTimeZone: CfStringRef;
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {}

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> ObjcId;
    fn sel_registerName(name: *const c_char) -> ObjcSel;
    fn objc_msgSend();
}

struct OwnedCf(CfRef);

impl OwnedCf {
    fn new(value: CfRef, fault: &str) -> Result<Self, String> {
        (!value.is_null()).then_some(Self(value)).ok_or_else(|| fault.to_string())
    }
}

impl Drop for OwnedCf {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) };
    }
}

struct MacProfile {
    locale: OwnedCf,
    time_zone: OwnedCf,
    locale_id: String,
    time_zone_id: String,
    hour_cycle: HostHourCycleV1,
}

fn cf_text(value: CfStringRef) -> Result<String, String> {
    if value.is_null() {
        return Err("host-temporal-format.macos-string-missing".to_string());
    }
    let capacity = unsafe { CFStringGetMaximumSizeForEncoding(CFStringGetLength(value), UTF8) }.checked_add(1).ok_or_else(|| "host-temporal-format.macos-string-capacity".to_string())?;
    let mut buffer = vec![0u8; usize::try_from(capacity).map_err(|_| "host-temporal-format.macos-string-capacity".to_string())?];
    if !unsafe { CFStringGetCString(value, buffer.as_mut_ptr().cast(), capacity, UTF8) } {
        return Err("host-temporal-format.macos-string-encoding".to_string());
    }
    Ok(unsafe { CStr::from_ptr(buffer.as_ptr().cast()) }.to_string_lossy().into_owned())
}

fn cf_owned_text(value: CfStringRef) -> Result<String, String> {
    let value = OwnedCf::new(value, "host-temporal-format.macos-string-create")?;
    cf_text(value.0)
}

fn cf_string(value: &str) -> Result<OwnedCf, String> {
    let value = CString::new(value).map_err(|_| "host-temporal-format.macos-string-nul".to_string())?;
    OwnedCf::new(unsafe { CFStringCreateWithCString(ptr::null(), value.as_ptr(), UTF8) }, "host-temporal-format.macos-string-create")
}

fn localized_pattern(locale: CfRef, skeleton: &str) -> Result<OwnedCf, String> {
    let skeleton = cf_string(skeleton)?;
    OwnedCf::new(unsafe { CFDateFormatterCreateDateFormatFromTemplate(ptr::null(), skeleton.0, 0, locale) }, "host-temporal-format.macos-pattern")
}

fn hour_cycle(pattern: &str) -> Result<HostHourCycleV1, String> {
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
    Err("host-temporal-format.macos-hour-cycle".to_string())
}

fn finish_profile(locale: OwnedCf, time_zone: OwnedCf) -> Result<MacProfile, String> {
    let locale_id = cf_text(unsafe { CFLocaleGetIdentifier(locale.0) })?.split('@').next().unwrap_or_default().replace('_', "-");
    let time_zone_id = cf_text(unsafe { CFTimeZoneGetName(time_zone.0) })?;
    let cycle_pattern = localized_pattern(locale.0, "j")?;
    let hour_cycle = hour_cycle(&cf_text(cycle_pattern.0)?)?;
    Ok(MacProfile { locale, time_zone, locale_id, time_zone_id, hour_cycle })
}

fn resolve_profile() -> Result<MacProfile, String> {
    let locale = OwnedCf::new(unsafe { CFLocaleCopyCurrent() }, "host-temporal-format.macos-locale")?;
    let time_zone = OwnedCf::new(unsafe { CFTimeZoneCopySystem() }, "host-temporal-format.macos-time-zone")?;
    finish_profile(locale, time_zone)
}

#[cfg(test)]
fn resolve_injected_profile(locale: &str, time_zone: &str) -> Result<MacProfile, String> {
    let locale_name = cf_string(locale)?;
    let time_zone_name = cf_string(time_zone)?;
    let locale = OwnedCf::new(unsafe { CFLocaleCreate(ptr::null(), locale_name.0) }, "host-temporal-format.macos-injected-locale")?;
    let time_zone = OwnedCf::new(unsafe { CFTimeZoneCreateWithName(ptr::null(), time_zone_name.0, 0) }, "host-temporal-format.macos-injected-time-zone")?;
    finish_profile(locale, time_zone)
}

fn absolute_text(profile: &MacProfile, timestamp_ms: i64, format: HostTemporalFormatV1) -> Result<String, String> {
    let skeleton = match (format, profile.hour_cycle) {
        (HostTemporalFormatV1::Time, HostHourCycleV1::H11) => "KKmmssa",
        (HostTemporalFormatV1::Time, HostHourCycleV1::H12) => "hhmmssa",
        (HostTemporalFormatV1::Time, HostHourCycleV1::H23) => "HHmmss",
        (HostTemporalFormatV1::Time, HostHourCycleV1::H24) => "kkmmss",
        (HostTemporalFormatV1::Date, _) => "yyyyMMdd",
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H11) => "yyyyMMddKKmma",
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H12) => "yyyyMMddhhmma",
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H23) => "yyyyMMddHHmm",
        (HostTemporalFormatV1::DateTime, HostHourCycleV1::H24) => "yyyyMMddkkmm",
        (HostTemporalFormatV1::Relative, _) => return Err("host-temporal-format.macos-relative-routed-as-absolute".to_string()),
    };
    let pattern = localized_pattern(profile.locale.0, skeleton)?;
    let formatter = OwnedCf::new(unsafe { CFDateFormatterCreate(ptr::null(), profile.locale.0, 0, 0) }, "host-temporal-format.macos-formatter")?;
    unsafe {
        CFDateFormatterSetFormat(formatter.0, pattern.0);
        CFDateFormatterSetProperty(formatter.0, kCFDateFormatterTimeZone, profile.time_zone.0);
    }
    let date = OwnedCf::new(unsafe { CFDateCreate(ptr::null(), timestamp_ms as f64 / 1_000.0 - CF_UNIX_EPOCH_DELTA) }, "host-temporal-format.macos-date")?;
    cf_owned_text(unsafe { CFDateFormatterCreateStringWithDate(ptr::null(), formatter.0, date.0) })
}

fn selector(name: &'static [u8]) -> ObjcSel {
    unsafe { sel_registerName(name.as_ptr().cast()) }
}

fn class(name: &'static [u8]) -> ObjcId {
    unsafe { objc_getClass(name.as_ptr().cast()) }
}

fn send_id(receiver: ObjcId, selector: ObjcSel) -> ObjcId {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel) -> ObjcId = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector) }
}

fn send_id_id(receiver: ObjcId, selector: ObjcSel, value: ObjcId) -> ObjcId {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel, ObjcId) -> ObjcId = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector, value) }
}

fn send_void_id(receiver: ObjcId, selector: ObjcSel, value: ObjcId) {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel, ObjcId) = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector, value) };
}

fn send_void_integer(receiver: ObjcId, selector: ObjcSel, value: isize) {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel, isize) = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector, value) };
}

fn send_void_integer_unit(receiver: ObjcId, selector: ObjcSel, value: isize, unit: usize) {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel, isize, usize) = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector, value, unit) };
}

fn send_c_string(receiver: ObjcId, selector: ObjcSel) -> *const c_char {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel) -> *const c_char = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector) }
}

fn send_void(receiver: ObjcId, selector: ObjcSel) {
    let call: unsafe extern "C" fn(ObjcId, ObjcSel) = unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { call(receiver, selector) };
}

struct OwnedObjc {
    raw: ObjcId,
    finish: ObjcSel,
}

impl OwnedObjc {
    fn new(raw: ObjcId, finish: ObjcSel) -> Result<Self, String> {
        (!raw.is_null()).then_some(Self { raw, finish }).ok_or_else(|| "host-temporal-format.macos-relative-formatter".to_string())
    }
}

impl Drop for OwnedObjc {
    fn drop(&mut self) {
        send_void(self.raw, self.finish);
    }
}

fn relative_text(profile: &MacProfile, timestamp_ms: i64, now_ms: i64) -> Result<String, String> {
    let (amount, unit) = relative_amount(timestamp_ms, now_ms);
    let unit = match unit {
        "year" => 1 << 2,
        "month" => 1 << 3,
        "day" => 1 << 4,
        "hour" => 1 << 5,
        "minute" => 1 << 6,
        "second" => 1 << 7,
        _ => return Err("host-temporal-format.macos-relative-unit".to_string()),
    };
    {
        let pool = OwnedObjc::new(send_id(class(b"NSAutoreleasePool\0"), selector(b"new\0")), selector(b"drain\0"))?;
        let formatter = OwnedObjc::new(send_id(send_id(class(b"NSRelativeDateTimeFormatter\0"), selector(b"alloc\0")), selector(b"init\0")), selector(b"release\0"))?;
        let components = OwnedObjc::new(send_id(send_id(class(b"NSDateComponents\0"), selector(b"alloc\0")), selector(b"init\0")), selector(b"release\0"))?;
        send_void_id(formatter.raw, selector(b"setLocale:\0"), profile.locale.0.cast_mut());
        send_void_integer(formatter.raw, selector(b"setDateTimeStyle:\0"), 0);
        send_void_integer(formatter.raw, selector(b"setUnitsStyle:\0"), 0);
        send_void_integer_unit(components.raw, selector(b"setValue:forComponent:\0"), amount as isize, unit);
        let output = send_id_id(formatter.raw, selector(b"localizedStringFromDateComponents:\0"), components.raw);
        let utf8 = if output.is_null() { ptr::null() } else { send_c_string(output, selector(b"UTF8String\0")) };
        let text = (!utf8.is_null()).then(|| unsafe { CStr::from_ptr(utf8) }.to_string_lossy().into_owned());
        drop(components);
        drop(formatter);
        drop(pool);
        text.ok_or_else(|| "host-temporal-format.macos-relative-output".to_string())
    }
}

fn format_profile(profile: MacProfile, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
    let labels = request
        .values
        .iter()
        .map(|value| match source_epoch_ms(&value.source) {
            None => Ok(invalid_text(&value.source)),
            Some(timestamp_ms) if value.format == HostTemporalFormatV1::Relative => relative_text(&profile, timestamp_ms, request.now_ms),
            Some(timestamp_ms) => absolute_text(&profile, timestamp_ms, value.format),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PlatformTemporalBatch { locale: profile.locale_id, time_zone: profile.time_zone_id, hour_cycle: profile.hour_cycle, labels })
}

pub(super) struct SystemPlatformAdapter;

impl PlatformTemporalAdapter for SystemPlatformAdapter {
    fn format(&self, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
        format_profile(resolve_profile()?, request)
    }
}

#[cfg(test)]
pub(super) struct InjectedPlatformAdapter<'a> {
    pub locale: &'a str,
    pub time_zone: &'a str,
}

#[cfg(test)]
impl PlatformTemporalAdapter for InjectedPlatformAdapter<'_> {
    fn format(&self, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
        format_profile(resolve_injected_profile(self.locale, self.time_zone)?, request)
    }
}
