# Roadmap

- [x] Create ASCII representation of how board should look like
- [x] Create test program with argparse for following arguments:
  - [x] Number of guesses before end of game (8..=12)
  - [x] Number of positions in guesses (4..=6)
  - [x] Clear is a valid value inside guesses (bool)
- [ ] Publish application to repositories
  - [ ] `dnf` and `yum` repos
  - [ ] `apt-get` and `apt` repositories
  - [ ] Publish to RedoxOS
- [ ] Update handling of arguments once `clap` upgrades
- [ ] Remove `#[allow(unstable_name_collisions)]` from `lib.rs` once `intersperse` is added to the standard library
