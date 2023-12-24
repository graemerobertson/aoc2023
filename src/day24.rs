use std::fs::File;
use std::io::{self, BufRead, BufReader};
use z3::ast::Ast;
use z3::*;

#[derive(Debug)]
struct Hailstone {
    x: isize,
    y: isize,
    _z: isize,
    x_velocity: isize,
    y_velocity: isize,
    _z_velocity: isize,
}

// Looking for these two statements to be true for some t and t':
// h1.x + t*h1.x_velocity = h2.x + t'*h2.x_velocity
// h1.y + t*h1.y_velocity = h2.y + t'*h2.y_velocity
//
// Rearranging:
// (h1.x + t*h1.x_velocity - h2.x)/h2.x_velocity = t'
// (h1.y + t*h1.y_velocity - h2.y)/h2.y_velocity = t'
// (h1.x + t*h1.x_velocity - h2.x)/h2.x_velocity = (h1.y + t*h1.y_velocity - h2.y)/h2.y_velocity
// (h1.x + t*h1.x_velocity - h2.x)*h2.y_velocity = (h1.y + t*h1.y_velocity - h2.y)*h2.x_velocity
// h1.x*h2.y_velocity + t*h1.x_velocity*h2.y_velocity - h2.x*h2.y_velocity = h1.y*h2.x_velocity + t*h1.y_velocity*h2.x_velocity - h2.y*h2.x_velocity
// t*h1.x_velocity*h2.y_velocity - t*h1.y_velocity*h2.x_velocity = h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity
// t*(h1.x_velocity*h2.y_velocity - h1.y_velocity*h2.x_velocity) = h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity
// t = (h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity)/(h1.x_velocity*h2.y_velocity - h1.y_velocity*h2.x_velocity)
//
// So, let's find if that t exists, and then check a) if both t and t' are positive (i.e. in the
// future) and b) if the intersection point is within the bounds specified.
fn paths_intersect(h1: &Hailstone, h2: &Hailstone) -> bool {
    if h1.x_velocity * h2.y_velocity == h1.y_velocity * h2.x_velocity {
        return false;
    }
    let t: f64 = (h1.y * h2.x_velocity - h1.x * h2.y_velocity - h2.y * h2.x_velocity
        + h2.y_velocity * h2.x) as f64
        / (h1.x_velocity * h2.y_velocity - h1.y_velocity * h2.x_velocity) as f64;
    // Check if t is in the past
    if t < 0.0 {
        return false;
    }
    // Check if t' is in the past
    if ((h1.x as f64 + t * h1.x_velocity as f64 - h2.x as f64) / h2.x_velocity as f64) < 0.0 {
        return false;
    }
    let x = h1.x as f64 + t * h1.x_velocity as f64;
    let y = h1.y as f64 + t * h1.y_velocity as f64;
    (200000000000000.0..=400000000000000.0).contains(&x)
        && (200000000000000.0..=400000000000000.0).contains(&y)
}

// x + t1*x_velocity = 260346828765750 + t1*64
// y + t1*y_velocity = 357833641339849 + t1*-114
// z + t1*z_velocity = 229809969824403 + t1*106
// x + t2*x_velocity = 340220726383465 + t2*-79
// y + t2*y_velocity = 393110064924024 + t2*-61
// z + t2*z_velocity = 226146987100003 + t2*158
// x + t3*x_velocity = 11361697274707 + t3*328
// y + t3*y_velocity = 101596061919750 + t3*162
// z + t3*z_velocity = 46099495948720 + t3*333

// Variables are t1, t2, t3, x, y, z, x_velocity, y_velocity, z_velocity
// 260346828765750, 357833641339849, 229809969824403 @ 64, -114, 106
// 340220726383465, 393110064924024, 226146987100003 @ -79, -61, 158
// 11361697274707, 101596061919750, 46099495948720 @ 328, 162, 333
// t1*x_velocity - t1*h1.x_velocity = h1.x - x
// t1 = (h1.x - x)/(x_velocity - h1.x_velocity)
// t1 = (h1.y - y)/(y_velocity - h1.y_velocity)
// t1 = (h1.z - z)/(z_velocity - h1.z_velocity)
// (h1.x - x)/(x_velocity - h1.x_velocity) = (h1.y - y)/(y_velocity - h1.y_velocity)
// x = h1.x - (h1.y - y)*(x_velocity - h1.x_velocity)/(y_velocity - h1.y_velocity)
// (h2.x - x)/(x_velocity - h2.x_velocity) = (h2.y - y)/(y_velocity - h2.y_velocity)
// (h2.x - h1.x + (h1.y - y)*(x_velocity - h1.x_velocity)/(y_velocity - h1.y_velocity))/(x_velocity - h2.x_velocity) = (h2.y - y)/(y_velocity - h2.y_velocity)

