use std::error::Error;

use async_recursion::async_recursion;
use demand::{DemandOption, Select};

use crate::raydium::{
    atas::wrap_sol::sol_wrap, bundler::pool_main, distribution::sol_distribution::distributor,
    lut::extend_lut::lut_main, wallets::wallets_main,
};

#[async_recursion]
pub async fn app() -> Result<(), Box<dyn Error>> {
    // let theme = theme();
    let ms = Select::new("Minter Mode")
        .description("Select the Mode")
        .filterable(true)
        .option(DemandOption::new("Generate Wallets").label("▪ Generate New Wallets"))
        .option(DemandOption::new("CreateLUT").label("▪ Create LUT"))
        .option(DemandOption::new("Distribute SOL").label("▪ Distribute SOL"))
        .option(DemandOption::new("Wrap SOL & ATAs").label("▪ Wrap SOL & ATAs"))
        .option(DemandOption::new("multi-Liquidity").label("▪ Bundle Liquidity"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Generate Wallets" => {
            let _ = wallets_main().await;
        }
        "CreateLUT" => {
            let _ = lut_main().await;
        }
        "Distribute SOL" => {
            let _ = distributor().await;
        }
        "Wrap SOL & ATAs" => {
            let _ = sol_wrap().await;
        }
        "multi-Liquidity" => {
            let _ = pool_main().await;
        }
        _ => {}
    }

    let _ = app().await;
    Ok(())
}
