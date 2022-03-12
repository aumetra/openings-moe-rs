#[tokio::main]
async fn main() {
    let all_openings = openings_moe::all().await.expect("Failed to fetch openings");

    for (idx, opening) in all_openings.iter().enumerate() {
        println!("Opening {}: {}", idx + 1, opening.source);
    }
}
