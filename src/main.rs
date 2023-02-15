use clap::Parser;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    target: i32,
    numbers: Vec<i32>,
}

#[derive(Debug, PartialEq, EnumIter)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Expression {
    expr: String,
    value: i32,
}

fn main() {
    let args = Args::parse();

    println!("Countdown, target: {}", args.target);

    let eq = Expression {
        expr: "".to_string(),
        value: 0,
    };

    let sol = find_solution(args.target, &args.numbers, eq);

    if sol.is_some() {
        println!("Found a solution! Try {}", sol.unwrap().expr);
    }
}

fn find_solution(target: i32, remaining: &Vec<i32>, expr: Expression) -> Option<Expression> {
    for (idx, val) in remaining.into_iter().enumerate() {
        let new_remaining: Vec<i32> = remaining
            .into_iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| i.to_owned() != idx)
            .map(|(_, x)| x)
            .collect();

        for op in Operation::iter() {
            if op == Operation::Divide {
                if expr.value % val != 0 {
                    continue;
                }
            }

            let (new_value, new_expr) = match op {
                Operation::Add => (expr.value + val, format!("({}{}{})", expr.expr, '+', val)),
                Operation::Subtract => (expr.value - val, format!("({}{}{})", expr.expr, '-', val)),
                Operation::Multiply => (expr.value * val, format!("({}{}{})", expr.expr, '*', val)),
                Operation::Divide => (expr.value / val, format!("({}{}{})", expr.expr, '/', val)),
            };

            let new_expr = Expression {
                expr: new_expr,
                value: new_value,
            };

            if new_value == target {
                return Some(new_expr);
            } else {
                if let Some(solution) = find_solution(target, &new_remaining, new_expr) {
                    return Some(solution);
                }
            }
        }
    }

    return None;
}
