use creek::{ReadDiskStream, SymphoniaDecoder};

// Open a read stream.
fn open_stream(file_path: &str) -> Option<ReadDiskStream<SymphoniaDecoder>> {
    let read_result = ReadDiskStream::<SymphoniaDecoder>::new(
        file_path,          // Path to file.
        0,                  // The frame in the file to start reading from.
        Default::default(), // Use default read stream options.
    );
    let mut stream = match read_result {
        Ok(stream) => {
            println!("Read stream opened successfully.");
            stream
        }
        Err(e) => {
            println!("Failed to open read stream: {}", e);
            return None;
        }
    };

    // Cache the start of the file into cache with index `0`.
    let _ = stream.cache(0, 0);

    // Tell the stream to seek to the beginning of file. This will also alert the stream to the existence
    // of the cache with index `0`.
    let seek_result = stream.seek(0, Default::default());
    match seek_result {
        Ok(_) => println!("Seeked to beginning of file."),
        Err(e) => {
            println!("Failed to seek to beginning of file: {}", e);
            return None;
        }
    };

    // Wait until the buffer is filled before sending it to the process thread.
    //
    // NOTE: Do ***not*** use this method in a real-time thread.
    let wait_result = stream.block_until_ready();
    match wait_result {
        Ok(_) => println!("Read stream is ready."),
        Err(e) => {
            println!("Failed to wait for read stream to be ready: {}", e);
            return None;
        }
    };

    Some(stream)

    // (Send `read_stream` to the audio processing thread)

    // Open a write stream.

    // WriteDiskStream::<WavEncoder<wav_bit_depth::Float32>>::new(
    //     "./test_files/wav_f32_stereo_out.wav", // Path to file.
    //     2,                                     // The number of channels in the file
    //     44100,                                 // The sample rate of the file
    //     Default::default(),                    // Use default write stream options.
    // )
    // .unwrap();

    // (Send `write_stream` to the audio processing thread)
}

fn read_data(
    stream: &mut ReadDiskStream<SymphoniaDecoder>,
    chunk_size: usize,
) -> Option<creek::read::ReadData<'_, f32>> {
    let read_result = stream.read(chunk_size);
    let data = match read_result {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to read from disk stream: {}", e);
            return None;
        }
    };

    println!("frames: {}", data.num_frames());
    Some(data)
}

// In the realtime audio processing thread:
fn read_audio_data(
    read_disk_stream: &mut ReadDiskStream<SymphoniaDecoder>,
    chunk_size: usize,
    current_offset: usize,
    accept_data: fn(Vec<&[f32]>),
) -> Option<()> {
    // Update read client and check if it is ready.
    //
    // NOTE: You should avoid using `unwrap()` in realtime code.
    if !read_disk_stream.is_ready().unwrap() {
        // If the look-ahead buffer is still buffering, We can choose to either continue
        // reading (which will return silence), or pause playback until the buffer is filled.
    }

    let done = current_offset >= read_disk_stream.block_size();
    if done {
        // If there is no more audio to read, we can choose to either loop back to the beginning
        // of the file, or stop playback.
        println!("No more audio frames to read.");
        return None;
    }

    let data = read_data(read_disk_stream, chunk_size);
    match data {
        Some(d) => {
            let num_channels = d.num_channels();
            let mut channels = Vec::with_capacity(num_channels);
            for c in 0..num_channels {
                channels.push(d.read_channel(c));
            }
            accept_data(channels);
        }
        None => return None,
    }

    read_audio_data(
        read_disk_stream,
        chunk_size,
        current_offset + chunk_size,
        accept_data,
    )

    // Seek to a new position in the file.
    // read_disk_stream.seek(50000, SeekMode::Auto};

    // assert_eq!(read_dist_stream.playhead(), 50000);

    // Send stereo data to be written to disk.

    // write_disk_stream.write(
    //     &[read_data.read_channel(0), read_data.read_channel(1)]
    // ).unwrap();
}

fn accept_data(data: Vec<&[f32]>) {
    println!("Received {} channels of data.", data.len());
    for (i, channel) in data.iter().enumerate() {
        println!("Channel {}: {} samples", i, channel.len());
    }
}
pub fn start_and_read() -> bool {
    let stream_result = open_stream("/home/nathan/Downloads/darling_i.mp3");
    let mut stream = match stream_result {
        Some(s) => s,
        None => {
            println!("Failed to open stream.");
            return false;
        }
    };
    read_audio_data(&mut stream, 512, 0, accept_data);

    true
}
