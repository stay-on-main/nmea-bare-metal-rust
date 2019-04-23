const MSG_TYPE_LEN:usize = 3;
const TAKLER_LEN:usize = 2;

#[derive(Debug)]
enum Talker {
    GL, // GLONASS Receiver
    GP, // Global Positioning System (GPS)
    AG, // Heading Track Controller (Autopilot): General
    AP, // Heading Track Controller (Autopilot): Magnetic
    AI, // Automatic Identification System
    CD, // Digital Selective Calling (DSC)
    CR, // Data Receiver
    CS, // Satellite
    CT, // Radio-Telephone (MF/HF)
    CV, // Radio-Telephone (VHF)
    CX, // Scanning Receiver
    DE, // DECCA Navigator
    DF, // Direction Finder
    EC, // Electronic Chart System (ECS)
    EI, // Electronic Chart Display & Information System (ECDIS)
    EP, // Emergency Position Indicating Beacon (EPIRB)
    ER, // Engine room Monitoring Systems
    GN, // Global Navigation Satellite System (GNSS)
    HC, // HEADING SENSORS: Compass, Magnetic
    HE, // Gyro, North Seeking
    HN, // Gyro, Non-North Seeking
    II, // Integrated Instrumentation
    IN, // Integrated Navigation
    LC, // Loran C
    RA, // Radar and/or Radar Plotting
    SD, // Sounder, depth
    SN, // Electronic Positioning System, other/general
    SS, // Sounder, scanning
    TI, // Turn Rate Indicator
    VD, // VELOCITY SENSORS: Doppler, other/general
    VM, // Speed Log, Water, Magnetic
    VW, // Speed Log, Water, Mechanical
    VR, // Voyage Data Recorder
    YX, // Transducer
    ZA, // TIMEKEEPERS, TIME/DATE: Atomic Clock
    ZC, // Chronometer
    ZQ, // Quartz
    ZV, // Radio Update
    WI, // Weather Instruments
}

fn nmea_get_talker(data: &[u8; TAKLER_LEN]) -> Option<Talker>
{
    match data {
        b"GL" => Some(Talker::GL),
        b"GP" => Some(Talker::GP),
        b"AG" => Some(Talker::AG),
        b"AP" => Some(Talker::AP),
        b"AI" => Some(Talker::AI),
        b"CD" => Some(Talker::CD),
        b"CR" => Some(Talker::CR),
        b"CS" => Some(Talker::CS),
        b"CT" => Some(Talker::CT),
        b"CV" => Some(Talker::CV),
        b"CX" => Some(Talker::CX),
        b"DE" => Some(Talker::DE),
        b"DF" => Some(Talker::DF),
        b"EC" => Some(Talker::EC),
        b"EI" => Some(Talker::EI),
        b"EP" => Some(Talker::EP),
        b"ER" => Some(Talker::ER),
        b"GN" => Some(Talker::GN),
        b"HC" => Some(Talker::HC),
        b"HE" => Some(Talker::HE),
        b"HN" => Some(Talker::HN),
        b"II" => Some(Talker::II),
        b"IN" => Some(Talker::IN),
        b"LC" => Some(Talker::LC),
        b"RA" => Some(Talker::RA),
        b"SD" => Some(Talker::SD),
        b"SN" => Some(Talker::SN),
        b"SS" => Some(Talker::SS),
        b"TI" => Some(Talker::TI),
        b"VD" => Some(Talker::VD),
        b"VM" => Some(Talker::VM),
        b"VW" => Some(Talker::VW),
        b"VR" => Some(Talker::VR),
        b"YX" => Some(Talker::YX),
        b"ZA" => Some(Talker::ZA),
        b"ZC" => Some(Talker::ZC),
        b"ZQ" => Some(Talker::ZQ),
        b"ZV" => Some(Talker::ZV),
        b"WI" => Some(Talker::WI),
        _ => None,
    }
}

#[derive(Debug)]
enum Msg {
    AAM, // Waypoint Arrival Alarm
    ALM, // Almanac data
    APA, // Auto Pilot A sentence
    APB, // Auto Pilot B sentence
    BOD, // Bearing Origin to Destination
    BWC, // Bearing using Great Circle route
    DTM, // Datum being used.
    GGA, // Fix information
    GLL, // Lat/Lon data
    GRS, // GPS Range Residuals
    GSA, // Overall Satellite data
    GST, // GPS Pseudorange Noise Statistics
    GSV, // Detailed Satellite data
    MSK, // send control for a beacon receiver
    MSS, // Beacon receiver status information.
    RMA, // recommended Loran data
    RMB, // recommended navigation data for gps
    RMC, // recommended minimum data for gps
    RTE, // route message
    TRF, // Transit Fix Data
    STN, // Multiple Data ID
    VBW, // dual Ground / Water Spped
    VTG, // Vector track an Speed over the Ground
    WCV, // Waypoint closure velocity (Velocity Made Good)
    WPL, // Waypoint Location information
    XTC, // cross track error
    XTE, // measured cross track error
    ZTG, // Zulu (UTC) time and time to go (to destination)
    ZDA, // Date and Time
}

