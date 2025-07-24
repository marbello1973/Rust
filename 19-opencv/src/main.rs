use core::panic;
use std::{error::Error, fmt::Result};



use opencv::{
    core::{Mat, Scalar, Size},
    highgui,
    imgproc,
    prelude::*,
    videoio,
    Result,
};

fn main() -> Result<(), Error> {
    // Abre la cámara (dispositivo 0)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L)?; 
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("No se pudo abrir la cámara.");
    }

    let mut prev_frame = Mat::default();
    let mut curr_frame = Mat::default();
    let mut diff_frame = Mat::default();

    loop {
        // Captura un fotograma actual
        cam.read(&mut curr_frame)?;
        if curr_frame.empty() {
            continue;
        }

        // Convierte a escala de grises
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&curr_frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

        // Si hay un fotograma previo, calcula la diferencia
        if !prev_frame.empty() {
            // Calcula la diferencia absoluta
            opencv::core::absdiff(&prev_frame, &gray_frame, &mut diff_frame)?;

            // Aplica un umbral para detectar movimiento
            let mut thresh_frame = Mat::default();
            imgproc::threshold(&diff_frame, &mut thresh_frame, 25.0, 255.0, imgproc::THRESH_BINARY)?;

            // Muestra el movimiento detectado
            highgui::imshow("Movimiento Detectado", &thresh_frame)?;
        }

        // Muestra el video en vivo
        highgui::imshow("Video en Vivo", &curr_frame)?;

        // Actualiza el fotograma anterior
        prev_frame = gray_frame;

        // Rompe el bucle si se presiona 'q'
        if highgui::wait_key(10)? == 'q' as i32 {
            break;
        }
    }

    Ok(())
}

