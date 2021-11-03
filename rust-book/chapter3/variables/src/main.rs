fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    
    shadowing(x);

    let res  = returning_values(x,2);
    println!("the return value is: {}", res);

    flow_pa(res);
    foreando(res);
    println!("la sucesion de fibo hasta el res termino es: {}",
             fibo(res as u32));
}

fn foreando(x: i32) {
    for i in 0..x {
         println!("vamos por la iter {}",i);
    }
}

fn shadowing(x:i32) {
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}


fn returning_values(mut x: i32,mut y: i32) -> i32 {
    let mut c = x + y;
    x = y * 2;
    y = x + 2;
    println!("c before {}", c);
    c = x + y;
    c
}


fn flow_pa(x:i32) {
    if x % 2 == 0 {
        println!("es par pa!");
    } else {
        println!("no era par pa!");
    }
}


fn fibo(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