fn nmea_get_msg_type(data: &[u8; MSG_TYPE_LEN]) -> Option<Msg>
{
    match data {
        b"AAM" => Some(Msg::AAM),
        b"ALM" => Some(Msg::ALM),
        b"APA" => Some(Msg::APA),
        b"APB" => Some(Msg::APB),
        b"BOD" => Some(Msg::BOD),
        b"BWC" => Some(Msg::BWC),
        b"DTM" => Some(Msg::DTM),
        b"GGA" => Some(Msg::GGA),
        b"GLL" => Some(Msg::GLL),
        b"GRS" => Some(Msg::GRS),
        b"GSA" => Some(Msg::GSA),
        b"GST" => Some(Msg::GST),
        b"GSV" => Some(Msg::GSV),
        b"MSK" => Some(Msg::MSK),
        b"MSS" => Some(Msg::MSS),
        b"RMA" => Some(Msg::RMA),
        b"RMB" => Some(Msg::RMB),
        b"RMC" => Some(Msg::RMC),
        b"RTE" => Some(Msg::RTE),
        b"TRF" => Some(Msg::TRF),
        b"STN" => Some(Msg::STN),
        b"VBW" => Some(Msg::VBW),
        b"VTG" => Some(Msg::VTG),
        b"WCV" => Some(Msg::WCV),
        b"WPL" => Some(Msg::WPL),
        b"XTC" => Some(Msg::XTC),
        b"XTE" => Some(Msg::XTE),
        b"ZTG" => Some(Msg::ZTG),
        b"ZDA" => Some(Msg::ZDA),
        _ => None,
    }
}


/*
fn ascii_dec_char_to_u32(data: &[u8]) -> Option<u32> 
{
    if data.len() == 0 {
        return None;
    }

    let mut res: u32 = 0;

    for &n in data {
        if (n < '0' as u8) || (n > '9' as u8) {
            return None;
        }

        res = res * 10 + (n as u32) - ('0' as u32);
    }
    return Some(res);
}
*/
fn ascii_hex_char_to_u32(data: &[u8]) -> Option<u32>
{
    if data.len() == 0 {
        return None;
    }

    let mut res: u32 = 0;

    for &n in data {
        let m = match n {
            b'0'...b'9' => n - b'0', // '0'..'9'
            b'A'...b'F' => n - b'A', // 'A'..'F'
            b'a'...b'f' => n - b'a', // 'a'..'f'
            _ => return None,
        };

        res = res * 16 + (m as u32);
    }
    return Some(res);
}

use std::str;

fn nmea_checksum(msg: &[u8]) -> Option<u8> {
    let mut checksum = 0u8;

    if msg.len() == 0 {
        return None;
    }

    for &c in msg {
        checksum ^= c;
    }

    return Some(checksum);
}

fn nmea_msg_skip_header(msg: &[u8]) -> Option<&[u8]> {
    if msg.len() == 0 {
        return None;
    }

    if (msg[0] == b'$') || (msg[0] == b'!') {

        return Some(&msg[1..]);
    }

    return None;
}

fn nmea_msg_skip_termination_chars(msg: &[u8]) -> Option<&[u8]> {
    if msg.len() < 2 {
        return None;
    }

    let termination_index = msg.len() - 2;

    if &msg[termination_index..] != b"\r\n" {
        return None;
    }

    return Some(&msg[..termination_index]);
}

fn nmea_msg_skip_checksum(msg: &[u8]) -> Option<(&[u8], u32)>
{
    if msg.len() < 3 || (msg[msg.len() - 3] != b'*') {
        return None;
    }

    match ascii_hex_char_to_u32(&msg[(msg.len() - 2)..]) {
        Some(checksum) => return Some((&msg[..(msg.len() - 3)], checksum)),
        None => return None,
    };
}

fn nmea_msg_skip_talker(msg: &[u8]) -> Option<(&[u8], Talker)>
{
    if msg.len() < TAKLER_LEN {
        return None;
    }

    match nmea_get_talker(&[msg[0], msg[1]]) {
        Some(talker) => Some((&msg[TAKLER_LEN..], talker)),
        None => return None,
    }
}

