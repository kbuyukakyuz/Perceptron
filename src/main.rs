use perceptron::*;
use perceptron::{SAMPLE_SIZE, TRAIN_COUNT};
fn main(){
    let mut inputs = Layer::new(SAMPLE_SIZE as usize, SAMPLE_SIZE as usize);
    let mut weights = Layer::new(SAMPLE_SIZE as usize, SAMPLE_SIZE as usize);
    for _ in 0..TRAIN_COUNT{
        inputs.random_rect();
        let adjusted = inputs.train(&mut weights);
        println!("{}", adjusted);
    }
}
