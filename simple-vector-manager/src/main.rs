use std::collections::HashMap;
use std::io;
fn main() {
    let mut vector: Vec<i32> = Vec::new();
    let mut is_exist: bool = false;

    println!("Приветсвую вас, уважаемый пользователь");

    loop {
        println!("-----МЕНЮ-----");
        println!("1. создать вектор и заполнить его");
        println!("2. добавить элементы в вектор");
        println!("3. Нахождение медианы");
        println!("4. Нахождение моды");
        println!("5. просмотреть вектор");

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("input error");

        let input = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => 0,
        };

        match input {
            1 => {
                if !is_exist {
                    println!("Вектор cоздан!");
                    is_exist = true;

                    println!("Введите кол-во элементов для заполнения:");

                    let n: i32 = get_and_check();

                    for i in 0..n {
                        println!("Введите {}-элемент", i + 1);
                        let n: i32 = get_and_check();
                        vector.push(n);
                    }
                } else {
                    println!("Вектор уже создан!");
                }
            }
            2 => {
                if is_exist {
                    println!("Введите количество элементов для добавления:");

                    let n: i32 = get_and_check();

                    for _ in 0..n {
                        println!("Введите {}-элемент", vector.len() + 1);
                        let n: i32 = get_and_check();
                        vector.push(n);
                    }
                } else {
                    println!("Вектора не обнаружено!");
                }
            }
            3 => {
                let mut new_vector: Vec<i32> = vector.clone();
                new_vector.sort_by(|a, b| a.cmp(b));

                println!("Медиана");
                println!("{}", new_vector[new_vector.len() / 2]);
            }
            4 => {
                let mut map: HashMap<i32, i32> = HashMap::new();

                for i in &vector {
                    let count = map.entry(*i).or_insert(0);
                    *count += 1;
                }

                let mut max: i32 = vector[0];

                for (key, value) in &map {
                    if max < *value {
                        max = *key
                    };
                }

                println!(
                    "Мода: {}, оно встречается {} раз",
                    max,
                    map.get(&max).copied().unwrap()
                );
            }
            5 => {
                if is_exist {
                    println!("Ваш вектор:");
                    println!("{:?}", vector);
                } else {
                    println!("Вектора не обнаружено!");
                }
            }
            _ => {
                println!("Досвидания, возращайтесь ещё");
                break;
            }
        };
    }
}

fn get_and_check() -> i32 {
    loop {
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("input error");

        match input.trim().parse() {
            Ok(input) => {
                return input;
            }
            Err(_) => {
                println!("Некорректный ввод, повторить попытку");
                continue;
            }
        };
    }
}
