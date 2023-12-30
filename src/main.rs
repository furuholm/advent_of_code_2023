use std::collections::HashSet;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/day3.txt")).split('\n');
    let Tokens {
        symbols,
        part_numbers,
    } = tokenize(input);

    let part_numbers_with_adjacent_symbols = part_numbers
        .into_iter()
        .filter(|part_number| part_number.adjacent().any(|pos| symbols.contains(&pos)));

    let sum: u32 = part_numbers_with_adjacent_symbols
        .map(|part_number| part_number.value)
        .sum();

    println!("{sum}");
}

fn tokenize(rows: impl Iterator<Item = &'static str>) -> Tokens {
    let mut tokens = Tokens::default();

    for (row, chars) in rows.enumerate() {
        let mut chars = chars.chars().enumerate().peekable();

        while let Some((col, c)) = chars.next() {
            match c {
                '.' => continue,
                c if c.is_numeric() => {
                    // Create iterator that advances chars
                    let chars_inner = std::iter::successors(Some(c), |_| {
                        // Only read next the if it's a digit
                        chars.next_if(|(_, c)| c.is_numeric()).map(|(_, c)| c)
                    });
                    // Collect numbers
                    let digits: Vec<u32> = chars_inner.map_while(|c| c.to_digit(10)).collect();
                    // Compute value from digits
                    let value = digits.iter().fold(0, |acc, number| acc * 10 + number);
                    tokens.part_numbers.insert(PartNumber {
                        start_pos: Pos { row, col },
                        len: digits.len(),
                        value,
                    })
                }
                _ => tokens.symbols.insert(Pos { row, col }),
            };
        }
    }

    tokens
}

#[derive(Debug, Default)]
struct Tokens {
    symbols: HashSet<Pos>,
    part_numbers: HashSet<PartNumber>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol(Pos);

#[derive(Debug, PartialEq, Eq, Hash)]
struct PartNumber {
    start_pos: Pos,
    len: usize,
    value: u32,
}

impl PartNumber {
    fn adjacent(&self) -> impl Iterator<Item = Pos> {
        // Ensure we don't go below 0.
        let start_row = self.start_pos.row.saturating_sub(1);
        let start_col = self.start_pos.col.saturating_sub(1);
        // We don't care if we are out of bounds in this direction
        let end_col = self.start_pos.col + self.len + 1;
        let end_row = self.start_pos.row + 2;

        (start_row..end_row)
            .flat_map(move |row| (start_col..end_col).map(move |col| Pos { row, col }))
    }
}
