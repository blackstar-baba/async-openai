use std::error::Error;

use async_openai::Client;
use async_openai::config::OpenAIConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://api.deepseek.com/v1";
    let key = "sk-358ba0d8445148f2bce187e87918c23e";

    let config = OpenAIConfig::new().with_api_base(url).with_api_key(key);
    let client = Client::with_config(config);

    // let client = Client::new(config);

    let model_list = client.models().list().await?;

    println!("List of models:\n {:#?}", model_list.data);

    // let model = client.models().retrieve("gpt-3.5-turbo-instruct").await?;
    // println!("gpt-3.5-turbo-instruct model id: {}", model.id);

    Ok(())
}
