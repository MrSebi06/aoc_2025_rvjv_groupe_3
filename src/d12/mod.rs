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

fn solve(possibilities: &[Vec<Vec<u64>>], grid: &mut Vec<u64>, shape_idx: usize) -> bool {
    if shape_idx == possibilities.len() {
        return true; // all shapes placed without overlap
    }
    for placement in &possibilities[shape_idx] {
        // check no overlap with current grid state
        if placement.iter().zip(grid.iter()).all(|(&a, &b)| a & b == 0) {
            // apply placement
            for (g, &p) in grid.iter_mut().zip(placement.iter()) {
                *g |= p;
            }
            if solve(possibilities, grid, shape_idx + 1) {
                return true;
            }
            // undo placement (backtrack)
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
        let block: Vec<_> = lines
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
            .collect::<Vec<_>>()
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
        let mut possibilities: Vec<Vec<Vec<u64>>> = vec![Vec::new(); allowed_shapes.iter().sum()];
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
                                let mut possibility = vec![0u64; size_y];
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
        // for shape_possibilities in possibilities {
        //     for possibility in shape_possibilities {
        //         for row in &possibility {
        //             for i in 0..size_x {
        //                 print!("{}", if row & (1 << i) != 0 { '#' } else { '.' });
        //             }
        //             println!();
        //         }
        //         println!();
        //     }
        // }

        let mut grid = vec![0u64; size_y];
        if solve(&possibilities, &mut grid, 0) {
            counter += 1;
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
