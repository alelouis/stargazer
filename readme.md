# Stargazer



## Objectives
**I am building a star explorer application : Stargazer.**

This project serves as a proxy for me to get better at:
- Rust
- ECS based architecture (with [Bevy](https://bevyengine.org))
- Basic 3D engine manipulations (projection matrices, quaternions, transforms..)
- Astronomy fancy details
- Shaders ? Maybe ?

### Current status
https://user-images.githubusercontent.com/6841652/135526724-30507e3a-f4a9-4605-9730-406ef8cbe9e1.mp

Stargazer is in early development (started early September 2021). The current envisioned features implemented/to be implemented are listed below.

### Done
- [x] Custom 3D to 2D projection
- [x] Equatorial grid render from paths
- [x] Constellation render from csv file
- [x] Camera movement from mouse
- [x] RA/DEC star coordinate fetching from Some(api)
- [x] Processing star data to theta/phi coordinates
- [x] Drawing stars individually from sprite
- [x] Multiple states (menu, config, etc...)
- [x] GUI button animation
- [x] FOV smoothing
- [x] Orbit camera smoothing

### To Do (not in order)
- [ ] Correct bug with multiple kb inputs for state transition
- [ ] Showing stars names
- [ ] Showing ra/dec values on screen border **(WIP)**
- [ ] Adaptive grid resolution
- [ ] Adaptive scene generation (generate only what can be seen by the camera)
- [ ] Align 3D world coordinates to RA/DEC and cardinal points
- [ ] Aesthetic concerns (make it beautiful) **(When I'm Bored)**

------------
## How to use
With Rust installed, run one of the binaries available:
```bash
cargo run --bin app --release
```
First compilation will take some time, see [dynamic linking](https://bevyengine.org/learn/book/getting-started/setup/) for faster compilation times. 

------------







