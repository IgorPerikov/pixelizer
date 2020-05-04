use std::ops::Range;

pub struct ImageSegmentIterator {
    column_segment_number: u32,
    row_segment_number: u32,
    segment_size: u32,
}

impl ImageSegmentIterator {
    pub fn get_points(&self) -> Vec<Point> {
        let mut elements = Vec::new();
        for column in self.get_columns() {
            for row in self.get_rows() {
                elements.push(Point { x: column, y: row });
            }
        }
        elements
    }

    fn get_columns(&self) -> Range<u32> {
        let start_column = self.column_segment_number * self.segment_size;
        start_column..(start_column + self.segment_size)
    }

    fn get_rows(&self) -> Range<u32> {
        let start_row = self.row_segment_number * self.segment_size;
        start_row..(start_row + self.segment_size)
    }

    pub fn create(
        column_segment_number: u32,
        row_segment_number: u32,
        segment_size: u32,
    ) -> ImageSegmentIterator {
        ImageSegmentIterator {
            column_segment_number,
            row_segment_number,
            segment_size,
        }
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}
