use scheduling::{get_overlaps, Meeting};

fn main() {
    let meetings1 = Box::new(vec![
        Meeting::new(1, 3),
        Meeting::new(5, 6),
        Meeting::new(7, 9),
        Meeting::new(9, 18)
    ]);
    let meetings2 = Box::new(vec![
        Meeting::new(2, 4),
        Meeting::new(4, 7),
        Meeting::new(10,19)
    ]);
    let overlaps = get_overlaps(&meetings1, &meetings2);
    println!("Employee 1 schedule: {:?}", meetings1);
    println!("Employee 2 schedule: {:?}", meetings2);
    if overlaps.len() > 0 {
        println!("Overlapping Period: {:?}", overlaps);
    } else {
        println!("They have no overlapping period");
    }
}
