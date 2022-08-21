use std::env;

fn main() {
    println!("- Input trade size (USD)");
    println!("- R value (ex 1)");
    println!("- Risk percentage as float (ex 0.02)");
    println!("- Stop loss delta (USD)");
    println!("==> To get asset position size.");

    let args: Vec<String> = env::args().collect();

    let tradesize_input = &args[1];
    let rvalue_input = &args[2];
    let riskpercentage_input = &args[3];
    let sl_input = &args[4];

    let tradesize_input = tradesize_input.parse::<f32>().unwrap();
    let rvalue_input = rvalue_input.parse::<f32>().unwrap();
    let riskpercentage_input = riskpercentage_input.parse::<f32>().unwrap();
    let sl_input = sl_input.parse::<f32>().unwrap();
    println!("______________________________________________________________________________________________");
    println!("Trade size input: {}", tradesize_input);
    println!("R-value input: {}", rvalue_input);
    println!("Risk percentage input: {}", riskpercentage_input);
    println!("______________________________________________________________________________________________");

    let risk = tradesize_input*riskpercentage_input;

    println!("Total risk: {} USD", risk);

    let rvalue = rvalue_input*&risk;

    println!("Risk after R taken into consideration {} USD", rvalue);

    let position_size =  rvalue/sl_input;

    println!("Asset position size {}", position_size);
}