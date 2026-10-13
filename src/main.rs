//! Versión 1.0
//! Cálculo del número π con precisión arbitraria.
//!
//! Fórmula: π = 16 × arctan(1/5) − 4 × arctan(1/239)
//!
//! Compilación:
//!     cargo build --release
//!
//! Calcular SHA-256:
//!     sha256sum ./target/release/pi-constant.exe
//!
//! Uso:
//!     pi-constant.exe [decimales]
//!
//! Ejemplos:
//!     pi-constant.exe
//!     pi-constant.exe 20
//!     pi-constant.exe 1000

use std::cmp::Ordering;
use std::env;
use std::fmt::Write as _;
use std::process;

const BASE: u64 = 1_000_000_000;
const DEFAULT_DECIMALS: usize = 100;

#[derive(Clone)]
struct BigUint {
    // Bloques little-endian en base 10^9.
    limbs: Vec<u64>,
}

impl BigUint {
    fn zero() -> Self {
        Self { limbs: vec![0] }
    }

    fn power_of_ten(exponent: usize) -> Self {
        let whole_limbs = exponent / 9;
        let remaining_digits = exponent % 9;
        let mut limbs = vec![0; whole_limbs + 1];
        limbs[whole_limbs] = 10_u64.pow(remaining_digits as u32);
        Self { limbs }
    }

