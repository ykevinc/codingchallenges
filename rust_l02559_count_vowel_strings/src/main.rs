use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
        let vowls: Vec<bool> = words
            .iter()
            .map(|word| {
                word.chars().next().map_or(false, |c| vowels.contains(&c))
                    && word.chars().last().map_or(false, |c| vowels.contains(&c))
            })
            .collect();
        let o = queries
            .iter()
            .map(|query| {
                vowls
                    .iter()
                    .skip(query[0] as usize)
                    .take((query[1] - query[0] + 1) as usize)
                    .filter(|&is_vowel| *is_vowel)
                    .count() as i32
            })
            .collect();
        return o;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![2, 3, 0],
            Solution::vowel_strings(
                vec!["aba", "bcb", "ece", "aa", "e"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
                vec![vec![0, 2], vec![1, 4], vec![1, 1]]
            )
        );
    }
}
