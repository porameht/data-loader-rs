use polars::prelude::*;
use loader::load_dataset;

mod loader;
fn main() -> anyhow::Result<()> {
    load_dataset("pythainlp/thainer-corpus-v2")?;
    // let args = ScanArgsParquet::default();
    // // Define dataset splits
    // let splits = std::collections::HashMap::from([
    //     ("train", "data/train-00000-of-00001-0daf377cbb9f6602.parquet"),
    //     ("validation", "data/validation-00000-of-00001-9f3b8ecdcef01487.parquet"), 
    //     ("test", "data/test-00000-of-00001-a463bba0cd720662.parquet")
    // ]);

    // // Read the training dataset
    // let dataset_path = format!("hf://datasets/pythainlp/thainer-corpus-v2/{}", splits["train"]);

    // let lf = LazyFrame::scan_parquet(dataset_path, args).unwrap();

    // // Print the first 5 rows of the dataset
    // let lf = lf.limit(5);

    // let df = lf.collect().unwrap();
    // println!("First 5 rows of the dataset:\n{}", df);


    Ok(())
}
