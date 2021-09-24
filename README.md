# stl2lnas

Convert .stl files to Lagrangian Nassu (.lnas) format.

It is an auxiliary project for [Nassu solver](https://bitbucket.org/aerosim-cfd/nassu), an LBM based CFD solver.


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

You can substitute `examples/convert_cube.yaml` for your configuration file.

## Lagrangian Nassu format

The Lagrangian Nassu format contains a set of points representing an object, each one with given normal and area.
Its points should be distributed evenly on the object's surface.
In order to do it, ``stl2lnas`` implements triangles division (refining).

The lagrangian nodes are used by IBM (Immersed Boundary Method) to represent a body and its physics.
Therefore the need for area and normal properties.

The format definition is:
```
header: "LAGRANGIAN NASSU NODES" (22 bytes)
minimun possible area: f32 (4 bytes)
maximun possible area: f32 (4 bytes)
number of points: usize (8 bytes)
foreach point (28 bytes):
    point's position (x, y, z): (f32, f32, f32) (12 bytes)
    point's normal (x, y, z): (f32, f32, f32) (12 bytes)
    point's area: f32 (4 bytes)
end
```

## Configuration files

Some STL and configuration examples are provided in the `examples` folder.

The example below illustrates what each 
```yaml
stl:
  # STL to convert
  filename: "examples/stl/cube.stl"
output:
  # Where to save output files
  folder: "output/cube"
  # Wheter to save as csv as well or not
  save_csv: true
conversion:
  # Refinement levels to generate (considering Nassu's LBM levels)
  # Each level, the delta x between points is divided by two
  lvls_generate: [0, 1, 2, 3, 4, 5, 6]
  # Characteristics for nodes lvl 0 (least refined),
  # other level can be deduced from it;
  lvl0:
    # Minimun possible area for triangles
    # Rule for next level:
    # min_area(n+1) = min_area(n)/4
    min_area: 0.2
    # Maximun possible area for triangles
    # Rule for next level:
    # max_area(n+1) = max_area(n)/4
    max_area: 1.0
    # Object's size in x
    # Rule for next level:
    # x_size(n+1) = x_size(n)/2
    x_size: 2
```

## Limitations

Some of the known limitations are:

- It cannot "derefine" triangles, this is, increase the size of triangles.
- It can only divide triangles by area, not considering its angles, which may disturb points distribution.
- It can only convert binary STL
