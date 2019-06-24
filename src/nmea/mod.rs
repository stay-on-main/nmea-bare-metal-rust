mod talker;
mod sentence;
mod gga;

#[derive(Debug)]
pub enum NmeaMsg {
    MsgGga(gga::Gga),
}

fn ascii_hex_char_to_u32(data: &[u8]) -> Option<u32>
{
    if data.len() == 0 {
        return None;
    }

    let mut res: u32 = 0;

    for &n in data {
        let m = match n {
            b'0'...b'9' => n - b'0',
            b'A'...b'F' => n - b'A',
            b'a'...b'f' => n - b'a',
            _ => return None,
        };

        res = res * 16 + (m as u32);
    }
    return Some(res);
}

fn do_checksum(msg: &[u8]) -> Option<u8> {
    let mut checksum = 0u8;

    if msg.len() == 0 {
        return None;
    }

    for &c in msg {
        checksum ^= c;
    }

    return Some(checksum);
}

fn msg_skip_header(msg: &[u8]) -> Option<&[u8]> {
    if msg.len() == 0 {
        return None;
    }

    if (msg[0] == b'$') || (msg[0] == b'!') {

        return Some(&msg[1..]);
    }

    return None;
}

fn msg_skip_termination_chars(msg: &[u8]) -> Option<&[u8]> {
    if msg.len() < 2 {
        return None;
    }

    let termination_index = msg.len() - 2;

    if &msg[termination_index..] != b"\r\n" {
        return None;
    }

    return Some(&msg[..termination_index]);
}

fn msg_skip_checksum(msg: &[u8]) -> Option<(&[u8], u32)>
{
    if msg.len() < 3 || (msg[msg.len() - 3] != b'*') {
        return None;
    }

    match ascii_hex_char_to_u32(&msg[(msg.len() - 2)..]) {
        Some(checksum) => return Some((&msg[..(msg.len() - 3)], checksum)),
        None => return None,
    };
}

fn msg_skip_talker(msg: &[u8]) -> Option<(&[u8], talker::Talker)>
{
    if msg.len() < talker::TAKLER_LEN {
        return None;
    }

    match talker::get_talker(&[msg[0], msg[1]]) {
        Some(talker) => Some((&msg[talker::TAKLER_LEN..], talker)),
        None => return None,
    }
}

fn msg_skip_msg_type(msg: &[u8]) -> Option<(&[u8], sentence::SentenceType)>
{
    if msg.len() < sentence::MSG_TYPE_LEN {
        return None;
    }

    match sentence::get_msg_type(&[msg[0], msg[1], msg[2]]) {
        Some(msg_type) => Some((&msg[sentence::MSG_TYPE_LEN..], msg_type)),
        None => return None,
    }
}

pub fn parse_msg(msg: &[u8]) -> Option<NmeaMsg> {
    let msg = match msg_skip_header(msg) {
        Some(m) => m,
        None => return None,
    };

    let msg = match msg_skip_termination_chars(msg) {
        Some(m) => m,
        None => return None,
    };

    let (msg, checksum) = match msg_skip_checksum(msg) {
        Some((m, c)) => (m, c),
        None => return None,
    };

    let calc_checksum = match do_checksum(&msg) {
        Some(checksum) => checksum,
        None => return None,
    };

    if calc_checksum != (checksum as u8) {
        return None;
    }

    println!("crc form msg: {}, calculated crc: {}", checksum, calc_checksum);

    let (msg, talker) = match msg_skip_talker(&msg) {
        Some((m, t)) => (m, t),
        None => return None,
    };

    println!("{:#?}", talker);

    let (msg, msg_type) = match msg_skip_msg_type(&msg) {
        Some((m, t)) => (m, t),
        None => return None,
    };

    println!("{:#?}", msg_type);

    if msg.len() < 1 {
        return None;
    }

    if msg[0] != b',' {
        return None;
    }

    let msg = &msg[1..];

    match msg_type {
        sentence::SentenceType::GGA => {
            match gga::Gga::new(msg) {
                Some(msg) => return Some(NmeaMsg::MsgGga(msg)),
                None => return None,
            }
        },
        _ =>  {
            println!("not implemented");
            return None;
        }
    }
}