pub(crate) fn day24() {
    let f: File = File::open("data/day24.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut hailstones: Vec<Hailstone> = vec![];
    for line in &lines {
        let split = line.split('@').collect::<Vec<&str>>();
        let positions = split[0]
            .split(',')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let velocities = split[1]
            .split(',')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        hailstones.push(Hailstone {
            x: positions[0],
            y: positions[1],
            _z: positions[2],
            x_velocity: velocities[0],
            y_velocity: velocities[1],
            _z_velocity: velocities[2],
        });
    }
    let mut part1_count: usize = 0;
    for (i, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(i + 1) {
            if paths_intersect(hailstone1, hailstone2) {
                part1_count += 1;
            }
        }
    }
    println!("Day 24 Part 1: {}", part1_count);

    // For part 2, just by looking at the first three hailstones we can generate 9 equations with
    // 9 unknowns (x, y, z are our starting position and x_velocity, y_velocity, z_velocity are
    // our velocities; t1, t2, and t3 are the times at which we intersect with the first three
    // hailstones):
    // x + t1*x_velocity = h1.x + t1*h1.x_velocity
    // y + t1*y_velocity = h1.y + t1*h1.y_velocity
    // z + t1*z_velocity = h1.z + t1*h1.z_velocity
    // x + t2*x_velocity = h2.x + t2*h2.x_velocity
    // y + t2*y_velocity = h2.y + t2*h2.y_velocity
    // z + t2*z_velocity = h2.z + t2*h2.z_velocity
    // x + t3*x_velocity = h3.x + t3*h3.x_velocity
    // y + t3*y_velocity = h3.y + t3*h3.y_velocity
    // z + t3*z_velocity = h3.z + t3*h3.z_velocity
    //
    // Solve these with the Z3 library. I've just hardcoded the values of the first three
    // hailstones.

    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);
    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");
    let t1 = ast::Int::new_const(&ctx, "t1");
    let t2 = ast::Int::new_const(&ctx, "t2");
    let t3 = ast::Int::new_const(&ctx, "t3");
    let x_velocity = ast::Int::new_const(&ctx, "x_velocity");
    let y_velocity = ast::Int::new_const(&ctx, "y_velocity");
    let z_velocity = ast::Int::new_const(&ctx, "z_velocity");

    // x + r*a = 260346828765750 + r*64
    let two_six_zero_three_dot_dot_dot = ast::Int::from_i64(&ctx, 260346828765750);
    let sixty_four = ast::Int::from_i64(&ctx, 64);
    solver.assert(
        &(&x + (&t1 * &x_velocity))._eq(&(&two_six_zero_three_dot_dot_dot + (&t1 * &sixty_four))),
    );
    // y + r*b = 357833641339849 - r*114
    let three_five_seven_eight_dot_dot_dot = ast::Int::from_i64(&ctx, 357833641339849);
    let one_one_four = ast::Int::from_i64(&ctx, 114);
    solver.assert(
        &(&y + (&t1 * &y_velocity))
            ._eq(&(&three_five_seven_eight_dot_dot_dot - (&t1 * &one_one_four))),
    );
    // z + r*c = 229809969824403 + r*106
    let two_two_nine_eight_dot_dot_dot = ast::Int::from_i64(&ctx, 229809969824403);
    let one_zero_six = ast::Int::from_i64(&ctx, 106);
    solver.assert(
        &(&z + (&t1 * &z_velocity))._eq(&(&two_two_nine_eight_dot_dot_dot + (&t1 * &one_zero_six))),
    );
    // x + s*a = 340220726383465 - s*79
    let three_four_zero_two_dot_dot_dot = ast::Int::from_i64(&ctx, 340220726383465);
    let seven_nine = ast::Int::from_i64(&ctx, 79);
    solver.assert(
        &(&x + (&t2 * &x_velocity))._eq(&(&three_four_zero_two_dot_dot_dot - (&t2 * &seven_nine))),
    );
    // y + s*b = 393110064924024 - s*61
    let three_nine_three_one_dot_dot_dot = ast::Int::from_i64(&ctx, 393110064924024);
    let six_one = ast::Int::from_i64(&ctx, 61);
    solver.assert(
        &(&y + (&t2 * &y_velocity))._eq(&(&three_nine_three_one_dot_dot_dot - (&t2 * &six_one))),
    );
    // z + s*c = 226146987100003 + s*158
    let two_two_six_one_dot_dot_dot = ast::Int::from_i64(&ctx, 226146987100003);
    let one_five_eight = ast::Int::from_i64(&ctx, 158);
    solver.assert(
        &(&z + (&t2 * &z_velocity))._eq(&(&two_two_six_one_dot_dot_dot + (&t2 * &one_five_eight))),
    );
    // x + t*a = 11361697274707 + t*328
    let one_one_three_six_dot_dot_dot = ast::Int::from_i64(&ctx, 11361697274707);
    let three_two_eight = ast::Int::from_i64(&ctx, 328);
    solver.assert(
        &(&x + (&t3 * &x_velocity))
            ._eq(&(&one_one_three_six_dot_dot_dot + (&t3 * &three_two_eight))),
    );
    // y + t*b = 101596061919750 + t*162
    let one_zero_one_five_dot_dot_dot = ast::Int::from_i64(&ctx, 101596061919750);
    let one_six_two = ast::Int::from_i64(&ctx, 162);
    solver.assert(
        &(&y + (&t3 * &y_velocity))._eq(&(&one_zero_one_five_dot_dot_dot + (&t3 * &one_six_two))),
    );
    // z + t*c = 46099495948720 + t*333
    let four_six_zero_nine_dot_dot_dot = ast::Int::from_i64(&ctx, 46099495948720);
    let three_three_three = ast::Int::from_i64(&ctx, 333);
    solver.assert(
        &(&z + (&t3 * &z_velocity))
            ._eq(&(&four_six_zero_nine_dot_dot_dot + (&t3 * &three_three_three))),
    );
    solver.check();
    let model = solver.get_model().unwrap();
    let res = model.eval(&(&x + &y + &z), true).unwrap();
    println!("Day 24 Part 2: {}", res);
}
