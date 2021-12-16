use std::fs::read_to_string;
use std::ops::Deref;
use itertools::Itertools;

struct Header {
    version: i32,
    type_id: i32,
}

impl Header {
    fn new(bitstream: &String) -> Header {
        let version = i32::from_str_radix(&bitstream[0..3], 2).unwrap();
        let type_id = i32::from_str_radix(&bitstream[3..6], 2).unwrap();
        Header {
            version,
            type_id,
        }
    }
    fn size(&self) -> usize {
        6
    }
}

trait SubPacket {
    fn size(&self) -> usize;
    fn list(&self) -> Vec<&Packet>;
    fn value(&self) -> u128;
}

struct LiteralValue {
    value: u128,
    size: usize,
}

impl LiteralValue {
    fn new(bitstream: &String) -> LiteralValue {
        let bs = bitstream.as_str();
        let mut data = "".to_string();
        let mut pos = 0;
        loop {
            let cont = bs.chars().nth(pos).unwrap();
            data += &bs[pos + 1..pos + 5];
            pos += 5;
            if cont == '0' {
                break;
            }
        }
        LiteralValue {
            value: u128::from_str_radix(data.as_str(), 2).unwrap(),
            size: pos,
        }
    }
}

impl SubPacket for LiteralValue {
    fn size(&self) -> usize {
        self.size
    }

    fn list(&self) -> Vec<&Packet> {
        vec![]
    }

    fn value(&self) -> u128 {
        self.value
    }
}

struct ListOfPackets {
    packets: Vec<Packet>,
    type_id: i32,
}

impl ListOfPackets {
    fn new(bitstream: &String, type_id: i32) -> ListOfPackets {
        let mut packets = vec![];
        let mut pos = 0;
        while !bitstream[pos..].chars().all(|c| c.eq_ignore_ascii_case(&'0')) {
            let p = Packet::new(&bitstream[pos..].to_string());
            pos += p.size();
            packets.push(p);
        }
        ListOfPackets {
            packets,
            type_id,
        }
    }
    fn new_fixed_count(bitstream: &String, n: usize, type_id: i32) -> ListOfPackets {
        let mut packets = vec![];
        let mut pos = 0;
        for _ in 0..n {
            let p = Packet::new(&bitstream[pos..].to_string());
            pos += p.size();
            packets.push(p);
        }
        ListOfPackets {
            packets,
            type_id,
        }
    }
}

impl SubPacket for ListOfPackets {
    fn size(&self) -> usize {
        self.packets.iter().map(|p| p.size()).sum()
    }
    fn list(&self) -> Vec<&Packet> {
        self.packets.iter().collect_vec()
    }

    fn value(&self) -> u128 {
        match self.type_id {
            0 => self.list().iter().map(|p| p.get_value()).sum(),
            1 => self.list().iter().map(|p| p.get_value()).fold(1, |a, b| a * b),
            2 => self.list().iter().map(|p| p.get_value()).min().unwrap(),
            3 => self.list().iter().map(|p| p.get_value()).max().unwrap(),
            5 => (self.list().first().unwrap().get_value() > self.list().iter().skip(1).next().unwrap().get_value()) as u128,
            6 => (self.list().first().unwrap().get_value() < self.list().iter().skip(1).next().unwrap().get_value()) as u128,
            7 => (self.list().first().unwrap().get_value() == self.list().iter().skip(1).next().unwrap().get_value()) as u128,
            _ => panic!()
        }
    }
}

struct Packet {
    header: Header,
    payload: Box<dyn SubPacket>,
    pos: usize,
}

impl Packet {
    fn new(bitstream: &String) -> Packet {
        let bs = bitstream.as_str();
        let mut pos = 0 as usize;
        let header = Header::new(bitstream);
        pos += header.size();
        let payload: Box<dyn SubPacket> = match header.type_id {
            4 => Box::new(LiteralValue::new(&bitstream[pos..].to_string())),
            x => {
                let length_type = bs.chars().nth(pos).unwrap();
                pos += 1;
                match length_type {
                    '0' => {
                        let total_size = usize::from_str_radix(&bs[pos..pos + 15], 2).unwrap();
                        pos += 15;
                        Box::new(ListOfPackets::new(&bs[pos..pos + total_size].to_string(), x))
                    }
                    '1' => {
                        let packet_num = usize::from_str_radix(&bs[pos..pos + 11], 2).unwrap();
                        pos += 11;
                        Box::new(ListOfPackets::new_fixed_count(&bs[pos..].to_string(), packet_num, x))
                    }
                    _ => panic!()
                }
            }
        };
        pos += payload.size();
        Packet {
            header,
            payload,
            pos,
        }
    }
    fn size(&self) -> usize {
        self.pos
    }
    fn get_value(&self) -> u128 {
        self.payload.value()
    }
}

fn part1(packet: &Packet) -> i32 {
    let s = packet.payload.deref().list()
        .iter()
        .map(|&p| part1(p))
        .sum::<i32>();
    s + packet.header.version
}

pub(crate) fn solve() {
    let hex = read_to_string("16.txt").unwrap();
    let bits = hex.chars()
        .map(|h| u16::from_str_radix(h.to_string().as_str(), 16).unwrap())
        .map(|x| format!("{:04b}", x))
        .join("");
    println!("{}", bits);
    let p = Packet::new(&bits);
    println!("{}", part1(&p));
    println!("{}", p.get_value());
}
