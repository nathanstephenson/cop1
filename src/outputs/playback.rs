use rodio::{OutputStreamBuilder, Sink};

pub fn create_output_track() -> impl FnMut(Vec<Vec<f32>>) {
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Cannot continue without audio output.");

    let mut sinks: Vec<Sink> = Vec::new();

    let play = move |data: Vec<Vec<f32>>| {
        for (i, track) in data.iter().enumerate() {
            if i >= sinks.len() {
                let new_sink = Sink::connect_new(&stream_handle.mixer());
                sinks.push(new_sink);
            }
            let source = rodio::buffer::SamplesBuffer::new(1, 44100, track.clone());
            // sinks[i].set_speed(0.01);
            sinks[i].append(source);
            sinks[i].set_volume(0.3);
            // println!("Playing track {} with {} samples", i, sinks[i].len());
            // sinks[i].play();
        }
    };
    play
}
