use std::env;

struct Arguments {
    start: f64,
    end: f64,
    increment: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut args_object = Arguments {
        start: 1f64,
        end: 10f64,
        increment: 1f64,
    };

    match args.len() {
        1 => {
            // 0 args: Use defaults
        }
        2 => {
            // 1 arg: Default start and increment
            args_object.end = args[1].parse::<f64>().unwrap();
        }
        3 => {
            // 2 args: Default increment
            args_object.start = args[1].parse::<f64>().unwrap();
            args_object.end = args[2].parse::<f64>().unwrap();
        }
        4 => {
            // 3 args: Replace defaults
            args_object.start = args[1].parse::<f64>().unwrap();
            args_object.end = args[2].parse::<f64>().unwrap();
            args_object.increment = args[3].parse::<f64>().unwrap();
        }
        _ => {
            panic!("Invalid number of arguments. Use format: <start> <end> <increment>");
        }
    };

    if args_object.start % args_object.increment != 0f64 {
        panic!(
            "Start ({}) must be divisible by {}",
            args_object.start, args_object.increment
        )
    };
    if args_object.end % args_object.increment != 0f64 {
        panic!(
            "End ({}) must be divisible by {}",
            args_object.end, args_object.increment
        )
    };

    println!(
        "Start: {}\nEnd: {}\nIncrement: {}",
        args_object.start, args_object.end, args_object.increment
    );
    println!("Sum: {}", sum(&args_object));
}

fn sum(args_object: &Arguments) -> f64 {
    let _end_sum: f64 = args_object.start + args_object.end;
    let _multiplier: f64 = multiplier(args_object);
    // println!("Multiplier: {}", _multiplier);
    _end_sum * _multiplier
}

fn multiplier(args_object: &Arguments) -> f64 {
    let _num_addend: i32 = num_addend(args_object);
    // println!("Number of Addends: {}", _num_addend);
    _num_addend as f64 / 2f64
}

fn num_addend(args_object: &Arguments) -> i32 {
    let _end_difference: f64 = args_object.end - args_object.start;
    let _average: f64 = _end_difference / args_object.increment;
    _average as i32 + 1
}
