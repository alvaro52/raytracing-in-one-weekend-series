use crate::shape::triangle::*;

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    bvh_nodes: Vec<BVHNode>,
    triangle_indices: Vec<usize>,
    centroids: Vec<Vec3>,
    nodes_used: usize,
    pub center: Vec3,
}

impl Mesh {
    pub fn new(path_to_model: &str, material: Material) -> Self {
        let model = match wavefront::Obj::from_file(path_to_model) {
            Ok(object) => object,
            Err(_) => {
                eprintln!("could not load model {}", path_to_model);
                panic!("could not load model");
            }
        };

        let triangles: Vec<Triangle> = model.triangles()
            .map(|[p1, p2, p3]| {
                let p1 = Vec3::from_array(p1.position());
                let p2 = Vec3::from_array(p2.position());
                let p3 = Vec3::from_array(p3.position());
                Triangle::new(p1, p2, p3, material.clone())
            })
            .collect();

        let mut model = Mesh {
            bvh_nodes: Vec::new(),
            triangle_indices: Vec::new(),
            centroids: Vec::new(),
            triangles,
            center: Vec3::ZERO,
            nodes_used: 0,
        };
        model.build_bvh();
        
        model
    }

    fn build_bvh(&mut self) {
        self.bvh_nodes = vec![BVHNode::default(); self.triangles.len() * 2 - 1];
        self.nodes_used = 1;
        self.bvh_nodes[0].num_triangles = self.triangles.len() as u32;
        self.triangle_indices = (0..self.triangles.len()).collect();
        self.centroids = self.triangles.iter()
            .map(|t| (t.p1 + t.p2 + t.p3) / 3.0)
            .collect();
        self.update_node_bounds(0);
        self.subdivide(0);
    }

    fn update_node_bounds(&mut self, node_index: usize) {
        let node = &mut self.bvh_nodes[node_index];
        (node.index..node.index + node.num_triangles).for_each(|i| {
            let triangle_index = self.triangle_indices[i as usize];
            let triangle = &self.triangles[triangle_index];
            node.aabb.min = node.aabb.min.min(triangle.p1).min(triangle.p2).min(triangle.p3);
            node.aabb.max = node.aabb.max.max(triangle.p1).max(triangle.p2).max(triangle.p3);
        });
        node.aabb.min = node.aabb.min - Vec3::splat(0.0001);
        node.aabb.max = node.aabb.max + Vec3::splat(0.0001);
    }

