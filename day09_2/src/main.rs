use std::{iter, ops::RangeInclusive};

use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 9, 2)]
fn main(input: &str) -> usize {
    let mut disk = parse_disk(input);

    for file in files_reverse(&disk) {
        let gap = gaps(&disk).find(|gap| gap.len() >= file.len() && gap.end < file.span.start);

        if let Some(gap) = gap {
            move_file(&mut disk, file, gap);
        }
    }

    let result = disk
        .iter()
        .enumerate()
        .filter(|(_, block)| block.is_file)
        .map(|(i, block)| i * block.id)
        .sum::<usize>();

    result
}

fn parse_disk(disk: &str) -> Vec<Block> {
    disk.trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .zip([true, false].into_iter().cycle())
        .enumerate()
        .flat_map(|(id, (size, is_file))| {
            let id = id / 2;
            let block = if is_file {
                Block { id, is_file }
            } else {
                Block { id: 0, is_file }
            };

            iter::repeat_n(block, size)
        })
        .collect::<Vec<_>>()
}

fn files_reverse(blocks: &[Block]) -> Vec<FileSpan> {
    blocks
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, block)| block.is_file)
        .map(|(i, file)| FileSpan::new_at(file.id, i))
        .coalesce(|b, a| {
            if a.contiguous_with(&b) {
                return Ok(a.joined_with(&b));
            }

            Err((b, a))
        })
        .collect()
}

fn gaps(blocks: &[Block]) -> impl Iterator<Item = Span> + use<'_> {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| !block.is_file)
        .map(|(i, _)| Span::new_at(i))
        .coalesce(|a, b| {
            if a.contiguous_with(&b) {
                return Ok(a.joined_with(&b));
            }

            Err((a, b))
        })
}

fn move_file(disk: &mut [Block], file_span: FileSpan, gap: Span) {
    debug_assert!(gap.len() >= file_span.len());

    let (gap_area, file_area) = disk.split_at_mut(file_span.span.start);
    let gap_area = &mut gap_area[gap.start..][..file_span.len()];
    let file_area = &mut file_area[..file_span.len()];

    gap_area.swap_with_slice(file_area);
}

#[derive(Copy, Clone)]
struct Block {
    id: usize,
    is_file: bool,
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new_at(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }

    pub fn contiguous_with(&self, other: &Self) -> bool {
        self.end + 1 == other.start
    }

    pub fn joined_with(&self, other: &Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

impl IntoIterator for Span {
    type Item = usize;
    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Debug)]
struct FileSpan {
    id: usize,
    span: Span,
}

impl FileSpan {
    pub fn new_at(id: usize, index: usize) -> Self {
        Self {
            id,
            span: Span::new_at(index),
        }
    }

    pub fn contiguous_with(&self, other: &Self) -> bool {
        self.id == other.id && self.span.contiguous_with(&other.span)
    }

    pub fn joined_with(&self, other: &Self) -> Self {
        debug_assert_eq!(self.id, other.id);

        Self {
            id: self.id,
            span: self.span.joined_with(&other.span),
        }
    }

    pub fn len(&self) -> usize {
        self.span.len()
    }
}
