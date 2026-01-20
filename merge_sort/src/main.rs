
fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]); // left half
        merge_sort(&mut arr[mid..]); // right half
        merge(arr, mid);
    }
    arr.to_vec()
}

fn main() {
    let mut vec = vec![6,22,4,7,3,5,1,8,2,19];
    merge_sort(&mut vec);
    println!("{:?}", vec)
}
