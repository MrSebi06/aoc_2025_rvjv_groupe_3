type Grid = Vec<u64>;

#[derive(Debug, Clone, Copy)]
struct Shape([u8; 3]);

impl Shape {
    #[inline(always)]
    fn a(&self) -> (u8) {
        self.0[0]
    }
    #[inline(always)]
    fn b(&self) -> (u8) {
        self.0[1]
    }
    #[inline(always)]
    fn c(&self) -> (u8) {
        self.0[2]
    }

    fn rotations(&self) -> [Shape; 4] {
        let r0 = *self;
        let r1 = r0.rotate90();
        let r2 = r1.rotate90();
        let r3 = r2.rotate90();
        [r0, r1, r2, r3]
    }

    fn rotate90(&self) -> Shape {
        let [a, b, c] = self.0;
        Shape([
            ((a >> 0 & 1) << 2 | (b >> 0 & 1) << 1 | (c >> 0 & 1)),
            ((a >> 1 & 1) << 2 | (b >> 1 & 1) << 1 | (c >> 1 & 1)),
            ((a >> 2 & 1) << 2 | (b >> 2 & 1) << 1 | (c >> 2 & 1)),
        ])
    }

    fn flip(&self) -> Shape {
        Shape(
            self.0
                .map(|row| ((row & 0b001) << 2) | (row & 0b010) | ((row & 0b100) >> 2)),
        )
    }

    fn all_orientations(&self) -> impl Iterator<Item = Shape> {
        let normal = self.rotations();
        let flipped = self.flip().rotations();
        normal.into_iter().chain(flipped.into_iter())
    }
}
impl From<[u8; 3]> for Shape {
    fn from(arr: [u8; 3]) -> Self {
        Shape(arr)
    }
}

fn empty_cells(grid: &Grid, size_x: usize) -> u32 {
    grid.iter()
        .map(|row| size_x as u32 - row.count_ones())
        .sum()
}

fn solve(
    possibilities: &[Vec<Grid>],
    grid: &mut Grid,
    shape_idx: usize,
    remaining_cells: &[u32],
    size_x: usize,
) -> bool {
    if shape_idx == possibilities.len() {
        return true;
    }
    if empty_cells(grid, size_x) < remaining_cells[shape_idx] {
        return false;
    }
    for placement in &possibilities[shape_idx] {
        if placement.iter().zip(grid.iter()).all(|(&a, &b)| a & b == 0) {
            for (g, &p) in grid.iter_mut().zip(placement.iter()) {
                *g |= p;
            }
            if solve(possibilities, grid, shape_idx + 1, remaining_cells, size_x) {
                return true;
            }
            for (g, &p) in grid.iter_mut().zip(placement.iter()) {
                *g &= !p;
            }
        }
    }
    false
}

fn solve_v2(
    possibilities: &[Vec<Grid>],
    grid: &mut Grid,
    shape_idx: usize,
    remaining_cells: &[u32],
    size_x: usize,
    groups: &[usize],
    min_index: &mut Vec<usize>,
) -> bool {
    if shape_idx == possibilities.len() {
        return true;
    }

    if empty_cells(grid, size_x) < remaining_cells[shape_idx] {
        return false;
    }

    // If same shape as previous slot, skip placements before previous choice
    let start = if shape_idx > 0 && groups[shape_idx] == groups[shape_idx - 1] {
        min_index[shape_idx - 1]
    } else {
        0
    };

    for (i, placement) in possibilities[shape_idx].iter().enumerate() {
        if i < start {
            continue;
        }
        if placement.iter().zip(grid.iter()).all(|(&a, &b)| a & b == 0) {
            for (g, &p) in grid.iter_mut().zip(placement.iter()) {
                *g |= p;
            }
            min_index[shape_idx] = i;
            if solve_v2(
                possibilities,
                grid,
                shape_idx + 1,
                remaining_cells,
                size_x,
                groups,
                min_index,
            ) {
                return true;
            }
            for (g, &p) in grid.iter_mut().zip(placement.iter()) {
                *g &= !p;
            }
        }
    }
    false
}

