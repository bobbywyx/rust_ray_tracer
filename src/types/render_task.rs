#[derive(Clone, Copy)]
pub struct RenderTask {
    pub start_id: i32,
    pub end_id: i32, // render [start_id,end_id)
    pub samples_per_pixel: i32,
}

impl RenderTask {
    pub(crate) fn new(start_id: i32, end_id: i32, samples_per_pixel: i32) -> Self {
        RenderTask {
            start_id: start_id,
            end_id: end_id,
            samples_per_pixel: samples_per_pixel,
        }
    }
}
