use std::io::stdin;

struct BmiRange {
    min: f32,
    max: f32,
    label: String,
}

struct Human {
    height_cm: f32,
    weight_kg: f32,
}

fn main() {
    let height= input("身長(cm) -> ");
    let weight= input("体重(kg) -> ");
    let human = Human{height_cm: height, weight_kg: weight};
    let result = get_bmi(&human);
    println!("{:.2}cm, {:.2}kg, bmi {:.2}, {}", human.height_cm, human.weight_kg, result.0, result.1);
}

fn get_bmi(human: &Human) -> (f32, String) {
    let bmi_list = [
        BmiRange{min: 0.0,  max: 18.5, label: "低体重".to_string()},
        BmiRange{min: 18.5, max: 25.0, label: "普通体重".to_string()},
        BmiRange{min: 25.0, max: 30.0, label: "肥満1度".to_string()},
        BmiRange{min: 30.0, max: 35.0, label: "肥満2度".to_string()},
        BmiRange{min: 35.0, max: 40.0, label: "肥満3度".to_string()},
        BmiRange{min: 40.0, max: 45.0, label: "肥満4度".to_string()},
    ];
    let height_m = human.height_cm / 100.0;
    let bmi = human.weight_kg / height_m.powf(2 as f32);

    let mut label = "out of range".to_string();
    for item in bmi_list.iter() {
        if item.min <= bmi && bmi < item.max {
            label = item.label.clone();
            break;
        }
    }

    (bmi, label)
}

fn input(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut in_string = String::new();
    stdin().read_line(&mut in_string).expect("");
    let res = in_string.trim().parse::<f32>();

    match res {
        Ok(value) => value,
        Err(_) => panic!("小数を入力してください"),
    }
}
