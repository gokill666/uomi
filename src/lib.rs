use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    role: String,
    content: String,
}

#[no_mangle]
pub extern "C" fn process_message(input_ptr: *const u8, input_len: usize) -> *mut u8 {
    let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let msg: ChatMessage = serde_json::from_slice(input).unwrap_or(ChatMessage {
        role: "system".into(),
        content: "invalid".into(),
    });
    let response = ChatMessage {
        role: "agent".into(),
        content: format!("Hello from UOMI starter agent! Received: {}", msg.content),
    };
    let json = serde_json::to_vec(&response).unwrap();
    let boxed = json.into_boxed_slice();
    let ptr = boxed.as_ptr() as *mut u8;
    std::mem::forget(boxed);
    ptr
}
