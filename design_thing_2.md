# Design Thign 2

In order to graph an arbitrary variable with an expression that contains other variables, we must have a way of constructing a point with
-- we could use Hash maps, but that sucks
-- there could also just be hardcoded `if dep_var == 'x' { /* ... */ }` garbage. That breaks down once we have multiple dependent variables.

Why am I using a "We" in this note? This is a project maintained only by me.

## Different types of graphs

* Cartesian - One/two indep. variables: (x, z). One dependent expression: (y). Window must contain firstX->lastX & firstZ->lastZ.
* Parametric - One indep. variable (t). Two/three dependent expressions (x, y, z). Window must contain firstT->lastT.
* Polar - One indep. variable (θ). One/two dependent expression: (r OR rx, ry). Window must contain firstθ->lastθ

...maybe have separate `Graph, ParametricGraph, PolarGraph` types that implement a `Graphable` trait?
But that just screws over Graph3D. Why can't there just be a `fn Graph::new(type: GraphType, dim: GraphDimensions) -> Graph`?
Might just be better to implement a `Graph` and `Graph3D` separately.

...maybe have `fn Graph::new(args: GraphArgs) -> Graph`, an enum like...
```rust
pub struct  {
	// todo
}

pub enum GraphArgs {
	Cartesian,
	Parametric(),
	Polar(),
}
```

Also maybe move the Window struct over to a sort of MinMax array with n members?
```rust
pub struct MinMax {
	min: f64,
	max: f64,
}

pub type Window = [MinMax]; // nope
```
