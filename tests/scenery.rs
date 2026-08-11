use std::path::PathBuf;

use rustgear::scenery::SceneryCatalog;

#[test]
fn load_apt_dat_finds_airports() {
    let mut catalog = SceneryCatalog::default();
    let path = PathBuf::from("/usr/share/games/flightgear/Airports/apt.dat.gz");
    catalog.load_apt_dat(&path);
    assert!(!catalog.airports.is_empty(), "apt.dat should load airports");
    assert!(catalog.airports.iter().any(|a| a.icao == "VHXX"));
}
