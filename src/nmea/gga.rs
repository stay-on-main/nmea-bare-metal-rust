/*
enum Fix
{
    0 = invalid
    1 = GPS fix (SPS)
    2 = DGPS fix
    3 = PPS fix
    4 = Real Time Kinematic
    5 = Float RTK
    6 = estimated (dead reckoning) (2.3 feature)
    7 = Manual input mode
    8 = Simulation mode
}
*/

#[derive(Debug)]
pub struct Gga {
    time: Option<u32>,
    lat: Option<f32>,
    lon: Option<f32>,
    fix_quality: Option<u32>,
    satellites: Option<u32>,
    hdop: Option<f32>,
    altitude: Option<f32>,
    geoid: Option<f32>,
}

fn ascii_to_f32(str: &[u8]) -> Option<f32>
{
    let mut num: u32 = 0;
    let mut frac: u32 = 0;
    let mut frac_flag = false;
    let mut frac_div: u32 = 1;

    for c in str {
        match c {
            b'0'...b'9' => {
                let n = (c - b'0') as u32;

                if frac_flag == false {
                    num = num * 10 + n;
                } else {
                    frac = frac * 10 + n;
                    frac_div *= 10;
                }
            },
            b'.' => {
                if frac_flag == true {
                    return None;
                }

                frac_flag = true;
            },
            _ => return None,
        }
    }

    Some((num as f32) + (frac as f32) / (frac_div as f32))
}

fn ascii_dec_to_u32(str: &[u8]) -> Option<u32>
{
    let mut num: u32 = 0;

     for c in str {
        match c {
            b'0'...b'9' => {
                num = num * 10 + (c - b'0') as u32;
            },
            _ => return None,
        }
    }

    Some(num)
}

impl Gga {
    pub fn new(msg: &[u8]) -> Option<Gga> {
        let mut gga = Gga {
            time: None,
            lat: None,
            lon: None,
            fix_quality: None,
            satellites: None,
            hdop: None,
            altitude: None,
            geoid: None,
        };

        let mut iter = msg.split(|num| *num == b',');

        match iter.next() {
            Some(time) => gga.time = ascii_dec_to_u32(time),
            _ => return None,
        }

        match iter.next() {
            Some(lat) => gga.lat = ascii_to_f32(lat),
            _ => return None,
        }

        let _lat_n = iter.next(); //skip N

        match iter.next() {
            Some(lon) =>  gga.lon = ascii_to_f32(lon),
            _ => return None,
        }

        let _lon_e = iter.next(); //skip E

        match iter.next() {
            Some(fix_quality) => gga.fix_quality = ascii_dec_to_u32(fix_quality),
            _ => return None,
        }

        match iter.next() {
            Some(satellites) => gga.satellites = ascii_dec_to_u32(satellites),
            _ => return None,
        }

        match iter.next() {
            Some(hdop) => gga.hdop = ascii_to_f32(hdop),
            _ => return None,
        }

        match iter.next() {
            Some(altitude) => gga.altitude = ascii_to_f32(altitude),
            _ => return None,
        }

        match iter.next() { // skip M
            Some(_) => {  },
            _ => println!("Must be 'M'"),
        }

        match iter.next() {
            Some(geoid) => gga.geoid = ascii_to_f32(geoid),
            _ => return None,
        }

        let _geoid_m = iter.next();
        Some(gga)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_to_f32_test() {
        assert_eq!(ascii_to_f32(b"34.056"), Some(34.056));
        assert_eq!(ascii_to_f32(b"50.7"), Some(50.7));
        assert_eq!(ascii_to_f32(b"50"), Some(50.0));
        assert_eq!(ascii_to_f32(b"5m0"), None);
        assert_eq!(ascii_to_f32(b"+7"), None);
        assert_eq!(ascii_to_f32(b"-7.43"), None);
        assert_eq!(ascii_to_f32(b"56.43.5"), None);
    }

    #[test]
    fn ascii_dec_to_u32_test() {
        assert_eq!(ascii_dec_to_u32(b"34"), Some(34));
        assert_eq!(ascii_dec_to_u32(b"50"), Some(50));
        assert_eq!(ascii_dec_to_u32(b"0"), Some(0));
        assert_eq!(ascii_dec_to_u32(b"5m0"), None);
        assert_eq!(ascii_dec_to_u32(b"7.0"), None);
        assert_eq!(ascii_dec_to_u32(b"56.43"), None);
        assert_eq!(ascii_dec_to_u32(b"56.43.5"), None);
    }
}