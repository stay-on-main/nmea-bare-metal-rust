pub const MSG_TYPE_LEN:usize = 3;


#[derive(Debug)]
pub enum SentenceType {
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

pub fn get_msg_type(data: &[u8; MSG_TYPE_LEN]) -> Option<SentenceType>
{
    match data {
        b"AAM" => Some(SentenceType::AAM),
        b"ALM" => Some(SentenceType::ALM),
        b"APA" => Some(SentenceType::APA),
        b"APB" => Some(SentenceType::APB),
        b"BOD" => Some(SentenceType::BOD),
        b"BWC" => Some(SentenceType::BWC),
        b"DTM" => Some(SentenceType::DTM),
        b"GGA" => Some(SentenceType::GGA),
        b"GLL" => Some(SentenceType::GLL),
        b"GRS" => Some(SentenceType::GRS),
        b"GSA" => Some(SentenceType::GSA),
        b"GST" => Some(SentenceType::GST),
        b"GSV" => Some(SentenceType::GSV),
        b"MSK" => Some(SentenceType::MSK),
        b"MSS" => Some(SentenceType::MSS),
        b"RMA" => Some(SentenceType::RMA),
        b"RMB" => Some(SentenceType::RMB),
        b"RMC" => Some(SentenceType::RMC),
        b"RTE" => Some(SentenceType::RTE),
        b"TRF" => Some(SentenceType::TRF),
        b"STN" => Some(SentenceType::STN),
        b"VBW" => Some(SentenceType::VBW),
        b"VTG" => Some(SentenceType::VTG),
        b"WCV" => Some(SentenceType::WCV),
        b"WPL" => Some(SentenceType::WPL),
        b"XTC" => Some(SentenceType::XTC),
        b"XTE" => Some(SentenceType::XTE),
        b"ZTG" => Some(SentenceType::ZTG),
        b"ZDA" => Some(SentenceType::ZDA),
        _ => None,
    }
}