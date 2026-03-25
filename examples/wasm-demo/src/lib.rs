use wasm_bindgen::prelude::*;

/// Returns a JSON string with title, format description, and total_time_ms.
#[wasm_bindgen]
pub fn module_info(data: &[u8]) -> Result<String, JsValue> {
    let mut player = oxdz::Oxdz::new(data, 44100, "")
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let mut mi = oxdz::ModuleInfo::new();
    player.module_info(&mut mi);
    Ok(format!(
        r#"{{"title":"{}","format":"{}","total_time_ms":{}}}"#,
        mi.title.trim().replace('"', "\\\""),
        mi.description.replace('"', "\\\""),
        mi.total_time
    ))
}

/// Renders up to max_seconds of audio, stopping at the first loop.
/// Returns interleaved stereo f32 samples at 44100 Hz (channel 0, channel 1, ...).
#[wasm_bindgen]
pub fn render_module(data: &[u8], max_seconds: u32) -> Result<Box<[f32]>, JsValue> {
    let rate = 44100u32;
    let mut player = oxdz::Oxdz::new(data, rate, "")
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    let mut fi = oxdz::FrameInfo::new();
    let max_time_ms = (max_seconds * 1000) as f32;
    let mut out: Vec<f32> = Vec::new();

    loop {
        player.frame_info(&mut fi);
        if fi.loop_count > 0 || fi.time > max_time_ms {
            break;
        }
        player.play_frame();
        for &s in player.buffer() {
            out.push(s as f32 / 32768.0);
        }
    }

    Ok(out.into_boxed_slice())
}
