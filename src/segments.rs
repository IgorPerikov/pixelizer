use std::ops::Range;

pub struct ImageSegmentIterator {
    column_segment_number: u32,
    row_segment_number: u32,
    segment_size: u32,
}

impl ImageSegmentIterator {
    pub fn get_points(&self) -> Vec<Point> {
        let mut elements = Vec::new();
        for row in self.get_rows() {
            for column in self.get_columns() {
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

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[cfg(test)]
mod segments_generation_tests {
    use super::*;

    #[test]
    fn should_return_all_points_of_segment() {
        let segment_iterator = ImageSegmentIterator::create(2, 2, 3);
        assert_eq!(
            vec![
                Point{x: 6, y: 6}, Point{x: 7, y: 6}, Point{x: 8, y: 6},
                Point{x: 6, y: 7}, Point{x: 7, y: 7}, Point{x: 8, y: 7},
                Point{x: 6, y: 8}, Point{x: 7, y: 8}, Point{x: 8, y: 8}
            ],
            segment_iterator.get_points()
        );
    }
}
