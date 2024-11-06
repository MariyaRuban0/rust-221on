fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total_weight % n != 0 {
        return -1;
    }

    let average = total_weight / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as isize - average as isize;
        moves += balance.abs();
    }

    moves
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let base_weight = 5;
    vec![base_weight; n]
}
#[test]
fn test() {
    let shipments = vec![7, 3, 3, 7, 5];
    let moves = count_permutation(&shipments);
    if moves == -1 {
        println!("Неможливо розподілити вантажі рівномірно");
    } else {
        println!("Мінімальна кількість переміщень: {}", moves);
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", generated_shipments);
}
