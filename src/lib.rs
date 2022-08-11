use std::fs::File;
use std::io::Write;
use rand::{thread_rng, Rng};
const PPM_SCALER: f32 =  25.0;
const PPM_COLOR_INTENSITY: f32 =  255.0;
const PPM_RANGE: f32 =  10.0;
const BIAS: f32 = 2.0;
pub const SAMPLE_SIZE: i32 = 250;
pub const TRAIN_COUNT: i32 = 20;

pub struct BaseLogisticRegressor {
    parameters: Option<Vec<f64>>,
}
impl BaseLogisticRegressor{
    fn new() -> BaseLogisticRegressor{
        BaseLogisticRegressor {parameters: None }
    }
}
pub struct Layer{
    height: usize,
    width: usize,
    data: Vec<Vec<f32>>,
}
impl Layer{
    pub fn new(height: usize, width: usize) -> Layer{
        Layer{
            height: height,
            width: width,
            data: vec![vec![0.0; width]; height],
        }
    }
    fn feed_forward(&mut self, weights: &mut Layer) -> f32{
        let mut output = 0.0;
        for i in 0..self.height{
            for j in 0..self.width{
                output += self.data[i][j] * weights.data[i][j];
            }
        }
        output
    }
    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, value: f32){
        let x0 = clampi(x, 0, self.width as i32 -1);
        let y0 = clampi(y, 0, self.height as i32 -1);
        let x1 = clampi(x0 + w - 1, 0, self.width as i32 -1);
        let y1 = clampi(y0 + h - 1, 0, self.height as i32 -1);
        for y in y0..=y1{
            for x in x0..=x1{
                self.data[y as usize][x as usize] = value;
            }
        }
    }
    fn fill_circle(&mut self, cx: i32, cy: i32, r: i32, value: f32){
        let x0 = clampi(cx - r, 0, self.width as i32 -1);
        let y0 = clampi(cy - r, 0, self.height as i32 -1);
        let x1 = clampi(cx + r, 0, self.width as i32 -1);
        let y1 = clampi(cy + r, 0, self.height as i32-1);
        for y in y0..=y1{
            for x in x0..=x1{
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= r * r{
                    self.data[y as usize][x as usize] = value;
                }
            }
        }
    }
    fn lerp(a: f32, b: f32, p: f32) -> f32 {
        a + (b - a) * p
    }
    fn blend_pixels_naively(&mut self, background: u32, foreground: u32, p: f32) -> u32 {
        let br = (background >> (8 * 2)) & 0xFF;
        let fr = (foreground >> (8 * 2)) & 0xFF;
        let r = Self::lerp(br as f32, fr as f32, p) as u32;
    
        let bg = (background >> (8 * 1)) & 0xFF;
        let fg = (foreground >> (8 * 1)) & 0xFF;
        let g = Self::lerp(bg as f32, fg as f32, p) as u32;
    
        let bb = (background >> (8 * 0)) & 0xFF;
        let fb = (foreground >> (8 * 0)) & 0xFF;
        let b = Self::lerp(bb as f32, fb as f32, p) as u32;
    
        (r << (8 * 2)) | (g << (8 * 1)) | (b << (8 * 0))
    }
    fn save_as_ppm(&mut self, filename: &str){
        let mut file = File::create(filename).unwrap();
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.width, self.height).unwrap();
        writeln!(file, "255").unwrap();
        for y in 0..self.height{
            for x in 0..self.width{
                let value = self.data[y][x];
                let r = ((1.0 - value) * PPM_COLOR_INTENSITY) as i32;
                let g = ((1.0 - value) * PPM_COLOR_INTENSITY) as i32;
                let b = (value * PPM_COLOR_INTENSITY) as i32;
                writeln!(file, "{} {} {}", r, g, b).unwrap();
            }
        }
    }
    fn add_inputs_from_weights(&mut self, weights: &mut Layer){
        for i in 0..self.height{
            for j in 0..self.width{
                weights.data[i][j] += self.data[i][j];
            }
        }
    }
    fn sub_inputs_from_weights(&mut self, weights: &mut Layer){
        for i in 0..self.height{
            for j in 0..self.width{
                weights.data[i][j] -= self.data[i][j];
            }
        }
    }
    pub fn random_rect(&mut self) {
        self.fill_rect(0, 0, self.width as i32, self.height as i32, 0.0);
        let x = thread_rng().gen_range(0..self.width);
        let y = thread_rng().gen_range(0..self.height);
        let w = thread_rng().gen_range(1..self.width);
        let h = thread_rng().gen_range(1..self.height);
        self.fill_rect(x as i32, y as i32, w as i32, h as i32, 1.0);
    }
    fn random_circle(&mut self){
        self.fill_rect(0, 0, self.width as i32, self.height as i32, 0.0);
        let cx = thread_rng().gen_range(0..self.width);
        let cy = thread_rng().gen_range(0..self.height);
        let r = thread_rng().gen_range(1..self.width);
        self.fill_circle(cx as i32, cy as i32, r as i32, 1.0);
    }
    pub fn train(&mut self, weights: &mut Layer) -> i32{
        let mut filepath = String::new();
        let mut count = 0;
        let mut adj = 0;
        for _ in 0..SAMPLE_SIZE{
            self.random_rect();
            if self.feed_forward(weights) > BIAS{
                self.sub_inputs_from_weights(weights);
                //snprintf
                filepath = format!("{}_{}.ppm", "weights", count);
                weights.save_as_ppm(&filepath);
                count += 1;
                adj += 1;
            }
            self.random_circle();
            if self.feed_forward(weights) <= BIAS{
                self.add_inputs_from_weights(weights);
                //snprintf
                filepath = format!("{}_{}.ppm", "weights", count);
                weights.save_as_ppm(&filepath);
                count += 1;
                adj += 1;
            }
    }
        adj
    }
}
fn clampi(x: i32, low: i32, high: i32)->i32{
    if x < low{
        low
    }else if x > high{
        high
    }else{
        x
    }
}