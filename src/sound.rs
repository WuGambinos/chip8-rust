use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::time::Duration;


pub fn beep() {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source for the sake of the example
    let source = SineWave::new(440)
        .take_duration(Duration::from_millis(100))
        .amplify(0.20);


    // The sound plays in a seperate thread. This
    // call will block the current thread until 
    // sink has finished playing all its queued sounds
    //
    sink.append(source);
    sink.sleep_until_end();
}
