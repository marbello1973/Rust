use rand::Rng;
use std::collections::HashMap;
use three_d::*;

const GRID_SIZE: f32 = 1.0;
const RESTITUTION: f32 = 0.8;
const GRAVITY: f32 = 9.8;
const BALL_COLOR: Srgba = Srgba::new(255, 255, 255, 255);
//const BALL_COLOR: Color = Color::WHITE;
const FLOOR_Y: f32 = -8.0;

struct Ball {
    position: Vec3,
    velocity: Vec3,
    radius: f32,
    _color: Srgba,
    mass: f32,
}

impl Ball {
    fn new(position: Vec3, radius: f32, _color: Srgba) -> Self {
        let mut rng = rand::rng();
        Self {
            position,
            velocity: vec3(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            ),
            radius,
            _color,
            mass: radius * radius * radius, // Masa proporcional al volumen
        }
    }

    fn update(&mut self, dt: f32) {
        // Aplicar gravedad (solo en el eje Y)
        self.velocity.y -= GRAVITY * dt;

        // Actualizar posición
        self.position += self.velocity * dt;
    }

    fn grid_key(&self) -> (i32, i32, i32) {
        (
            (self.position.x / GRID_SIZE).floor() as i32,
            (self.position.y / GRID_SIZE).floor() as i32,
            (self.position.z / GRID_SIZE).floor() as i32,
        )
    }
}

struct SpatialGrid {
    grid: HashMap<(i32, i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.grid.clear();
    }

    fn insert(&mut self, key: (i32, i32, i32), ball_index: usize) {
        self.grid
            .entry(key)
            .or_insert_with(Vec::new)
            .push(ball_index);
    }

    fn nearby_keys(key: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let mut keys = Vec::new();
        for x in (key.0 - 1)..=(key.0 + 1) {
            for y in (key.1 - 1)..=(key.1 + 1) {
                for z in (key.2 - 1)..=(key.2 + 1) {
                    keys.push((x, y, z));
                }
            }
        }
        keys
    }
}

fn main() {
    // Inicializar ventana 3D
    let window = Window::new(WindowSettings {
        title: "Simulación 3D de Pelotas".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Contexto de renderizado
    let context = window.gl();

    // Cámara
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 5.0, 15.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    // Control de cámara
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    //luces
    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(-1.0, -1.0, -1.0));
    let point_light = PointLight::new(
        &context,
        5.0,
        Srgba::WHITE,
        vec3(5.0, 10.0, 5.0),
        Attenuation::default(),
    );
    // Crear pelotas
    let mut rng = rand::rng();
    let mut balls = Vec::new();
    for _ in 0..50 {
        balls.push(Ball::new(
            vec3(
                rng.random_range(-5.0..5.0),
                rng.random_range(0.0..10.0),
                rng.random_range(-5.0..5.0),
            ),
            rng.random_range(0.2..0.5),
            Srgba::new(255, 255, 255, 255),
            //Color::WHITE,
        ));
    }

    // Esferas para renderizado
    let mut spheres = Vec::new();
    for ball in &balls {
        let mut sphere = Gm::new(
            Mesh::new(&context, &CpuMesh::sphere(32)),
            PhysicalMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: BALL_COLOR,
                    ..Default::default()
                },
            ),
        );
        sphere.set_transformation(
            Mat4::from_translation(ball.position) * Mat4::from_scale(ball.radius),
        );
        spheres.push(sphere);
    }

    // Piso
    let mut floor = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(128, 128, 128, 255),
                metallic: 0.1,
                roughness: 0.7,
                ..Default::default()
            },
        ),
    );
    floor.set_transformation(
        Mat4::from_translation(vec3(0.0, FLOOR_Y, 0.0))
            * Mat4::from_scale(10.0)
            * Mat4::from_angle_x(radians(-90.0)),
    );

    // Cuadrícula espacial
    let mut spatial_grid = SpatialGrid::new();

    // Bucle principal
    window.render_loop(move |mut frame_input| {
        // Actualizar cámara
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Tiempo delta
        let dt = 0.016; // ~60 FPS

        // Actualizar cuadrícula espacial
        spatial_grid.clear();
        for (i, ball) in balls.iter().enumerate() {
            spatial_grid.insert(ball.grid_key(), i);
        }

        // Actualizar física
        for i in 0..balls.len() {
            balls[i].update(dt);

            // Colisión con el piso
            if balls[i].position.y - balls[i].radius < FLOOR_Y {
                balls[i].position.y = FLOOR_Y + balls[i].radius;
                balls[i].velocity.y = -balls[i].velocity.y * RESTITUTION;
                balls[i].velocity.x *= 0.1; // Fricción
                balls[i].velocity.z *= 0.1;

                if balls[i].velocity.y.abs() < 0.1 {
                    balls[i].velocity.y = 0.5;
                }
            }

            // Obtener celdas vecinas
            let key = balls[i].grid_key();
            let nearby_keys = SpatialGrid::nearby_keys(&key);

            // Chequear colisiones con pelotas cercanas
            for nearby_key in nearby_keys {
                if let Some(indices) = spatial_grid.grid.get(&nearby_key) {
                    for &j in indices {
                        if i != j && j < balls.len() {
                            let diff = balls[i].position - balls[j].position;
                            let distance = diff.magnitude();
                            let min_distance = balls[i].radius + balls[j].radius;

                            if distance < min_distance {
                                // Resolver colisión
                                let normal = diff.normalize();
                                let relative_velocity = balls[i].velocity - balls[j].velocity;
                                let velocity_along_normal = relative_velocity.dot(normal);

                                // Solo si se están acercando
                                if velocity_along_normal > 0.0 {
                                    continue;
                                }

                                let mass_i = balls[i].mass;
                                let mass_j = balls[j].mass;

                                let impulse_magnitude = -(1.0 + RESTITUTION)
                                    * velocity_along_normal
                                    / (1.0 / mass_i + 1.0 / mass_j);

                                let impulse = normal * impulse_magnitude;
                                balls[i].velocity += impulse / mass_i;
                                balls[j].velocity -= impulse / mass_j;

                                // Separar las pelotas
                                let overlap = min_distance - distance;
                                let correction = overlap * 0.5 * normal;
                                balls[i].position += correction;
                                balls[j].position -= correction;
                            }
                        }
                    }
                }
            }
        }

        // Actualizar esferas de renderizado
        for (i, sphere) in spheres.iter_mut().enumerate() {
            if i < balls.len() {
                sphere.set_transformation(
                    Mat4::from_translation(balls[i].position) * Mat4::from_scale(balls[i].radius),
                );
            }
        }

        // Renderizar

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0))
            .render(
                &camera,
                spheres
                    .iter()
                    .map(|s| s as &dyn Object)
                    .chain(std::iter::once(&floor as &dyn Object)),
                &[&ambient, &directional, &point_light],
            );

        FrameOutput::default()
    });
}
