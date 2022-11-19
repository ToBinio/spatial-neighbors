use colored::Colorize;
use spatial_neighbors::quad_tree::QuadTree;
use spatial_neighbors::SpatialPartitioner;

const WIDTH: f64 = 10.0;
const HEIGHT: f64 = 10.0;
const RADIUS: f64 = 3.0;
const POSITION: (f64, f64) = (0.0, 0.0);

fn main() {
    let mut elements = generate_elements();

    let mut quad_tree = QuadTree::with_capacity((-WIDTH / 2.0)..(WIDTH / 2.0), (-HEIGHT / 2.0)..(HEIGHT / 2.0), 5);

    for (index, element) in elements.iter().enumerate() {
        quad_tree.insert(element.location, index)
    }

    let in_circle = quad_tree.in_circle(POSITION, RADIUS);

    for index in in_circle {
        elements.get_mut(index).unwrap().in_circle = true;
    }

    print_elements(&elements);
}

struct Element {
    location: (f64, f64),
    in_circle: bool,
}

fn generate_elements() -> Vec<Element> {
    let mut elements = Vec::new();

    let width = WIDTH as i32;
    let height = HEIGHT as i32;

    for y in -height / 2..height / 2 {
        for x in -width / 2..width / 2 {
            elements.push(Element {
                location: (x as f64 + 0.5, y as f64 + 0.5),
                in_circle: false,
            });
        }
    }

    elements
}

fn print_elements(elements: &Vec<Element>) {

    let width = WIDTH as i32;
    let height = HEIGHT as i32;

    for y in -height / 2..height / 2 {
        for x in -width / 2..width / 2 {
            let index = (y + height / 2) * width + (x + width / 2);

            if elements.get(index as usize).unwrap().in_circle {
                print!("{}", format!(" X ").green())
            } else {
                print!(" * ")
            }
        }

        println!()
    }
}