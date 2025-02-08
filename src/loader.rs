use hf_hub::api::sync::Api;
use std::path::PathBuf;
use std::fs;

const DATA_DIR: &str = "datasets";

pub fn load_dataset(dataset_name: &str) -> anyhow::Result<()> {
    let file_path = format!("{}/{}", DATA_DIR, dataset_name);
    fs::create_dir_all(&file_path).map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;

    let api = Api::new()?;
    let repo = api.dataset(dataset_name.to_string());

    let info = repo.info().map_err(|e| anyhow::anyhow!("Failed to fetch dataset info: {}", e))?;
    
    println!("Found {} files to download", info.siblings.len());
    
    for (index, sibling) in info.siblings.iter().enumerate() {
        let filename = sibling.rfilename.split('/').last().unwrap_or(&sibling.rfilename);
        let local_path = PathBuf::from(&file_path).join(filename);
        
        if local_path.exists() {
            println!("[{}/{}] Skipping existing file: {}", index + 1, info.siblings.len(), filename);
            continue;
        }
        
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent).map_err(|e| anyhow::anyhow!("Failed to create parent directory: {}", e))?;
        }

        let cached_path = repo.get(&sibling.rfilename)?;
        fs::copy(&cached_path, &local_path)
            .map_err(|e| anyhow::anyhow!("Failed to copy file {}: {}", filename, e))?;
            
        println!("[{}/{}] Downloaded: {}", index + 1, info.siblings.len(), filename);
    }

    println!("All files downloaded successfully!");
    Ok(())
}