fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let (mut r, mut n) = (1, s.len() as i32 - 1);

        if num_rows == 1 {
            return s;
        }

        loop {
            if n <= num_rows - 1 {
                break;
            }
            n -= num_rows - 1;
            if n <= num_rows - 1 {
                r += n;
                break;
            }
            r += num_rows - 1;
            n -= num_rows - 1;
        }
        let mut m = vec![vec![0 as char; r as usize]; num_rows as usize];
        let (mut dx, mut dy): (i32, i32) = (0, 1);
        let (mut x, mut y): (i32, i32) = (0, 0);
        for (i, c) in s.chars().enumerate() {
            m[y as usize][x as usize] = c;
            if i as i32 % (num_rows - 1) == 0 {
                if y == 0 {
                    dx = 0;
                    dy = 1;
                } else {
                    dx = 1;
                    dy = -1;
                }
            }
            y += dy;
            x += dx;
        }
        return m
            .into_iter()
            .flat_map(|mm| mm.into_iter().filter(|c| *c != '\0'))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }
}
