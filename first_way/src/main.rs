mod config;

use crate::config::CONFIG;
use serde_json::{json, Value};
use std::thread::sleep;
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error, Response};

fn main() {
    let agent = AgentBuilder::new().max_idle_connections(1).build();

    //通知初始化成功
    //notify scf ready
    let _ = post_data(&CONFIG.ready_url, &json!({ "msg": "ready"}), &agent);

    loop {
        //循环读取事件
        //read event from scf
        match event_get(&CONFIG.event_url, &agent) {
            Ok(response) => {
                //获取到事件通知
                //get event notify
                if let Err(err) = process_event(response, &agent) {
                    let err_res = post_data(
                        &CONFIG.error_url,
                        &json!({ "msg": format!("{:?}", err) }),
                        &agent,
                    );
                    println!("event process error {:?} res={:?}", err, err_res);
                }
            }
            Err(err) => {
                let err_res = post_data(
                    &CONFIG.error_url,
                    &json!({ "msg": format!("{:?}", err) }),
                    &agent,
                );
                println!("event reading error {:?} res={:?}", err, err_res);
                sleep(Duration::from_millis(100));
            }
        }
    }
}

//处理逻辑业务
//deal with event
fn process_event(response: Response, agent: &Agent) -> Result<(), Error> {
    let event: Value = response.into_json().unwrap();
    if let Ok(v) = event_get(event["url"].as_str().unwrap(), agent) {
        let resp: Value = v.into_json().unwrap();
        let data = post_data(&CONFIG.response_url, &resp, agent).unwrap();
        println!("Invoke response: {}", data);
    } else {
    };
    Ok(())
}

fn post_data(url: &str, data: &Value, agent: &Agent) -> Result<String, Error> {
    let resp = agent
        .post(url)
        .send_json(data)
        .unwrap()
        .into_string()
        .unwrap();
    Ok(format!("{}",resp))
}

fn event_get(url: &str, agent: &Agent) -> Result<Response, Error> {
    agent.get(url).call()
}
