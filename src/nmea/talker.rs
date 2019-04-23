pub const TAKLER_LEN:usize = 2;

#[derive(Debug)]
pub enum Talker {
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

pub fn get_talker(data: &[u8; TAKLER_LEN]) -> Option<Talker>
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