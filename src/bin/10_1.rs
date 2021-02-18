fn main() {
    let lists1 = vec![10, 20, 30, 40, 50];
    let l1 = get_largest_number(&lists1);
    assert_eq!(l1, 50);
    println!("The latest number is {}", l1);
    let lists1 = vec![9, 1,2,3,4,5];
    let l2 = get_largest_number(&lists1);
    assert_eq!(l2, 9);
}

fn get_largest_number(lists: &[i32]) -> i32 {
    let mut largest = lists[0];
    for i in lists {
        if *i > largest {
            largest = *i;
        }
    }
    largest
}