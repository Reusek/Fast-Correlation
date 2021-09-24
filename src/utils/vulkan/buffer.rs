use vulkano::buffer::CpuAccessibleBuffer;

pub struct Buffer<T> {
    size: u32,
    data: ndarray::Array1<T>,
    v_buffer: std::sync::Arc<CpuAccessibleBuffer<[usize]>,
}

impl<T> Buffer<T> {
    pub fn new(
        device: std::sync::Arc<vulkano::device::Device>,
        data: ndarray::Array1<T>
    ) -> Buffer<T> {
        use vulkano::buffer::BufferUsage;
        use std::convert::TryFrom;

        let v_buffer = CpuAccessibleBuffer::from_data(
            device.clone(),
            BufferUsage::all(),
            false,
            &data
        ).expect("failed to create buffer");

        Buffer {
            size: u32::try_from(data.len()).unwrap(),
            data: data,
            v_buffer: v_buffer
        }
    }
}