    fn normalize(&mut self) {
        while self.limbs.len() > 1 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    fn is_zero(&self) -> bool {
        self.limbs.len() == 1 && self.limbs[0] == 0
    }

    fn compare(&self, other: &Self) -> Ordering {
        match self.limbs.len().cmp(&other.limbs.len()) {
            Ordering::Equal => self
                .limbs
                .iter()
                .rev()
                .cmp(other.limbs.iter().rev()),
            ordering => ordering,
        }
    }

    fn add_assign(&mut self, other: &Self) {
        if self.limbs.len() < other.limbs.len() {
            self.limbs.resize(other.limbs.len(), 0);
        }

        let mut carry = 0_u64;
        for index in 0..self.limbs.len() {
            let rhs = other.limbs.get(index).copied().unwrap_or(0);
            let value = self.limbs[index] + rhs + carry;
            self.limbs[index] = value % BASE;
            carry = value / BASE;
        }

        if carry != 0 {
            self.limbs.push(carry);
        }
    }

    fn subtract_assign(&mut self, other: &Self) {
        assert!(self.compare(other) != Ordering::Less);
        let mut borrow = 0_i64;

        for index in 0..self.limbs.len() {
            let lhs = self.limbs[index] as i64 - borrow;
            let rhs = other.limbs.get(index).copied().unwrap_or(0) as i64;
            if lhs < rhs {
                self.limbs[index] = (lhs + BASE as i64 - rhs) as u64;
                borrow = 1;
            } else {
                self.limbs[index] = (lhs - rhs) as u64;
                borrow = 0;
            }
        }

        debug_assert_eq!(borrow, 0);
        self.normalize();
    }

    fn multiply_assign(&mut self, factor: u64) {
        let mut carry = 0_u128;
        for limb in &mut self.limbs {
            let value = *limb as u128 * factor as u128 + carry;
            *limb = (value % BASE as u128) as u64;
            carry = value / BASE as u128;
        }

        while carry != 0 {
            self.limbs.push((carry % BASE as u128) as u64);
            carry /= BASE as u128;
        }
        self.normalize();
    }

    fn divide_assign(&mut self, divisor: u64) {
        let mut remainder = 0_u128;
        for limb in self.limbs.iter_mut().rev() {
            let value = remainder * BASE as u128 + *limb as u128;
            *limb = (value / divisor as u128) as u64;
            remainder = value % divisor as u128;
        }
        self.normalize();
    }

    fn to_decimal_string(&self) -> String {
        let mut limbs = self.limbs.iter().rev();
        let mut result = limbs.next().unwrap_or(&0).to_string();
        for limb in limbs {
            write!(&mut result, "{limb:09}").expect("escribir en String no puede fallar");
        }
        result
    }
}

fn arctangent_inverse(scale: &BigUint, inverse: u64) -> Result<BigUint, String> {
    // atan(1/x) = 1/x - 1/(3*x^3) + 1/(5*x^5) - ...
    let inverse_squared = inverse
        .checked_mul(inverse)
        .ok_or("desbordamiento al preparar la serie")?;
    let mut power = scale.clone();
    power.divide_assign(inverse);
    let mut result = BigUint::zero();
    let mut index = 0_usize;

    loop {
        let denominator = index
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or("la precisión solicitada es demasiado grande")?;
        let denominator = u64::try_from(denominator)
            .map_err(|_| "la precisión solicitada es demasiado grande")?;

        let mut term = power.clone();
        term.divide_assign(denominator);
        if term.is_zero() {
            break;
        }

        if index % 2 == 0 {
            result.add_assign(&term);
        } else {
            result.subtract_assign(&term);
        }

        power.divide_assign(inverse_squared);
        index = index
            .checked_add(1)
            .ok_or("la precisión solicitada es demasiado grande")?;
    }

    Ok(result)
}

fn calculate_pi(decimals: usize) -> Result<String, String> {
    // Machin: pi = 16*atan(1/5) - 4*atan(1/239).
    // Las cifras de guarda absorben los truncamientos de la aritmética entera.
    let guard_digits = 32_usize
        .checked_add(decimals.to_string().len())
        .ok_or("el número de decimales es demasiado grande")?;
    let precision = decimals
        .checked_add(guard_digits)
        .ok_or("el número de decimales es demasiado grande")?;
    let scale = BigUint::power_of_ten(precision);

    let mut first = arctangent_inverse(&scale, 5)?;
    first.multiply_assign(16);
    let mut second = arctangent_inverse(&scale, 239)?;
    second.multiply_assign(4);
    first.subtract_assign(&second);

    let expected_length = precision
        .checked_add(1)
        .ok_or("el número de decimales es demasiado grande")?;
    let mut digits = first.to_decimal_string();
    if digits.len() < expected_length {
        digits.insert_str(0, &"0".repeat(expected_length - digits.len()));
    }

    let kept_length = decimals
        .checked_add(1)
        .ok_or("el número de decimales es demasiado grande")?;
    let round_up = digits
        .as_bytes()
        .get(kept_length)
        .is_some_and(|digit| *digit >= b'5');
    digits.truncate(kept_length);

    if round_up {
        let mut bytes = digits.into_bytes();
        let mut index = bytes.len();
        loop {
            if index == 0 {
                bytes.insert(0, b'1');
                break;
            }
            index -= 1;
            if bytes[index] == b'9' {
                bytes[index] = b'0';
            } else {
                bytes[index] += 1;
                break;
            }
        }
        digits = String::from_utf8(bytes).expect("solo se generan cifras ASCII");
    }

    if decimals == 0 {
        Ok(digits)
    } else {
        digits.insert(1, '.');
        Ok(digits)
    }
}

fn print_usage(program: &str) {
    println!(
        "Uso: {program} [DECIMALES]\n\n\
         Calcula pi redondeado al número indicado de decimales.\n\
         Si se omite DECIMALES, se calculan {DEFAULT_DECIMALS}.\n\n\
         Ejemplos:\n  {program} 50\n  {program} 1000"
    );
}

fn run() -> Result<(), String> {
    let mut arguments = env::args();
    let program = arguments
        .next()
        .unwrap_or_else(|| "pi-constant".to_owned());
    let first = arguments.next();

    if matches!(first.as_deref(), Some("-h" | "--help")) {
        print_usage(&program);
        return Ok(());
    }

    if arguments.next().is_some() {
        return Err(format!("demasiados argumentos\nPrueba «{program} --help»"));
    }

    let decimals = match first {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| format!("«{value}» no es un entero no negativo válido"))?,
        None => DEFAULT_DECIMALS,
    };

    println!("{}", calculate_pi(decimals)?);
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("Error: {message}");
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_pi;

    #[test]
    fn known_digits_are_correct() {
        assert_eq!(calculate_pi(0).unwrap(), "3");
        assert_eq!(calculate_pi(5).unwrap(), "3.14159");
        assert_eq!(calculate_pi(20).unwrap(), "3.14159265358979323846");
    }
}
