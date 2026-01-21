use core::num;

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let 循环可以用来在每次迭代中弹出栈顶元素，直到栈为空。
    // 输出：
    // 3
    // 2
    // 1
    //
    // 注意：
    // 1. while let 循环可以用来在每次迭代中弹出栈顶元素，直到栈为空。
    // 2. 栈是一种后进先出（LIFO）的数据结构，所以弹出的元素顺序是 3, 2, 1。
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = 1;
    match x {
        1 | 2 => {
            println!("x is 1, 2, or 3");
        }
        3 => {
            println!("x is 3");
        }
        _ => {
            println!("x is not 1, 2, or 3");
        }
    }

    let x = 5;
    match x {
        1..=5 => {
            println!("x is between 1 and 5");
        }
        _ => {
            println!("x is not between 1 and 5");
        }
    }

    let c = 'a';
    match c {
        'a'..='g' => {
            println!("c is a letter between a and g");
        }
        'h'..='n' => {
            println!("c is a letter between h and n");
        }
        _ => {
            println!("c is a letter between o and z");
        }
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 不指定名称
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y } => {
            println!("match x is zero y is {}", y);
        }
        _ => {
            println!("match x is not zero y is not zero");
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move x is {} y is {}", x, y);
        }
        Message::Write(s) => {
            println!("Write {}", s);
        }
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor r is {} g is {} b is {}", r, g, b);
        }
    }

    let ((feet, inches), Point { x, y}) = ((3, 10), Point { x: 3, y: -10});

    println!(
        "feet is {} inches is {} x is {} y is {}",
        feet, inches, x, y
    );

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("setting_value is Some(_) new_setting_value is Some(_)");
        }
        _ => {
            println!("setting_value is None or new_setting_value is None");
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
        _ => {
            println!("All numbers are different");
        }
    }

    println!("-----------");
    let s = Some(String::from("Hello!"));

    if let Some(_s) = &s {
        println!("found a string");
    }

    println!("{:?}", s);

    println!("--------------------------------------------使用_, 不绑定任何变量,没有转移所有权");
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => {
            println!("x is {}", x);
        },
        _ => {
            println!("x is not 0");
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("first is {} last is {}", first, last);
        }
    }


}
