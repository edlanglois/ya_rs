# ya_rs

Yars' Revenge in Rust

Just a small rush code project inspired by Tantan.
Implement Yars' Revenge in <12 hours in Rust.

Learning Bevy & Rust!

**12 / 12 Hours Complete!** [Play it here!!](https://brandon-reinhart.github.io/ya_rs/)

Hour 1:
- [x] Plan
- [x] Basic app setup.
- [x] Yar
 
Hour 2:
- [x] Yar flight w/ anims.
- [x] Yar bullet.
  
Hour 3:
- [x] Atari Resolution (Sort of...)
- [x] Clear Color == Black
- [x] Destroy bullet when hit side of screen.
- [x] Prevent Yar from flying offscreen horizontally.
- [x] Wrap Yar when fly offscreen vertically.
- [x] Get screen size and sprite size (roughly) correct relative to 2600.
 
Hour 4:
- [x] Qotile
- [x] Fix alpha
- [x] Zorlon Cannon
- [x] Collision
- [x] Zorlon Cannon create on collide Qotile.

Hour 5:
- [x] Zorlon Cannon follow Yar.
- [x] Shoot fires Zorlon Cannon.
- [x] Zorlon Cannon despawns if leave world / hit Yar / hit Qotile.

Hour 6:
- [x] Yar Death
- [x] Destroyer Missile

Hour 7:
- [x] Refactor and cleanup based on things learned so far!

Hour 8:
- [x] Neutral Zone
- [x] Neutral Zone provides immunity from Destroyer Missile

Hour 9:
- [x] Swirl
- [x] Swirl fires at Yar

Hour 10:
- [x] Swirl resets Qotile on leave world.
- [x] Swirl kills Yar on collide
- [x] Improve swirl state timing.
- [x] Zorlon Cannon kills Qotile & Swirl

Hour 11:
- [x] Qotile's Shield
- [x] Yar bites/eats Shield on collide.
- [x] Zorlon Cannon create on Eat.
- [x] Bullet damages shield.
- [x] Zorlon Cannon damages shield.

Hour 12:
- [x] Yar cannot shoot while in the Neutral Zone.
- [x] Qotile and Destroyer Missile despawn on Yar death.
- [x] WASM - Wasn't much to do here, Bevy just worked with wasm-pack.
- [x] React Web & Host

Missing Features:
- [ ] Scoring / Scoreboard
- [ ] Victory / Death Screen
- [ ] 4 Lives
- [ ] Scoring
- [ ] Shifting Shield
- [ ] Moving Shield
- [ ] Game Modes 2 & 4 (Multiplayer)
- [ ] Game Mode 3 - Alternating Shields
- [ ] Game Mode 4 - Bouncing Zorlon Cannon
- [ ] Game Mode 6 - Ultimate Yars
- [ ] VFX: Zorlon Cannon Pulse
- [ ] VFX: Qotile Death Transition
- [ ] Sounds (need to extract from 2600 by running code slices in enmulator?)
