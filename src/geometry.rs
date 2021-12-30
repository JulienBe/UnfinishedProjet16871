pub mod geometry {
    pub struct Point {
        x: i16,
        y: i16,
    }

    impl Point {
        pub fn new(x: i16, y: i16) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    pub struct SectionPoint {
        point: Point,
    }

    impl SectionPoint {
        pub fn new(x: i16, y: i16) -> Self {
            let point = Point { x, y };
            Self {
                point
            }
        }
    }

    pub struct Section {
        origin: Point,
        points: Vec<SectionPoint>,
    }

    impl Section {
        pub fn new(origin: Point, points: Vec<SectionPoint>) -> Self {
            Self {
                origin,
                points,
            }
        }
    }

    pub fn create_section(origin: Point, width: i16, height: i16) -> Section {
        let bottom_left = SectionPoint::new(0, 0);
        let bottom_right = SectionPoint::new(width, 0);
        let top_left = SectionPoint::new(0, height);
        let top_right = SectionPoint::new(width, height);

        Section::new(origin, vec![bottom_left, top_left, top_right, bottom_right])
    }
}