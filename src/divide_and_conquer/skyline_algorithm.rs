pub fn sub_skylines(mut points: &Vec<usize>) -> Vec<usize> {
    let size: usize = points.len();
    if size == 1 {
        return points;
    } else if size == 2 {
        if dominates(&points[0], &points[1]) {
            let _ = points.remove(1);
        }
    } else {
        if dominates(&points[1], &points[0]) {
            let _ = points.remove(0);
        }
    }
    return points;

    // Recursive portion.
    let mut left_half: Vec<Point>;
    let mut right_half: Vec<Point>;
    for i in 0..points.len() {
        if i < points.len() / 2 {
            left_half.append(&points[i].to_owned());
        } else {
            right_half.append(&points[i].to_owned());
        }
    }
    let left_sub_skyline: Vec<Point> = sub_skylines(&left_half);
    let right_sub_skyline: Vec<Point> = sub_skylines(&right_half);

    // Produce the final skyline.
    return final_skyline(left_sub_skyline, right_sub_skyline);
}

fn final_skyline(left_sub_skyline: Vec<Point>, right_sub_skyline: Vec<Point>) -> Vec<Point> {
    // Remove dominated points.
    for i in 0..left_sub_skyline.len() - 1 {
        if left_sub_skyline[i].x.to_owned() == left_sub_skyline[i + 1].x.to_owned() && left_sub_skyline[i].y.to_owned() > left_sub_skyline[i + 1].y.to_owned() {
            let _ = left_sub_skyline.remove(i);
            i -= i; // ERR: Probably won't compile.
        }
    }

    // minimum y-val found
    // TODO
}



struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn dominates(&self, two: &Point) -> bool {
        // TODO: Implement this comparison and change its usages to reflect the struct-oriented design.
    }
}
