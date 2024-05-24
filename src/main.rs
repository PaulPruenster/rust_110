use std::{thread, time::Duration};

fn calc_110(line: &mut Vec<i32>) {
    let temp = line.clone();

    for i in 1..temp.len() - 1 {
        let repr: String = [
            temp[i - 1].to_string(),
            temp[i].to_string(),
            temp[i + 1].to_string(),
        ]
        .concat();

        line[i] = match repr.as_str() {
            "111" => 0,
            "110" => 1,
            "101" => 1,
            "100" => 0,
            "011" => 1,
            "010" => 1,
            "001" => 1,
            "000" => 0,
            _ => 420,
        };
    }
}

fn print_110(line: &Vec<i32>) {
    for cell in line {
        print!("{}", if *cell == 1 { "█" } else { " " });
    }
    println!();
}

fn main() {
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();

    let mut line = vec![0; cols as usize];
    let len = line.len();
    line[len - 1] = 1;

    loop {
        print_110(&line);
        calc_110(&mut line);
        thread::sleep(Duration::from_millis(5));
    }
}
