const MIN_CHR: usize = 39;
const MAX_CHR: usize = 122;

static CHRS_BY_CHR_ID: [u8; 32] = [
    b'e', b'a', b'i', b'o', b't', b'h', b'n', b'r', b's', b'l', b'u', b'c', b'w', b'm', b'd', b'b', b'p', b'f', b'g', b'v', b'y', b'k', b'-', b'H', b'M', b'T', b'\'', b'B', b'x', b'I', b'W', b'L'
];


static CHR_IDS_BY_CHR: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, -1, -1, -1, -1, -1, 22, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 27, -1, -1, -1, -1, -1, 23, 29, -1, -1, 31, 24, -1, -1, -1, -1, -1, -1, 25, -1, -1, 30, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 15, 11, 14, 0, 17, 18, 5, 2, -1, 21, 9, 13, 6, 3, 16, -1, 7, 8, 4, 10, 19, 12, 28, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

static SUCCESSOR_IDS_BY_CHR_ID_AND_CHR_ID: [[i8; 32]; 32] = [
    [7, 4, 12, -1, 6, -1, 1, 0, 3, 5, -1, 9, -1, 8, 2, -1, 15, 14, -1, 10, 11, -1, -1, -1, -1, -1, -1, -1, 13, -1, -1, -1],
    [-1, -1, 6, -1, 1, -1, 0, 3, 2, 4, 15, 11, -1, 9, 5, 10, 13, -1, 12, 8, 7, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [9, 11, -1, 4, 2, -1, 0, 8, 1, 5, -1, 6, -1, 3, 7, 15, -1, 12, 10, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, 14, 7, 5, -1, 1, 2, 8, 9, 0, 15, 6, 4, 11, -1, 12, 3, -1, 10, -1, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [2, 4, 3, 1, 5, 0, -1, 6, 10, 9, 7, 12, 11, -1, -1, -1, -1, 13, -1, -1, 8, -1, 15, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [0, 1, 2, 3, 4, -1, -1, 5, 9, 10, 6, -1, -1, 8, 15, 11, -1, 14, -1, -1, 7, -1, 13, -1, -1, -1, 12, -1, -1, -1, -1, -1],
    [2, 8, 7, 4, 3, -1, 9, -1, 6, 11, -1, 5, -1, -1, 0, -1, -1, 14, 1, 15, 10, 12, -1, -1, -1, -1, 13, -1, -1, -1, -1, -1],
    [0, 3, 1, 2, 6, -1, 9, 8, 4, 12, 13, 10, -1, 11, 7, -1, -1, 15, 14, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 6, 3, 4, 1, 2, -1, -1, 5, 10, 7, 9, 11, 12, -1, -1, 8, 14, -1, -1, 15, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 6, 2, 5, 9, -1, -1, -1, 10, 1, 8, -1, 12, 14, 4, -1, 15, 7, -1, 13, 3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [8, 10, 9, 15, 1, -1, 4, 0, 3, 2, -1, 6, -1, 12, 11, 13, 7, 14, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [1, 3, 6, 0, 4, 2, -1, 7, 13, 8, 9, 11, -1, -1, 15, -1, -1, -1, -1, -1, 10, 5, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [3, 0, 1, 4, -1, 2, 5, 6, 7, 8, -1, 14, -1, -1, 9, 15, -1, 12, -1, -1, -1, 10, 11, -1, -1, -1, 13, -1, -1, -1, -1, -1],
    [0, 1, 3, 2, 15, -1, 12, -1, 7, 14, 4, -1, -1, 9, -1, 8, 5, 10, -1, -1, 6, -1, 13, -1, -1, -1, 11, -1, -1, -1, -1, -1],
    [0, 3, 1, 2, -1, -1, 12, 6, 4, 9, 7, -1, -1, 14, 8, -1, -1, 15, 11, 13, 5, -1, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 5, 7, 2, 10, 13, -1, 6, 8, 1, 3, -1, -1, 14, 15, 11, -1, -1, -1, 12, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 2, 6, 3, 7, 10, -1, 1, 9, 4, 8, -1, -1, 15, -1, 12, 5, -1, -1, -1, 11, -1, 13, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [1, 3, 4, 0, 7, -1, 12, 2, 11, 8, 6, 13, -1, -1, -1, -1, -1, 5, -1, -1, 10, 15, 9, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [1, 3, 5, 2, 13, 0, 9, 4, 7, 6, 8, -1, -1, 15, -1, 11, -1, -1, 10, -1, 14, -1, 12, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 2, 1, 3, -1, -1, -1, 6, -1, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [1, 11, 4, 0, 3, -1, 13, 12, 2, 7, -1, -1, 15, 10, 5, 8, 14, -1, -1, -1, -1, -1, 9, -1, -1, -1, 6, -1, -1, -1, -1, -1],
    [0, 9, 2, 14, 15, 4, 1, 13, 3, 5, -1, -1, 10, -1, -1, -1, -1, 6, 12, -1, 7, -1, 8, -1, -1, -1, 11, -1, -1, -1, -1, -1],
    [-1, 2, 14, -1, 1, 5, 8, 7, 4, 12, -1, 6, 9, 11, 13, 3, 10, 15, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 1, 3, 2, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [4, 3, 1, 5, -1, -1, -1, 0, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [2, 8, 4, 1, -1, 0, -1, 6, -1, -1, 5, -1, 7, -1, -1, -1, -1, -1, -1, -1, 10, -1, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1],
    [12, 5, -1, -1, 1, -1, -1, 7, 0, 3, -1, 2, -1, 4, 6, -1, -1, -1, -1, 8, -1, -1, 15, -1, 13, 9, -1, -1, -1, -1, -1, 11],
    [1, 3, 2, 4, -1, -1, -1, 5, -1, 7, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, 8, -1, -1],
    [5, 3, 4, 12, 1, 6, -1, -1, -1, -1, 8, 2, -1, -1, -1, -1, 0, 9, -1, -1, 11, -1, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, 0, -1, 1, 12, 3, -1, -1, -1, -1, 5, -1, -1, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, 4, -1, -1, 6, -1, 10],
    [2, 3, 1, 4, -1, 0, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1],
    [5, 1, 3, 0, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, -1, -1, -1, -1, -1, 9, -1, -1, 6, -1, 7]
];

static CHRS_BY_CHR_AND_SUCCESSOR_ID: [[u8; 16]; (MAX_CHR - MIN_CHR) as usize] = [
    [b's', b't', b'c', b'l', b'm', b'a', b'd', b'r', b'v', b'T', b'A', b'L', b'e', b'M', b'Y', b'-'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'-', b't', b'a', b'b', b's', b'h', b'c', b'r', b'n', b'w', b'p', b'm', b'l', b'd', b'i', b'f'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'u', b'e', b'i', b'a', b'o', b'r', b'y', b'l', b'I', b'E', b'R', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'e', b'a', b'o', b'i', b'u', b'A', b'y', b'E', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b't', b'n', b'f', b's', b'\'', b'm', b'I', b'N', b'A', b'E', b'L', b'Z', b'r', b'V', b'R', b'C'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'o', b'a', b'y', b'i', b'u', b'e', b'I', b'L', b'D', b'\'', b'E', b'Y', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'r', b'i', b'y', b'a', b'e', b'o', b'u', b'Y', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'h', b'o', b'e', b'E', b'i', b'u', b'r', b'w', b'a', b'H', b'y', b'R', b'Z', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'h', b'i', b'e', b'a', b'o', b'r', b'I', b'y', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'n', b't', b's', b'r', b'l', b'd', b'i', b'y', b'v', b'm', b'b', b'c', b'g', b'p', b'k', b'u'],
    [b'e', b'l', b'o', b'u', b'y', b'a', b'r', b'i', b's', b'j', b't', b'b', b'v', b'h', b'm', b'd'],
    [b'o', b'e', b'h', b'a', b't', b'k', b'i', b'r', b'l', b'u', b'y', b'c', b'q', b's', b'-', b'd'],
    [b'e', b'i', b'o', b'a', b's', b'y', b'r', b'u', b'd', b'l', b'-', b'g', b'n', b'v', b'm', b'f'],
    [b'r', b'n', b'd', b's', b'a', b'l', b't', b'e', b'm', b'c', b'v', b'y', b'i', b'x', b'f', b'p'],
    [b'o', b'e', b'r', b'a', b'i', b'f', b'u', b't', b'l', b'-', b'y', b's', b'n', b'c', b'\'', b'k'],
    [b'h', b'e', b'o', b'a', b'r', b'i', b'l', b's', b'u', b'n', b'g', b'b', b'-', b't', b'y', b'm'],
    [b'e', b'a', b'i', b'o', b't', b'r', b'u', b'y', b'm', b's', b'l', b'b', b'\'', b'-', b'f', b'd'],
    [b'n', b's', b't', b'm', b'o', b'l', b'c', b'd', b'r', b'e', b'g', b'a', b'f', b'v', b'z', b'b'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'e', b'n', b'i', b's', b'h', b'l', b'f', b'y', b'-', b'a', b'w', b'\'', b'g', b'r', b'o', b't'],
    [b'e', b'l', b'i', b'y', b'd', b'o', b'a', b'f', b'u', b't', b's', b'k', b'w', b'v', b'm', b'p'],
    [b'e', b'a', b'o', b'i', b'u', b'p', b'y', b's', b'b', b'm', b'f', b'\'', b'n', b'-', b'l', b't'],
    [b'd', b'g', b'e', b't', b'o', b'c', b's', b'i', b'a', b'n', b'y', b'l', b'k', b'\'', b'f', b'v'],
    [b'u', b'n', b'r', b'f', b'm', b't', b'w', b'o', b's', b'l', b'v', b'd', b'p', b'k', b'i', b'c'],
    [b'e', b'r', b'a', b'o', b'l', b'p', b'i', b't', b'u', b's', b'h', b'y', b'b', b'-', b'\'', b'm'],
    [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'e', b'i', b'o', b'a', b's', b'y', b't', b'd', b'r', b'n', b'c', b'm', b'l', b'u', b'g', b'f'],
    [b'e', b't', b'h', b'i', b'o', b's', b'a', b'u', b'p', b'c', b'l', b'w', b'm', b'k', b'f', b'y'],
    [b'h', b'o', b'e', b'i', b'a', b't', b'r', b'u', b'y', b'l', b's', b'w', b'c', b'f', b'\'', b'-'],
    [b'r', b't', b'l', b's', b'n', b'g', b'c', b'p', b'e', b'i', b'a', b'd', b'm', b'b', b'f', b'o'],
    [b'e', b'i', b'a', b'o', b'y', b'u', b'r', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'],
    [b'a', b'i', b'h', b'e', b'o', b'n', b'r', b's', b'l', b'd', b'k', b'-', b'f', b'\'', b'c', b'b'],
    [b'p', b't', b'c', b'a', b'i', b'e', b'h', b'q', b'u', b'f', b'-', b'y', b'o', b'\x00', b'\x00', b'\x00'],
    [b'o', b'e', b's', b't', b'i', b'd', b'\'', b'l', b'b', b'-', b'm', b'a', b'r', b'n', b'p', b'w']
];


struct Pack {
    word: u32,
    bytes_packed: usize,
    bytes_unpacked: usize,
    offsets: [usize; 8],
    masks: [i16; 8],
    header_mask: u8,
    header: u8,
}

const PACK_COUNT: usize = 3;
const MAX_SUCCESSOR_N: usize = 7;

const PACKS: [Pack; PACK_COUNT] = [
    Pack {
        word: 0x80000000,
        bytes_packed: 1,
        bytes_unpacked: 2,
        offsets: [26, 24, 24, 24, 24, 24, 24, 24],
        masks: [15, 3, 0, 0, 0, 0, 0, 0],
        header_mask: 0xc0,
        header: 0x80,
    },
    Pack {
        word: 0xc0000000,
        bytes_packed: 2,
        bytes_unpacked: 4,
        offsets: [25, 22, 19, 16, 16, 16, 16, 16],
        masks: [15, 7, 7, 7, 0, 0, 0, 0],
        header_mask: 0xe0,
        header: 0xc0,
    },
    Pack {
        word: 0xe0000000,
        bytes_packed: 4,
        bytes_unpacked: 8,
        offsets: [23, 19, 15, 11, 8, 5, 2, 0],
        masks: [31, 15, 15, 15, 7, 7, 7, 3],
        header_mask: 0xf0,
        header: 0xe0,
    },
];

fn decode_header(val: u8) -> isize {
    let mut i = -1;
    let mut val = val as i8; // Make sure we use i32 for correct shifting behavior
    while val < 0 {
        val <<= 1;
        i += 1;
    }
    i
}

struct Code {
    word: u32,
    bytes: [u8; 4],
}

fn check_indices(indices: &[i16], pack_n: usize) -> bool {
    for i in 0..PACKS[pack_n].bytes_unpacked {
        if indices[i] > PACKS[pack_n].masks[i] {
            return false;
        }
    }
    true
}

fn find_best_encoding(indices: &[i16], n_consecutive: usize) -> isize {
    for p in (0..PACK_COUNT).rev() {
        if n_consecutive >= PACKS[p].bytes_unpacked && check_indices(indices, p) {
            return p as isize;
        }
    }
    -1
}

fn swap(x: u32) -> u32 {
    if cfg!(target_endian = "little") {
        (x<<24) + ((x&0x0000FF00)<<8) + ((x&0x00FF0000)>>8) + (x>>24)
    } else {
        x
    }
}

fn shoco_compress(original: &[u8], strlen: usize, out : &mut[u8], buf_size: usize) -> usize {
    let mut o_ind = 0;
    let out_end_ind = buf_size;
    let mut in_ind = 0;
    let mut indices: [i16; MAX_SUCCESSOR_N + 1] = [0; MAX_SUCCESSOR_N + 1];
    let mut last_chr_index: isize;
    let mut current_index: isize;
    let mut successor_index: isize;
    let mut n_consecutive: usize = 1;
    let mut code = Code {word: 0, bytes: [0; 4]};
    let mut pack_n: isize;
    let mut rest: isize;
    let in_end_ind = strlen;

    while in_ind < original.len() {
        if strlen > 0 && in_ind == in_end_ind {
            break;
        }
        // find the longest string of known successors
        indices[0] = CHR_IDS_BY_CHR[original[in_ind] as usize] as i16;
        last_chr_index = indices[0] as isize;
        if last_chr_index < 0 {
            //goto last_resort
            if original[in_ind] & 0x80 != 0 {
                // non-ascii case
                if o_ind + 2 > out_end_ind {
                    return buf_size + 1;
                }
                out[o_ind] = 0x00;
                o_ind += 1;
            } else {
                //an ascii byte
                if o_ind + 1 > out_end_ind {
                    return buf_size + 1;
                }
            }
            out[o_ind] = original[in_ind];
            o_ind += 1;
            in_ind += 1;
            continue;
        }

        rest = in_end_ind as isize - in_ind as isize;
        for n in 1..(MAX_SUCCESSOR_N + 1) {
            n_consecutive = n;
            if strlen > 0 && n_consecutive as isize == rest {
                break;
            }

            if n_consecutive + in_ind >= original.len() {
                break;
            }

            current_index = CHR_IDS_BY_CHR[original[in_ind + n_consecutive] as usize] as isize;
            if current_index < 0 { // '\0' is always -1
                break;
            }

            successor_index = SUCCESSOR_IDS_BY_CHR_ID_AND_CHR_ID[last_chr_index as usize][current_index as usize] as isize;
            if successor_index < 0 {
                break;
            }

            indices[n_consecutive] = successor_index as i16;
            last_chr_index = current_index;
        }
        if n_consecutive < 2 {
            //goto last_resort;
            if original[in_ind] & 0x80 != 0 {
                // non-ascii case
                if o_ind + 2 > out_end_ind {
                    return buf_size + 1;
                }
                out[o_ind] = 0x00;
                o_ind += 1;
            } else {
                //an ascii byte
                if o_ind + 1 > out_end_ind {
                    return buf_size + 1;
                }
            }
            out[o_ind] = original[in_ind];
            o_ind += 1;
            in_ind += 1;
            continue;
        }
        pack_n = find_best_encoding(&indices, n_consecutive);
        if pack_n >= 0 {
            if PACKS[pack_n as usize].bytes_packed + o_ind > out_end_ind {
                return buf_size + 1;
            }

            code.word = PACKS[pack_n as usize].word;
            for i in 0..PACKS[pack_n as usize].bytes_unpacked {
                code.word |= (indices[i] as u32) << PACKS[pack_n as usize].offsets[i];
            }

            // In the little-endian world, we need to swap what's
            // in the register to match the memory representation.
            // On big-endian systems, this is a dummy.
            code.word = swap(code.word);
            if cfg!(target_endian = "little") {
                code.bytes = code.word.to_le_bytes();
            } else {
                code.bytes = code.word.to_be_bytes();
            }
        


            // if we'd just copy the word, we might write over the end
            // of the output string
            for i in 0..PACKS[pack_n as usize].bytes_packed {
                out[o_ind + i] = code.bytes[i];
            }
            o_ind += PACKS[pack_n as usize].bytes_packed;
            in_ind += PACKS[pack_n as usize].bytes_unpacked;
        } else {
            //goto last_resort
            if original[in_ind] & 0x80 != 0 {
                // non-ascii case
                if o_ind + 2 > out_end_ind {
                    return buf_size + 1;
                }
                out[o_ind] = 0x00;
                o_ind += 1;
            } else {
                //an ascii byte
                if o_ind + 1 > out_end_ind {
                    return buf_size + 1;
                }
            }
            out[o_ind] = original[in_ind];
            o_ind += 1;
            in_ind += 1;
        }
    }

    o_ind
}

fn shoco_decompress(original: &[u8], complen: usize, out: &mut [u8], buf_size: usize) -> usize {
    let mut o_ind = 0;
    let out_end_ind = buf_size;
    let mut in_ind = 0;
    let mut last_chr: u8;
    let mut code = Code {word: 0, bytes: [0; 4]};
    let mut offset: usize;
    let mut mask: i16;
    let mut mark: isize;
    let in_end_ind = complen;

    while in_ind < in_end_ind {
        mark = decode_header(original[in_ind]);
        if mark < 0 {
            if o_ind >= out_end_ind {
                return buf_size + 1;
            }

            // ignore the sentinel value for non-ascii chars
            if original[in_ind] == 0x00 {
                in_ind += 1;
                if in_ind >= in_end_ind {
                    return usize::MAX;
                }
            }

            out[o_ind] = original[in_ind];
            o_ind += 1;
            in_ind += 1;
        } else {
            if (PACKS[mark as usize].bytes_unpacked + o_ind) > out_end_ind {
                return buf_size + 1;
            }
            else if (PACKS[mark as usize].bytes_packed + in_ind) > in_end_ind {
                return usize::MAX;
            }

            for i in 0..PACKS[mark as usize].bytes_packed {
                    code.bytes[i] = original[in_ind + i];   
            }
            if cfg!(target_endian = "little") {
                code.word = u32::from_le_bytes(code.bytes);
            } else {
                code.word = u32::from_be_bytes(code.bytes);
            }
        
            code.word = swap(code.word);

            // unpack the leading char
            offset = PACKS[mark as usize].offsets[0];
            mask = PACKS[mark as usize].masks[0];
            last_chr = CHRS_BY_CHR_ID[((code.word >> offset) & mask as u32) as usize];
            out[o_ind] = last_chr;

            // unpack the successor chars
            for i in 1..PACKS[mark as usize].bytes_unpacked {
                offset = PACKS[mark as usize].offsets[i];
                mask = PACKS[mark as usize].masks[i];
                last_chr = CHRS_BY_CHR_AND_SUCCESSOR_ID[last_chr as usize - MIN_CHR][((code.word >> offset) & mask as u32) as usize];
                if (o_ind + i) >= buf_size {
                    break;
                }
                out[o_ind + i] = last_chr;
            }
            o_ind += PACKS[mark as usize].bytes_unpacked;
            in_ind += PACKS[mark as usize].bytes_packed;
        }
    }

    // Ensure to null-terminate if there's space, mimic C behavior
    if o_ind < out_end_ind {
        out[o_ind] = b'\0';
    }

    o_ind
}

fn main() {
    let mut buf_1: [u8; 1] = [0; 1];
    let mut buf_2: [u8; 2] = [0; 2];
    let mut buf_4: [u8; 4] = [0; 4];
    let mut buf_large: [u8; 4096] = [0; 4096];
    let mut ret: usize;
    
    let large_str = "This is a large string that won't possibly fit into a small buffer";
    let non_ascii_str = "Übergrößenträger";

    // Test compression
    ret = shoco_compress(large_str.as_bytes(), 0, &mut buf_2, 2);
    assert!(ret == 3); // bufsize + 1 if buffer too small

    ret = shoco_compress(large_str.as_bytes(), 0, &mut buf_large, 4096);
    assert!(ret <= large_str.len());

    ret = shoco_compress(b"a", 0, &mut buf_1, 1);
    assert!(ret == 1); // bufsize if null byte didn't fit

    buf_2[1] = b'x';
    ret = shoco_compress(b"a", 0, &mut buf_2, 2);
    assert!(ret == 1); // compressed string length without null byte
    assert!(buf_2[1] == b'x'); // Check if canary byte is still alive

    ret = shoco_compress(b"a", 0, &mut buf_4, 4);
    assert!(ret == 1); 

    ret = shoco_compress(b"test", 0, &mut buf_4, 4);
    assert!(ret <= 4);

    buf_4[1] = b'x';
    ret = shoco_compress(b"test", 1, &mut buf_4, 4); // Buffer large enough, but strlen said "just compress first char"
    assert!(ret == 1);
    assert!(buf_4[1] == b'x');

    ret = shoco_compress(b"t\x80", 1, &mut buf_4, 4);
    assert!(ret == 1);
    assert!(buf_4[1] == b'x');

    buf_4[1] = b'y';
    ret = shoco_compress(b"test", 1, &mut buf_4, 1);
    assert!(ret == 1); 
    assert!(buf_4[1] == b'y');// No null byte written

    buf_4[1] = b'z';
    ret = shoco_compress(b"a", 1, &mut buf_4, 4);
    assert!(ret == 1);
    assert!(buf_4[1] == b'z');

    buf_4[1] = b'b';
    ret = shoco_compress(b"a", 2, &mut buf_4, 4); //fail
    assert!(ret == 1);
    assert!(buf_4[1] == b'b');

    ret = shoco_compress(b"\xE4", 0, &mut buf_1, 1); // Assumes that 'ä' is not in the frequent chars table
    assert!(ret == 2);

    ret = shoco_compress(b"abca", 0, &mut buf_2, 2);
    assert!(ret == 3);

    // Test decompression
    let mut compressed_large: [u8; 4096] = [0; 4096];
    let large_len: usize = large_str.len();
    let mut comp_len: usize;
    comp_len = shoco_compress(large_str.as_bytes(), 0, &mut compressed_large, 4096);
    // println!("{:?}", compressed_large);

    buf_large[large_len] = b'x';
    ret = shoco_decompress(&compressed_large, comp_len, &mut buf_large, 4096);
    assert!(ret == large_len);
    assert!(large_str.as_bytes().eq(&buf_large[0..large_len])); //fail
    assert!(buf_large[large_len] == b'\0'); // null byte written

    ret = shoco_decompress(&compressed_large, comp_len, &mut buf_2, 2);
    assert!(ret == 3); // ret = bufsize + 1, because buffer too small

    buf_large[large_len] = b'x';
    ret = shoco_decompress(&compressed_large, comp_len, &mut buf_large, large_len);
    assert!(ret == large_len);
    assert!(buf_large[large_len] != b'\0'); // no null byte written

    ret = shoco_decompress(&compressed_large, 5, &mut buf_large, 4096);
    assert!((ret < large_len) || (ret == 4097)); // either fail (bufsize + 1) or it happened to work

    let mut compressed_non_ascii: [u8; 256] = [0; 256];
    let non_ascii_len: usize = non_ascii_str.len();
    comp_len = shoco_compress(non_ascii_str.as_bytes(), 0, &mut compressed_non_ascii, 256);

    buf_large[non_ascii_len] = b'x';
    ret = shoco_decompress(&compressed_non_ascii, comp_len, &mut buf_large, 4096);
    assert!(ret == non_ascii_len);
    assert!(non_ascii_str.as_bytes().eq(&buf_large[0..non_ascii_len]));
    assert!(buf_large[non_ascii_len] == b'\0'); // null byte written
  
    ret = shoco_decompress(b"\x00", 1, &mut buf_large, 4096);
    assert!(ret == usize::MAX);
  
    ret = shoco_decompress(b"\xe0ab", 3, &mut buf_large, 4096);
    assert!(ret == usize::MAX);
  
    let mut comp:[u8; 2] = [0; 2];
    let str:[u8; 2] = [b'A'; 2];
    comp_len = shoco_compress(&str, 0, &mut comp, 2);
    ret = shoco_decompress(&comp, comp_len, &mut buf_1, 1);
    assert!(ret == 2);

    println!("All tests passed.");
}
