/// count duplicates
use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::io::Read;

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    println!("overlaps: {}", find_overlaps(&input)?);
    Ok(())
}

fn find_overlaps(input: &str) -> Result<usize, Err> {
    let mut claims = Vec::with_capacity(1381);
    let mut intersections = vec![];
    let mut intersected_points: HashSet<Point> = HashSet::new();
    let mut i = 0;
    for line in input.lines() {
        let claim = Claim::from_line(line)?;
        for c in &claims {
            i += 1;
            claim.intersection(c).map(|rect| {
                intersected_points.extend(rect.points().iter());
                intersections.push(rect);
            });
        }
        claims.push(claim);
    }

    let mut intersected;
    for claim in &claims {
        intersected = false;
        for claim2 in &claims {
            if claim.id != claim2.id && claim.intersection(claim2).is_some() {
                intersected = true;
                break;
            }
        }
        if !intersected {
            println!("no intersection: {}", claim.id);
        }
    }

    eprintln!("claims count: {}", claims.len());
    eprintln!("intersection count: {}", intersections.len());
    eprintln!("intersected points: {}", intersected_points.len());
    eprintln!("intersections checked: {}", i);

    Ok(intersected_points.len())
}

#[derive(Debug)]
struct Claim {
    id: u32,
    points: [Point; 4],
    r: Rect,
}

impl Claim {
    fn from_line(line: &str) -> Result<Claim, Err> {
        let id: u32 = line
            .chars()
            .skip(1)
            .take_while(|chr| *chr != ' ')
            .collect::<String>()
            .parse()?;
        let pos = line
            .chars()
            .skip_while(|chr| *chr != '@')
            .skip(1)
            .take_while(|chr| *chr != ':')
            .skip(1)
            .collect::<String>();
        let split = pos.split(',').collect::<Vec<_>>();
        let (left, top): (u32, u32) = (split[0].parse()?, split[1].parse()?);
        let size = line
            .chars()
            .skip_while(|c| *c != ':')
            .skip(2)
            .collect::<String>();

        let split = size.split('x').collect::<Vec<_>>();
        let (width, height): (u32, u32) = (split[0].parse()?, split[1].parse()?);

        let top_left = Point { x: left, y: top };
        let top_right = Point {
            x: left + width - 1,
            y: top,
        };
        let bottom_right = Point {
            x: left + width - 1,
            y: top + height - 1,
        };
        let bottom_left = Point {
            x: left,
            y: top + height - 1,
        };

        Ok(Claim {
            id,
            points: [top_left, top_right, bottom_right, bottom_left],
            r: Rect {
                top_left,
                bottom_right,
            },
        })
    }

    fn intersection(&self, other: &Claim) -> Option<Rect> {
        if other.r.max_x() < self.r.min_x()
            || self.r.max_x() < other.r.min_x()
            || other.r.max_y() < self.r.min_y()
            || self.r.max_y() < other.r.min_y()
        {
            return None;
        }

        let rect = Rect {
            top_left: Point {
                x: max(self.r.min_x(), other.r.min_x()),
                y: min(self.r.max_y(), other.r.max_y()),
            },
            bottom_right: Point {
                x: min(self.r.max_x(), other.r.max_x()),
                y: max(self.r.min_y(), other.r.min_y()),
            },
        };
        Some(rect)
    }
}

#[derive(Debug)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    fn min_x(&self) -> u32 {
        self.top_left.x
    }

    fn max_x(&self) -> u32 {
        self.bottom_right.x
    }

    fn min_y(&self) -> u32 {
        self.top_left.y
    }

    fn max_y(&self) -> u32 {
        self.bottom_right.y
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        for x in self.min_x()..=self.max_x() {
            for y in self.max_y()..=self.min_y() {
                points.push(Point { x, y });
            }
        }
        points
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}
