use std::str;

pub fn parse_gga(msg: &[u8])
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