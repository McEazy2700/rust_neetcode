fn main() {
    let mut nums = vec![1, 2, 3, 4];
    // let has_dups = brute_contains_duplicate(&nums);
    let has_dups = sort_contains_duplicates(&mut nums);
    println!("{}", has_dups)
}

/**
 Space -> O(n)
 Time -> O(n^2)
 */
fn brute_contains_duplicate(items: Vec<i32>) -> bool {
    let mut found = vec![];
    for item in items {
        if !found.contains(&item) {
            found.push(item.clone())
        } else {
            return true
        }
    }
    return false
}

fn sort_contains_duplicates(items: &mut Vec<i32>) -> bool {
    items.sort();
    for (index, &item) in items.iter().enumerate() {
        if items.len() >= index + 1 {
            if item == items[index + 1] {
                return true
            } else {
                return false
            }
        } else {
            return false
        }
    }
    return false
}
