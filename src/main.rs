mod rt;

use rt::vec3::Vec3;
use rt::ray::Ray;
use rt::image::{ Image, Pixel };

fn main() {
    let nx = 200;
    let ny = 100;

    let mut image = Image::new(nx, ny);

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&ray);
            
            let ir = (255.99 * col.get_x()) as u8;
            let ig = (255.99 * col.get_y()) as u8;
            let ib = (255.99 * col.get_z()) as u8;

            image.get_mut(i, j).unwrap().set(ir, ig, ib)
        }
    }

    image.print()
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}