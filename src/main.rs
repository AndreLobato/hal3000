use rpi_led_matrix::{LedMatrix, LedMatrixOptions};
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_6X10}, pixelcolor::{Rgb888}, prelude::*, primitives::{Circle, PrimitiveStyleBuilder}, text::Text};
use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use tokio::{task};

struct HalState {
    pulse_speed: f32,
    text: String,
}

fn hall_eye(state: Arc<Mutex<HalState>>) -> () {
    println!("Spaning thread...");
    let mut options = LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_refresh_rate(false);
    options.set_hardware_mapping("adafruit-hat");
    options.set_parallel(1);
    options.set_scan_mode(1);
    let matrix = LedMatrix::new(Some(options), None).unwrap();
    let mut canvas = matrix.canvas();
    
    let mut t: f32 = 0.0;
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::BLUE);
    loop {
        canvas.clear();
        let brightness = (t.sin() * 128.0 + 128.0) as u8;
        let s = state.lock().unwrap();
        Circle::new(Point::new(22,6), 20)
            .into_styled(PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::new(brightness,0,0))
                .build())
            .draw(&mut canvas).unwrap();
        Text::new(&s.text, Point::new(26,16), style).draw(&mut canvas).unwrap();
        canvas = matrix.swap(canvas);
        t += 0.1 * s.pulse_speed;
    }

}
async fn say_hi(state: Arc<Mutex<HalState>>) {
    let mut counter = 0;
    loop{
        {
            let mut s = state.lock().unwrap();
            s.text = counter.to_string();
            counter += 1;
        }
        let _ = sleep(Duration::from_secs(3));
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(
        HalState{
            pulse_speed:0.5,
            text: String::new()
        }
    ));
    let state_clone = Arc::clone(&state);
    let eye_future = thread::spawn(|| hall_eye(state_clone));
    let say_future = task::spawn(say_hi(Arc::clone(&state)));
    eye_future.join().unwrap();
    let _ = tokio::join!(say_future);

}
