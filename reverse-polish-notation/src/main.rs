fn main() {
  let expected_exp = "6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6";
  let input_exp = String::from("6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -");
  let expected_val = 26.2840;

  match rpn(&input_exp) {
    Ok(val) => {
      debug_assert_eq!(format!("{:.4}", expected_val), format!("{:.4}", val));
      println!("{} = {}", expected_exp, val);
    },
    Err(msg) => println!("{}", msg),
  }
}

fn rpn(exp: &String) -> Result<f64, String> {
  let mut stack: Vec<f64> = Vec::new();
  let mut result = Err(String::from("No operation"));

  for token in exp.split_whitespace() {
    if let Ok(val) = token.parse::<f64>() {
      stack.push(val);
      result = Ok(val);
    } else {
      match token {
        "+" => apply2(&mut stack, |x, y| x + y),
        "-" => apply2(&mut stack, |x, y| x - y),
        "*" => apply2(&mut stack, |x, y| x * y),
        "/" => apply2(&mut stack, |x, y| x / y),
        _ => {
          result = Err(format!("Unknown token {}", token));
          break;
        },
      }
    }
  }

  if let Err(_) = result {
    result
  } else if let Some(ans) = stack.pop() {
    Ok(ans)
  } else {
    Err(String::from("Empty stack"))
  }
}

fn apply2<FnOP>(stack: &mut Vec<f64>, f_op: FnOP) where FnOP: Fn(f64, f64) -> f64 {
  if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
    stack.push(f_op(x, y));
  } else {
    panic!("no values");
  }
}