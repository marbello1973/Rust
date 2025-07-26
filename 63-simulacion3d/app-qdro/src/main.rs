use three_d::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Failed to initialize window: {0}")]
    WindowCreationError(String),
    
    #[error("Graphics context failure: {0}")]
    GraphicsError(String),
}

fn main() {
    match win() {
        Ok(_) => println!("Aplicación terminó exitosamente"),
        Err(error) => {
            eprintln!("Error en la aplicación: {}", error);
            std::process::exit(1);
        }
    }
}

fn win() -> Result<(), ApplicationError> {
    // Configuración de la ventana
    let window_settings = WindowSettings {
        title: "Esfera 3D".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    };

    // Crear ventana
    let window = Window::new(window_settings)
        .map_err(|e| ApplicationError::WindowCreationError(e.to_string()))?;

    // Obtener contexto gráfico
    let context = window.gl();
    
    // Crear esfera
    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(32)), // 16 segmentos para la esfera
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(204, 51, 15, 255), // Color rojizo (usando Srgba)
                ..Default::default()
            },
        ),
    );

    // Configurar cámara
    let camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 3.0), // Posición de la cámara
        vec3(0.0, 0.0, 0.0),  // Punto de mira (centro de la escena)
        vec3(0.0, 1.0, 0.0),  // Vector arriba
        degrees(45.0),        // Campo de visión
        0.1,                  // Plano cercano
        100.0,                // Plano lejano
    );

    // Bucle de renderizado
    window.render_loop(move |frame_input| {
        // Rotar la esfera
        sphere.set_transformation(
            Mat4::from_angle_y(radians(frame_input.elapsed_time as f32 * 0.5))
        );

        // Renderizar
        frame_input.screen().clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        sphere.render(&camera, &[&AmbientLight::new(&context, 0.5, Srgba::WHITE)]);
        
        FrameOutput::default()
    });

    Ok(())
}