//! Math helpers

pub fn float_eq(a: f32, b: f32) -> bool {
    float_eq_eps(a, b, 0.0001)
}

pub fn float_eq_eps(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

/// Compute bresenham line between two points
/// Adapted from https://www.codeproject.com/Articles/15604/Ray-casting-in-a-2D-tile-based-environment
pub fn bresenham_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut x1 = x1;
    let mut x2 = x2;
    let mut y1 = y1;
    let mut y2 = y2;

    let mut result = vec![];
    let absx = (x2 - x1).abs();
    let absy = (y2 - y1).abs();

    let steep = absy > absx;
    if steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }

    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }

    let deltax = x2 - x1;
    let deltay = (y2 - y1).abs();
    let mut error = 0;
    let mut y = y1;
    let ystep = if y1 < y2 { 1 } else { -1 };

    for x in x1..=x2 {
        if steep {
            result.push((y, x));
        } else {
            result.push((x, y));
        }

        error += deltay;
        if 2 * error >= deltax {
            y += ystep;
            error -= deltax;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert!(float_eq(1.0, 1.0));
        assert!(float_eq(1.000000, 1.000001));
    }

    #[test]
    fn test_bresenham() {
        let ((x1, y1), (x2, y2)) = ((0, 0), (4, 5));
        assert_eq!(
            bresenham_line(x1, y1, x2, y2),
            vec![(0, 0), (1, 1), (2, 2), (2, 3), (3, 4), (4, 5)]
        );
    }
}
