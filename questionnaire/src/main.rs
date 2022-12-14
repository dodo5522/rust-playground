use std::{collections::HashMap, hash::Hash};

fn questionnaire() {
    const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";
    let mut h = HashMap::new();

    h.insert("A", 0);
    h.insert("B", 0);
    h.insert("C", 0);

    for w in V_DATA.split(',') {
        h.insert(w ,h[w] + 1);
    }

    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, h[k]);
    }
}

fn tukimei() {
    let tuki = ["睦月", "如月", "弥生", "卯月", "皐月", "水無月", "文月", "葉月", "長月", "神無月", "霜月", "師走"];
    let mut tuki_map: HashMap<&str, u32> = HashMap::new();

    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, (i + 1) as u32);
    }

    println!("{}: {}月", "神無月", tuki_map["神無月"]);
}

fn main() {
    //questionnaire();
    tukimei();
}