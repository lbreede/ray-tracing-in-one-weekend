# Ray Tracing in One Weekend

(in Rust)

![Final Image Render](images/image_24_1200px_500ssp.jpg)

- Resolution: 1200x675 px
- Samples per Pixel: 500
- Max Depth: 50

## To do

- [x] Refactor the `Hittable` trait so that `hit()` returns an `Option<HitRecord>` rather than a `bool`
- [x] Refactor the `Material` trait so that `scatter()` returns an `Option` rather than a `bool`
- [ ] Consider using an `Arc` instead of a `Box` for materials and hittables