use noise::{NoiseFn, Perlin};

pub struct TerrainGenerator {
    perlin: Perlin,
    min_height: f64,
    max_height: f64,
}

impl TerrainGenerator {
    pub fn new(seed: u32, min_height: f64, max_height: f64) -> Self {
        Self {
            perlin: Perlin::new(seed),
            min_height,
            max_height,
        }
    }

    fn get_raw_perlin(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y])
    }

    pub fn get_perlin(&self, x: f64, y: f64) -> f64 {
        let value = self.get_raw_perlin(x, y);
        let normalized_value = (value + 1.0) / 2.0;
        self.min_height + normalized_value * (self.max_height - self.min_height)
    }
}