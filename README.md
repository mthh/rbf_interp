# rbf_interp
[![Build Status](https://travis-ci.org/mthh/rbf_interp.svg?branch=master)](https://travis-ci.org/mthh/rbf_interp)

Rust library for *Radial Basis Function Interpolation*.

**Want to interpolate from a set of known points at any point:**
```rust
let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                   Pt::new(0.0, 100.0, 6.0),
                   Pt::new(75.0, 25.0, 3.1),
                   Pt::new(100.0, 75.0, 7.4)];
let rbf = Rbf::new(&obs_pts, "linear", None);
// Compute the value at point (0.0, 50.0) :
let interpolated_value = rbf.interp_point((0.0, 50.0)), 0.0000001);
```

**Want to interpolate from a set of known points on a regular grid:**
```rust
let obs_points_two_stocks = vec![Pt::new(3.5, 3.5, 100.0), Pt::new(6.5, 6.5, 100.0)];
// Define the bbox as xmin, xmax, ymin, ymax:
let bbox = Bbox::new(0.0, 10.0, 0.0, 10.0);
// Define the resolution on x and y axis:
let (reso_x, reso_y) = (40, 40);
// Get a vector of point with the interpolated value:
let res_rbf = rbf_interpolation(reso_x,
                                reso_y,
                                &bbox,
                                &obs_points_two_stocks,
                                "inverse_multiquadratic",
                                Some(1.66))
```

Various radial basis function are implemented :
"linear", "cubic", "thin_plate", "quintic", "gaussian", "multiquadratic" and "inverse_multiquadratic".

## Usage
```rust
[dependencies]
rbf_interp = "0.1"
```

## Example with gnuplot
```
cargo run --example plot
```
