# Communicator — visual system, product theory, and roadmap

This document covers three things:

1. **[What the visual upgrade changed](#1-the-visual-upgrade)** — the design system now in `src/theme.rs` and how the screens use it.
2. **[Why this app is worth building](#2-why-this-app-is-useful)** — who it serves, what it competes with, and where its real advantage lies.
3. **[How to make it genuinely useful](#3-roadmap)** — an honest gap list and a prioritised roadmap.

---

## 1. The visual upgrade

### 1.1 Before

The interface was functional but undesigned: default egui greys, flat rectangles painted with `rect_filled`, invisible affordances, no hover or press feedback, no elevation, no empty states, and a category expansion that swapped the screen contents instantly. Every colour and size was a literal in the middle of an 700-line `update()`.

### 1.2 After

![Home plane](images/screenshot-home.png)

Everything visual now derives from a single token module, `src/theme.rs`. Nothing in `app.rs` invents a colour or a radius.

| Token group | Values | Why |
| --- | --- | --- |
| Spacing | `SPACE` 8, `GUTTER` 24, `GRID` 24 | The brief's 24 pt anti-"fat-finger" rhythm, expressed once. Nodes snap to `GRID` on drag release. |
| Node size | 184 × 144 pt | Roughly 4× the 44 pt minimum touch target on every axis — reachable with a fist, a stylus, or a headstick. |
| Radii | node 22, card 18, pill 99 | "Soft-Tech": rounded but not toy-like. |
| Elevation | 4 levels of soft shadow, blur 8→32, no offset drift | Depth without gradients. Disabled entirely in high-contrast mode. |
| Type scale | body 19, button 19, heading 26, node label 21–26 | Sized to be read by a *listener* standing nearby, not only by the user. |
| Palettes | Light / Dark / High contrast | Three complete palettes; every surface, ink, outline and accent is derived per theme. |

**The canvas.** A soft dot grid (48 pt pitch) drawn under the nodes makes the snap grid legible and gives the board a sense of place while panning. When a category is open, the whole canvas takes a 6 % wash of that category's colour — an ambient "you are inside this node" cue that needs no reading.

**The nodes.** One function, `paint_node`, draws every category and term tile: soft shadow, hairline outline derived from the fill, an emoji/icon glyph, and an auto-contrasting label (text colour is chosen by relative luminance, so a user-picked pastel and a user-picked navy both stay readable). Three feedback states, all silent and all animated:

- **hover** — 1.5 % scale-up, deeper shadow, soft inner highlight;
- **press** — 3 % scale-*down* (the tile "gives" under the finger);
- **release** — a colour-matched ripple expands and fades over 450 ms.

The brief asked for confirmation "without the need for haptic or audio feedback". This is that.

**Semantic zoom.** Opening a category no longer swaps the screen. The tapped tile's centre becomes the camera origin: the rest of the plane scales up and fades out *past* the viewer, while the category's words scale up from that same point. Going back plays it in reverse. Both directions are 240 ms, cubic-out. Nodes are non-interactive during the flight, which also kills the mis-tap that a mid-transition tap used to cause.

**Wayfinding.** A breadcrumb in the top bar always reads `Plane › Category`, with the category chip tinted its own colour. The Back button sits at a fixed canvas position (24, 24) — never moving, never scrolling away — and the plane chip is also a tap target for going back.

![Inside a category, with a sentence under construction](images/screenshot-sentence.png)

**The dock.** Words land as pill chips filled with the parent category's colour, so the sentence carries its grammatical colour-coding — *People* blue, *Actions* green, *Needs* amber. Tapping a chip removes it (a red ✕ badge fades in on hover so the affordance is discoverable but never in the way). `Undo` and `Clear` are disabled rather than hidden when the dock is empty, and an empty dock shows a dashed placeholder that says what to do instead of showing a void.

**The ghost menu.** Still summoned only on demand, but now a floating card with real elevation that can be dragged anywhere on screen — the reach-zone accommodation from the brief. Planes are full-width 52 pt rows with their icon; the active one is filled with the accent colour.

**Settings.** Rebuilt as sections in sunken cards: appearance (theme, interface size, language), planes (add / rename / re-icon / delete with automatic re-homing of orphaned categories), and vocabulary (category list with colour rails, plus an editor with 10 preset swatches, a `#RRGGBB` field, an icon field, a plane picker, and per-word rows). It closes with a 180 × 52 pt **Done** button, because a 16 pt window ✕ is not a target this audience can hit.

**Interface size.** A slider drives egui's zoom factor from 80 % to 180 %, scaling the entire UI — the single highest-leverage accessibility control in the app, and it persists.

![Dark theme](images/screenshot-dark.png)

### 1.3 Beyond pixels

Three changes were structural rather than cosmetic, because the visuals were meaningless without them:

- **Planes became real.** They were a label that changed a string. Now every category belongs to a plane, the canvas shows only the active plane's categories, and planes can be created, renamed, re-iconed and deleted. Without this, the breadcrumb had nothing to say.
- **Icons became data.** `Category` and `Term` each carry an `icon: String`. The brief asked for "a high-contrast emoji or photo indicator"; this is the emoji half. (See [3.2](#32-next-weeks-1-4) for the photo half.)
- **Theme, scale and plane are persisted**, and every new field has a `#[serde(default)]` so an older saved board still loads.

Rendering was verified end-to-end by driving the real binary headlessly (Xvfb + software GL) with synthetic pointer events and capturing framebuffers — the screenshots above are the actual app, not mockups.

### 1.4 What was deliberately not done

- No new dependencies. Everything is `egui` primitives.
- No auto-layout of nodes. Positions are the user's motor memory; see [3.1](#31-now-the-things-that-block-real-use).
- No sound, no speech — that is a feature, not a style, and it is the top item on the roadmap.

---

## 2. Why this app is useful

### 2.1 The user

Augmentative and Alternative Communication (AAC) users are people who cannot rely on speech: cerebral palsy, ALS/MND, post-stroke aphasia, late-stage Parkinson's, traumatic brain injury, autism with limited speech, or a temporary state such as intubation in an ICU. The README targets the hardest sub-case — **non-verbal *and* limited dexterity** — which rules out dense keyboard grids and small targets.

There are two users, not one, and they never use the app at the same time:

- **The speaker** taps and needs zero cognitive overhead, huge targets, and a board that never rearranges itself.
- **The carer / SLP / family member** configures the board, and needs a rich editor, backup, and the ability to change a board in thirty seconds while the speaker waits.

Communicator already recognises this split (Edit mode vs. normal mode). That is the single most important structural decision in the app, and it should be made even sharper.

### 2.2 The market gap

- Dedicated AAC hardware runs into thousands of dollars and is usually gated behind insurance or a health service.
- The strong iPad apps (Proloquo2Go, TouchChat, LAMP Words for Life) sell for roughly £150–£250 per licence, iOS-only, and their vocabularies are English-first.
- The free tier is thin, and the *non-English* free tier is close to empty. Turkish AAC vocabulary in particular is scarce — and this codebase already ships Turkish alongside English.

A free, offline-capable, installable web app that runs on any borrowed tablet or laptop, with no account and no store, is a real gap — especially for hospitals (a board that can be spun up for one patient for one week) and for families outside the English-speaking market.

### 2.3 The actual differentiator

Most AAC apps are **grids**. Communicator is a **spatial canvas**: the carer places tiles anywhere and they stay exactly where they were put.

That sounds like a cosmetic difference. It is not, and it cuts both ways:

- **The win — motor planning.** The reason clinical systems like LAMP work is that a word's *location* never changes, so selecting it becomes muscle memory rather than a visual search. A canvas with persistent coordinates supports this perfectly, and lets the carer shape the layout around a specific person's reach: everything within a 15 cm arc on the left, if that is what the person can reach.
- **The risk — clutter and drift.** Free positioning also allows overlapping, unreachable and forgotten tiles. The grid snapping and pan clamping added in this pass are the first defences; auto-layout must never be added.

The second differentiator is **planes as contexts**. Clinicians already build "environmental boards" — one for the dinner table, one for the ward. Making that a first-class object, with a one-tap switch and its own spatial layout, matches how the domain actually works.

### 2.4 Honest assessment

As of today the app is a well-designed **point board** — a communication partner still has to look at the screen and read the sentence aloud. That is genuinely useful (it is what laminated paper boards do, and they are used every day in ICUs), but it is one feature away from being a **speech device**, which is a different category of product. That feature is text-to-speech, and it is why the roadmap below starts where it does.

---

## 3. Roadmap

Ordered by "how much does this change whether a real person can use the app", not by effort.

### 3.1 Now — the things that block real use

**1. Speech output.** Without it the user cannot address someone who is not looking at the screen — across a room, in the dark, or a nurse facing away. A `Speak` button next to the dock, plus optional speak-on-tap for each word.
*Implementation:* `web_sys::SpeechSynthesis` on wasm and the `tts` crate natively, behind a small `trait Voice { fn speak(&self, text: &str); }` so the app core stays platform-agnostic. Voice, rate and pitch belong in Settings; Turkish voice availability must be checked at runtime and reported honestly if missing.

**2. Export / import of the whole board as JSON.** Today a board lives only in browser local storage. A cleared cache, a new device or a reinstall destroys hours of a carer's work — and one such loss ends the app's use in that household permanently. A single "Save board to file" / "Load board" pair converts the app from a toy into something a service could recommend. Add a `schema_version` field now, before there are boards in the wild to migrate.

**3. Undo for editing, and a guard on destructive actions.** Deleting a category deletes all of its words with no confirmation and no recovery. A one-step undo stack (or a confirm step on delete) is a few dozen lines and removes the worst failure mode in the app.

**4. Lock Edit mode.** A single tap on `Edit` currently hands the speaker a board they can dismantle by accident. Gate it behind a long-press, or a 4-digit PIN set by the carer. Standard practice in every shipping AAC app.

**5. Real accessibility metadata.** `eframe` is built with the `accesskit` feature, but the canvas nodes are hand-painted rectangles, so a screen reader sees an empty window. Register a label and role for each node. Many AAC users have co-occurring low vision; this is not a niche.

### 3.2 Next — weeks 1–4

**6. Switch and dwell access.** Tapping is not the only input. The clinical standard for severe motor impairment is **single- or dual-switch scanning**: the board highlights each tile in turn and the user hits one switch to select. Add:
- row/column or linear scanning with a configurable step time;
- dwell selection (hover for N ms to activate) for eye-gaze and head-pointer users;
- release-activation and a hold-to-activate delay so a tremor or a dragged finger does not fire the wrong tile.
This is the single change that widens the addressable user base the most.

**7. A persistent core-word bar.** Around 200 "core words" (*I, want, go, more, stop, help, no, again*) account for the large majority of everything anyone says. Burying them one level deep costs two taps every time. A always-visible strip of 8–12 pinned words, above the dock, is the highest-frequency saving available.

**8. Photos, not just emoji.** A photograph of *their* mug, *their* carer, *their* street is more recognisable than any pictogram — this is well established in the literature and universally requested by families. Needs an image loader, a picker, and a move from `localStorage` (~5 MB) to IndexedDB on the web.

**9. A better sentence dock.** Reorder by drag, insert at a position, and a large read-aloud line rendered at listener-distance type size. Right now a mis-tapped word can only be removed, not moved.

**10. Phrase bank.** Whole utterances that are needed instantly and identically every time — "I need to lie down", "Call my mother", "That hurts". One tap, no assembly. In an emergency, sentence-building is the wrong interaction.

### 3.3 Later — the product, not the app

**11. Multiple profiles**, so one tablet serves a family, a ward or a classroom.

**12. Usage insight for the therapist.** Which words are actually used, which are never touched, which are hunted for. This is how a speech therapist tunes a board between sessions. It must be local-only and explicitly opt-in — this is medical-adjacent data about a person who may not be able to object.

**13. Vocabulary starter packs.** A blank board is an intimidating first run for a carer in a hospital corridor. Ship curated sets — *Hospital*, *Home*, *School*, *Post-stroke* — in English and Turkish, importable in one tap. The single largest reduction in time-to-first-sentence.

**14. Sharing a board.** A carer configures on a laptop, the speaker uses a tablet. A QR code or a file is enough; no server required. Anything cloud-based must stay optional — many families will not accept an account.

**15. Text entry with prediction** for literate users who prefer to spell, with the board as fallback.

**16. Localisation beyond two languages,** and moving strings out of the `match` in `app.rs` into data files so a translator never has to touch Rust. Turkish is the wedge; Arabic, Kurdish and Urdu are similarly underserved and would need RTL support.

### 3.4 What to validate before building much more

Everything above is a hypothesis. Five sessions with two real users and one SLP would settle most of it:

| Question | How to measure |
| --- | --- |
| Is the spatial canvas better than a grid for this user? | Taps-to-target and mis-taps for the same 10 words in both layouts |
| Does semantic zoom preserve the mental map? | Can the user find a word 24 h later without a hint? |
| Is 184 × 144 the right tile size? | Mis-tap rate per size, per user — expect it to differ wildly |
| Can a carer build a usable 30-word board unaided? | Time-to-first-sentence, and where they get stuck |
| Do the category colours actually speed up selection? | Selection time with colour vs. a monochrome build |

The metric that matters is not sessions or retention. It is **utterances per day** and **time-to-first-utterance for a new user** — everything else is a proxy.

### 3.5 Technical debt worth clearing early

- **Schema versioning and migration** before any real board exists. `#[serde(default)]` covers additive changes only.
- **Move `t()`'s string table out of `app.rs`** into a data file; it is already the longest thing in the file and it grows with every language.
- **A test for the geometry** — pan clamping, grid snapping and the zoom transform are pure functions and should be unit-tested; they are also the parts most likely to break silently on a screen size nobody tried.
- **Cap the icon field to a single grapheme cluster.** Nothing stops a carer from pasting a paragraph into it today.
- **Bundled fonts only cover a subset of emoji.** The font shipped with `epaint` lacks several common glyphs (🤲, 🤕, 🧑, ZWJ sequences), which render as empty boxes. Either bundle a fuller emoji font or ship a validated picker rather than a free-text field.
