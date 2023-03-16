use rand::Rng;

const inferior_limit: u8 = 0; // li
const superior_limit: u8 = 2; // ls
const number_of_individuals: u16 = 500; // nc
const number_of_generations: u8 = 50; // ng
const probability_crossover: f32 = 0.3; // pincrucisare
const probability_mutation: f32 = 0.1; // pmutatie
const kek: u8 = 0;
static mut fmax: FUnassigned = 0.0;
static mut xmax: FUnassigned = 0.0;

// TODO - Check all types and replace with standard types if correct
type Unassigned = i64;
type FUnassigned = f64;
type ArrUnassigned<'a> = &'a [i64];
type FArrUnassigned<'a> = &'a [f64];

fn f(x: FUnassigned) -> FUnassigned {
    return x.sin();
}

fn x(v: Unassigned) -> FUnassigned {
    return (inferior_limit as Unassigned
        + v / 65535 * (superior_limit - inferior_limit) as Unassigned) as FUnassigned;
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
fn crossover(c: ArrUnassigned, p1: Unassigned, p2: Unassigned) {
    let v1 = c[p1 as usize];
    let v2 = c[p2 as usize];
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=16);

    for j in random_number..=16 {
        if get_bit(v2, j as Unassigned) == 1 {
            set_bit(c[p1 as usize], j as Unassigned);
        } else {
            reset_bit(c[p1 as usize], j as Unassigned);
        }
        if get_bit(v1, j as Unassigned) == 1 {
            set_bit(c[p2 as usize], j as Unassigned);
        } else {
            reset_bit(c[p2 as usize], j as Unassigned);
        }
    }
}

fn mutation(c: ArrUnassigned, p: Unassigned) {
    let cp = c[p as usize];
    for j in 1..=16 {
        let mut rng = rand::thread_rng();
        let random_number: FUnassigned = rng.gen_range(0.0..=1.0);
        if probability_mutation > random_number as f32 {
            if get_bit(cp, j) == 1 {
                reset_bit(cp, j);
            } else {
                set_bit(cp, j);
            }
        }
    }
    if f(x(c[p as usize])) > f(x(c[p as usize])) {
        c[p as usize] = cp
    }
}

fn initialization(c: ArrUnassigned) {
    let mut rng = rand::thread_rng();
    for i in 1..=number_of_generations {
        let random_number: Unassigned = rng.gen_range(0..=65536);
        c[i as usize] = random_number;
    }
    unsafe {
        xmax = x(c[1]);
        fmax = f(xmax);
    }
}

fn selection(c: ArrUnassigned) {
    let c_prime: ArrUnassigned;
    let s: FUnassigned = 0.0;
    let v: FArrUnassigned;
    let p: FArrUnassigned;
    for i in 1..=number_of_individuals {
        v[i as usize] = f(x(c[i as usize]));
        s = s + v[i as usize];
    }
    p[1] = v[1] / s;
    for i in 2..=number_of_individuals {
        p[i as usize] = p[(i - 1) as usize] + v[i as usize] / s;
    }
    for i in 1..=number_of_individuals {
        let rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0..=1.0);
        for j in 1..=number_of_individuals {
            if random_number > p[j as usize] && random_number <= p[(j + 1) as usize] {
                c_prime[i as usize] = c[j as usize];
            }
        }
        for i in 1..=number_of_generations {
            c[i as usize] = c_prime[i as usize];
        }
    }
}

fn recombination(c: ArrUnassigned) {
    let p1: Unassigned;
    let p2: Unassigned;
    let first = true;
    let rng = rand::thread_rng();
    for i in 1..=number_of_individuals {
        let random_number = rng.gen_range(0.0..=1.0);
        if random_number < probability_crossover {
            if first {
                first = false;
                p1 = i as Unassigned;
            } else {
                first = true;
                p2 = i as Unassigned;
                crossover(c, p1, p2);
            }
        }
    }
}

fn modification(c: ArrUnassigned) {
    for i in 1..=number_of_individuals {
        mutation(c, i as Unassigned);
    }
}

unsafe fn max(generation_number: Unassigned) {
    let c: ArrUnassigned;
    initialization(c);
    for t in 1..number_of_generations {
        selection(c);
        recombination(c);
        modification(c);
        let maxiteri: FUnassigned = 1.0;
        let maxiterf = f(x(c[1]));
        for i in 2..=number_of_individuals {
            if f(x(c[i as usize])) > maxiterf {
                maxiteri = i as FUnassigned;
                maxiterf = f(x(c[i as usize]));
            }
        }
        if maxiterf > fmax {
            fmax = maxiterf;
            xmax = x(c[maxiteri]);
            print!("{}, {}", xmax, fmax);
        }
    }
}

fn main() {
    unsafe {
        max(number_of_generations as Unassigned);
    }
    // x(v);
    // get_bit(v, i);
    // const inferior_limit: u8 = 0;
    // const superior_limit: u8 = 2;
    // const number_of_individuals: u16 = 500;
    // const number_of_generations: u8 = 50;
    // const probability_crossover: f32 = 0.3;
    // const probability_mutation: f32 = 0.1;
}
