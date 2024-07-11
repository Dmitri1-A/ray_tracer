use std::error::Error;
use ray_tracer::render;

fn main() -> Result<(), Box<dyn Error>> {
    render(400)
}
