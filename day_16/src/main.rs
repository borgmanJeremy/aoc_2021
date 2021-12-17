use nom::{
    bytes::complete::{is_a, tag, take},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeId {
    Literal,
    Operator,
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

fn main() {
    let raw_input: Vec<u8> = "D2FE28"
        .chars()
        .map(|c| char_to_bool_vec(c))
        .flatten()
        .collect();

    let (input, version) = get_version(&raw_input).unwrap();
    let (input, type_id) = is_literal(input).unwrap();
    match type_id {
        TypeId::Literal => {
            let (input, literal) = parse_literal(input);
            println!("literal: {}", literal);
        }
        TypeId::Operator => {
            todo!()
        }
    }
    println!(
        "version: {:?} type id: {:?}, rest: {:?} ",
        version, type_id, input
    );
}
