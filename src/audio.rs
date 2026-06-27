use std::io::Cursor;

use rodio::{Decoder, DeviceSinkBuilder, Player};

/// Decode and play an in-memory audio byte slice, blocking until playback ends.
/// No temporary files are written to disk.
pub(crate) fn play_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() {
        return Err("Audio buffer is empty".to_string());
    }

    // 1. Create a Cursor from the in-memory bytes and decode it
    let cursor = Cursor::new(bytes.to_vec());
    let source = Decoder::new(cursor).map_err(|e| format!("Failed to decode audio: {}", e))?;

    // 2. Get an OS-Sink handle to the default physical sound device.
    // The handle must live as long as the player, so we bind it to a variable.
    let handle = {
        let mut sink = DeviceSinkBuilder::open_default_sink()
            .map_err(|e| format!("Failed to open audio output device: {}", e))?;
        sink.log_on_drop(false);
        sink
    };

    // 3. Create a Player connected to the OS-Sink's mixer
    let player = Player::connect_new(&handle.mixer());

    // 4. Add the decoded source to the player
    player.append(source);

    // 5. Block the current thread until the player has finished playing
    player.sleep_until_end();

    Ok(())
}
