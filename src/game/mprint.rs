use colored::Colorize;

#[allow(dead_code)]
pub fn info(a: String) {
    println!("📔 {} : {}", "Info".purple().bold(), a);
}

#[allow(dead_code)]
pub fn success(a: String) {
    println!("✅ {} : {}", "Success".green().bold(), a);
}

#[allow(dead_code)]
pub fn error(a: String) {
    println!("☠️ {} : {}", "Error".red().bold(), a);
}

#[allow(dead_code)]
pub fn warning(a: String) {
    println!("⚠️ {} : {}", "Warning".yellow().bold(), a);
}
