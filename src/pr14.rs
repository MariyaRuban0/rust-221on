use std::collections::HashSet;

// Структура точки на площині
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Структура прямокутника на площині
struct Rectangle {
    a: Point, // ліва верхня точка
    b: Point, // права нижня точка
}

// Функція для обчислення фактичної зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied_points: HashSet<(i32, i32)> = HashSet::new();

    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                occupied_points.insert((x, y));
            }
        }
    }

    occupied_points.len() as i32
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Тест функції
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
#[test]
fn main() {
    area_occupied_test();
    println!("Success!");
}