type Point = (f32, f32);

fn is_left(p0: &Point, p1: &Point, p2: &Point) -> f32 {
    (p1.0 - p0.0) * (p2.1 - p0.1) - (p2.0 - p0.0) * (p1.1 - p0.1)
}


fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let mut wn = 0;

    let mut polys = poly.to_owned();
    polys.push(poly[0]);

    for i in 0..poly.len() {
        //println!("i:{:?},i+1:{:?},is_left:{}", polys[i], polys[i + 1], is_left(&polys[i], &polys[i + 1], &point));
        if polys[i].1 <= point.1 && polys[i + 1].1 > point.1 && is_left(&polys[i], &polys[i + 1], &point) > 0. {
            wn += 1;
        } else if polys[i].1 > point.1 && polys[i + 1].1 <= point.1 && is_left(&polys[i], &polys[i + 1], &point) < 0. {
            wn -= 1;
        }

        //println!("winding number:{}", wn);
    }

    if wn == 0 {
        false
    } else {
        true
    }
}
