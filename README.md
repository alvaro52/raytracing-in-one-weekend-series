# Ray Tracing in One Weekend

This project implements the concepts from the *Ray Tracing in One Weekend* 
series using Rust. It covers the core techniques of ray tracing, building a 
toy ray tracer from "scratch", along with adding more advanced features in 
subsequent parts of the series.

## In One Weekend
In the first part, the goal is to create a simple ray tracer capable of rendering spheres with reflections and basic shading. Here's the final scene rendered by the ray tracer after completing *Ray Tracing in One Weekend*:

![Final Scene](/saves/in_one_weekend_final_scene.png)

## The Next Week
In The Next Week, the ray tracer is extended to handle more complex 
materials, lighting models and textures. Here are some 
sample renders showcasing these features:

### Added Moving Spheres and Some Textures
![Checkered Ground with Background](/saves/checkered_ground.png)

### Simple Light Source And Perlin Noise
A scene with a simple light source:

![Simple Light](/saves/simple_light.png)

### Cornell Box
The classic Cornell Box scene, especially focusing on **color bleeding**—where the red and green walls softly influence the surrounding surfaces:

![Cornell Box](/saves/cornell_box.png)

### Cornell Box with Smoke Blocks
A variation of the Cornell Box featuring constant mediums like smoke blocks

![Cornell Box with Smoke Blocks](/saves/cornell_box_smoke_blocks.png)

### Finished Implementing AABBs

Then tested with a mesh that has 870000 triangles, with 1000 samples per 
pixel took "only" 3 minutes:

![Mesh with AABBs](/saves/dragon.png)
