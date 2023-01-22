use proconio::input;

fn main() {
	input! {
		n: u32,
		s: [String; n],
	}

	let ss: Vec<String> = s.into_iter().rev().collect();

	for w in ss {
		println!("{}", w);
	}
}
