#[derive(Debug, Clone, Copy)]
struct Item {
    weight: usize,
    value: usize,
}

struct Knapsack {
    max_value: usize,
    selected_items: Vec<Item>,
}

fn main() {
    let items = vec![
        Item { weight: 3, value: 5 },
        Item { weight: 4, value: 6 },
        Item { weight: 5, value: 8 },
        Item { weight: 6, value: 10 },
    ];
    let capacity = 10;

    let Knapsack {
        max_value,
        selected_items,
    } = knapsack(&items, capacity);
    println!("\nMaximum value: {}", max_value);
    println!("Selected items: {:?}", selected_items);
}

fn knapsack(items: &[Item], capacity: usize) -> Knapsack {
    let n = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    println!("Initial DP table:");
    print_dp_table(&dp, capacity);

    for i in 1..=n {
        for j in 0..=capacity {
            if items[i - 1].weight <= j {
                dp[i][j] = std::cmp::max(
                    dp[i - 1][j],
                    dp[i - 1][j - items[i - 1].weight] + items[i - 1].value,
                );
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
        println!("\nDP table after processing item {} (weight: {}, value: {}):", i, items[i - 1].weight, items[i - 1].value);
        print_dp_table(&dp, capacity);
    }

    let mut selected_items = Vec::new();
    let mut remaining_capacity = capacity;
    for i in (1..=n).rev() {
        if dp[i][remaining_capacity] != dp[i - 1][remaining_capacity] {
            selected_items.push(items[i - 1]);
            remaining_capacity -= items[i - 1].weight;
        }
    }

    selected_items.reverse();

    Knapsack {
        max_value: dp[n][capacity],
        selected_items,
    }
}

fn print_dp_table(dp: &[Vec<usize>], capacity: usize) {
    print!("     ");
    for j in 0..=capacity {
        print!("{:2} ", j);
    }
    println!();
    for (i, row) in dp.iter().enumerate() {
        if i == 0 {
            print!(" 0 | ");
        } else {
            print!("{:2} | ", i);
        }
        for &val in row {
            print!("{:2} ", val);
        }
        println!();
    }
}
