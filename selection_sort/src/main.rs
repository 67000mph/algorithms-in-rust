fn main() {
    let mut my_list = vec![5, 3, 6, 2, 10];
    println!("Array before sorting: {:?}", my_list);
    selection_sort(&mut my_list);
    println!(" Array after sorting: {:?}", my_list);
}

fn selection_sort(list: &mut Vec<i32>) {
    let mut new_list: Vec<i32> = Vec::new();
    for _ in 0..list.len() {
        let smallest = find_smallest(&list);
        new_list.push(list.remove(smallest));
    }
    *list = new_list;
}

fn find_smallest(list: &Vec<i32>) -> usize {
    let mut smallest = list[0];
    let mut smallest_idx = 0;
    for (i, v) in list.iter().enumerate() {
        if *v < smallest {
            smallest = *v;
            smallest_idx = i;
        }
    }
    smallest_idx
}
