# WA3 Reflection

## Educational Concept & Target Audience

### What math, science, or natural topic does your simulation teach?

Orbital mechanics and rocket simulation

### How is it adapted for beginner, intermediate, and advanced audiences?

* **Beginner:** [Explain how the simulation is simplified for new learners.]
* **Intermediate:** [Describe how more complexity and features are introduced.]
* **Advanced:** [Detail how the simulation presents the most challenging and comprehensive version of the topic.]

### Elaborate on learning objectives for each audience level.

* **Beginner:** Beginners will only need to achieve a stable orbit around the moon
* **Intermediate:** Intermediate learners will be able to escape moon to start their journey to earth
* **Advanced:** Advanced learners will be able to return to earth and land safely

---

## How does your simulation facilitate learning?

### What interactive elements promote exploration?

Open world mechanics that allows users to freely move their spaceship to anywhere in the simu

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
    * [List key features for beginners.]
* **Intermediate:**
    * [List key features for intermediate users.]
* **Advanced:**
    * [List key features for advanced users.]

### User controls: please provide a clear list of all keyboard/mouse controls

* [List keyboard and/or mouse controls, e.g., 'W, A, S, D' for movement, 'Click' to interact, 'Space' to pause.]

---

## Extensions & Creative Features

### Everything you did beyond the requirements!

I implemented a predictional trajectory feature that shows where the spaceship will go based on its current velocity and
position. This helps users plan their maneuvers better.

## Screenshots and Demo

### Screenshots showing different modes, levels, and key features. Paste in.

[Insert images here.]

### Demo video link

[Paste your demo video link here.]

---

## Reflections:

[Write a brief reflection on your experience with this project. What did you learn? What challenges did you face? What would you do differently next time?]

---

## Source Code

### p5js preview link

[Paste your p5js preview or GitHub Pages link here.]

### Include any special setup instructions if needed

[Provide instructions for setting up and running your code locally, if applicable.]

### GitHub link is optional but no harm adding to your portfolio!

[Paste your GitHub repository link here, if you have one.]

---

## Acknowledgments

### Cite all sources that inspired or influenced your project

[List and cite all sources, including tutorials, articles, and other projects.]

### Anyone gave useful feedback? Put here too!

[Verlet integration](https://www.algorithm-archive.org/contents/verlet_integration/verlet_integration.html)

[Law of Universal Gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation)

### Include existing code, tutorials, your own prior projects, and any AI assistance

I used GitHub Copilot to generate boilerplate code for UI elements and world setup