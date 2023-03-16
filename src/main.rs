const inferior_limit: u8 = 0;
const superior_limit: u8 = 2;
const number_of_individuals: u16 = 500;
const number_of_generations: u8 = 50;
const probability_crossover: f32 = 0.3;
const probability_mutation: f32 = 0.1;

type Unassigned = i64;

fn x(v: Unassigned) -> Unassigned {
    return (inferior_limit as Unassigned
        + v / 65535 * (superior_limit - inferior_limit) as Unassigned) as Unassigned;
}

fn get_bit(v: Unassigned, i: Unassigned) -> Unassigned {
    return v >> i & 1;
}

fn set_bit(v: Unassigned, i: Unassigned) -> Unassigned {
    return v | (1 << i);
}

fn reset_bit(v: Unassigned, i: Unassigned) -> Unassigned {
    return v & !(1 << i);
}

/* Incrucisare */
fn crossover(c: Unassigned, p1: Unassigned, p2: Unassigned) {
    let V1 = c[p1];
    let V2 = c[p2];
    let r = 
}

fn main() {
    // x(v);
    // get_bit(v, i);
    // const inferior_limit: u8 = 0;
    // const superior_limit: u8 = 2;
    // const number_of_individuals: u16 = 500;
    // const number_of_generations: u8 = 50;
    // const probability_crossover: f32 = 0.3;
    // const probability_mutation: f32 = 0.1;
}
