#[test]
fn test() {
    let num_triangles = 4;
    draw_christmas_tree(num_triangles);
}

fn draw_christmas_tree(num_triangles: usize) {
    let mut total_height = 0;
    for triangle in 0..num_triangles {
        total_height += triangle + 3;
    }

    let mut current_height = 0;
    for triangle in 0..num_triangles {
        let lines = triangle + 3;

        for line in 0..lines {
            let spaces = total_height  - line +0;
            let stars = 2 * line + 1;

            let spaces_str = " ".repeat(spaces);
            let stars_str = "*".repeat(stars);

            println!("{}{}", spaces_str, stars_str);
        }

        current_height += lines;
    }
}

