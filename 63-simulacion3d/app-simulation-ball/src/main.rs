use three_d::*;

fn main() {
    let mut position_y: f32 = 0.5;
    let mut velocity_y: f32 = 0.0;
    let gravity: f32 = -0.005;
    let floor_y: f32 = -0.5; // piso

    let window = Window::new(WindowSettings {
        title: "Círculo 3D".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    // Crear la geometría del círculo (como un disco plano)
    let circle_mesh = CpuMesh::circle(64); // 64 segmentos
    let material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new(255, 0, 0, 255), // Color rojo
            ..Default::default()
        },
    );
    let mut circle = Gm::new(Mesh::new(&context, &circle_mesh), material);

    // Posiciona el círculo un poco más lejos del centro de la cámara
    circle.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 0.0)));

    // Cámara
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0), // posición de la cámara
        vec3(0.0, 0.0, 0.0), // hacia dónde mira
        vec3(0.0, 1.0, 0.0), // vector "arriba"
        degrees(45.0),
        0.1,
        100.0,
    );

    // Luz ambiental
    let ambient_light = AmbientLight::new(&context, 1.0, Srgba::WHITE);

    // Bucle principal
    window.render_loop(move |frame_input| {

        // Actualiza la posición del círculo
        velocity_y += gravity;
        position_y += velocity_y;
        if position_y < floor_y {
            position_y = floor_y;
            velocity_y = -velocity_y * 0.8;
        }

        if velocity_y.abs() < 0.001 && position_y < floor_y{
            velocity_y = 0.0;
        }
        
        // Actualiza viewport si cambia el tamaño
        camera.set_viewport(frame_input.viewport);
        
        // Limpia la pantalla
        frame_input
        .screen()
        .clear(ClearState::color_and_depth(0.9, 0.9, 0.9, 1.0, 1.0));
    
        // Actualiza la posición del círculo
        circle.set_transformation(Mat4::from_translation(vec3(0.0, position_y, 0.0)) * Mat4::from_scale(0.1));
        
        // Renderiza el círculo
        circle.render(&camera, &[&ambient_light]);

        FrameOutput::default()
    });
}
