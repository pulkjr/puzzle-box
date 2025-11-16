# Puzzle Box (Rust + Raspberry Pi Pico)

A puzzle box project written in Rust, built on the [rp-rs rp2040 project template](https://github.com/rp-rs/rp2040-project-template).
This box challenges players with a series of electronic and cryptology-based puzzles, progressing through multiple stages until either **Success** or **Failure**.

---

## Project Overview

The puzzle box runs on a Raspberry Pi Pico microcontroller and is designed to test problem-solving skills through staged challenges.
Each stage must be completed to advance, with a countdown timer adding urgency after initialization.

---

## 🎮 Puzzle Stages

1. **Init Stage**
   - System powers on and initializes.
   - First puzzle must be solved to proceed.
   - Countdown timer begins after completion; failure to progress before timeout results in **Failed** state.

2. **Running Stage**
   - Intermediate puzzles involving electronic theory and cryptology.
   - Successful completion advances to Culminating Test.

3. **Culminating Test Stage**
   - Final and most complex puzzle.
   - Requires mastery of prior concepts.
   - Determines eligibility for final outcome.

4. **Failed Stage**
   - Triggered if countdown expires or puzzles are solved incorrectly.
   - Box signals failure.

5. **Success Stage**
   - Triggered if all puzzles are solved correctly within time limits.
   - Box signals success.

---

## ⚙️ Materials Required

- **Microcontroller**
  - Raspberry Pi Pico
- **Electronics**
  - Breadboard
  - Jumper wires
  - LEDs
  - Resistors
  - Capacitors
  - Push buttons / switches
  - Buzzer or speaker
- **Power**
  - USB cable (for Pico)
  - Battery pack

## Puzzle Elements

---

## 🛠️ Setup & Development

1. Clone the template:
   ```bash
   git clone https://github.com/rp-rs/rp2040-project-template
   ```
2. Install Rust toolchain and dependencies.
3. Build and flash to Raspberry Pi Pico:

   ```bash
   cargo build --release
   ```

4. Copy the .uf2 file to the Pico.
