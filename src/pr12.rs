use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

fn print_vector_info(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:   [");
    for (i, value) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", value);
    }
    println!("]");

    let (min_sum, index1, index2) = min_adjacent_sum(data);

    // Вивід позначення мінімальної пари
    print!("indexes: ");
    for i in 0..data.len() {
        if i == index1 {
            print!("\\__");
        } else if i == index2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[index1], data[index2], min_sum, index1, index2
    );
}
#[test]
fn test() {
    let data = gen_random_vector(20);
    print_vector_info(&data);
}
