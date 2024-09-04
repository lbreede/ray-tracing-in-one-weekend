# Ray Tracing in One Weekend

(in Rust)

![Final Image Render](images/image_24_1200px_500ssp.png)

- Resolution: 1200x675 px
- Samples per Pixel: 500
- Max Depth: 50

## To do

- [ ] Refactor the `Hittable` trait so that `hit()` returns an `Optiona<HitRecord>` rather that a `bool`