# SignalLooper Project Planning & Documentation

## Table of Contents
  * [Project Description](#project-description)
  * [Core Features](#core-features)
  * [Tech Stack](#tech-stack)
  * [Testing & CI/CD](#testing--cicd)
  * [Team Management](#team-management)
  * [Sprint Planning](#sprint-planning)

---

## Project Description

**SignalLooper** is a desktop looper and pedalboard workstation for musicians, producers, and hobbyists.

Recording and layering loops with custom effects chains is often found within expensive hardware or bloated DAWs. SignalLooper makes it easy to record a riff, add custom or preset pedals, layer a bassline and other tracks, and export your demo, all within an intuitive desktop app. Built for quick demos and effects chain experimentation.

---

## Core Features

### Audio Engine
- [ ] Live audio input capture with `cpal`
- [ ] Loop recording with configurable length
- [ ] Metronome with count-in
- [ ] Playback state machine (idle -> recording -> playing ->  overdubbing)
- [ ] 4 independent tracks with olume, pan, mute

### DPS (Effects)
- [ ] Pedal trait/interface for building effects
- [ ] **Must-have effects**: Delay, Overdrive, Low-Pass Filter
- [ ] **Nice-to-have effects**: Reverb, Chorus, Tremolo
- [ ] Real-time parameter control (knobs)
- [ ] Customize effect chains per track

### UI
- [ ] Main transport controls (record, play, stop, undo)
- [ ] Track list with faders and mute buttons
- [ ] Pedalboard view for each track
- [ ] Waveform visualization of recorded loops
- [ ] Loop length selecotr

### Import/Export & Project Management
- [ ] Record live audio input or drop-in existing audio files
- [ ] Export mix to WAV, MP3
- [ ] Save/Load project files (time allowing)

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri 2.0 |
| Backend | Rust |
| Audio I/O | `cpal` |
| DSP | Custom Rust processing |
| Frontend | React + TypeScript |
| State Management | Zustand |
| Build Tools | `cargo` + `npm` |

---

## Testing & CI/CD

- **Testing Frameworks:** `cargo test` for Rust, Jest for frontend
- **CI:** GitHub Actions (run tests on PR, lint, build)
- Pull requests require passing tests and code review
- Use `tauri dev` for local development with hot-reload

---

## Team Management

### Team Members 
- **Scrum Master:** [Harrison Johns](https://github.com/johnsh9656)  
- **Frontend Developers:**
- **Rust/Backend Developers:**  
- **DSP:**

### Team Communication
- **Platform:** Discord  
- **Meeting Schedule:**
  - Sprint Review: Weekly
  - Other Meetings: Potential for in-person meetings depending on team availability

### Roles and Responsibilities

- **Scrum Master:**
  - Maintain project vision
  - Create project boilerplate
  - Create tickets
  - Define and update backlog
  - Facilitate sprint planning
  - Review and merge pull requests
  - Support developers

- **Developers:**
  - Self-assign tickets from KanBan board
  - Develop features through ticket completion
  - Create test cases to for feature/ticket acceptance
  - Create pull requests for scrum master to review
  - Report/fix bugs
  - Propose new features/fixes
  - Work across teams


### Collaboration Tools
We will be using GitHub Projects as a way to organize our project as use of this system will make it easy for us to adhere to the Scrum framework. We will use a Kanban Board style system which will allow sprint progress to be easily monitored. Issues will be assigned to developers, and are expected to be completed by a designated due date. Developers will work in a branch, then submit a pull request to be reviewed by the Scrum Master before being merged to the main branch, ultimately closing the issue. 

### Code Acceptance

A feature is complete when:
- Code works as specified
- Passed code review
- Unit tests pass
- Documentation updated
- No merge conflicts

---

## Sprint Planning

### Sprint Structure
- **Sprint Length:** 2 weeks
- **Review:** once every week

#### Sprint 1: Project Setup & Audio I/O
**Goal:** Get the app running and producing sound
**Deliverables:**
- Tuari + React boilerplate  with CI configured
- `cpal` integrated, test tone playing from speakers
- Basic UI shell (record, play, stop buttons)

#### Sprint 2: Single-Track Looper
**Goal:** Record a loop and play it back
**Deliverables:**
- Audio input capture working
- Audio file selector if not live recording
- Loop state machine (idle -> recording -> playing -> overdubbing)
- Metronome count-in
- Loop length selector?
- Waveform visualization of recorded loop
- Volume fader and pan control

#### Sprint 3: Effect Chain Pipeline
**Deliverables:**
- Pedal trait/interface in Rust
- 3 effects: Delay, Overdrive, Low-Pass Filter
- Pedalboard UI for track
- Drag pedals from sidebar onto track pedalboard
- Knob controls for effect  parameters
- Bypass toggle per effect
- Prep for Fall Presentation & Demo

#### Sprint 4: Scaling to 4 Tracks
**Goal:** Support 4 tracks with independent effects
**Deliverables:**
- Multi-track engine (4 tracks)
- Independent effect chains  per track
- Mute buttons
- Master fader?
- Undo/redo for recording takes

#### Sprint 5: More Effects & Export
**Goal:** Expand effects library and enable project export
**Deliverables:**
- 2 new effects: Reverb, Chrous
- Export mix to WAV, MP3\
- Keyboard shortcuts

#### Sprint 6: Final Polish & Demo
**Goal:** Bug fixes, polish, presentation prep
**Deliverables:**
- Save/load project files ?
- UI polish and animations
- Perfomance tuning (latency, CPU)
- Documentation complete
- Open Source Presentation & Demo

**sprint planning expected to change over development*

---

## 🔗 Resources

- [Tauri Docs](https://tauri.app/)
- [cpal Docs](https://docs.rs/cpal/)
- *[Atlassian: Sprint Planning](https://www.atlassian.com/agile/scrum/sprint-planning#:~:text=What%20is%20sprint%20planning%3F,with%20the%20whole%20scrum%20team.)*