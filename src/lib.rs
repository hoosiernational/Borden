
fn into_sixes(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    input.chunks(3).for_each(|chunk| match chunk.len() {
        1 => {
            output.push(chunk[0] >> 2);
            output.push(chunk[0] << 4 & 0b00111111);
            output.push(255);
            output.push(255);
        }
        2 => {
            output.push(chunk[0] >> 2);
            output.push((chunk[0] << 4 & 0b00111111) | (chunk[1] >> 4));
            output.push(chunk[1] << 2 & 0b00111111);
            output.push(255);
        }
        3 => {
            output.push(chunk[0] >> 2);
            output.push((chunk[0] << 4 & 0b00111111) | chunk[1] >> 4);
            output.push((chunk[1] << 2 & 0b00111111) | (chunk[2] >> 6));
            output.push(chunk[2] & 0b00111111);
        }
        _ => {}
    });
    output
}

fn into_eights(input: &[u8]) -> Vec<u8> {
    if input.len() % 4 != 0 {
        Vec::new()
    } else {
        let mut output = Vec::new();
        input.chunks(4).for_each(|chunk| {
            output.push((chunk[0] << 2) | (chunk[1] >> 4));
            if chunk[2] != 255 {
                output.push((chunk[1] << 4) | (chunk[2] >> 2));
                if chunk[3] != 255 {
                    output.push((chunk[2] << 6) | chunk[3]);
                }
            }
        });
        output
    }
}

fn six_to_char(input: u8) -> u8 {
    match input {
        0 => 65,
        1 => 66,
        2 => 67,
        3 => 68,
        4 => 69,
        5 => 70,
        6 => 71,
        7 => 72,
        8 => 73,
        9 => 74,
        10 => 75,
        11 => 76,
        12 => 77,
        13 => 78,
        14 => 79,
        15 => 80,
        16 => 81,
        17 => 82,
        18 => 83,
        19 => 84,
        20 => 85,
        21 => 86,
        22 => 87,
        23 => 88,
        24 => 89,
        25 => 90,
        26 => 97,
        27 => 98,
        28 => 99,
        29 => 100,
        30 => 101,
        31 => 102,
        32 => 103,
        33 => 104,
        34 => 105,
        35 => 106,
        36 => 107,
        37 => 108,
        38 => 109,
        39 => 110,
        40 => 111,
        41 => 112,
        42 => 113,
        43 => 114,
        44 => 115,
        45 => 116,
        46 => 117,
        47 => 118,
        48 => 119,
        49 => 120,
        50 => 121,
        51 => 122,
        52 => 48,
        53 => 49,
        54 => 50,
        55 => 51,
        56 => 52,
        57 => 53,
        58 => 54,
        59 => 55,
        60 => 56,
        61 => 57,
        62 => 43,
        63 => 47,
        _ => 61,
    }
}

fn char_to_six(input: u8) -> u8 {
    match input {
        65 => 0,
        66 => 1,
        67 => 2,
        68 => 3,
        69 => 4,
        70 => 5,
        71 => 6,
        72 => 7,
        73 => 8,
        74 => 9,
        75 => 10,
        76 => 11,
        77 => 12,
        78 => 13,
        79 => 14,
        80 => 15,
        81 => 16,
        82 => 17,
        83 => 18,
        84 => 19,
        85 => 20,
        86 => 21,
        87 => 22,
        88 => 23,
        89 => 24,
        90 => 25,
        97 => 26,
        98 => 27,
        99 => 28,
        100 => 29,
        101 => 30,
        102 => 31,
        103 => 32,
        104 => 33,
        105 => 34,
        106 => 35,
        107 => 36,
        108 => 37,
        109 => 38,
        110 => 39,
        111 => 40,
        112 => 41,
        113 => 42,
        114 => 43,
        115 => 44,
        116 => 45,
        117 => 46,
        118 => 47,
        119 => 48,
        120 => 49,
        121 => 50,
        122 => 51,
        48 => 52,
        49 => 53,
        50 => 54,
        51 => 55,
        52 => 56,
        53 => 57,
        54 => 58,
        55 => 59,
        56 => 60,
        57 => 61,
        43 => 62,
        47 => 63,
        _ => 255,
    }
}

///Encodes an array of bytes into a Base64 string
pub fn encode(input: &[u8]) -> String {
    String::from_utf8(
        into_sixes(input)
            .into_iter()
            .map(|x| six_to_char(x))
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

///Decodes a Base64 string into an array of bytes
pub fn decode(input: &str) -> Vec<u8> {
    into_eights(
        &input
            .as_bytes()
            .into_iter()
            .map(|x| char_to_six(*x))
            .collect::<Vec<u8>>(),
    )
}
