use ::intersection::Intersection;
use ::math::{Ray, Point3, Vec3};


pub trait Surface {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

pub struct Sphere {
    pub position: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(p:Point3, r:f64) -> Sphere {
        Sphere {
            position: p,
            radius: r,
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // Distance between ray origin and sphere center
        let ray_origin_to_sphere: Vec3 = self.position - ray.origin;

        let dot = ray_origin_to_sphere.dot(ray.direction);

        // Point at which the ray passes closest to the sphere's center
        let projection : Point3 = ray.origin + ray.direction.scale(dot);

        // How close the ray comes to the sphere's center
        let distance = projection.distance_from(self.position);

        if dot <= 0.0 {
            // The sphere is behind the ray
            return None
        } else if distance > self.radius {
            // Ray is too far away to hit
            return None
        } else if distance == self.radius {
            // Single intersection tangent to the sphere
            let intersection_point = projection;
            return Some(Intersection{
                point: intersection_point,
                normal: (self.position - intersection_point).normalize()
            })
        } else {
            // Two intersections (we only want the close one)
            // This ignores the ray being inside the sphere!

            // Distance from `projection` to the actual intersection point
            let d : f64 = (self.radius*self.radius - distance*distance).sqrt();

            // Move from `projection` backwards along the ray by `d`--
            // here's the actual intersection point
            let intersection_point : Point3 = projection -
                                     ray.direction.scale(-1.0).scale(d);

            return Some(Intersection{
                point: intersection_point,
                normal: (self.position - intersection_point).normalize()
            })
        }
    }
}

pub struct Plane {
    pub position: Point3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(p:Point3, n:Vec3) -> Plane {
        Plane {
            position: p,
            normal: n,
        }
    }
}

impl Surface for Plane {
    // TODO: Reasoning unclear!
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let dot = ray.direction.dot(self.normal);
        if dot == 0.0 {
            return None
        }
        let distance = (self.position - ray.origin).dot(self.normal) / dot;
        if distance >= 0.0 {
            return Some(Intersection{
                point: ray.position(distance),
                normal: self.normal
            })
        } else {
            return None
        }
    }
}

#[test]
fn test_sphere_intersection_with_ray_at_two_points() {
    let sphere = Sphere::new(Point3::new(0.0,0.0,0.0), 1.0);
    let ray = Ray::new(Point3::new(-5.0,0.0,0.0), Vec3::new(1.0,0.0,0.0));
    let intersection : Intersection = sphere
                                          .intersect(ray)
                                          .expect("No intersection");
    assert_eq!(intersection.point, Point3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_sphere_intersection_with_ray_at_tangent() {
    let sphere = Sphere::new(Point3::new(0.0,0.0,0.0), 1.0);
    let ray = Ray::new(Point3::new(-5.0,0.0,1.0), Vec3::new(1.0,0.0,0.0));
    let intersection : Intersection = sphere
                                          .intersect(ray)
                                          .expect("No intersection");
    assert_eq!(intersection.point, Point3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_plane_intersection_with_ray_() {
    let plane = Plane::new(Point3::new(0.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
    let ray = Ray::new(Point3::new(0.0,1.0,0.0), Vec3::new(0.0,-1.0,1.0));
    let intersection : Intersection = plane
                                          .intersect(ray)
                                          .expect("No intersection");
    assert_eq!(intersection.point, Point3::new(0.0, 0.0, 1.0));
}
