fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 1u8;

    println!("\nfib_loop:");
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", a);
        if i > n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 1u8);
    println!("\nfib_while:");
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", a);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    println!("\nfib_for:");
    for _i in 1..=n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", a);
    }
}
fn main() {
    let n = 4;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
