//! ⚙️ Epw (energyplus) engine — 🚧 scaffolded by W1b: a REAL LOCATION header-line parser (all
//! 10 comma-separated fields per the EnergyPlus Weather File spec) + a real magic sniff (first
//! line keyword). The remaining 7 header lines + all 35 per-record columns land in W2/W3.

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EpwLocationSummary {
    pub city: String, pub state_province: String, pub country: String, pub source: String,
    pub wmo: String, pub latitude: f64, pub longitude: f64, pub timezone: f64, pub elevation: f64,
}

pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);
    text.trim_start().starts_with("LOCATION,")
}

/// 📐️ EPW LOCATION line: `LOCATION,City,StateProvince,Country,Source,WMO,Latitude,Longitude,
/// TimeZone,Elevation` — 10 fields.
pub fn parse_location_line(line: &str) -> Result<EpwLocationSummary, String> {
    let fields: Vec<&str> = line.trim_end().split(',').collect();
    if fields.len() < 10 || fields[0] != "LOCATION" {
        return Err(format!("epw: LOCATION line must have 10 fields, got {}", fields.len()));
    }
    let parse_f = |s: &str| s.parse::<f64>().map_err(|e| format!("epw: bad numeric field {s:?}: {e}"));
    Ok(EpwLocationSummary {
        city: fields[1].to_string(),
        state_province: fields[2].to_string(),
        country: fields[3].to_string(),
        source: fields[4].to_string(),
        wmo: fields[5].to_string(),
        latitude: parse_f(fields[6])?,
        longitude: parse_f(fields[7])?,
        timezone: parse_f(fields[8])?,
        elevation: parse_f(fields[9])?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniffs_and_parses_a_real_shaped_location_line() {
        let line = "LOCATION,Hannover,NI,DEU,SRC,100000,52.37,9.74,1.0,55.0";
        assert!(sniff_real_bytes(line.as_bytes()));
        let loc = parse_location_line(line).expect("parse");
        assert_eq!(loc.city, "Hannover");
        assert_eq!(loc.latitude, 52.37);
        assert_eq!(loc.elevation, 55.0);
    }

    #[test]
    fn rejects_a_short_location_line() {
        assert!(parse_location_line("LOCATION,Hannover").is_err());
    }
}

//#region 🔖️Register
/// 📌️ Registers this standard's single (✳️any) subset. Real magic-byte `sniff_real_bytes`/
/// `parse_minimal` above are used by the subset's analyzer; schema descriptor + document codec
/// registration is the subset composer's own job (see that module).
pub fn register() {
    crate::artifacts::epw::standards::energyplus::subsets::any::composer::register();
}
//#endregion 🔖️Register
