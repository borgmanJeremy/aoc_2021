use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    error::Error,
    error::ErrorKind,
    Err, IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeId {
    Literal,
    Operator,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum LengthType {
    PacketLength,
    PacketCount,
}
fn char_to_bool_vec(input: char) -> Vec<u8> {
    match input {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("invalid char"),
    }
}

fn is_literal(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([1, 0, 0])(b)?;
    Ok((res, TypeId::Literal))
}

fn is_op_0(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([0, 0, 0])(b)?;
    Ok((res, TypeId::Operator))
}
fn is_op_1(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([0, 0, 1])(b)?;
    Ok((res, TypeId::Operator))
}
fn is_op_2(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([0, 1, 0])(b)?;
    Ok((res, TypeId::Operator))
}

fn is_op_3(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([0, 1, 1])(b)?;
    Ok((res, TypeId::Operator))
}
fn is_op_5(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([1, 0, 1])(b)?;
    Ok((res, TypeId::Operator))
}
fn is_op_6(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([1, 1, 0])(b)?;
    Ok((res, TypeId::Operator))
}
fn is_op_7(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = tag([1, 1, 1])(b)?;
    Ok((res, TypeId::Operator))
}

fn is_operator(b: &[u8]) -> IResult<&[u8], TypeId> {
    let (res, _) = alt((is_op_1, is_op_2, is_op_3, is_op_5, is_op_6, is_op_7))(b)?;
    Ok((res, TypeId::Operator))
}

fn subpacket_length(b: &[u8]) -> IResult<&[u8], LengthType> {
    let (b, is_eleven_bit) = take(1usize)(b)?;
    if is_eleven_bit == [1] {
        Ok((b, LengthType::PacketCount))
    } else {
        Ok((b, LengthType::PacketLength))
    }
}

fn take_as_u8(b: &[u8], n: usize) -> IResult<&[u8], usize> {
    let (b, count) = take(n)(b)?;

    let mut sum = 0;
    for (idx, bit) in count.iter().rev().enumerate() {
        sum += u32::pow(2 * (*bit as u32), idx as u32);
    }

    Ok((b, sum as usize))
}

fn is_last_literal(b: &[u8]) -> IResult<&[u8], bool> {
    let (b, is_last) = take(1usize)(b)?;
    if is_last == [0] {
        Ok((b, true))
    } else {
        Ok((b, false))
    }
}

fn literal_value(b: &[u8]) -> IResult<&[u8], u8> {
    let (b, val) = take(4usize)(b)?;
    Ok((b, val[0] * 8 + val[1] * 4 + val[2] * 2 + val[3]))
}

fn get_version(b: &[u8]) -> IResult<&[u8], u8> {
    let (b, ver) = take(3usize)(b)?;
    Ok((b, ver[0] * 4 + ver[1] * 2 + ver[2]))
}

fn parse_literal(input: &[u8]) -> (&[u8], u32) {
    let mut persistant_input = input;
    let mut val_list = Vec::new();

    loop {
        let (input, is_last) = is_last_literal(&persistant_input).unwrap();
        let (input, val) = literal_value(&input).unwrap();
        val_list.push(val);
        persistant_input = input;
        if is_last {
            break;
        }
    }
    let literal = val_list.iter().enumerate().fold(0, |acc, (idx, val)| {
        if idx != val_list.len() - 1 {
            let exp = u32::pow(2, (val_list.len() - idx) as u32);
            acc + ((*val as u32) << exp)
        } else {
            let exp = 0;
            acc + ((*val as u32) << exp)
        }
    });

    (persistant_input, literal)
}

fn parse_packet(input: &[u8]) -> (&[u8], usize) {
    let starting_size = input.len();
    let (input, version) = get_version(&input).unwrap();
    let (input, type_id) = alt((is_literal, is_operator))(input).unwrap();
    match type_id {
        TypeId::Literal => {
            let (input, literal) = parse_literal(input);
            let ending_size = input.len();
            let bits_parsed = starting_size - ending_size;
            println!("literal: {}", literal);
            return (input, bits_parsed);
        }
        TypeId::Operator => {
            let (input, subpacket_type) = subpacket_length(input).unwrap();
            match subpacket_type {
                LengthType::PacketLength => {
                    let (input, subpacket_len) = take_as_u8(input, 15).unwrap();
                    println!("subpacket len: {}", subpacket_len);
                    let mut total_parsed = 0;
                    let mut persistent_input = input;
                    loop {
                        let res = parse_packet(&persistent_input);
                        total_parsed += res.1;
                        persistent_input = res.0;
                        if total_parsed >= subpacket_len {
                            break;
                        }
                    }
                    let ending_size = input.len();
                    let bits_parsed = starting_size - ending_size;
                    return (input, bits_parsed);
                }
                LengthType::PacketCount => {
                    let (input, subpacket_len) = take_as_u8(input, 11).unwrap();
                    println!("subpacket len: {}", subpacket_len);
                    parse_packet(&input);
                    let ending_size = input.len();
                    let bits_parsed = starting_size - ending_size;
                    return (input, bits_parsed);
                }
            }
        }
    }
}

fn main() {
    let input: Vec<u8> = "38006F45291200"
        .chars()
        .map(|c| char_to_bool_vec(c))
        .flatten()
        .collect();

    parse_packet(&input);
}
