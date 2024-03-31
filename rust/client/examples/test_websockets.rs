use backpack_rs::ws_model::{WebsocketEvent, WebsocketEventUntag};
use serde::de::DeserializeOwned;
use serde_json::from_str;

fn process_message<WE: DeserializeOwned>(msg: &str) -> Result<WE, serde_json::Error> {
    let event: WE = from_str(msg)?;
    Ok(event)
}

fn main() {
    let json_data: &str = r#"{"data":{"A":"274.5","B":"272.0","E":1711867343981688,"T":1711867343980962,"a":"4.4030","b":"4.4029","e":"bookTicker","s":"WIF_USDC","u":161567336},"stream":"bookTicker.WIF_USDC"}"#;



    let result = process_message::<WebsocketEventUntag>(json_data);

    match result {
        Ok(parsed_event) => {
            // 使用 parsed_event
            println!("{:?}", parsed_event);
        }
        Err(e) => {
            // 错误处理
            println!("An error occurred: {}", e);
        }
    }

}
