#[derive(Debug, Clone, Copy)]
struct Item {
    weight: i32,
    value: i32,
}

struct Knapsack {
    max_value: i32,
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

fn knapsack(items: &[Item], capacity: i32) -> Knapsack {
    let n = items.len();
    let mut dp = vec![vec![0; (capacity + 1) as usize]; n + 1];

    // Print initial DP table (all zeros)
    println!("Initial DP table:");
    print_dp_table(&dp, capacity);

    // Fill the DP table
    for i in 1..=n {
        for j in 0..=capacity {
            if items[i - 1].weight <= j {
                dp[i][j as usize] = std::cmp::max(
                    dp[i - 1][j as usize],
                    dp[i - 1][(j - items[i - 1].weight) as usize] + items[i - 1].value,
                );
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize];
            }
        }
        // Print DP table after processing each item
        println!("\nDP table after processing item {} (weight: {}, value: {}):", i, items[i - 1].weight, items[i - 1].value);
        print_dp_table(&dp, capacity);
    }

    // Backtrack to find the selected items
    let mut selected_items = Vec::new();
    let mut remaining_capacity = capacity;
    for i in (1..=n).rev() {
        if dp[i][remaining_capacity as usize] != dp[i - 1][remaining_capacity as usize] {
            selected_items.push(items[i - 1]);
            remaining_capacity -= items[i - 1].weight;
        }
    }

    // Reverse to maintain the original order of items
    selected_items.reverse();

    Knapsack {
        max_value: dp[n][capacity as usize],
        selected_items,
    }
}

// Helper function to print the DP table
fn print_dp_table(dp: &[Vec<i32>], capacity: i32) {
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
