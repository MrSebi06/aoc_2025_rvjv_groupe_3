use std::cmp::max;
use std::io::Split;
use std::ops;

pub fn p1(input: &str) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|x| {
            let mut split = x.splitn(2, ',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut max_area = 0;
    for i in 0..coords.len() - 1 {
        let (x1, y1) = coords[i];
        for j in i + 1..coords.len() {
            let (x2, y2) = coords[j];
            let area = (y2.abs_diff(y1) + 1) * (x2.abs_diff(x1) + 1);
            max_area = if area > max_area { area } else { max_area };
        }
    }

    max_area
}

struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn cross(&self, other: &Vec2) -> i64 {
        (self.x * other.y) - (self.y * other.x)
    }

    fn dot(&self, other: &Vec2) -> i64 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn p2(input: &str) -> u64 {
    let polygon_coords: Vec<(i64, i64)> = input
        .lines()
        .map(|x| {
            let mut split = x.splitn(2, ',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut max_area = 0;
    for i in 0..polygon_coords.len() - 1 {
        let (x1, y1) = polygon_coords[i];
        for j in i + 1..polygon_coords.len() {
            let (x2, y2) = polygon_coords[j];
            let rect_coords = [(x1, y1), (x2, y1), (x2, y2), (x1, y2)];
            for rect_i in 0..rect_coords.len() {
                let p = Vec2 {
                    x: rect_coords[rect_i].0,
                    y: rect_coords[rect_i].1,
                };
                let pr = Vec2 {
                    x: rect_coords[(rect_i + 1) % rect_coords.len()].0,
                    y: rect_coords[(rect_i + 1) % rect_coords.len()].1,
                };

                for polygon_i in 0..polygon_coords.len() {
                    if polygon_i == i {
                        continue;
                    }

                    let q = Vec2 {
                        x: polygon_coords[polygon_i].0,
                        y: polygon_coords[polygon_i].1,
                    };
                    let qs = Vec2 {
                        x: polygon_coords[(polygon_i + 1) % polygon_coords.len()].0,
                        y: polygon_coords[(polygon_i + 1) % polygon_coords.len()].1,
                    };

                    let r = &pr - &p;
                    let s = &qs - &q;

                    let rxs = (&r).cross(&s);
                    let qpxr = (&q - &p).cross(&r);

                    if rxs == 0 {
                        if qpxr == 0 {
                            let t0 = (&q - &p).dot(&r) / (&r.dot(&r));
                            let t1 = t0 + &s.dot(&r) / (&r.dot(&r));
                        }
                    }
                    let t = (&q - &p).cross(&s) / rxs;
                    let u = qpxr / rxs;
                }
            }

            // let area = (y2.abs_diff(y1) + 1) * (x2.abs_diff(x1) + 1);
            // max_area = if area > max_area { area } else { max_area };
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use crate::d9::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d9_test.txt");
        assert_eq!(p1(input), 50)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d9_test.txt");
        assert_eq!(p2(input), 24)
    }
}
