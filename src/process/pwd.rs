use rand::seq::{IndexedRandom, SliceRandom};

use crate::PwdOpts;

const UPPERCASE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER_CHARS: &[u8] = b"0123456789";
const SYMBOL_CHARS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

pub fn process_pwd(opts: PwdOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE_CHARS);
        password.push(
            *UPPERCASE_CHARS
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty"),
        );
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWERCASE_CHARS);
        password.push(
            *LOWERCASE_CHARS
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty"),
        );
    }

    if opts.numbers {
        chars.extend_from_slice(NUMBER_CHARS);
        password.push(
            *NUMBER_CHARS
                .choose(&mut rng)
                .expect("NUMBER won't be empty"),
        );
    }

    if opts.symbols {
        chars.extend_from_slice(SYMBOL_CHARS);
        password.push(
            *SYMBOL_CHARS
                .choose(&mut rng)
                .expect("SYMBOL won't be empty"),
        );
    }

    for _ in 0..(opts.length - password.len() as u8) {
        let choose = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*choose);
    }

    password.shuffle(&mut rng);

    println!("{}", String::from_utf8(password)?);

    Ok(())
}
