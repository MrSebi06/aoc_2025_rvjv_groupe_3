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

fn point_in_polygon(px: i64, py: i64, polygon: &[(i64, i64)]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        if (yi > py) != (yj > py) {
            let edge = (xj - xi, yj - yi);
            let cross = (px - xi) * edge.1 - (py - yi) * edge.0;
            if cross == 0 {
                return true;
            }
            if (yj > yi && cross < 0) || (yj < yi && cross > 0) {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}

fn is_polygon_vertex(x: i64, y: i64, polygon: &[(i64, i64)]) -> bool {
    polygon.iter().any(|&(px, py)| px == x && py == y)
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
            let area = (y2.abs_diff(y1) + 1) * (x2.abs_diff(x1) + 1);
            if area < max_area {
                continue;
            }
            let rect_coords = [(x1, y1), (x2, y1), (x2, y2), (x1, y2)];

            let mut rect_crosses = false;
            for rect_i in 0..rect_coords.len() {
                if rect_crosses {
                    break;
                }
                let (rx1, ry1) = rect_coords[rect_i];
                let (rx2, ry2) = rect_coords[(rect_i + 1) % rect_coords.len()];

                for polygon_i in 0..polygon_coords.len() {
                    let (px1, py1) = polygon_coords[polygon_i];
                    let (px2, py2) = polygon_coords[(polygon_i + 1) % polygon_coords.len()];

                    let r_horizontal = ry1 == ry2;
                    let p_horizontal = py1 == py2;
                    if r_horizontal == p_horizontal {
                        continue;
                    }

                    let (hx1, hx2, hy, vx, vy1, vy2) = if r_horizontal {
                        (
                            rx1.min(rx2),
                            rx1.max(rx2),
                            ry1,
                            px1,
                            py1.min(py2),
                            py1.max(py2),
                        )
                    } else {
                        (
                            px1.min(px2),
                            px1.max(px2),
                            py1,
                            rx1,
                            ry1.min(ry2),
                            ry1.max(ry2),
                        )
                    };

                    if hx1 < vx && vx < hx2 && vy1 < hy && hy < vy2 {
                        rect_crosses = true;
                        break;
                    }
                }
            }
            let mut polygon_vertex_inside_rect = false;
            let (min_x, max_x) = (x1.min(x2), x1.max(x2));
            let (min_y, max_y) = (y1.min(y2), y1.max(y2));
            for &(px, py) in &polygon_coords {
                if px > min_x && px < max_x && py > min_y && py < max_y {
                    polygon_vertex_inside_rect = true;
                    break;
                }
            }
            if polygon_vertex_inside_rect {
                continue;
            }
            if !rect_crosses {
                let cx = (x1 + x2) / 2;
                let cy = (y1 + y2) / 2;
                let corners_and_center = [(x2, y1), (x1, y2), (cx, cy)];
                let all_inside = corners_and_center.iter().all(|&(cx, cy)| {
                    is_polygon_vertex(cx, cy, &polygon_coords)
                        || point_in_polygon(cx, cy, &polygon_coords)
                });
                if all_inside {
                    max_area = max_area.max(area);
                }
            }
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

    #[test]
    fn p2_test_custom() {
        let input = include_str!("d9_test_custom.txt");
        assert_eq!(p2(input), 45)
    }
}
