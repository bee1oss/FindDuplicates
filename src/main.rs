fn main() {
    let nums = [10, 1, 2, 3, 4, 5, 3, 6, 7, 8, 7, 9, 10, 10, 10, 7, 7, 7];
    let chars = ['x', 'a', 'b', 'x', 'b', 'a', 'c', 'v', 's', 'd', 'f', 'x'];
    //print!("{:?}", find_duplicate_integer(&nums));
}

fn find_duplicate_integer<L:Display>(list:&[L])->Vec<L>{
    let mut duplicates: Vec<L> = Vec::new();

    for i in 0..(list.len()) {
        for j in (i + 1)..(list.len()) {
            if list[i] == list[j] {
                if !duplicates.contains(&list[i]) {
                    duplicates.push(list[i]);
                }
            }
        }
    }
    duplicates
}