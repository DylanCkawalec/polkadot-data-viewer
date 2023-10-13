use colored::*;// Import the colored crate

mod collect;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://rpc.polkadot.io";

    utils::fetch::fetch_finalized_block_hash(url).await?;
    let hash = collect::fetched::get_finalized_block_hash();

    if let Some(ref hash) = hash {
        utils::fetch::fetch_and_print_block_header(url, hash).await?;
        utils::fetch::fetch_and_print_extrinsics(url, hash).await?;
        
        // Assuming you have the extrinsic as a string.
        if let Some(extrinsic) = collect::fetched::get_extrinsics() {
            utils::fetch::submit_extrinsic(url, &extrinsic).await?;
        }
    }
   
    let block_header = collect::fetched::get_block_header();
    let extrinsics = collect::fetched::get_extrinsics();
    let extrinsic_hash = collect::fetched::get_extrinsic_hash();

    utils::fetch::fetch_and_set_chain(url).await?;
    let chain = collect::fetched::get_chain();

    println!("{}", "Block Hash:".blue().bold());
    println!(
        "{}",
        hash.unwrap_or_else(|| "None".to_string())
    );
    println!("{}", "Block Header:".green().bold());
    println!(
        "{}",
        block_header.unwrap_or_else(|| "None".to_string())
    );
    
    println!("{}", "Extrinsics:".red().bold());
    println!(
        "{}",
        extrinsics.unwrap_or_else(|| "None".to_string())
    );
    println!("{}", "Extrinsic Hash:".magenta().bold());
    println!(
        "{}",
        extrinsic_hash.unwrap_or_else(|| "None".to_string())
    );
    println!("{}", "Chain:".magenta().bold()); // Using magenta for a pinkish color
    println!(
        "{}",
        chain.unwrap_or_else(|| "None".to_string())
    );
    // Using colored output for the headings
    
    Ok(())
}
