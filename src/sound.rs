use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::time::Duration;

pub fn beep() {
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok((s, s_h)) => (s, s_h),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    /*
    match Sink::try_new(&stream_handle) {
        Ok(sink) => {
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

        Err(err) => panic!("ERROR: {:?}", err),
    };
    */
}
