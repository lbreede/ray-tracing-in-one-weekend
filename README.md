# Ray Tracing in One Weekend

(in Rust)

![Final Image Render](images/image_24_1200px_500ssp.jpg)

- Resolution: 1200x675 px
- Samples per Pixel: 500
- Max Depth: 50

## To do

- [x] Refactor the `Hittable` trait so that `hit()` returns an `Option<HitRecord>` rather than a `bool`
- [x] Refactor the `Material` trait so that `scatter()` returns an `Option` rather than a `bool`
- [x] Consider using an `Arc` instead of a `Box` for materials and hittables
- [x] Add trait declaration `Send + Sync`
- [x] Reduce amount of method arguments, possibly using the builder pattern
- [ ] Multi-thread using `rayon`

## Learnings

Following the book and translating C++ to Rust line by line is great to get the project up and running. Here are some changes I've done, after getting my final render out.

### Option types instead of bools

When following the book, twice we create a method that takes empty data structures as arguments that we attempt to internally mutate. If successful the method returns `true`, if not, `false`. This sounds like a job for Rust's `Option<_>` type.

#### Hit Method

For the `hit()` method of the `Hittable` trait, instead of passing a mutable `HitRecord`, we can remove the argument and return `Some(HitRecord)` or `None`, instead of `true` or `false`. Later, we can use `if let Some(rec)` to unwrap the value again.

Before:

```rust
// src/hittable.rs

fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.max;
    for object in self.objects.iter() {
        if object.hit(&r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *rec = temp_rec.clone();
        }
    }
    hit_anything
}

// src/camera.rs

let mut rec = HitRecord::default();
if world.hit(&r, Interval::new(0.001, f32::INFINITY), &mut rec) {
    // ...
}

```

After:

```rust
// src/hittable.rs

fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let mut closest_so_far = ray_t.max;
    let mut hit_anything = None;
    for object in self.objects.iter() {
        if let Some(rec) = object.hit(&r, Interval::new(ray_t.min, closest_so_far)) {
            closest_so_far = rec.t;
            hit_anything = Some(rec);
        }
    }
    hit_anything
}

// src/camera.rs

if let Some(rec) = world.hit(&r, Interval::new(0.001, f32::INFINITY)) {
    // ...
}
```

#### Scatter method

Something similar is used in the `scatter()` method that is part of the `Material` trait. It takes a mutable vec3 `attenuation` and a mutable ray `scattered` as arguments and mutates them internally. If successful, it returns `true`. If not, `false`.

Since we are dealing with two return values now, we can wrap them in a new structure I called `ScatterResult`. After that, we can wrap it once again in the `Option<_>` type.

Before:

```rust
// src/material.rs

fn scatter(
    &self,
    _r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vector3<f32>,
    scattered: &mut Ray,
) -> bool {
    let mut scatter_direction = rec.normal + random_unit_vector();

    if near_zero(&scatter_direction) {
        scatter_direction = rec.normal;
    }

    *scattered = Ray::new(rec.p, scatter_direction);
    *attenuation = self.albedo;
    true
}

// src/camera.rs

let mut scattered = Ray::default();
let mut attenuation = Vector3::default();
if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
    return attenuation.component_mul(&Camera::ray_color(&scattered, depth - 1, world));
}

```

After:

```rust
// src/material.rs

pub struct ScatterResult {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray,
}

fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
    let mut scatter_direction = rec.normal + random_unit_vector();

    if near_zero(&scatter_direction) {
        scatter_direction = rec.normal;
    }

    Some(ScatterResult {
        attenuation: self.albedo,
        scattered: Ray::new(rec.p, scatter_direction),
    })
}

// src/camera.rs

if let Some(scatter) = rec.mat.scatter(r, &rec) {
    return scatter.attenuation.component_mul(&Camera::ray_color(
        &scatter.scattered,
        depth - 1,
        world)
    );
}
```

Much cleaner :crab:

