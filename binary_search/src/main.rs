mod binary_search;

fn main() {
    let arr = [
        1, 10, 20, 47, 59, 63, 75, 88, 99,
        107, 120, 133, 155, 162, 176, 188,
        199, 200, 210, 222
    ];
    let target: i32 = 47;
    if let Some(result) = binary_search::interactive(&arr, arr.len(), &target) {
        println!("{} found at index {}", target, result);
    } else {
        println!("{} not found", target);
    }
}
