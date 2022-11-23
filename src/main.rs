// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/11/23 09:11:23 by Thibault Cheneviere                      \\
//   Updated: 2022/11/23 09:46:02 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game !!");

	let val = rand::thread_rng().gen_range(1..=100);
	
	loop {
		let res = core(val);

		if res {
			break;
		}
	}
	println!("Exiting game...");
}

fn core(val: u32) ->  bool {
	print!("Guess a number : ");
	io::stdout().flush().unwrap();

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Cannot read from stdin");

	let guess_num: u32 = guess.trim().parse().expect("Please type a number!");

	println!("You guessed {guess}");

	return compare(val, guess_num);
}

fn compare(val: u32, guess: u32) -> bool {
	match guess.cmp(&val) {
		Ordering::Less => println!("Too small"),
		Ordering::Greater => println!("Too big"),
		Ordering::Equal => {
			println!("Win !");
			return true;
		}
	}

	return false;
}
