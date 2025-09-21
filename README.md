# WA3 Reflection

## Educational Concept & Target Audience

### What math, science, or natural topic does your simulation teach?

Orbital mechanics and rocket simulation

### How is it adapted for beginner, intermediate, and advanced audiences?

* **Beginner:** The simulation starts with a simple objective - escape the Moon's gravity well. All controls are clearly displayed on screen, and the trajectory prediction helps visualize where their spacecraft will go. The physics simulation handles the complex orbital mechanics automatically.
* **Intermediate:** Once escaped from the Moon, learners must navigate to Earth and achieve a stable Low Earth Orbit (LEO). This requires understanding orbital velocity, altitude management, and fuel conservation while dealing with multiple gravitational bodies.
* **Advanced:** The final challenge involves performing a controlled landing on Earth, which requires precise velocity control, understanding atmospheric entry (simulated through altitude restrictions), and managing fuel reserves throughout the entire multi-stage mission.

### Elaborate on learning objectives for each audience level.

* **Beginner:** Beginners will only need to achieve a stable orbit around the moon
* **Intermediate:** Intermediate learners will be able to escape moon to start their journey to earth
* **Advanced:** Advanced learners will be able to return to earth and land safely

---

## How does your simulation facilitate learning?

### What interactive elements promote exploration?

Open world mechanics that allows users to freely move their spaceship to anywhere in the simulation. Key interactive features include:

* **Real-time trajectory prediction** - Shows future orbital path to help plan maneuvers
* **Multi-body gravitational system** - Earth, Luna (large moon), and Europa (smaller moon) create complex orbital dynamics
* **Fuel management** - Limited fuel forces strategic thinking about when and how much to thrust
* **Time warp controls** - Speed up simulation with `[` and `]` keys (restricted near celestial bodies for safety)
* **Visual feedback** - Live UI showing altitude, speed, fuel status, and current objective progress
* **Progressive objectives** - Clear goals guide learning from simple escape to complex orbital mechanics

---

## Design Documentation

### Your planning process, including any diagrams, sketches, or descriptions

Initially, I planned to only demonstrate the gravity slingshot mechanics, but later I decided to add more features to
make it a more complete experience. I sketched out the UI layout and the different levels of complexity for each
audience level.

### UI/UX design considerations for different audience levels

Since the topic is already quite advanced, I decided to show all the information to all audience levels. The information
provided is also quite minimal, so it won't overwhelm beginners.

### How you applied the PhET guidelines mentioned in the spec, if any (reference specific principles)

To be honest, I didn't explicitly follow the PhET guidelines, but I did try to make the simulation as interactive and
engaging as possible. I also tried to make the controls intuitive and easy to use.

### Did you try to get, and incorporate any feedback?

I did have a few playtests with my friends to try to find places where I can improve the user experience. One of the
main feedback I got was that the controls were a bit too sensitive, so I adjusted the sensitivity to make it easier to
control.

---

## Implementation Overview

### How it works

My simulations is a 2D N-body gravitational simulator where the user can control a spaceship to explore the moon and
earth.

### List features for each audience level

* **Beginner:**
    * Simple escape objective from Moon surface
    * Clear visual trajectory prediction lines
    * Real-time UI showing essential information (fuel, thrust, altitude)
    * Automatic gravity and physics calculations
    * Forgiving collision detection and physics

* **Intermediate:**
    * Multi-body orbital mechanics with Earth and two moons
    * Fuel consumption and management
    * Time warp controls for faster simulation
    * Earth orbit achievement challenge
    * Central body switching (Moon vs Earth sphere of influence)

* **Advanced:**
    * Precise landing mechanics requiring velocity control
    * Complex trajectory planning across multiple gravitational bodies
    * Fuel optimization across entire mission profile
    * Full N-body physics simulation with realistic orbital mechanics
    * Achievement of complete mission from Moon escape to Earth landing

### User controls: please provide a clear list of all keyboard/mouse controls

**Spacecraft Control:**
* `↑` Arrow Up - Increase thrust by 10% (up to 100%)
* `↓` Arrow Down - Decrease thrust by 10% (down to 0%)
* `←` Arrow Left - Rotate spacecraft counterclockwise (1° per press)
* `→` Arrow Right - Rotate spacecraft clockwise (1° per press)

**Time Management:**
* `[` Left Bracket - Decrease time warp (slower simulation)
* `]` Right Bracket - Increase time warp (faster simulation, up to 250,000x)
* Note: Time warp automatically restricted when near celestial bodies for safety

**Camera Controls:**
* `Mouse Wheel` - Zoom in/out (camera scale from 1x to 100,000x)
* `Left Mouse Drag` or `Middle Mouse Drag` - Pan camera around the simulation
* Camera automatically follows central body movement to reduce manual panning

---

## Extensions & Creative Features

### Everything you did beyond the requirements!

**Advanced Physics Implementation:**
* Full N-body gravitational simulation using Verlet integration
* Real-time trajectory prediction with adaptive timestep optimization
* Collision detection with continuous collision checking
* Central body sphere-of-influence switching (Moon vs Earth gravity dominance)

**Educational Enhancements:**
* Progressive objective system with three difficulty levels
* Smart time warp restrictions based on altitude for realistic space operations
* Comprehensive fuel management system with consumption rates
* Visual trajectory prediction helping users understand orbital mechanics

**Technical Features:**
* Built with Rust and Bevy game engine for high performance
* Efficient prediction system using simplified 2-body approximation for speed
* Smart camera following with automatic central body tracking
* Scalable physics constants allowing realistic Earth-Moon-spacecraft system

## Screenshots and Demo

### Screenshots

![](assets/ss1.png)
![](assets/ss2.png)

### Demo video link

![YouTube link](https://youtu.be/d-lTe6LuVLM)

---

## Reflections:

One challenge that I faced during the coding process was the prediction system since the underlying architecture of my simulator was an N-body simulator, however that would be too inefficient to predict for a lot of future points, therefore I opted for a simple 2 body simulation with larger timesteps so that it can simulate faster.

---

## Source Code

### Preview link
[GitHub Pages](https://netsbot.github.io/cepwa3)


### Include any special setup instructions if needed

For better performance there are pre-built Linux binaries on the releases page.

If you are using a different platform or you do not feel comfortable running random binaries, you can build the project from source by
```
git clone https://github.com/netsbot/cepwa3-bevy.git
cd cepwa3-bevy
cargo run
```

Make sure that you have the latest version of rust installed on your computer.

### GitHub link is optional but no harm adding to your portfolio!

https://github.com/netsbot/cepwa3-bevy

---

## Acknowledgments

### Cite all sources that inspired or influenced your project

[N-body problem](https://natureofcode.com/forces/#the-n-body-problem)

[Bevy cheat book](bevy-cheatbook.github.io/introduction.html)

[Rocket engine sound](https://pixabay.com/sound-effects/rocket-loop-99748/)

[Bevy project template](https://github.com/NiklasEi/bevy_game_template)

### Anyone gave useful feedback? Put here too!

[Verlet integration](https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html)

[Law of Universal Gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation)

### Include existing code, tutorials, your own prior projects, and any AI assistance

I used GitHub Copilot to generate boilerplate code for UI elements and world setup
