#[test]
fn test() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;

            // Добавляем условие для основной и второстепенной диагоналей
            let is_main_diagonal = x == y * W / H;
            let is_secondary_diagonal = x == (H - 1 - y) * W / H;

            let c = if is_horizontal || is_vertical || is_main_diagonal || is_secondary_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

