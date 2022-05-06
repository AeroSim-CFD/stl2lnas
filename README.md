# stl2lnas

Convert .stl files to Lagrangian Nassu (.lnas) format.

This is an auxiliary project for [Nassu solver](https://bitbucket.org/aerosim-cfd/nassu),
an LBM based CFD solver.


1. [Usage](#usage)
2. [Lagrangian Nassu format](#lagrangian-nassu-format)
3. [Configuration files](#configuration-files)
4. [Limitations](#limitations)


## Usage

To use the program, you may run

```bash
# --release: "makes the program faster"
# --: finish cargo's arguments
# --cfg <file>: configuration file to use
cargo run --release -- --cfg examples/convert_cube.yaml
```

You can substitute `examples/convert_cube.yaml` with your configuration file.

## Lagrangian Nassu format (.lnas)

The Lagrangian Nassu format contains informations for representing a body. 
It follows similar compact strategy as [Wavefront obj format](https://en.wikipedia.org/wiki/Wavefront_.obj_file), but restricts its polygons to triangles.

The format is used to define nodes (points) that are used by IBM (Immersed Boundary Method) to represent a body and its physics.

The format definition is:
```yaml
# Format version. Every major, breaks compatibility 
# v0.2.0 is not compatible with v0.1.0, but it is with v0.2.0)
version: "v0.2.0"
# Name to use for export
name: "cube"
# Size to use in x for normalization
normalization_x: 16.0
geometry:
  # Points are represented as a list [(x0, y0, z0), (x1, y1, z1), ..., (xk, yk, zk)] in f32
  points: "base64"
  # Triangles are represented as a list [(p01, p02, p03), (p11, p12, p13), ..., (pn1, pn2, pn3)] in u32
  # Where each value in the triple is the point index in the `points` list.
  triangles: "base64"
  # Normal may be recovered using "right hand" rule. 
  # That is, it considers rotation in sequence p1->p2->p3, so normal is
  # U = p2 - p1; V = p3 - p1 then the normal N = U X V
```

### Compactation impact

The compactation of `.lnas` format is mainly due to not repeating the vertices shared between triangles.
So the impact of it increases with the number of shared vertices.

It also uses the points order in the triangle to store the triangle's normal information, not needing to explicitly specify it.

The number of floating points from STL to LNAS may be calculated by:

$N_{LNAS} = V_{STL} + \frac{T_{STL}}{2S}$

Where $V_{STL}$ is the number of unique vertices, $T_{STL}$ the number of triangles and $S$ the average number of triangles that a vertice shares.


## Configuration files

Some STL and configuration examples are provided in the `examples` folder.

The example below describes what each field does
 
```yaml
stl:
  # STL to convert
  filename: "examples/stl/cube.stl"
output:
  # Where to save output files
  folder: "output/cube"
  # Save in csv format as well
  save_csv: true
  # Copy original STL to output folder
  copy_stl: true
conversion:
  # Size in x to use
  x_size: 2
```

## Limitations

Some of the known limitations are:

- It cannot "derefine" triangles, this is, increase the size of triangles. This limits the STL resolution, because its triangles cannot be smaller than the minimum possible area.
- It does not consider triangle angles. This may disturb points distribution.
- It can only convert binary STL files
