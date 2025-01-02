fn main() {
    println!("Hello, world!");
}

struct Solution {}

#[derive(Debug)]
struct Stock {
    day: usize,
    price: i32,
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut stocks: Vec<Stock> = prices
            .iter()
            .enumerate()
            .map(|(i, &p)| Stock { day: i, price: p })
            .collect();
        stocks.sort_by(|d1, d2| d1.price.cmp(&d2.price));
        let lows: Vec<i32> = prices
            .iter()
            .scan(i32::MAX, |min, &price| {
                *min = (*min).min(price);
                Some(*min)
            })
            .collect();
        let mut best_profit = 0;
        for stock in stocks.iter() {
            best_profit = best_profit.max(stock.price - lows[stock.day])
        }
        best_profit
    }

   	#[allow(dead_code)]
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for (_day, &price) in prices.iter().enumerate() {
            min_price = min_price.min(price);

            let profit = price - min_price;

            max_profit = max_profit.max(profit);
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(4, Solution::max_profit(vec![1, 5]));
    }
}
