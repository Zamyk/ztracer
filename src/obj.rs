use wavefront_obj;
use crate::camera::Point3;
use super::triangle::TTriangle;

pub fn parse(path: &str) -> Option<Vec<TTriangle<f64>>> {

    let obj_contents = std::fs::read_to_string(path).map_err(|e| {
        println!("Error reading OBJ file: {}: {}", path, e);
    }).ok()?;

    let obj_set = wavefront_obj::obj::parse(obj_contents);
    if obj_set.is_err() {
        println!("Error parsing obj file: {} {}", path, obj_set.err().unwrap());
        return None;
    }
    let obj_set = obj_set.unwrap();
    let obj = &obj_set.objects[0];

    let mut triangles : Vec<TTriangle<f64>> = vec![];

    for shape in &obj.geometry[0].shapes {
        if let wavefront_obj::obj::Primitive::Triangle((i1, _, _), (i2, _, _), (i3, _, _)) = shape.primitive {
            let to_point = |i: usize| -> Point3 {
                Point3{x: obj.vertices[i].x, y: obj.vertices[i].y, z: obj.vertices[i].z}
            };

            let v1 = to_point(i1);
            let v2 = to_point(i2);
            let v3 = to_point(i3);

            triangles.push(TTriangle::<f64>{v1:v1, v2:v2, v3:v3});
        }
    }

    Some(triangles)
}