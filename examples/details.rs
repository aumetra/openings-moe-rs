use openings_moe::entity::list::Entry;

const LAIN_OPENING_UID: &str = "Opening1-SerialExperimentsLain";

#[tokio::main]
async fn main() {
    let entry = Entry {
        uid: LAIN_OPENING_UID.into(),
        ..Entry::default()
    };
    let details = entry.details().await.expect("Failed to fetch details");

    println!("Name: {}", details.data.title);
    for (idx, video_url) in details
        .videos()
        .expect("Failed to produce URL list")
        .into_iter()
        .enumerate()
    {
        println!("Video source #{}: {video_url}", idx + 1);
    }
}
