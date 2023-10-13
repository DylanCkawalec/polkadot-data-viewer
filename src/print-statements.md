```
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
```