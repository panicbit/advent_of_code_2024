use aoc::aoc;

#[aoc(2024, 9, 1)]
fn main(input: &str) -> i64 {
    let mut blocks = parse_blocks(input.trim());

    compact_files(&mut blocks);

    calculate_checksum(&blocks)
}

fn compact_files(blocks: &mut [Block]) {
    let mut start = 0;
    let mut end = blocks.len() - 1;

    while start < end {
        if !blocks[start].is_free() {
            start += 1;
            continue;
        }

        if !blocks[end].is_file() {
            end -= 1;
            continue;
        }

        blocks.swap(start, end);
    }
}

fn calculate_checksum(block: &[Block]) -> i64 {
    block
        .iter()
        .enumerate()
        .filter_map(|(position, block)| match block {
            Block::File(id) => Some(position as i64 * id),
            Block::Free => None,
        })
        .sum()
}

fn parse_blocks(input: &str) -> Vec<Block> {
    let mut id = -1;
    let mut blocks = Vec::new();
    let mut is_file = true;

    for ch in input.chars() {
        let len = ch.to_digit(10).unwrap();

        let block = if is_file {
            id += 1;
            Block::File(id)
        } else {
            Block::Free
        };

        for _ in 0..len {
            blocks.push(block);
        }

        is_file = !is_file;
    }

    blocks
}

#[derive(Debug, Copy, Clone)]
enum Block {
    File(i64),
    Free,
}

impl Block {
    fn is_file(&self) -> bool {
        matches!(self, Block::File(_))
    }

    fn is_free(&self) -> bool {
        matches!(self, Block::Free)
    }
}
