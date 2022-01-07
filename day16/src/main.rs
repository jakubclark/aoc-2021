fn convert<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

fn binary_str_to_number(s: &str) -> usize {
    let mut res = 0;
    for c in s.chars() {
        res <<= 1;
        res |= c.to_digit(10).unwrap();
    }
    res as usize
}

#[derive(Debug)]
enum LengthType {
    /// If the length type ID is 0,
    /// then the next 15 bits are a number that represents
    /// the total length in bits of the sub-packets contained
    /// by this packet.
    LengthInBits = 0,
    /// If the length type ID is 1, then the next 11 bits
    /// are a number that represents the number of sub-packets
    /// immediately contained by this packet.
    NumberOfSubPackets = 1,
}

impl From<char> for LengthType {
    fn from(s: char) -> Self {
        match s {
            '0' => LengthType::LengthInBits,
            '1' => LengthType::NumberOfSubPackets,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PacketType {
    LiteralValue,
    Operator(usize),
}

impl From<&str> for PacketType {
    fn from(s: &str) -> Self {
        let num = binary_str_to_number(s);
        match num {
            4 => PacketType::LiteralValue,
            n => PacketType::Operator(n),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    r#type: PacketType,
    value: usize,
    // The total number of bits, this packet takes up. May include sub-packets
    packet_size_bits: usize,
    sub_packets: Vec<Packet>,
    length_type_id: Option<LengthType>,
}

impl Packet {
    fn total_size(&self) -> usize {
        let mut sum = self.packet_size_bits;
        for packet in self.sub_packets.iter() {
            sum += packet.total_size();
        }
        sum
    }

    fn sum_versions(&self) -> usize {
        let mut sum = self.version;
        for packet in self.sub_packets.iter() {
            sum += packet.sum_versions();
        }
        sum
    }

    fn new(s: &str) -> Packet {
        let binary = s.chars().map(convert).collect::<Vec<_>>().join("");
        Packet::new_inner(binary.as_str())
    }

    fn new_inner(binary: &str) -> Packet {
        let version = binary_str_to_number(&binary[0..3]);
        let packet_type = PacketType::from(&binary[3..6]);
        let packet = match packet_type {
            PacketType::LiteralValue => Packet::create_literal_packet(version, binary),
            PacketType::Operator(_) => Packet::create_operator_packet(version, binary),
        };
        packet
    }

    fn create_literal_packet(version: usize, binary: &str) -> Packet {
        // First 3 bits are the version, the next 3 bits are the packet type (literal)
        let mut packet_size_bits = 6;
        let mut i = 6;
        let mut value = 0;

        loop {
            let slice = &binary[i..i + 5];
            value |= binary_str_to_number(&slice[1..]);

            packet_size_bits += slice.len();
            i += 5;

            if slice.chars().next() == Some('0') {
                // Last Group
                break;
            }
            value <<= 4;
        }

        Packet {
            version,
            r#type: PacketType::LiteralValue,
            value,
            packet_size_bits,
            sub_packets: Vec::new(),
            length_type_id: None,
        }
    }

    fn create_operator_packet(version: usize, binary: &str) -> Packet {
        // First 3 bits are the version, the next 3 bits are the packet type (operator)
        let mut packet_size_bits = 6;

        let length_type_id = LengthType::from(binary.chars().nth(6).unwrap());
        // 1 bit for the length type id
        packet_size_bits += 1;

        let mut sub_packets = Vec::new();

        match length_type_id {
            LengthType::LengthInBits => {
                packet_size_bits += 15;
                let number_of_bits = binary_str_to_number(&binary[7..22]);
                let mut i = 22;
                let mut count = 0;
                while count < number_of_bits {
                    let slice = &binary[i..];
                    let packet = Packet::new_inner(slice);
                    i += packet.total_size();
                    count += packet.total_size();
                    sub_packets.push(packet);
                }
            }
            LengthType::NumberOfSubPackets => {
                packet_size_bits += 11;
                let number_of_subpackets = binary_str_to_number(&binary[7..18]);
                let mut i = 18;
                for _ in 0..number_of_subpackets {
                    let slice = &binary[i..];
                    let packet = Packet::new_inner(slice);
                    i += packet.total_size();
                    sub_packets.push(packet);
                }
            }
        };

        Packet {
            version: version,
            r#type: PacketType::Operator(0),
            value: 0,
            packet_size_bits,
            sub_packets,
            length_type_id: Some(length_type_id),
        }
    }
}

fn parse_input() -> Packet {
    let input = include_str!("../input.txt");
    let packet = Packet::new(input);
    packet
}

fn solve_part1() -> usize {
    let packet = parse_input();
    packet.sum_versions()
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
}
