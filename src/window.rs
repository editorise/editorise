use crate::{
    buffer::Buffer,
    core::{Point, Size},
};

pub struct Window {
    pub position: Point<u16>,
    pub size: Size<u16>,
    pub buffers: Vec<Buffer>,
}

impl Window {
    pub fn get_active_buffer(&self) -> &Buffer {
        self.buffers.get(0).unwrap()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(0).unwrap()
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.size.width = width;
        self.size.height = height;

        for buffer in self.buffers.iter_mut() {
            buffer.area.width = width;
            buffer.area.height = height;
        }
    }

    pub fn get_active_buffer_visible_x(&self, column: usize) -> u16 {
        self.position.x + self.get_active_buffer().column_to_visible_x(column)
    }

    pub fn get_active_buffer_visible_y(&self, row: usize) -> u16 {
        self.position.y + self.get_active_buffer().row_to_visible_y(row)
    }
}
