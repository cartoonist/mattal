use rand;
use std::io;
use std::io::Write;

const NOFSYL: usize = 35;

fn josephus(n: usize, k: usize) -> usize {
    let mut position = 0;
    for i in 2..n + 1 {
        position = (position + k) % i;
    }
    return position + 1;
}

fn print_status(status: &Vec<bool>, kidx: usize, foot: bool) {
    for i in 0..status.len() {
        if i == kidx {
            print!("💀");
        } else if status[i] {
            print!("👻");
        } else if foot {
            print!("🦶");
        } else {
            print!("👤");
        }
    }
    println!();
}

fn print_header(n: usize) {
    print!("\t");
    for i in 0..(n / 10 + 1) {
        print!("{} ", i);
        for _ in 0..10 - i.to_string().len() {
            print!("  ");
        }
    }
    println!();
    print!("\t");
    for i in 0..n {
        print!("{} ", i % 10);
    }
    println!();
}

fn simulate(n: usize, s: usize, k: usize, foot: bool) -> usize {
    let mut status = vec![false; n];
    print_header(n);

    let mut j = s;
    for i in (1..n + 1).rev() {
        let mut cnt = k;
        while cnt != 0 {
            if !status[j] {
                cnt -= 1;
            }
            j = (j + 1) % n;
        }
        j = (j + n - 1) % n;
        status[j] = true;

        if i != 1 {
            print!("{}\t", j);
            print_status(&status, j, foot);
        }
    }
    return j;
}

fn play() {
    let mut players = Vec::new();

    // Read number of players
    let mut buffer = String::new();
    println!("Hi! I'm Usta! 👋🧓");
    print!(
        "I can’t quite see you properly without my glasses. \
            Could you help me count how many of you there are? "
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    println!("Great! Now, could you please tell me your names?");
    for i in 0..n {
        buffer.clear();
        print!("[Player {}]: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            players.push(format!("P{}", i+1).to_string());
        }
        else {
            players.push(buffer.trim().to_string());
        }
    }

    print!("Okay, ");
    for i in 0..n - 1 {
        print!("{}, ", &players[i]);
    }
    println!("and {}; let's start playing!", &players[n - 1]);

    let sidx = rand::random::<usize>() % n;
    println!("Starting from you, {}...", players[sidx]);
    let survived = simulate(n * 2, sidx * 2, NOFSYL, true);
    println!("Congratulations, {}! You survived!", players[survived / 2]);
}

fn help() {
    println!("Usage: mattal [command]\n");
    println!("Commands:");
    println!("\tplay\t\t\tStart a new game of Attal Mattal");
    println!("\trun <nplayers> <k>\tSimulate and solve the Josephus problem with n players and every k-th player eliminated");
    println!("\trun <min-n> <max-n> <k>\tSimulate and solve the Josephus problem with [min-n, max-n) players and every k-th player eliminated");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "play" {
        play();
        return;
    } else if args.len() == 4 && args[1] == "run" {
        let n = args[2].parse::<usize>().unwrap();
        let k = args[3].parse::<usize>().unwrap();
        println!("Josephus: {}", josephus(n, k));
        simulate(n, 0, k, false);
        return;
    } else if args.len() == 5 && args[1] == "run" {
        let n_lb = args[2].parse::<usize>().unwrap();
        let n_ub = args[3].parse::<usize>().unwrap();
        let k = args[4].parse::<usize>().unwrap();
        for n in n_lb..n_ub {
            println!("Josephus({}, {}): {}", n, k, josephus(n, k));
        }
    } else {
        help();
        std::process::exit(1);
    }
}
