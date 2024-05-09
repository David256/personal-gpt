// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use async_std::task::JoinHandle;
use std::error::Error;
use std::future::IntoFuture;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::{env, thread};
use tokio::runtime::{Handle, Runtime};

use async_openai::types::{AudioResponseFormat, CreateTranscriptionRequestArgs};
use async_openai::Client;
use chrono::Local;

use pv_recorder::PvRecorderBuilder;

static mut FILENAME: String = String::new();
static mut SAMPLE_RATE: usize = 16000;

static LISTENING: AtomicBool = AtomicBool::new(false);
static SAVED: AtomicBool = AtomicBool::new(false);
static WAS_ERROR: AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async fn transcribe_srt(filename: String) -> Result<String, Box<dyn Error>> {
    println!("Will send this file: {}", filename);
    let client = Client::new();
    let request = CreateTranscriptionRequestArgs::default()
        .file(filename)
        .model("whisper-1")
        .response_format(AudioResponseFormat::Json)
        .build()?;

    let response = client.audio().transcribe(request).await?;
    Ok(response.text)
}

#[tauri::command(async)]
async fn speech_to_text(filename: &str) -> Result<String, ()> {
    let path = filename.to_owned();

    // let runtime = Arc::new(Runtime::new().unwrap());

    // let handle = thread::spawn({
    //     let runtime = Arc::clone(&runtime);

    //     move || {
    //         runtime.block_on(async {
    //             match transcribe_srt(path).await {
    //                 Ok(text) => text,
    //                 Err(e) => {
    //                     println!("Error AI: {}", e);
    //                     String::new()
    //                 }
    //             }
    //         })
    //     }
    // });

    let transcribed = match transcribe_srt(path).await {
        Ok(text) => text,
        Err(e) => {
            println!("Error AI: {}", e);
            String::new()
            // Err(())
        }
    };
    Ok(transcribed)
}

#[tauri::command]
fn start_recording(index: Option<usize>) {
    match index {
        Some(i) => println!("called start_recording: {}", i),
        None => println!("called start_recording: None"),
    }

    unsafe {
        let filename = Local::now().format("%Y-%m-%d %H:%M:%S.wav").to_string();

        let temp_dir = env::temp_dir();
        let mut temp_path = PathBuf::new();
        temp_path.push(temp_dir);
        temp_path.push(filename);
        let abs_path = temp_path.to_string_lossy().to_string();
        println!("temp_path = {}", abs_path);

        FILENAME = abs_path;
    }

    println!("Initializing pvrecorder...");

    thread::spawn(move || {
        let mut recorder_builder = PvRecorderBuilder::new(512);
        let _recorder_builder = match index {
            Some(i) => recorder_builder.device_index(i as i32).init(),
            None => recorder_builder.init(),
        };
        let recorder = _recorder_builder.expect("Failed to initialize pvrecorder");
        recorder.start().expect("Failed to start audio recording");
        LISTENING.store(true, Ordering::SeqCst);
        SAVED.store(false, Ordering::SeqCst);
        WAS_ERROR.store(false, Ordering::SeqCst);

        println!("Recording...");

        let mut audio_data = Vec::new();
        while LISTENING.load(Ordering::SeqCst) {
            let read = recorder.read();
            if read.is_err() {
                WAS_ERROR.store(true, Ordering::SeqCst);
                LISTENING.store(false, Ordering::SeqCst);
                break;
            }
            let frame = read.unwrap();
            audio_data.extend_from_slice(&frame);
        }

        println!("Stop recording...");
        recorder.stop().expect("Failed to stop audio recording");

        println!("Dumping audio to file...");
        unsafe {
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: SAMPLE_RATE as u32,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            println!("Will save in {}", FILENAME.clone());

            let mut writer = hound::WavWriter::create(FILENAME.clone(), spec).unwrap();
            for sample in audio_data {
                writer.write_sample(sample).unwrap();
            }
        }

        SAVED.store(true, Ordering::SeqCst);
    });
}

#[tauri::command]
fn stop_recording() -> String {
    println!("called stop_recording");

    LISTENING.store(false, Ordering::SeqCst);

    while !SAVED.load(Ordering::SeqCst) {}

    SAVED.store(false, Ordering::SeqCst);

    unsafe { FILENAME.clone() }
}

#[tauri::command]
fn get_error_message() -> bool {
    // println!("called get_error_message");
    WAS_ERROR.load(Ordering::SeqCst).clone()
}

#[tauri::command]
fn list_devices() -> Vec<String> {
    println!("Getting audio devices...");

    let audio_devices = PvRecorderBuilder::default().get_available_devices();

    match audio_devices {
        Ok(audio_devices) => audio_devices,
        Err(err) => {
            println!("Failed to get audio devices: {}", err);
            Vec::<String>::new()
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            list_devices,
            speech_to_text,
            get_error_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
