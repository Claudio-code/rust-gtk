
fn bubble_sort(array: &mut Vec<i8>) -> Vec<i8> {
    let mut sorted: bool;
    for _ in 1..=array.len() - 1 {
        sorted = true;

        for j in 0..=array.len() - 2  {
            if array[j] < array[j + 1] {
                continue;
            }

            array.swap(j, j + 1);
            sorted = false;
        }

        if sorted {
            break;
        }
    }
    array.to_vec()
}

fn main() {
    let mut vec = vec![5, 4, 1, 3, 2];
    bubble_sort(&mut  vec);
    println!("{:?}", vec);

    let mut vec2 = vec![5, 1, 4, 2, 8];
    bubble_sort(&mut  vec2);
    println!("{:?}", vec2);

}
