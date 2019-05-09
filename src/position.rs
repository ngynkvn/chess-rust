type Bitboard = u64;

#[derive(Debug)]
pub enum Color {
    White,
    Black,
    All,
}

#[derive(Debug)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Position {
    white: Bitboard,
    black: Bitboard,
    pawn: Bitboard,
    bishop: Bitboard,
    knight: Bitboard,
    king: Bitboard,
    queen: Bitboard,
    rook: Bitboard,
    turn: Color,
}

impl Position {
    pub fn new() -> Self {
        Self {
            white: 0xffff,
            black: 0xffff000000000000,
            pawn: 0xff00000000ff00,
            bishop: 0x2400000000000024,
            knight: 0x4200000000000042,
            queen: 0x1000000000000010,
            king: 0x800000000000008,
            rook: 0x8100000000000081,
            turn: Color::White,
        }
    }

    pub fn get(&self, piece: &Piece, color: &Color) -> Bitboard {
        let color_board = match color {
            Color::Black => self.black,
            Color::White => self.white,
            Color::All => self.white | self.black,
        };
        let piece_board = match piece {
            Piece::Pawn => self.pawn,
            Piece::Bishop => self.bishop,
            Piece::Knight => self.knight,
            Piece::King => self.king,
            Piece::Queen => self.queen,
            Piece::Rook => self.rook,
        };
        color_board & piece_board
    }
}

fn piece_char(p: &Piece, c: &Color) -> char {
    let mut chr = match p {
        Piece::Pawn => 'p',
        Piece::Bishop => 'b',
        Piece::Knight => 'n',
        Piece::King => 'k',
        Piece::Queen => 'q',
        Piece::Rook => 'r',
    };
    match c {
        Color::White => chr.make_ascii_lowercase(),
        Color::Black => chr.make_ascii_uppercase(),
        Color::All => panic!(),
    };
    chr
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const COLORS: [Color; 2] = [Color::White, Color::Black];
        const PIECES: [Piece; 6] = [
            Piece::Pawn,
            Piece::Bishop,
            Piece::Rook,
            Piece::Knight,
            Piece::King,
            Piece::Queen,
        ];
        let mut output = ['.'; 64];
        for c in &COLORS {
            for p in &PIECES {
                let pos = self.get(p, c);
                let mut mask = 0;
                while mask < 64 {
                    let chr = pos & (1 << mask);
                    if chr != 0 {
                        output[mask] = piece_char(p, c);
                    }
                    mask += 1;
                }
            }
        }
        let mut out = std::string::String::new();
        for i in output.chunks_exact(8) {
            let s: String = i.iter().collect();
            out.push_str(&s);
            out.push('\n');
        }
        writeln!(f, "{}", out)
    }
}