fn nmea_msg_skip_msg_type(msg: &[u8]) -> Option<(&[u8], Msg)>
{
    if msg.len() < MSG_TYPE_LEN {
        return None;
    }

    match nmea_get_msg_type(&[msg[0], msg[1], msg[2]]) {
        Some(msg_type) => Some((&msg[MSG_TYPE_LEN..], msg_type)),
        None => return None,
    }
}
/*
struct Splitter<'a> {
    data: &'a[u8],
    delim: u8,
}

impl <'a> Splitter<'a> {
    fn new(data: &'a[u8], delim: u8) -> My {
        My { data: data, delim: delim}
    }

    fn next(&mut self) -> Option<&[u8]> {
        for i in 0..self.data.len() {
            if self.data[i] == self.delim {
                let res = Some(&self.data[0..i]);
                self.data = &self.data[(i + 1)..];
                return res;
            }
        }

        return None;
    }
}
*/
fn parse_gga(msg: &[u8])
{
    let mut iter = msg.split(|num| *num == b',');

    let time = iter.next();

    match time {
        Some(t) => println!("time: {}", str::from_utf8(t).unwrap()),
        _ => println!("time is invalid"),
    }

    let lat = iter.next();

    match lat {
        Some(l) => println!("lat: {}", str::from_utf8(l).unwrap()),
        _ => println!("lat isn't available "),
    }

    let _lat_n = iter.next(); //skip N
    let lon = iter.next();

    match lon {
        Some(l) => println!("lon: {}", str::from_utf8(l).unwrap()),
        _ => println!("lon isn't available "),
    }

    let _lon_e = iter.next(); //skip E

    let fix_quality= iter.next();

    match fix_quality {
        Some(f) => println!("fix quality: {}", str::from_utf8(f).unwrap()),
        _ => println!("fix quality isn't available "),
    }

    let num_of_satellites= iter.next();

    match num_of_satellites {
        Some(n) => println!("number of satellites: {}", str::from_utf8(n).unwrap()),
        _ => println!("number of satellites isn't available "),
    }

    let hdop = iter.next();

    match hdop {
        Some(h) => println!("hdop: {}", str::from_utf8(h).unwrap()),
        _ => println!("hdop isn't available "),
    }

    let altitude = iter.next();

    match altitude {
        Some(a) => println!("altitude: {}", str::from_utf8(a).unwrap()),
        _ => println!("altitude isn't available "),
    }

    let _altitude_m = iter.next(); // skip M

    let geoid = iter.next();

    match geoid {
        Some(g) => println!("geoid: {}", str::from_utf8(g).unwrap()),
        _ => println!("geoid isn't available "),
    }

    let _geoid_m = iter.next(); // skip M
    //println!("{}", str::from_utf8(iter.next().unwrap()).unwrap());
}

fn nmea_parse_msg(msg: &[u8]) -> bool {
    let msg = match nmea_msg_skip_header(msg) {
        Some(m) => m,
        None => return false,
    };

    let msg = match nmea_msg_skip_termination_chars(msg) {
        Some(m) => m,
        None => return false,
    };

    let (msg, checksum) = match nmea_msg_skip_checksum(msg) {
        Some((m, c)) => (m, c),
        None => return false,
    };

    let calc_checksum = match nmea_checksum(&msg) {
        Some(checksum) => checksum,
        None => return false,
    };

    if calc_checksum != (checksum as u8) {
        return false;
    }

    println!("crc form msg: {}, calculated crc: {}", checksum, calc_checksum);

    let (msg, talker) = match nmea_msg_skip_talker(&msg) {
        Some((m, t)) => (m, t),
        None => return false,
    };

    println!("{:#?}", talker);

    let (msg, msg_type) = match nmea_msg_skip_msg_type(&msg) {
        Some((m, t)) => (m, t),
        None => return false,
    };

    println!("{:#?}", msg_type);

    if msg.len() < 1 {
        return false;
    }

    if msg[0] != b',' {
        return false;
    }

    let msg = &msg[1..];

    match msg_type {
        Msg::GGA => parse_gga(msg),
        _ => println!("not implemented"),
    }

    println!("{}", str::from_utf8(msg).unwrap());
    return true;
}

fn main() {
    //println!("{}", ascii_char_to_u32("12345".as_bytes()).unwrap());
    nmea_parse_msg("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n".as_bytes());
    //nmea_parse_msg("$GPRMC,092751.000,A,5321.6802,N,00630.3371,W,0.06,31.66,280511,,,A*45\r\n".as_bytes());
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(ascii_char_to_u32("12345".as_bytes()), Some(12345u32));
        assert_eq!(ascii_char_to_u32("5".as_bytes()), Some(5u32));
        assert_eq!(ascii_char_to_u32("".as_bytes()), None);
        assert_eq!(ascii_char_to_u32("6dx.".as_bytes()), None);
    }
}
