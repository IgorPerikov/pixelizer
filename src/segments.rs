use image::{DynamicImage, GenericImageView};
use std::ops::Range;

#[derive(PartialEq, Debug)]
pub struct ImageSegmentIterator {
    column_segment_number: u32,
    row_segment_number: u32,
    segment_size: u32,
}

pub fn split_image_by_segments(input: &DynamicImage, tile_size: u32) -> Vec<ImageSegmentIterator> {
    let mut segments = Vec::new();
    for row_segment_number in 0..(input.height() / tile_size) {
        for column_segment_number in 0..(input.width() / tile_size) {
            segments.push(ImageSegmentIterator {
                column_segment_number,
                row_segment_number,
                segment_size: tile_size,
            });
        }
    }
    segments
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
    fn should_properly_split_image_by_segments() {
        let image = DynamicImage::new_rgba8(10, 10);
        let segments = split_image_by_segments(&image, 5);
        assert_eq!(
            vec![
                ImageSegmentIterator {
                    column_segment_number: 0,
                    row_segment_number: 0,
                    segment_size: 5
                },
                ImageSegmentIterator {
                    column_segment_number: 1,
                    row_segment_number: 0,
                    segment_size: 5
                },
                ImageSegmentIterator {
                    column_segment_number: 0,
                    row_segment_number: 1,
                    segment_size: 5
                },
                ImageSegmentIterator {
                    column_segment_number: 1,
                    row_segment_number: 1,
                    segment_size: 5
                }
            ],
            segments
        )
    }

    #[test]
    fn segment_should_return_all_points() {
        let segment_iterator = ImageSegmentIterator {
            column_segment_number: 2,
            row_segment_number: 2,
            segment_size: 3,
        };
        assert_eq!(
            vec![
                Point { x: 6, y: 6 },
                Point { x: 7, y: 6 },
                Point { x: 8, y: 6 },
                Point { x: 6, y: 7 },
                Point { x: 7, y: 7 },
                Point { x: 8, y: 7 },
                Point { x: 6, y: 8 },
                Point { x: 7, y: 8 },
                Point { x: 8, y: 8 }
            ],
            segment_iterator.get_points()
        );
    }
}
