use loader::load_dataset;

mod loader;
fn main() -> anyhow::Result<()> {
    load_dataset("pythainlp/thainer-corpus-v2")?;
    Ok(())
}
