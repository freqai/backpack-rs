use backpack_client::ws_model::{WebsocketEvent, WebsocketEventUntag};
use serde::de::DeserializeOwned;
use serde_json::from_str;

fn process_message<WE: DeserializeOwned>(msg: &str) -> Result<WE, serde_json::Error> {
    let event: WE = from_str(msg)?;
    Ok(event)
}

fn main() {
    let json_data: &str = r#"{"data":{"E":1711899660078481,"S":"Bid","T":1711899660077996,"V":"RejectTaker","X":"New","Z":"0","e":"orderAccepted","f":"GTC","i":"112191056122806272","o":"LIMIT","p":"4.6771","q":"10.8","s":"WIF_USDC","t":null,"z":"0"},"stream":"account.orderUpdate"}"#;



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
