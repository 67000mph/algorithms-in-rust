fn main() {
    let my_list = [1, 3, 5, 7, 9];
    let items_to_find = [3, -1, 5, 42, 4];

    for item in items_to_find {
        let idx = binary_search(&my_list, item);
        match idx {
            Some(v) => println!("Item {} is at position {} in the list.", item, v),
            None => println!("Item {} is not in the list.", item)
        }
    }
}

fn binary_search(list: &[i32], item: i32) -> Option<isize> {
    let mut low: isize = 0;
    let mut high = list.len() as isize - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid as usize];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}
