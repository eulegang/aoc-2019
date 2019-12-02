use utils::lines;

fn main() {
    let weights: Vec<i32> = lines();
    let mut total = 0;

    for req in weights.iter().map(|weight| req_req(*weight)) {
        total += req;
    }

    println!("total requirement: {}", total)
}

fn fuel_req(weight: i32) -> i32 {
    (weight / 3) - 2
}

fn req_req(weight: i32) -> i32 {
    let mut total = 0;
    let mut sub_weight = fuel_req(weight);
    while sub_weight >= 0 {
        total += sub_weight;
        sub_weight = fuel_req(sub_weight);
    }

    total
}
