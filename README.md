The System Architect Prompt

Role: Act as a Senior UX/UI Designer specializing in Assistive Technology (AAC - Augmentive and Alternative Communication).

Project Goal: Create a high-fidelity interface for "Communicator," an app for non-verbal users with limited dexterity. The design must prioritize visual hierarchy and spatial memory.

1. The "Planes" Architecture (Navigation)

    The Multiverse Layout: Design a background consisting of user-named "Planes of Existence" (e.g., "Home," "Hospital," "Social"). Use a subtle depth effect or breadcrumb trail to show which plane is active.

    The Menu System: Settings and Plane-switching must be hidden in a "Ghost Menu" that only populates when explicitly triggered, preventing accidental clicks. Menus must be "floatable" (movable) to accommodate the user's comfortable range of motion.

2. The Category & Term Nodes (Interaction)

    Dynamic Bubbles: Create categories as rounded rectangles. Each must support a high-contrast emoji or photo indicator.

    Expansion Logic: When a category is selected, use a "semantic zoom" transition. The parent category should expand smoothly into its children (sub-categories or root terms) while keeping a clear, persistent "Back" or "Up" button in a consistent spatial location.

    Visual Constraints: Ensure elements are resizable but utilize a grid-snapping system to prevent "clutter" and overlapping.

3. The Sentence Builder (The "Term Collection")

    The Dock: A dedicated area at the bottom of the screen that acts as a staging area for selected terms.

    Visual Syntax: Each term in the dock must be wrapped in a rounded rectangle. The fill color of the rectangle must match the parent category’s user-defined color (e.g., Actions = Green, People = Blue).

    Customization: Provide a palette UI where users can assign HEX codes or presets to specific categories.

4. Design Aesthetic & Accessibility

    Style: Clean, modern, and "Soft-Tech." Avoid harsh gradients. Use 24px padding between elements to prevent "fat-finger" errors.

    Feedback: Include visual "press" states (gentle scaling or glow) to confirm selection without the need for haptic or audio feedback if the user prefers silence.

UX Logic Breakdown for Implementation

To ensure this project succeeds, I have structured the underlying logic of the prompt based on these UX pillars:
Feature	UX Rationale
Floating Menus	Essential for users with "reach zones" (e.g., someone using a stylus with their mouth or a single finger may only be able to reach the left side of the screen).
Color Coding	Uses Pre-attentive Processing—the brain recognizes the color "Yellow" for "Needs/Urgent" faster than it reads the word.
Semantic Zoom	Avoids jarring screen flashes. By "expanding" the category rather than "opening a new page," the user maintains their mental map of where they are.
The Bottom Dock	Placing the sentence builder at the bottom keeps the "output" close to the user's hand/input device, minimizing eye travel.