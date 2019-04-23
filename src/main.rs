mod nmea;


fn main() {
    //println!("{}", ascii_char_to_u32("12345".as_bytes()).unwrap());
    nmea::parse_msg("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n".as_bytes());
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