pub fn p1(input: &str) -> u64 {
    let mut lines = input.lines().peekable();
    let mut blocks = vec![];
    while lines.peek().is_some() {
        let block: Vec<&str> = lines
            .by_ref()
            .take_while(|l| l.trim().len() > 0 && !l.ends_with(":"))
            .collect();
        if !block.is_empty() {
            blocks.push(block);
        }
    }
    // println!("{blocks:?}");

    let regions = blocks.pop().expect("error while parsing blocks");

    let mut shapes: Vec<Shape> = Vec::new();
    for block in blocks {
        let arr: [u8; 3] = block
            .iter()
            .map(|&l| {
                l.chars()
                    .enumerate()
                    .map(|(i, c)| if c == '#' { 1u8 << i } else { 0 })
                    .sum()
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        shapes.push(Shape::from(arr));
    }
    // println!("{shapes:?}");

    let mut counter = 0;
    for region in regions {
        // println!("{}", region.splitn(2, ":").nth(0).unwrap());
        let mut parts = region.splitn(2, ':');
        let mut size = parts
            .next()
            .unwrap()
            .splitn(2, 'x')
            .map(|v| v.parse::<usize>().unwrap());
        let (size_x, size_y) = (size.next().unwrap(), size.next().unwrap());
        // let region_mask = vec![0u64; size_y];
        // println!("{region_mask:?}");

        let allowed_shapes: Vec<usize> = parts
            .next()
            .unwrap()
            .trim()
            .splitn(shapes.len(), " ")
            .map(|x| x.parse().unwrap())
            .collect();
        let total_pieces: usize = allowed_shapes.iter().sum();
        let blocks = (size_x / 3) * (size_y / 3);

        if total_pieces <= blocks {
            counter += 1;
        } else {
            let total_cells: usize = allowed_shapes
                .iter()
                .zip(shapes.iter())
                .map(|(&count, shape)| {
                    count
                        * shape
                            .0
                            .iter()
                            .map(|r| r.count_ones() as usize)
                            .sum::<usize>()
                })
                .sum();
            if total_cells > size_x * size_y {
                // Impossible: not enough room
                continue;
            } else {
                let mut possibilities: Vec<Vec<Grid>> =
                    vec![Vec::new(); allowed_shapes.iter().sum()];
                let mut groups: Vec<usize> = Vec::new();
                let mut slot = 0;
                for (i, shape) in shapes.iter().enumerate() {
                    for _ in 0..allowed_shapes[i] {
                        groups.push(i);
                        let mut seen = std::collections::HashSet::new();
                        let orientations: Vec<Shape> = shape
                            .all_orientations()
                            .filter(|s| seen.insert(s.0))
                            .collect();
                        for oriented_shape in orientations {
                            for y in 0..size_y {
                                for x in 0..size_x {
                                    let mask = (1u64 << size_x) - 1;

                                    if y + oriented_shape.0.len() > size_y {
                                        continue;
                                    }

                                    let lines: Vec<u64> = oriented_shape
                                        .0
                                        .iter()
                                        .map(|&line| (line as u64) << x)
                                        .collect();
                                    if lines.iter().all(|&r| r & !mask == 0) {
                                        let mut possibility: Grid = vec![0u64; size_y];
                                        for (i, &r) in lines.iter().enumerate() {
                                            possibility[y + i] = r;
                                        }
                                        possibilities[slot].push(possibility);
                                    }
                                }
                            }
                        }
                        slot += 1;
                    }
                }
                // for shape_possibilities in possibilities.iter().as_ref() {
                //     for possibility in shape_possibilities {
                //         for row in possibility {
                //             for i in 0..size_x {
                //                 print!("{}", if row & (1 << i) != 0 { '#' } else { '.' });
                //             }
                //             println!();
                //         }
                //         println!();
                //     }
                // }

                println!(
                    "SOLVER NEEDED: {}x{} pieces={} cells={} grid={}",
                    size_x,
                    size_y,
                    total_pieces,
                    total_cells,
                    size_x * size_y
                );
                // After building possibilities and groups, before solving:
                // Sort groups so the shape type with fewest placements-per-copy goes first
                let mut group_order: Vec<usize> = (0..6).collect(); // 6 shape types
                group_order.sort_by_key(|&shape_id| {
                    possibilities
                        .iter()
                        .zip(groups.iter())
                        .find(|(_, g)| **g == shape_id)
                        .map(|(p, _)| p.len())
                        .unwrap_or(usize::MAX)
                });
                // Rebuild possibilities and groups in new order
                let mut sorted_possibilities = Vec::new();
                let mut sorted_groups = Vec::new();
                for &shape_id in &group_order {
                    for (p, &g) in possibilities.iter().zip(groups.iter()) {
                        if g == shape_id {
                            sorted_possibilities.push(p.clone());
                            sorted_groups.push(g);
                        }
                    }
                }
                let possibilities = sorted_possibilities;
                let groups = sorted_groups;

                let piece_sizes: Vec<u32> = possibilities
                    .iter()
                    .map(|placements| placements[0].iter().map(|r| r.count_ones()).sum())
                    .collect();
                let mut grid = vec![0u64; size_y];
                let mut remaining_cells = vec![0u32; possibilities.len() + 1];
                for i in (0..possibilities.len()).rev() {
                    remaining_cells[i] = remaining_cells[i + 1] + piece_sizes[i];
                }

                if solve_v2(
                    &possibilities,
                    &mut grid,
                    0,
                    &remaining_cells,
                    size_x,
                    &groups,
                    &mut vec![0; groups.len()],
                ) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::d12::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d12_test.txt");
        assert_eq!(p1(input), 2)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d12_test.txt");
        assert_eq!(p2(input), 2)
    }
}
