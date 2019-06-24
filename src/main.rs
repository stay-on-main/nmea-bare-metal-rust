mod nmea;

//use std::str;

fn main() {
    //println!("{}", ascii_char_to_u32("12345".as_bytes()).unwrap());
    let msg = nmea::parse_msg("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n".as_bytes());
    println!("{:#?}", msg);
    //println!("{}", str::from_utf8(msg).unwrap());
    //nmea_parse_msg("$GPRMC,092751.000,A,5321.6802,N,00630.3371,W,0.06,31.66,280511,,,A*45\r\n".as_bytes());
}
