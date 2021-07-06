#[allow(dead_code)]
pub fn print_move(from: &str, to: &str) {
    println!("move {} to {}", from, to)
}

#[allow(dead_code)]
pub fn hanoi_recursive(n: u32, from: &str, to: &str, via: &str, movefn: fn(from: &str, to: &str)) {
    if n == 1 {
        movefn(from, to);
    } else {
        hanoi_recursive(n - 1, from, via, to, movefn);
        hanoi_recursive(1, from, to, via, movefn);
        hanoi_recursive(n - 1, via, to, from, movefn);
    }
}

#[allow(dead_code)]
pub fn hanoi_simunator<'a>(n: u32, from: &'a str, to: &'a str, via: &'a str) {
    struct Frame<'a> {
        pc: u8,
        n: u32,
        from: &'a str,
        to: &'a str,
        via: &'a str,
    }

    impl Frame<'_> {
        fn set_pc(&mut self, pc: u8) {
            self.pc = pc;
        }
    }

    let mut top = 0;
    let mut stacks = Vec::with_capacity(64);
	for _ in 0..64 {
		let f = Frame{pc: 0, n: 0, from: "", to: "", via: ""};
		stacks.push(f);
	}

    macro_rules! call {
        ($n: expr, $from: expr, $to: expr, $via: expr) => {{
            stacks[top] = Frame {
                pc: 0,
                n: $n,
                from: $from,
                to: $to,
                via: $via,
            };
            top += 1
        }};
    }
    macro_rules! ret {
        () => {
            top -= 1
        };
    }

    call!(n, from, to, via);

    while top > 0 {
        let f = &mut stacks[top - 1];
		let pc = f.pc;
        f.set_pc(f.pc+1);
        match pc {
            0 => {
                if f.n == 1 {
                    println!("move {} to {}", f.from, f.to);
					f.set_pc(4);
                }
            }
            1 => call!(f.n - 1, f.from, f.via, f.to),
            2 => call!(1, f.from, f.to, f.via),
            3 => call!(f.n-1, f.via, f.to, f.from),
            _ => ret!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_recursive() {
        hanoi_recursive(3, "A", "B", "C", print_move)
    }

    #[test]
    fn test_hanoi_simunator() {
        hanoi_simunator(30, "A", "B", "C")
    }
}