    fn subdivide(&mut self, node_index: usize) {
        let node = self.bvh_nodes.get(node_index).unwrap().clone();
        if node.num_triangles <= 2 {
            return;
        }
        let (cost, axis, split_pos) = self.find_best_split_plane(&node);
        if cost > node.cost() {
            return;
        }
        let mut i = node.index;
        let mut j = i + node.num_triangles - 1;
        while i <= j {
            let t_index = self.triangle_indices[i as usize];
            if self.centroids[t_index][axis] < split_pos {
                i += 1;
            } else {
                self.triangle_indices.swap(i as usize, j as usize);
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }
        let left_count = i - node.index;
        if left_count == 0 || left_count == node.num_triangles {
            return;
        }

        let left_child_index = self.nodes_used;
        self.nodes_used += 1;
        let right_child_index = self.nodes_used;
        self.nodes_used += 1;
        let left_child = self.bvh_nodes.get_mut(left_child_index).unwrap();
        left_child.index = node.index;
        left_child.num_triangles = left_count;
        let right_child = self.bvh_nodes.get_mut(right_child_index).unwrap();
        right_child.index = i;
        right_child.num_triangles = node.num_triangles - left_count;
        let node = self.bvh_nodes.get_mut(node_index).unwrap();
        node.index = left_child_index as u32;
        node.num_triangles = 0;
        self.update_node_bounds(left_child_index);
        self.update_node_bounds(right_child_index);
        self.subdivide(left_child_index);
        self.subdivide(right_child_index);
    }

    fn find_best_split_plane(&self, node: &BVHNode) -> (f32, usize, f32) {
        let mut best_cost = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_position = f32::INFINITY;
        let candidates_num = 8;
        (0..3)
            .for_each(|axis| {
                let extent = node.aabb.max[axis] - node.aabb.min[axis];
                let fraction = extent / (candidates_num + 2) as f32;
                (1..=candidates_num)
                    .map(|i| i as f32 * fraction + node.aabb.min[axis])
                    .for_each(|candidate_position| {
                        self.evaluate_sah(node, axis, candidate_position)
                            .inspect(|cost| {
                                if *cost < best_cost {
                                    best_cost = *cost;
                                    best_axis = axis;
                                    best_position = candidate_position;
                                }
                            });
                    })
            });

        (best_cost, best_axis, best_position)
    }

    fn evaluate_sah(&self, node: &BVHNode, axis: usize, position: f32) -> Option<f32> {
        let mut left = AABB::default();
        let mut right = AABB::default();
        let mut left_count = 0;
        let mut right_count = 0;
        (0..node.num_triangles).into_iter()
            .map(|i| self.triangle_indices[(node.index + i) as usize])
            .for_each(|index| {
                let centroid = self.centroids[index];
                let triangle = &self.triangles[index];
                if centroid[axis] < position {
                    left.min = left.min.min(triangle.p1).min(triangle.p2).min(triangle.p3);
                    left.max = left.max.max(triangle.p1).max(triangle.p2).max(triangle.p3);
                    left_count += 1;
                } else {
                    right.min = right.min.min(triangle.p1).min(triangle.p2).min(triangle.p3);
                    right.max = right.max.max(triangle.p1).max(triangle.p2).max(triangle.p3);
                    right_count += 1;
                }
            });

        let cost = left_count as f32 * left.area() + right_count as f32 * right.area();
        (cost > 0.0).then_some(cost)
    }

    fn transverse_bvh(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let mut stack = Vec::with_capacity(64);
        let mut closest_t = interval.end;
        let mut hit_record = None;
        stack.push(0);

        while !stack.is_empty() {
            let node = &self.bvh_nodes[stack.pop().unwrap()];
            if node.is_leaf() {
                (closest_t, hit_record) = (0..node.num_triangles)
                    .map(|i| self.triangle_indices[(node.index + i) as usize])
                    .map(|triangle_index| &self.triangles[triangle_index])
                    .fold((closest_t, hit_record), |(closest_t, closest_hit), triangle| {
                        if let Some(hit) = triangle.hits(ray, interval.start..closest_t) {
                            (hit.t, Some(hit))
                        } else {
                            (closest_t, closest_hit)
                        }
                    });

                continue
            }

            let left_child_index = node.index as usize;
            let right_child_index = node.index as usize + 1;
            let left_node = &self.bvh_nodes[left_child_index];
            let right_node = &self.bvh_nodes[right_child_index];

            let distance_left = left_node.ray_aabb_distance(ray, closest_t);
            let distance_right = right_node.ray_aabb_distance(ray, closest_t);

            match (distance_left, distance_right) {
                (Some(left_t), Some(right_t)) if left_t > right_t => {
                    stack.push(left_child_index);
                    stack.push(right_child_index);
                },
                (Some(left_t), Some(right_t)) if left_t <= right_t => {
                    stack.push(right_child_index);
                    stack.push(left_child_index);
                },
                (Some(_), None) => stack.push(left_child_index),
                (None, Some(_)) => stack.push(right_child_index),
                _ => {}
            }
        }

        hit_record
    }
    
    pub fn rotate_y(&mut self, angle: f32) {
        let rotation = Mat4::from_rotation_y(angle.to_radians());
        for triangle in self.triangles.iter_mut() {
            triangle.transform(&rotation);
        }
        self.build_bvh();
    }
    
    pub fn translate(&mut self, translation: Vec3) {
        let translation = Mat4::from_translation(translation);
        for triangle in self.triangles.iter_mut() {
            triangle.transform(&translation);
        }
        self.build_bvh();
    }
    
    pub fn scale(&mut self, scale: Vec3) {
        let scale = Mat4::from_scale(scale);
        for triangle in self.triangles.iter_mut() {
            triangle.transform(&scale);
        }
        self.build_bvh();
    }
}

impl Hittable for Mesh {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        self.transverse_bvh(ray, interval)
    }
}

#[derive(Clone)]
struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn area(&self) -> f32 {
        let extent = self.max - self.min;
        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Vec3::INFINITY,
            max: Vec3::NEG_INFINITY
        }
    }
}

#[derive(Clone)]
struct BVHNode {
    aabb: AABB,
    index: u32,
    pub num_triangles: u32,
}

impl Default for BVHNode {
    fn default() -> Self {
        Self {
            aabb: AABB::new(Vec3::INFINITY, Vec3::NEG_INFINITY),
            index: 0,
            num_triangles: 0,
        }
    }

}

impl BVHNode {
    fn is_leaf(&self) -> bool {
        self.num_triangles > 0
    }

    pub fn ray_aabb_distance(&self, ray: &Ray, closest_t: f32) -> Option<f32> {
        let t_min = (self.aabb.min - ray.origin) / ray.direction;
        let t_max = (self.aabb.max - ray.origin) / ray.direction;
        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);
        let t_near = t1.max_element();
        let t_far = t2.min_element();
        let did_hit = t_near < t_far && t_far > 0.001 && t_near < closest_t;

        did_hit.then_some(t_near)
    }

    pub fn cost(&self) -> f32 {
        self.num_triangles as f32 * self.aabb.area()
    }
}
