use fastrand::Rng;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct MigrationSimulator {
    particles: Vec<(f32, f32)>,
    origins: Vec<(f32, f32)>,
    destinations: Vec<(f32, f32)>,
    speeds: Vec<f32>,
    progress: Vec<f32>,
    city_coords: Vec<(f32, f32)>, // Almacena todas las coordenadas de ciudades
    rng: Rng,  // Generador de números aleatorios
}

#[wasm_bindgen]
impl MigrationSimulator {
    #[wasm_bindgen(constructor)]
     pub fn new(city_coords: Vec<f32>) -> Self {
        let mut rng = Rng::new();
        let cities: Vec<(f32, f32)> = city_coords.chunks(2).map(|c| (c[0], c[1])).collect();

        let particles: Vec<(f32, f32)> = (0..500)
            .map(|_| cities[rng.usize(0..cities.len())])
            .collect();

        MigrationSimulator {
            particles: particles.clone(),
            origins: particles.clone(),
            destinations: (0..500)
                .map(|_| cities[rng.usize(0..cities.len())])
                .collect(),
            speeds: (0..500).map(|_| rng.f32() * 0.003 + 0.002).collect(),
            progress: vec![0.0; 500],
            city_coords: cities,
            rng,
        }
    }

    pub fn update(&mut self) {
        for i in 0..500 {
            self.progress[i] += self.speeds[i];
            
            if self.progress[i] >= 1.0 {
                self.progress[i] = 0.0;
                self.origins[i] = self.particles[i];
                
                let current_city = self.find_nearest_city(self.particles[i]);
                let mut new_city = current_city;
                while new_city == current_city && self.city_coords.len() > 1 {
                    new_city = self.rng.usize(0..self.city_coords.len());
                }
                self.destinations[i] = self.city_coords[new_city];
            }
            
            let t = self.progress[i];
            let smooth_t = t * t * (3.0 - 2.0 * t);
            self.particles[i] = (
                self.origins[i].0 + (self.destinations[i].0 - self.origins[i].0) * smooth_t,
                self.origins[i].1 + (self.destinations[i].1 - self.origins[i].1) * smooth_t,
            );
        }
    }

    // Encuentra la ciudad más cercana a una posición
    fn find_nearest_city(&self, point: (f32, f32)) -> usize {
        self.city_coords
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = (a.0 - point.0).powi(2) + (a.1 - point.1).powi(2);
                let dist_b = (b.0 - point.0).powi(2) + (b.1 - point.1).powi(2);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    #[wasm_bindgen]
    pub fn get_particles(&self) -> *const f32 {
        self.particles.as_ptr() as *const f32
    }

    #[wasm_bindgen]
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

